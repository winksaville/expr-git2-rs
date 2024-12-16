use git2::Repository;
use log::debug;
use std::env;
use std::error::Error;
use std::io::{self, Write};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".")]
    repo_path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    custom_logger::env_logger_init(&log_level);

    debug!("main:+");

    let args = Args::parse();
    debug!("args={:?}", args);


    let repo = Repository::open(args.repo_path)?;
    get_top_level_info(&repo, &mut io::stdout())?;
    debug!("main:-");

    Ok(())
}

/// Retrieves and writes top-level repository information to the given writer.
fn get_top_level_info<W: Write>(repo: &Repository, writer: &mut W) -> Result<(), Box<dyn Error>> {
    writeln!(writer, "Is bare: {}", repo.is_bare())?;
    writeln!(writer, "Is worktree: {}", repo.is_worktree())?;
    writeln!(writer, "Path to repository: {:?}", repo.path())?;
    writeln!(writer, "Workdir: {:?}", repo.workdir())?;
    writeln!(writer, "HEAD reference: {:?}", repo.head()?.name())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::{Repository, Signature, Oid, BranchType};
    use std::fs;
    use std::io::Cursor;
    use std::path::{Path, PathBuf, MAIN_SEPARATOR};
    use std::sync::Once;
    use log::debug;

    static INIT: Once = Once::new();

    fn init_logger() {
        INIT.call_once(|| {
            let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
            //println!("RUST_LOG={}", log_level);
            custom_logger::env_logger_init(&log_level);
        });
    }

    #[test]
    fn test_logging() {
        init_logger();
        debug!("Test with logging");
    }

    #[test]
    fn test_get_top_level_info() -> Result<(), Box<dyn Error>> {
        init_logger();

        fs::create_dir_all("test_repos/test_top_level_info")?;
        let test_dir = PathBuf::from("test_repos/test_top_level_info");

        // Create an empty repository
        create_empty_repo(&test_dir, "Initial commit")?;

        // Open the repository
        let repo = Repository::open(&test_dir)?;

        // Capture output in an in-memory buffer
        let mut output = Cursor::new(Vec::new());
        get_top_level_info(&repo, &mut output)?;

        // Convert buffer to a string
        let output_str = String::from_utf8(output.into_inner())?;
        debug!("\n{output_str}");

        let test_dir_str = add_trailing_separator_canonicalized(&test_dir)?;
        let repo_dir_str = add_trailing_separator_canonicalized(test_dir.join(".git"))?;

        // Verify repository information
        assert!(output_str.contains("Is bare: false"));
        assert!(output_str.contains("Is worktree: false"));
        assert!(output_str.contains(&format!("Path to repository: {repo_dir_str:?}")));
        assert!(output_str.contains(&format!("Workdir: Some({test_dir_str:?})")));
        assert!(output_str.contains(r#"HEAD reference: Some("refs/heads/main")"#));

        Ok(())
    }

    #[test]
    fn test_empty_repo() -> Result<(), Box<dyn Error>> {
        init_logger();

        let test_dir = PathBuf::from("test_repos/test_empty_repo");

        // Create an empty repository
        create_empty_repo(&test_dir, "Initial commit")?;

        // Re-open the repository
        let repo = Repository::open(&test_dir)?;

        // Get head commit
        let head_commit = repo.head()?.peel_to_commit()?;

        // Assert all the parameters are as expected
        assert!(head_commit.parent_count() == 0, "Unexpected: commit has {} parents", head_commit.parent_count());
        assert!(head_commit.id() != Oid::from_str("0")?, "Unexpected: id is 0");
        assert!(head_commit.message().unwrap_or_default() == "Initial commit", "Unexpected: commit message");
        assert!(head_commit.author().name().unwrap_or_default() == "Test User", "Unexpected: author name");
        assert!(head_commit.author().email().unwrap_or_default() == "test@example.com", "Unexpected: author email");
        assert!(head_commit.committer().name().unwrap_or_default() == "Test User", "Unexpected: committer name");
        assert!(head_commit.committer().email().unwrap_or_default() == "test@example.com", "Unexpected: committer email");

        // Verify the HEAD commit points to an empty tree
        let tree = head_commit.tree()?;
        assert!(tree.iter().next().is_none(), "Tree is not empty");

        // there should be no blob
        assert!(repo.find_blob(head_commit.id()).is_err(), "Unexpected: blob found");

        // There should be 1 local branch
        let b = &repo.branches(Some(BranchType::Local))?.into_iter().collect::<Result<Vec<_>, _>>()?;
        assert!(b.len() == 1, "Unexpected: branches found");
        let (b0, b0_type) = &b[0];
        assert!(*b0_type == BranchType::Local, "Unexpected: b0_type={b0_type:?}");

        // The branch should be named "main"
        let b0_name = b0.name()?.unwrap();
        assert!(b0_name == "main", "Unexpected: b0_name={b0_name}");

        Ok(())
    }

    // thx to GPT4o: https://chatgpt.com/share/675f6532-ba08-800c-847e-6a3dd874a4dc
    fn add_trailing_separator_canonicalized<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
        let canonicalized = fs::canonicalize(&path)?;
        let path_str = canonicalized.to_string_lossy();
        Ok(if path_str.ends_with(MAIN_SEPARATOR) {
            path_str.to_string()
        } else {
            format!("{}{}", path_str, MAIN_SEPARATOR)
        })
    }

    /// Creates an empty Git repository at the specified path.
    fn create_empty_repo<P: AsRef<Path>>(path: P, message: &str) -> Result<(), Box<dyn Error>> {
        // Clean up any existing test repo
        let path = path.as_ref();
        if path.exists() {
            fs::remove_dir_all(path)?;
        }

        // Initialize the repository
        let repo = Repository::init(path)?;

        // Set explicit branch for compatibility
        repo.set_head("refs/heads/main")?;

        // Get the empty index and write it
        let mut index = repo.index()?;
        index.write()?;

        // Create an empty tree
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;

        // Commit the empty tree
        let signature = Signature::now("Test User", "test@example.com")?;
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &[],
        )?;

        Ok(())
    }
}

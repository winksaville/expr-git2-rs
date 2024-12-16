use clap::Parser;
use git2::{Commit, DiffFormat, DiffOptions, Repository, Tree};
use log::debug;
use std::env;
use std::error::Error;
use std::io::{self, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = ".")]
    repo_path: String,

    #[arg(short, long, default_value = "head")]
    sha: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "None".to_string());
    custom_logger::env_logger_init(&log_level);

    debug!("main:+");

    let args = Args::parse();
    debug!("args={:?}", args);

    // Top-level repository information
    let repo = Repository::open(&args.repo_path)?;
    get_top_level_info(&repo, &mut io::stdout())?;

    // Get the commit specified by the SHA or the head commit
    let commit = if args.sha == "head" {
        // Get head commit
        repo.head()?.peel_to_commit()?
    } else {
        // Get the commit specified by the SHA
        repo.find_commit(repo.revparse_single(&args.sha)?.peel_to_commit()?.id())?
    };
    show_commit(&args.repo_path, &commit)?;

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

fn show_commit(repo_path: &str, commit: &Commit) -> Result<(), Box<dyn Error>> {
    log::info!(
        "show_commit:+ repo_path: {repo_path}, commit_id: {}",
        commit.id()
    );
    let repo = Repository::open(repo_path)?;

    //let commit = repo.revparse_single(oid_string)?.peel_to_commit()?;
    let commit_tree = commit.tree()?;
    let parents = commit.parents().collect::<Vec<_>>();

    match parents.len() {
        0 => {
            println!("TODO: Handle root commit");
        }
        1 => {
            // Handle non-merge commits
            let parent = parents.first().unwrap();
            let parent_tree = parent.tree()?;
            let _ = print_diff(&repo, &parent_tree, &commit_tree, "parent vs commit_tree");
        }
        2 => {
            // Create an in-memory index to store the merged result
            let mut index = repo.index()?;

            // Add parent0_tree entries to the index
            add_tree_to_index(&mut index, &repo, &parents[0].tree()?, None)?;

            // Write the index to create a new tree
            let tree_oid = index.write_tree()?;
            let new_tree = repo.find_tree(tree_oid)?;
            //print_diff(&repo, &new_tree, &parents[0].tree()?, "new_tree vs parents[0].tree() should be the same!!!!!")?;

            let org_tree = parents[0].tree()?;
            let mut org_tree_iter = org_tree.iter();
            for entry in new_tree.iter() {
                if let Some(org_entry) = org_tree_iter.next() {
                    println!(
                        "Org tree entry: path={}, id={}, mode={}",
                        org_entry.name().unwrap_or("<invalid>"),
                        org_entry.id(),
                        org_entry.filemode()
                    );
                }
                println!(
                    "New tree entry: path={}, id={}, mode={}",
                    entry.name().unwrap_or("<invalid>"),
                    entry.id(),
                    entry.filemode()
                );
            }

            // // Handle merge commits
            // let merged_baseline_tree =
            //     create_merged_baseline_tree(&repo, &parents[0], &parents[1])?;
            // let _ = print_diff(
            //     &repo,
            //     &merged_baseline_tree,
            //     &commit_tree,
            //     "merged_baseline_tree vs commit_tree",
            // );
        }
        _ => {
            println!("TODO: Handle more than 2 parents");
        }
    }

    println!(
        "commit id: {}, summary: '{}', parent.len: {}, parents: {:?}",
        commit.id(),
        commit.summary().unwrap_or(""),
        parents.len(),
        parents.iter().map(|p| p.id()).collect::<Vec<_>>(),
    );

    log::info!(
        "show_commit:- repo_path: {repo_path}, commit_id: {}",
        commit.id()
    );
    Ok(())
}

fn add_tree_to_index(
    index: &mut git2::Index,
    repo: &git2::Repository,
    tree: &git2::Tree,
    base_path: Option<&std::path::Path>,
) -> Result<(), git2::Error> {
    for entry in tree.iter() {
        let entry_name = entry
            .name()
            .ok_or_else(|| git2::Error::from_str("Invalid tree entry name"))?;
        let oid = entry.id();
        let filemode = entry.filemode() as u32;

        // Combine base path with the current entry name
        let full_path = if let Some(base) = base_path {
            base.join(entry_name)
        } else {
            std::path::PathBuf::from(entry_name)
        };

        println!("Processing entry: full_path={}", full_path.display());

        let object = repo.find_object(oid, None)?;
        if let Some(blob) = object.as_blob() {
            println!("Adding blob to index: {}", full_path.display());
            index.add_frombuffer(
                &git2::IndexEntry {
                    ctime: git2::IndexTime::new(0, 0),
                    mtime: git2::IndexTime::new(0, 0),
                    dev: 0,
                    ino: 0,
                    mode: filemode,
                    uid: 0,
                    gid: 0,
                    file_size: blob.size() as u32,
                    id: oid,
                    path: full_path.to_str().unwrap().as_bytes().to_vec(),
                    flags: 0,
                    flags_extended: 0,
                },
                blob.content(),
            )?;
        } else if let Some(subtree) = object.as_tree() {
            println!("Recursing into subtree: {}", full_path.display());
            add_tree_to_index(index, repo, subtree, Some(&full_path))?;
        } else {
            return Err(git2::Error::from_str("Unexpected object type in tree"));
        }
    }
    Ok(())
}

fn print_diff(
    repo: &Repository,
    from_tree: &Tree,
    to_tree: &Tree,
    label: &str,
) -> Result<(), Box<dyn Error>> {
    println!("print_diff:+ {label}");

    // Create and configure DiffOptions
    let mut opts = DiffOptions::new();
    opts.include_untracked(true)
        .include_ignored(false)
        .context_lines(3)
        .recurse_untracked_dirs(true) // To include untracked directories
        .include_unmodified(false);

    let diff = repo.diff_tree_to_tree(Some(from_tree), Some(to_tree), Some(&mut opts))?;

    // Print the diff in Patch format with prefixes
    diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
        // Prefix for added/removed/unchanged lines
        let prefix = match line.origin() {
            '+' => "+", // Line added
            '-' => "-", // Line removed
            ' ' => " ", // Unchanged line
            '@' => "@", // Hunk header
            _ => " ",   // Fallback for other types
        };

        // Print the prefixed line
        print!(
            "{}{}",
            prefix,
            std::str::from_utf8(line.content()).unwrap_or("[INVALID UTF-8]")
        );

        true
    })?;

    println!("print_diff:- {label} END");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::{BranchType, Oid, Repository, Signature};
    use log::debug;
    use std::fs;
    use std::io::Cursor;
    use std::path::{Path, PathBuf, MAIN_SEPARATOR};
    use std::sync::Once;

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
        assert!(
            head_commit.parent_count() == 0,
            "Unexpected: commit has {} parents",
            head_commit.parent_count()
        );
        assert!(
            head_commit.id() != Oid::from_str("0")?,
            "Unexpected: id is 0"
        );
        assert!(
            head_commit.message().unwrap_or_default() == "Initial commit",
            "Unexpected: commit message"
        );
        assert!(
            head_commit.author().name().unwrap_or_default() == "Test User",
            "Unexpected: author name"
        );
        assert!(
            head_commit.author().email().unwrap_or_default() == "test@example.com",
            "Unexpected: author email"
        );
        assert!(
            head_commit.committer().name().unwrap_or_default() == "Test User",
            "Unexpected: committer name"
        );
        assert!(
            head_commit.committer().email().unwrap_or_default() == "test@example.com",
            "Unexpected: committer email"
        );

        // Verify the HEAD commit points to an empty tree
        let tree = head_commit.tree()?;
        assert!(tree.iter().next().is_none(), "Tree is not empty");

        // there should be no blob
        assert!(
            repo.find_blob(head_commit.id()).is_err(),
            "Unexpected: blob found"
        );

        // There should be 1 local branch
        let b = &repo
            .branches(Some(BranchType::Local))?
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;
        assert!(b.len() == 1, "Unexpected: branches found");
        let (b0, b0_type) = &b[0];
        assert!(
            *b0_type == BranchType::Local,
            "Unexpected: b0_type={b0_type:?}"
        );

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
        repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[])?;

        Ok(())
    }
}

use git2::Repository;
use std::error::Error;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let repo = Repository::open(".")?;
    get_top_level_info(&repo, &mut io::stdout())
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
    use git2::{Repository, Signature};
    use std::fs;
    use std::io::Cursor;
    use std::path::Path;

    #[test]
    fn test_get_top_level_info() -> Result<(), Box<dyn Error>> {
        let test_dir = fs::canonicalize("test_repo")?;
        let test_dir_str = format!("{}/", test_dir.display());
        let repo_dir = test_dir.join(".git");
        let repo_dir_str = format!("{}/", repo_dir.display());

        {
            // Clean up any existing test repo
            if test_dir.exists() {
                fs::remove_dir_all(&test_dir)?;
            }

            // Create a new repository
            let repo = Repository::init(&test_dir)?;

            // Create a new file in the repository
            let file_path = test_dir.join("README.md");
            fs::write(&file_path, "Hello, world!")?;

            // Stage the file
            let mut index = repo.index()?;
            index.add_path(Path::new("README.md"))?;
            index.write()?;

            // Commit the changes
            let tree_id = index.write_tree()?;
            let tree = repo.find_tree(tree_id)?;
            let signature = Signature::now("Test User", "test@example.com")?;
            repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                "Initial commit",
                &tree,
                &[],
            )?;
        }

        {
            // Re-open the repository
            let repo = Repository::open(&test_dir)?;

            // Capture output in an in-memory buffer
            let mut output = Cursor::new(Vec::new());
            get_top_level_info(&repo, &mut output)?;

            // Convert buffer to a string
            let output_str = String::from_utf8(output.into_inner())?;
            //print!("{output_str}");

            // Verify repository information
            assert!(output_str.contains("Is bare: false"));
            assert!(output_str.contains("Is worktree: false"));
            assert!(output_str.contains(&format!("Path to repository: {repo_dir_str:?}")));
            assert!(output_str.contains(&format!("Workdir: Some({test_dir_str:?})")));
            assert!(output_str.contains(r#"HEAD reference: Some("refs/heads/main")"#));
        }

        Ok(())
    }
}

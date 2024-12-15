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
    use git2::{IndexAddOption, Repository, Signature};
    use std::fs;
    use std::io::Cursor;
    use std::path::Path;

    #[test]
    fn test_get_top_level_info() -> Result<(), Box<dyn Error>> {
        let test_dir = Path::new("test_repo");
        
        // Clean up any existing test repo
        if test_dir.exists() {
            fs::remove_dir_all(test_dir)?;
        }

        // Create a new repository
        let repo = Repository::init(test_dir)?;

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
        repo.commit(Some("HEAD"), &signature, &signature, "Initial commit", &tree, &[])?;

        // Capture output in an in-memory buffer
        let mut output = Cursor::new(Vec::new());
        get_top_level_info(&repo, &mut output)?;

        // Convert buffer to a string
        let output_str = String::from_utf8(output.into_inner()).expect("Valid UTF-8");

        // Verify repository information
        assert!(output_str.contains("Is bare: false"));
        assert!(output_str.contains("Is worktree: true"));
        assert!(output_str.contains(&format!("Path to repository: {:?}", test_dir.join(".git"))));
        assert!(output_str.contains(&format!("Workdir: {:?}", test_dir)));
        assert!(output_str.contains("HEAD reference: Some(\"refs/heads/master\")"));

        Ok(())
    }
}

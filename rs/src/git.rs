use anyhow::{Error, Result};
use std::path::Path;

/// Return name of the currently active git branch.
pub fn get_branch(path: impl AsRef<Path>) -> Result<String> {
    // Find the repository by searching up through parent directories
    let repo = gix::discover(path)?;

    let head = repo
        .head_ref()?
        .ok_or_else(|| Error::msg("Failed to get HEAD reference"))?;

    let name = head.name().shorten();

    Ok(name.to_string())
}

/// Return the current commit hash in short format.
///
/// The minimum length is 4, the default is the effective value of the `core.abbrev` configuration variable (see <https://git-scm.com/docs/git-config>)
pub fn get_commit_hash(path: impl AsRef<Path>) -> Result<String> {
    // Find the repository by searching up through parent directories
    let repo = gix::discover(path)?;

    // Get the HEAD commit
    let mut head = repo.head()?;
    let head_commit = head.peel_to_commit_in_place()?;

    // Shorten the hash as defined by `core.abbrev`
    let short_hash = head_commit.id().shorten()?.to_string();

    Ok(short_hash)
}

/// Return whether the git repository has uncommitted changes.
pub fn is_dirty(path: impl AsRef<Path>) -> Result<bool> {
    // Find the repository by searching up through parent directories
    let repo = gix::discover(path)?;
    Ok(repo.is_dirty()?)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::Command;
    use tempfile::TempDir;
    use test_utils;

    use super::*;

    #[test]
    fn test_get_branch() {
        // Create a temporary directory for our test git repository
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_path = temp_dir.path();

        test_utils::setup_git_repo(temp_path, None);

        // Test the get_branch function
        let branch = get_branch(temp_path).expect("Failed to get branch");
        assert_eq!(branch, "master");
    }

    #[test]
    fn test_get_commit_hash() {
        // Create a temporary directory for our test git repository
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_path = temp_dir.path();

        // Set up git repository
        test_utils::setup_git_repo(temp_path, None);

        // Get the hash using our function
        let our_hash = get_commit_hash(temp_path).expect("Failed to get commit hash");

        // Get the hash using git command
        let git_output = Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .current_dir(temp_path)
            .output()
            .expect("Failed to get git hash");
        let git_hash = String::from_utf8(git_output.stdout)
            .expect("Git output was not valid UTF-8")
            .trim()
            .to_string();

        // Compare the hashes
        assert_eq!(our_hash, git_hash, "Our hash should match git's hash");
    }

    #[test]
    fn test_is_dirty() {
        // Create a temporary directory for our test git repository
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_path = temp_dir.path();

        // Set up git repository
        test_utils::setup_git_repo(temp_path, None);

        // Print git status for debugging
        let status_output = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(temp_path)
            .output()
            .expect("Failed to get git status");
        println!(
            "Git status after setup: {:?}",
            String::from_utf8_lossy(&status_output.stdout)
        );

        // Initially the repository should be clean
        let status = is_dirty(temp_path).expect("Failed to get status");
        assert!(!status, "Repository should be clean after initial setup");

        // Create a new untracked file
        let file_path = temp_path.join("untracked.txt");
        fs::write(&file_path, "untracked content").expect("Failed to write untracked file");

        // Stage the file
        Command::new("git")
            .args(["add", "untracked.txt"])
            .current_dir(temp_path)
            .output()
            .expect("Failed to stage file");

        // Repository should still be dirty with staged changes
        let status = is_dirty(temp_path).expect("Failed to get status");
        assert!(status, "Repository should be dirty with staged changes");

        // Commit the file
        Command::new("git")
            .args(["commit", "-m", "Add untracked file"])
            .current_dir(temp_path)
            .output()
            .expect("Failed to commit");

        // Repository should be clean again
        let status = is_dirty(temp_path).expect("Failed to get status");
        assert!(!status, "Repository should be clean after committing");
    }
}

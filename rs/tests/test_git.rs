use std::fs;
use std::process::Command;
use tempfile::TempDir;
mod utils;

#[test]
fn test_get_branch() {
    // Create a temporary directory for our test git repository
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    utils::setup_git_repo(temp_path, None);

    let branch = artisan_tools::git::get_branch(temp_path).expect("Failed to get branch");
    assert_eq!(branch, "master");
}

#[test]
fn test_get_commit_hash() {
    // Create a temporary directory for our test git repository
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    utils::setup_git_repo(temp_path, None);

    // Get the hash using our function
    let our_hash =
        artisan_tools::git::get_commit_hash(temp_path).expect("Failed to get commit hash");

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

    utils::setup_git_repo(temp_path, None);

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
    let status = artisan_tools::git::is_dirty(temp_path).expect("Failed to get status");
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
    let status = artisan_tools::git::is_dirty(temp_path).expect("Failed to get status");
    assert!(status, "Repository should be dirty with staged changes");

    // Commit the file
    Command::new("git")
        .args(["commit", "-m", "Add untracked file"])
        .current_dir(temp_path)
        .output()
        .expect("Failed to commit");

    // Repository should be clean again
    let status = artisan_tools::git::is_dirty(temp_path).expect("Failed to get status");
    assert!(!status, "Repository should be clean after committing");
}

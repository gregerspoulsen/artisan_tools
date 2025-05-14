use std::fs;
use std::path::Path;
use std::process::Command;

/// Helper function to set up a git repository in the given directory with an optional version
pub fn setup_git_repo(path: &Path, version: Option<&str>) {
    // Initialize git repository
    Command::new("git")
        .args(["init"])
        .current_dir(path)
        .output()
        .expect("Failed to initialize git repository");

    // Configure git user for the test
    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(path)
        .output()
        .expect("Failed to configure git user name");

    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(path)
        .output()
        .expect("Failed to configure git user email");

    // Create a dummy file and commit it
    let file_path = path.join("dummy.txt");
    fs::write(&file_path, "dummy content").expect("Failed to write dummy file");

    // Create .at-version file with provided version or default
    let version_file = path.join(".at-version");
    fs::write(&version_file, version.unwrap_or("0.1.0")).expect("Failed to write version file");

    // Add and commit both files
    Command::new("git")
        .args(["add", "dummy.txt", ".at-version"])
        .current_dir(path)
        .output()
        .expect("Failed to add files");

    Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(path)
        .output()
        .expect("Failed to commit");
} 
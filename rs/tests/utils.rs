use std::fs;
use std::path::Path;
use std::process::Command;
use semver::Version;

/// Default version to use when setting up a git repository
pub const DEFAULT_VERSION: Version = Version::new(0, 1, 0);

/// Helper function to set up a git repository in the given directory with a version
pub fn setup_git_repo(path: &Path, version: Option<Version>) {
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

    // Use the provided version or default
    let version = version.unwrap_or(DEFAULT_VERSION);

    // Create .at-version file with version
    let version_file = path.join(".at-version");
    fs::write(&version_file, version.to_string()).expect("Failed to write version file");

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
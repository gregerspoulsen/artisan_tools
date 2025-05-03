use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;


#[test]
fn test_get_branch() {
    // Create a temporary directory for our test git repository
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    
    // Set up git repository
    setup_git_repo(temp_path);
    
    // Change to the temporary directory
    std::env::set_current_dir(temp_path).expect("Failed to change directory");
    
    // Test the get_branch function
    let branch = artisan_tools::git::get_branch().expect("Failed to get branch");
    assert_eq!(branch, "main");
}

/// Helper function to set up a git repository in the given directory
fn setup_git_repo(path: &Path) {
    // Initialize git repository
    Command::new("git")
        .args(["init"])
        .current_dir(path)
        .output()
        .expect("Failed to initialize git repository");
    
    // Create a dummy file and commit it
    let file_path = path.join("dummy.txt");
    fs::write(&file_path, "dummy content").expect("Failed to write dummy file");
    
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
    
    // Add and commit the file
    Command::new("git")
        .args(["add", "dummy.txt"])
        .current_dir(path)
        .output()
        .expect("Failed to add file");
    
    Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(path)
        .output()
        .expect("Failed to commit");
    
    // Rename the default branch to main (in case it's master)
    Command::new("git")
        .args(["branch", "-M", "main"])
        .current_dir(path)
        .output()
        .expect("Failed to rename branch to main");
} 
use artisan_tools::version_mod;
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use pretty_assertions::assert_str_eq;
use std::{fs, process::Command};
use testresult::TestResult;
use assert_fs::TempDir;
mod utils;

/// Test that when cwd has no .at-version file, we error with an informative error message
#[test]
fn at_version_update_no_at_version_file_errors() -> TestResult {
    // Arrange
    let test_dir = TempDir::new()?;
    let mut cmd = Command::cargo_bin("at")?;
    cmd.args(["version", "update"]);
    cmd.current_dir(&test_dir);

    // Act
    // Assert (assert also runs the command)
    cmd.assert().failure().stderr(predicate::str::contains(
        "Failed to read from version file: .at-version",
    ));

    Ok(())
}

/// Test that when cwd has a .at-version file, we correctly retrieve the version
#[test]
fn at_version_update_at_version_file_exists_ok() -> TestResult {
    // Arrange
    const VERSION: &str = "0.1.0";
    let test_dir = TempDir::new()?;
    utils::setup_git_repo(test_dir.path(), Some(VERSION));
    
    let mut cmd = Command::cargo_bin("at")?;
    cmd.args(["version", "update"]).current_dir(&test_dir);

    // Act + Assert
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "VERSION updated to {VERSION}"
        )));
    let version_file_contents = fs::read_to_string(test_dir.join("VERSION"))?;
    assert_str_eq!(version_file_contents, VERSION);

    Ok(())
}

/// Test that version_mod::get returns just the version when git_info is false
#[test]
fn version_get_without_git_info() -> TestResult {
    // Arrange
    const VERSION: &str = "1.2.3";
    let test_dir = TempDir::new()?;
    utils::setup_git_repo(test_dir.path(), Some(VERSION));

    // Switch the directory with the .at-version file
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&test_dir)?;

    // Act
    let result = version_mod::get(false)?;

    // Cleanup and Assert
    std::env::set_current_dir(original_dir)?;
    assert_str_eq!(result, VERSION);
    Ok(())
}

/// Test that version_mod::get returns version with git info when git_info is true
#[test]
fn version_get_with_git_info() -> TestResult {
    const VERSION: &str = "1.2.3";
    let test_dir = TempDir::new()?;
    utils::setup_git_repo(test_dir.path(), Some(VERSION));

    // Switch the directory with the .at-version file
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&test_dir)?;

    // Act
    let result = version_mod::get(true)?;

    std::env::set_current_dir(original_dir)?;

    // The result should be in format: version+branch-hash
    // We know the version is "1.2.3" and branch should be "main" or "master"
    assert!(
        result.starts_with("1.2.3+"),
        "Version should start with 1.2.3+"
    );
    assert!(
        result.contains("main-") || result.contains("master-"),
        "Should contain branch name"
    );
    assert!(
        !result.ends_with("-dirty"),
        "Should not be dirty as we committed all changes"
    );

    Ok(())
}

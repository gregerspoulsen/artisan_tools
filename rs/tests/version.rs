use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use pretty_assertions::assert_str_eq;
use std::{fs, process::Command};
use testresult::TestResult;

/// Test that when cwd has no RELEASE file, we error with an informative error message
#[test]
fn at_version_update_no_release_file_errors() -> TestResult {
    // Arrange
    let test_dir = assert_fs::TempDir::new()?;
    let mut cmd = Command::cargo_bin("at")?;
    cmd.args(["version", "update"]);
    cmd.current_dir(&test_dir);

    // Act
    // Assert (assert also runs the command)
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read the RELEASE file"));

    Ok(())
}

/// Test that when cwd has a RELEASE file, we correctly retrieve the version
#[test]
fn at_version_update_release_file_exists_ok() -> TestResult {
    // Arrange
    const RELEASE_VERSION: &str = "0.1.0";
    let test_dir = assert_fs::TempDir::new()?;
    let release_file = test_dir.join("RELEASE");
    fs::write(release_file, RELEASE_VERSION)?;
    let mut cmd = Command::cargo_bin("at")?;
    cmd.args(["version", "update"]).current_dir(&test_dir);

    // Act + Assert
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "VERSION updated to {RELEASE_VERSION}"
        )));
    let version_file_contents = fs::read_to_string(test_dir.join("VERSION"))?;
    assert_str_eq!(version_file_contents, RELEASE_VERSION);

    Ok(())
}

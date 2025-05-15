use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::TempDir;
use predicates::prelude::*; // Used for writing assertions
use pretty_assertions::assert_str_eq;
use semver::Version;
use std::{fs, process::Command};
use test_utils;
use testresult::TestResult;

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
        "Failed to read from version file: ./.at-version",
    ));

    Ok(())
}

/// Test that when cwd has a .at-version file, we correctly retrieve the version
#[test]
fn at_version_update_at_version_file_exists_ok() -> TestResult {
    // Arrange
    let version = Version::new(0, 1, 0);
    let test_dir = TempDir::new()?;
    test_utils::setup_git_repo(test_dir.path(), Some(version.clone()));

    let mut cmd = Command::cargo_bin("at")?;
    cmd.args(["version", "update"]).current_dir(&test_dir);

    // Act + Assert
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "VERSION updated to {version}"
        )));
    let version_file_contents = fs::read_to_string(test_dir.join("VERSION"))?;
    assert_str_eq!(version_file_contents, version.to_string());

    Ok(())
}

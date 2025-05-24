use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::TempDir;
use predicates::prelude::*; // Used for writing assertions
use pretty_assertions::assert_str_eq;
use semver::Version;
use test_utils::testrepo::TestRepo;
use std::{fs, process::Command};
use testresult::TestResult;

/// Test that when cwd has no .at-version file, we error with an informative error message
#[test]
fn at_version_update_no_at_version_file_errors() -> TestResult {
    // Arrange
    let test_dir = TempDir::new()?;
    let mut cmd = Command::cargo_bin("at")?;
    cmd.args(["version", "update"]);
    cmd.current_dir(&test_dir);

    let file_path_prefix = if cfg!(target_os = "windows") {
        r".\"
    } else {
        "./"
    };

    // Act
    // Assert (assert also runs the command)
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains(format!(
            "Failed to read from version file: {file_path_prefix}.at-version"
        )));

    Ok(())
}

/// Test that when cwd has a .at-version file, we correctly retrieve the version
#[test]
fn at_version_update_at_version_file_exists_ok() -> TestResult {
    // Arrange
    let version = Version::new(0, 1, 0);
    let repo = TestRepo::builder().init(true).build();
    repo.init_at_version(&version);

    let mut cmd = Command::cargo_bin("at")?;
    cmd.args(["version", "update"]).current_dir(repo.path());

    // Act + Assert
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "VERSION updated to {version}"
        )));
    let version_file_contents = fs::read_to_string(repo.path().join("VERSION"))?;
    assert_str_eq!(version_file_contents, version.to_string());

    Ok(())
}

use assert_cmd::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;
use std::{fs, process::Command};
use testresult::TestResult;

/// Test that init creates an at-changeset file with the specified target
#[test]
fn test_changeset_init_creates_file() -> TestResult {
    // Arrange
    let test_dir = TempDir::new()?;
    let mut cmd = Command::cargo_bin("at")?;
    cmd.args(["changeset", "init", "major"]);
    cmd.current_dir(&test_dir);

    // Act & Assert
    cmd.assert().success().stdout(predicate::str::contains(
        "Created new changeset file with target: MAJOR",
    ));

    let changeset_content = fs::read_to_string(test_dir.join("at-changeset"))?;
    assert!(changeset_content.contains("TARGET: MAJOR"));
    assert!(changeset_content.contains("CHANGELOG:"));
    assert!(changeset_content.contains("### Added"));

    Ok(())
}

/// Test that init fails with an invalid target
#[test]
fn test_changeset_init_invalid_target() -> TestResult {
    // Arrange
    let test_dir = TempDir::new()?;
    let mut cmd = Command::cargo_bin("at")?;
    cmd.args(["changeset", "init", "invalid"]);
    cmd.current_dir(&test_dir);

    // Act & Assert
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid value 'invalid'"));

    Ok(())
}

/// Test that init works with all valid targets
#[test]
fn test_changeset_init_all_targets() -> TestResult {
    let test_dir = TempDir::new()?;
    let targets = ["major", "minor", "patch", "unreleased"];

    for target in targets {
        // Arrange
        let mut cmd = Command::cargo_bin("at")?;
        cmd.args(["changeset", "init", target]);
        cmd.current_dir(&test_dir);

        // Act & Assert
        cmd.assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Created new changeset file with target: {}",
                target.to_uppercase()
            )));

        let changeset_content = fs::read_to_string(test_dir.join("at-changeset"))?;
        assert!(changeset_content.contains(&format!("TARGET: {}", target.to_uppercase())));
    }

    Ok(())
}

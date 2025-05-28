use assert_cmd::prelude::*;
use assert_fs::TempDir;
use predicates::prelude::*;
use std::{fs, process::Command};
use testresult::TestResult;

use artisan_tools::changeset::{get_template_content, BumpTarget};

/// Test that init creates an at-changeset file with the specified target
#[test]
fn test_changeset_init_creates_file() -> TestResult {
    // Arrange
    let test_dir = TempDir::new()?;
    let mut cmd = Command::cargo_bin("at")?;
    cmd.args(["changeset", "major"]);
    cmd.current_dir(&test_dir);

    // Act & Assert
    cmd.assert().success().stdout(predicate::str::contains(
        "Created new changeset file with target: MAJOR",
    ));

    let changeset_content = fs::read_to_string(test_dir.join("at-changeset"))?;
    let expected = get_template_content(BumpTarget::Major);

    assert_eq!(changeset_content, expected);

    Ok(())
}

/// Test that init fails with an invalid target
#[test]
fn test_changeset_init_invalid_target() -> TestResult {
    // Arrange
    let test_dir = TempDir::new()?;
    let mut cmd = Command::cargo_bin("at")?;
    cmd.args(["changeset", "invalid"]);
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
        cmd.args(["changeset", target]);
        cmd.current_dir(&test_dir);

        // Act & Assert
        cmd.assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Created new changeset file with target: {}",
                target.to_uppercase()
            )));

        let changeset_content = fs::read_to_string(test_dir.join("at-changeset"))?;

        // Convert string target to BumpTarget enum
        let bump_target = match target {
            "major" => BumpTarget::Major,
            "minor" => BumpTarget::Minor,
            "patch" => BumpTarget::Patch,
            "unreleased" => BumpTarget::Unreleased,
            _ => unreachable!("Invalid target should have been caught by CLI validation"),
        };

        let expected = get_template_content(bump_target);
        assert_eq!(changeset_content, expected);
    }

    Ok(())
}

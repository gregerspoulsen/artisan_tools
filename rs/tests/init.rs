use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::TempDir;
use predicates::prelude::*; // Used for writing assertions
use pretty_assertions::assert_str_eq;
use semver::Version;
use std::{fs, process::Command};
use test_utils::prelude::*;
use testresult::TestResult;

#[test]
fn at_init_defaults_python_pyproject_toml_repo() -> TestResult {
    // Arrange
    let repo = TestRepo::builder().init(true).build();
    repo.init_gitignore();
    repo.create_stage_commit("pyproject.toml", None, "initial commit");

    let mut cmd = Command::cargo_bin("at")?;
    cmd.args(["init", "--yes"]).current_dir(repo.path());

    // Act + Assert
    cmd.assert().success();
    assert!(
        repo.path().join("at.toml").is_file(),
        "Expected an 'at.toml' to be present after 'at init'"
    );

    Ok(())
}

#[test]
fn at_init_defaults_rust_dry_run() -> TestResult {
    // Arrange
    let repo = TestRepo::builder().init(true).build();
    repo.init_gitignore();
    repo.create_stage_commit("pyproject.toml", None, "initial commit");
    assert!(
        repo.git_assert_success(["diff-files", "--quiet"])
            .status
            .success(),
        "Repo should be clean before dry-run command"
    );

    let mut cmd = Command::cargo_bin("at")?;
    cmd.args(["init", "--yes", "--dry-run"])
        .current_dir(repo.path());

    // Act + Assert
    cmd.assert().success();
    assert!(
        repo.git_assert_success(["diff-files", "--quiet"])
            .status
            .success(),
        "Repo should be clean after dry-run command"
    );

    Ok(())
}

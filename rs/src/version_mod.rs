use crate::git;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

const AT_VERSION_FILE: &str = ".at-version";

/// Read content from specified at-version file.
/// If the path is None, it defaults to ".at-version".
fn read_at_version() -> Result<String> {
    let version = fs::read_to_string(Path::new(AT_VERSION_FILE)).context(format!(
        "Failed to read from version file: {}",
        AT_VERSION_FILE
    ))?;
    Ok(version.trim().to_string())
}

/// Get current version, optionally including git information
///
/// If git_info is true, appends branch, hash and dirty status in the format:
/// version+branch-hash[-dirty]
pub fn get(git_info: bool) -> Result<String> {
    let mut version = read_at_version().context("Failed to read the version file")?;

    if git_info {
        let branch = git::get_branch(".")?;
        let branch = branch.replace('_', "-");

        let hash = git::get_commit_hash(".")?;

        let is_dirty = git::is_dirty(".")?;
        let dirty = if is_dirty { "-dirty" } else { "" };

        version = format!("{version}+{branch}-{hash}{dirty}");
    }

    Ok(version)
}



#[cfg(test)]
mod tests {
    use semver::Version;
    use pretty_assertions::assert_str_eq;
    use testresult::TestResult;
    use assert_fs::TempDir;
    use crate::utils;

    use super::*;

/// Test that version_mod::get returns just the version when git_info is false
#[test]
fn version_get_without_git_info() -> TestResult {
    // Arrange
    let version = Version::new(1, 2, 3);
    let test_dir = TempDir::new()?;
    utils::setup_git_repo(test_dir.path(), Some(version.clone()));

    // Switch the directory with the .at-version file
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&test_dir)?;

    // Act
    let result = get(false)?;

    // Cleanup and Assert
    std::env::set_current_dir(original_dir)?;
    assert_str_eq!(result, version.to_string());
    Ok(())
}

/// Test that version_mod::get returns version with git info when git_info is true
#[test]
fn version_get_with_git_info() -> TestResult {
    let version = Version::new(1, 2, 3);
    let test_dir = TempDir::new()?;
    utils::setup_git_repo(test_dir.path(), Some(version.clone()));

    // Switch the directory with the .at-version file
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&test_dir)?;

    // Act
    let result = get(true)?;

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

}
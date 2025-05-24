use crate::git;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

const AT_VERSION_FILE: &str = ".at-version";

/// Read content from the specified version file.
fn read_at_version(path: &Path) -> Result<String> {
    let version = fs::read_to_string(path).context(format!(
        "Failed to read from version file: {}",
        path.display()
    ))?;
    Ok(version.trim().to_string())
}

/// Get current version, optionally including git information
///
/// If git_info is true, appends branch, hash and dirty status in the format:
/// version+branch-hash[-dirty]
///
/// # Arguments
/// * `path` - Path to the directory containing the version file
/// * `git_info` - Whether to include git information in the version string
pub fn get(path: impl AsRef<Path>, git_info: bool) -> Result<String> {
    let version_path = path.as_ref().join(AT_VERSION_FILE);
    let mut version = read_at_version(&version_path).context("Failed to read the version file")?;

    if git_info {
        let branch = git::get_branch(&path)?;
        let branch = branch.replace('_', "-");

        let hash = git::get_commit_hash(&path)?;

        let is_dirty = git::is_dirty(&path)?;
        let dirty = if is_dirty { "-dirty" } else { "" };

        version = format!("{version}+{branch}-{hash}{dirty}");
    }

    Ok(version)
}

#[cfg(test)]
mod tests {
    use assert_fs::TempDir;
    use pretty_assertions::assert_str_eq;
    use semver::Version;
    use test_utils;
    use testresult::TestResult;

    use super::*;

    /// Test that version_mod::get returns just the version when git_info is false
    #[test]
    fn version_get_without_git_info() -> TestResult {
        // Arrange
        let version = Version::new(1, 2, 3);
        let test_dir = TempDir::new()?;
        test_utils::setup_git_repo(test_dir.path(), Some(version.clone()));

        // Act
        let result = get(test_dir.path(), false)?;

        // Assert
        assert_str_eq!(result, version.to_string());
        Ok(())
    }

    /// Test that version_mod::get returns version with git info when git_info is true
    #[test]
    fn version_get_with_git_info() -> TestResult {
        let version = Version::new(1, 2, 3);
        let test_dir = TempDir::new()?;
        test_utils::setup_git_repo(test_dir.path(), Some(version.clone()));

        // Act
        let result = get(test_dir.path(), true)?;

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

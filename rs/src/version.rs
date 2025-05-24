use crate::git;
use anyhow::{Context, Result};
use semver::BuildMetadata;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fmt, fs};

const AT_VERSION_FILE: &str = ".at-version";
// Returns the path to the version file relative to the repository root
fn relative_version_file(path: impl AsRef<Path>) -> PathBuf {
    path.as_ref().join(AT_VERSION_FILE)
}

/// Read content from the specified version file.
fn read_at_version(path: &Path) -> Result<String> {
    let version = fs::read_to_string(path).context(format!(
        "Failed to read from version file: {}",
        path.display()
    ))?;
    Ok(version.trim().to_string())
}

#[derive(Debug)]
pub struct AtVersion {
    ver: semver::Version,
}

impl AtVersion {
    /// Construct an [AtVersion] from a path to a repository
    pub fn new(repo: impl AsRef<Path>) -> Result<Self> {
        let ver = read_at_version(&relative_version_file(repo))
            .context("Failed to read the version file")?;
        let ver = semver::Version::from_str(&ver)?;
        Ok(Self { ver })
    }

    /// Construct an [AtVersion] from a path to a repository including build metadata in the format `version+branch-hash[-dirty]`
    pub fn with_metadata(repo: impl AsRef<Path>) -> Result<Self> {
        let mut at_ver = Self::new(&repo)?;
        let branch = git::get_branch(&repo)?;
        let branch = branch.replace('_', "-");

        let hash = git::get_commit_hash(&repo)?;

        let is_dirty = git::is_dirty(&repo)?;
        let dirty = if is_dirty { "-dirty" } else { "" };

        let metadata = format!("{branch}-{hash}{dirty}");
        at_ver.ver.build = BuildMetadata::new(&metadata)?;
        Ok(at_ver)
    }
}

impl fmt::Display for AtVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ver)
    }
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
        let result = AtVersion::new(test_dir.path())?;

        // Assert
        assert_str_eq!(result.to_string(), version.to_string());
        Ok(())
    }

    /// Test that version_mod::get returns version with git info when git_info is true
    #[test]
    fn version_get_with_git_info() -> TestResult {
        let version = Version::new(1, 2, 3);
        let test_dir = TempDir::new()?;
        test_utils::setup_git_repo(test_dir.path(), Some(version.clone()));

        // Act
        let result = AtVersion::with_metadata(test_dir.path())?.to_string();

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

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
    use pretty_assertions::assert_str_eq;
    use semver::Version;
    use test_log::test;
    use test_utils::{self, testrepo::TestRepo};
    use testresult::TestResult;

    use super::*;

    /// Test that version_mod::get returns just the version when git_info is false
    #[test]
    fn version_get_without_git_info() -> TestResult {
        // Arrange
        let version = Version::new(1, 2, 3);
        let repo = TestRepo::builder().init(true).build();
        repo.init_at_version(&version);

        // Act
        let result = AtVersion::new(repo.path())?;

        assert_str_eq!(result.to_string(), version.to_string());
        Ok(())
    }

    /// Test that version_mod::get returns version with git info when git_info is true
    #[test]
    fn version_get_with_git_info() -> TestResult {
        let initial_branch_name = "master";
        let version = Version::new(1, 2, 3);
        let repo = TestRepo::builder()
            .init(true)
            .initial_branch_name(initial_branch_name)
            .build();
        repo.init_at_version(&version);
        let head_short_sha = repo.head_short_sha();
        let expected_version_str = format!("1.2.3+{initial_branch_name}-{head_short_sha}");

        // Act
        let get_version = AtVersion::with_metadata(repo.path())?.to_string();

        assert_str_eq!(get_version, expected_version_str);
        Ok(())
    }
}

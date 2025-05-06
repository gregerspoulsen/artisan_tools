use std::fs;
use std::path::Path;
use anyhow::{Result, Context};
use crate::git;

const AT_VERSION_FILE: &str = ".at-version";

/// Read content from specified at-version file.
/// If the path is None, it defaults to ".at-version".
fn read_at_version() -> Result<String> {
    let version = fs::read_to_string(Path::new(AT_VERSION_FILE))
        .context(format!("Failed to read from version file: {}", AT_VERSION_FILE))?;
    Ok(version.trim().to_string())
}

/// Get current version, optionally including git information
/// 
/// If git_info is true, appends branch, hash and dirty status in the format:
/// version+branch-hash[-dirty]
pub fn get(git_info: bool) -> Result<String> {
    let mut version = read_at_version()
        .context("Failed to read the version file")?;

    if git_info {
        // Get current branch
        let branch = git::get_branch(".")?;
        let branch = branch.replace('_', "-");

        // Get commit hash
        let hash = git::get_commit_hash(".")?;

        // Get clean status
        let is_clean = !git::get_status(".")?;
        let dirty = if is_clean { "" } else { "-dirty" };

        version = format!("{}+{}-{}{}", version, branch, hash, dirty);
    }

    Ok(version)
}

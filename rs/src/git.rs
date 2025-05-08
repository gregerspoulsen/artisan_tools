use anyhow::{Error, Result};
use std::path::Path;

/// Return name of the currently active git branch.
pub fn get_branch<P: AsRef<Path>>(path: P) -> Result<String> {
    // Find the repository by searching up through parent directories
    let repo = gix::discover(path)?;

    let head = repo
        .head_ref()?
        .ok_or_else(|| Error::msg("Failed to get HEAD reference"))?;
        
    let name = head.name().shorten();
    
    Ok(name.to_string())
}

/// Return the current commit hash in short format (first 7 characters).
pub fn get_commit_hash<P: AsRef<Path>>(path: P) -> Result<String> {
    // Find the repository by searching up through parent directories
    let repo = gix::discover(path)?;

    // Get the HEAD commit
    let mut head = repo.head()?;
    let head_commit = head.peel_to_commit_in_place()?;

    // Get the short hash (7 characters)
    let short_hash = head_commit.id().to_string()[..7].to_string();

    Ok(short_hash)
}

/// Return whether the git repository has uncommitted changes.
///
/// Returns true if there are any uncommitted changes
/// false if the working directory is clean.
pub fn is_dirty<P: AsRef<Path>>(path: P) -> Result<bool> {
    // Find the repository by searching up through parent directories
    let repo = gix::discover(path)?;
    Ok(repo.is_dirty()?)
}

use anyhow::{Result, Error};

/// Return name of the currently active git branch.
pub fn get_branch() -> Result<String> {
    // Find the repository by searching up through parent directories
    let repo = gix::discover(".")?;
    
    // Get the HEAD reference
    let head = repo.head_ref()?
        .ok_or_else(|| Error::msg("Failed to get HEAD reference"))?;
    
    // Get the branch name (shortened from refs/heads/name to just name)
    let name = head.name().shorten();
    
    // Convert to a regular String
    Ok(name.to_string())
}

/// Return the current commit hash in short format (first 7 characters).
pub fn get_commit_hash() -> Result<String> {
    // Find the repository by searching up through parent directories
    let repo = gix::discover(".")?;
    
    // Get the HEAD commit
    let mut head = repo.head()?;
    let head_commit = head.peel_to_commit_in_place()?;
    
    // Get the short hash (7 characters)
    let short_hash = head_commit.id().to_string()[..7].to_string();
    
    Ok(short_hash)
}

/// Return whether the git repository has uncommitted changes.
/// 
/// Returns true if there are any uncommitted changes (including untracked files),
/// false if the working directory is clean.
pub fn get_status() -> Result<bool> {
    // Find the repository by searching up through parent directories
    let repo = gix::discover(".")?;
    
    // Get the repository status
    let status = repo.is_dirty()?;
    
    // Check if the repository is dirty (has uncommitted changes)
    Ok(status)
}
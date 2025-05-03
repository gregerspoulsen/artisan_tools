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
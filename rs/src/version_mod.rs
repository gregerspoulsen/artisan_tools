use std::fs;
use std::path::Path;
use anyhow::{Result, Context};



/// Read content from specified at-version file.
/// If the path is None, it defaults to ".at-version".
pub fn read_at_version(path: Option<&str>) -> Result<String> {
    let default_path = ".at-version";
    let file_path = path.unwrap_or(default_path);
    let version = fs::read_to_string(Path::new(file_path))
        .context(format!("Failed to read from version file: {}", file_path))?;
    Ok(version)
}

/// Get current version
pub fn get() -> Result<String> {
    let version = read_at_version(None)
        .context("Failed to read the version file")?;
    Ok(version)
}

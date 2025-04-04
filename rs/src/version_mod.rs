use std::fs;
use std::path::Path;
use anyhow::{Result, Context};

//const AT_VERSION_FILE: &str = ".at-version";
const AT_VERSION_FILE: &str = "RELEASE";

/// Read content from specified at-version file.
/// If the path is None, it defaults to ".at-version".
fn read_at_version() -> Result<String> {
    let version = fs::read_to_string(Path::new(AT_VERSION_FILE))
        .context(format!("Failed to read from version file: {}", AT_VERSION_FILE))?;
    Ok(version)
}

/// Get current version
pub fn get() -> Result<String> {
    let version = read_at_version()
        .context("Failed to read the version file")?;
    // Here we should do some Git checks...
    Ok(version)
}

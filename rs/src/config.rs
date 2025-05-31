use anyhow::Result;
use config::{Config, ConfigError, File, FileFormat};
use serde::{Deserialize, Serialize};
use toml_edit::DocumentMut;

use crate::init::ProjectType;

const DEFAULT_CONFIG_TOML: &str = include_str!("../templates/default_at_config.toml");

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AtConfig {
    pub version: VersionConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VersionConfig {
    /// Files with the current project version (semantic version), updated when an `at merge` is performed.
    /// Example: link = ["pyproject.toml", "rs/Cargo.toml", "RELEASE"]
    /// This configuration would keep the version updated in 3 places:
    /// - The pyproject.toml at the project root
    /// - The Cargo.toml in the 'rs' subdirectory
    /// - the custom file named RELEASE, which would simply contain the version and nothing more
    pub link: Vec<String>,

    /// This table concerns the version with build metadata in the format:
    /// <SEMVER>+<BRANCH>-<GIT_SHORT_SHA>[-dirty]
    pub extended: VersionExtendedConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VersionExtendedConfig {
    /// Files where the version with the build metadata is written to as is
    /// (will truncate the file)
    pub raw: Vec<String>,
}

impl Default for AtConfig {
    fn default() -> Self {
        Self {
            version: VersionConfig {
                link: vec![],
                extended: VersionExtendedConfig { raw: vec![] },
            },
        }
    }
}

impl AtConfig {
    pub const DEFAULT_RAW_VERSION_FILE: &str = "VERSION";
    pub const NAME: &str = "at.toml";

    /// Customize the default config for a project type and optionally with a project file
    ///
    /// This is intended for first time initializing a config for a project
    pub fn customize_for(project: &ProjectType, project_file: Option<&str>) -> Result<DocumentMut> {
        let mut config: DocumentMut = AtConfig::default_config_template().parse()?;
        match project {
            ProjectType::Go | ProjectType::ManualSetup => Ok(config),
            ProjectType::Rust => {
                let mut arr = toml_edit::Array::new();
                arr.push("Cargo.toml");
                config["version"]["link"] = arr.into();
                Ok(config)
            }
            ProjectType::JavaScriptOrTypeScript | ProjectType::Python => {
                if let Some(project_file) = project_file {
                    let mut arr = toml_edit::Array::new();
                    arr.push(project_file);
                    config["version"]["link"] = arr.into();
                    Ok(config)
                } else {
                    // In this case we don't know which project file to add
                    Ok(config)
                }
            }
        }
    }

    /// Default config, agnostic to the project type
    pub fn default_config_template() -> &'static str {
        DEFAULT_CONFIG_TOML
    }

    /// Load configuration from a TOML file
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name(path).format(FileFormat::Toml))
            .build()?;

        config.try_deserialize()
    }

    /// Load configuration from a TOML string
    pub fn from_toml_str(toml_str: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::from_str(toml_str, FileFormat::Toml))
            .build()?;

        config.try_deserialize()
    }

    /// Create configuration with defaults and then overlay from file if it exists
    pub fn load_with_defaults(config_path: Option<&str>) -> Result<Self, ConfigError> {
        let mut builder = Config::builder();

        // Start with defaults
        builder = builder.set_default("version.link", Vec::<String>::new())?;
        builder = builder.set_default("version.extended.raw", Vec::<String>::new())?;

        // Overlay with file if provided and exists
        if let Some(path) = config_path {
            builder = builder.add_source(
                File::with_name(path)
                    .format(FileFormat::Toml)
                    .required(false), // Don't fail if file doesn't exist
            );
        }

        let config = builder.build()?;
        config.try_deserialize()
    }

    /// Convert back to TOML string
    pub fn to_toml_string(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_str_eq;
    use testresult::TestResult;

    use super::*;

    #[test]
    fn test_load_from_toml_string() {
        let toml_str = r#"
[version]
link = ["Cargo.toml", "RELEASE"]

[version.extended]
raw = ["VERSION"]
"#;

        let config = AtConfig::from_toml_str(toml_str).unwrap();
        assert_eq!(config.version.link, vec!["Cargo.toml", "RELEASE"]);
        assert_eq!(config.version.extended.raw, vec!["VERSION"]);
    }

    #[test]
    fn test_load_from_toml_string_version_table_only_errors() {
        let toml_str = r#"
[version]
link = ["Cargo.toml", "RELEASE"]
"#;

        let config = AtConfig::from_toml_str(toml_str).unwrap_err();
        assert_str_eq!(
            config.to_string(),
            "missing field `extended` for key `version`"
        );
    }

    #[test]
    fn test_customize_for_rust() -> TestResult {
        let toml_document = AtConfig::customize_for(&ProjectType::Rust, None)?;
        let config: AtConfig = AtConfig::from_toml_str(&toml_document.to_string())?;

        assert_eq!(config.version.link, vec!["Cargo.toml"]);
        Ok(())
    }

    #[test]
    fn test_to_toml_string_roundtrip() -> TestResult {
        let toml_document = AtConfig::customize_for(&ProjectType::Python, Some("pyproject.toml"))?;
        let config = AtConfig::from_toml_str(&toml_document.to_string())?;
        let toml_output = config.to_toml_string()?;

        // Should be able to parse it back
        let parsed = AtConfig::from_toml_str(&toml_output)?;
        assert_eq!(parsed.version.link, vec!["pyproject.toml"]);
        Ok(())
    }

    #[test]
    fn test_load_with_defaults() -> TestResult {
        let config = AtConfig::load_with_defaults(None)?;
        assert!(config.version.link.is_empty());
        assert!(config.version.extended.raw.is_empty());
        Ok(())
    }
}

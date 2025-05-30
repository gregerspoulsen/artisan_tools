use std::fs;
use std::io::Write;
use std::path::Path;

// Default TEMPLATE content
const TEMPLATE: &str = include_str!("../templates/at-changeset-template.yaml");

#[derive(Debug, Clone, Copy, Default, PartialEq, clap::ValueEnum)]
pub enum BumpTarget {
    Major,
    Minor,
    #[default]
    Patch,
    Unreleased,
}

impl BumpTarget {
    pub fn as_str(&self) -> &'static str {
        match self {
            BumpTarget::Major => "MAJOR",
            BumpTarget::Minor => "MINOR",
            BumpTarget::Patch => "PATCH",
            BumpTarget::Unreleased => "UNRELEASED",
        }
    }
}

/// Get the content of the changeset template with the target replaced.
pub fn get_template_content(target: BumpTarget) -> String {
    TEMPLATE.replace("target: patch", &format!("target: {}", target.as_str()))
}

pub fn create_changeset_template(
    path: impl AsRef<Path>,
    target: BumpTarget,
) -> std::io::Result<()> {
    let content = get_template_content(target);
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::fs;
    use tempfile::tempdir;
    use yaml_rust2::YamlLoader;

    #[test]
    fn test_get_template_content() {
        // Test all BumpTarget variants
        let targets = [
            BumpTarget::Major,
            BumpTarget::Minor,
            BumpTarget::Patch,
            BumpTarget::Unreleased,
        ];

        for target in targets {
            // Generate content with the function under test
            let content = get_template_content(target);

            // Parse the YAML content
            let docs = YamlLoader::load_from_str(&content).expect("Failed to parse YAML");
            let root = &docs[0];

            // Extract the target value from the YAML
            assert_eq!(root["target"].as_str(), Some(target.as_str()));

            assert!(
                !root["changes"].is_badvalue(),
                "YAML is missing the top-level `changes` section"
            );
        }
    }

    #[test]
    fn test_create_changeset_template() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("at-changeset");

        create_changeset_template(&file_path, BumpTarget::Minor).unwrap();

        assert!(file_path.exists());
        let content = fs::read_to_string(file_path).unwrap();
        let expected = get_template_content(BumpTarget::Minor);

        assert_eq!(content, expected);
    }
}

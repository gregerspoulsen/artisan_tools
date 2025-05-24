use std::fs;
use std::io::Write;
use std::path::Path;

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

pub fn create_changeset_template(
    path: impl AsRef<Path>,
    target: BumpTarget,
) -> std::io::Result<()> {
    let template = include_str!("../templates/at-changeset-template.txt");
    let content = template.replace("TARGET: patch", &format!("TARGET: {}", target.as_str()));

    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_create_changeset_template() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("at-changeset");

        create_changeset_template(&file_path, BumpTarget::Minor).unwrap();

        assert!(file_path.exists());
        let content = fs::read_to_string(file_path).unwrap();
        assert!(content.contains("TARGET: MINOR"));
        assert!(content.contains("CHANGELOG:"));
        assert!(content.contains("### Added"));
    }
}

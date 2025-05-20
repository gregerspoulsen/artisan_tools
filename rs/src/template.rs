use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, clap::ValueEnum)]
pub enum BumpTarget {
    Major,
    Minor,
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

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "major" => Some(Self::Major),
            "minor" => Some(Self::Minor),
            "patch" => Some(Self::Patch),
            "unreleased" => Some(Self::Unreleased),
            _ => None,
        }
    }
}

pub fn create_changeset_template<P: AsRef<Path>>(path: P, target: BumpTarget) -> std::io::Result<()> {
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

    #[test]
    fn test_bump_target_from_str() {
        assert_eq!(BumpTarget::from_str("major"), Some(BumpTarget::Major));
        assert_eq!(BumpTarget::from_str("MINOR"), Some(BumpTarget::Minor));
        assert_eq!(BumpTarget::from_str("patch"), Some(BumpTarget::Patch));
        assert_eq!(BumpTarget::from_str("unreleased"), Some(BumpTarget::Unreleased));
        assert_eq!(BumpTarget::from_str("invalid"), None);
    }
}
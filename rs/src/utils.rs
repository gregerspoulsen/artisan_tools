use std::{io, path::PathBuf};

/// Newtype wrapping a path that is supposed to point at the project root
pub struct ProjectRootDir(pub PathBuf);

impl TryFrom<PathBuf> for ProjectRootDir {
    type Error = io::Error;

    fn try_from(p: PathBuf) -> std::result::Result<Self, Self::Error> {
        let abs_path = p.canonicalize()?;
        Ok(Self(abs_path))
    }
}

impl ProjectRootDir {
    pub fn contains_file(&self, name: &str) -> bool {
        self.0.join(name).is_file()
    }
}

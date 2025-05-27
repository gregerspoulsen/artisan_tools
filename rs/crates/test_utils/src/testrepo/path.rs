use std::{
    fmt,
    path::{Path, PathBuf},
};

use tempfile::TempDir;

// An absolute path relative to the test repo
pub(crate) struct ResolvedPath(pub(crate) PathBuf);

impl fmt::Display for ResolvedPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

/// Resolves paths to absolute paths relative to the repo
#[derive(Debug)]
pub(crate) struct PathResolver {
    repo: PathBuf,
}

impl PathResolver {
    // Create a new [PathResolver]
    pub(crate) fn new(repo: &TempDir) -> Self {
        Self {
            repo: repo.path().canonicalize().unwrap(),
        }
    }

    // Resolve a path to a [ResolvedPath] which is an absolute path relative to the test repo
    //
    // e.g. if you supply "dummy.txt" it will ensure that "dummy.txt" points to a path
    // within the test repo
    pub(crate) fn resolve(&self, p: impl AsRef<Path>) -> ResolvedPath {
        if !p.as_ref().starts_with(&self.repo) {
            ResolvedPath(self.repo.join(p))
        } else {
            ResolvedPath(p.as_ref().to_path_buf())
        }
    }
}

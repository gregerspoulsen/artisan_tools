use tempfile::TempDir;

use crate::{
    prelude::{LocalRepo, TestRepo, WithRemote},
    testrepo::path::PathResolver,
    GitRepo,
};

/// Represents a remote repository within the test environment
#[derive(Debug)]
pub struct TestRemote {
    pub(crate) default_branch: Option<String>,
    pub(crate) tempdir: TempDir,
    pub(crate) resolver: PathResolver,
}

impl TestRemote {
    /// Create a [TestRemote] for testing with a remote repository
    pub fn new() -> Self {
        let tempdir = TempDir::new().expect("Failed to create temp directory");
        let resolver = PathResolver::new(&tempdir);
        Self {
            tempdir,
            resolver,
            default_branch: None,
        }
    }
}

impl Default for TestRemote {
    fn default() -> Self {
        Self::new()
    }
}

/// A remote (bare) repo is what is hosted on e.g. GitHub, the repo(s) we push to
pub trait RemoteRepo: GitRepo {
    /// Returns the initial branch the repo was set up with
    fn default_branch(&self) -> &str;

    /// Initialize the remote repo as a bare git repository
    fn init(&mut self, default_branch: &str);
}

/// Utility for settings up the remote repo for various scenarios
pub trait RemoteRepoSetup: RemoteRepo {
    /// Set up the remote with an initial commit (requires a temporary working copy)
    /// This creates a temporary non-bare repo, makes initial commits, then pushes to the bare remote
    fn create_initial_commit(&self) {
        let work_repo = TestRepo::builder()
            .init(true)
            .initial_branch_name(self.default_branch())
            .git_user_name("remote setup")
            .git_user_mail("remote@example.com")
            .build();

        work_repo.create_stage_commit("README.md", Some("# Test Repo\n"), "Initial commit");
        work_repo.add_remote("origin", &self.path().to_string_lossy());
        work_repo.push("origin", self.default_branch());
    }

    /// Add tags to the repo
    fn add_tags(&self, tags: &[(&str, &str)]) {
        if tags.is_empty() {
            return;
        }

        let work_repo = TestRepo::builder().init(false).build();

        work_repo.clone(&self.path().to_string_lossy(), ".");
        work_repo.config_git_user("remote tag setup", "remote_tags@example.com");

        // Create tags
        for (tag_name, tag_message) in tags {
            work_repo.create_annotated_tag(tag_name, tag_message);
        }
        work_repo.push_tags();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use testresult::TestResult;

    #[test]
    fn test_build_remote_test_repo() -> TestResult {
        let expected_tags = &[
            ("v0.1.0", "initial release"),
            ("v1.0.0", "perfect release!"),
        ];

        let mut remote = TestRemote::new();
        remote.init("master");
        assert!(remote.list_tags().is_empty());

        remote.create_initial_commit();
        remote.add_tags(expected_tags);

        let tags = remote.list_tags();
        eprintln!("{tags:?}");

        assert_eq!(&tags[0], expected_tags[0].0);
        assert_eq!(&tags[1], expected_tags[1].0);
        assert_eq!(tags.len(), 2);

        Ok(())
    }
}

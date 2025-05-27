use std::{
    io::Write as _,
    path::{Path, PathBuf},
};

use bon::bon;
use path::PathResolver;
use semver::Version;
use tempfile::TempDir;

use crate::{
    testremote::{RemoteRepo as _, TestRemote},
    GitRepo,
};

pub(crate) mod path;

/// Utility for creating temporary git repositories during testing
#[derive(Debug)]
pub struct TestRepo {
    pub(crate) tempdir: TempDir,
    pub(crate) resolver: PathResolver,
    pub(crate) remote: Option<TestRemote>,
}

#[bon]
impl TestRepo {
    /// Initialize a [TestRepo]
    #[builder]
    pub fn new(
        #[builder(default = false)] init: bool,
        #[builder(default = "test user")] git_user_name: &str,
        #[builder(default = "test@example.com")] git_user_mail: &str,
        #[builder(default = "master")] initial_branch_name: &str,
        #[builder(default = false)] with_remote: bool,
    ) -> Self {
        let tempdir = TempDir::new().expect("Failed to create temp directory");
        let resolver = PathResolver::new(&tempdir);
        let mut remote = None;
        if with_remote {
            let mut r = TestRemote::new();
            r.init(initial_branch_name);
            remote = Some(r);
        }
        let test_repo = Self {
            tempdir,
            resolver,
            remote,
        };

        if init {
            test_repo.init(initial_branch_name);
            test_repo.config_git_user(git_user_name, git_user_mail);
        }

        // Set up remote if it exists
        if let Some(ref remote) = test_repo.remote {
            test_repo.add_remote("origin", remote.path().to_str().unwrap());
        }

        test_repo
    }
}

/// A local repo is what a developer usually works directly on, i.e. a non-bare repository
pub trait LocalRepo: GitRepo {
    /// Return the test repo as a [gix::Repository]
    ///
    /// Requires the test repo to be initialized
    fn as_gix_repo(&self) -> gix::Repository {
        gix::discover(self.path()).expect("Could not get test repo as a gix repository")
    }

    // Initialize git repository
    fn init(&self, default_branch: &str) {
        self.git_assert_success(["init", &format!("--initial-branch={default_branch}")]);
    }

    // Configure git user for the test
    fn config_git_user(&self, name: &str, mail: &str) {
        self.git_assert_success(["config", "user.name", name]);
        self.git_assert_success(["config", "user.email", mail]);
    }

    /// Create a commit
    fn commit(&self, msg: &str) {
        self.git_assert_success(["commit", "-m", msg]);
    }

    /// Create a commit and allow it to be empty
    fn commit_empty(&self, msg: &str) {
        self.git_assert_success(["commit", "--allow-empty", "-m", msg]);
    }

    /// Checks out the parent commit, fails if the HEAD commit has no parent
    ///
    /// This will bring the head in DETACHED state. The same thing can be achieved with
    /// e.g. `git checkout <SHA>`
    fn checkout_parent(&self) {
        self.git_assert_success(["checkout", "HEAD~1"]);
    }

    /// Returns the trimmed output of `git rev-parse --short HEAD`
    fn head_short_sha(&self) -> String {
        let rev_parse_output = self
            .git_assert_success(["rev-parse", "--short", "HEAD"])
            .stdout;
        let head_hash_output = String::from_utf8(rev_parse_output).unwrap();
        head_hash_output.trim().to_owned()
    }

    /// Stage a file in the test repo, errors if it doesn't already exist
    fn stage(&self, file: impl AsRef<Path>) {
        let resolved = self.resolve_path(file).to_string();
        self.git_assert_success(["add", &resolved]);
    }

    /// Create a file in the test repo
    fn create(&self, file: impl AsRef<Path>, contents: Option<&str>) {
        self.write_to_repo(file, contents);
    }

    /// Create and add a file
    fn create_and_stage(&self, file: impl AsRef<Path>, contents: Option<&str>) {
        self.create(&file, contents);
        self.stage(file);
    }

    /// Create a file, stage, and commit it
    fn create_stage_commit(
        &self,
        file: impl AsRef<Path>,
        contents: Option<&str>,
        commit_msg: &str,
    ) {
        self.create_and_stage(file, contents);
        self.commit(commit_msg);
    }

    /// Create, add, & commit a `.gitignore`
    fn init_gitignore(&self) {
        self.create_stage_commit(".gitignore", None, "add .gitignore");
    }

    /// Add a line to the [TestRepo] .gitignore
    fn add_to_gitignore(&self, line: impl AsRef<str>) {
        // Ensure the line ends with a newline
        let line = line.as_ref();
        let line = if line.ends_with("\n") {
            line.to_owned()
        } else {
            format!("{line}\n")
        };
        let mut gitignore = self.open_file_append(".gitignore");
        gitignore
            .write_all(line.as_bytes())
            .expect("Failed writing to .gitinore");
    }

    /// Stage and commit .gitignore
    fn stage_commit_gitignore(&self, commit_msg: &str) {
        self.stage(".gitignore");
        self.commit(commit_msg);
    }

    /// Create, add, & commit the '.at-version' file with the given [Version]
    fn init_at_version(&self, version: &Version) {
        self.create_stage_commit(
            PathBuf::from(".at-version"),
            Some(&version.to_string()),
            "add at-version",
        );
    }

    /// Create a lightweight tag
    fn create_tag(&self, name: &str) {
        self.git_assert_success(["tag", name]);
    }

    /// Create an annotated tag
    fn create_annotated_tag(&self, name: &str, msg: &str) {
        self.git_assert_success(["tag", "-a", name, "-m", msg]);
    }
}

/// Operations on a remote repository
pub trait WithRemote: LocalRepo {
    /// Get a reference to the remote repository (panics if no remote is set up)
    ///
    /// NOTE: Typically you want to add a utility function instead of accessing the remote directly
    fn remote(&self) -> &TestRemote;

    /// Add a remote repository
    fn add_remote(&self, name: &str, url: &str) {
        self.git_assert_success(["remote", "add", name, url]);
    }

    /// Clone a repo to a path
    fn clone(&self, url: &str, path: impl AsRef<Path>) {
        self.git_assert_success(["clone", url, &path.as_ref().to_string_lossy()]);
    }

    /// Push to remote
    fn push(&self, remote: &str, branch: &str) {
        self.git_assert_success(["push", remote, branch]);
    }

    /// Push tags to remote origin
    fn push_tags(&self) {
        self.git_assert_success(["push", "--tags"]);
    }

    /// Push with upstream tracking
    fn push_set_upstream(&self, remote: &str, branch: &str) {
        self.git_assert_success(["push", "-u", remote, branch]);
    }

    /// Fetch
    fn fetch(&self) {
        self.git_assert_success(["fetch"]);
    }

    /// Pull from remote
    fn pull(&self, remote: &str, branch: &str) {
        self.git_assert_success(["pull", remote, branch]);
    }
}

#[cfg(test)]
mod tests {
    use testresult::TestResult;

    use crate::prelude::RemoteRepoSetup;

    use super::*;

    #[test]
    fn test_build_test_repo() -> TestResult {
        let testrepo = TestRepo::builder().build();
        testrepo.init("custom-branch-name");

        let stdout = testrepo.git(["status"])?.stdout;
        let stdout = String::from_utf8(stdout)?;
        eprintln!("---\n{stdout}\n---",);

        assert!(stdout.contains("custom-branch-name"));
        Ok(())
    }

    #[test]
    fn test_build_test_repo_with_remote() -> TestResult {
        let testrepo = TestRepo::builder().with_remote(true).init(true).build();

        testrepo.remote().create_initial_commit();
        let tag = "v0.1.0";
        testrepo.remote().add_tag(tag, None);

        let tags = testrepo.list_tags();
        assert!(tags.is_empty());

        testrepo.fetch();
        let tags = testrepo.list_tags();
        assert_eq!(tags, vec![tag]);

        Ok(())
    }
}

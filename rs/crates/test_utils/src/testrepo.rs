use bon::bon;
use semver::Version;
use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    process::{Command, Output},
};
use tempfile::TempDir;

#[derive(Debug)]
pub struct TestRepo {
    tempdir: TempDir,
    version: Version,
    init: bool,
    commit_file: Option<(PathBuf, String)>,
    git_user_name: String,
    git_user_mail: String,
}

#[bon]
impl TestRepo {
    /// Default version to use when setting up a git repository
    pub const DEFAULT_VERSION: Version = Version::new(0, 1, 0);

    #[builder]
    pub fn new(
        #[builder(default = TestRepo::DEFAULT_VERSION)] version: Version,
        #[builder(default = false)] init: bool,
        commit_file: Option<(PathBuf, String)>,
        #[builder(default = "test user")] git_user_name: &str,
        #[builder(default = "test@example.com")] git_user_mail: &str,
        #[builder(default = "master")] initial_branch_name: &str,
    ) -> Self {
        let test_repo = Self {
            tempdir: TempDir::new().expect("Failed to create temp directory"),
            version,
            init,
            commit_file,
            git_user_name: git_user_name.to_owned(),
            git_user_mail: git_user_mail.to_owned(),
        };

        if init {
            test_repo.init(initial_branch_name);
        }

        test_repo.config_git_user(git_user_name, git_user_mail);

        if let Some((file, contents)) = &test_repo.commit_file {
            test_repo.create_add_commit_file(file, contents, "Initial commit");
        }

        test_repo
    }

    /// Path to the test repo
    pub fn path(&self) -> &Path {
        self.tempdir.path()
    }

    // Initialize git repository
    pub fn init(&self, default_branch: &str) {
        self.git(["init", &format!("--initial-branch={default_branch}")])
            .expect("Failed to initialize git repository");
    }

    // Configure git user for the test
    pub fn config_git_user(&self, name: &str, mail: &str) {
        self.git(["config", "user.name", name])
            .expect("Failed to configure git user name");
        self.git(["config", "user.email", mail])
            .expect("Failed to configure git user email");
    }

    pub fn commit(&self, msg: &str) {
        self.git(["commit", "-m", msg]).expect("Failed to commit");
    }

    // Resolve the path to an absolute path relative to the test repo
    //
    // e.g. if you supply "dummy.txt" it will ensure that "dummy.txt" points to a path
    // within the test repo
    fn resolve_path(&self, p: impl AsRef<Path>) -> PathBuf {
        let abs_repo_p = self.path().canonicalize().unwrap();
        if !p.as_ref().starts_with(&abs_repo_p) {
            abs_repo_p.join(p)
        } else {
            p.as_ref().to_path_buf()
        }
    }

    // Same as `resolve_path` but returns it as a String
    fn resolve_path_and_to_string(&self, p: impl AsRef<Path>) -> String {
        self.resolve_path(p).display().to_string()
    }

    /// Add a file
    pub fn add_file(&self, file: &Path) {
        let resolved = self.resolve_path_and_to_string(file);
        self.git(["add", &resolved]);
    }

    /// Create and add a file
    pub fn create_add_file(&self, file: &Path, contents: &str) {
        let resolved = self.resolve_path(file);
        fs::write(&resolved, contents).expect("Failed to write file");
        self.add_file(&resolved);
    }

    /// Create a file and commit it
    pub fn create_add_commit_file(&self, file: &Path, contents: &str, commit_msg: &str) {
        self.create_add_file(file, contents);
        self.commit(commit_msg);
    }

    pub fn init_version(&self, version: Option<Version>) {
        self.create_add_commit_file(
            &PathBuf::from(".at-version"),
            &version.unwrap_or(Self::DEFAULT_VERSION).to_string(),
            "add at-version",
        );
    }

    pub fn git<I, S>(&self, args: I) -> io::Result<Output>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        Command::new("git")
            .args(args)
            .current_dir(self.path())
            .output()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_test_repo() {
        let testrepo = TestRepo::builder().build();
        testrepo.init("custom-branch-name");
        assert_eq!(testrepo.git_user_name, "test user");
        let stdout = testrepo.git(["status"]).unwrap().stdout;
        let stdout = String::from_utf8(stdout).unwrap();
        eprintln!("---\n{stdout}\n---",);

        assert!(stdout.contains("custom-branch-name"));
    }
}

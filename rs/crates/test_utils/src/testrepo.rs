use bon::bon;
use semver::Version;
use std::{
    ffi::OsStr,
    fmt, fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, Output},
};
use tempfile::TempDir;

// An absolute path relative to the test repo
struct ResolvedPath(PathBuf);

impl fmt::Display for ResolvedPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

/// Resolves paths to absolute paths relative to the repo
#[derive(Debug)]
struct PathResolver {
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
    fn resolve(&self, p: impl AsRef<Path>) -> ResolvedPath {
        if !p.as_ref().starts_with(&self.repo) {
            ResolvedPath(self.repo.join(p))
        } else {
            ResolvedPath(p.as_ref().to_path_buf())
        }
    }
}

/// Utility for creating temporary git repositories during testing
#[derive(Debug)]
pub struct TestRepo {
    tempdir: TempDir,
    resolver: PathResolver,
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
    ) -> Self {
        let tempdir = TempDir::new().expect("Failed to create temp directory");
        let resolver = PathResolver::new(&tempdir);
        let test_repo = Self { tempdir, resolver };

        if init {
            test_repo.init(initial_branch_name);
        }

        test_repo.config_git_user(git_user_name, git_user_mail);

        test_repo
    }

    /// Path to the test repo
    pub fn path(&self) -> &Path {
        self.tempdir.path()
    }

    /// Return the test repo as a [gix::Repository]
    ///
    /// Requires the test repo to be initialized
    pub fn as_gix_repo(&self) -> gix::Repository {
        gix::discover(self.path()).expect("Could not get test repo as a gix repository")
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

    /// Create a commit
    pub fn commit(&self, msg: &str) {
        self.git(["commit", "-m", msg]).expect("Failed to commit");
    }

    /// Create a commit and allow it to be empty
    pub fn commit_empty(&self, msg: &str) {
        self.git(["commit", "--allow-empty", "-m", msg])
            .expect("Failed to commit");
    }

    /// Checks out the parent commit, fails if the HEAD commit has no parent
    ///
    /// This will bring the head in DETACHED state. The same thing can be achieved with
    /// e.g. `git checkout <SHA>`
    pub fn checkout_parent(&self) {
        self.git(["checkout", "HEAD~1"])
            .expect("Failed checking out parent commit");
    }

    /// Returns the trimmed output of `git rev-parse --short HEAD`
    pub fn head_short_sha(&self) -> String {
        let rev_parse_output = self
            .git(["rev-parse", "--short", "HEAD"])
            .expect("Git command failed")
            .stdout;
        let head_hash_output = String::from_utf8(rev_parse_output).unwrap();
        head_hash_output.trim().to_owned()
    }

    /// Stage a file in the test repo, errors if it doesn't already exist
    pub fn stage(&self, file: impl AsRef<Path>) {
        let resolved = self.resolver.resolve(file).to_string();
        self.git(["add", &resolved])
            .expect("Failed to git add file");
    }

    /// Create a file in the test repo
    pub fn create(&self, file: impl AsRef<Path>, contents: Option<&str>) {
        let resolved = self.resolver.resolve(file);
        self.write_to_repo(resolved, contents);
    }

    // Open a file in the test repo, takes a [ResolvedPath] for safety
    #[builder]
    fn open_file(
        &self,
        #[builder(start_fn)] file: impl AsRef<Path>,
        #[builder(default = true)] read: bool,
        #[builder(default = false)] write: bool,
        #[builder(default = false)] append: bool,
        #[builder(default = false)] truncate: bool,
    ) -> fs::File {
        let resolved = self.resolver.resolve(file);
        fs::OpenOptions::new()
            .read(read)
            .write(write)
            .append(append)
            .truncate(truncate)
            .open(resolved.0)
            .expect("Failed to open file")
    }

    // Write a file to the test repo, takes a [ResolvedPath] for safety
    fn write_to_repo(&self, file: ResolvedPath, contents: Option<&str>) {
        fs::write(file.0, contents.unwrap_or("")).expect("Failed to write file");
    }

    /// Create and add a file
    pub fn create_and_stage(&self, file: impl AsRef<Path>, contents: Option<&str>) {
        self.create(&file, contents);
        self.stage(file);
    }

    /// Create a file, stage, and commit it
    pub fn create_stage_commit(
        &self,
        file: impl AsRef<Path>,
        contents: Option<&str>,
        commit_msg: &str,
    ) {
        self.create_and_stage(file, contents);
        self.commit(commit_msg);
    }

    /// Create, add, & commit a `.gitignore`
    pub fn init_gitignore(&self) {
        self.create_stage_commit(".gitignore", None, "add .gitignore");
    }

    /// Add a line to the [TestRepo] .gitignore
    pub fn add_to_gitignore(&self, line: impl AsRef<str>) {
        // Ensure the line ends with a newline
        let line = line.as_ref();
        let line = if line.ends_with("\n") {
            line.to_owned()
        } else {
            format!("{line}\n")
        };
        let mut gitignore = self.open_file(".gitignore").write(true).append(true).call();
        gitignore
            .write_all(line.as_bytes())
            .expect("Failed writing to .gitinore");
    }

    /// Stage and commit .gitignore
    pub fn stage_commit_gitignore(&self, commit_msg: &str) {
        self.stage(".gitignore");
        self.commit(commit_msg);
    }

    /// Create, add, & commit the '.at-version' file with the given [Version]
    pub fn init_at_version(&self, version: &Version) {
        self.create_stage_commit(
            PathBuf::from(".at-version"),
            Some(&version.to_string()),
            "add at-version",
        );
    }

    /// Run `git` in the test repo
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
    use testresult::TestResult;

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
}

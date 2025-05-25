pub mod prelude;
pub mod testremote;
pub mod testrepo;

use std::{
    ffi::OsStr,
    fs,
    io::{self},
    path::Path,
    process::{Command, Output},
};

use testremote::{RemoteRepo, RemoteRepoSetup, TestRemote};
use testrepo::{path::ResolvedPath, LocalRepo, TestRepo, WithRemote};

/// Trait for functions that should not be accessible outside the crate
pub(crate) trait Internals {
    /// Resolve a path to a path relative to the repo
    fn resolve_path(&self, p: impl AsRef<Path>) -> ResolvedPath;

    /// Write a file to the test repo
    fn write_to_repo(&self, file: impl AsRef<Path>, contents: Option<&str>) {
        let resolved = self.resolve_path(file);
        fs::write(resolved.0, contents.unwrap_or("")).expect("Failed to write file");
    }

    // Open a file in the repo for appending
    fn open_file_append(&self, file: impl AsRef<Path>) -> fs::File {
        let resolved = self.resolve_path(file);
        fs::OpenOptions::new()
            .read(true)
            .append(true)
            .truncate(false)
            .open(resolved.0)
            .expect("Failed to open file")
    }
}

/// The basics of a [GitRepo]
#[allow(
    private_bounds,
    reason = "By design we don't want users to use the function defined in the Internals"
)]
pub trait GitRepo: Internals {
    /// Path to the repo
    fn path(&self) -> &Path;

    /// List all tags in the repo, ordered with oldest first
    fn list_tags(&self) -> Vec<String> {
        let output = self.git_assert_success(["tag", "--list"]);
        String::from_utf8(output.stdout)
            .expect("Invalid UTF-8 in tag list")
            .lines()
            .map(|line| line.to_owned())
            .collect()
    }

    /// Run `git` in the repo
    fn git<I, S>(&self, args: I) -> io::Result<Output>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        Command::new("git")
            .args(args)
            .current_dir(self.path())
            .output()
    }

    /// Run `git` in the repo and panic if the command didn't return success exit code
    ///
    /// Ensures that all command output is printed in case of failure
    fn git_assert_success<I, S>(&self, args: I) -> Output
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr> + Clone + std::fmt::Debug,
    {
        let args: Vec<_> = args.into_iter().collect();
        let cloned_args = args.clone();
        let mut cmd = Command::new("git");
        cmd.args(args).current_dir(self.path());
        let out = cmd.output().expect("Failed executing command");
        if !out.status.success() {
            // generate debug info before panicking
            let mut full_cmd = String::from("git");
            for a in cloned_args {
                full_cmd.push(' ');
                full_cmd.push_str(a.as_ref().to_string_lossy().into_owned().as_str());
            }
            eprintln!("| $ '{full_cmd}' failed");
            let exit_code = out.status.code();
            let stdout_str = String::from_utf8_lossy(&out.stdout);
            let stderr_str = String::from_utf8_lossy(&out.stderr);
            if stdout_str.is_empty() {
                eprintln!("(stdout empty)");
            } else {
                eprintln!("-- stdout --");
                eprintln!("{stdout_str}");
                eprintln!("-- end of stdout --");
            }
            if stderr_str.is_empty() {
                eprintln!("(stderr empty")
            } else {
                eprintln!("-- stderr --");
                eprintln!("{stderr_str}");
                eprintln!("-- end of stderr --");
            }

            panic!(
                "'{full_cmd}' failed with exit code {exit_code:?}. Current dir: {}",
                self.path().display()
            );
        }
        out
    }
}

impl GitRepo for TestRepo {
    fn path(&self) -> &Path {
        self.tempdir.path()
    }
}

impl GitRepo for TestRemote {
    fn path(&self) -> &Path {
        self.tempdir.path()
    }
}

impl Internals for TestRepo {
    fn resolve_path(&self, p: impl AsRef<Path>) -> ResolvedPath {
        self.resolver.resolve(p)
    }
}

impl Internals for TestRemote {
    fn resolve_path(&self, p: impl AsRef<Path>) -> ResolvedPath {
        self.resolver.resolve(p)
    }
}

impl WithRemote for TestRepo {
    fn remote(&self) -> &TestRemote {
        self.remote.as_ref().expect("No remote repo set up")
    }
}

impl LocalRepo for TestRepo {}

impl RemoteRepo for TestRemote {
    fn default_branch(&self) -> &str {
        self.default_branch
            .as_deref()
            .expect("No default branch, did you forget to initialize the repo?")
    }

    fn init(&mut self, default_branch: &str) {
        self.default_branch = Some(default_branch.to_owned());
        self.git_assert_success([
            "init",
            "--bare",
            &format!("--initial-branch={default_branch}"),
        ]);
    }
}
impl RemoteRepoSetup for TestRemote {}

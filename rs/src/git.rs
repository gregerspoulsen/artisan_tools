use anyhow::{bail, Result};
use gix::{features::progress, Repository};
use std::path::Path;

/// Return name of the currently active git branch.
pub fn get_branch(path: impl AsRef<Path>) -> Result<String> {
    // Find the repository by searching up through parent directories
    let repo = gix::discover(path)?;

    let head = repo.head()?;

    let name = match head.kind {
        gix::head::Kind::Symbolic(reference) => reference.name.shorten().to_string(),
        gix::head::Kind::Detached {
            target: _,
            peeled: _,
        } => todo!("Handle detached HEAD state"),
        gix::head::Kind::Unborn(full_name) => full_name.shorten().to_string(),
    };
    Ok(name)
}

/// Return the current commit hash in short format.
///
/// The minimum length is 4, the default is the effective value of the `core.abbrev` configuration variable (see <https://git-scm.com/docs/git-config>)
pub fn get_commit_hash(path: impl AsRef<Path>) -> Result<String> {
    // Find the repository by searching up through parent directories
    let repo = gix::discover(path)?;

    // Get the HEAD commit
    let mut head = repo.head()?;
    let head_commit = head.peel_to_commit_in_place()?;

    // Shorten the hash as defined by `core.abbrev`
    let short_hash = head_commit.id().shorten()?.to_string();

    Ok(short_hash)
}

/// Return whether the git repository has uncommitted changes.
///
/// We define a dirty as a repo has:
/// - Staged changes
/// - Untracked and unignored files
///
/// Additionally we don't consider a repo dirt if it fulfills the above and the repo:
/// - Has no commits
/// - Has untracked but ignored file(s)
/// - Has changes to ignored file(s)
pub fn is_dirty(path: impl AsRef<Path>) -> Result<bool> {
    // Find the repository by searching up through parent directories
    let repo = gix::discover(path)?;

    // If there repo has any untracked (and unignored) files it's dirty
    if has_untracked_changes(&repo)? {
        log::debug!("Repo is dirty - untracked changes");
        return Ok(true);
    }

    match repo.is_dirty() {
        Ok(is_dirty) => {
            if is_dirty {
                log::debug!("Repo is dirty - Index changes");
            } else {
                log::debug!("Repo is clean - No index changes")
            }
            Ok(is_dirty)
        }
        // SHOULD only happen if the repository has no commits
        Err(e) => {
            log::warn!("{e}");
            let head = repo.head()?;
            let no_commits = matches!(head.kind, gix::head::Kind::Unborn(_));
            if no_commits {
                log::debug!("Repo has no commits");
                let staged_changes = has_staged_changes(&repo)?;
                if staged_changes {
                    log::debug!("Repo is dirty - staged changes");
                } else {
                    log::debug!("Repo is clean - no commits, staged, or untracked changes");
                }
                Ok(staged_changes)
            } else {
                bail!("Unable to determine repository status");
            }
        }
    }
}

// Returns whether or not a [Repository] has untracked changes
fn has_untracked_changes(repo: &Repository) -> Result<bool> {
    // Check for untracked files using the proper status API

    let has_untracked = repo
        .status(progress::Discard)?
        .untracked_files(gix::status::UntrackedFiles::Collapsed)
        .index_worktree_submodules(gix::status::Submodule::AsConfigured { check_dirty: true })
        .into_index_worktree_iter(Vec::new())?
        .take_while(Result::is_ok)
        .next()
        .is_some();
    Ok(has_untracked)
}

// Returns whether or not a [Repository] has staged changes
fn has_staged_changes(repo: &Repository) -> Result<bool> {
    let index = repo.index_or_empty()?;
    Ok(!index.entries().is_empty())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_str_eq;
    use test_utils;
    use test_utils::testrepo::TestRepo;
    use testresult::TestResult;
    // Enables logging for all tests
    use test_log::test;

    use super::*;

    #[test]
    fn test_has_untracked_changes_empty_repo() -> TestResult {
        let repo = TestRepo::builder().init(true).build();
        let has_untracked = has_untracked_changes(&repo.as_gix_repo())?;
        assert!(
            !has_untracked,
            "An empty initialized repo has no untracked changes"
        );
        Ok(())
    }

    #[test]
    fn test_has_staged_changes_empty_repo() -> TestResult {
        let repo = TestRepo::builder().init(true).build();
        let staged_changes = has_staged_changes(&repo.as_gix_repo())?;
        assert!(
            !staged_changes,
            "An empty initialized repo has no staged changes"
        );
        Ok(())
    }

    #[test]
    fn test_has_untracked_changes_repo_with_untracked() -> TestResult {
        let repo = TestRepo::builder().init(true).build();
        repo.create_file("test.txt", None);
        let has_untracked = has_untracked_changes(&repo.as_gix_repo())?;
        assert!(
            has_untracked,
            "A repo with a newly created file SHOULD have untracked status"
        );
        Ok(())
    }

    #[test]
    fn test_has_untracked_changes_repo_with_staged() -> TestResult {
        let repo = TestRepo::builder().init(true).build();
        repo.create_add_file("test.txt", None);
        let has_untracked = has_untracked_changes(&repo.as_gix_repo())?;
        assert!(
            !has_untracked,
            "A repo with a newly staged file should NOT have untracked status"
        );
        Ok(())
    }

    #[test]
    fn test_has_untracked_changes_repo_with_initial_commit() -> TestResult {
        let repo = TestRepo::builder().init(true).build();
        repo.create_add_commit_file("test.txt", None, "initial commit");
        let has_untracked = has_untracked_changes(&repo.as_gix_repo())?;
        assert!(
            !has_untracked,
            "A repo with an initial commit should NOT have untracked status"
        );
        Ok(())
    }

    #[test]
    fn test_has_untracked_changes_repo_with_initial_commit_and_untracked_file() -> TestResult {
        let repo = TestRepo::builder().init(true).build();
        repo.create_add_commit_file("test.txt", None, "initial commit");
        repo.create_file("new_file.txt", None);
        let has_untracked = has_untracked_changes(&repo.as_gix_repo())?;
        assert!(
            has_untracked,
            "A repo with an initial commit and a newly created file SHOULD have untracked status"
        );
        Ok(())
    }

    #[test]
    fn test_get_branch_uninitialized_repo() {
        let repo = TestRepo::builder().init(false).build();

        let err_str = get_branch(repo.path()).unwrap_err().to_string();
        assert!(err_str.starts_with("Could not find a git repository in"));
        assert!(err_str.ends_with("or in any of its parents"));
    }

    #[test]
    fn test_get_branch_repo_with_no_commits() {
        // Arrange
        let default_branch_name = "trunk";
        let repo = TestRepo::builder()
            .init(true)
            .initial_branch_name(default_branch_name)
            .build();

        // Act
        let branch = get_branch(repo.path()).expect("Failed to get branch");

        // Assert
        assert_str_eq!(branch, default_branch_name);
    }

    #[test]
    fn test_get_branch_repo_with_initial_commit() {
        // Arrange
        let default_branch_name = "master";
        let repo = TestRepo::builder()
            .init(true)
            .initial_branch_name(default_branch_name)
            .build();
        repo.create_add_commit_file("dummy.txt", Some("dummy contents"), "Initial commit");

        // Act
        let branch = get_branch(repo.path()).expect("Failed to get branch");

        // Assert
        assert_str_eq!(branch, default_branch_name);
    }

    #[test]
    #[should_panic(expected = "not yet implemented: Handle detached HEAD state")]
    fn test_get_branch_repo_detached_from_head() {
        // Arrange
        let default_branch_name = "master";
        let repo = TestRepo::builder()
            .init(true)
            .initial_branch_name(default_branch_name)
            .build();
        repo.commit_empty("initial commit");
        repo.commit_empty("another commit");
        repo.checkout_parent();

        // Act
        let branch = get_branch(repo.path()).expect("Failed to get branch");
        // Assert
        assert_str_eq!(branch, default_branch_name);
    }

    #[test]
    fn test_get_commit_hash() -> TestResult {
        // Arrange
        let repo = TestRepo::builder().init(true).build();
        repo.commit_empty("initial commit");
        let expected_hash = repo.head_short_sha();

        // Act
        let our_hash = get_commit_hash(repo.path()).expect("Failed to get commit hash");

        // Compare the hashes
        assert_str_eq!(our_hash, expected_hash, "Our hash should match git's hash");
        Ok(())
    }

    #[test]
    fn test_is_dirty_unitiailized_repo() -> TestResult {
        let repo = TestRepo::builder().init(false).build();

        let err_str = is_dirty(repo.path()).unwrap_err().to_string();
        assert!(err_str.starts_with("Could not find a git repository in"));
        assert!(err_str.ends_with("or in any of its parents"));
        Ok(())
    }

    #[test]
    fn test_is_dirty_initialized_repo() -> TestResult {
        let repo = TestRepo::builder().init(true).build();

        let dirty = is_dirty(repo.path())?;
        assert!(!dirty, "Repository should be CLEAN after initial setup");
        Ok(())
    }

    #[test]
    fn test_is_dirty_untracked_file() -> TestResult {
        let repo = TestRepo::builder().init(true).build();
        repo.create_file("test.txt", None);

        let dirty = is_dirty(repo.path())?;
        assert!(
            dirty,
            "Repository should be DIRTY after creating untracked file"
        );
        Ok(())
    }

    #[test]
    fn test_is_dirty_staged_file() -> TestResult {
        let repo = TestRepo::builder().init(true).build();
        repo.create_add_file("test.txt", None);

        let dirty = is_dirty(repo.path())?;
        assert!(dirty, "Repository should be DIRTY after staging file");
        Ok(())
    }

    #[test]
    fn test_is_dirty_committed_file() -> TestResult {
        let repo = TestRepo::builder().init(true).build();
        repo.create_add_commit_file("test.txt", None, "initial commit");

        let dirty = is_dirty(repo.path())?;
        assert!(!dirty, "Repository should be CLEAN after staging file");
        Ok(())
    }

    #[test]
    fn test_is_dirty_committed_file_then_untracked() -> TestResult {
        let repo = TestRepo::builder().init(true).build();
        repo.create_add_commit_file("test.txt", None, "initial commit");
        repo.create_file("test2.txt", None);

        let dirty = is_dirty(repo.path())?;
        assert!(
            dirty,
            "Repository should be DIRTY after creating an untracked file"
        );
        Ok(())
    }
}

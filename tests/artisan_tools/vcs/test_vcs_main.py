from artisan_tools.vcs.main import (
    run_git_command,
    check_tag,
    check_current_branch,
    add_and_push_tag,
    get_commit_hash,
    check_clean,
)


def test_check_tag_exists(setup_git_repos):
    # Since the current directory is now repo2, no need to specify cwd
    assert check_tag("v1.0.1")


def test_check_tag_does_not_exist(setup_git_repos):
    # Check for a tag that doesn't exist in repo2
    assert not check_tag("nonexistent-tag")


def test_add_and_push_tag(setup_git_repos, app):
    # Add a new tag in repo2 and check if it exists
    add_and_push_tag(app.config["vcs"], "v1.0.2", "Another tag")
    assert "v1.0.2" in run_git_command("tag")


def test_check_current_branch(setup_git_repos):
    # Assuming the default branch is 'master' in repo2
    assert check_current_branch("master")
    assert not check_current_branch("nonexistent-branch")


def test_get_commit_hash(setup_git_repos):
    # Get commit hash directly using git:
    commit_hash = run_git_command("rev-parse HEAD")
    # Check short commit hash:
    assert get_commit_hash() == commit_hash[:7]
    # Check full commit hash:
    assert get_commit_hash(short=False) == commit_hash


def test_check_clean(setup_git_repos):
    # Assuming the working directory is clean in repo2
    assert check_clean()
    # Make a change in the working directory and check again
    with open("file.txt", "a") as f:
        f.write("New line")
    assert not check_clean()

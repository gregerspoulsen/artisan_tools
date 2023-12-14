import pytest

from artisan_tools.vcs import (
    run_git_command,
)


@pytest.fixture
def setup_git_repos(tmp_path, monkeypatch):
    """
    Fixture for creating git repo with remote for testing.
    """
    # Create the first repository
    repo1 = tmp_path / "repo1"
    repo1.mkdir()
    run_git_command("git init", cwd=repo1)
    (repo1 / "file.txt").write_text("Test file")
    run_git_command("git add file.txt", cwd=repo1)
    run_git_command(
        (
            "git -c 'user.name=Test Bot' -c 'user.email=bot@none.com' "
            "commit -m 'Initial commit'"
        ),
        cwd=repo1,
    )
    run_git_command("git tag v1.0.1", cwd=repo1)

    # Create the second repository by cloning the first
    repo2 = tmp_path / "repo2"
    run_git_command(f"git clone {repo1} {repo2}")

    # Change cwd for tests:
    monkeypatch.chdir(repo2)

    return repo1, repo2

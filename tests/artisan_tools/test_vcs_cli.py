from typer.testing import CliRunner
from artisan_tools.cli import (
    app,
)  # Replace 'your_cli_script' with the name of your script

from artisan_tools.vcs import run_git_command

runner = CliRunner()

# Assuming setup_git_repos is your pytest fixture for setting up git repositories


def test_check_tag_exists(setup_git_repos):
    result = runner.invoke(app, ["check-tag", "v1.0.1"])
    assert result.exit_code == 0
    assert "Tag 'v1.0.1' exists in the remote repository." in result.stdout


def test_check_tag_does_not_exist(setup_git_repos):
    result = runner.invoke(app, ["check-tag", "nonexistent-tag"])
    assert result.exit_code == 1
    assert (
        "Tag 'nonexistent-tag' does not exist in the remote repository."
        in result.stdout
    )


def test_check_branch(setup_git_repos):
    # Assuming the default branch is 'master'
    result = runner.invoke(app, ["check-branch", "master"])
    assert result.exit_code == 0
    assert "Current branch is 'master'." in result.stdout


def test_add_tag(setup_git_repos):
    repo1, repo2 = setup_git_repos
    result = runner.invoke(
        app,
        [
            "add-tag",
            "v1.0.2",
            "Test tag",
            "--git-user-name",
            "Test Bot",
            "--git-user-email",
            "bot@none.com",
        ],
    )
    assert result.exit_code == 0
    assert (
        "Tag 'v1.0.2' has been successfully added and pushed to remote" in result.stdout
    )
    tags = run_git_command("git tag", cwd=repo1).splitlines()
    assert "v1.0.2" in tags

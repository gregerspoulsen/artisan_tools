import subprocess
from typer.testing import CliRunner
from artisan_tools.cli import (
    app,
)

runner = CliRunner()

# Assuming setup_git_repos is your pytest fixture for setting up git repositories


def test_check_tag_exists(setup_git_repos):
    result = runner.invoke(app, ["check-no-tag", "v1.0.1"])
    assert result.exit_code == 1
    assert "Tag 'v1.0.1' already exists in the remote repository." in result.stdout


def test_check_tag_does_not_exist(setup_git_repos):
    result = runner.invoke(app, ["check-no-tag", "nonexistent-tag"])
    assert result.exit_code == 0


def test_check_branch(setup_git_repos):
    # Assuming the default branch is 'master'
    result = runner.invoke(app, ["check-branch", "master"])
    assert result.exit_code == 0
    assert "Current branch is 'master'." in result.stdout


def test_add_release_tag(setup_git_repos):
    # Write a VERSION file in the test repository
    version_file = setup_git_repos[1] / "VERSION"
    with version_file.open("w") as f:
        f.write("1.0.0")

    # Run the Typer command
    result = runner.invoke(
        app,
        [
            "add-release-tag",
            str(version_file),
            "--git-options=-c user.name=CI -c user.email=N/A",
        ],
    )

    print(result.stdout)
    assert result.exit_code == 0
    assert (
        "Tagged current changeset as 'v1.0.0' and pushed to remote repository."
        in result.stdout
    )

    # Check if the tag was actually created in the repository
    tags = subprocess.run(
        ["git", "tag"], cwd=setup_git_repos[0], capture_output=True, text=True
    )
    assert "v1.0.0" in tags.stdout

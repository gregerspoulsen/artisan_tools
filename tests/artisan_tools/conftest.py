import pytest

from artisan_tools.log import setup_root_handler
from artisan_tools.vcs.main import run_git_command


@pytest.fixture(scope="session", autouse=True)
def start_log():
    setup_root_handler(level="debug")


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
    (repo1 / "artisan.yaml").write_text("")
    run_git_command("git add file.txt", cwd=repo1)
    run_git_command("git add artisan.yaml", cwd=repo1)
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


@pytest.fixture
def app():
    """
    Fixture for creating a Artisan Tool app for testing
    """
    from artisan_tools.app import App

    # Create empty artisan.yaml file:
    with open("artisan.yaml", "w"):
        pass

    app = App()
    return app

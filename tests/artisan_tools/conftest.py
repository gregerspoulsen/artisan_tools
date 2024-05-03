import shutil
import pytest
import subprocess
import yaml
import time

from artisan_tools.log import setup_root_handler
from artisan_tools.vcs.main import run_git_command


@pytest.fixture(scope="session", autouse=True)
def start_log():
    """
    Enable log output in tests.
    """
    setup_root_handler(level="debug")


@pytest.fixture
def setup_git_repos(tmp_path, monkeypatch):
    """
    Fixture for creating git repo with remote for testing.
    """
    # Create the first repository
    repo1 = tmp_path / "repo1"
    repo1.mkdir()
    run_git_command("init", cwd=repo1)
    (repo1 / "file.txt").write_text("Test file")
    (repo1 / "artisan.yaml").write_text("")
    run_git_command("add file.txt", cwd=repo1)
    run_git_command("add artisan.yaml", cwd=repo1)
    run_git_command(
        "-c 'user.name=Test Bot' -c 'user.email=bot@none.com' "
        "commit -m 'Initial commit'",
        cwd=repo1,
    )
    run_git_command("tag v1.0.1", cwd=repo1)

    # Create the second repository by cloning the first
    repo2 = tmp_path / "repo2"
    run_git_command(f"clone {repo1} {repo2}")

    # Change cwd for tests:
    monkeypatch.chdir(repo2)

    return repo1, repo2


@pytest.fixture
def app():
    """
    Fixture for creating a Artisan Tool app for testing.
    """
    from artisan_tools.app import App

    app = App()
    app.load_extensions()
    return app


def find_available_executables(executable_names):
    """
    Check list for available executables.
    """
    return [name for name in executable_names if shutil.which(name) is not None]


engines = ["docker", "podman"]
available_engines = find_available_executables(engines)


@pytest.fixture(scope="session")
def registry(request):
    """
    Fixture for running registry with podman.
    """
    engine = available_engines[0]  # Just use whatever is available
    # Start the registry
    subprocess.run(
        [
            engine,
            "run",
            "-d",
            "-p",
            "5000:5000",
            "--name",
            "registry",
            "registry:latest",
        ],
        check=True,
    )
    time.sleep(0.5)
    yield "localhost:5000"

    # Stop the registry
    subprocess.run([engine, "rm", "-f", "registry"])


@pytest.fixture()
def app_with_config(tmp_path, monkeypatch):
    """
    Fixture for creating a configuration file for testing.
    """
    monkeypatch.chdir(tmp_path)

    # Create config file:
    config = {
        "container": {
            "auth": {
                "method": "direct",
                "user": "test",
                "token": "test",
            },
            "registry": "localhost:5000",
            "engine": "podman",
            "options": ["--tls-verify=False"],
        }
    }
    with open("artisan.yaml", "w") as f:
        yaml.dump(config, f)

    # Create VERSION file:
    with open("VERSION", "w") as f:
        f.write("0.99.9")

    monkeypatch.setenv("CR_TOKEN", "test_token")

    from artisan_tools.app import App

    app = App()
    app.load_extensions()
    return app

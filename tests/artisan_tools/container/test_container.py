import subprocess
import shutil
import pytest

from artisan_tools.container.main import check_login


def find_available_executables(executable_names):
    return [name for name in executable_names if shutil.which(name) is not None]


engines = ["docker", "podman"]
available_engines = find_available_executables(engines)
options = {"docker": [], "podman": ["--tls-verify=False"]}


@pytest.mark.parametrize("engine", available_engines)
def test_check_login(registry, engine):
    # Check that we're not logged in
    assert not check_login(registry, engine)

    # Log in using subprocess call to podman:
    subprocess.run(
        [
            engine,
            "login",
            "-u",
            "testuser",
            "-p",
            "testpassword",
            registry,
        ]
        + options[engine],
        check=True,
    )

    # Check that we're logged in
    assert check_login(registry, engine)

    # Log out
    subprocess.run([engine, "logout", registry], check=True)

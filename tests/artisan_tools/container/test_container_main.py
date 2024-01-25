import subprocess
import shutil
import pytest

from artisan_tools.container.main import check_login, login, logout, push
from artisan_tools.utils import container_engines

available_engines = container_engines()
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


@pytest.mark.parametrize("engine", available_engines)
def test_login(registry, engine):
    # Check that we're not logged in
    assert not check_login(registry, engine)

    # Log in
    login("testuser", "testpassword", registry, engine, options[engine])

    # Check that we're logged in
    assert check_login(registry, engine)

    # Log out
    logout(registry, engine)

@pytest.mark.parametrize("engine", available_engines)
def test_push(registry, engine):
    image = "alpine"

    # Make sure image is pulled:
    subprocess.run([engine, "pull", image] + options[engine], check=True)

    # Push the image
    push(image, f"{registry}/{image}", engine, options=options[engine])
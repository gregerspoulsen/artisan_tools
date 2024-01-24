import pytest
import subprocess


from artisan_tools.container.main import login, check_login

def test_check_login(registry):
    # Check that we're not logged in
    assert not check_login(registry, engine="podman")

    # Log in using subprocess call to podman:
    subprocess.run(["podman", "login", "-u", "testuser", "-p", "testpassword", "--tls-verify=False", registry], check=True)

    # Check that we're logged in
    assert check_login(registry, engine="podman")

    # Log out
    subprocess.run(["podman", "logout", registry], check=True)


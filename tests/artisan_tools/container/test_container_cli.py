import os
import pytest
import typer
import subprocess

from typer.testing import CliRunner
from artisan_tools.container.cli import factory  # Adjust this import based on your actual module structure
from artisan_tools.utils import container_engines

if not 'podman' in container_engines():
    pytest.skip("podman not available", allow_module_level=True)


@pytest.fixture()
def runner():
    return CliRunner()

# Test Successful Login
def test_login_successful(runner, app_with_config, registry):
    result = runner.invoke(factory(app_with_config), ["login"], catch_exceptions=False)
    assert result.exit_code == 0, result.stdout
    assert "Successfully logged in" in result.output  # Adjust based on your actual success message

# Test Missing Token Variable
def test_login_missing_token(runner, app_with_config, registry):
    del os.environ["CR_TOKEN"]
    result = runner.invoke(factory(app_with_config), ["login"], catch_exceptions=False)
    assert result.exit_code != 0, result.stdout
    assert "Error, environment variable" in result.output

# Test Logout
def test_logout(runner, app_with_config):
    result = runner.invoke(factory(app_with_config), ["logout"], catch_exceptions=False)
    assert result.exit_code == 0, result.stdout
    assert "Successfully logged out" in result.output  # Adjust based on your actual success message

# Test Push
def test_push(runner, app_with_config, registry):
    src = "alpine"
    target = "localhost:5000/alpine:test"
    # Pull image with podman:
    subprocess.run(["podman", "pull", src, '--tls-verify=False'], check=True)

    result = runner.invoke(factory(app_with_config), ["push", src, target], catch_exceptions=False)
    assert result.exit_code == 0, result.stdout
    assert "Successfully pushed" in result.output  # Adjust based on your actual success message

    # Check if image is present in registry
    result = subprocess.run(["podman", "pull", target, '--tls-verify=False'], check=True)
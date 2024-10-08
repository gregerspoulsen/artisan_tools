import pytest
import subprocess

from typer.testing import CliRunner
from artisan_tools.container.cli import (
    factory,
)  # Adjust this import based on your actual module structure
from artisan_tools.utils import container_engines

if "podman" not in container_engines():
    pytest.skip("podman not available", allow_module_level=True)


@pytest.fixture()
def runner():
    return CliRunner()


# Test Successful Login
def test_login_successful(runner, app_with_config, registry):
    result = runner.invoke(factory(app_with_config), ["login"], catch_exceptions=False)
    assert result.exit_code == 0, result.stdout
    assert (
        "Successfully logged in" in result.output
    )  # Adjust based on your actual success message


# Test Logout
def test_logout(runner, app_with_config):
    result = runner.invoke(factory(app_with_config), ["logout"], catch_exceptions=False)
    assert result.exit_code == 0, result.stdout
    assert (
        "Successfully logged out" in result.output
    )  # Adjust based on your actual success message


# Test Push
@pytest.mark.skipif("podman" not in container_engines(), reason="Requires podman")
def test_push(runner, app_with_config, registry):
    src = "alpine"
    target = "localhost:5000/alpine"
    tags = ["test", "test2"]

    # Pull image with podman:
    subprocess.run(["podman", "pull", src, "--tls-verify=False"], check=True)

    result = runner.invoke(
        factory(app_with_config), ["push", src, target, *tags], catch_exceptions=False
    )
    assert result.exit_code == 0, result.stdout
    assert (
        "Successfully pushed" in result.output
    )  # Adjust based on your actual success message

    # Check if image is present in registry
    for tag in tags:
        result = subprocess.run(
            ["podman", "pull", target + ":" + tag, "--tls-verify=False"], check=True
        )


@pytest.mark.skipif("docker" not in container_engines(), reason="Requires docker")
@pytest.mark.parametrize("tags", [[], ["test", "test2"]])
def test_build_push(runner, app_with_config, registry, tags):
    # Create a Dockerfile
    with open("Dockerfile", "w") as f:
        f.write("FROM alpine\n")

    # Build and push
    repository = f"{registry}/test"
    runner.invoke(
        factory(app_with_config),
        [
            "build-push",
            repository,
            *tags,
            "--platform",
            "linux/amd64",
            "--platform",
            "linux/arm64",
            "--option",
            "--file=Dockerfile",
        ],
        catch_exceptions=False,
    )

    # Check that the image exists
    for tag in tags:
        subprocess.run(
            ["docker", "pull", f"{repository}:{tag}"],
            check=True,
        )


@pytest.mark.skipif("docker" not in container_engines(), reason="Requires docker")
def test_build_push_error(runner, app_with_config, registry):
    # Create a Dockerfile
    with open("Dockerfile", "w") as f:
        f.write("FROM alpine-does-not-exist\n")

    # Build and push
    repository = f"{registry}/test"
    result = runner.invoke(
        factory(app_with_config),
        [
            "build-push",
            repository,
            "test",
            "--platform",
            "linux/amd64",
            "--platform",
            "linux/arm64",
            "--option",
            "--file=Dockerfile",
        ],
        catch_exceptions=False,
    )
    assert result.exit_code == 1, result.stdout


def test_command_error(runner, app_with_config, registry):
    command = "ls -j"
    result = runner.invoke(
        factory(app_with_config), ["command", command], catch_exceptions=False
    )
    assert result.exit_code == 2, result.stdout


def test_command(runner, app_with_config, registry):
    command = "ls"
    result = runner.invoke(
        factory(app_with_config), ["command", command], catch_exceptions=False
    )
    assert result.exit_code == 0, result.stdout

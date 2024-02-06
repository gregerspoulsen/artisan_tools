import subprocess
import pytest

from artisan_tools.container.main import check_login, login, logout, push, build_push
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


@pytest.mark.skipif("docker" not in available_engines, reason="Requires docker")
def test_build_push(registry, tmpdir):
    # Create a Dockerfile
    dockerfile = tmpdir.join("Dockerfile")
    dockerfile.write("FROM alpine")

    # Build and push
    tags = ["tag1", "tag2"]
    repository = f"{registry}/test_build_push"
    platforms = ("linux/amd64", "linux/arm64")
    build_push(repository, tags, platforms, context=str(tmpdir))

    # Check that the image exists
    for tag in tags:
        subprocess.run(
            ["docker", "pull", f"{repository}:{tag}"],
            check=True,
        )


@pytest.mark.skipif("docker" not in available_engines, reason="Requires docker")
def test_build_push_no_tag(registry, tmpdir):
    # Create a Dockerfile
    dockerfile = tmpdir.join("Dockerfile")
    dockerfile.write("FROM alpine")

    # Build and push
    repository = f"{registry}/test_build_push"
    platforms = ("linux/amd64", "linux/arm64")
    build_push(repository, (), platforms, context=str(tmpdir))

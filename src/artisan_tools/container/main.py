import subprocess
import os

from artisan_tools.log import get_logger

logger = get_logger("container")

# import typer


def check_login(registry: str, engine: str = "docker"):
    """
    Check if the user is logged in to a container registry using CLI.

    This function checks if the user is already logged in to the specified
    container registry by examining the specific engine configuration file.

    Parameters
    ----------
    registry : str
        The URL of the container registry.
    engine : str, optional
        The container engine to use. Default is Docker.
    """
    # Check if already logged in by checking the Docker config file
    if engine == "docker":
        docker_config_path = os.path.expanduser("~/.docker/config.json")
        if os.path.isfile(docker_config_path):
            try:
                with open(docker_config_path, "r") as file:
                    config_contents = file.read()

                # Check if the registry URL is present in the Docker configuration file
                return registry in config_contents
            except Exception as e:
                print(f"Error reading Docker config file: {e}")
                return False
        else:
            return False
    elif engine == "podman":
        # Check if logged in using podman login --get-login

        out = subprocess.run(
            ["podman", "login", "--get-login", registry],
            text=True,
            capture_output=True,
        )
        if out.returncode == 0:
            return True
        elif out.returncode == 125 and "not logged in" in out.stderr:
            return False
        else:
            raise RuntimeError(f"Failed to check login status: {out.stderr}")
    else:
        raise ValueError(f"Unknown container engine: {engine}")


def login(
    username: str,
    token: str,
    registry: str = "https://index.docker.io/v1",
    engine: str = "docker",
    options: list = (),
):
    """
    Log in to a container registry using CLI.

    This function checks if the user is already logged in to the specified
    container registry by examining the specific engine configuration file.
    If not already logged in, it uses the CLI to log in to the registry using
    the provided username and password/token

    Parameters
    ----------
    username : str
        The username for the Docker registry.
    token : str
        The password or token for the registry.
    registry : str, optional
        The URL of the container registry. Default is dockerhub.
    engine : str, optional
        The container engine to use. Default is Docker.
    options : list, optional
        Additional options to pass to the login command.
    """
    logger.debug(f"Logging in to {registry} using {engine} and username {username}")

    if check_login(registry, engine):
        return

    # Log in to the registry
    try:
        subprocess.run(
            [engine, "login", registry, "-u", username, "--password-stdin"]
            + list(options),
            input=token,
            text=True,
            capture_output=True,
            check=True,
        )
    except subprocess.CalledProcessError as e:
        print(f"Failed to log in to {registry}: {e.output}")
        raise


def logout(
    registry: str = "https://index.docker.io/v1",
    engine: str = "docker",
    options: list = (),
):
    """
    Log out of container registry using CLI.

    Parameters
    ----------
    username : str
        The username for the Docker registry.
    registry : str, optional
        The URL of the container registry. Default is dockerhub.
    engine : str, optional
        The container engine to use. Default is Docker.
    options : list, optional
        Additional options to pass to the login command.
    """
    # Log out of the registry
    try:
        subprocess.run(
            [engine, "logout", registry] + list(options),
            text=True,
            capture_output=True,
            check=True,
        )
    except subprocess.CalledProcessError as e:
        print(f"Failed to log out of {registry}: {e.output}")
        raise


def push(source: str, target: str, engine: str = "docker", options: list = ()) -> None:
    """
    Tag and push a Docker image to a container registry.

    Parameters
    ----------
    source : str
        The source image.
    target : str
        Target to push to.
    """
    # Tag the image
    try:
        subprocess.run([engine, "tag", source, target], check=True)
    except subprocess.CalledProcessError as e:
        print(f"Failed to tag image: {e.output}")
        raise

    # Push the image
    try:
        subprocess.run([engine, "push", target] + list(options), check=True)
    except subprocess.CalledProcessError as e:
        print(f"Failed to push image: {e.output}")
        raise


# def setup(app):
#     app.add_cli(cli)


if __name__ == "__main__":
    print(check_login("localhost:5000"))

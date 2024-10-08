from .main import login as login_main
from .main import logout as logout_main
from .main import push as push_main
from .main import build_push as build_push_main
from .main import check_login as check_login_main

from artisan_tools.utils import get_item, get_env_var
from artisan_tools.app import App

import subprocess
from contextlib import contextmanager
from typing import List


def login(app):
    """
    Login to container registry as specified in the configuration file.

    When `auth == direct`, use the `user` and `token` directly.
    When `auth == env`, use the environment variables specified in `user` and `token`

    Returns
    -------
    bool
        True if login was successful, False otherwise.
    """
    config = app.config["container"]
    registry = get_item(config, "registry", "registry")
    engine = get_item(config, "engine", "container engine")

    # Check if already logged in:
    if check_login_main(registry, engine):
        return False

    auth = config["auth"]
    match auth["method"]:
        case "direct":
            user = get_item(auth, "user", "username")
            token = get_item(auth, "token", "token")
        case "env":
            user_var = get_item(auth, "user_var", "environment variable for username")
            token_var = get_item(auth, "token_var", "environment variable for token")
            user = get_env_var(
                user_var,
                f"Error, environment variable {user_var} not set, it should "
                "contain username for logging in to registry.",
            )
            token = get_env_var(
                token_var,
                f"Error, environment variable {token_var} not set, it should "
                "contain token for logging in to registry.",
            )
        case _:
            raise ValueError(
                "Error, invalid value for auth method, should be 'direct' or 'env'"
            )

    options = get_item(config, "options", "options")

    logged_in = login_main(
        username=user,
        token=token,
        registry=registry,
        engine=engine,
        options=options,
    )

    return logged_in


def logout(app):
    """
    Log out of container registry as specified in the configuration file.
    """
    config = app.config["container"]
    registry = get_item(config, "registry", "registry")
    engine = get_item(config, "engine", "container engine")

    logout_main(
        registry=registry,
        engine=engine,
    )


@contextmanager
def authorized_registry(app):
    """
    Context manager for running commands with access to container registry.
    """
    logged_in = login(app)
    try:
        yield
    finally:
        if logged_in:
            logout(app)


def push(app: App, source: str, target: str, tags: List[str]) -> None:
    """
    Push a Docker image to a container registry.

    Logging in/out is handled automatically using with details from the
    configuration file.

    Args:
        app: The application instance.
        source: The source image to push, can contain tags.
        target: The target image to push to, must not include tags.
        tags: List of tags to push to the target image. Tags will be parsed by
            the parser extension.

    Raises:
        ValueError: If the target image contains tags.
    """
    # Check that target doesn't contain tags, that is no colons after
    # the last slash:
    if target.split("/")[-1].count(":") > 0:
        raise ValueError("Error, target image must not contain tags")

    # Parse tags:
    parser = app.get_extension("parser")
    targets = [target + ":" + parser.parse(app, tag) for tag in tags]  # type: ignore[attr-defined]

    config = app.config["container"]
    engine = get_item(config, "engine", "container engine")
    options = get_item(config, "options", "options")

    with authorized_registry(app):
        for target in targets:
            push_main(
                source=source,
                target=target,
                engine=engine,
                options=options,
            )
            print(f"Successfully pushed {source} to {target}")


def build_push(
    app: App,
    repository: str,
    tags: List[str],
    platforms: tuple[str, ...] = ("linux/amd64",),
    context: str = ".",
    options: tuple[str, ...] = (),
) -> None:
    """
    Build and push a container image.

    Args:
        app: The application instance.
        repository: The repository to push to.
        tags: List of tags to push to the target image.
            Tags will be parsed by the parser extension.
        platforms: List of platforms to build for. Default is linux/amd64.
        context : The build context. Default is current directory.
        options : Additional options to pass to the build command.
    """
    # Parse tags:
    parser = app.get_extension("parser")
    parsed_tags = [parser.parse(app, tag) for tag in tags]  # type: ignore[attr-defined]

    with authorized_registry(app):
        build_push_main(
            repository=repository,
            tags=parsed_tags,
            platforms=platforms,
            context=context,
            options=options,
        )


def run_command_with_auth(app, command: str):
    """
    Run shell command with access to container registry.

    The command will log in to the registry (if not already logged in), execute
    the shell command (and log out again).
    """
    with authorized_registry(app):
        subprocess.run(command, shell=True, check=True)

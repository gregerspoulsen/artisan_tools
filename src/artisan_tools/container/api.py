from .main import login as login_main
from .main import logout as logout_main
from .main import push as push_main
from artisan_tools.utils import get_item, get_env_var

import subprocess


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
    auth = config["auth"]

    if auth["method"] == "direct":
        user = get_item(auth, "user", "username")
        token = get_item(auth, "token", "token")

    if auth["method"] == "env":
        user_var = get_item(auth, "user_var", "environment variable for username")
        token_var = get_item(auth, "token_var", "environment variable for token")
        user = get_env_var(
            user_var,
            (
                f"Error, environment variable {user_var} not set, it should "
                "contain username for logging in to registry."
            ),
        )
        token = get_env_var(
            token_var,
            (
                f"Error, environment variable {token_var} not set, it should "
                "contain token for logging in to registry."
            ),
        )

    registry = get_item(config, "registry", "registry")
    engine = get_item(config, "engine", "container engine")
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


def push(app, source: str, target: str, tags: list[str]) -> None:
    """
    Push a Docker image to a container registry. Logging in/out is handled
    automatically using with details from the configuration file.

    Parameters
    ----------
    source : str
        The source image to push, can contain tags.
    target : str
        The target image to push to, must not include tags
    tags : list[str]
        List of tags to push to the target image. Tags will be parsed by
        the parser extension.
    """
    # Check that target doesn't contain tags, that is no colons after
    # the last slash:
    if target.split("/")[-1].count(":") > 0:
        raise ValueError("Error, target image must not contain tags")

    # Parse tags:
    parser = app.get_extension("parser")
    targets = [target + ":" + parser.parse(app, tag) for tag in tags]

    config = app.config["container"]
    engine = get_item(config, "engine", "container engine")
    options = get_item(config, "options", "options")

    logged_in = login(app)

    for target in targets:
        push_main(
            source=source,
            target=target,
            engine=engine,
            options=options,
        )
        print(f"Successfully pushed {source} to {target}")

    if logged_in:
        logout(app)


def run_command_with_auth(app, command: str):
    """
    Run shell command with access to container registry. The command will log in
    to the registry (if not already logged in), execute the shell command
    (and log out again).
    """

    logged_in = login(app)

    try:
        subprocess.run(command, shell=True, check=True)
    except Exception:
        raise
    finally:
        if logged_in:
            logout(app)

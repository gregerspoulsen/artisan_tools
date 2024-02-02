from artisan_tools.app import App
from artisan_tools.version.main import (
    read_version_file,
    bump_version,
    check_version,
    replace_in_file,
    run_hook,
)


def get_version(app: App):
    """
    Retrieve current project version.

    The version will be read from the configured version file.

    Args:
    app (App): The application object.

    Returns:
    str: The version of the package.
    """
    file_path = app.config["version"]["file"]

    return read_version_file(file_path)


def update_version(app: App, target: str):
    """
    Update version in file using string substitution.

    Args:
        app (App): The application object.
        target (str): Either part to bump [major|minor|patch] or a full version
            string. In the latter case it must be a valid semver string.

    Returns:
        str: The updated version string.

    Raises:
        ValueError: If the target is not a valid value.

    Example:
        >>> app = App()
        >>> update_version(app, "minor")
        '1.2.0'
    """
    current_version = get_version(app)

    if target in ["major", "minor", "patch"]:
        new_version = bump_version(current_version, target)
    else:
        new_version = target
        if not check_version(target, release=False):
            raise ValueError(
                f"Invalid value: {target}. Target must be 'major', 'minor', "
                "'patch', or a valid semver string."
            )

    # Update version file
    main_file = app.config["version"]["file"]
    replace_in_file(main_file, current_version, new_version)

    # Run hooks
    hooks = app.config["version"]["hooks"]
    for hook in hooks:
        run_hook(hook, current_version, new_version)

    return new_version

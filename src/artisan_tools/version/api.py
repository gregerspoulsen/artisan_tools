from artisan_tools.version.main import (
    read_version_file,
    bump_version,
    check_version,
    replace_in_file,
    run_hook,
)


def get_version(app):
    """
    Retrieve version. If a file path is provided, the version will be read from
    that file. Otherwise, the version will be read from the configured version
    file.

    Returns:
    str: The version of the package.
    """
    file_path = app.config["version"]["file"]

    return read_version_file(file_path)


def update_version(app, target: str):
    """
    Update version in file using string subsitution.

    Parameters:
    file_path (str): Path to file containing the version string.
    target (str): Either part to bump [major|minor|patch] or a full version
        string. In the latter case it must be a valid semver string.
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

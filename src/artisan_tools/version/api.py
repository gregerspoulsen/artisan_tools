from artisan_tools.app import App
from artisan_tools.version.main import (
    read_version_file,
    bump_version,
    check_version,
    write_version_file,
    run_hook,
)

from artisan_tools.log import get_logger

log = get_logger("version.api")


def get_version(app: App):
    """
    Retrieve current project version.

    The version will be read from the configured version file.

    Args:
    app (App): The application object.

    Returns:
    str: The version of the package.
    """
    file_path = app.config["version"]["current"]

    return read_version_file(file_path)


def bump(app: App, target: str):
    """
    Replace version in file using string substitution.

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
        >>> replace_version(app, "minor")
        '1.2.0'
    """
    current_version = read_version_file(app.config["version"]["release"])

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
    main_file = app.config["version"]["release"]
    write_version_file(main_file, new_version)

    # Run hooks
    hooks = app.config["version"]["hooks"]
    for hook in hooks:
        run_hook(hook, new_version)

    return new_version


def update(app, release: bool = False):
    """
    Update version in `VERSION` file.

    If the version is not a release additional build info is added to the
    version read from the 'RELEASE' file.

    Args:
        app (App): The application object.
        release (bool): Flag to indicate if the version is a release
            in which case build info is not added.
    """
    # Read RELEASE file:
    version = read_version_file(app.config["version"]["release"])

    if not release:
        # Get curren branch:
        branch = app.get_extension("vcs").get_current_branch()
        # Get commit hash:
        hash = app.get_extension("vcs").get_commit_hash()
        # Get clean status:
        is_clean = app.get_extension("vcs").check_clean()
        dirty = "+dirty" if not is_clean else ""

        version = f"{version}+{branch}+{hash}{dirty}"

    # Write to VERSION file:
    version_file = app.config["version"]["current"]
    write_version_file(version_file, version)
    log.info(f"File: {version_file} updated to {version}")

    # Run hooks
    hooks = app.config["version"]["hooks"]
    for hook in hooks:
        run_hook(hook, version)

    return version

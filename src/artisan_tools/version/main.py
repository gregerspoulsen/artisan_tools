"""
Tools to support release of a package.
"""

import os
import semver

from artisan_tools.log import get_logger

logger = get_logger("version.main")


def check_version(version: str, release=True) -> None:
    """
    Check that the version is a proper semver version.

    Parameters
    ----------
    version : str
        The version string to check.
    release : bool
        Flag indicating whether to check for a release version, default True.

    """
    try:
        parsed_version = semver.Version.parse(version)
        if release:
            return parsed_version.prerelease is None and parsed_version.build is None
        else:
            return True
    except ValueError:
        return False


def bump_version(version: str, part: str):
    """
    Bump the specified part (major, minor, or patch) of a semantic version string.

    Args:
    version: A semantic version string (e.g., '1.2.3').
    part: The part of the version to bump ('major', 'minor', or 'patch').

    Returns:
    str: The bumped version string.

    Raises:
    ValueError: If the part is not 'major', 'minor', or 'patch'.
    """
    parsed_version = semver.Version.parse(version)

    if part == "major":
        new_version = parsed_version.bump_major()
    elif part == "minor":
        new_version = parsed_version.bump_minor()
    elif part == "patch":
        new_version = parsed_version.bump_patch()
    else:
        raise ValueError("Invalid part to bump: must be 'major', 'minor', or 'patch'")

    return str(new_version)


def read_version_file(file_path: str = "VERSION"):
    """
    Read a version from a file.

    Args:
    file_path: Path to the file containing the version string.
    """
    # Check the file exists:
    if not os.path.exists(file_path):
        raise FileNotFoundError(f"File not found: {file_path}")

    # Read the version from the file
    with open(file_path, "r") as file:
        current_version = file.read().strip()

    return current_version


def replace_in_file(file_path: str, current_version: str, new_version: str):
    """
    Replace the version in a file.

    Args:
    file_path: The path to the file.
    current_version: The current version string.
    new_version: The new version string.

    Raises:
    ValueError: If the current version is not found in the file.
    """
    # Read the file
    with open(file_path, "r") as file:
        file_contents = file.read()

    # Make sure the target string is at least once in the file:
    if current_version not in file_contents:
        raise ValueError(
            f"Target string '{current_version}' not found in file: {file_path}"
        )

    # Replace the version
    file_contents = file_contents.replace(current_version, new_version)

    # Write the file back to disk
    with open(file_path, "w") as file:
        file.write(file_contents)

    logger.info(f"Updated version in file: {file_path} to {new_version}")


available_hooks = {"string_replace": replace_in_file}


def run_hook(hook, current_version, new_version):
    """
    Execute a hook.

    Parameters
    ----------
    hook : str
        The hook to execute.
    current_version : str
        The current version.
    new_version : str
        The new version.
    """
    if not isinstance(hook, dict):
        raise ValueError(
            f"Invalid hook: {hook}, it must be a dictionary with a 'method' key"
        )
    if "method" not in hook:
        raise ValueError(
            f"Invalid hook: {hook}, it must be a dictionary with a 'method' key"
        )
    method = hook["method"]
    if method not in available_hooks:
        raise ValueError(f"Invalid hook: {hook}, it m {list(available_hooks.keys())}")
    hook_func = available_hooks[method]
    hook.pop("method")
    hook_func(current_version=current_version, new_version=new_version, **hook)

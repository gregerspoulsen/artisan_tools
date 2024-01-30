"""
Tools to support release of a package.
"""

import os
import semver


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


def bump_version(version, part):
    """
    Bump the specified part (major, minor, or patch) of a semantic version string.

    Parameters:
    version (str): A semantic version string (e.g., '1.2.3').
    part (str): The part of the version to bump ('major', 'minor', or 'patch').

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


def read_version_file(file_path="VERSION"):
    """
    Read a version from a file.

    Parameters:
    file_path (str): Path to the file containing the version string.
    """
    # Check the file exists:
    if not os.path.exists(file_path):
        raise FileNotFoundError(f"File not found: {file_path}")

    # Read the version from the file
    with open(file_path, "r") as file:
        current_version = file.read().strip()

    return current_version


def replace_in_file(file_path, target, replacement):
    """
    Replace the version in a file.

    Parameters
    ----------
    file_path (str): Path to the file.
    target (str): The target string to replace.
    replacement (str): The new string.
    """
    # Read the file
    with open(file_path, "r") as file:
        file_contents = file.read()

    # Replace the version
    file_contents = file_contents.replace(target, replacement)

    # Write the file back to disk
    with open(file_path, "w") as file:
        file.write(file_contents)

"""
Tools to support release of a package.
"""

import os
import semver


def check_version(version: str) -> None:
    """
    Check that the version is a proper semver release version.

    :param version: The version to check

    """
    try:
        parsed_version = semver.Version.parse(version)
        return parsed_version.prerelease is None and parsed_version.build is None
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


def bump_version_file(file_path, part):
    """
    Read a version from a file, bump it, and write the bumped version back to the file.

    Parameters:
    file_path (str): Path to the file containing the version string.
    part (str): The part of the version to bump ('major', 'minor', or 'patch').
    """
    current_version = read_version_file(file_path)

    # Bump the version using the bump_version function
    new_version = bump_version(current_version, part)

    # Write the new version to the file
    with open(file_path, "w") as file:
        file.write(new_version)

    return new_version


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

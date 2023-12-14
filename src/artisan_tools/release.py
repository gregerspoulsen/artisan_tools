"""
Tools for preparing a release
"""
import semver


def check_version(version: str) -> None:
    """
    Check that the version is a proper semver release version.

    :param version: The version to check

    """
    try:
        parsed_version = semver.parse(version)
        return parsed_version["prerelease"] is None and parsed_version["build"] is None
    except ValueError:
        return False


def check_version_file(file_path: str) -> None:
    """
    Check that the version in the specified file is a proper semver release version.

    :param file_path: The path to the file containing the version

    """
    with open(file_path, "r") as file:
        version = file.read().strip()
    return check_version(version)

"""
Tools to support release of a package.
"""

import os
import re
import semver

from artisan_tools.log import get_logger

logger = get_logger("version.main")


def check_version(version: str, release=True) -> bool:
    """
    Check that the version is a proper semver version.

    Args:
    version (str): The version string to check.
    release (bool, optional): Flag indicating whether to check for a release
    version. Defaults to True.

    Returns:
    bool: True if the version is a proper semver version, False otherwise.
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


def write_version_file(file_path: str, version: str) -> None:
    """
    Write version to a file.

    Args:
    file_path: Path to the file to write the version string.
    version: The version string to write.
    """
    # Make sure version ends with a single newline:
    version = version.strip() + "\n"

    with open(file_path, "w") as file:
        file.write(version)

    logger.info(f"Version written to file: {file_path}")


def replace_regex_in_file(
    file_path: str, pattern: str, new_version: str, **kwargs
) -> None:
    """
    Replaces version in a file based on a regex pattern.

    Args:
    file_path (str): The path to the file where replacements are made.
    pattern (str): The regex pattern to match.
    new_version (str): The new version to write

    Returns:
    None
    """
    # Read the file contents
    with open(file_path, "r", encoding="utf-8") as file:
        file_content = file.read()

    # Make sure the pattern is found in the file:
    if not re.search(pattern, file_content, flags=re.MULTILINE):
        raise ValueError(f"Pattern not found in file: {file_path}")

    # Replace the text using the provided regex pattern and replacement
    modified_content = re.sub(pattern, new_version, file_content, flags=re.MULTILINE)

    # Write the modified content back to the file
    with open(file_path, "w", encoding="utf-8") as file:
        file.write(modified_content)

    logger.info(
        f"Updated version in file: {file_path} to {new_version} using regex pattern:"
        f" {pattern}"
    )


def replace_in_pyproject(file_path: str, new_version: str, **kwargs) -> None:
    """
    Updates the version number in a pyproject.toml file.

    Args:
    file_path (str): The path to the pyproject.toml file.
    new_version (str): The new version number to write.

    Returns:
    None
    """
    # Regex pattern to match the entire line containing the version
    pattern = r'^(version\s*=\s*")([^"]+)(")$'
    # Replacement pattern includes the new version number
    replacement = r"\g<1>" + new_version + r"\g<3>"
    replace_regex_in_file(file_path, pattern, replacement)

    logger.info(f"Version updated to {new_version} in {file_path}")


available_hooks = {
    "regex_replace": replace_regex_in_file,
    "pyproject_replace": replace_in_pyproject,
}


def run_hook(hook, new_version):
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
    hook_func(new_version=new_version, **hook)

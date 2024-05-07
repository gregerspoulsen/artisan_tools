import pytest
import unittest
from artisan_tools.version.main import (
    bump_version,
    check_version,
    replace_in_file,
    run_hook,
    replace_regex_in_file,
    replace_in_pyproject,
    write_version_file,
)


# --- bump_version -------------------------------------------------------------
def test_bump_version_major():
    assert bump_version("1.2.3", "major") == "2.0.0"


def test_bump_version_minor():
    assert bump_version("1.2.3", "minor") == "1.3.0"


def test_bump_version_patch():
    assert bump_version("1.2.3", "patch") == "1.2.4"


def test_bump_version_invalid_part():
    with pytest.raises(ValueError):
        bump_version("1.2.3", "invalid_part")


# --- check_version ------------------------------------------------------------
class TestVersionCheck(unittest.TestCase):
    def test_proper_release(self):
        # Test with a proper release version
        self.assertTrue(check_version("1.2.3"))

    def test_pre_release(self):
        # Test with a pre-release version
        self.assertFalse(check_version("1.2.3-alpha.1"))

    def test_invalid_version(self):
        # Test with an invalid version string
        self.assertFalse(check_version("invalid-version"))

    def test_build_metadata_release(self):
        # Test with a version that includes build metadata
        self.assertFalse(check_version("1.2.3+build.4"))

    def test_build_metadata(self):
        # Test with a version that includes build metadata
        self.assertTrue(check_version("1.2.3+build.4", release=False))


def test_write_version_file(tmpdir):
    # Arrange
    version = "0.1.0"
    file_path = tmpdir.join("VERSION")

    # Act
    write_version_file(file_path, version)

    # Assert
    assert file_path.read() == version


def test_replace_in_file(tmpdir):
    # Arrange
    old_version = "0.1.0"
    new_version = "0.2.0"
    file_path = tmpdir.join("VERSION")
    file_path.write(old_version)

    # Act
    replace_in_file(file_path, old_version, new_version)

    # Assert
    assert file_path.read() == new_version


def test_replace_in_file_no_occurrence(tmpdir):
    # Arrange
    old_version = "0.1.0"
    new_version = "0.2.0"
    file_path = tmpdir.join("VERSION")
    file_path.write("0.2.0")

    # Act
    with pytest.raises(ValueError):
        replace_in_file(file_path, old_version, new_version)


def test_replace_regex_in_file(tmpdir):
    # Arrange
    file_path = tmpdir.join("test_file.txt")
    file_path.write("Version: 1.2.3")
    pattern = r"\d+\.\d+\.\d+"
    new_version = "2.0.0"

    # Act
    replace_regex_in_file(str(file_path), pattern, new_version)

    # Assert
    assert file_path.read() == "Version: 2.0.0"


def test_replace_in_pyproject(tmpdir):
    # Create a temporary pyproject.toml file
    pyproject_file = tmpdir.join("pyproject.toml")
    pyproject_file.write('[tool.poetry]\nname = "my-package"\nversion = "1.0.0"')

    # Call the function to update the version
    new_version = "2.0.0"
    replace_in_pyproject(str(pyproject_file), new_version)

    # Read the updated pyproject.toml file
    with open(str(pyproject_file), "r") as file:
        updated_content = file.read()

    # Check if the version is updated
    assert 'version = "2.0.0"' in updated_content


# --- Hooks --------------------------------------------------------------------


def test_run_hook_invalid():
    with pytest.raises(ValueError):
        run_hook("invalid_hook", new_version="0.1.0", current_version="0.0.1")


def test_run_hook_string_replace(tmpdir):
    file_content = "test\nversion: 0.0.1\ntest"
    expected_content = "test\nversion: 0.1.0\ntest"

    # Create file:
    file_path = tmpdir.join("test.txt")
    file_path.write(file_content)

    hook = {"method": "string_replace", "file_path": file_path}
    run_hook(hook, new_version="0.1.0", current_version="0.0.1")

    assert file_path.read() == expected_content


def test_run_hook_regex_replace(tmpdir):
    # Create a test file
    test_file = tmpdir.join("testfile.txt")
    test_file.write("The initial version is 1.0.0.")

    # Define the file path, pattern, and new version
    file_path = str(test_file)
    pattern = r"1\.0\.0"
    new_version = "2.0.0"

    # Call the function to replace the version
    replace_regex_in_file(file_path, pattern, new_version)

    # Read the modified file
    modified_content = test_file.read()

    # Assert to check if the replacement was successful
    assert (
        modified_content == "The initial version is 2.0.0."
    ), f"Expected 'The initial version is 2.0.0.', but got '{modified_content}'"


def test_run_hook_pyproject_replace(tmpdir):
    # Create a temporary pyproject.toml file
    pyproject_file = tmpdir.join("pyproject.toml")
    pyproject_file.write('[tool.poetry]\nname = "my-package"\nversion = "1.0.0"')

    # Call the function to update the version
    new_version = "2.0.0"
    hook = {"method": "pyproject_replace", "file_path": str(pyproject_file)}
    run_hook(hook, new_version=new_version, current_version="1.0.0")

    # Read the updated pyproject.toml file
    with open(str(pyproject_file), "r") as file:
        updated_content = file.read()

    # Check if the version is updated
    assert 'version = "2.0.0"' in updated_content

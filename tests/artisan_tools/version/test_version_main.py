import pytest
import unittest
from artisan_tools.version.main import bump_version, check_version, replace_in_file


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


def test_replace_in_file_simple(tmpdir):
    # Arrange
    old_version = "0.1.0"
    new_version = "0.2.0"
    file_path = tmpdir.join("VERSION")
    file_path.write(old_version)

    # Act
    replace_in_file(file_path, old_version, new_version)

    # Assert
    assert file_path.read() == new_version

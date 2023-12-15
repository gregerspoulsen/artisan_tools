import unittest

from artisan_tools.release import check_version, check_version_file


class TestVersionCheck(unittest.TestCase):
    def test_proper_release(self):
        # Test with a proper release version
        self.assertTrue(check_version("1.2.3"))

    def test_pre_release(self):
        # Test with a pre-release version
        self.assertFalse(check_version("1.2.3-alpha.1"))

    # def test_build_metadata(self):
    #     # Test with a version that includes build metadata
    #     self.assertFalse(check_version("1.2.3+build.4"))

    def test_invalid_version(self):
        # Test with an invalid version string
        self.assertFalse(check_version("invalid-version"))


def test_version_check_file(tmpdir):
    temp_file = tmpdir.join("version.txt")
    temp_file.write("1.2.3rc1")
    assert check_version_file(str(temp_file)) is False

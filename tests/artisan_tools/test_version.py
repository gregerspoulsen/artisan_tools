import pytest
from artisan_tools.version.version import bump_version, bump_version_file


def test_bump_version_major():
    assert bump_version("1.2.3", "major") == "2.0.0"


def test_bump_version_minor():
    assert bump_version("1.2.3", "minor") == "1.3.0"


def test_bump_version_patch():
    assert bump_version("1.2.3", "patch") == "1.2.4"


def test_bump_version_invalid_part():
    with pytest.raises(ValueError):
        bump_version("1.2.3", "invalid_part")


def test_read_version_file(tmpdir):
    temp_file = tmpdir.join("version.txt")
    temp_file.write("1.2.3")
    assert bump_version_file(str(temp_file), "minor") == "1.3.0"


def test_bump_version_file(tmpdir):
    temp_file = tmpdir.join("version.txt")
    temp_file.write("1.2.3")
    new_version = bump_version_file(str(temp_file), "minor")
    assert new_version == "1.3.0"
    assert temp_file.read() == "1.3.0"


def test_bump_version_file_nonexistent(tmpdir):
    nonexistent_file = tmpdir.join("nonexistent_file.txt")
    with pytest.raises(FileNotFoundError):
        bump_version_file(str(nonexistent_file), "minor")

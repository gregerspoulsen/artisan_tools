import re

from artisan_tools.version.api import get_version, bump, update


# --- get_version --------------------------------------------------------------
def test_get_version(tmpdir, app):
    # Create a custom version file in the tmpdir
    custom_version_file = tmpdir.join("custom_version.txt")
    custom_version_file.write("2.4.6")

    app.config["version"]["current"] = str(custom_version_file)
    assert get_version(app) == "2.4.6"


# --- Replace_version -----------------------------------------------------------
def test_update_part(app_with_config):
    bump(app_with_config, "minor")
    update(app_with_config, release=True)
    assert get_version(app_with_config) == "0.100.0"


def test_update_full(app_with_config):
    bump(app_with_config, "1.2.3-rc1")
    update(app_with_config, release=True)
    assert get_version(app_with_config) == "1.2.3-rc1"


def test_update_with_hook(app_with_config, tmpdir):
    # Create file:
    test_file = tmpdir.join("hook.txt")
    test_file.write("afsdf\nversion: 0.99.9\nasdfasdf")

    hook = {
        "method": "regex_replace",
        "file_path": "hook.txt",
        "pattern": r"\d+\.\d+\.\d+",
    }
    app_with_config.config["version"]["hooks"] = [hook]

    # Act
    bump(app_with_config, "minor")

    # Assert
    assert test_file.read() == "afsdf\nversion: 0.100.0\nasdfasdf"


def test_update_release(app_with_config):
    update(app_with_config, release=True)
    assert get_version(app_with_config) == "0.99.9"


def test_update_non_release(app_with_repo):
    update(app_with_repo)

    version = get_version(app_with_repo)
    print(get_version(app_with_repo))
    assert re.match(r"^0.99.9\+master\+\w{7}$", version)

    # Modify the repo:
    with open("file.txt", "a") as f:
        f.write("New line")

    update(app_with_repo)
    version = get_version(app_with_repo)
    assert re.match(r"^0.99.9\+master\+\w{7}\+dirty$", version)

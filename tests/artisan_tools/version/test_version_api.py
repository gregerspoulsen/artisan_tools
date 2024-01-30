from artisan_tools.version.api import get_version, update_version


# --- get_version --------------------------------------------------------------
def test_get_version(tmpdir, app):
    # Create a custom version file in the tmpdir
    custom_version_file = tmpdir.join("custom_version.txt")
    custom_version_file.write("2.4.6")

    app.config["version"]["file"] = str(custom_version_file)
    assert get_version(app) == "2.4.6"


# --- update_version -----------------------------------------------------------
def test_update_part(app_with_config):
    update_version(app_with_config, "minor")
    assert get_version(app_with_config) == "0.100.0"


def test_update_full(app_with_config):
    update_version(app_with_config, "1.2.3-rc1")
    assert get_version(app_with_config) == "1.2.3-rc1"


def test_update_with_hook(app_with_config, tmpdir):
    # Create file:
    test_file = tmpdir.join("hook.txt")
    test_file.write("afsdf\nversion: 0.99.9\nasdfasdf")

    hook = {"method": "string_replace", "file_path": test_file}
    app_with_config.config["version"]["hooks"] = [hook]

    # Act
    update_version(app_with_config, "minor")

    # Assert
    assert test_file.read() == "afsdf\nversion: 0.100.0\nasdfasdf"

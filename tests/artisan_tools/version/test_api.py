from artisan_tools.version.api import get_version

# --- get_version ---------------------------------------------------------- ###
def test_get_version(tmpdir, app):
    # Create a custom version file in the tmpdir
    custom_version_file = tmpdir.join("custom_version.txt")
    custom_version_file.write("2.4.6")

    app.config["version"]["file"] = str(custom_version_file)
    # Check if the version is read from the custom file
    print(app.config)
    assert get_version(app) == "2.4.6"
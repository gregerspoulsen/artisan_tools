from artisan_tools.parser.api import parse


def test_parse_replaces_version_tag(app_with_config):
    app = app_with_config
    version = app.get_extension("version").get_version(app)

    input_string = "The current version is v@version."
    expected_output = f"The current version is v{version}."

    result = parse(app, input_string)

    assert (
        result == expected_output
    ), "The @version tag should be replaced with the actual version."


def test_parse_empty_string(app_with_config):
    # Arrange
    input_string = ""
    expected_output = ""

    # Act
    result = parse(app_with_config, input_string)

    # Assert
    assert result == expected_output, "An empty string should return an empty string."

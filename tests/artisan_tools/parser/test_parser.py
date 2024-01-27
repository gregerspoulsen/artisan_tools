from artisan_tools.parser.api import parse


def test_parse_replaces_version_tag(app):
    version = app.get_extension("version").get_version(app)

    input_string = "The current version is v@version."
    expected_output = f"The current version is v{version}."

    result = parse(app, input_string)

    assert (
        result == expected_output
    ), "The @version tag should be replaced with the actual version."


def test_parse_empty_string(app):
    # Arrange
    input_string = ""
    expected_output = ""

    # Act
    result = parse(app, input_string)

    # Assert
    assert result == expected_output, "An empty string should return an empty string."

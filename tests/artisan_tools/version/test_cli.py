import pytest

from typer.testing import CliRunner

from artisan_tools.version.cli import (
    factory,
)  # Replace with the actual import from your CLI script

runner = CliRunner()


def test_bump_with_default_file(tmpdir, app):
    # Create the default 'VERSION' file in the tmpdir
    version_file = tmpdir.join("VERSION")
    version_file.write("1.2.3")

    # Change the current working directory to tmpdir
    with tmpdir.as_cwd():
        # Run the CLI command without specifying the file path
        result = runner.invoke(factory(app), ["bump", "minor"], catch_exceptions=False)

        # Check if the command was successful and output is correct
        assert result.exit_code == 0
        assert "Version bumped to 1.3.0 in ./VERSION" in result.stdout
        assert version_file.read() == "1.3.0"


def test_bump_with_specified_file(tmpdir, app):
    # Create a custom version file in the tmpdir
    custom_version_file = tmpdir.join("custom_version.txt")
    custom_version_file.write("2.4.6")

    # Run the CLI command specifying the custom file path
    result = runner.invoke(
        factory(app),
        ["bump", "patch", str(custom_version_file)],
        catch_exceptions=False,
    )

    # Check if the command was successful and output is correct
    assert result.exit_code == 0
    assert "Version bumped to 2.4.7 in " in result.stdout
    assert custom_version_file.read() == "2.4.7"


def test_bump_nonexistent_file(tmpdir, app):
    # Run the CLI command with a non-existent file
    result = runner.invoke(
        factory(app), ["bump", "minor", str("nonexistent_file")], catch_exceptions=False
    )

    # Check if the correct error message is displayed
    assert result.exit_code != 0


def test_bump_bad_part(tmpdir, app):
    # Run the CLI command with a bad part
    result = runner.invoke(factory(app), ["bump", "bad_part"], catch_exceptions=False)

    # Check if the correct error message is displayed
    assert result.exit_code != 0


# Test CLI with version argument
@pytest.mark.parametrize(
    "param",
    [
        {"version": "1.0.0", "exit_code": 0},
        {"version": "1.0.0.rc1", "exit_code": 1},
    ],
)
def test_verify(app, tmpdir, monkeypatch, param):
    file = tmpdir.join("VERSION")
    monkeypatch.setitem(app.config, "version", {"file": str(file)})

    # Test with valid version
    file.write(param["version"])

    result = runner.invoke(factory(app), ["verify"], catch_exceptions=False)
    assert result.exit_code == param["exit_code"], result.output

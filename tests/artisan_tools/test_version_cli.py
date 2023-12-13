from typer.testing import CliRunner
from artisan_tools.cli import app  # Replace with the actual import from your CLI script

runner = CliRunner()


def test_cli_with_default_file(tmpdir):
    # Create the default 'VERSION' file in the tmpdir
    version_file = tmpdir.join("VERSION")
    version_file.write("1.2.3")

    # Change the current working directory to tmpdir
    with tmpdir.as_cwd():
        # Run the CLI command without specifying the file path
        result = runner.invoke(app, ["bump", "minor"])

        # Check if the command was successful and output is correct
        assert result.exit_code == 0
        assert "Version bumped to 1.3.0 in VERSION" in result.stdout
        assert version_file.read() == "1.3.0"


def test_cli_with_specified_file(tmpdir):
    # Create a custom version file in the tmpdir
    custom_version_file = tmpdir.join("custom_version.txt")
    custom_version_file.write("2.4.6")

    # Run the CLI command specifying the custom file path
    result = runner.invoke(app, ["bump", "patch", str(custom_version_file)])

    # Check if the command was successful and output is correct
    assert result.exit_code == 0
    assert "Version bumped to 2.4.7 in " in result.stdout
    assert custom_version_file.read() == "2.4.7"


def test_cli_nonexistent_file(tmpdir):
    # Run the CLI command with a non-existent file
    result = runner.invoke(app, ["bump", "minor", str("nonexistent_file")])

    # Check if the correct error message is displayed
    assert result.exit_code != 0


def test_bad_part(tmpdir):
    # Run the CLI command with a bad part
    result = runner.invoke(app, ["bump", "bad_part"])

    # Check if the correct error message is displayed
    assert result.exit_code != 0

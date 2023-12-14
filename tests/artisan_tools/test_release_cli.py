from typer.testing import CliRunner
from artisan_tools.cli import (
    app,
)

runner = CliRunner()


# Test CLI with version argument
def test_cli_with_version():
    result = runner.invoke(app, ["verify-version", "--version", "1.0.0"])
    print(result.output)
    assert result.exit_code == 0
    assert "Version is a proper semver release version." in result.output

    result = runner.invoke(app, ["verify-version", "--version", "1.0.0-alpha"])
    assert result.exit_code == 1
    assert "Invalid semver version." in result.output


# Test CLI with file_path argument
def test_cli_with_file_path(tmpdir):
    file = tmpdir.join("version.txt")

    # Test with valid version
    file.write("1.0.0")
    result = runner.invoke(app, ["verify-version", "--file-path", file.strpath])
    assert result.exit_code == 0
    assert "Version is a proper semver release version." in result.output

    # Test with invalid version
    file.write("invalid")
    result = runner.invoke(app, ["verify-version", "--file-path", file.strpath])
    assert result.exit_code == 1
    assert "Invalid semver version." in result.output

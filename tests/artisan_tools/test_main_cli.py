from typer.testing import CliRunner
from artisan_tools.main_cli import cli


def test_version_command():
    # Arrange
    runner = CliRunner()

    # Act
    result = runner.invoke(cli, ["--version"])

    # Assert
    assert result.exit_code == 0
    with open("VERSION", "r") as file:
        assert result.output.strip() == file.read().strip()

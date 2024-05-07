from typer.testing import CliRunner
from artisan_tools.main_cli import cli
import re


def test_version_command():
    # Arrange
    runner = CliRunner()

    # Act
    result = runner.invoke(cli, ["--version"])

    # Assert
    assert result.exit_code == 0
    assert re.match(
        r"^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$",
        result.stdout,
    )

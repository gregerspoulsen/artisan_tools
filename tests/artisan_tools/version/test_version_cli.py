from typer.testing import CliRunner

from artisan_tools.version.cli import (
    factory,
)

runner = CliRunner()


def test_bump(tmpdir, app_with_config):
    app = app_with_config
    result = runner.invoke(factory(app), ["bump", "major"], catch_exceptions=False)

    # Check if the command was successful and output is correct
    assert result.exit_code == 0
    app.get_extension("version").update(app, release=True)
    version = app.get_extension("version").get_version(app)
    assert version == "1.0.0"


def test_get(tmpdir, app_with_config):
    app = app_with_config
    result = runner.invoke(factory(app), ["get"], catch_exceptions=False)

    # Check if the command was successful and output is correct
    assert result.exit_code == 0
    version = app.get_extension("version").get_version(app)
    assert result.output == version

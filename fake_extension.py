"""
Simple extension to be used in tests
"""

import typer


def test_function(*args, **kwargs):
    return args, kwargs


cli = typer.Typer(name="test_cli", help="Test CLI")


def setup(app):
    app.register_extension("test_ext", {"test_function": test_function})
    app.add_cli(cli)

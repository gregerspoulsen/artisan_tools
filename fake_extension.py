"""
Simple extension to be used in tests.
"""

import typer


def test_function(*args, **kwargs):
    """
    Test Function.
    """
    return args, kwargs


cli = typer.Typer(name="test_cli", help="Test CLI")


def setup(app):
    """
    Setup Fake Extension.
    """
    app.register_extension("test_ext", {"test_function": test_function})
    app.add_cli(cli)

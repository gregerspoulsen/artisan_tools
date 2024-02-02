"""
Simple file for exposing click object to sphinx.
"""

import artisan_tools.app
import typer

app = artisan_tools.app.App()
app.load_extensions()
click = typer.main.get_command(app.cli)

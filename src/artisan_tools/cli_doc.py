"""
Simple file for exposing click object to sphinx.
"""

import artisan_tools.app
import typer
import os

# Create empty artisan.yaml file if it doesn't exist:
if not os.path.exists("artisan.yaml"):
    with open("artisan.yaml", "w") as f:
        f.write("")

app = artisan_tools.app.App()
app.load_extensions()
click = typer.main.get_command(app.cli)

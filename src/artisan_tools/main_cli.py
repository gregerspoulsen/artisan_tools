"""
CLI exposed directly by artisan-tools.
"""

import typer
from importlib.metadata import version, PackageNotFoundError

cli = typer.Typer(name="artisan tools")


def version_callback(value: bool):
    """
    Print the current version of artisan-tools.
    """
    if value:
        try:
            version_number = version("artisan-tools")
        except PackageNotFoundError:
            version_number = "Package not found"
        typer.echo(f"{version_number}")
        raise typer.Exit()


@cli.callback()
def main(
    version: bool = typer.Option(
        None,
        "--version",
        callback=version_callback,
        is_eager=True,
        help="Show artisan-tools version",
    )
):
    """
    Artisan Tools CLI.
    """
    pass

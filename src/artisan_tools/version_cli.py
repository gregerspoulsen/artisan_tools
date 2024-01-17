import typer

from artisan_tools.version import bump_version_file
from artisan_tools import get_config

app = typer.Typer(
    name="version", help=("Commands for retrieving and settings version" "information.")
)

config = get_config()


@app.command()
def bump(
    part: str = typer.Argument(  # noqa: B008
        ..., help="The part of the version to bump ('major', 'minor', or 'patch')."
    ),
    file_path: str = typer.Argument(  # noqa: B008
        None,
        help="Path to the file containing the version string. Defaults to 'VERSION'.",
    ),
):
    """
    Bump the version in the specified file.
    """
    if file_path is None:
        file_path = config["version"]["file"]
    try:
        new_version = bump_version_file(file_path, part)
        typer.echo(f"Version bumped to {new_version} in {file_path}")
    except FileNotFoundError:
        raise typer.BadParameter(f"File not found: {file_path}")

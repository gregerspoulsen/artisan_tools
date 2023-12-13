import typer
from artisan_tools.version import bump_version_file

app = typer.Typer()


@app.command()
def bump(
    part: str = typer.Argument(  # noqa: B008
        ..., help="The part of the version to bump ('major', 'minor', or 'patch')."
    ),
    file_path: str = typer.Argument(  # noqa: B008
        "VERSION",
        help="Path to the file containing the version string. Defaults to 'VERSION'.",
    ),
):
    """
    Bump the version in the specified file.
    """
    try:
        new_version = bump_version_file(file_path, part)
        typer.echo(f"Version bumped to {new_version} in {file_path}")
    except FileNotFoundError:
        raise typer.BadParameter(f"File not found: {file_path}")


@app.command()
def hello(name: str):
    typer.echo(f"Hello {name}")


if __name__ == "__main__":
    app()

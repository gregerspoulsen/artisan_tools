import typer

import artisan_tools.vcs_cli
import artisan_tools.release
import artisan_tools.version_cli

app = typer.Typer()

app.add_typer(artisan_tools.version_cli.app)

app.add_typer(artisan_tools.vcs_cli.app)


@app.command()
def verify_version(
    version: str = typer.Option(  # noqa: B008
        None, help="The semver version string to check."
    ),
    file: str = typer.Option(  # noqa: B008
        None, help="The path to the file containing the version string."
    ),
    check_tag: bool = typer.Option(  # noqa: B008
        False, help="Check that current versions isn't already a tag"
    ),
):
    """
    Verify if a given version or version in a file is a proper semver release version.

    :param version: The semver version string to check.
    :param file_path: The path to the file containing the version string.
    :param check_tag: Check that current versions isn't already a tag
    """
    if version:
        result = artisan_tools.release.check_version(version)
    elif file:
        result = artisan_tools.release.check_version_file(file)
    else:
        typer.echo("Please provide either a version string or a file path.")
        raise typer.Exit(code=1)

    if result:
        typer.secho(
            "Version is a proper semver release version.", fg=typer.colors.GREEN
        )
    else:
        typer.secho("Invalid semver version.", fg=typer.colors.RED)
        raise typer.Exit(code=1)

    if check_tag:
        with open(file, "r") as file:
            version = "v" + file.read().strip()
        if artisan_tools.vcs.check_tag(version):
            typer.secho(f"Tag '{version}' already exists.", fg=typer.colors.RED)
            raise typer.Exit(code=2)
        else:
            typer.secho(f"Tag '{version}' does not exist.", fg=typer.colors.GREEN)
    typer.secho("All checks passed.", fg=typer.colors.GREEN)


if __name__ == "__main__":
    app()

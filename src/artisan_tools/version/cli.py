import typer

from artisan_tools.version.main import bump_version_file, check_version
from artisan_tools.version.api import get_version


def factory(app):
    cli = typer.Typer(
        name="version",
        help=("Commands for retrieving and settings version" "information."),
    )

    @cli.command()
    def bump(
        part: str = typer.Argument(  # noqa: B008
            ..., help="The part of the version to bump ('major', 'minor', or 'patch')."
        ),
        file_path: str = typer.Argument(  # noqa: B008
            None,
            help="Path to file containing the version string. Defaults to 'VERSION'.",
        ),
    ):
        """
        Bump the version in the specified file.
        """
        if file_path is None:
            file_path = app.config["version"]["file"]
        try:
            new_version = bump_version_file(file_path, part)
            typer.echo(f"Version bumped to {new_version} in {file_path}")
        except FileNotFoundError:
            raise typer.BadParameter(f"File not found: {file_path}")

    @cli.command()
    def verify(
        check_tag: bool = typer.Option(  # noqa: B008
            False, help="Check that current versions isn't already a tag"
        ),
    ):
        """
        Verify if the current version is valid.

        :param check_tag: Check that current versions isn't already a tag
        """
        version = get_version(app)
        result = check_version(version)

        if result:
            typer.secho(
                "Version is a proper semver release version.", fg=typer.colors.GREEN
            )
        else:
            typer.secho("Invalid semver version.", fg=typer.colors.RED)
            raise typer.Exit(code=1)

        if check_tag:
            vcs_version = "v" + version
            vcs = app.get_extension("vcs")
            if vcs.check_tag(vcs_version):
                typer.secho(f"Tag '{vcs_version}' already exists.", fg=typer.colors.RED)
                raise typer.Exit(code=2)
            else:
                typer.secho(
                    f"Tag '{vcs_version}' does not exist.", fg=typer.colors.GREEN
                )
        typer.secho("All checks passed.", fg=typer.colors.GREEN)

    return cli

import typer

from artisan_tools.version.main import check_version
from artisan_tools.version.api import get_version, update_version


def factory(app):
    cli = typer.Typer(
        name="version",
        help=("Tools for managing version information."),
    )

    @cli.command()
    def bump(
        part: str = typer.Argument(  # noqa: B008
            ..., help=("Part to bump [major|minor|patch] or a full version " "string.")
        ),
    ):
        """
        Bump the version in the specified file.
        """
        new_version = update_version(app, part)
        typer.secho(f"Version bumped to {new_version}", fg=typer.colors.GREEN)

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

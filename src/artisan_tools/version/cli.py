import typer
from rich import print as rprint
from artisan_tools.version.main import check_version
from artisan_tools.version.api import get_version, update_version


def factory(app):
    """
    Create CLI for version extension.
    """
    cli = typer.Typer(
        name="version",
        help="Tools for managing version information.",
    )

    @cli.command()
    def bump(
        part: str = typer.Argument(  # noqa: B008
            ..., help="Part to bump [major|minor|patch] or a full version string."
        ),
    ):
        """
        Bump the version in the specified file.
        """
        new_version = update_version(app, part)
        rprint(f"[green]Version bumped to {new_version}")

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
            rprint(f"[green]{version} is a proper semver release")
        else:
            rprint(f"[bold red]Invalid semver version: {version}.")
            raise typer.Exit(code=1)

        if check_tag:
            vcs_version = "v" + version
            vcs = app.get_extension("vcs")
            if vcs.check_tag(vcs_version):
                rprint(f"[bold red]Tag '{vcs_version}' already exists.")
                raise typer.Exit(code=2)
            else:
                rprint(f"[green]Tag '{vcs_version}' does not exist.")
        rprint("[bold green]Verification passed.")

    @cli.command()
    def get():
        """
        Print the current version to stdout.
        """
        version = get_version(app)
        print(f"{version}", end="")

    return cli

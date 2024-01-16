import typer
from typing import Optional

import artisan_tools.vcs
import artisan_tools.release
import artisan_tools.version_cli

app = typer.Typer()

app.add_typer(artisan_tools.version_cli.app)

@app.command()
def check_no_tag(
    tag: str = typer.Argument(  # noqa: B008
        ..., help="The tag to check in the remote repository."
    )
):
    """
    Check if a specific tag exists in the remote Git repository.
    """
    if artisan_tools.vcs.check_tag(tag):
        typer.secho(
            f"Tag '{tag}' already exists in the remote repository.",
            fg=typer.colors.RED,
            bold=True,
        )
        raise typer.Exit(code=1)


@app.command()
def check_branch(
    expected_branch: str = typer.Argument(  # noqa: B008
        ..., help="The branch name to check."
    )
):
    """
    Check if the current Git branch is the specified branch.
    """
    if artisan_tools.vcs.check_current_branch(expected_branch):
        print(f"Current branch is '{expected_branch}'.")
        raise typer.Exit(code=0)
    else:
        print(f"Current branch is not '{expected_branch}'.")
        raise typer.Exit(code=1)


@app.command()
def add_release_tag(
    file: str = typer.Argument(  # noqa: B008
        "VERSION",
        help="Path to the file containing the version string. Defaults to 'VERSION'.",
    ),
    git_options: Optional[str] = typer.Option(  # noqa: B008
        None, help="Git options to pass on, e.g. '-c user.name=John -c user.email=None'"
    ),
):
    """
    Add a tag to the current commit and push it to a remote Git repository.
    """
    version = artisan_tools.version.read_version_file(file)
    tag = "v" + version
    check_no_tag(tag)

    artisan_tools.vcs.add_and_push_tag(
        tag_name=tag, message=f"Tag Release v{version}", git_options=git_options
    )
    typer.echo(f"Tagged current changeset as '{tag}' and pushed to remote repository.")


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

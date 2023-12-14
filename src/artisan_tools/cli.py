import typer
from artisan_tools.version import bump_version_file
import artisan_tools.vcs
from typing import Optional

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
def check_tag(
    tag: str = typer.Argument(  # noqa: B008
        ..., help="The tag to check in the remote repository."
    )
):
    """
    Check if a specific tag exists in the remote Git repository.
    """
    if artisan_tools.vcs.check_tag(tag):
        print(f"Tag '{tag}' exists in the remote repository.")
        typer.Exit(code=0)
    else:
        print(f"Tag '{tag}' does not exist in the remote repository.")
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
def add_tag(
    tag_name: str = typer.Argument(..., help="The tag name to be added"),  # noqa: B008
    message: str = typer.Argument(..., help="The message for the tag"),  # noqa: B008
    remote: Optional[str] = typer.Option(  # noqa: B008
        "origin", help="The remote name, defaults to 'origin'"
    ),
    git_user_name: Optional[str] = typer.Option(  # noqa: B008
        None, help="Git user name for commit"
    ),
    git_user_email: Optional[str] = typer.Option(  # noqa: B008
        None, help="Git user email for commit"
    ),
):
    """
    Add a tag to the current commit and push it to a remote Git repository.
    """
    git_config = {}
    if git_user_name:
        git_config["user.name"] = git_user_name
    if git_user_email:
        git_config["user.email"] = git_user_email

    artisan_tools.vcs.add_and_push_tag(tag_name, message, remote, git_config=git_config)
    typer.echo(
        f"Tag '{tag_name}' has been successfully added and pushed to remote '{remote}'."
    )


if __name__ == "__main__":
    app()

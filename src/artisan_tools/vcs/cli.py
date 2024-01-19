import typer

import artisan_tools.vcs.main

import artisan_tools

def factory(app):

    cli = typer.Typer(
        name="vcs", help=("Commands for retrieving and settings version" "information.")
    )


    @cli.command()
    def check_no_tag(
        tag: str = typer.Argument(  # noqa: B008
            ..., help="The tag to check in the remote repository."
        )
    ):
        """
        Check if a specific tag exists in the remote Git repository.
        """
        if artisan_tools.vcs.main.check_tag(tag):
            typer.secho(
                f"Tag '{tag}' already exists in the remote repository.",
                fg=typer.colors.RED,
                bold=True,
            )
            raise typer.Exit(code=1)


    @cli.command()
    def check_branch(
        expected_branch: str = typer.Argument(  # noqa: B008
            ..., help="The branch name to check."
        )
    ):
        """
        Check if the current Git branch is the specified branch.
        """
        if artisan_tools.vcs.main.check_current_branch(expected_branch):
            print(f"Current branch is '{expected_branch}'.")
            raise typer.Exit(code=0)
        else:
            print(f"Current branch is not '{expected_branch}'.")
            raise typer.Exit(code=1)


    @cli.command()
    def add_release_tag():
        """
        Add a tag to the current commit and push it to a remote Git repository.
        """
        version = app.get_extension('version').get_version(app)
        tag = "v" + version
        check_no_tag(tag)

        artisan_tools.vcs.main.add_and_push_tag(app.config['vcs'],
            tag_name=tag, message=f"Tag Release v{version}"
        )
        typer.echo(f"Tagged current changeset as '{tag}' and pushed to remote repository.")

    return cli
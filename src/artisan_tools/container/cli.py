import typer
import subprocess
import typing
from artisan_tools import error

from artisan_tools.container import api


def factory(app):
    cli = typer.Typer(name="container", help="Tools for container images")

    @cli.command()
    def login():
        """
        Login to container registry as specified in the configuration file.
        When `auth == direct`, use the `user` and `token` directly.
        When `auth == env`, use the environment variables specified in `user`
        and `token`
        """
        api.login(app)
        typer.echo("Successfully logged in to {config['registry']}")

    @cli.command()
    def logout():
        """
        Log out of the container registry. Registry and engine are read from
        the configuration file.
        """
        api.logout(app)
        typer.echo("Successfully logged out of {config['registry']}")

    @cli.command()
    def push(source: str, target: str, tags: list[str]) -> None:
        """
        Push a Docker image to a container registry. Logging in/out is handled
        automatically using with details from the configuration file.

        Parameters
        ----------
        source : str
            The source image to push, can contain tags.
        target : str
            The target image to push to, must not include tags
        tags : list[str]
            List of tags to push to the target image. Tags will be parsed by
            the parser extension.
        """

        api.push(app, source, target, tags)
        typer.secho(
            f"Successfully pushed {source} to {target} with tags {tags}",
            fg=typer.colors.GREEN,
        )

    @cli.command()
    def command(command: str):
        """
        Run a command with authentication to container registry
        """
        try:
            api.run_command_with_auth(app, command)
        except subprocess.CalledProcessError as e:
            typer.secho("Error running command", fg=typer.colors.RED)
            raise typer.Exit(code=e.returncode)

    @cli.command()
    def build_push(
        repository: str = typer.Argument(..., help="The repository to push to."),
        tags: typing.List[str] = typer.Argument(
            ..., help="List of tags. Tags will be parsed by the parser extension"
        ),
        platform: typing.List[str] = typer.Option(
            ["linux/amd64"],
            "--platform",
            help=(
                "Platform to build for. Can be used multiple times. Default is"
                " ('linux/amd64')."
            ),
        ),
        context: str = typer.Option(
            ".", "--context", help="The build context. Default is current directory."
        ),
        options: typing.List[str] = typer.Option(
            [], "--options", help="Additional options to pass to the build command."
        ),
    ):
        """
        Build and push a container image to a container registry.
        Example: `build-push ghcr.io/user/test tag1 tag2 --platform linux/amd64
          --platform linux/arm64`
        """
        typer.echo((
            f"Preparing to build an push to {repository} with tags {tags} "
            "for platforms {platform}"
        ))
        try:
            api.build_push(app, repository, tags, platform, context, options)
        except error.ExternalError as e:
            typer.secho("Error building and pushing image", fg=typer.colors.RED)
        typer.secho(
            f"Successfully built and pushed {repository} with tags {tags}",
            fg=typer.colors.GREEN,
            )

    return cli


if __name__ == "__main__":
    from artisan_tools.app import App

    app = App()
    app.load_extensions()
    cli = factory(app)
    cli()

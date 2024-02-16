import typer
import subprocess
import typing


from artisan_tools import error

from artisan_tools.container import api


def factory(app):
    """
    Create CLI for container extension.
    """
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
        Log out of the container registry.

        Registry and engine are read from the configuration file.
        """
        api.logout(app)
        typer.echo("Successfully logged out of {config['registry']}")

    @cli.command()
    def push(source: str, target: str, tags: typing.List[str]):
        """
        Pushes a Docker image to a container registry.

        Args:
            source: The source image to push, can contain tags.
            target: The target image to push to, must not include tags.
            tags: List of tags to push to the target image. Tags will be parsed by
                the parser extension.

        Returns:
            None: This method does not return anything.

        Raises:
            None: This method does not raise any exceptions.
        """
        api.push(app, source, target, tags)
        typer.secho(
            f"Successfully pushed {source} to {target} with tags {tags}",
            fg=typer.colors.GREEN,
        )

    @cli.command()
    def command(command: str):
        """
        Run a command with authentication to container registry.
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
            default=None,
            help=(
                "List of tags. Tags will be parsed by the parser extension, if no tags"
                " are provided the containers are just build"
            ),
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
        option: typing.List[str] = typer.Option(
            [],
            "--option",
            help=(
                "Additional options to pass to the build command, can be "
                "used multiple times"
            ),
        ),
    ):
        """
        Build (and push) a container image to a container registry.

        Example: ``build-push ghcr.io/user/test tag1 tag2 --platform linux/amd64
          --platform linux/arm64 --option "--file=/path/to/Dockerfile"``
        """
        typer.echo(
            f"Preparing to build an push to {repository} with tags {tags} "
            f"for platforms {platform} and options {option}"
        )
        try:
            api.build_push(
                app, repository, tags, tuple(platform), context, tuple(option)
            )
        except error.ExternalError:
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

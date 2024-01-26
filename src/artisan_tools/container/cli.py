import typer

from artisan_tools.container import api


def factory(app):
    cli = typer.Typer(name="container", help=("Tools for container images"))

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

    return cli


if __name__ == "__main__":
    from artisan_tools.app import App

    app = App()
    app.load_extensions()
    cli = factory(app)
    cli()

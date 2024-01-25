import typer
import os

import artisan_tools.container.main as atc


def factory(app):
    cli = typer.Typer(
        name="container", help=("Commands for working with container images")
    )
    config = app.config["container"]

    @cli.command()
    def login():
        """
        Log in to the container registry. Username, token variable, registry
        and engine are read from the configuration file.
        """
        # Read token from environment variable
        token = os.getenv(config["token_var"])
        if token is None:
            typer.secho(
                (
                    f"Error, environment variable {config['token_var']} not set, "
                    "it should contain token for logging in to registry."
                ),
                fg=typer.colors.RED,
            )
            raise typer.Exit(code=5)

        logged_in = atc.login(
            username=config["user"],
            token=token,
            registry=config["registry"],
            engine=config["engine"],
            options=config["options"],
        )
        typer.echo("Successfully logged in to {config['registry']}")
        return logged_in

    @cli.command()
    def logout():
        """
        Log out of the container registry. Registry and engine are read from
        the configuration file.
        """
        atc.logout(
            registry=config["registry"],
            engine=config["engine"],
        )
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
            List of tags to push to the target image.
        """

        # Check that target doesn't contain tags, that is no colons after
        # the last slash:
        if target.split("/")[-1].count(":") > 0:
            typer.secho(
                "Error, target image must not contain tags", fg=typer.colors.RED
            )
            raise typer.Exit(code=10)

        targets = [target + ":" + tag for tag in tags]

        logged_in = login()

        for target in targets:
            atc.push(
                source=source,
                target=target,
                engine=config["engine"],
                options=config["options"],
            )
            typer.secho(
                f"Successfully pushed {source} to {target}", fg=typer.colors.GREEN
            )
        if logged_in:
            logout()

    return cli


if __name__ == "__main__":
    from artisan_tools.app import App

    app = App()
    app.load_extensions()
    cli = factory(app)
    cli()

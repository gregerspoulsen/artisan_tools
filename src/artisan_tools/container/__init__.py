from . import cli


def setup(app):
    """
    Setup the version module.
    """
    app.add_cli(cli.factory(app))

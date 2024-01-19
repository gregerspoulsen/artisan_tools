from . import api
from . import cli

def setup(app):
    """
    Setup the version module.
    """
    app.register_extension("vcs", api)

    app.add_cli(cli.factory(app))
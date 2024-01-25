from . import api


def setup(app):
    """
    Setup the version module.
    """
    app.register_extension("parser", api)

from .version import (  # noqa: F401
    get_version,
)


def get_typer_app():
    from .version_cli import app

    return app

from artisan_tools.app import App


def parse(app: App, string: str) -> str:
    """
    Parse string to perform replacements.

    The following replacements are performed:
    - @version: The current version

    Parameters
    ----------
    app : App
        The application instance.
    string : str
        The string to parse.
    """
    version = app.get_extension("version").get_version(app)  # type: ignore[attr-defined]
    return string.replace("@version", version)

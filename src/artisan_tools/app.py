import typer
import types
import importlib
from .log import get_logger
from .config import load_config

_std_extensions = [
    "artisan_tools.vcs",
    "artisan_tools.version",
]


class App:
    def __init__(self):
        self.cli = typer.Typer()
        self.extensions = {}
        self.logger = get_logger("App")
        self.config = load_config()
        self._load_extensions()

    def _load_extensions(self):
        """
        Load standard extensions and extensions specified in the config.
        """
        for extension in _std_extensions + self.config["extensions"]:
            ext = importlib.import_module(extension)
            ext.setup(self)

    def add_cli(self, cli: typer.Typer):
        """
        Add a (sub)cli to this app.

        Parameters:
        cli (typer.Typer): The cli to add.
        """
        self.cli.add_typer(cli)
        self.logger.debug(f"Added sub-cli: {cli}")

    def register_extension(self, name: str, extension: object | dict) -> None:
        """
        Register a extension with this app.

        Parameters:
        name (str): The name of the extension
        extension (object | dict): The extension to register.
        """
        if isinstance(extension, dict):
            extension = types.SimpleNamespace(**extension)
        self.extensions[name] = extension
        self.logger.debug(f"Registered extension: {name}: {extension}")

    def get_extension(self, name: str) -> object:
        """
        Get an extension by name.

        Parameters:
        name (str): The name of the extension to get.

        Returns:
        object: The extension
        """
        if name not in self.extensions:
            raise ValueError(
                f"Extension not found: {name}, available extensions: {self.extensions}"
            )
        return self.extensions[name]

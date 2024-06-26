import typer
import types
import importlib
import sys
import os
from .log import get_logger
from .config import load_config
from .main_cli import cli

# Add CWD to path to allow import of local extensions
sys.path.append(os.getcwd())

_std_extensions = [
    "artisan_tools.vcs",
    "artisan_tools.version",
    "artisan_tools.parser",
    "artisan_tools.container",
]


class App:
    """
    Artisan Tools Application.
    """

    def __init__(self):
        """
        Artisan Tools Application.
        """
        self.cli = cli
        self.extensions = {}
        self.logger = get_logger("App")
        self.config = load_config()

    def load_extensions(self):
        """
        Load extensions (standard and config specified).
        """
        for extension in _std_extensions + self.config["extensions"]:
            self._load_extension(extension)

    def add_cli(self, cli: typer.Typer):
        """
        Add a (sub)cli to this app.

        Args:
        cli: The cli to add.
        """
        self.cli.add_typer(cli)
        self.logger.debug(f"Added sub-cli: {cli}")

    def register_extension(self, name: str, extension: object | dict) -> None:
        """
        Register a extension with this app.

        Args:
        name: The name of the extension
        extension: The extension to register.
        """
        if isinstance(extension, dict):
            extension = types.SimpleNamespace(**extension)
        self.extensions[name] = extension
        self.logger.debug(f"Registered extension: {name}: {extension}")

    def get_extension(self, name: str) -> object:
        """
        Get an extension by name.

        Args:
        name: The name of the extension to get.

        Returns:
        object: The extension
        """
        if name not in self.extensions:
            raise ValueError(
                f"Extension not found: {name}, available extensions: {self.extensions}"
            )
        return self.extensions[name]

    def _load_extension(self, extension: str):
        """
        Load an extension by name.

        Args:
        extension: The name of the extension to load.
        """
        ext = importlib.import_module(extension)
        ext.setup(self)

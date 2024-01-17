import importlib
from artisan_tools import log

logger = log.get_logger("modules")

_modules = {}


def _load_modules():
    """
    Load all modules.
    """
    from artisan_tools import get_config

    _config = get_config()
    for module_name in _config["modules"]:
        logger.debug(f"Loading module: '{module_name}'")
        module = importlib.import_module(module_name)
        _modules[module_name] = module


def get_module(name):
    """
    Get a module by name.
    """
    # Load modules if not already loaded:
    if not _modules:
        _load_modules()

    if name not in _modules:
        raise ValueError(
            f"No module named '{name}'. Available modules are: {list(_modules.keys())}"
        )

    return _modules[name]

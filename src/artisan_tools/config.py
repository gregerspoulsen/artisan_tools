import yaml
import copy
import os

from artisan_tools.log import get_logger

logger = get_logger("config")


def load_config(config_dir=None):
    """
    Loads configuration for artisan tools.

    The configuration is a merge of the local artisan.yaml file and the
    base_config.yaml file distributed with artisan tools.


    Returns:
    dict: The contents of the config.yaml file as a dictionary.
    """
    # Load base config:
    module_path = os.path.dirname(os.path.realpath(__file__))
    base_config = read_yaml(os.path.join(module_path, "base_config.yaml"))

    # Set local config file path:
    if config_dir is not None:
        local_config_file = os.path.join(config_dir, "artisan.yaml")
    else:
        local_config_file = os.path.join(os.getcwd(), "artisan.yaml")

    # Check local config exists:
    if not os.path.exists(local_config_file):
        raise FileNotFoundError(
            f"{local_config_file} not found. Please create this file in the "
            "root of your project."
        )

    # Load local config:
    logger.info(f"Loading local config from {local_config_file}")
    local_config = read_yaml(local_config_file)

    # Merge configs:
    config = recursive_merge(base_config, local_config)
    check_config(config)
    return config


def check_config(config: dict) -> None:
    """
    Check the configuration.
    """
    if "version" in config:
        if "file" in config["version"]:
            raise ValueError(
                "With artisan-tools 1.0.0 version.file is no longer a valid key. "
                "see CHANGELOG.md and documentation for details."
            )
    if "hooks" in config["version"]:
        raise ValueError(
            "With artisan-tools 1.0.0 hooks is no longer a valid key. "
            "see CHANGELOG.md and documentation for details."
        )


def read_yaml(file_path: str):
    """
    Read YAML file and return contents as a dictionary.

    If the file is empty an empty dictionary is returned.

    Args:
    file_path: The path to the YAML file to be read.

    Returns:
    dict: The contents of the YAML file as a dictionary.
    """
    with open(file_path, "r") as file:
        output = yaml.safe_load(file)
    if output is None:
        return {}
    else:
        return output


def recursive_merge(d1: dict, d2: dict):
    """
    Recursively merges two dictionaries.

    If a key exists in both, and both values are dictionaries, merge them. Otherwise,
    the value from the second dictionary overwrites the one in the first.

    Args:
    d1: The first dictionary to be merged.
    d2: The second dictionary, whose values will be merged into the first.

    Returns:
    dict: A new dictionary with the merged content of d1 and d2.
    """
    merged = copy.deepcopy(d1)
    for k, v in d2.items():
        if k in merged and isinstance(merged[k], dict) and isinstance(v, dict):
            merged[k] = recursive_merge(merged[k], v)
        else:
            merged[k] = v
    return merged


def write_yaml(data: dict, file_path: str):
    """
    Writes a Python dictionary to a YAML file.

    Args:
    data: The dictionary to write to the YAML file.
    file_path: The path to the file where the YAML content should be written.
    """
    with open(file_path, "w") as file:
        yaml.dump(data, file, default_flow_style=False)

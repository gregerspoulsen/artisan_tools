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
            (
                f"{local_config_file} not found. Please create this file in the "
                "root of your project."
            )
        )

    # Load local config:
    logger.info(f"Loading local config from {local_config_file}")
    local_config = read_yaml(local_config_file)

    # Merge configs:
    config = recursive_merge(base_config, local_config)
    return config


def read_yaml(file_path):
    """
    Reads a YAML file and returns its contents as a Python dictionary. If the
    file is empty an empty dictionary is returned.

    Parameters:
    file_path (str): The path to the YAML file to be read.

    Returns:
    dict: The contents of the YAML file as a dictionary.
    """
    with open(file_path, "r") as file:
        output = yaml.safe_load(file)
    if output is None:
        return {}
    else:
        return output


def recursive_merge(d1, d2):
    """
    Recursively merges two dictionaries. If a key exists in both,
    and both values are dictionaries, merge them. Otherwise,
    the value from the second dictionary overwrites the one in the first.

    Parameters:
    d1 (dict): The first dictionary to be merged.
    d2 (dict): The second dictionary, whose values will be merged into the first.

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


def write_yaml(data, file_path):
    """
    Writes a Python dictionary to a YAML file.

    Parameters:
    data (dict): The dictionary to write to the YAML file.
    file_path (str): The path to the file where the YAML content should be written.

    Returns:
    None
    """
    with open(file_path, "w") as file:
        yaml.dump(data, file, default_flow_style=False)


_config = load_config()


def get_config():
    """
    Get the configuration dictionary.
    """
    return copy.deepcopy(_config)

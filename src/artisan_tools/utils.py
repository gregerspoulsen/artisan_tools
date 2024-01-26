import shutil
import os


def check_executables(executable_names):
    """
    Generate list of executables that are available on the system.
    """
    return [name for name in executable_names if shutil.which(name) is not None]


def container_engines():
    """
    Generate list of container engines that are available on the system.
    """
    return check_executables(["docker", "podman"])

def get_env_var(variable_name, error_message):
    """
    Check if an environment variable is set and not empty.
    """
    value = os.getenv(variable_name)
    if value is None or value == "":
        raise ValueError(error_message)
    return value

def get_item(mapping, key, expected_content_message):
    """
    Get an entry from the configuration file.
    """
    if key not in mapping:
        raise ValueError(f"No entry '{key}', it should contain {expected_content_message}")
    value = mapping.get(key)
    if value is None:
        raise ValueError(f"Entry '{key}' is empty, it should contain {expected_content_message}")
    return value
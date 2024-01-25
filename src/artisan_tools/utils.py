import shutil

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
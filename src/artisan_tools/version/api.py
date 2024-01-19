
from artisan_tools.version.main import read_version_file

def get_version(app):
    """
    Retrieve version. If a file path is provided, the version will be read from
    that file. Otherwise, the version will be read from the configured version
    file.

    Returns:
    str: The version of the package.
    """
    file_path = app.config['version']['file']

    return read_version_file(file_path)
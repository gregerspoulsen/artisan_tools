import subprocess


def run_git_command(command):
    """
    Execute a git command and return its output as a string.

    Args:
    command (str): The git command to run.

    Returns:
    str: The output of the git command, or None if an error occurred.
    """

    result = subprocess.check_output(
        command, stderr=subprocess.STDOUT, shell=True, encoding="utf-8"
    )
    return result.strip()


def get_remote_tags():
    """
    Retrieve a list of tags from the remote git repository.

    Returns:
    list of str: A list of tags from the remote repository.
    """
    remote_tags_output = run_git_command("git ls-remote --tags")
    return [line.split("/")[-1] for line in remote_tags_output.split("\n") if line]


def check_tag(tag):
    """
    Check if a given tag exists in the remote git repository.

    Args:
    tag (str): The tag to check for in the remote repository.

    Returns:
    bool: True if the tag exists in the remote repository, False otherwise.
    """
    remote_tags = get_remote_tags()
    return tag in remote_tags

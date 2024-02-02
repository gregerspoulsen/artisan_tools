import subprocess


def run_git_command(command, cwd=None):
    """
    Execute a git command and return its output as a string.

    Args:
    command (str): The git command to run.
    cwd (str): The path to the directory in which to run the command. Optional.

    Returns:
    str: The output of the git command
    """
    options = "-c safe.directory=*"
    git_command = f"git {options} {command}"
    result = subprocess.check_output(
        git_command, stderr=subprocess.STDOUT, shell=True, encoding="utf-8", cwd=cwd
    )
    return result.strip()


def get_remote_tags():
    """
    Retrieve a list of tags from the remote git repository.

    Returns:
    list of str: A list of tags from the remote repository.
    """
    remote_tags_output = run_git_command("ls-remote --tags")
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


def get_current_branch():
    """
    Returns the name of the currently checked-out Git branch.
    """
    return run_git_command("branch --show-current")


def check_current_branch(expected_branch):
    """
    Check if the current Git branch is the expected branch.

    Args:
    expected_branch (str): The name of the branch to check against.

    Returns:
    bool: True if the current branch matches the expected branch, False otherwise.
    """
    current_branch = get_current_branch()
    return current_branch == expected_branch


def add_and_push_tag(config, tag_name, message, remote="origin"):
    """
    Add a tag to the current commit and push it to a remote repository.

    Args:
        config (dict): The configuration dict with the 'username' and 'email'.
        tag_name (str): The name of the tag to be added.
        message (str): The message associated with the tag.
        remote (str): The name of the remote repository. Defaults to 'origin'.

    """
    # Add the tag
    git_options = (
        f"-c user.name=\"{config['username']}\" -c user.email=\"{config['email']}\""
    )
    run_git_command(f"{git_options} tag -a {tag_name} -m '{message}'")

    # Push the tag to the remote repository
    run_git_command(f"push {remote} {tag_name}")

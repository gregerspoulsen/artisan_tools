import subprocess
import unittest
from unittest.mock import patch

from artisan_tools.vcs import (
    run_git_command,
    get_remote_tags,
    check_tag,
    check_current_branch,
)


class TestGitTagChecker(unittest.TestCase):
    def test_run_git_command_success(self):
        with patch("subprocess.check_output") as mock_check_output:
            mock_check_output.return_value = "v1.0.0\nv2.0.0"
            result = run_git_command("git tag")
            self.assertEqual(result, "v1.0.0\nv2.0.0")

    def test_run_git_command_failure(self):
        with patch(
            "subprocess.check_output",
            side_effect=subprocess.CalledProcessError(1, "cmd"),
        ):
            with self.assertRaises(subprocess.CalledProcessError):
                run_git_command("git tag")

    def test_get_remote_tags(self):
        with patch("subprocess.check_output") as mock_check_output:
            mock_check_output.return_value = "refs/tags/v1.0.0\nrefs/tags/v2.0.0"
            tags = get_remote_tags()
            self.assertIn("v1.0.0", tags)
            self.assertIn("v2.0.0", tags)

    def test_check_tag_exists(self):
        with patch("subprocess.check_output") as mock_check_output:
            mock_check_output.return_value = "refs/tags/v1.0.0\nrefs/tags/v2.0.0"
            self.assertTrue(check_tag("v1.0.0"))

    def test_check_tag_does_not_exist(self):
        with patch("subprocess.check_output") as mock_check_output:
            mock_check_output.return_value = "refs/tags/v1.0.0\nrefs/tags/v2.0.0"
            self.assertFalse(check_tag("v3.0.0"))

    @patch("artisan_tools.vcs.run_git_command")
    def test_check_current_branch_true(self, mock_run_git_command):
        mock_run_git_command.return_value = "main"
        self.assertTrue(check_current_branch("main"))

    @patch("artisan_tools.vcs.run_git_command")
    def test_check_current_branch_false(self, mock_run_git_command):
        mock_run_git_command.return_value = "develop"
        self.assertFalse(check_current_branch("main"))

    @patch("artisan_tools.vcs.run_git_command")
    def test_check_current_branch_error(self, mock_run_git_command):
        mock_run_git_command.side_effect = subprocess.CalledProcessError(1, "cmd")
        with self.assertRaises(subprocess.CalledProcessError):
            check_current_branch("any-branch")
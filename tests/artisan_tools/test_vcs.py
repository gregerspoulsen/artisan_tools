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

    def test_get_remote_tags(self):
        tags = get_remote_tags()
        self.assertIn('v0.1.0', tags)

    def test_check_tag_exists(self):
        self.assertTrue(check_tag('v0.1.0'))

    def test_check_tag_does_not_exist(self):
        self.assertFalse(check_tag('nonexistent-tag'))

    @patch('artisan_tools.vcs.run_git_command')
    def test_check_current_branch_true(self, mock_run_git_command):
        mock_run_git_command.return_value = 'main'
        self.assertTrue(check_current_branch('main'))

    @patch('artisan_tools.vcs.run_git_command')
    def test_check_current_branch_false(self, mock_run_git_command):
        mock_run_git_command.return_value = 'develop'
        self.assertFalse(check_current_branch('main'))

    @patch('artisan_tools.vcs.run_git_command')
    def test_check_current_branch_error(self, mock_run_git_command):
        mock_run_git_command.side_effect = subprocess.CalledProcessError(1, 'cmd')
        with self.assertRaises(subprocess.CalledProcessError):
            check_current_branch('any-branch')
# FILEPATH: /app/tests/api/test_api.py

import pytest
import subprocess

from artisan_tools.container.api import login, logout, push, run_command_with_auth


def test_login_direct(app_with_config, registry):
    # Arrange
    app = app_with_config
    app.config["container"]["auth"] = {
        "method": "direct",
        "user": "mock_username",
        "token": "mock_token",
    }

    result = login(app)
    assert result
    logout(app)


def test_login_env(app_with_config, monkeypatch):
    # Arrange
    app = app_with_config
    app.config["container"]["auth"] = {
        "method": "env",
        "user_var": "USER_VAR",
        "token_var": "TOKEN_VAR",
    }

    monkeypatch.setenv("USER_VAR", "mock_username")
    monkeypatch.setenv("TOKEN_VAR", "mock_token")

    result = login(app)
    assert result
    logout(app)


def test_login_env_missing_env_vars(app_with_config):
    # If already logged in it should be allowed
    app = app_with_config

    app.config["container"]["auth"] = {
        "method": "direct",
        "user": "mock_username",
        "token": "mock_token",
    }

    login(app)

    app.config["container"]["auth"] = {
        "method": "env",
        "user_var": "USER_VAR",
        "token_var": "TOKEN_VAR",
    }

    result = login(app)
    assert not result
    logout(app)


def test_push(app_with_config, registry):
    src = "alpine"
    target = "localhost:5000/alpine"
    tags = ["test", "test2"]

    # Pull image with podman:
    subprocess.run(["podman", "pull", src, "--tls-verify=False"], check=True)

    push(app_with_config, src, target, tags)

    # Check if image is present in registry
    for tag in tags:
        subprocess.run(
            ["podman", "pull", target + ":" + tag, "--tls-verify=False"], check=True
        )


def test_push_tag_in_target(app_with_config):
    src = "alpine"
    target = "localhost:5000/alpine:test"
    tags = ["test", "latest"]

    with pytest.raises(ValueError):
        push(app_with_config, src, target, tags)


def test_run_command_with_auth(app_with_config, registry):
    app = app_with_config
    command = "ls"

    # Act
    run_command_with_auth(app, command)

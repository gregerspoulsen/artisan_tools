import pytest
from artisan_tools.utils import get_env_var, get_item
import os

def test_get_env_var_success(monkeypatch):
    monkeypatch.setenv("TEST_VARIABLE", "test_value")
    result = get_env_var("TEST_VARIABLE", "Error message")
    assert result == "test_value"

def test_get_env_var_fail_no_value(monkeypatch):
    if "TEST_VARIABLE" in os.environ:
        monkeypatch.delenv("TEST_VARIABLE")
    with pytest.raises(ValueError, match="Error message"):
        get_env_var("TEST_VARIABLE", "Error message")

def test_get_env_var_fail_empty_value(monkeypatch):
    monkeypatch.setenv("TEST_VARIABLE", "")
    with pytest.raises(ValueError, match="Error message"):
        get_env_var("TEST_VARIABLE", "Error message")


def test_get_item_success():
    mapping = {"key": "value"}
    key = "key"
    expected_content_message = "a value"
    result = get_item(mapping, key, expected_content_message)
    assert result == "value"

def test_get_item_fail_no_key():
    mapping = {}
    key = "key"
    expected_content_message = "a value"

    with pytest.raises(ValueError, match=f"No entry '{key}', it should contain {expected_content_message}"):
        get_item(mapping, key, expected_content_message)

def test_get_item_fail_empty_value():
    mapping = {"key": None}
    key = "key"
    expected_content_message = "a value"
    with pytest.raises(ValueError, match=f"Entry '{key}' is empty, it should contain {expected_content_message}"):
        get_item(mapping, key, expected_content_message)
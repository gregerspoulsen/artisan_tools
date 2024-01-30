import pytest
import yaml

from artisan_tools.config import recursive_merge, load_config, read_yaml


def test_merge_with_unique_keys():
    dict1 = {"a": 1, "b": 2}
    dict2 = {"c": 3, "d": 4}
    expected = {"a": 1, "b": 2, "c": 3, "d": 4}
    assert recursive_merge(dict1, dict2) == expected


def test_merge_with_overlapping_keys():
    dict1 = {"a": 1, "b": 2}
    dict2 = {"b": 3, "c": 4}
    expected = {"a": 1, "b": 3, "c": 4}
    assert recursive_merge(dict1, dict2) == expected


def test_merge_with_nested_dictionaries():
    dict1 = {"a": 1, "b": {"x": 1}}
    dict2 = {"b": {"y": 2}, "c": 3}
    expected = {"a": 1, "b": {"x": 1, "y": 2}, "c": 3}
    assert recursive_merge(dict1, dict2) == expected


def test_merge_with_deeply_nested_dictionaries():
    dict1 = {"a": {"b": {"c": 1}}}
    dict2 = {"a": {"b": {"d": 2}}}
    expected = {"a": {"b": {"c": 1, "d": 2}}}
    assert recursive_merge(dict1, dict2) == expected


def test_merge_with_empty_dictionary():
    dict1 = {"a": 1, "b": 2}
    dict2 = {}
    expected = {"a": 1, "b": 2}
    assert recursive_merge(dict1, dict2) == expected


def test_merge_into_empty_dictionary():
    dict1 = {}
    dict2 = {"a": 1, "b": 2}
    expected = {"a": 1, "b": 2}
    assert recursive_merge(dict1, dict2) == expected


# Helper function to create temporary YAML files for testing
def create_temp_yaml(file_path, content):
    with open(file_path, "w") as file:
        yaml.dump(content, file)


def test_load_config_merged_correctly(tmp_path):
    """
    Test that the local and base configs are merged correctly.
    """
    # Create local config file
    local_config_content = {"setting2": "local", "setting3": "local"}
    local_config_file = tmp_path / "artisan.yaml"
    create_temp_yaml(local_config_file, local_config_content)

    config = load_config(config_dir=tmp_path)

    assert config["version"] == {"file": "./VERSION", "hooks": []}
    assert config["setting2"] == "local"
    assert config["setting3"] == "local"


def test_load_config_raises_error_when_local_config_missing(tmp_path, monkeypatch):
    """
    Test that FileNotFoundError is raised when the local config is missing.
    """
    # Create base config file only
    base_config_content = {"setting1": "base", "setting2": "base"}
    base_config_file = tmp_path / "base_config.yaml"
    create_temp_yaml(base_config_file, base_config_content)

    # Set up the environment to use the temporary path
    monkeypatch.chdir(tmp_path)

    # Run the test
    with pytest.raises(FileNotFoundError):
        load_config()


def test_read_yaml_with_valid_content(tmpdir):
    """
    Test read_yaml function with valid YAML content.
    """
    # Create a temporary YAML file with content
    sample_content = {"key1": "value1", "key2": "value2"}
    temp_yaml = tmpdir.join("sample.yaml")
    create_temp_yaml(temp_yaml, sample_content)

    # Read the YAML file
    result = read_yaml(str(temp_yaml))
    assert result == sample_content


def test_read_yaml_with_empty_file(tmpdir):
    """
    Test read_yaml function with an empty YAML file.
    """
    # Create an empty temporary YAML file
    empty_yaml = tmpdir.join("empty.yaml")
    with open(empty_yaml, "w"):
        pass

    # Read the YAML file
    result = read_yaml(str(empty_yaml))
    assert result == {}

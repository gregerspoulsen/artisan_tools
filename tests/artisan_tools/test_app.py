import pytest
from artisan_tools.app import App
import typer
import types


# Test the add_cli method
def test_add_cli():
    app = App()
    cli = typer.Typer()
    app.add_cli(cli)


# Test the register_extension method
@pytest.mark.parametrize("extension", [types.SimpleNamespace(**{"a": 1}), {"a": 1}])
def test_register_extension(extension):
    app = App()
    app.register_extension("test_ext", extension)
    ext = app.get_extension("test_ext")
    assert ext.a == 1


def test_get_extension_not_found():
    app = App()
    with pytest.raises(ValueError) as excinfo:
        app.get_extension("non_existent")
    assert "Extension not found" in str(excinfo.value)


def test_load_extension_module():
    app = App()
    app._load_extension("artisan_tools.version")

    # Test API call:
    app.get_extension("version").get_version(app)


def test_load_extension_local(tmp_path):
    app = App()
    app._load_extension("fake_extension")

    # Test API call:
    out = app.get_extension("test_ext").test_function("x")
    assert out == (("x",), {})

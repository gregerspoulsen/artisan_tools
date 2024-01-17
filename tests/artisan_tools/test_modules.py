from artisan_tools import get_module
import artisan_tools.version


def test_get_module():
    vm = get_module("artisan_tools.version")
    assert vm == artisan_tools.version

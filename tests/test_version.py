from importlib.metadata import version


def test_version():
    assert version("textconf").count(".") == 2

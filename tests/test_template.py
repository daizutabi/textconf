from pathlib import Path
from tempfile import NamedTemporaryFile, TemporaryDirectory

import pytest
from pytest import MonkeyPatch


class Class:
    pass


def test_existing_file(tmp_path: Path):
    from textconf.template import get_template_file

    path = tmp_path / "template.jinja"
    path.touch()

    assert get_template_file(Class, path) == path


def test_existing_file_in_dir(tmp_path: Path, monkeypatch: MonkeyPatch):
    from textconf.template import get_template_file

    path = tmp_path / "dir" / "template.jinja"
    path.parent.mkdir()
    path.touch()

    monkeypatch.chdir(tmp_path)
    assert get_template_file(Class, "template.jinja", dir="dir") == path


def test_get_from_class(tmp_path: Path, monkeypatch: MonkeyPatch):
    from textconf.template import get_template_file

    with NamedTemporaryFile(dir=Path(__file__).parent) as temp_file:
        path = Path(temp_file.name)
        monkeypatch.chdir(tmp_path)
        assert get_template_file(Class, path.name) == path


def test_get_from_class_dir(tmp_path: Path, monkeypatch: MonkeyPatch):
    from textconf.template import get_template_file

    with TemporaryDirectory(dir=Path(__file__).parent) as tmp_dir:
        path = Path(tmp_dir).joinpath("template.jinja")
        path.touch()

        monkeypatch.chdir(tmp_path)
        file = get_template_file(Class, "template.jinja", dir=Path(tmp_dir).name)
        assert file == path


def test_get_from_class_parent_dir(tmp_path: Path, monkeypatch: MonkeyPatch):
    from textconf.template import get_template_file

    with TemporaryDirectory(dir=Path(__file__).parent.parent) as tmp_dir:
        path = Path(tmp_dir).joinpath("template.jinja")
        path.touch()

        monkeypatch.chdir(tmp_path)
        file = get_template_file(Class, "template.jinja", dir=Path(tmp_dir).name)
        assert file == path


def test_current_dir_not_found(tmp_path: Path):
    from textconf.template import get_template_file

    with pytest.raises(FileNotFoundError):
        get_template_file(Class, "template.jinja")

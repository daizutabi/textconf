from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING

import pytest

from textconf.config import BaseConfig
from textconf.testing import assert_render_in

if TYPE_CHECKING:
    from pathlib import Path

    from jinja2 import Environment
    from pytest import MonkeyPatch


@pytest.fixture
def template_file(tmp_path: Path):
    path = tmp_path / "template.jinja"
    text = "A{{a|myfilter}}|B{{myfunc(b)}}|C{{a+b|myfilter(2)}}|D{{myfunc(a*b,3)}}|"
    path.write_text(text)
    return path


@pytest.fixture(autouse=True)
def _setup(monkeypatch: MonkeyPatch, template_file: Path):
    monkeypatch.chdir(template_file.parent)
    yield


def myfilter(x: float, a: int = 1) -> float:
    return x + a


def myfunc(x: float, a: int = 2) -> float:
    return x * a


@dataclass
class Config(BaseConfig):
    a: float = 0
    b: float = 0

    @classmethod
    def set_environment(cls, env: Environment) -> None:
        env.filters["myfilter"] = myfilter
        env.globals["myfunc"] = myfunc


def test_render_filter():
    cfg = Config("template.jinja", a=10)
    assert_render_in(cfg, "A11.0|")


def test_render_filter_arg():
    cfg = Config("template.jinja", a=20, b=10)
    assert_render_in(cfg, "C32.0|")


def test_render_func():
    cfg = Config("template.jinja", b=3)
    assert_render_in(cfg, "B6.0|")


def test_render_func_arg():
    cfg = Config("template.jinja", a=5, b=3)
    assert_render_in(cfg, "D45.0|")

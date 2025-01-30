from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING

import pytest

from textconf.config import BaseConfig
from textconf.testing import (
    assert_render_endswith,
    assert_render_eq,
    assert_render_in,
    assert_render_startswith,
)

if TYPE_CHECKING:
    from pathlib import Path
    from typing import Self

    from pytest import MonkeyPatch


@dataclass
class Config(BaseConfig):
    _template_: str = "template.jinja"
    x: int = 0
    y: int = 0

    @classmethod
    def update(cls, cfg: Self) -> None:
        cfg.y = 2 * cfg.x

    @classmethod
    def z(cls, cfg: Self) -> str:
        return "z" * cfg.x


TEMPLATE_FILE = "X{{x}}|Y{{y}}|Z{{z}}"


@pytest.fixture
def template_file(tmp_path: Path):
    path = tmp_path / "template.jinja"
    path.write_text(TEMPLATE_FILE)
    return path


@pytest.fixture(autouse=True)
def _setup(monkeypatch: MonkeyPatch, template_file: Path):
    monkeypatch.chdir(template_file.parent)
    yield


def test_render_eq():
    cfg = Config()
    assert_render_eq(cfg, "X0|Y0|Z")


def test_render_in():
    cfg = Config(x=2)
    Config.update(cfg)
    assert_render_in(cfg, "|Y4|")


def test_render_startswqith():
    cfg = Config(x=3)
    assert_render_startswith(cfg, "X3|")


def test_render_endswith():
    cfg = Config(x=5)
    assert_render_endswith(cfg, "|Zzzzzz")


def test_render_eq_raises():
    cfg = Config()
    with pytest.raises(AssertionError):
        assert_render_eq(cfg, "X1|Y2|Zz")


def test_render_in_raises():
    cfg = Config(x=2)
    with pytest.raises(AssertionError):
        assert_render_in(cfg, "|Y3|")


def test_render_startswqith_raises():
    cfg = Config(x=3)
    with pytest.raises(AssertionError):
        assert_render_startswith(cfg, "X2|")


def test_render_endswith_raises():
    cfg = Config(x=5)
    with pytest.raises(AssertionError):
        assert_render_endswith(cfg, "|Zzz")


@dataclass
class Context(BaseConfig):
    _template_: str = "template.jinja"
    x: int = 0

    @classmethod
    def context(cls, cfg: Self) -> dict[str, int | str]:
        y = cls.y(cfg)
        return {"y": y, "z": cls.z(cfg, y)}

    @classmethod
    def y(cls, cfg: Self) -> int:
        return 2 * cfg.x

    @classmethod
    def z(cls, cfg: Self, y: int) -> str:
        return "z" * (cfg.x + y)


def test_context():
    cfg = Context(x=2)
    assert_render_eq(cfg, "X2|Y4|Zzzzzzz")


def test_context_kwargs():
    cfg = Context(x=2)
    assert_render_eq(cfg, "X2|Y3|Zzzzzzz", y=3)
    assert_render_eq(cfg, "X2|Y3|ZA", y=3, z="A")


def test_context_kwargs_none():
    cfg = Context(x=2)
    assert_render_eq(cfg, "X2|Y4|Zzzzzzz", z=None)

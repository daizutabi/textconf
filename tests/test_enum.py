from __future__ import annotations

from dataclasses import dataclass
from enum import auto
from typing import TYPE_CHECKING

import pytest
from omegaconf import OmegaConf

from textconf.config import BaseConfig
from textconf.enum import RenderableEnum

if TYPE_CHECKING:
    from pathlib import Path
    from typing import Self

    from pytest import MonkeyPatch


class Model(RenderableEnum):
    MISSING = auto()
    A = auto()
    B = auto()


@dataclass
class Config(BaseConfig):
    model: Model = Model.MISSING
    y: int = 1


@dataclass
class A(Config):
    model: Model = Model.A

    @classmethod
    def x(cls, cfg: Self) -> str:
        return "1"


@dataclass
class B(Config):
    model: Model = Model.B

    @classmethod
    def x(cls, cfg: Self) -> str:
        return "2"


TEMPLATE_FILE = "{{x}}|{{y}}"


@pytest.fixture
def template_file(tmp_path: Path):
    path = tmp_path / "template.jinja"
    path.write_text(TEMPLATE_FILE)
    return path


@pytest.fixture(autouse=True)
def _setup(monkeypatch: MonkeyPatch, template_file: Path):
    monkeypatch.chdir(template_file.parent)
    yield


@pytest.mark.parametrize(("model", "x"), [(Model.A, "1|1"), (Model.B, "2|1")])
def test_render(model: Model, x: str):
    cfg = Config("template.jinja", model=model)
    assert cfg.model.render(cfg) == x


@pytest.mark.parametrize(("model", "x"), [(Model.A, "1|1"), (Model.B, "2|1")])
def test_render_structured(model: Model, x: str):
    cfg = Config("template.jinja", model=model)
    cfg = OmegaConf.structured(cfg)
    assert cfg.model.render(cfg) == x


@pytest.mark.parametrize(("model", "x"), [(Model.A, "1|10"), (Model.B, "2|10")])
def test_render_args(model: Model, x: str):
    cfg = Config("template.jinja", model=model)
    assert cfg.model.render(cfg, {"y": 10}) == x


@pytest.mark.parametrize(("model", "x"), [(Model.A, "1|100"), (Model.B, "2|100")])
def test_render_kwargs(model: Model, x: str):
    cfg = Config("template.jinja", model=model)
    assert cfg.model.render(cfg, y=100) == x


def test_missing():
    cfg = Config("template.jinja")
    with pytest.raises(ValueError):
        cfg.model.render(cfg)

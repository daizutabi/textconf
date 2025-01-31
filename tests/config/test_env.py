from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING

from textconf.config import BaseConfig
from textconf.testing import assert_render_in

if TYPE_CHECKING:
    from typing import Self

    from jinja2 import Environment


def myfilter(x: float, a: int = 1) -> float:
    return x + a


def myfunc(x: float, a: int = 2) -> float:
    return x * a


TEMPLATE = "A{{a|myfilter}}|B{{myfunc(b)}}|C{{a+b|myfilter(2)}}|D{{myfunc(a*b,3)}}|"


@dataclass
class Config(BaseConfig):
    _template_: str = TEMPLATE
    a: float = 0
    b: float = 0

    @classmethod
    def set_environment(cls, env: Environment) -> None:
        env.filters["myfilter"] = myfilter
        env.globals["myfunc"] = myfunc

    @classmethod
    def render(cls, cfg: Self, **kwargs) -> str:
        if "b" in kwargs:
            kwargs["b"] *= 100

        return super().render(cfg, **kwargs)


def test_filter():
    cfg = Config(a=10)
    assert_render_in(cfg, "A11.0|")


def test_filter_arg():
    cfg = Config(a=20, b=10)
    assert_render_in(cfg, "C32.0|")


def test_func():
    cfg = Config(b=3)
    assert_render_in(cfg, "B6.0|")


def test_func_arg():
    cfg = Config(a=5, b=3)
    assert_render_in(cfg, "D45.0|")


def test_render_kwargs():
    cfg = Config()
    assert_render_in(cfg, "A21|", a=20)


def test_render_kwargs_custom():
    cfg = Config(a=10, b=10)
    assert_render_in(cfg, "B4000|", a=20, b=20)

from dataclasses import dataclass, field
from pathlib import Path

import pytest

from textconf.render import render


@pytest.fixture
def template_file(tmp_path: Path):
    path = tmp_path / "template.jinja"
    path.write_text("I{{i}}|F{{f}}|S{{s}}|L{{l}}")
    return path


@pytest.fixture
def template(template_file: Path):
    from textconf.template import get_environment

    env = get_environment(template_file)
    return env.get_template(template_file.name)


def test_render_kwargs(template):
    assert render(template, i=1, f=2.0, s="3", l=[4, 5, 6]) == "I1|F2.0|S3|L[4, 5, 6]"


def test_render_cfg(template):
    s = render(template, {"i": 1, "f": 2.0, "s": "3", "l": [4, 5, 6]})
    assert s == "I1|F2.0|S3|L[4, 5, 6]"


def test_render_arg(template):
    s = render(template, {}, {"i": 1, "f": 2.0, "s": "3", "l": [4, 5, 6]})
    assert s == "I1|F2.0|S3|L[4, 5, 6]"


def test_render_arg_merge(template):
    s = render(template, {"i": 10}, {"i": 1, "f": 2.0, "s": "3", "l": [4, 5, 6]})
    assert s == "I1|F2.0|S3|L[4, 5, 6]"


@pytest.fixture
def template_file_dot(tmp_path: Path):
    path = tmp_path / "template.jinja"
    path.write_text("D{{d.x}}")
    return path


@pytest.fixture
def template_dot(template_file_dot):
    from textconf.template import get_environment

    env = get_environment(template_file_dot)
    return env.get_template(template_file_dot.name)


def test_render_kwargs_dot(template_dot):
    assert render(template_dot, d={"x": 1}) == "D1"


def test_render_cfg_dot(template_dot):
    assert render(template_dot, {"d": {"x": 2}}) == "D2"


def test_render_arg_dot(template_dot):
    assert render(template_dot, {"d": {"x": 2}}, {"d": {"x": 3}}) == "D3"


def test_render_dot_list(template_dot):
    assert render(template_dot, {}, ["d.x=4"]) == "D4"


def test_render_dot_dict(template_dot):
    assert render(template_dot, {}, {"d.x": 5}) == "D5"


@dataclass
class Config:
    i: int = 10
    f: float = 20.0
    s: str = "str"
    l: list[int] = field(default_factory=lambda: [10, 11, 12])  # noqa: E741


def test_render_dataclass(template):
    assert render(template, Config()) == "I10|F20.0|Sstr|L[10, 11, 12]"


def test_render_mixed(template):
    s = render(template, Config(i=20, l=[1, 2, 3]), f=1e3, s="STR")
    assert s == "I20|F1000.0|SSTR|L[1, 2, 3]"


def test_render_missing(template):
    assert render(template) == "I|F|S|L"


@dataclass
class X:
    x: int = 10


@dataclass
class Dot:
    d: X = field(default_factory=X)


def test_render_dataclass_dot(template_dot):
    s = render(template_dot, Dot())
    assert s == "D10"


def test_render_mixed_dot_list(template_dot):
    assert render(template_dot, Dot(d=X(x=20)), ["d.x=30"]) == "D30"


def test_render_mixed_dot_dict(template_dot):
    assert render(template_dot, Dot(d=X(x=20)), {"d.x": 40}) == "D40"

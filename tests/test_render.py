from dataclasses import dataclass, field
from pathlib import Path

import pytest
from jinja2 import Template

TEMPLATE_FILE = "I{{i}}|F{{f}}|S{{s}}|L{{l}}"


@pytest.fixture
def template_file(tmp_path: Path):
    path = tmp_path / "template.jinja"
    path.write_text(TEMPLATE_FILE)
    return path


def test_get_template(template_file):
    from textconf.render import get_template

    tmpl = get_template(template_file)
    assert isinstance(tmpl, Template)


def test_render_kwargs(template_file):
    from textconf.render import render

    s = render(template_file, i=1, f=2.0, s="3", l=[4, 5, 6])
    assert s == "I1|F2.0|S3|L[4, 5, 6]"


def test_render_cfg(template_file):
    from textconf.render import render

    s = render(template_file, {"i": 1, "f": 2.0, "s": "3", "l": [4, 5, 6]})
    assert s == "I1|F2.0|S3|L[4, 5, 6]"


def test_render_arg(template_file):
    from textconf.render import render

    s = render(template_file, {}, {"i": 1, "f": 2.0, "s": "3", "l": [4, 5, 6]})
    assert s == "I1|F2.0|S3|L[4, 5, 6]"


def test_render_arg_merge(template_file):
    from textconf.render import render

    s = render(template_file, {"i": 10}, {"i": 1, "f": 2.0, "s": "3", "l": [4, 5, 6]})
    assert s == "I1|F2.0|S3|L[4, 5, 6]"


TEMPLATE_FILE_DOT = "D{{d.x}}"


@pytest.fixture
def template_file_dot(tmp_path: Path):
    path = tmp_path / "template.jinja"
    path.write_text(TEMPLATE_FILE_DOT)
    return path


def test_get_template_dot(template_file_dot):
    from textconf.render import get_template

    tmpl = get_template(template_file_dot)
    assert isinstance(tmpl, Template)


def test_render_kwargs_dot(template_file_dot):
    from textconf.render import render

    s = render(template_file_dot, d={"x": 1})
    assert s == "D1"


def test_render_cfg_dot(template_file_dot):
    from textconf.render import render

    s = render(template_file_dot, {"d": {"x": 2}})
    assert s == "D2"


def test_render_arg_dot(template_file_dot):
    from textconf.render import render

    s = render(template_file_dot, {"d": {"x": 2}}, {"d": {"x": 3}})
    assert s == "D3"


def test_render_dot_list(template_file_dot):
    from textconf.render import render

    s = render(template_file_dot, {}, ["d.x=4"])
    assert s == "D4"


def test_render_dot_dict(template_file_dot):
    from textconf.render import render

    s = render(template_file_dot, {}, {"d.x": 5})
    assert s == "D5"


@dataclass
class Config:
    i: int = 10
    f: float = 20.0
    s: str = "str"
    l: list[int] = field(default_factory=lambda: [10, 11, 12])  # noqa: E741


def test_render_dataclass(template_file):
    from textconf.render import render

    s = render(template_file, Config())
    assert s == "I10|F20.0|Sstr|L[10, 11, 12]"


def test_render_mixed(template_file):
    from textconf.render import render

    s = render(template_file, Config(i=20, l=[1, 2, 3]), f=1e3, s="STR")
    assert s == "I20|F1000.0|SSTR|L[1, 2, 3]"


def test_render_missing(template_file):
    from textconf.render import render

    s = render(template_file)
    assert s == "I|F|S|L"


@dataclass
class X:
    x: int = 10


@dataclass
class Dot:
    d: X = field(default_factory=X)


def test_render_dataclass_dot(template_file_dot):
    from textconf.render import render

    s = render(template_file_dot, Dot())
    assert s == "D10"


def test_render_mixed_dot_list(template_file_dot):
    from textconf.render import render

    s = render(template_file_dot, Dot(d=X(x=20)), ["d.x=30"])
    assert s == "D30"


def test_render_mixed_dot_dict(template_file_dot):
    from textconf.render import render

    s = render(template_file_dot, Dot(d=X(x=20)), {"d.x": 40})
    assert s == "D40"

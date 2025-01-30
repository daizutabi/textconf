import math
from pathlib import Path

import pytest
from jinja2 import Environment


@pytest.fixture
def write_template(tmp_path: Path):
    def write_template(text: str) -> Path:
        path = tmp_path / "template.jinja"
        path.write_text(text)
        return path

    return write_template


def test_write_template(write_template):
    s = write_template("{{x}}")
    assert s.read_text() == "{{x}}"


@pytest.mark.parametrize(
    ("x", "expected"),
    [
        (1.2345, "1.2345"),
        (1.234e-9, "1.234e-09"),
        (0.1 + 0.2, "0.30000000000000004"),
        (4e-8 * 3, "1.2000000000000002e-07"),
    ],
)
def test_without_format(write_template, x, expected):
    from textconf.render import render

    template_file = write_template("{{x}}")
    assert render(template_file, x=x) == expected


@pytest.mark.parametrize(
    ("x", "expected"),
    [(1.2345, "1.234"), (1.23456, "1.235"), (1.234e-3, "0.001")],
)
def test_decimal_place(write_template, x, expected):
    from textconf.render import render

    template_file = write_template('{{ "{:.3f}".format(x) }}')
    assert render(template_file, x=x) == expected


@pytest.mark.parametrize(
    ("x", "expected"),
    [
        (1.2345, "1.23"),
        (1.234e-3, "0.00123"),
        (5.6789e-9, "5.68e-09"),
        (0.1 + 0.2, "0.3"),
        (4e-8 * 3, "1.2e-07"),
    ],
)
def test_significant_figures(write_template, x, expected):
    from textconf.render import render

    template_file = write_template('{{ "{:.3g}".format(x) }}')
    assert render(template_file, x=x) == expected


@pytest.mark.parametrize(
    ("x", "y", "expected"),
    [
        (4e-8, 3, "1.2e-07"),
        (4e-8, 1 / 3, "1.33e-08"),
    ],
)
def test_significant_figures_calc(write_template, x, y, expected):
    from textconf.render import render

    template_file = write_template('{{"{:.3g}".format(x*y)}}')
    assert render(template_file, x=x, y=y) == expected


def significant_figures(value: float, ndigits: int) -> str:
    if value == 0:
        return "0"

    return f"{value:.{ndigits}g}"


def zero_if_small(value: float, threshold: float = 1e-12) -> float:
    return 0 if abs(value) < threshold else value


@pytest.fixture(scope="module")
def env():
    env = Environment()
    env.filters["sformat"] = significant_figures
    env.filters["zero_if_small"] = zero_if_small
    env.globals["sin"] = math.sin
    return env


@pytest.mark.parametrize(
    ("x", "ndigits", "expected"),
    [
        (1.2345, 2, "1.2"),
        (1.234e-3, 3, "0.00123"),
        (5.6789e-9, 2, "5.7e-09"),
        (0.1 + 0.2, 7, "0.3"),
        (4e-8 * 3, 7, "1.2e-07"),
    ],
)
def test_filter(env: Environment, x, ndigits, expected):
    from textconf.render import render

    template = env.from_string("{{ x|sformat(" + str(ndigits) + ") }}")
    assert render(template, x=x) == expected


@pytest.mark.parametrize(
    ("x", "zero", "expected"),
    [
        (0, False, "0"),
        (math.pi, False, "1.22e-16"),
        (math.pi, True, "0"),
        (math.pi / 2, False, "1"),
    ],
)
def test_func(env: Environment, x, zero, expected):
    from textconf.render import render

    if zero:
        text = "{{ sin(x)|zero_if_small|sformat(3) }}"
    else:
        text = "{{ sin(x)|sformat(3) }}"

    template = env.from_string(text)
    assert render(template, x=x) == expected

from pathlib import Path

import pytest
from jinja2 import Environment


@pytest.fixture(scope="module")
def env():
    return Environment()


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


# def significant_figures(value: float, ndigits: int) -> str:
#     if value == 0:
#         return "0"

#     return f"{value:.{ndigits}g}"


# @pytest.mark.parametrize(
#     ("x", "ndigits", "expected"),
#     [
#         (1.2345, 3, "1.23"),
#         (1.234e-3, 3, "0.00123"),
#         (5.6789e-9, 3, "5.68e-09"),
#         (0.1 + 0.2, 3, "0.3"),
#         (4e-8 * 3, 3, "1.2e-07"),
#     ],
# )
# def test_filter(write_template, env: Environment, x, ndigits, expected):
#     from textconf.render import render

#     env.filters["sformat"] = significant_figures

#     env.get_template("{{ x|sformat(" + str(ndigits) + ") }}")

#     template_file = write_template("{{ x|sformat(" + str(ndigits) + ") }}")
#     assert render(template_file, x=x) == expected

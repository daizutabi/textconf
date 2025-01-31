import pytest


@pytest.fixture(scope="module")
def single_run(collect):
    outputs = collect("app.py", [])
    return outputs[0]


@pytest.mark.parametrize(
    ("key", "value"),
    [
        ("x", "0.1"),
        ("y", "0.2"),
        ("z", "0.30000000000000004"),
        ("zf", "0.3"),
        ("twice", "1.2000000000000002"),
        ("point", "(0.1, 0.2)"),
        ("distance", "0.224"),
    ],
)
def test_single_run(single_run, key, value):
    assert single_run[key] == value


@pytest.fixture(scope="module")
def multirun(collect):
    return collect("app.py", ["-m", "x=1e-5,5e-5", "y=2e-5"])


@pytest.fixture(scope="module")
def multirun1(multirun):
    if multirun[0]["x"] == "1e-05":
        return multirun[0]

    return multirun[1]


@pytest.fixture(scope="module")
def multirun2(multirun):
    if multirun[0]["x"] == "1e-05":
        return multirun[1]

    return multirun[0]


@pytest.mark.parametrize(
    ("key", "value"),
    [
        ("x", "1e-05"),
        ("y", "2e-05"),
        ("z", "3.0000000000000004e-05"),
        ("zf", "3e-05"),
        ("twice", "0.00012000000000000002"),
        ("point", "(1e-05, 2e-05)"),
        ("distance", "2.24e-05"),
    ],
)
def test_multirun1(multirun1, key, value):
    assert multirun1[key] == value


@pytest.mark.parametrize(
    ("key", "value"),
    [
        ("x", "5e-05"),
        ("y", "2e-05"),
        ("z", "7.000000000000001e-05"),
        ("zf", "7e-05"),
        ("twice", "0.00028000000000000003"),
        ("point", "(5e-05, 2e-05)"),
        ("distance", "5.39e-05"),
    ],
)
def test_multirun2(multirun2, key, value):
    assert multirun2[key] == value

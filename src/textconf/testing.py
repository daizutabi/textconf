"""Test the rendering process of configuration objects and their results.

This module provides test utilities to assert that the rendering
process of a configuration object produces the expected output.
It is designed to be used in test suites to verify that the configuration
and rendering mechanisms are functioning correctly.
"""

from __future__ import annotations

from typing import TYPE_CHECKING

from omegaconf import OmegaConf

if TYPE_CHECKING:
    from collections.abc import Callable

    from textconf.config import Renderable


def _assert_render(
    cfg: Renderable,
    expected: str | list[str],
    callback: Callable[[str, str], None],
    *args,
    **kwargs,
) -> None:
    if isinstance(expected, str):
        expected = [expected]

    dc = OmegaConf.structured(cfg)
    cls = type(cfg)

    for config in [cfg, dc]:
        text = cls.render(config, *args, **kwargs)
        for ex in expected:
            callback(text, ex)


def assert_render_in(
    cfg: Renderable,
    expected: str | list[str],
    *args,
    **kwargs,
) -> None:
    """Assert that a rendered text contains a substring.

    This function takes a configuration object, renders a text using the
    configuration's class render method, and then asserts that the expected
    substring is present in the rendered text. It performs this check twice:
    once with the original configuration object and once with a configuration
    object converted to a dictionary using OmegaConf.

    Args:
        cfg (BaseConfig): The configuration object to render the text from.
        expected (str | list[str]): The substring that must be present in the
            rendered text.
        *args: Additional positional arguments to pass to the `render` function.
        **kwargs: Additional keyword arguments to pass to the `render` function.

    Raises:
        AssertionError: If the expected substring is not found in the rendered
            text for either the original configuration object or the OmegaConf
            dictionary configuration.

    """

    def callback(text: str, expected: str) -> None:
        if expected not in text:
            msg = (
                "The rendered text is missing the expected substring.\n"
                "Missing substring:\n"
                f"{expected!r}\n"
                "Rendered text:\n"
                f"{text!r}"
            )
            raise AssertionError(msg)

    _assert_render(cfg, expected, callback, *args, **kwargs)


def assert_render_eq(
    cfg: Renderable,
    expected: str,
    *args,
    **kwargs,
) -> None:
    """Assert that a rendered text is equal to the expected string.

    This function takes a configuration object, renders a text using the
    configuration's class render method, and then asserts that the rendered
    text is equal to the expected string. It performs this check twice:
    once with the original configuration object and once with a configuration
    object converted to a dictionary using OmegaConf.

    Args:
        cfg (Renderable): The configuration object to render the text from.
        expected (str): The string that the rendered text must match.
        *args: Additional positional arguments to pass to the `render` function.
        **kwargs: Additional keyword arguments to pass to the `render` function.

    Raises:
        AssertionError: If the rendered text does not match the expected
            string for either the original configuration object or the
            OmegaConf dictionary configuration.

    """

    def callback(text: str, expected: str) -> None:
        if expected != text:
            msg = (
                "The rendered text does not match the expected string.\n"
                "Expected:\n"
                f"{expected!r}\n"
                "Rendered:\n"
                f"{text!r}"
            )
            raise AssertionError(msg)

    _assert_render(cfg, expected, callback, *args, **kwargs)


def assert_render_startswith(
    cfg: Renderable,
    expected: str,
    *args,
    **kwargs,
) -> None:
    """Assert that a rendered text starts with the expected string.

    This function takes a configuration object, renders a text using the
    configuration's class render method, and asserts that the rendered text
    starts with the specified string. It performs this check twice: once with
    the original configuration object and once with a configuration object
    converted to a dictionary using OmegaConf.

    Args:
        cfg (Renderable): The configuration object to render the text from.
        expected (str): The string that the rendered text must start with.
        *args: Additional positional arguments to pass to the `render` method.
        **kwargs: Additional keyword arguments to pass to the `render` method.

    Raises:
        AssertionError: If the rendered text does not start with the
            expected string for either the original configuration object or
            the OmegaConf dictionary configuration.

    """

    def callback(text: str, expected: str) -> None:
        if not text.startswith(expected):
            msg = "Rendered text does not start with the expected "
            msg += f"string: {expected!r}, but it starts with "
            msg += f"{text[:len(expected)]!r} instead."
            raise AssertionError(msg)

    _assert_render(cfg, expected, callback, *args, **kwargs)


def assert_render_endswith(
    cfg: Renderable,
    expected: str,
    *args,
    **kwargs,
) -> None:
    """Assert that a rendered text ends with the expected string.

    This function takes a configuration object, renders a text using the
    configuration's class render method, and asserts that the rendered text
    ends with the specified string. It performs this check twice: once with
    the original configuration object and once with a configuration object
    converted to a dictionary using OmegaConf.

    Args:
        cfg (Renderable): The configuration object to render the text from.
        expected (str): The string that the rendered text must end with.
        *args: Additional positional arguments to pass to the `render` method.
        **kwargs: Additional keyword arguments to pass to the `render` method.

    Raises:
        AssertionError: If the rendered text does not end with the
            expected string for either the original configuration object or
            the OmegaConf dictionary configuration.

    """

    def callback(text: str, expected: str) -> None:
        if not text.endswith(expected):
            msg = "Rendered text does not end with the expected "
            msg += f"string: {expected!r}, but it ends with "
            msg += f"{text[-len(expected):]!r} instead."
            raise AssertionError(msg)

    _assert_render(cfg, expected, callback, *args, **kwargs)

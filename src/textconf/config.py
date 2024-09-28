"""Provide tools for configuration and template-based rendering.

This module defines a base configuration class for text along with
functions to locate and render templates using these configurations.
It supports dynamic discovery of template methods within classes.
"""

from __future__ import annotations

import inspect
from abc import ABC, abstractmethod
from dataclasses import dataclass
from inspect import Signature
from pathlib import Path  # noqa: TCH003
from typing import TYPE_CHECKING, TypeGuard

from .render import render
from .template import get_template_file

if TYPE_CHECKING:
    from collections.abc import Callable, Iterator
    from typing import Any, Self


@dataclass
class Renderable(ABC):
    """Represent a renderable class."""

    @classmethod
    @abstractmethod
    def render(cls, cfg: Self, *args, **kwargs) -> str:
        """Render the given configuration and return a string."""


@dataclass
class BaseConfig(Renderable):
    """Represent a base configuration for text.

    This class provides a structure for storing configuration parameters
    and methods for updating and rendering text based on templates.

    Attributes:
        _template_ (str): The name of the template file.

    """

    _template_: str | Path = ""

    @classmethod
    def update(cls, cfg: Self) -> None:
        """Update the configuration in-place with new values.

        This method should be overridden by subclasses to update
        configuration parameters before rendering the template.

        Args:
            cfg (BaseConfig): The configuration instance to be updated.

        """

    @classmethod
    def render(cls, cfg: Self, **kwargs) -> str:
        """Render text from the specified configuration.

        This method locates the template file, updates the configuration,
        and renders the text using the template and additional keyword
        arguments provided. It supports dynamic template methods defined
        in the class.

        Args:
            cfg (BaseConfig): The configuration instance to render the
                text from.
            **kwargs: Additional keyword arguments to pass to the
                template rendering.

        Returns:
            str: The rendered text as a string.

        Raises:
            FileNotFoundError: If the template file does not exist
                in any of the searched directories.

        """
        cls.update(cfg)

        params = kwargs.copy()

        for name, obj in iter_template_methods(cls):
            if name not in params:
                params[name] = obj(cfg)

        template_file = get_template_file(cls, cfg._template_)
        return render(template_file, cfg, **params)


def iter_template_methods(cls: object) -> Iterator[tuple[str, Callable[[Any], str]]]:
    """Yield name and method pairs of template methods from a given class.

    This function iterates over all members of a class, checks if each member
    is a template method using the `is_template_method` function, and yields the
    name and the method itself if it is a template method.

    Args:
        cls (object): The class object whose members are to be checked
            for being template methods.

    Yields:
        tuple[str, Callable[[Any], str]]: An iterator of tuples, each
        containing the name of the template method and the method
        itself.

    """
    members = inspect.getmembers(cls)
    for name, obj in members:
        if not name.startswith("_") and is_template_method(obj):
            yield name, obj


def is_template_method(obj: object) -> TypeGuard[Callable[[Any], str]]:
    """Check if the object is a template method with specific characteristics.

    A template method in this context is considered to be a method that:
    - Is a bound method of a class (not a static or free function).
    - Accepts exactly one argument.
    - Has a return annotation that is neither None nor missing.

    Args:
        obj (object): The object to be inspected and verified.

    Returns:
        bool: True if the object is a method that matches the template
        method criteria, with a return annotation other than None.
        False otherwise.

    """
    if not inspect.ismethod(obj) or not inspect.isclass(obj.__self__):
        return False

    signature = inspect.signature(obj)
    if signature.return_annotation in [None, "None", Signature.empty]:
        return False

    return len(signature.parameters) == 1

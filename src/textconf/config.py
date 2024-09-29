"""Provide tools for configuration and template-based rendering.

This module defines a base configuration class for text along with
functions to locate and render templates using these configurations.
It supports dynamic discovery of template methods within classes.
"""

from __future__ import annotations

from abc import ABC, abstractmethod
from dataclasses import dataclass
from pathlib import Path  # noqa: TCH003
from typing import TYPE_CHECKING

from .render import render
from .template import get_template_file, iter_template_methods

if TYPE_CHECKING:
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
        _template_ (str | Path): The name or path of the template file.

    """

    _template_: str | Path = ""

    @classmethod
    def update(cls, cfg: Self) -> None:
        """Update the configuration in-place.

        This method should be overridden by subclasses to update
        configuration parameters before rendering the template.

        Args:
            cfg (Self): The configuration instance to be updated.

        """

    @classmethod
    def render(cls, cfg: Self, *args: dict[str, Any] | list[str], **kwargs) -> str:
        """Render text from the specified configuration.

        This method locates the template file, updates the configuration,
        and renders the text using the template and additional keyword
        arguments provided. It supports dynamic template methods defined
        in the class.

        Args:
            cfg (Self): The configuration instance to render the
                text from.
            *args (dict[str, Any] | list[str]): Additional positional
                arguments to include in the template context.
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
        return render(template_file, cfg, *args, **params)

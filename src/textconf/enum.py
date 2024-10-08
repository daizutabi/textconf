"""Provide renderable enums."""

from __future__ import annotations

import inspect
from enum import Enum
from typing import TYPE_CHECKING

from .config import Renderable

if TYPE_CHECKING:
    from typing import Any


class RenderableEnum(Enum):
    """Renderable enum."""

    def render(
        self,
        cfg: Renderable,
        *args: dict[str, Any] | list[str],
        **kwargs,
    ) -> str:
        """Render the enum value using the provided configuration.

        Args:
            cfg (object): The configuration instance to render the
                enum value with.
            *args (dict[str, Any] | list[str]): Additional positional
                arguments to include in the template context.
            **kwargs: Additional keyword arguments to pass to the
                template rendering.

        Returns:
            str: The rendered output from the corresponding renderable class.

        """
        module = inspect.getmodule(self)

        for name, obj in inspect.getmembers(module):
            if name == self.name and issubclass(obj, Renderable):
                return obj.render(cfg, *args, **kwargs)

        msg = f"No renderable class found for {self.name}"
        raise ValueError(msg)

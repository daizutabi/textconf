"""Provide utilities for loading and rendering Jinja2 templates.

Leverage the omegaconf library for configuration management.
It is designed to facilitate the dynamic generation of content
based on template files and configurable context parameters.
"""

from __future__ import annotations

from typing import TYPE_CHECKING

from omegaconf import DictConfig, OmegaConf

if TYPE_CHECKING:
    from typing import Any

    from jinja2 import Template


def render(
    template: Template,
    cfg: object | None = None,
    *args: dict[str, Any] | list[str],
    **kwargs,
) -> str:
    """Render a Jinja2 template with the given context.

    Take a template object and a configuration object or dictionary, and
    renders the template with the provided context. Additional context can
    be passed as keyword arguments.

    Args:
        template (Template): The template to render.
        cfg (object | None): The configuration object or dictionary to use as context
            for rendering the template. If configuration is not an instance of
            DictConfig, it will be converted using OmegaConf.structured.
        *args (dict[str, Any] | list[str]): Additional positional arguments to
            include in the template context.
        **kwargs: Additional keyword arguments to include in the template context.

    Returns:
        str: The rendered template as a string.
    """
    if not cfg:
        cfg = {}
    elif not isinstance(cfg, DictConfig):
        cfg = OmegaConf.structured(cfg)

    if args:
        dotlist = []
        for arg in args:
            dotlist.extend(to_dotlist(arg) if isinstance(arg, dict) else arg)

        arg = OmegaConf.from_dotlist(dotlist)
        cfg = OmegaConf.merge(cfg, arg)

    return template.render(cfg, **kwargs)


def to_dotlist(cfg: dict[str, Any]) -> list[str]:
    """Convert a dictionary to a list of dotlist strings.

    Args:
        cfg (dict[str, Any]): The dictionary to convert to a dotlist string.

    Returns:
        list[str]: A list of dotlist strings.
    """
    return [f"{k}={v}" for k, v in cfg.items()]

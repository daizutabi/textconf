"""Template file handling."""

from __future__ import annotations

import inspect
from inspect import Signature
from pathlib import Path
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from collections.abc import Callable, Iterator
    from typing import Any, TypeGuard


def get_template_file(
    cls: object,
    filename: str | Path,
    dir: str | Path = "templates",  # noqa: A002
) -> Path:
    """Return the path to a template file associated with a given class.

    This function searches for a template file with the specified name,
    first in the current directory, then in the provided directory,
    and finally in the directory relative to the module where the class
    is defined. If the file is not found, a FileNotFoundError is raised.

    Args:
        cls (object): The class object associated with the template file. It is
            used to determine the relative path if the template file is not
            found in the current or provided directory.
        filename (str | Path): The name of the template file.
        dir (str | Path, optional): The directory where the template files
            are stored. Defaults to "templates".

    Returns:
        Path: The absolute path to the template file.

    Raises:
        FileNotFoundError: If the template file does not exist in any of the
            searched directories.

    """
    file = Path(filename)

    if file.exists():
        return file.absolute()

    file_ = dir / file
    if file_.exists():
        return file_.absolute()

    from_dir = Path(inspect.getfile(cls)).parent  # type: ignore
    return _get_template_file_from_dir(file, dir, from_dir).absolute()


def _get_template_file_from_dir(file: Path, dir: str | Path, from_dir: Path) -> Path:  # noqa: A002
    file_ = from_dir / file
    if file_.exists():
        return file_

    file_ = from_dir / dir / file
    if file_.exists():
        return file_

    if from_dir.parent != from_dir:
        return _get_template_file_from_dir(file, dir, from_dir.parent)

    raise FileNotFoundError


def iter_template_methods(cls: object) -> Iterator[tuple[str, Callable[[Any], Any]]]:
    """Yield name and method pairs of template methods from a given class.

    Iterate over all members of a class, check if each member is a template
    method using the `is_template_method` function, and yield the name
    and the method itself if it is a template method.

    Args:
        cls (object): The class object whose members are to be checked
            for being template methods.

    Yields:
        tuple[str, Callable[[Any], Any]]: Tuples, each containing the name
        of the template method and the method itself.

    """
    members = inspect.getmembers(cls)
    for name, obj in members:
        if not name.startswith("_") and is_template_method(obj):
            yield name, obj


def is_template_method(obj: object) -> TypeGuard[Callable[[Any], Any]]:
    """Check if the object is a template method.

    A template method in this context is considered to be a method that

    - Is a bound method of a class (not a static or free function).
    - Accepts exactly one argument.
    - Has a return annotation that is neither None nor missing.

    Args:
        obj (object): The object to be inspected.

    Returns:
        bool: True if the object is a method that matches the template
        method criteria. False otherwise.

    """
    if not inspect.ismethod(obj) or not inspect.isclass(obj.__self__):
        return False

    signature = inspect.signature(obj)
    if signature.return_annotation in [None, "None", Signature.empty]:
        return False

    return len(signature.parameters) == 1

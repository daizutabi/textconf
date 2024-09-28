"""Template file handling."""

import inspect
from pathlib import Path


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

import os
import subprocess
import sys
import uuid
from pathlib import Path

import pytest


@pytest.fixture(scope="module")
def run(tmp_path_factory: pytest.TempPathFactory):
    parent = Path(__file__).parent

    def run(filename: str, args: list[str]):
        cwd = Path.cwd()

        os.chdir(tmp_path_factory.mktemp(str(uuid.uuid4())))

        args = [sys.executable, (parent / filename).as_posix(), *args]

        subprocess.run(args, check=False)

        outputs = [p.read_text() for p in Path().glob("**/text.txt")]

        os.chdir(cwd)

        return outputs

    return run


def to_dict(text: str) -> dict[str, str]:
    return dict(line.split(":") for line in text.splitlines())


@pytest.fixture(scope="module")
def collect(run):
    def collect(filename: str, args: list[str]):
        outputs = run(filename, args)
        return [to_dict(output) for output in outputs]

    return collect

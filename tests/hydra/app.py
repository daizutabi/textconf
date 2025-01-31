from __future__ import annotations

import math
from dataclasses import dataclass
from pathlib import Path
from typing import TYPE_CHECKING

import hydra
from hydra.core.config_store import ConfigStore
from hydra.core.hydra_config import HydraConfig

from textconf import BaseConfig

if TYPE_CHECKING:
    from typing import Self

    from jinja2 import Environment


@dataclass
class Point:
    x: float
    y: float

    def render(self) -> str:
        return f"({self.x}, {self.y})"


def distance(p: Point) -> float:
    return math.sqrt(p.x * p.x + p.y * p.y)


def twice(x: float) -> float:
    return 2 * x


@dataclass
class Config(BaseConfig):
    x: float = 0.1
    y: float = 0.2

    @staticmethod
    def point(cfg: Config) -> Point:
        return Point(cfg.x, cfg.y)

    @classmethod
    def z(cls, cfg: Self) -> float:
        return cfg.x + cfg.y

    @classmethod
    def context(cls, cfg: Self) -> dict[str, str]:
        point = cls.point(cfg)
        return {"point": point.render(), "distance": f"{distance(point):.3g}"}

    @classmethod
    def set_environment(cls, env: Environment) -> None:
        env.filters["twice"] = twice


ConfigStore.instance().store(name="config", node=Config("base.jinja"))


@hydra.main(version_base=None, config_name="config")
def app(cfg: Config):
    text = Config.render(cfg)
    path = Path(HydraConfig.get().runtime.output_dir) / "text.txt"
    path.write_text(text)


if __name__ == "__main__":
    app()

from __future__ import annotations

from dataclasses import dataclass
from typing import Mapping


@dataclass(frozen=True, slots=True)
class Location:
    """A playable office area in the abstract company map."""

    id: str
    name: str
    description: str
    connections: tuple[str, ...]
    tags: tuple[str, ...] = ()
    danger: int = 0


LocationMap = Mapping[str, Location]


def _load_runtime_default_locations() -> dict[str, Location]:
    from tui_adv.game.content import load_default_locations

    return load_default_locations()


DEFAULT_LOCATIONS: dict[str, Location] = _load_runtime_default_locations()

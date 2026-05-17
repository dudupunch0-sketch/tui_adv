from __future__ import annotations

from dataclasses import dataclass
from typing import Mapping


@dataclass(frozen=True, slots=True)
class Item:
    """Display metadata for an inventory item."""

    id: str
    name: str
    description: str
    kind: str
    tags: tuple[str, ...] = ()
    usable: bool = False


ItemMap = Mapping[str, Item]


def load_runtime_default_items() -> dict[str, Item]:
    """Load runtime default item metadata from packaged YAML content."""

    from tui_adv.game.content import load_default_items

    return load_default_items()


DEFAULT_ITEMS: dict[str, Item] = load_runtime_default_items()

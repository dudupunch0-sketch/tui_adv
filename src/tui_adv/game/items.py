from __future__ import annotations

from dataclasses import dataclass, field, replace
from typing import TYPE_CHECKING, Mapping

if TYPE_CHECKING:
    from tui_adv.game.state import GameState

_RESOURCE_NAMES = ("health", "sanity", "battery", "hunger", "thirst")


@dataclass(frozen=True, slots=True)
class Item:
    """Display metadata and optional use effects for an inventory item."""

    id: str
    name: str
    description: str
    kind: str
    tags: tuple[str, ...] = ()
    usable: bool = False
    use_effects: Mapping[str, int] = field(default_factory=dict)
    use_log: str = ""


@dataclass(frozen=True, slots=True)
class ItemUseResult:
    item: Item
    before_state: GameState
    state: GameState

    @property
    def new_logs(self) -> tuple[str, ...]:
        return tuple(self.state.log[len(self.before_state.log) :])


ItemMap = Mapping[str, Item]


def use_item(
    state: GameState,
    item_id: str,
    items: ItemMap | None = None,
) -> GameState:
    return use_item_result(state, item_id, items).state


def use_item_result(
    state: GameState,
    item_id: str,
    items: ItemMap | None = None,
) -> ItemUseResult:
    item_map = DEFAULT_ITEMS if items is None else items
    item = item_map.get(item_id)
    if item is None:
        raise ValueError(f"알 수 없는 아이템: {item_id}")
    if item_id not in state.inventory:
        raise ValueError(f"소지품에 없는 아이템: {item_id}")
    if not item.usable or not item.use_effects:
        raise ValueError(f"사용할 수 없는 아이템: {item_id}")

    effects = _normalized_effects(item)
    inventory = list(state.inventory)
    inventory.remove(item_id)
    used_state = replace(
        state,
        player=state.player.apply_delta(**effects),
        inventory=inventory,
        clues=list(state.clues),
        flags=list(state.flags),
        seen_encounters=list(state.seen_encounters),
        unlocked_achievements=list(state.unlocked_achievements),
        log=[*state.log, item.use_log or f"{item.name}을 사용했다."],
    ).advance_turn()
    return ItemUseResult(item=item, before_state=state, state=used_state)


def _normalized_effects(item: Item) -> dict[str, int]:
    effects = {resource_name: 0 for resource_name in _RESOURCE_NAMES}
    for resource_name, amount in item.use_effects.items():
        if resource_name not in effects:
            raise ValueError(f"unknown resource in item {item.id}: {resource_name}")
        effects[resource_name] = int(amount)
    return effects


def load_runtime_default_items() -> dict[str, Item]:
    """Load runtime default item metadata from packaged YAML content."""

    from tui_adv.game.content import load_default_items

    return load_default_items()


DEFAULT_ITEMS: dict[str, Item] = load_runtime_default_items()

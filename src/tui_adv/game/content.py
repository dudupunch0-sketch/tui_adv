"""YAML content loading and public-content validation."""

from __future__ import annotations

from importlib.resources import files
from pathlib import Path
from typing import TYPE_CHECKING, Any, Iterable

import yaml

if TYPE_CHECKING:
    from tui_adv.game.encounters import (
        AbilityCheck,
        Choice,
        Conditions,
        Encounter,
        Outcome,
    )
    from tui_adv.game.achievements import Achievement
    from tui_adv.game.items import Item
    from tui_adv.game.locations import Location

DATA_DIR = files("tui_adv.data")
_RESOURCE_NAMES = ("health", "sanity", "battery", "hunger", "thirst")


def load_default_locations() -> dict[str, Location]:
    return load_locations(DATA_DIR.joinpath("locations.yaml"))


def load_default_encounters() -> dict[str, Encounter]:
    return load_encounters(DATA_DIR.joinpath("encounters.yaml"))


def load_default_items() -> dict[str, Item]:
    return load_items(DATA_DIR.joinpath("items.yaml"))


def load_default_endings():
    return load_endings(DATA_DIR.joinpath("endings.yaml"))


def load_default_achievements() -> dict[str, Achievement]:
    return load_achievements(DATA_DIR.joinpath("achievements.yaml"))


def load_locations(path: Path | Any) -> dict[str, Location]:
    data = _read_yaml(path)
    locations: dict[str, Location] = {}
    for entry in data.get("locations", []):
        location = _location_from_data(entry)
        if location.id in locations:
            raise ValueError(f"duplicate location id: {location.id}")
        locations[location.id] = location
    _validate_location_connections(locations)
    return locations


def load_items(path: Path | Any) -> dict[str, Item]:
    data = _read_yaml(path)
    items: dict[str, Item] = {}
    for entry in data.get("items", []):
        item = _item_from_data(entry)
        if item.id in items:
            raise ValueError(f"duplicate item id: {item.id}")
        items[item.id] = item
    return items


def load_encounters(path: Path | Any) -> dict[str, Encounter]:
    data = _read_yaml(path)
    encounters: dict[str, Encounter] = {}
    for entry in data.get("encounters", []):
        encounter = _encounter_from_data(entry)
        if encounter.id in encounters:
            raise ValueError(f"duplicate encounter id: {encounter.id}")
        encounters[encounter.id] = encounter
    return encounters


def load_endings(path: Path | Any):
    from tui_adv.game.endings import Ending

    data = _read_yaml(path)
    endings: dict[str, Ending] = {}
    for entry in data.get("endings", []):
        ending = Ending(
            id=entry["id"],
            name=entry["name"],
            kind=entry["kind"],
            priority=int(entry.get("priority", 0)),
            conditions=_conditions_from_data(entry.get("conditions", {})),
            text=entry.get("text", ""),
            local_hint_id=entry.get("local_hint_id"),
        )
        if ending.id in endings:
            raise ValueError(f"duplicate ending id: {ending.id}")
        endings[ending.id] = ending
    return endings


def load_achievements(path: Path | Any) -> dict[str, Achievement]:
    from tui_adv.game.achievements import Achievement

    data = _read_yaml(path)
    achievements: dict[str, Achievement] = {}
    for entry in data.get("achievements", []):
        achievement = Achievement(
            id=entry["id"],
            name=entry["name"],
            description=entry.get("description", ""),
            conditions=_conditions_from_data(entry.get("conditions", {})),
            hidden=bool(entry.get("hidden", False)),
        )
        if achievement.id in achievements:
            raise ValueError(f"duplicate achievement id: {achievement.id}")
        achievements[achievement.id] = achievement
    return achievements


def validate_public_content() -> list[str]:
    """Return validation errors for committed public YAML content."""

    errors: list[str] = []
    try:
        locations = load_default_locations()
        items = load_default_items()
        encounters = load_default_encounters()
        endings = load_default_endings()
        achievements = load_default_achievements()
    except Exception as exc:  # pragma: no cover - test output needs message, not traceback.
        return [f"content load failed: {exc}"]

    for encounter in encounters.values():
        _validate_conditions_locations(
            errors, f"encounter:{encounter.id}", encounter.conditions, locations
        )
        _validate_conditions_items(
            errors, f"encounter:{encounter.id}", encounter.conditions, items
        )
        for choice in encounter.choices:
            _validate_conditions_locations(
                errors, f"choice:{encounter.id}.{choice.id}", choice.conditions, locations
            )
            _validate_conditions_items(
                errors, f"choice:{encounter.id}.{choice.id}", choice.conditions, items
            )
            _validate_outcome_destination(
                errors, f"choice:{encounter.id}.{choice.id}.outcome", choice.outcome, locations
            )
            _validate_outcome_items(
                errors, f"choice:{encounter.id}.{choice.id}.outcome", choice.outcome, items
            )
            if choice.check is not None:
                _validate_outcome_destination(
                    errors,
                    f"choice:{encounter.id}.{choice.id}.success",
                    choice.check.success,
                    locations,
                )
                _validate_outcome_items(
                    errors,
                    f"choice:{encounter.id}.{choice.id}.success",
                    choice.check.success,
                    items,
                )
                _validate_outcome_destination(
                    errors,
                    f"choice:{encounter.id}.{choice.id}.failure",
                    choice.check.failure,
                    locations,
                )
                _validate_outcome_items(
                    errors,
                    f"choice:{encounter.id}.{choice.id}.failure",
                    choice.check.failure,
                    items,
                )

    for ending in endings.values():
        _validate_conditions_locations(
            errors, f"ending:{ending.id}", ending.conditions, locations
        )
        _validate_conditions_items(
            errors, f"ending:{ending.id}", ending.conditions, items
        )

    for achievement in achievements.values():
        _validate_conditions_locations(
            errors, f"achievement:{achievement.id}", achievement.conditions, locations
        )
        _validate_conditions_items(
            errors, f"achievement:{achievement.id}", achievement.conditions, items
        )

    secrets_data = _read_yaml(DATA_DIR.joinpath("secrets.example.yaml"))
    for secret in secrets_data.get("secrets", []):
        if "final_hint" in secret:
            errors.append(f"public secret {secret.get('id', '<missing>')} has final_hint")
    return errors


def _location_from_data(entry: dict[str, Any]) -> Location:
    from tui_adv.game.locations import Location

    return Location(
        id=entry["id"],
        name=entry["name"],
        description=entry["description"],
        connections=_tuple(entry.get("connections", entry.get("exits", ()))),
        tags=_tuple(entry.get("tags", ())),
        danger=int(entry.get("danger", entry.get("danger_modifier", 0))),
    )


def _item_from_data(entry: dict[str, Any]) -> Item:
    from tui_adv.game.items import Item

    return Item(
        id=entry["id"],
        name=entry["name"],
        description=entry.get("description", ""),
        kind=entry["type"],
        tags=_tuple(entry.get("tags", ())),
        usable=bool(entry.get("usable", False)),
    )


def _encounter_from_data(entry: dict[str, Any]) -> Encounter:
    from tui_adv.game.encounters import Encounter

    return Encounter(
        id=entry["id"],
        title=entry["title"],
        body=entry["body"],
        conditions=_conditions_from_data(entry.get("conditions", {})),
        choices=tuple(_choice_from_data(choice) for choice in entry.get("choices", [])),
        repeatable=bool(entry.get("repeatable", False)),
        weight=int(entry.get("weight", 1)),
    )


def _choice_from_data(entry: dict[str, Any]) -> Choice:
    from tui_adv.game.encounters import Choice

    check_data = entry.get("check")
    return Choice(
        id=entry["id"],
        label=entry["label"],
        conditions=_conditions_from_data(entry.get("conditions", {})),
        cost=dict(entry.get("cost", {})),
        outcome=_outcome_from_data(entry.get("outcome", {})),
        check=_check_from_data(check_data) if check_data else None,
    )


def _check_from_data(entry: dict[str, Any]) -> AbilityCheck:
    from tui_adv.game.encounters import AbilityCheck

    return AbilityCheck(
        ability=entry["ability"],
        difficulty=int(entry["difficulty"]),
        success=_outcome_from_data(entry.get("success", {})),
        failure=_outcome_from_data(entry.get("failure", {})),
    )


def _conditions_from_data(entry: dict[str, Any]) -> Conditions:
    from tui_adv.game.encounters import Conditions

    return Conditions(
        locations=_tuple(entry.get("locations", ())),
        disaster_types=_tuple(entry.get("disaster_types", ())),
        required_items=_tuple(entry.get("required_items", entry.get("has_items", ()))),
        required_clues=_tuple(entry.get("required_clues", entry.get("has_clues", ()))),
        required_flags=_tuple(entry.get("required_flags", entry.get("has_flags", ()))),
        forbidden_flags=_tuple(entry.get("forbidden_flags", entry.get("missing_flags", ()))),
        min_resources=dict(entry.get("min_resources", {})),
        max_resources=dict(entry.get("max_resources", {})),
        min_abilities=dict(entry.get("min_abilities", {})),
    )


def _outcome_from_data(entry: dict[str, Any]) -> Outcome:
    from tui_adv.game.encounters import Outcome

    resources = dict(entry.get("resources", {}))
    for resource_name in _RESOURCE_NAMES:
        if resource_name in entry:
            resources[resource_name] = entry[resource_name]
    return Outcome(
        health=int(resources.get("health", 0)),
        sanity=int(resources.get("sanity", 0)),
        battery=int(resources.get("battery", 0)),
        hunger=int(resources.get("hunger", 0)),
        thirst=int(resources.get("thirst", 0)),
        add_items=_tuple(entry.get("add_items", ())),
        remove_items=_tuple(entry.get("remove_items", ())),
        add_clues=_tuple(entry.get("add_clues", ())),
        add_flags=_tuple(entry.get("add_flags", ())),
        remove_flags=_tuple(entry.get("remove_flags", ())),
        destination_id=entry.get("destination_id"),
        danger=int(entry.get("danger", 0)),
        log=entry.get("log", ""),
    )


def _validate_location_connections(locations: dict[str, Location]) -> None:
    for location in locations.values():
        for connection_id in location.connections:
            if connection_id not in locations:
                raise ValueError(
                    f"location {location.id} references unknown connection: {connection_id}"
                )


def _validate_conditions_locations(
    errors: list[str],
    label: str,
    conditions: Conditions,
    locations: dict[str, Location],
) -> None:
    for location_id in conditions.locations:
        if location_id not in locations:
            errors.append(f"{label} references unknown location: {location_id}")


def _validate_conditions_items(
    errors: list[str],
    label: str,
    conditions: Conditions,
    items: dict[str, Item],
) -> None:
    for item_id in conditions.required_items:
        if item_id not in items:
            errors.append(f"{label} references unknown item: {item_id}")


def _validate_outcome_destination(
    errors: list[str],
    label: str,
    outcome: Outcome,
    locations: dict[str, Location],
) -> None:
    if outcome.destination_id and outcome.destination_id not in locations:
        errors.append(f"{label} references unknown destination: {outcome.destination_id}")


def _validate_outcome_items(
    errors: list[str],
    label: str,
    outcome: Outcome,
    items: dict[str, Item],
) -> None:
    for item_id in (*outcome.add_items, *outcome.remove_items):
        if item_id not in items:
            errors.append(f"{label} references unknown item: {item_id}")


def _read_yaml(path: Path | Any) -> dict[str, Any]:
    text = path.read_text(encoding="utf-8")
    loaded = yaml.safe_load(text)
    if loaded is None:
        return {}
    if not isinstance(loaded, dict):
        raise ValueError(f"YAML root must be a mapping: {path}")
    return loaded


def _tuple(values: Iterable[str]) -> tuple[str, ...]:
    return tuple(values)

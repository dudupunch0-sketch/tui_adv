"""YAML content loading and public-content validation."""

from __future__ import annotations

from importlib.resources import files
from pathlib import Path
from typing import Any, Iterable

import yaml

from tui_adv.game.encounters import AbilityCheck, Choice, Conditions, Encounter, Outcome
from tui_adv.game.locations import DEFAULT_LOCATIONS

DATA_DIR = files("tui_adv.data")
_RESOURCE_NAMES = ("health", "sanity", "battery", "hunger", "thirst")


def load_default_encounters() -> dict[str, Encounter]:
    return load_encounters(DATA_DIR.joinpath("encounters.yaml"))


def load_default_endings():
    return load_endings(DATA_DIR.joinpath("endings.yaml"))


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


def validate_public_content() -> list[str]:
    """Return validation errors for committed public YAML content."""

    errors: list[str] = []
    try:
        encounters = load_default_encounters()
        endings = load_default_endings()
    except Exception as exc:  # pragma: no cover - test output needs message, not traceback.
        return [f"content load failed: {exc}"]

    for encounter in encounters.values():
        _validate_conditions_locations(
            errors, f"encounter:{encounter.id}", encounter.conditions
        )
        for choice in encounter.choices:
            _validate_conditions_locations(
                errors, f"choice:{encounter.id}.{choice.id}", choice.conditions
            )
            _validate_outcome_destination(
                errors, f"choice:{encounter.id}.{choice.id}.outcome", choice.outcome
            )
            if choice.check is not None:
                _validate_outcome_destination(
                    errors,
                    f"choice:{encounter.id}.{choice.id}.success",
                    choice.check.success,
                )
                _validate_outcome_destination(
                    errors,
                    f"choice:{encounter.id}.{choice.id}.failure",
                    choice.check.failure,
                )

    for ending in endings.values():
        _validate_conditions_locations(errors, f"ending:{ending.id}", ending.conditions)

    secrets_data = _read_yaml(DATA_DIR.joinpath("secrets.example.yaml"))
    for secret in secrets_data.get("secrets", []):
        if "final_hint" in secret:
            errors.append(f"public secret {secret.get('id', '<missing>')} has final_hint")
    return errors


def _encounter_from_data(entry: dict[str, Any]) -> Encounter:
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
    return AbilityCheck(
        ability=entry["ability"],
        difficulty=int(entry["difficulty"]),
        success=_outcome_from_data(entry.get("success", {})),
        failure=_outcome_from_data(entry.get("failure", {})),
    )


def _conditions_from_data(entry: dict[str, Any]) -> Conditions:
    return Conditions(
        locations=_tuple(entry.get("locations", ())),
        disaster_types=_tuple(entry.get("disaster_types", ())),
        required_items=_tuple(entry.get("required_items", entry.get("has_items", ()))),
        required_flags=_tuple(entry.get("required_flags", entry.get("has_flags", ()))),
        forbidden_flags=_tuple(entry.get("forbidden_flags", entry.get("missing_flags", ()))),
        min_resources=dict(entry.get("min_resources", {})),
        max_resources=dict(entry.get("max_resources", {})),
        min_abilities=dict(entry.get("min_abilities", {})),
    )


def _outcome_from_data(entry: dict[str, Any]) -> Outcome:
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


def _validate_conditions_locations(
    errors: list[str],
    label: str,
    conditions: Conditions,
) -> None:
    for location_id in conditions.locations:
        if location_id not in DEFAULT_LOCATIONS:
            errors.append(f"{label} references unknown location: {location_id}")


def _validate_outcome_destination(
    errors: list[str],
    label: str,
    outcome: Outcome,
) -> None:
    if outcome.destination_id and outcome.destination_id not in DEFAULT_LOCATIONS:
        errors.append(f"{label} references unknown destination: {outcome.destination_id}")


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

"""Machine-readable storypack candidate DB loading and validation.

This module intentionally reads docs/content/storypack_db from a repository root.
The DB is design-time content, not runtime game content.
"""

from __future__ import annotations

from dataclasses import dataclass
import json
from pathlib import Path
from typing import Any

DB_RELATIVE_DIR = Path("docs/content/storypack_db")
STORYPACKS_FILE = "storypacks.json"
ENCOUNTER_SITUATIONS_FILE = "encounter_situations.json"

_ALLOWED_STATUSES = {
    "raw",
    "candidate",
    "curated",
    "promoted",
    "merged",
    "rejected",
    "implemented_in_storypack_preview",
}
_ALLOWED_PRIORITY_CLASSES = {
    "main_forced",
    "route_key",
    "npc_relation",
    "random_pack",
    "generic_pressure",
    "ambient",
}
_ALLOWED_SURFACES = {
    "messenger",
    "intranet",
    "meeting_minutes",
    "reservation_panel",
    "cctv",
    "document_viewer",
    "approval_system",
    "organization_chart",
    "build_log",
    "payroll_sheet",
    "security_gate",
    "office_object",
    "commute_rift",
    "market_street",
    "office_items",
    "sect_courtyard",
    "training_chore",
    "cheonggi_record",
    "fragment_choice",
    "sect_raid",
    "faction_negotiation",
    "food",
    "daily_care",
}
_ALLOWED_ANOMALY_TYPES = {
    "mismatched_floor",
    "delayed_time",
    "future_record",
    "absent_people",
    "identity_erasure",
    "document_contamination",
    "space_loop",
    "permission_denied_as_existence",
    "worldline_branch",
    "world_displacement",
    "workplace_memory_mismatch",
    "outsider_without_sect",
    "first_brawl_defeat",
    "notebook_oracle",
    "fragment_choice",
    "sect_debt",
    "faction_pressure",
    "qi_deviation",
    "oath_binding",
}
_ALLOWED_PRESSURE_TYPES = {
    "health",
    "sanity",
    "battery",
    "hunger",
    "thirst",
    "danger",
    "relation",
}
_ALLOWED_NPC_SLOTS = {
    "infra_interpreter",
    "hr_identity_keeper",
    "security_witness",
    "facility_pathfinder",
    "finance_record_reader",
    "pm_worldline_mediator",
    "cleaning_unofficial_witness",
    "newcomer_mirror",
    "early_rescuer",
    "sect_master_guardian",
    "archive_keeper",
    "righteous_ally",
    "sapa_ally",
    "cheonggi_record_keeper",
    "blood_moon_antagonist",
}
_PRIVATE_ONLY_FIELDS = {
    "final_hint",
    "actual_ip_address",
    "office_location",
    "treasure_location",
}
_SAFE_CHOICE_ROLES = {"leave", "ignore", "wait"}
_REQUIRED_STORYPACK_FIELDS = (
    "id",
    "world_id",
    "status",
    "name",
    "one_line",
    "main_surfaces",
    "anomaly_types",
    "main_phases",
    "reusable_npc_slots",
    "ending_candidates",
    "main_spine_support",
)
_REQUIRED_CARD_FIELDS = (
    "id",
    "world_id",
    "storypack_id",
    "status",
    "phase",
    "priority_class",
    "location_tags",
    "surface",
    "anomaly_type",
    "pressure_type",
    "choice_shapes",
    "outcome_hooks",
    "main_spine_link",
)


@dataclass(frozen=True)
class StorypackRecord:
    id: str
    world_id: str
    status: str
    source_refs: tuple[str, ...]
    name: str
    one_line: str
    main_surfaces: tuple[str, ...]
    anomaly_types: tuple[str, ...]
    main_phases: tuple[str, ...]
    sensitive_topics: tuple[str, ...]
    reusable_npc_slots: tuple[str, ...]
    ending_candidates: tuple[str, ...]
    main_spine_support: str
    runtime_promotion_notes: str


@dataclass(frozen=True)
class EncounterSituationCard:
    id: str
    world_id: str
    storypack_id: str
    status: str
    phases: tuple[str, ...]
    priority_class: str
    location_tags: tuple[str, ...]
    surfaces: tuple[str, ...]
    anomaly_types: tuple[str, ...]
    pressure_types: tuple[str, ...]
    npc_slots: tuple[str, ...]
    candidate_characters: tuple[str, ...]
    summary: str
    setup_text: str
    choice_shapes: tuple[dict[str, Any], ...]
    outcome_hooks: dict[str, Any]
    main_spine_link: str
    randomization_notes: str
    promotion_notes: str


@dataclass(frozen=True)
class StorypackDB:
    storypacks: dict[str, StorypackRecord]
    encounter_cards: dict[str, EncounterSituationCard]

    def cards_by_storypack(self, storypack_id: str) -> tuple[str, ...]:
        return tuple(
            card_id
            for card_id, card in self.encounter_cards.items()
            if card.storypack_id == storypack_id
        )


def load_storypack_db(root: Path | str) -> StorypackDB:
    """Load the design-time storypack DB from a repository root."""

    root_path = Path(root)
    storypacks_data, cards_data = _read_db_files(root_path)
    storypacks = {
        record.id: record for record in (_storypack_from_data(entry) for entry in storypacks_data)
    }
    cards = {card.id: card for card in (_card_from_data(entry) for entry in cards_data)}
    return StorypackDB(storypacks=storypacks, encounter_cards=cards)


def validate_storypack_db(root: Path | str) -> list[str]:
    """Return design-time DB validation errors without raising on bad content."""

    root_path = Path(root)
    try:
        storypacks_data, cards_data = _read_db_files(root_path)
    except Exception as exc:
        return [f"storypack DB load failed: {exc}"]

    errors: list[str] = []
    storypack_worlds: dict[str, str] = {}
    storypack_ids: set[str] = set()
    card_ids: set[str] = set()

    for entry in storypacks_data:
        storypack_id = str(entry.get("id", "<missing>"))
        for field in _REQUIRED_STORYPACK_FIELDS:
            if _missing(entry, field):
                errors.append(f"storypack {storypack_id} missing field: {field}")
        if storypack_id in storypack_ids:
            errors.append(f"duplicate storypack id: {storypack_id}")
        storypack_ids.add(storypack_id)
        storypack_worlds[storypack_id] = str(entry.get("world_id", ""))
        status = str(entry.get("status", ""))
        if status and status not in _ALLOWED_STATUSES:
            errors.append(f"storypack {storypack_id} has unknown status: {status}")
        for surface in _as_tuple(entry.get("main_surfaces", ())):
            if surface not in _ALLOWED_SURFACES:
                errors.append(f"storypack {storypack_id} has unknown surface: {surface}")
        for anomaly_type in _as_tuple(entry.get("anomaly_types", ())):
            if anomaly_type not in _ALLOWED_ANOMALY_TYPES:
                errors.append(
                    f"storypack {storypack_id} has unknown anomaly_type: {anomaly_type}"
                )
        for npc_slot in _as_tuple(entry.get("reusable_npc_slots", ())):
            if npc_slot not in _ALLOWED_NPC_SLOTS:
                errors.append(f"storypack {storypack_id} has unknown npc_slot: {npc_slot}")
        if not str(entry.get("main_spine_support", "")).strip():
            errors.append(f"storypack {storypack_id} main_spine_support is required")

    for entry in cards_data:
        card_id = str(entry.get("id", "<missing>"))
        for field in _REQUIRED_CARD_FIELDS:
            if _missing(entry, field):
                errors.append(f"card {card_id} missing field: {field}")
        if card_id in card_ids:
            errors.append(f"duplicate card id: {card_id}")
        card_ids.add(card_id)

        storypack_id = str(entry.get("storypack_id", ""))
        card_world_id = str(entry.get("world_id", ""))
        if storypack_id not in storypack_worlds:
            errors.append(f"card {card_id} references unknown storypack_id: {storypack_id}")
        elif card_world_id != storypack_worlds[storypack_id]:
            errors.append(
                f"card {card_id} world_id {card_world_id} does not match storypack "
                f"{storypack_id} world_id {storypack_worlds[storypack_id]}"
            )

        status = str(entry.get("status", ""))
        if status and status not in _ALLOWED_STATUSES:
            errors.append(f"card {card_id} has unknown status: {status}")
        priority_class = str(entry.get("priority_class", ""))
        if priority_class and priority_class not in _ALLOWED_PRIORITY_CLASSES:
            errors.append(f"card {card_id} has unknown priority_class: {priority_class}")
        for surface in _as_tuple(entry.get("surface", ())):
            if surface not in _ALLOWED_SURFACES:
                errors.append(f"card {card_id} has unknown surface: {surface}")
        for anomaly_type in _as_tuple(entry.get("anomaly_type", ())):
            if anomaly_type not in _ALLOWED_ANOMALY_TYPES:
                errors.append(f"card {card_id} has unknown anomaly_type: {anomaly_type}")
        for pressure_type in _as_tuple(entry.get("pressure_type", ())):
            if pressure_type not in _ALLOWED_PRESSURE_TYPES:
                errors.append(f"card {card_id} has unknown pressure_type: {pressure_type}")
        for npc_slot in _as_tuple(entry.get("npc_slots", ())):
            if npc_slot not in _ALLOWED_NPC_SLOTS:
                errors.append(f"card {card_id} has unknown npc_slot: {npc_slot}")

        if not _has_safe_choice(entry.get("choice_shapes", ())):
            errors.append(f"card {card_id} has no fallback/safe choice role")
        if not _has_outcome_hook(entry.get("outcome_hooks", {})):
            errors.append(f"card {card_id} has no outcome hook")
        if not str(entry.get("main_spine_link", "")).strip():
            errors.append(f"card {card_id} main_spine_link is required")

    _validate_no_private_fields(errors, "storypacks", storypacks_data)
    _validate_no_private_fields(errors, "encounter_situations", cards_data)
    return errors


def _storypack_from_data(entry: dict[str, Any]) -> StorypackRecord:
    return StorypackRecord(
        id=str(entry["id"]),
        world_id=str(entry["world_id"]),
        status=str(entry["status"]),
        source_refs=_as_tuple(entry.get("source_refs", ())),
        name=str(entry["name"]),
        one_line=str(entry["one_line"]),
        main_surfaces=_as_tuple(entry.get("main_surfaces", ())),
        anomaly_types=_as_tuple(entry.get("anomaly_types", ())),
        main_phases=_as_tuple(entry.get("main_phases", ())),
        sensitive_topics=_as_tuple(entry.get("sensitive_topics", ())),
        reusable_npc_slots=_as_tuple(entry.get("reusable_npc_slots", ())),
        ending_candidates=_as_tuple(entry.get("ending_candidates", ())),
        main_spine_support=str(entry.get("main_spine_support", "")),
        runtime_promotion_notes=str(entry.get("runtime_promotion_notes", "")),
    )


def _card_from_data(entry: dict[str, Any]) -> EncounterSituationCard:
    return EncounterSituationCard(
        id=str(entry["id"]),
        world_id=str(entry["world_id"]),
        storypack_id=str(entry["storypack_id"]),
        status=str(entry["status"]),
        phases=_as_tuple(entry.get("phase", ())),
        priority_class=str(entry["priority_class"]),
        location_tags=_as_tuple(entry.get("location_tags", ())),
        surfaces=_as_tuple(entry.get("surface", ())),
        anomaly_types=_as_tuple(entry.get("anomaly_type", ())),
        pressure_types=_as_tuple(entry.get("pressure_type", ())),
        npc_slots=_as_tuple(entry.get("npc_slots", ())),
        candidate_characters=_as_tuple(entry.get("candidate_characters", ())),
        summary=str(entry.get("summary", "")),
        setup_text=str(entry.get("setup_text", "")),
        choice_shapes=tuple(
            dict(choice) for choice in entry.get("choice_shapes", ()) if isinstance(choice, dict)
        ),
        outcome_hooks=dict(entry.get("outcome_hooks", {})),
        main_spine_link=str(entry.get("main_spine_link", "")),
        randomization_notes=str(entry.get("randomization_notes", "")),
        promotion_notes=str(entry.get("promotion_notes", "")),
    )


def _read_db_files(root: Path) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    db_dir = root / DB_RELATIVE_DIR
    storypacks = _read_json_list(db_dir / STORYPACKS_FILE)
    cards = _read_json_list(db_dir / ENCOUNTER_SITUATIONS_FILE)
    return storypacks, cards


def _read_json_list(path: Path) -> list[dict[str, Any]]:
    data = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(data, list):
        raise ValueError(f"JSON root must be a list: {path}")
    for index, entry in enumerate(data):
        if not isinstance(entry, dict):
            raise ValueError(f"JSON entry {index} must be an object: {path}")
    return data


def _as_tuple(value: Any) -> tuple[str, ...]:
    if value is None:
        return ()
    if isinstance(value, str):
        return (value,)
    return tuple(str(item) for item in value)


def _missing(entry: dict[str, Any], field: str) -> bool:
    if field not in entry:
        return True
    value = entry[field]
    if value is None:
        return True
    if isinstance(value, str) and not value.strip():
        return True
    if isinstance(value, (list, tuple, dict)) and not value:
        return True
    return False


def _has_safe_choice(choice_shapes: Any) -> bool:
    if not isinstance(choice_shapes, list):
        return False
    for choice in choice_shapes:
        if not isinstance(choice, dict):
            continue
        role = str(choice.get("role", ""))
        if role.startswith("safe_") or role in _SAFE_CHOICE_ROLES:
            return True
    return False


def _has_outcome_hook(outcome_hooks: Any) -> bool:
    if not isinstance(outcome_hooks, dict):
        return False
    for value in outcome_hooks.values():
        if isinstance(value, list) and value:
            return True
        if isinstance(value, dict) and value:
            return True
        if isinstance(value, str) and value.strip():
            return True
    return False


def _validate_no_private_fields(errors: list[str], label: str, payload: Any) -> None:
    if isinstance(payload, dict):
        for key, value in payload.items():
            if key in _PRIVATE_ONLY_FIELDS:
                errors.append(f"{label} has private-only field: {key}")
            _validate_no_private_fields(errors, label, value)
    elif isinstance(payload, list):
        for value in payload:
            _validate_no_private_fields(errors, label, value)

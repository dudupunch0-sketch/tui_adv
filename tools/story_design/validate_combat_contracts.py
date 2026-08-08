#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
from pathlib import Path

import yaml


EXPECTED = {
    "termination": "combat_termination",
    "simulation_version": "combat_simulation_version",
    "identity": "combat_display_identity",
    "logs": "combat_log_presentation",
    "tactical_zones": "combat_tactical_zones",
    "intervention": "combat_intervention_response",
}
REQUIRED_TERMINATION = [
    "forced_stop",
    "captured",
    "surrendered",
    "fled",
    "objective_completed",
    "both_sides_defeated",
    "one_side_defeated",
    "max_ticks",
]
REQUIRED_FACTS = {
    "forced_stop_requested",
    "actor_captured",
    "actor_surrendered",
    "actor_fled",
    "objective_progressed",
    "both_sides_defeated",
    "one_side_defeated",
    "tick_limit_reached",
}
REQUIRED_RESULTS = {
    "victory",
    "defeat",
    "escape",
    "surrender",
    "capture",
    "objective",
    "forced_stop",
}
LEGACY_SELECTOR_ALIASES = {"self", "target", "observer", "opponent", "any"}
ACTION_KINDS = ["set_flag", "create_loot_entitlement", "grant_item"]
STRATEGY_SCOPES = ["all_allies", "role", "combatants"]
STRATEGY_DURATIONS = ["until_replaced", "next_segment"]
STRATEGY_OPERATIONS = [
    "set_role_weight",
    "set_targeting_rule",
    "set_target_policy",
    "clear_override",
]
LIFECYCLE_STATES = ["running", "paused_for_intervention", "terminal"]
KO_TICK_ORDER = [
    "apply_tick_results",
    "collect_provisional_terminal_facts",
    "detect_intervention",
    "apply_intervention",
    "recompute_terminal_facts",
    "settle_terminal",
]
FINGERPRINTS = [
    "intent_fingerprint",
    "decision_receipt_fingerprint",
    "checkpoint_fingerprint",
]


class UniqueKeyLoader(yaml.SafeLoader):
    pass


def construct_unique_mapping(loader, node, deep=False):
    mapping = {}
    for key_node, value_node in node.value:
        key = loader.construct_object(key_node, deep=deep)
        if key in mapping:
            raise yaml.constructor.ConstructorError(
                "while constructing a mapping",
                node.start_mark,
                f"duplicate key: {key}",
                key_node.start_mark,
            )
        mapping[key] = loader.construct_object(value_node, deep=deep)
    return mapping


UniqueKeyLoader.add_constructor(
    yaml.resolver.BaseResolver.DEFAULT_MAPPING_TAG,
    construct_unique_mapping,
)


def load(path: Path):
    return yaml.load(path.read_text(encoding="utf-8"), Loader=UniqueKeyLoader)


def runtime_simulation_version() -> str:
    repo_root = Path(__file__).resolve().parents[2]
    source = repo_root / "crates/escape-core/src/combat_contract.rs"
    match = re.search(
        r'pub const CURRENT_SIMULATION_VERSION:\s*&str\s*=\s*"([^"]+)"',
        source.read_text(encoding="utf-8"),
    )
    if not match:
        raise ValueError(f"cannot read runtime simulation capability from {source}")
    return match.group(1)


def interval_error(zones, axis):
    ranges = []
    for zone in zones:
        bounds = zone.get("bounds", {}).get(axis)
        if not isinstance(bounds, list) or len(bounds) != 2:
            return f"{axis} bounds must contain [min, max]"
        lo, hi = bounds
        if not (
            isinstance(lo, (int, float))
            and isinstance(hi, (int, float))
            and 0 <= lo < hi <= 1
        ):
            return f"{axis} bounds out of range"
        ranges.append((lo, hi, zone.get("id")))
    ranges.sort()
    cursor = 0.0
    for lo, hi, zid in ranges:
        if lo < cursor:
            return f"{axis} overlap at {zid}"
        if lo > cursor:
            return f"{axis} gap before {zid}"
        cursor = hi
    if cursor != 1.0:
        return f"{axis} incomplete coverage at {cursor}"
    return None


def require_equal(errors, label, actual, expected):
    if actual != expected:
        errors.append(f"{label}: expected {expected!r}, got {actual!r}")


def validate_intervention(value, errors):
    response = value.get("response_payload", {})
    require_equal(
        errors,
        "intervention response optional payloads",
        response.get("optional_fields"),
        ["strategy_modifier", "special_effect"],
    )
    require_equal(errors, "intervention response minimum payload", response.get("minimum_present"), 1)
    require_equal(
        errors,
        "intervention derived kinds",
        response.get("derived_kinds"),
        ["effect_only", "strategy_only", "both"],
    )
    forbidden = set(response.get("forbidden_fields", []))
    if not {"intervention_kind", "resolution_kind", "hybrid_kind"}.issubset(forbidden):
        errors.append("intervention: explicit kind/resolution/hybrid fields must be forbidden")
    require_equal(
        errors,
        "intervention composite strategy outcomes",
        response.get("composite_strategy_applies_on"),
        ["special_effect_success", "special_effect_failure"],
    )

    transaction = value.get("transaction", {})
    require_equal(errors, "intervention transaction boundary", transaction.get("boundary"), "one_pause_before_next_tick")
    require_equal(errors, "intervention transaction snapshot", transaction.get("input_snapshot"), "pause_snapshot")
    require_equal(errors, "intervention checkpoint states", transaction.get("checkpoint_states"), ["pre_transaction", "post_transaction"])
    require_equal(errors, "intervention partial serialization", transaction.get("partial_apply_serialization"), "forbidden")
    require_equal(errors, "intervention delayed workflow", transaction.get("delayed_or_multi_tick_workflow"), "forbidden")
    failure = transaction.get("validation_failure", {})
    for key in ("state_changes", "cost_changes", "rng_draws", "history_changes"):
        require_equal(errors, f"intervention validation failure {key}", failure.get(key), 0)
    require_equal(errors, "intervention validation failure pause", failure.get("pause"), "retained")
    require_equal(
        errors,
        "intervention rng draws",
        transaction.get("rng_draws"),
        {"strategy_only": 0, "deterministic_special_effect": 0, "probabilistic_special_effect": 1},
    )

    registry = value.get("registry", {})
    require_equal(errors, "intervention selector registry version", registry.get("selector_version"), "v1")
    require_equal(errors, "intervention formula registry version", registry.get("formula_version"), "v1")
    require_equal(errors, "intervention canonical runtime IDs", registry.get("canonical_only_runtime"), True)
    require_equal(errors, "intervention unknown registry ID", registry.get("unknown_id"), "validator_error")
    require_equal(errors, "intervention legacy policy", registry.get("legacy_policy"), "migration_only")
    require_equal(errors, "intervention legacy alias set", set(registry.get("legacy_aliases", [])), LEGACY_SELECTOR_ALIASES)
    require_equal(errors, "intervention selector tie break", registry.get("tie_break"), "stable_combatant_id_ascending")
    selectors = registry.get("selector_ids", {})
    canonical_ids = list(selectors.get("executor", [])) + list(selectors.get("target", []))
    if not canonical_ids or len(canonical_ids) != len(set(canonical_ids)):
        errors.append("intervention: selector IDs must be present and unique")
    for selector_id in canonical_ids:
        if selector_id in LEGACY_SELECTOR_ALIASES or not selector_id.startswith("combat.selector.") or ".v1." not in selector_id:
            errors.append(f"intervention: legacy or non-canonical selector ID: {selector_id}")
    formula_ids = registry.get("formula_ids", [])
    if not formula_ids or len(formula_ids) != len(set(formula_ids)):
        errors.append("intervention: formula IDs must be present and unique")
    for formula_id in formula_ids:
        if not formula_id.startswith("combat.formula.v1."):
            errors.append(f"intervention: non-canonical formula ID: {formula_id}")

    special = value.get("special_effect", {})
    require_equal(errors, "intervention outcome action kinds", special.get("outcome_action_kinds"), ACTION_KINDS)
    require_equal(errors, "intervention action plan owner", special.get("action_plan_owner"), "combat_core")
    require_equal(errors, "intervention action application owner", special.get("application_owner"), "gamecore")
    require_equal(errors, "intervention exactly-once keys", special.get("exactly_once_keys"), ["action_id", "application_transaction_id"])
    loot = special.get("loot_entitlement", {})
    require_equal(errors, "intervention loot claim defaults", loot.get("default_claimable_results"), ["victory", "objective"])
    require_equal(errors, "intervention loot denied defaults", loot.get("default_denied_results"), ["escape", "defeat", "surrender", "capture", "forced_stop"])
    require_equal(errors, "intervention mutual defeat loot", loot.get("default_denied_reasons"), ["both_sides_defeated"])

    strategy = value.get("strategy_modifier", {})
    require_equal(errors, "intervention strategy baseline", strategy.get("baseline"), "immutable")
    require_equal(errors, "intervention strategy scopes", strategy.get("scopes"), STRATEGY_SCOPES)
    require_equal(errors, "intervention strategy operations", strategy.get("operations"), STRATEGY_OPERATIONS)
    require_equal(errors, "intervention strategy durations", strategy.get("durations"), STRATEGY_DURATIONS)
    require_equal(errors, "intervention strategy default duration", strategy.get("default_duration"), "until_replaced")
    require_equal(errors, "intervention strategy precedence", strategy.get("precedence"), ["combatant", "role", "side", "baseline"])
    stacking = strategy.get("stacking", {})
    require_equal(errors, "intervention same-field stacking", stacking.get("same_scope_and_field"), "latest_replaces")
    require_equal(errors, "intervention disjoint stacking", stacking.get("disjoint_fields"), "coexist")
    require_equal(errors, "intervention additive stacking", stacking.get("additive_numeric"), "forbidden")
    require_equal(errors, "intervention arbitrary patch", stacking.get("arbitrary_json_patch"), "forbidden")

    lifecycle = value.get("lifecycle", {})
    require_equal(errors, "intervention lifecycle states", lifecycle.get("states"), LIFECYCLE_STATES)
    require_equal(errors, "intervention pause terminal candidate", lifecycle.get("pause_is_termination_candidate"), False)
    require_equal(errors, "intervention stable pause ID", lifecycle.get("stable_pause_id"), "required")
    require_equal(errors, "intervention stale response", lifecycle.get("stale_response"), "reject")
    require_equal(errors, "intervention nested pause", lifecycle.get("nested_pause"), "forbidden")
    require_equal(errors, "intervention host timeout", lifecycle.get("host_timeout"), "submit_no_intervention")
    require_equal(errors, "intervention KO tick order", lifecycle.get("ko_tick_order"), KO_TICK_ORDER)
    forced_stop = lifecycle.get("forced_stop", {})
    require_equal(errors, "intervention forced stop override", forced_stop.get("overrides_pause"), True)
    require_equal(errors, "intervention forced stop priority", forced_stop.get("terminal_priority"), 10)

    receipt = value.get("checkpoint_and_receipt", {})
    require_equal(errors, "intervention storage model", receipt.get("storage_model"), "resolved_decision_receipt_plus_compact_state_snapshot")
    require_equal(errors, "intervention save schema", receipt.get("save_schema_version"), 2)
    require_equal(errors, "intervention checkpoint schema", receipt.get("combat_checkpoint_schema_version"), 2)
    require_equal(errors, "intervention receipt schema", receipt.get("decision_receipt_schema_version"), 1)
    require_equal(errors, "intervention fingerprints", receipt.get("fingerprints"), FINGERPRINTS)
    require_equal(errors, "intervention next segment seed", receipt.get("next_segment_seed_input"), "decision_receipt_fingerprint")
    require_equal(errors, "intervention legacy v1 status", receipt.get("legacy_v1", {}).get("selection_status"), "legacy_no_effect")
    require_equal(errors, "intervention legacy retroactive effects", receipt.get("legacy_v1", {}).get("retroactive_effect_application"), "forbidden")

    handoff = value.get("runtime_handoff", {})
    require_equal(errors, "intervention runtime implementation status", handoff.get("implementation_complete"), False)


def validate(root: Path):
    errors = []
    data = {}
    for key, contract_id in EXPECTED.items():
        path = root / "contracts" / f"{key}.yml"
        if not path.exists():
            errors.append(f"missing contract: {path.relative_to(root)}")
            continue
        try:
            value = load(path)
        except Exception as exc:
            errors.append(f"{path.name}: invalid YAML or duplicate key: {exc}")
            continue
        data[key] = value
        if not isinstance(value, dict) or value.get("contract_id") != contract_id:
            errors.append(f"{path.name}: wrong contract_id")
        if isinstance(value, dict) and value.get("status") != "canonical":
            errors.append(f"{path.name}: status must be canonical")
        if isinstance(value, dict) and value.get("runtime_status") != "handoff_required":
            errors.append(f"{path.name}: runtime_status must remain handoff_required")

    schema_path = root / "schema/combat_intervention.schema.json"
    try:
        schema = json.loads(schema_path.read_text(encoding="utf-8"))
        if schema.get("properties", {}).get("contract_id", {}).get("const") != "combat_intervention_response":
            errors.append("combat_intervention.schema.json: wrong contract_id const")
    except Exception as exc:
        errors.append(f"combat_intervention.schema.json: invalid or missing JSON schema: {exc}")

    term = data.get("termination", {})
    order = term.get("priority_order", [])
    ids = [x.get("id") for x in order if isinstance(x, dict)]
    priorities = [x.get("priority") for x in order if isinstance(x, dict)]
    if ids != REQUIRED_TERMINATION:
        errors.append("termination: priority order mismatch")
    if len(priorities) != len(set(priorities)) or priorities != sorted(priorities):
        errors.append("termination: priorities must be unique and ascending")
    if term.get("tie_policy") != "validator_error":
        errors.append("termination: tie policy must be validator_error")
    if set(term.get("primitive_facts", [])) != REQUIRED_FACTS:
        errors.append("termination: primitive facts mismatch")
    mapping = term.get("objective_mapping", {})
    if set(mapping.get("allowed_result_kinds", [])) != REQUIRED_RESULTS:
        errors.append("termination: objective result enum mismatch")

    try:
        runtime_version = runtime_simulation_version()
    except Exception as exc:
        errors.append(f"simulation_version: {exc}")
        runtime_version = None
    sim = data.get("simulation_version", {})
    versions = sim.get("supported_versions", [])
    current = [
        x.get("version")
        for x in versions
        if isinstance(x, dict) and x.get("current_runtime_observed") is True
    ]
    if runtime_version is not None and current != [runtime_version]:
        errors.append(
            f"simulation_version: contract current {current!r} does not match code capability {runtime_version!r}"
        )
    if sim.get("unsupported_version") != "validator_error" or sim.get("missing_version") != "validator_error":
        errors.append("simulation_version: unsupported/missing versions must fail")
    if sim.get("fallback") != "forbidden":
        errors.append("simulation_version: fallback must be forbidden")

    identity = data.get("identity", {})
    if identity.get("fallback_order") != [
        "encounter_alias",
        "canonical_name",
        "declared_generic_role_label",
        "unknown_combatant",
    ]:
        errors.append("identity: fallback order mismatch")
    if identity.get("internal_id", {}).get("user_visible") != "forbidden":
        errors.append("identity: internal id must not be user-visible")

    logs = data.get("logs", {})
    if logs.get("group_key") != ["tick", "template_family", "actor_id", "target_id"]:
        errors.append("logs: unstable group key")
    if not {"terminal", "status", "objective"}.issubset(set(logs.get("non_groupable", []))):
        errors.append("logs: terminal/status/objective must be non-groupable")

    zones = data.get("tactical_zones", {})
    basis = zones.get("coordinate_basis", {})
    if basis.get("normalized_bounds") != [0.0, 1.0] or basis.get("raw_coordinates") != "debug_only":
        errors.append("tactical_zones: normalized/debug coordinate policy mismatch")
    layout = zones.get("default_layout", {})
    if layout.get("depth_boundaries") != [0.0, 0.33, 0.66, 1.0] or layout.get("lane_boundaries") != [0.0, 0.5, 1.0]:
        errors.append("tactical_zones: default boundaries mismatch")
    for axis in ("depth", "lane"):
        if "zones" in zones:
            problem = interval_error(zones["zones"], axis)
            if problem:
                errors.append(f"tactical_zones: {problem}")

    intervention = data.get("intervention")
    if isinstance(intervention, dict):
        validate_intervention(intervention, errors)
        if runtime_version is not None:
            require_equal(
                errors,
                "intervention observed runtime simulation version",
                intervention.get("runtime_handoff", {}).get("current_runtime_simulation_version"),
                runtime_version,
            )

    return errors, runtime_version


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, required=True)
    args = parser.parse_args()
    errors, runtime_version = validate(args.root.resolve())
    result = {
        "status": "PASS" if not errors else "FAIL",
        "errors": errors,
        "runtime_status": "handoff_required",
        "supported_simulation_version_observed": runtime_version,
    }
    print(json.dumps(result, ensure_ascii=False, indent=2))
    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())

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
ACTION_RECEIPT_STATUSES = ["applied", "already_applied", "pending_claim", "rejected"]
RECEIPT_REQUIRED_FIELDS = {
    "receipt_id",
    "pause_id",
    "evaluation_fingerprint",
    "application_transaction_id",
    "authored_response_id",
    "canonical_registry_ids",
    "resolved_executor_id",
    "resolved_target_ids",
    "resolved_strategy_scope_ids",
    "formula_receipt",
    "strategy_before_fingerprint",
    "strategy_after_fingerprint",
    "effect_ids",
    "outcome_actions",
    "action_receipts",
    "lifecycle_before",
    "lifecycle_after",
    "terminal_facts",
    "raw_log_references",
}
INTERVENTION_RAW_EVENTS = {
    "decision": "intervention_decision_committed",
    "strategy": "strategy_overlay_applied",
    "effect": "special_effect_resolved",
    "action": "outcome_action_applied",
    "entitlement": "loot_entitlement_created",
    "claim": "loot_claim_resolved",
}
SUPPORTED_SCHEMA_KEYWORDS = {
    "$schema", "$id", "$ref", "$defs", "title", "description", "type", "const",
    "enum", "required", "properties", "additionalProperties", "items", "minItems",
    "minProperties", "uniqueItems", "pattern", "allOf", "anyOf", "oneOf", "not",
    "contains",
}


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


def load_json_schema(path: Path):
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise ValueError("schema root must be an object")
    unsupported = []
    validate_schema_keywords(value, "$", unsupported)
    if unsupported:
        raise ValueError("unsupported schema keywords: " + "; ".join(unsupported))
    return value


def validate_schema_keywords(schema, path, errors):
    if not isinstance(schema, dict):
        errors.append(f"{path}: schema node must be an object")
        return
    for key in schema:
        if key not in SUPPORTED_SCHEMA_KEYWORDS:
            errors.append(f"{path}: unsupported keyword {key!r}")
    for container in ("properties", "$defs"):
        values = schema.get(container, {})
        if isinstance(values, dict):
            for key, child in values.items():
                validate_schema_keywords(child, f"{path}/{container}/{key}", errors)
    for container in ("allOf", "anyOf", "oneOf"):
        values = schema.get(container, [])
        if isinstance(values, list):
            for index, child in enumerate(values):
                validate_schema_keywords(child, f"{path}/{container}/{index}", errors)
    for key in ("items", "not", "contains"):
        child = schema.get(key)
        if isinstance(child, dict):
            validate_schema_keywords(child, f"{path}/{key}", errors)
    child = schema.get("additionalProperties")
    if isinstance(child, dict):
        validate_schema_keywords(child, f"{path}/additionalProperties", errors)


def resolve_local_ref(root_schema, ref):
    if not isinstance(ref, str) or not ref.startswith("#/"):
        raise ValueError(f"only local JSON Schema refs are supported: {ref!r}")
    value = root_schema
    for part in ref[2:].split("/"):
        key = part.replace("~1", "/").replace("~0", "~")
        if not isinstance(value, dict) or key not in value:
            raise ValueError(f"unresolved JSON Schema ref: {ref}")
        value = value[key]
    if not isinstance(value, dict):
        raise ValueError(f"JSON Schema ref does not resolve to an object: {ref}")
    return value


def schema_type_matches(value, expected):
    checks = {
        "object": lambda item: isinstance(item, dict),
        "array": lambda item: isinstance(item, list),
        "string": lambda item: isinstance(item, str),
        "integer": lambda item: isinstance(item, int) and not isinstance(item, bool),
        "number": lambda item: isinstance(item, (int, float)) and not isinstance(item, bool),
        "boolean": lambda item: isinstance(item, bool),
        "null": lambda item: item is None,
    }
    checker = checks.get(expected)
    if checker is None:
        raise ValueError(f"unsupported JSON Schema type: {expected!r}")
    return checker(value)


def validate_json_schema(instance, schema, root_schema=None, path="$"):
    root_schema = schema if root_schema is None else root_schema
    errors = []
    if "$ref" in schema:
        try:
            target = resolve_local_ref(root_schema, schema["$ref"])
        except ValueError as exc:
            return [f"{path}: {exc}"]
        return validate_json_schema(instance, target, root_schema, path)

    expected_type = schema.get("type")
    if expected_type is not None and not schema_type_matches(instance, expected_type):
        return [f"{path}: expected type {expected_type}, got {type(instance).__name__}"]
    if "const" in schema and instance != schema["const"]:
        errors.append(f"{path}: expected const {schema['const']!r}, got {instance!r}")
    if "enum" in schema and instance not in schema["enum"]:
        errors.append(f"{path}: value {instance!r} is not in enum {schema['enum']!r}")
    if isinstance(instance, str) and "pattern" in schema:
        if re.search(schema["pattern"], instance) is None:
            errors.append(f"{path}: value {instance!r} does not match {schema['pattern']!r}")
    if isinstance(instance, dict):
        for key in schema.get("required", []):
            if key not in instance:
                errors.append(f"{path}: missing required property {key!r}")
        properties = schema.get("properties", {})
        for key, value in instance.items():
            if key in properties:
                errors.extend(validate_json_schema(value, properties[key], root_schema, f"{path}.{key}"))
            elif schema.get("additionalProperties") is False:
                errors.append(f"{path}: unknown property {key!r}")
            elif isinstance(schema.get("additionalProperties"), dict):
                errors.extend(validate_json_schema(value, schema["additionalProperties"], root_schema, f"{path}.{key}"))
        if len(instance) < schema.get("minProperties", 0):
            errors.append(f"{path}: expected at least {schema['minProperties']} properties")
    if isinstance(instance, list):
        if len(instance) < schema.get("minItems", 0):
            errors.append(f"{path}: expected at least {schema['minItems']} items")
        if schema.get("uniqueItems"):
            fingerprints = [json.dumps(item, ensure_ascii=False, sort_keys=True) for item in instance]
            if len(fingerprints) != len(set(fingerprints)):
                errors.append(f"{path}: duplicate array items are forbidden")
        item_schema = schema.get("items")
        if isinstance(item_schema, dict):
            for index, value in enumerate(instance):
                errors.extend(validate_json_schema(value, item_schema, root_schema, f"{path}[{index}]"))
        contains = schema.get("contains")
        if isinstance(contains, dict) and not any(
            not validate_json_schema(value, contains, root_schema, f"{path}[{index}]")
            for index, value in enumerate(instance)
        ):
            errors.append(f"{path}: no item satisfies contains")
    for key in ("allOf",):
        for child in schema.get(key, []):
            errors.extend(validate_json_schema(instance, child, root_schema, path))
    for key in ("anyOf", "oneOf"):
        branches = schema.get(key)
        if branches:
            branch_results = [
                validate_json_schema(instance, child, root_schema, path)
                for child in branches
            ]
            valid_count = sum(not result for result in branch_results)
            if key == "anyOf" and valid_count == 0:
                errors.append(f"{path}: does not satisfy anyOf")
            if key == "oneOf" and valid_count != 1:
                errors.append(f"{path}: must satisfy exactly one oneOf branch, got {valid_count}")
                if valid_count == 0 and isinstance(instance, dict) and "kind" in instance:
                    for child, branch_errors in zip(branches, branch_results):
                        target = child
                        if "$ref" in child:
                            target = resolve_local_ref(root_schema, child["$ref"])
                        expected_kind = (
                            target.get("properties", {})
                            .get("kind", {})
                            .get("const")
                        )
                        if expected_kind == instance["kind"]:
                            errors.extend(branch_errors)
                            break
    if "not" in schema and not validate_json_schema(instance, schema["not"], root_schema, path):
        errors.append(f"{path}: matches forbidden schema")
    return errors


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
    require_equal(
        errors,
        "intervention preflight error result",
        response.get("preflight_error_result"),
        "reject_entire_response_transaction",
    )
    require_equal(
        errors,
        "intervention special effect failure classification",
        response.get("special_effect_failure_is_outcome"),
        True,
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
    require_equal(errors, "intervention preflight failure class", failure.get("failure_class"), "authoring_or_input_preflight_error")
    require_equal(errors, "intervention rollback scope", failure.get("rollback_scope"), "entire_response_transaction")
    require_equal(
        errors,
        "intervention resolved failure strategy",
        transaction.get("outcome_failure", {}).get("strategy_modifier"),
        "applied",
    )
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
    strategy_rule_ids = registry.get("strategy_targeting_rule_ids", [])
    if not strategy_rule_ids or len(strategy_rule_ids) != len(set(strategy_rule_ids)):
        errors.append("intervention: strategy targeting IDs must be present and unique")

    migration = value.get("legacy_migration", {})
    require_equal(errors, "intervention legacy migration boundary", migration.get("boundary"), "offline_or_load_time_migration_only")
    require_equal(errors, "intervention canonical legacy rejection", migration.get("canonical_runtime_authoring"), "reject_legacy_fields_and_aliases")
    require_equal(errors, "intervention ambiguous legacy conversion", migration.get("automatic_ambiguous_conversion"), "forbidden")
    require_equal(errors, "intervention ambiguous legacy result", migration.get("ambiguous_alias_result"), "designer_review_required")
    require_equal(errors, "intervention legacy reward migration", migration.get("retroactive_reward_or_effect"), "forbidden")

    special = value.get("special_effect", {})
    require_equal(errors, "intervention outcome action kinds", special.get("outcome_action_kinds"), ACTION_KINDS)
    require_equal(errors, "intervention action plan owner", special.get("action_plan_owner"), "combat_core")
    require_equal(errors, "intervention action application owner", special.get("application_owner"), "gamecore")
    action_fields = special.get("outcome_action_fields", {})
    require_equal(errors, "intervention set_flag authored fields", action_fields.get("set_flag"), ["flag_id"])
    require_equal(errors, "intervention loot authored fields", action_fields.get("create_loot_entitlement"), ["item_id", "source_selector_id", "claim_policy"])
    require_equal(errors, "intervention grant authored fields", action_fields.get("grant_item"), ["item_id"])
    exactly_once = special.get("exactly_once", {})
    require_equal(errors, "intervention action ID owner", exactly_once.get("id_generation_owner", {}).get("action_id"), "combat_core")
    require_equal(errors, "intervention response transaction ID owner", exactly_once.get("id_generation_owner", {}).get("response_application_transaction_id"), "gamecore")
    require_equal(errors, "intervention claim transaction ID owner", exactly_once.get("id_generation_owner", {}).get("claim_application_transaction_id"), "gamecore")
    require_equal(errors, "intervention retry action result", exactly_once.get("retry", {}).get("response_action"), "matching_action_id_returns_already_applied_without_mutation")
    require_equal(errors, "intervention entitlement retry", exactly_once.get("retry", {}).get("entitlement_creation"), "matching_entitlement_id_returns_existing_entitlement")
    require_equal(errors, "intervention claim retry", exactly_once.get("retry", {}).get("entitlement_claim"), "matching_claim_action_id_returns_already_applied_without_mutation")
    require_equal(errors, "intervention transaction ID separation", exactly_once.get("transaction_separation", {}).get("same_transaction_id"), "forbidden")
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
    require_equal(errors, "intervention terminal receipt schema", receipt.get("terminal_receipt_schema_version"), 1)
    require_equal(errors, "intervention receipt required fields", set(receipt.get("receipt_required_fields", [])), RECEIPT_REQUIRED_FIELDS)
    require_equal(errors, "intervention action receipt statuses", receipt.get("action_receipt_statuses"), ACTION_RECEIPT_STATUSES)
    require_equal(errors, "intervention fingerprints", receipt.get("fingerprints"), FINGERPRINTS)
    require_equal(errors, "intervention next segment seed", receipt.get("next_segment_seed_input"), "decision_receipt_fingerprint")
    require_equal(errors, "intervention legacy v1 status", receipt.get("legacy_v1", {}).get("selection_status"), "legacy_no_effect")
    require_equal(errors, "intervention legacy retroactive effects", receipt.get("legacy_v1", {}).get("retroactive_effect_application"), "forbidden")

    provenance = value.get("provenance_and_logs", {})
    require_equal(errors, "intervention receipt raw event policy", provenance.get("receipt_raw_event_policy"), "references_only_no_event_duplication")
    require_equal(errors, "intervention receipt entity references", provenance.get("receipt_entity_references"), "canonical_entity_ids_only")
    require_equal(errors, "intervention receipt zone references", provenance.get("receipt_zone_references"), "canonical_zone_ids_or_selector_provenance")
    require_equal(errors, "intervention receipt display labels", provenance.get("display_labels_in_receipt"), "forbidden")
    require_equal(errors, "intervention hidden state capture", provenance.get("hidden_state_capture"), "only_formula_inputs_and_fingerprints_required_for_replay")
    require_equal(errors, "intervention raw event types", provenance.get("raw_event_types"), INTERVENTION_RAW_EVENTS)

    handoff = value.get("runtime_handoff", {})
    require_equal(errors, "intervention runtime implementation status", handoff.get("implementation_complete"), False)


def validate(root: Path, authoring_payload: Path | None = None):
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

    schemas = {}
    for name in ("combat_intervention", "combat_intervention_response"):
        schema_path = root / f"schema/{name}.schema.json"
        try:
            schemas[name] = load_json_schema(schema_path)
        except Exception as exc:
            errors.append(f"{schema_path.name}: invalid or unsupported JSON schema: {exc}")

    intervention_schema = schemas.get("combat_intervention")
    intervention_value = data.get("intervention")
    if intervention_schema is not None and isinstance(intervention_value, dict):
        errors.extend(
            f"combat_intervention.schema.json: {message}"
            for message in validate_json_schema(intervention_value, intervention_schema)
        )

    if authoring_payload is not None:
        response_schema = schemas.get("combat_intervention_response")
        if response_schema is None:
            errors.append("authoring payload cannot be validated without combat_intervention_response.schema.json")
        else:
            try:
                payload = load(authoring_payload)
            except Exception as exc:
                errors.append(f"authoring payload: invalid YAML or duplicate key: {exc}")
            else:
                errors.extend(
                    f"authoring payload schema: {message}"
                    for message in validate_json_schema(payload, response_schema)
                )

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
    require_equal(errors, "logs intervention raw event map", logs.get("intervention_raw_events"), INTERVENTION_RAW_EVENTS)
    if not set(INTERVENTION_RAW_EVENTS.values()).issubset(set(logs.get("non_groupable", []))):
        errors.append("logs: intervention events must be non-groupable")
    require_equal(
        errors,
        "logs receipt reference policy",
        logs.get("receipt_policy"),
        "references_sequence_range_and_fingerprint_without_copying_raw_events",
    )

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

    return errors, runtime_version, "combat_intervention_response" in schemas


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--authoring-payload", type=Path)
    args = parser.parse_args()
    errors, runtime_version, authoring_schema_loaded = validate(
        args.root.resolve(),
        args.authoring_payload.resolve() if args.authoring_payload else None,
    )
    result = {
        "status": "PASS" if not errors else "FAIL",
        "errors": errors,
        "runtime_status": "handoff_required",
        "supported_simulation_version_observed": runtime_version,
        "schema_engine": "builtin_strict_subset",
        "authoring_payload_schema_loaded": authoring_schema_loaded,
        "authoring_payload_instance_validation": (
            "validated" if args.authoring_payload is not None else "not_requested"
        ),
    }
    print(json.dumps(result, ensure_ascii=False, indent=2))
    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())

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
EXPECTED_EXECUTOR_IDS = [
    "combat.selector.executor.v1.observer",
    "combat.selector.executor.v1.any_capable",
]
EXPECTED_TARGET_IDS = [
    "combat.selector.target.v1.executor_self",
    "combat.selector.target.v1.selected_target",
    "combat.selector.target.v1.nearest_active_enemy",
    "combat.selector.target.v1.lowest_health_active_ally",
    "combat.selector.target.v1.surrounded_active_ally",
    "combat.selector.target.v1.all_active_allies",
]
EXPECTED_FORMULA_IDS = ["combat.formula.v1.fixed_chance"]
EXPECTED_STRATEGY_RULE_IDS = [
    "combat.strategy.targeting.v1.attackers_of",
    "combat.strategy.targeting.v1.rearmost_active_enemy",
    "combat.strategy.targeting.v1.focus_resolved_target",
]
EXPECTED_OPPORTUNITY_PROVENANCE_FIELDS = [
    "bound_target_ids",
    "bound_target_tick",
    "bound_target_state_at_tick_start",
    "trigger_tick",
]
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
RECEIPT_FORMULA_FIELDS = [
    "formula_id",
    "normalized_formula_parameters",
    "input_fingerprint",
    "rng_namespace",
    "rng_draw_index",
    "roll",
    "outcome",
]
INTERVENTION_RAW_EVENTS = {
    "decision": "intervention_decision_committed",
    "strategy": "strategy_overlay_applied",
    "effect": "special_effect_resolved",
    "action": "outcome_action_applied",
    "entitlement": "loot_entitlement_created",
    "claim": "loot_claim_resolved",
}
I7_DTO_NAMES = [
    "CombatInterventionResponseInput",
    "CombatInterventionResponsePlan",
    "CombatInterventionCommitResult",
    "CombatLootClaimInput",
    "CombatLootClaimResult",
    "CombatLootEntitlement",
]
I7_STATUS_NAMES = ["applied", "already_applied", "pending_claim", "rejected"]
I7_DTO_FIELDS = {
    "input": ["pause_id", "evaluation_fingerprint", "authored_response_id", "payload"],
    "plan": ["response_application_transaction_id", "pause_id", "evaluation_fingerprint", "precondition_game_state_fingerprint", "resolved_executor_id", "resolved_target_ids", "resolved_outcome", "strategy_overlay_plan", "effect_application_plan", "outcome_action_plan", "formula_receipt", "decision_receipt_draft", "deterministic_next_segment_seed", "provenance"],
    "output": ["status", "decision_receipt", "action_receipts", "next_segment_seed"],
    "claim_input": ["entitlement_id", "terminal_facts_fingerprint"],
    "claim_output": ["status", "claim_receipt", "inventory_delta"],
    "entitlement": ["entitlement_id", "item_id", "source_ids", "source_provenance", "claim_policy", "origin_combat_id", "origin_pause_id", "origin_decision_receipt_fingerprint", "status"],
}
SUPPORTED_SCHEMA_KEYWORDS = {
    "$schema", "$id", "$ref", "$defs", "title", "description", "type", "const",
    "enum", "required", "properties", "additionalProperties", "items", "minItems",
    "minProperties", "uniqueItems", "pattern", "allOf", "anyOf", "oneOf", "not",
    "contains", "minimum", "maximum", "minLength", "maxItems",
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
    if isinstance(instance, str) and "minLength" in schema and len(instance) < schema["minLength"]:
        errors.append(f"{path}: string length {len(instance)} is less than minLength {schema['minLength']}")
    if isinstance(instance, (int, float)) and not isinstance(instance, bool):
        if "minimum" in schema and instance < schema["minimum"]:
            errors.append(f"{path}: value {instance!r} is less than minimum {schema['minimum']!r}")
        if "maximum" in schema and instance > schema["maximum"]:
            errors.append(f"{path}: value {instance!r} is greater than maximum {schema['maximum']!r}")
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
        if "maxItems" in schema and len(instance) > schema["maxItems"]:
            errors.append(f"{path}: expected at most {schema['maxItems']} items")
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


def require_membership(errors, label, value, allowed):
    if value not in allowed:
        errors.append(f"{label}: unknown canonical registry ID {value!r}")


def validate_authoring_membership(payload, registry, errors):
    """Patterns in the JSON schema are supplemented by closed registry checks."""
    selectors = registry.get("selector_ids", {})
    executor_ids = selectors.get("executor", [])
    target_ids = selectors.get("target", [])
    strategy_ids = registry.get("strategy_targeting_rule_ids", [])

    effect = payload.get("special_effect")
    if isinstance(effect, dict):
        require_membership(errors, "authoring executor_selector_id", effect.get("executor_selector_id"), executor_ids)
        require_membership(errors, "authoring target_selector_id", effect.get("target_selector_id"), target_ids)
        for branch_name in ("success", "failure"):
            branch = effect.get(branch_name, {})
            for action in branch.get("outcome_actions", []):
                if not isinstance(action, dict):
                    continue
                if action.get("kind") == "create_loot_entitlement":
                    require_membership(errors, "authoring loot source_selector_id", action.get("source_selector_id"), target_ids)

    strategy = payload.get("strategy_modifier")
    if isinstance(strategy, dict):
        scope = strategy.get("scope", {})
        if scope.get("kind") == "combatants":
            for selector_id in scope.get("combatant_selector_ids", []):
                require_membership(errors, "authoring combatant scope selector ID", selector_id, target_ids)
        for operation in strategy.get("operations", []):
            if isinstance(operation, dict) and operation.get("kind") == "set_targeting_rule":
                require_membership(errors, "authoring strategy targeting rule ID", operation.get("rule_id"), strategy_ids)


def validate_non_empty_authored_ids(value, errors, path="$"):
    if isinstance(value, dict):
        for key, child in value.items():
            child_path = f"{path}.{key}"
            if key.endswith("_id") and isinstance(child, str) and not child:
                errors.append(f"authoring payload schema: {child_path}: ID must not be empty")
            validate_non_empty_authored_ids(child, errors, child_path)
    elif isinstance(value, list):
        for index, child in enumerate(value):
            validate_non_empty_authored_ids(child, errors, f"{path}[{index}]")


def validate_effect_target_preflight(value, errors):
    # Boundary: this validator receives only static authored payloads.  Until
    # intervention.yml supplies a closed CombatEffectCatalog, it must enforce
    # the declared two-branch preflight policy but must not infer effect/target
    # compatibility from authored effect_ids or runtime state.
    compatibility = value.get("canonical_semantics", {}).get("effect_target_compatibility", {})
    require_equal(errors, "effect-target compatibility contract", compatibility, {
        "source_registry": "pause_snapshot_bound_CombatEffectCatalog.effects",
        "branches": ["success", "failure"],
        "effect_ids": "all_authored_effect_ids_in_both_branches",
        "unknown_effect_id": "preflight_error",
        "definition_target_selector": "canonical_target_selector_id_required",
        "compatibility_rule": "effect_target_selector_exactly_matches_response_special_effect_target_selector_id",
        "validate_before_rng": ["success_branch", "failure_branch"],
        "incompatibility": "preflight_error",
        "rejection_mutations": {"state": 0, "cost": 0, "rng": 0, "history": 0},
        "static_authoring_validator_without_catalog": "do_not_guess_combinations",
        "runtime_preflight": "mandatory_machine_contract",
    })


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
    require_equal(errors, "intervention executor registry membership", selectors.get("executor"), EXPECTED_EXECUTOR_IDS)
    require_equal(errors, "intervention target registry membership", selectors.get("target"), EXPECTED_TARGET_IDS)
    if len(selectors.get("executor", [])) != len(set(selectors.get("executor", []))) or len(selectors.get("target", [])) != len(set(selectors.get("target", []))):
        errors.append("intervention registry: duplicate array items")
    canonical_ids = list(selectors.get("executor", [])) + list(selectors.get("target", []))
    if not canonical_ids or len(canonical_ids) != len(set(canonical_ids)):
        errors.append("intervention: selector IDs must be present and unique")
    for selector_id in canonical_ids:
        if selector_id in LEGACY_SELECTOR_ALIASES or not selector_id.startswith("combat.selector.") or ".v1." not in selector_id:
            errors.append(f"intervention: legacy or non-canonical selector ID: {selector_id}")
    formula_ids = registry.get("formula_ids", [])
    require_equal(errors, "intervention formula registry membership", formula_ids, EXPECTED_FORMULA_IDS)
    if len(formula_ids) != len(set(formula_ids)):
        errors.append("intervention registry: duplicate array items")
    if not formula_ids or len(formula_ids) != len(set(formula_ids)):
        errors.append("intervention: formula IDs must be present and unique")
    for formula_id in formula_ids:
        if not formula_id.startswith("combat.formula.v1."):
            errors.append(f"intervention: non-canonical formula ID: {formula_id}")
    strategy_rule_ids = registry.get("strategy_targeting_rule_ids", [])
    require_equal(errors, "intervention strategy-rule registry membership", strategy_rule_ids, EXPECTED_STRATEGY_RULE_IDS)
    if len(strategy_rule_ids) != len(set(strategy_rule_ids)):
        errors.append("intervention registry: duplicate array items")
    if not strategy_rule_ids or len(strategy_rule_ids) != len(set(strategy_rule_ids)):
        errors.append("intervention: strategy targeting IDs must be present and unique")
    fixed_chance = registry.get("formula_definitions", {}).get("combat.formula.v1.fixed_chance", {})
    require_equal(errors, "fixed chance parameters", fixed_chance.get("parameters", {}).get("required"), ["chance_percent"])
    require_equal(errors, "fixed chance authored inputs", fixed_chance.get("authored_inputs_only"), True)
    require_equal(errors, "fixed chance modifiers", fixed_chance.get("modifiers"), "forbidden")
    require_equal(errors, "fixed chance rounding", fixed_chance.get("rounding"), "forbidden")
    require_equal(errors, "fixed chance clamp", fixed_chance.get("clamp"), "forbidden")
    require_equal(errors, "fixed chance roll", fixed_chance.get("roll"), "integer_range_0_99")
    require_equal(errors, "fixed chance success", fixed_chance.get("success"), "roll_less_than_chance_percent")
    selector_definitions = registry.get("selector_definitions", {})
    nearest = selector_definitions.get("nearest_active_enemy", {})
    require_equal(errors, "nearest enemy distance metric", nearest.get("ordering"), "minimum_occupied_footprint_hex_distance_between_executor_and_candidate_ascending_then_stable_id")
    surrounded = selector_definitions.get("surrounded_active_ally", {})
    require_equal(errors, "surrounded ally candidate set", surrounded.get("candidate_set"), "active_same_side_combatants_including_executor")
    require_equal(errors, "surrounded ally self occupancy", surrounded.get("candidate_self_occupancy"), "entire_candidate_footprint_excluded_from_ally_occupancy_count")
    require_equal(errors, "surrounded ally predicate", surrounded.get("predicate"), "distinct_enemy_occupied_hexes_at_least_3_and_distinct_ally_occupied_hexes_zero")
    require_equal(errors, "surrounded ally neighborhood", surrounded.get("neighborhood"), "six_adjacent_hexes")
    require_equal(errors, "surrounded ally exclusions", surrounded.get("exclusions"), ["ko", "departed", "captured"])
    require_equal(errors, "surrounded ally ordering", surrounded.get("ordering"), "enemy_count_descending_then_stable_id")
    lowest = selector_definitions.get("lowest_health_active_ally", {})
    require_equal(errors, "lowest health candidate set", lowest.get("candidate_set"), "active_same_side_combatants_including_executor")
    require_equal(errors, "lowest health ordering", lowest.get("ordering"), "exact_integer_rational_cross_multiplication_current_hp_over_max_hp_ascending_then_stable_id")
    all_allies = selector_definitions.get("all_active_allies", {})
    require_equal(errors, "all allies candidate set", all_allies.get("candidate_set"), "active_same_side_combatants_including_executor")
    require_equal(errors, "all allies ordering", all_allies.get("ordering"), "stable_id_ascending")
    require_equal(errors, "all allies multi-target", all_allies.get("multi_target"), True)

    provenance = registry.get("opportunity_provenance", {})
    require_equal(errors, "selected target provenance fields", provenance.get("required_fields"), EXPECTED_OPPORTUNITY_PROVENANCE_FIELDS)
    require_equal(errors, "selected target provenance source", provenance.get("source"), "pause_snapshot")
    require_equal(errors, "selected target bound IDs", provenance.get("bound_target_ids"), "immutable_single_bound_target_ids_or_empty")
    selected = selector_definitions.get("selected_target", {})
    require_equal(errors, "selected target resolution", selected.get("resolves"), "exactly_one_immutable_bound_target; active_at_pause_or_active_at_trigger_tick_start_then_ko_same_tick_only")
    require_equal(errors, "selected target AI fallback", selected.get("ai_current_target_fallback"), "forbidden")
    executor_ids = selectors.get("executor", [])
    require_equal(errors, "executor observer rule", executor_ids[0] if len(executor_ids) > 0 else None, "combat.selector.executor.v1.observer")
    require_equal(errors, "executor any-capable rule", executor_ids[1] if len(executor_ids) > 1 else None, "combat.selector.executor.v1.any_capable")
    require_equal(errors, "executor tie-break", registry.get("tie_break"), "stable_combatant_id_ascending")

    migration = value.get("legacy_migration", {})
    require_equal(errors, "intervention legacy migration boundary", migration.get("boundary"), "offline_or_load_time_migration_only")
    require_equal(errors, "intervention canonical legacy rejection", migration.get("canonical_runtime_authoring"), "reject_legacy_fields_and_aliases")
    require_equal(errors, "intervention ambiguous legacy conversion", migration.get("automatic_ambiguous_conversion"), "forbidden")
    require_equal(errors, "intervention ambiguous legacy result", migration.get("ambiguous_alias_result"), "designer_review_required")
    require_equal(errors, "intervention legacy reward migration", migration.get("retroactive_reward_or_effect"), "forbidden")

    special = value.get("special_effect", {})
    require_equal(errors, "intervention outcome action kinds", special.get("outcome_action_kinds"), ACTION_KINDS)
    require_equal(errors, "intervention action plan owner", special.get("action_plan_owner"), "combat_core")
    require_equal(errors, "intervention action application owner", special.get("application_owner"), "combat_intervention_transaction")
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
    require_equal(errors, "intervention strategy precedence", strategy.get("precedence"), ["combatant", "role", "all_allies_side", "baseline"])
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
    require_equal(errors, "intervention receipt optional fields", receipt.get("receipt_optional_fields"), ["formula_receipt"])
    require_equal(errors, "intervention formula receipt fields", receipt.get("formula_receipt_fields"), RECEIPT_FORMULA_FIELDS)
    require_equal(errors, "intervention strategy-only formula receipt", receipt.get("formula_receipt_rules", {}).get("strategy_only"), "absent")
    require_equal(errors, "intervention special-effect formula receipt", receipt.get("formula_receipt_rules", {}).get("special_effect_present"), "required")
    require_equal(errors, "intervention action receipt statuses", receipt.get("action_receipt_statuses"), ACTION_RECEIPT_STATUSES)
    require_equal(errors, "intervention fingerprints", receipt.get("fingerprints"), FINGERPRINTS)
    require_equal(errors, "intervention next segment seed", receipt.get("next_segment_seed_input"), "decision_receipt_fingerprint")
    require_equal(errors, "intervention legacy v1 status", receipt.get("legacy_v1", {}).get("selection_status"), "legacy_no_effect")
    require_equal(errors, "intervention legacy retroactive effects", receipt.get("legacy_v1", {}).get("retroactive_effect_application"), "forbidden")

    encoding = value.get("transaction", {}).get("probabilistic_rng", {}).get("canonical_encoding", {})
    require_equal(errors, "v3 RNG canonical serialization", encoding.get("serialization"), "utf_8_canonical_json_array")
    require_equal(errors, "RNG JSON whitespace", encoding.get("whitespace"), "none")
    require_equal(errors, "RNG JSON object key order", encoding.get("object_key_order"), "lexicographic")
    require_equal(errors, "RNG JSON integer encoding", encoding.get("integer_encoding"), "decimal")
    require_equal(errors, "RNG formula parameter key order", encoding.get("normalized_formula_parameters_key_order"), "lexicographic")
    require_equal(errors, "RNG sub-seed derivation owner", encoding.get("sub_seed_derivation_hash"), "fnv_1a_64_current_v3")
    require_equal(errors, "RNG sub-seed input order", value.get("transaction", {}).get("probabilistic_rng", {}).get("canonical_sub_seed_inputs"), [
        "formula_semantic_tag", "rng_namespace", "effective_segment_seed", "simulation_version", "manifest_fingerprint", "segment_index", "pause_tick", "pause_id", "evaluation_fingerprint", "authored_response_id", "formula_id", "normalized_formula_parameters", "resolved_executor_id", "canonical_ordered_target_ids"
    ])
    require_equal(errors, "RNG draw index", value.get("transaction", {}).get("probabilistic_rng", {}).get("draw_index"), 0)

    exactly_once = special.get("exactly_once", {})
    require_equal(errors, "exactly-once action ID inputs", exactly_once.get("deterministic_inputs", {}).get("action_id"), ["decision_receipt_fingerprint", "outcome_branch", "action_index"])
    require_equal(errors, "exactly-once entitlement ID inputs", exactly_once.get("deterministic_inputs", {}).get("entitlement_id"), ["action_id"])
    require_equal(errors, "exactly-once transaction ID separation", exactly_once.get("transaction_separation", {}).get("same_transaction_id"), "forbidden")
    require_equal(errors, "loot entitlement creation timing", loot.get("timing"), "create_on_outcome_claim_on_terminal")
    require_equal(errors, "direct grant atomic timing", special.get("direct_grant", {}).get("timing"), "immediate_atomic_commit")

    provenance = value.get("provenance_and_logs", {})
    require_equal(errors, "intervention receipt raw event policy", provenance.get("receipt_raw_event_policy"), "references_only_no_event_duplication")
    require_equal(errors, "intervention receipt entity references", provenance.get("receipt_entity_references"), "canonical_entity_ids_only")
    require_equal(errors, "intervention receipt zone references", provenance.get("receipt_zone_references"), "canonical_zone_ids_or_selector_provenance")
    require_equal(errors, "intervention receipt display labels", provenance.get("display_labels_in_receipt"), "forbidden")
    require_equal(errors, "intervention hidden state capture", provenance.get("hidden_state_capture"), "only_formula_inputs_and_fingerprints_required_for_replay")
    require_equal(errors, "intervention raw event types", provenance.get("raw_event_types"), INTERVENTION_RAW_EVENTS)

    handoff = value.get("runtime_handoff", {})
    require_equal(errors, "intervention runtime implementation status", handoff.get("implementation_complete"), False)


def validate_i7_contract(value, errors):
    contract = value.get("i7_contract", {})
    require_equal(errors, "I7 decision status", contract.get("decision_status"), "all_open_questions_closed")
    require_equal(errors, "I7 response plan owner module", contract.get("owner_modules", {}).get("response_plan", {}).get("existing_owner_module"), "crates/escape-core/src/combat_runtime.rs")
    require_equal(errors, "I7 atomic mutation owner module", contract.get("owner_modules", {}).get("atomic_mutation", {}).get("module"), "crates/escape-core/src/combat_intervention_transaction.rs")
    require_equal(errors, "I7 atomic mutation module status", contract.get("owner_modules", {}).get("atomic_mutation", {}).get("status"), "new_required_module")
    require_equal(errors, "I7 entry boundary module", contract.get("owner_modules", {}).get("entry_boundary", {}).get("existing_owner_module"), "crates/escape-core/src/lib.rs")
    require_equal(errors, "I7 persistence owner module", contract.get("owner_modules", {}).get("persistence_boundary", {}).get("existing_owner_module"), "crates/escape-core/src/save.rs")

    dto = contract.get("dto_contract", {})
    actual_dto_names = []
    for key in ("input", "plan", "output", "claim_input", "claim_output", "entitlement"):
        item = dto.get(key, {})
        actual_dto_names.append(item.get("name"))
        require_equal(errors, f"I7 {key} DTO status", item.get("status"), "new_required_type")
        require_equal(errors, f"I7 {key} DTO fields", item.get("fields"), I7_DTO_FIELDS[key])
    require_equal(errors, "I7 DTO names", actual_dto_names, I7_DTO_NAMES)
    plan_boundary = contract.get("plan_boundary", {})
    require_equal(errors, "I7 plan draft input-only", plan_boundary.get("input_only_plan_fields"), ["decision_receipt_draft"])
    require_equal(errors, "I7 plan draft candidate use", plan_boundary.get("candidate_use"), "finalize_decision_receipt_before_swap")
    require_equal(errors, "I7 plan draft persistence exclusion", plan_boundary.get("never_persisted_to"), ["GameState.combat_intervention_ledger", "SaveEnvelope.combat_checkpoint"])
    require_equal(errors, "I7 plan durable replacement", plan_boundary.get("durable_replacement"), "decision_receipt")

    actions = contract.get("action_semantics", {})
    require_equal(errors, "I7 set_flag semantics", actions.get("set_flag"), "idempotent_set_existing_or_new_flag")
    require_equal(errors, "I7 grant_item semantics", actions.get("grant_item"), "exact_once_direct_inventory_grant")
    require_equal(errors, "I7 entitlement semantics", actions.get("create_loot_entitlement"), "create_unclaimed_entitlement_without_inventory_mutation")
    require_equal(errors, "I7 action plan owner", actions.get("action_plan_owner"), "combat_core")
    require_equal(errors, "I7 atomic commit owner", actions.get("atomic_commit_owner"), "combat_intervention_transaction")
    require_equal(errors, "I7 fully resolved plan", actions.get("fully_resolved_plan"), "GameCore_transaction_consumes_plan_without_selector_formula_branch_or_target_re_evaluation")
    require_equal(errors, "I7 stale precondition", actions.get("stale_precondition"), "precondition_game_state_fingerprint_mismatch_rejected_before_candidate_swap")

    timing = contract.get("receipt_timing", {})
    preflight = timing.get("preflight_receipt", {})
    resolved = timing.get("resolved_decision_receipt", {})
    require_equal(errors, "I7 preflight receipt storage", preflight.get("status"), "transient_active_combat_session_only")
    require_equal(errors, "I7 preflight receipt timing", preflight.get("timing"), "immediately_after_successful_preflight_before_gamestate_mutation")
    require_equal(errors, "I7 resolved receipt storage", resolved.get("status"), "durable_after_successful_atomic_commit")
    require_equal(errors, "I7 resolved receipt timing", resolved.get("timing"), "finalized_with_action_receipts_and_cached_commit_result_in_candidate_before_swap_becomes_durable_with_atomic_swap_before_next_segment_continuation")

    durable = contract.get("durable_state", {})
    ledger = durable.get("ledger", {})
    require_equal(errors, "I7 ledger type", ledger.get("name"), "CombatInterventionLedger")
    require_equal(errors, "I7 ledger location", ledger.get("location"), "GameState.combat_intervention_ledger")
    require_equal(errors, "I7 durable ledger fields", ledger.get("fields"), ["committed_response_results_by_transaction_id", "applied_action_ids", "unresolved_loot_entitlements_by_id", "loot_claim_receipts_by_action_id", "applied_claim_action_ids"])
    require_equal(errors, "I7 cross-session durable location", durable.get("cross_session_durable"), "GameState.combat_intervention_ledger")
    require_equal(errors, "I7 active checkpoint location", durable.get("active_session_checkpoint"), "SaveEnvelope.combat_checkpoint")
    require_equal(errors, "I7 checkpoint serialization", durable.get("checkpoint_serialization"), "allowed_for_paused_session_restart")
    require_equal(errors, "I7 checkpoint disposal", durable.get("checkpoint_disposal"), "terminal_or_forced_stop_discarded_and_never_promoted_to_ledger")
    forbidden = set(durable.get("forbidden_serialized_fields", []))
    if not {"transaction_scratch", "partial_working_copy", "renderer_state"}.issubset(forbidden):
        errors.append("I7 forbidden serialized transient fields are incomplete")

    save_boundary = contract.get("save_version_boundary", {})
    require_equal(errors, "I7 observed SaveEnvelope schema", save_boundary.get("observed_current_save_schema_version"), 1)
    require_equal(errors, "I7 observed SaveEnvelope type", save_boundary.get("observed_current_save_type"), "SaveEnvelope")
    require_equal(errors, "I7 target save schema", save_boundary.get("i7b_target_save_schema_version"), 2)
    require_equal(errors, "I7 v1 backward load", save_boundary.get("v1_backward_load"), "missing_combat_intervention_ledger_defaults_to_empty_CombatInterventionLedger")
    require_equal(errors, "I7 migration boundary", save_boundary.get("migration_boundary"), "load_time_only")
    require_equal(errors, "I7 migration scope", save_boundary.get("migration_scaffolding"), "only_empty_defaults_and_one_version_gate")
    require_equal(errors, "I7 checkpoint current schema", save_boundary.get("checkpoint_schema_version_current"), 1)
    require_equal(errors, "I7 checkpoint target schema", save_boundary.get("checkpoint_schema_version_i2b_i7a_target"), 2)
    require_equal(errors, "I7 checkpoint v1 provenance policy", save_boundary.get("checkpoint_v1_provenance_missing"), "explicit_reject")

    statuses = contract.get("transaction_statuses", {})
    for name in I7_STATUS_NAMES:
        if not isinstance(statuses.get(name), str) or not statuses[name].strip():
            errors.append(f"I7 transaction status missing: {name}")
    require_equal(errors, "I7 partial status", statuses.get("partial"), "forbidden")
    require_equal(errors, "I7 response statuses", contract.get("status_surface", {}).get("response_result"), ["applied", "already_applied", "rejected"])
    require_equal(errors, "I7 action receipt statuses", contract.get("status_surface", {}).get("outcome_action_receipt"), ["applied", "already_applied", "pending_claim"])
    require_equal(errors, "I7 claim result statuses", contract.get("status_surface", {}).get("claim_result"), ["applied", "already_applied", "rejected"])
    require_equal(errors, "I7 terminal policy rejection", contract.get("status_surface", {}).get("terminal_policy_reject"), "inventory_grant_zero_durable_denial_receipt_unresolved_entitlement_removed_claim_idempotency_recorded")

    rollback = contract.get("rollback_contract", {})
    require_equal(errors, "I7 rollback preflight order", rollback.get("preflight_order"), ["validation", "selector_resolution", "formula_resolution", "both_branch_effect_target_compatibility", "precondition_game_state_fingerprint"])
    require_equal(errors, "I7 rollback candidate", rollback.get("candidate_state"), "clone_GameState_and_ledger_apply_entire_plan")
    require_equal(errors, "I7 rollback commit", rollback.get("commit"), "swap_candidate_only_after_all_operations_and_receipts_succeed")
    require_equal(errors, "I7 rollback failure", rollback.get("failure"), "discard_candidate_original_state_ledger_history_cost_unchanged_pause_retained")
    require_equal(errors, "I7 rollback partial apply", rollback.get("partial_apply"), "forbidden")
    require_equal(errors, "I7 stale rollback", rollback.get("stale_precondition"), "response_result_rejected_zero_candidate_swap_ledger_history_cost_mutation_pause_retained")
    require_equal(errors, "I7 no rejudgement", rollback.get("no_rejudgement"), "GameCore_transaction_does_not_re_evaluate_selector_formula_branch_or_target")
    receipt_seed = contract.get("receipt_seed_order", {})
    require_equal(errors, "I7 candidate receipt seed order", receipt_seed.get("candidate_contains_before_swap"), ["decision_receipt", "action_receipts", "committed_response_results_by_transaction_id", "deterministic_next_segment_seed"])
    require_equal(errors, "I7 receipt seed cache", receipt_seed.get("next_segment_seed"), "plan_deterministic_next_segment_seed_copied_to_commit_result_and_cache")
    require_equal(errors, "I7 transaction retry equality", receipt_seed.get("retry_result"), "cached_CombatInterventionCommitResult_equality")
    require_equal(errors, "I7 lifecycle receipt running paused", contract.get("lifecycle_receipts", {}).get("running_to_paused"), "preflight_context_is_active_session_only")
    require_equal(errors, "I7 lifecycle receipt paused running", contract.get("lifecycle_receipts", {}).get("paused_to_running"), "resolved_decision_receipt_written_only_after_commit")
    require_equal(errors, "I7 lifecycle receipt paused terminal", contract.get("lifecycle_receipts", {}).get("paused_to_terminal"), "terminal_claim_is_separate_transaction")
    require_equal(errors, "I7 lifecycle receipt forced stop", contract.get("lifecycle_receipts", {}).get("forced_stop"), "invalidates_pause_and_rejects_pending_response")
    require_equal(errors, "I7 handoff next action", contract.get("handoff_next_action"), "implement_I2b_then_I7a_with_new_transaction_module_and_ledger_types_and_no_runtime_changes_in_this_design_slice")

    work_packages = contract.get("work_packages", [])
    required_work_package_tests = {
        "I2b": {"same_tick_ko_acceptance", "stable_id_order_independence", "rng_tuple_replay", "both_branch_target_compatibility_preflight"},
        "I7a": {"preflight_zero_mutation", "transaction_retry_equal_result", "action_collision_rollback", "same_item_distinct_actions"},
        "I7b": {"restart_exactly_once", "claim_denial_retry", "checkpoint_v1_policy"},
        "I7c": {"same_tick_ko_intervention_before_settlement", "stale_response", "terminal_claim_policy"},
    }
    for item in work_packages:
        expected_tests = required_work_package_tests.get(item.get("id"), set())
        if not expected_tests.issubset(set(item.get("tests", []))):
            errors.append(f"I7 {item.get('id')} core acceptance tests are incomplete")

    require_equal(errors, "I7 I2b status", contract.get("runtime_delta_i2b", {}).get("status"), "separate_prerequisite_work_package")
    required_i2b = set(contract.get("runtime_delta_i2b", {}).get("required", []))
    if not {"same_tick_ko_provenance", "any_capable_stable_id", "rng_semantic_domain_tuple", "effect_target_selector_canonicalization_and_source_provenance"}.issubset(required_i2b):
        errors.append("I7 I2b runtime delta is incomplete")
    require_equal(errors, "I7 work package order", [item.get("id") for item in work_packages], ["I2b", "I7a", "I7b", "I7c"])
    for item in work_packages:
        for field in ("inputs", "outputs", "tests"):
            if not item.get(field):
                errors.append(f"I7 {item.get('id')} {field} must be explicit")
    require_equal(errors, "I7 implementation order", contract.get("implementation_order"), ["I2b", "I7a", "I7b", "I7c"])

def validate_canonical_semantics(value, errors):
    semantics = value.get("canonical_semantics", {})
    selected = semantics.get("selected_target", {})
    require_equal(errors, "canonical selected target cardinality", selected.get("cardinality"), "exactly_one")
    require_equal(errors, "canonical selected target binding", selected.get("binding"), "immutable_bound_target_from_pause_opportunity_provenance")
    require_equal(errors, "canonical selected target accepted states", selected.get("accepted_when"), ["active_at_pause_snapshot", "active_at_triggering_tick_start_and_ko_during_that_same_tick"])
    require_equal(errors, "canonical selected target rejected states", selected.get("rejected_when"), ["previously_ko", "fled", "departed", "captured"])
    require_equal(errors, "canonical selected target provenance fields", selected.get("provenance_fields"), ["bound_target_ids", "bound_target_tick", "bound_target_state_at_tick_start", "trigger_tick"])
    require_equal(errors, "canonical selected target resolution", selected.get("resolution"), "no_re_evaluation_against_later_state_within_atomic_transaction")
    if "departed" in selected:
        require_equal(errors, "canonical selected target departed definition", selected.get("departed"), "rejected")
    if "departed_meaning" in selected:
        require_equal(errors, "canonical selected target departed definition", selected.get("departed_meaning"), "non_flee_departure")

    executor = semantics.get("executor", {})
    observer = executor.get("observer", {})
    require_equal(errors, "canonical observer opportunity", observer.get("opportunity_observer"), "exact")
    require_equal(errors, "canonical observer requirements", observer.get("requires"), ["can_act", "required_capabilities"])
    require_equal(errors, "canonical observer mapping", observer.get("mapping"), "exactly_one_active_combatant_with_hp_greater_than_zero")
    require_equal(errors, "canonical observer ambiguity", observer.get("otherwise"), "preflight_error")
    capable = executor.get("any_capable", {})
    require_equal(errors, "canonical any-capable selection", capable.get("selection"), "stable_combatant_id_ascending_first")
    if "mapping" in capable:
        require_equal(errors, "canonical any-capable mapping", capable.get("mapping"), "exactly_one_active_combatant_with_hp_greater_than_zero")
    if "candidate_mapping" in capable:
        require_equal(errors, "canonical any-capable candidate mapping", capable.get("candidate_mapping"), "exactly_one_active_combatant_with_hp_greater_than_zero_per_candidate")
    require_equal(errors, "canonical any-capable zero result", capable.get("zero_candidates"), "preflight_error")
    require_equal(errors, "canonical any-capable ordering source", capable.get("ordering_source"), "stable_combatant_id_not_vector_or_insertion_order")

    multi = semantics.get("multi_target", {})
    require_equal(errors, "canonical multi-target ordering", multi.get("application_order"), "effect-major, then target stable ascending")
    require_equal(errors, "canonical multi-target exactly once", multi.get("exactly_once_per_effect_target_pair"), True)
    require_equal(errors, "canonical multi-target atomic rejection", multi.get("atomic_rejection"), {"state_mutations": 0, "cost_mutations": 0, "rng_draws": 0})
    lowest = semantics.get("lowest_health", {})
    require_equal(errors, "canonical lowest-health population", lowest.get("population"), "active_same_side_combatants_including_executor")
    require_equal(errors, "canonical lowest-health max hp invariant", lowest.get("required_snapshot_invariant"), "every_candidate_max_hp_greater_than_zero")
    require_equal(errors, "canonical lowest-health invalid snapshot", lowest.get("invalid_snapshot"), "corrupted_snapshot_preflight_error")
    require_equal(errors, "canonical lowest-health comparison", lowest.get("comparison"), "integer_rational_cross_multiplication_no_float")
    require_equal(errors, "canonical lowest-health tie break", lowest.get("tie_break"), "stable_combatant_id_ascending")
    surrounded = semantics.get("surrounded", {})
    require_equal(errors, "canonical surrounded neighborhood", surrounded.get("neighborhood"), "six_adjacent_hexes_intersecting_candidate_anchor")
    require_equal(errors, "canonical surrounded counts", surrounded.get("counts"), "distinct_occupied_hexes_not_combatants")
    require_equal(errors, "canonical surrounded ally count", surrounded.get("ally_count"), "exclude_entire_candidate_footprint")
    require_equal(errors, "canonical surrounded active footprints", surrounded.get("active_footprints_only"), True)
    scope = semantics.get("strategy_scope", {})
    require_equal(errors, "canonical strategy scopes", scope.get("allowed"), ["combatants", "role", "all_allies"])
    require_equal(errors, "canonical all-allies side precedence", scope.get("all_allies_resolution"), "executor_side_all_allies_side_overlay")
    require_equal(errors, "canonical unsupported strategy scope", scope.get("unsupported_authoring_scope"), "side")
    require_equal(errors, "canonical strategy precedence", scope.get("precedence"), ["combatant", "role", "all_allies_side", "baseline"])
    formula = semantics.get("formula_fingerprint", {})
    require_equal(errors, "canonical formula semantic tag", formula.get("semantic_tag"), "combat.formula.v1.fixed_chance")
    require_equal(errors, "canonical formula namespace", formula.get("namespace"), "actual_combat")
    require_equal(errors, "canonical formula tuple prefix", formula.get("tuple_prefix"), ["combat.formula.v1.fixed_chance", "actual_combat"])
    require_equal(errors, "canonical formula encoding", formula.get("encoding"), "utf_8_compact_canonical_json_array")
    require_equal(errors, "canonical formula object keys", formula.get("object_keys"), "BTreeMap_lexicographic")
    require_equal(errors, "canonical formula hash", formula.get("hash"), "FNV-1a_64")
    require_equal(errors, "canonical input fingerprint", formula.get("input_fingerprint"), "lowercase_16_hex_of_fnv_result")
    require_equal(errors, "canonical sub-seed relation", formula.get("sub_seed"), "same_64_bit_fnv_result")
    require_equal(errors, "canonical roll modulo", formula.get("draw_zero_roll"), "sub_seed_mod_100")
    require_equal(errors, "canonical registry membership policy", semantics.get("registry_membership"), "unknown_ids_are_preflight_or_validator_errors")
    require_equal(errors, "canonical outcome selector provenance", semantics.get("outcome_resolution", {}).get("action_source_selectors"), "resolve_in_pause_preflight_and_record_provenance")
    require_equal(errors, "canonical outcome target ordering", semantics.get("outcome_resolution", {}).get("target_ordering"), "deterministic")


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
    for name in ("combat_intervention", "combat_intervention_response", "combat_simulation_version"):
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
                if isinstance(payload, dict) and isinstance(intervention_value, dict):
                    validate_non_empty_authored_ids(payload, errors)
                    validate_authoring_membership(payload, intervention_value.get("registry", {}), errors)

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
    simulation_schema = schemas.get("combat_simulation_version")
    if simulation_schema is not None:
        errors.extend(
            f"combat_simulation_version.schema.json: {message}"
            for message in validate_json_schema(sim, simulation_schema)
        )
    versions = sim.get("supported_versions", [])
    if not isinstance(versions, list) or any(
        not isinstance(item, dict)
        or not isinstance(item.get("version"), str)
        or not isinstance(item.get("source"), str)
        or not isinstance(item.get("current_runtime_observed"), bool)
        or not isinstance(item.get("features"), list)
        for item in versions
    ):
        errors.append("simulation_version schema: malformed supported_versions item")
    if not isinstance(sim.get("authoring"), dict) or "required_simulation_version" not in sim.get("authoring", {}):
        errors.append("simulation_version schema: missing authoring.required_simulation_version")
    if sim.get("authoring", {}).get("compatibility") != "exact":
        errors.append("simulation_version schema: compatibility must be exact")
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
    require_equal(errors, "simulation version supported set", [x.get("version") for x in versions if isinstance(x, dict)], ["v3"])
    require_equal(errors, "simulation authoring version required", sim.get("authoring", {}).get("required_simulation_version"), "required")
    require_equal(errors, "simulation compatibility", sim.get("authoring", {}).get("compatibility"), "exact")

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
        validate_effect_target_preflight(intervention, errors)
        validate_canonical_semantics(intervention, errors)
        validate_intervention(intervention, errors)
        validate_i7_contract(intervention, errors)
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

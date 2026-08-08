import json
import shutil
import subprocess
import sys
from pathlib import Path

import pytest
import yaml


ROOT = Path(__file__).parents[3]
VALIDATOR = ROOT / "tools/story_design/validate_combat_contracts.py"
SOURCE = ROOT / "docs/content/design_source"


def run(root, authoring_payload=None):
    command = [sys.executable, str(VALIDATOR), "--root", str(root)]
    if authoring_payload is not None:
        command.extend(["--authoring-payload", str(authoring_payload)])
    return subprocess.run(
        command,
        text=True,
        capture_output=True,
        check=False,
    )


def fixture(tmp_path):
    root = tmp_path / "design_source"
    shutil.copytree(SOURCE / "contracts", root / "contracts")
    (root / "schema").mkdir()
    for name in ("combat_intervention.schema.json", "combat_intervention_response.schema.json"):
        shutil.copy2(SOURCE / "schema" / name, root / "schema" / name)
    return root


def mutate(path, mutate_fn):
    value = yaml.safe_load(path.read_text(encoding="utf-8"))
    mutate_fn(value)
    path.write_text(
        yaml.safe_dump(value, allow_unicode=True, sort_keys=False),
        encoding="utf-8",
    )


def write_payload(tmp_path, value):
    path = tmp_path / "response.yml"
    path.write_text(
        yaml.safe_dump(value, allow_unicode=True, sort_keys=False),
        encoding="utf-8",
    )
    return path


def special_effect_payload():
    return {
        "special_effect": {
            "formula_id": "combat.formula.v1.fixed_chance",
            "formula_parameters": {"chance_hundredths": 5000},
            "executor_selector_id": "combat.selector.executor.v1.observer",
            "target_selector_id": "combat.selector.target.v1.selected_target",
            "success": {
                "effect_ids": ["stagger"],
                "outcome_actions": [
                    {"kind": "set_flag", "flag_id": "combat.intervention.succeeded"},
                    {"kind": "grant_item", "item_id": "quest_token"},
                ],
            },
            "failure": {"effect_ids": [], "outcome_actions": []},
        }
    }


def strategy_payload():
    return {
        "strategy_modifier": {
            "scope": {"kind": "all_allies"},
            "duration": "until_replaced",
            "operations": [
                {
                    "kind": "set_targeting_rule",
                    "rule_id": "combat.strategy.targeting.v1.attackers_of",
                }
            ],
        }
    }


def assert_invalid(root, message):
    result = run(root)
    assert result.returncode != 0
    assert message in result.stdout


def test_contracts_pass_and_match_runtime_capability(tmp_path):
    result = run(fixture(tmp_path))
    assert result.returncode == 0, result.stdout
    report = json.loads(result.stdout)
    assert report["status"] == "PASS"
    assert report["runtime_status"] == "handoff_required"
    assert report["supported_simulation_version_observed"] == "v3"


def test_contract_and_authoring_schemas_have_distinct_root_roles(tmp_path):
    root = fixture(tmp_path)
    schema = json.loads(
        (root / "schema/combat_intervention.schema.json").read_text(encoding="utf-8")
    )
    response_schema = json.loads(
        (root / "schema/combat_intervention_response.schema.json").read_text(encoding="utf-8")
    )
    assert schema["properties"]["contract_id"]["const"] == "combat_intervention_response"
    assert schema["description"].startswith("Validates the complete")
    assert response_schema["description"].startswith("Validates only the composite payload")
    assert response_schema["$id"] == "combat_intervention_response.schema.json"


@pytest.mark.parametrize("payload_kind", ["effect", "strategy", "both"])
def test_authoring_schema_accepts_three_payload_presence_shapes(tmp_path, payload_kind):
    root = fixture(tmp_path)
    payload = {}
    if payload_kind in {"effect", "both"}:
        payload.update(special_effect_payload())
    if payload_kind in {"strategy", "both"}:
        payload.update(strategy_payload())
    result = run(root, write_payload(tmp_path, payload))
    assert result.returncode == 0, result.stdout
    assert json.loads(result.stdout)["authoring_payload_validated"] is True


def test_authoring_schema_rejects_empty_and_legacy_kind_payloads(tmp_path):
    root = fixture(tmp_path)
    assert "does not satisfy anyOf" in run(root, write_payload(tmp_path, {})).stdout
    legacy = special_effect_payload()
    legacy["resolution_kind"] = "effect"
    assert "unknown property 'resolution_kind'" in run(
        root, write_payload(tmp_path, legacy)
    ).stdout


@pytest.mark.parametrize(
    ("action", "missing"),
    [
        ({"kind": "set_flag"}, "flag_id"),
        (
            {
                "kind": "create_loot_entitlement",
                "item_id": "quest_token",
                "claim_policy": "default_terminal_policy",
            },
            "source_selector_id",
        ),
        ({"kind": "grant_item"}, "item_id"),
    ],
)
def test_authoring_action_payload_requires_kind_specific_fields(tmp_path, action, missing):
    root = fixture(tmp_path)
    payload = special_effect_payload()
    payload["special_effect"]["success"]["outcome_actions"] = [action]
    result = run(root, write_payload(tmp_path, payload))
    assert result.returncode != 0
    assert f"missing required property '{missing}'" in result.stdout


def test_duplicate_yaml_key_fails_before_semantic_validation(tmp_path):
    root = fixture(tmp_path)
    path = root / "contracts/intervention.yml"
    path.write_text(
        path.read_text(encoding="utf-8") + "\nstatus: canonical\n",
        encoding="utf-8",
    )
    assert_invalid(root, "duplicate key: status")


def test_root_schema_rejects_deleted_required_field(tmp_path):
    root = fixture(tmp_path)
    mutate(
        root / "contracts/intervention.yml",
        lambda data: data["transaction"].pop("boundary"),
    )
    assert_invalid(root, "missing required property 'boundary'")


def test_root_schema_rejects_unknown_property(tmp_path):
    root = fixture(tmp_path)
    mutate(
        root / "contracts/intervention.yml",
        lambda data: data.__setitem__("implementation_complete", True),
    )
    assert_invalid(root, "unknown property 'implementation_complete'")


def test_schema_yaml_mismatch_is_detected_by_actual_root_validation(tmp_path):
    root = fixture(tmp_path)
    path = root / "schema/combat_intervention.schema.json"
    schema = json.loads(path.read_text(encoding="utf-8"))
    schema["properties"]["contract_id"]["const"] = "wrong_contract"
    path.write_text(json.dumps(schema, ensure_ascii=False, indent=2), encoding="utf-8")
    assert_invalid(root, "expected const 'wrong_contract'")


def test_termination_priority_tie_fails(tmp_path):
    root = fixture(tmp_path)
    mutate(
        root / "contracts/termination.yml",
        lambda data: data["priority_order"][1].__setitem__("priority", 10),
    )
    assert_invalid(root, "priorities must be unique")


def test_unknown_objective_result_fails(tmp_path):
    root = fixture(tmp_path)
    mutate(
        root / "contracts/termination.yml",
        lambda data: data["objective_mapping"]["allowed_result_kinds"].append("unknown"),
    )
    assert_invalid(root, "objective result enum mismatch")


def test_simulation_version_must_match_code_capability(tmp_path):
    root = fixture(tmp_path)
    mutate(
        root / "contracts/simulation_version.yml",
        lambda data: data["supported_versions"][0].__setitem__("version", "v9"),
    )
    assert_invalid(root, "does not match code capability 'v3'")


def test_identity_internal_id_leak_fails(tmp_path):
    root = fixture(tmp_path)
    mutate(
        root / "contracts/identity.yml",
        lambda data: data["internal_id"].__setitem__("user_visible", "allowed"),
    )
    assert_invalid(root, "internal id must not be user-visible")


def test_log_boundary_fails(tmp_path):
    root = fixture(tmp_path)
    mutate(
        root / "contracts/logs.yml",
        lambda data: data["non_groupable"].remove("terminal"),
    )
    assert_invalid(root, "terminal/status/objective")


def test_zone_gap_and_overlap_fail(tmp_path):
    root = fixture(tmp_path)
    path = root / "contracts/tactical_zones.yml"
    mutate(
        path,
        lambda data: data.__setitem__(
            "zones",
            [
                {"id": "front", "bounds": {"depth": [0.0, 0.4], "lane": [0.0, 1.0]}},
                {"id": "back", "bounds": {"depth": [0.5, 1.0], "lane": [0.0, 1.0]}},
            ],
        ),
    )
    assert_invalid(root, "depth gap")
    mutate(
        path,
        lambda data: data.__setitem__(
            "zones",
            [
                {"id": "front", "bounds": {"depth": [0.0, 0.6], "lane": [0.0, 1.0]}},
                {"id": "back", "bounds": {"depth": [0.5, 1.0], "lane": [0.0, 1.0]}},
            ],
        ),
    )
    assert_invalid(root, "depth overlap")


def test_response_payload_requires_composite_presence_contract(tmp_path):
    root = fixture(tmp_path)
    mutate(
        root / "contracts/intervention.yml",
        lambda data: data["response_payload"].__setitem__("minimum_present", 0),
    )
    assert_invalid(root, "response minimum payload")


@pytest.mark.parametrize("legacy_id", ["self", "target", "observer", "opponent", "any"])
def test_legacy_selector_aliases_are_migration_only(tmp_path, legacy_id):
    root = fixture(tmp_path)
    mutate(
        root / "contracts/intervention.yml",
        lambda data: data["registry"]["selector_ids"]["target"].append(legacy_id),
    )
    assert_invalid(root, "legacy or non-canonical selector ID")


def test_unknown_formula_namespace_fails(tmp_path):
    root = fixture(tmp_path)
    mutate(
        root / "contracts/intervention.yml",
        lambda data: data["registry"].__setitem__("formula_ids", ["fixed_chance"]),
    )
    assert_invalid(root, "non-canonical formula ID")


@pytest.mark.parametrize("registry_kind", ["selector", "formula", "strategy"])
def test_registry_ids_must_be_unique(tmp_path, registry_kind):
    root = fixture(tmp_path)

    def duplicate(data):
        registry = data["registry"]
        if registry_kind == "selector":
            registry["selector_ids"]["target"].append(
                registry["selector_ids"]["target"][0]
            )
        elif registry_kind == "formula":
            registry["formula_ids"].append(registry["formula_ids"][0])
        else:
            registry["strategy_targeting_rule_ids"].append(
                registry["strategy_targeting_rule_ids"][0]
            )

    mutate(root / "contracts/intervention.yml", duplicate)
    assert_invalid(root, "duplicate array items")


def test_outcome_action_enum_is_closed(tmp_path):
    root = fixture(tmp_path)
    mutate(
        root / "contracts/intervention.yml",
        lambda data: data["special_effect"]["outcome_action_kinds"].append("run_script"),
    )
    assert_invalid(root, "outcome action kinds")


@pytest.mark.parametrize(
    ("path", "value", "message"),
    [
        (("scopes",), ["all_allies", "role", "combatants", "world"], "strategy scopes"),
        (("durations",), ["until_replaced", "next_segment", "multi_tick"], "strategy durations"),
        (("stacking", "additive_numeric"), "allowed", "additive stacking"),
        (("stacking", "arbitrary_json_patch"), "allowed", "arbitrary patch"),
    ],
)
def test_strategy_contract_rejects_open_ended_scope_duration_and_stacking(
    tmp_path, path, value, message
):
    root = fixture(tmp_path)

    def change(data):
        target = data["strategy_modifier"]
        for key in path[:-1]:
            target = target[key]
        target[path[-1]] = value

    mutate(root / "contracts/intervention.yml", change)
    assert_invalid(root, message)


@pytest.mark.parametrize(
    ("field", "value", "message"),
    [
        ("nested_pause", "allowed", "nested pause"),
        ("stale_response", "accept_latest", "stale response"),
        ("pause_is_termination_candidate", True, "pause terminal candidate"),
    ],
)
def test_lifecycle_rejects_pause_terminal_conflation(tmp_path, field, value, message):
    root = fixture(tmp_path)
    mutate(
        root / "contracts/intervention.yml",
        lambda data: data["lifecycle"].__setitem__(field, value),
    )
    assert_invalid(root, message)


def test_ko_tick_order_is_exact(tmp_path):
    root = fixture(tmp_path)
    mutate(
        root / "contracts/intervention.yml",
        lambda data: data["lifecycle"]["ko_tick_order"].reverse(),
    )
    assert_invalid(root, "intervention KO tick order")


@pytest.mark.parametrize(
    ("field", "value", "message"),
    [
        ("overrides_pause", False, "forced stop override"),
        ("terminal_priority", 20, "forced stop priority"),
    ],
)
def test_forced_stop_must_override_pause_at_priority_ten(tmp_path, field, value, message):
    root = fixture(tmp_path)
    mutate(
        root / "contracts/intervention.yml",
        lambda data: data["lifecycle"]["forced_stop"].__setitem__(field, value),
    )
    assert_invalid(root, message)


def test_checkpoint_versions_and_receipt_seed_order_are_fixed(tmp_path):
    root = fixture(tmp_path)
    path = root / "contracts/intervention.yml"
    mutate(
        path,
        lambda data: data["checkpoint_and_receipt"].__setitem__(
            "combat_checkpoint_schema_version", 1
        ),
    )
    assert_invalid(root, "checkpoint schema")
    root = fixture(tmp_path / "second")
    mutate(
        root / "contracts/intervention.yml",
        lambda data: data["checkpoint_and_receipt"].__setitem__(
            "next_segment_seed_input", "intent_fingerprint"
        ),
    )
    assert_invalid(root, "next segment seed")


def test_receipt_required_fields_and_status_enum_are_closed(tmp_path):
    root = fixture(tmp_path)
    mutate(
        root / "contracts/intervention.yml",
        lambda data: data["checkpoint_and_receipt"]["receipt_required_fields"].remove(
            "application_transaction_id"
        ),
    )
    assert_invalid(root, "receipt required fields")
    root = fixture(tmp_path / "status")
    mutate(
        root / "contracts/intervention.yml",
        lambda data: data["checkpoint_and_receipt"]["action_receipt_statuses"].append(
            "unknown"
        ),
    )
    assert_invalid(root, "action receipt statuses")


@pytest.mark.parametrize(
    ("field", "value", "message"),
    [
        ("partial_apply_serialization", "allowed", "partial serialization"),
        ("delayed_or_multi_tick_workflow", "allowed", "delayed workflow"),
    ],
)
def test_transaction_rejects_partial_or_delayed_application(tmp_path, field, value, message):
    root = fixture(tmp_path)
    mutate(
        root / "contracts/intervention.yml",
        lambda data: data["transaction"].__setitem__(field, value),
    )
    assert_invalid(root, message)


def test_probabilistic_effect_consumes_exactly_one_rng_draw(tmp_path):
    root = fixture(tmp_path)
    mutate(
        root / "contracts/intervention.yml",
        lambda data: data["transaction"]["rng_draws"].__setitem__(
            "probabilistic_special_effect", 2
        ),
    )
    assert_invalid(root, "rng draws")

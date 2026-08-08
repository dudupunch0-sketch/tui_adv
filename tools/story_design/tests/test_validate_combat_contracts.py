import json
import shutil
import subprocess
from pathlib import Path

import pytest
import yaml


ROOT = Path(__file__).parents[3]
VALIDATOR = ROOT / "tools/story_design/validate_combat_contracts.py"
SOURCE = ROOT / "docs/content/design_source"


def run(root):
    return subprocess.run(
        ["python3", str(VALIDATOR), "--root", str(root)],
        text=True,
        capture_output=True,
        check=False,
    )


def fixture(tmp_path):
    root = tmp_path / "design_source"
    shutil.copytree(SOURCE / "contracts", root / "contracts")
    (root / "schema").mkdir()
    shutil.copy2(
        SOURCE / "schema/combat_intervention.schema.json",
        root / "schema/combat_intervention.schema.json",
    )
    return root


def mutate(path, mutate_fn):
    value = yaml.safe_load(path.read_text(encoding="utf-8"))
    mutate_fn(value)
    path.write_text(
        yaml.safe_dump(value, allow_unicode=True, sort_keys=False),
        encoding="utf-8",
    )


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


def test_intervention_schema_is_valid_json_and_exposes_response_authoring(tmp_path):
    root = fixture(tmp_path)
    schema = json.loads(
        (root / "schema/combat_intervention.schema.json").read_text(encoding="utf-8")
    )
    assert schema["properties"]["contract_id"]["const"] == "combat_intervention_response"
    assert "responseAuthoring" in schema["$defs"]


def test_duplicate_yaml_key_fails_before_semantic_validation(tmp_path):
    root = fixture(tmp_path)
    path = root / "contracts/intervention.yml"
    path.write_text(
        path.read_text(encoding="utf-8") + "\nstatus: canonical\n",
        encoding="utf-8",
    )
    assert_invalid(root, "duplicate key: status")


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

import json
import shutil
import subprocess
from pathlib import Path
import yaml

ROOT = Path(__file__).parents[3]
VALIDATOR = ROOT / "tools/story_design/validate_combat_contracts.py"
SOURCE = ROOT / "docs/content/design_source"

def run(root):
    return subprocess.run(
        ["python3", str(VALIDATOR), "--root", str(root)],
        text=True, capture_output=True, check=False,
    )

def fixture(tmp_path):
    root = tmp_path / "design_source"
    shutil.copytree(SOURCE / "contracts", root / "contracts")
    return root

def mutate(path, mutate_fn):
    value = yaml.safe_load(path.read_text(encoding="utf-8"))
    mutate_fn(value)
    path.write_text(yaml.safe_dump(value, allow_unicode=True, sort_keys=False), encoding="utf-8")

def test_contracts_pass_and_declare_handoff_required(tmp_path):
    result = run(fixture(tmp_path))
    assert result.returncode == 0, result.stdout
    report = json.loads(result.stdout)
    assert report["status"] == "PASS"
    assert report["runtime_status"] == "handoff_required"

def test_termination_priority_tie_fails(tmp_path):
    root = fixture(tmp_path)
    mutate(root / "contracts/termination.yml", lambda d: d["priority_order"][1].__setitem__("priority", 10))
    result = run(root)
    assert result.returncode != 0
    assert "priorities must be unique" in result.stdout

def test_unknown_objective_result_fails(tmp_path):
    root = fixture(tmp_path)
    mutate(root / "contracts/termination.yml", lambda d: d["objective_mapping"]["allowed_result_kinds"].append("unknown"))
    result = run(root)
    assert result.returncode != 0
    assert "objective result enum mismatch" in result.stdout

def test_unsupported_version_capability_fails(tmp_path):
    root = fixture(tmp_path)
    mutate(root / "contracts/simulation_version.yml", lambda d: d["supported_versions"][0].__setitem__("version", "v9"))
    result = run(root)
    assert result.returncode != 0
    assert "current supported runtime version v1 is missing" in result.stdout

def test_identity_internal_id_leak_fails(tmp_path):
    root = fixture(tmp_path)
    mutate(root / "contracts/identity.yml", lambda d: d["internal_id"].__setitem__("user_visible", "allowed"))
    result = run(root)
    assert result.returncode != 0
    assert "internal id must not be user-visible" in result.stdout

def test_log_boundary_fails(tmp_path):
    root = fixture(tmp_path)
    mutate(root / "contracts/logs.yml", lambda d: d["non_groupable"].remove("terminal"))
    result = run(root)
    assert result.returncode != 0
    assert "terminal/status/objective" in result.stdout

def test_zone_gap_and_overlap_fail(tmp_path):
    root = fixture(tmp_path)
    path = root / "contracts/tactical_zones.yml"
    mutate(path, lambda d: d.__setitem__("zones", [
        {"id": "front", "bounds": {"depth": [0.0, 0.4], "lane": [0.0, 1.0]}},
        {"id": "back", "bounds": {"depth": [0.5, 1.0], "lane": [0.0, 1.0]}},
    ]))
    result = run(root)
    assert result.returncode != 0
    assert "depth gap" in result.stdout
    mutate(path, lambda d: d.__setitem__("zones", [
        {"id": "front", "bounds": {"depth": [0.0, 0.6], "lane": [0.0, 1.0]}},
        {"id": "back", "bounds": {"depth": [0.5, 1.0], "lane": [0.0, 1.0]}},
    ]))
    result = run(root)
    assert result.returncode != 0
    assert "depth overlap" in result.stdout

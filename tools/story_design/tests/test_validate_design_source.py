import json
import subprocess
import sys
from pathlib import Path

import yaml


SCRIPT = Path(__file__).parents[1] / "validate_design_source.py"


def write_fixture(tmp_path, *, semantic=False, warning_only=False):
    root = tmp_path / "design_source"
    (root / "events").mkdir(parents=True)
    (root / "rewards" / "titles").mkdir(parents=True)
    (root / "reward_mappings").mkdir(parents=True)
    (root / "reports" / "validation").mkdir(parents=True)
    (root / "manifest.yml").write_text(
        "schema_version: 1\nlocal_canonical_declared: false\n", encoding="utf-8"
    )
    event = {
        "id": "event_fixture",
        "kind": "event",
        "status": "draft",
        "runtime_status": "design",
        "source_refs": [{"export_file": "fixture.csv"}],
        "notion_properties": {"선택지 요약": "정상 선택"},
    }
    (root / "events" / "event_fixture.yml").write_text(
        yaml.safe_dump(event, allow_unicode=True), encoding="utf-8"
    )
    reward = {
        "id": "wuxia_title_with_underscore",
        "kind": "title",
        "status": "draft",
        "runtime_status": "design",
        "source_refs": [{"export_file": "fixture.csv"}],
    }
    (root / "rewards" / "titles" / "reward_fixture.yml").write_text(
        yaml.safe_dump(reward, allow_unicode=True), encoding="utf-8"
    )
    mapping = {
        "id": "mapping_fixture",
        "status": "draft",
        "runtime_status": "design",
        "source_refs": [{"export_file": "fixture.csv"}],
        "notion_properties": {
            "사건 ID": "missing_event" if semantic else "event_fixture",
            "보상 ID": "wuxia_title_with_underscore",
            "선택지·조건 키": "정상 선택",
        },
    }
    (root / "reward_mappings" / "mapping_fixture.yml").write_text(
        yaml.safe_dump(mapping, allow_unicode=True), encoding="utf-8"
    )
    return root


def run(root):
    return subprocess.run(
        [sys.executable, str(SCRIPT), "--root", str(root)],
        capture_output=True,
        text=True,
    )


def test_underscore_reward_id_and_capability(tmp_path):
    result = run(write_fixture(tmp_path))
    assert result.returncode == 0, result.stdout
    report = json.loads(result.stdout)
    assert report["semantic_errors"] == []
    assert report["capabilities"]["next_fallback"] == "not_available_in_source"


def test_semantic_error_is_nonzero(tmp_path):
    result = run(write_fixture(tmp_path, semantic=True))
    assert result.returncode != 0
    report = json.loads(result.stdout)
    assert report["semantic_errors"]


def test_unverifiable_only_policy_is_zero(tmp_path):
    root = write_fixture(tmp_path)
    event = root / "events" / "event_fixture.yml"
    data = yaml.safe_load(event.read_text(encoding="utf-8"))
    data["source_refs"] = [{"export_file": "fixture.csv"}]
    data["notion_properties"]["후일담 카드 연결"] = "설명만 있는 후일담 연결"
    event.write_text(yaml.safe_dump(data, allow_unicode=True), encoding="utf-8")
    result = run(root)
    assert result.returncode == 0
    report = json.loads(result.stdout)
    assert report["structural_errors"] == []
    assert report["semantic_errors"] == []
    assert report["unverifiable"]
    assert report["policy"]["warnings_and_unverifiable_only_do_not_fail"] is True

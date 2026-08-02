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
    (root / "afterthoughts").mkdir(parents=True)
    (root / "graphs").mkdir(parents=True)
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
    afterthought = {
        "id": "notion_0012",
        "kind": "afterthought",
        "status": "draft",
        "runtime_status": "design",
        "source_refs": [{"export_file": "fixture.csv"}],
        "notion_properties": {"카드명": "청류문의 후일", "카드 ID": "epilogue_qingliu_future"},
    }
    (root / "afterthoughts" / "afterthought_fixture.yml").write_text(
        yaml.safe_dump(afterthought, allow_unicode=True), encoding="utf-8"
    )
    second_afterthought = dict(afterthought)
    second_afterthought["id"] = "notion_0013"
    second_afterthought["notion_properties"] = {"카드명": "서하린의 후일", "카드 ID": "epilogue_seoharin_future"}
    (root / "afterthoughts" / "afterthought_second.yml").write_text(
        yaml.safe_dump(second_afterthought, allow_unicode=True), encoding="utf-8"
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
    (root / "reports" / "afterthought_triage.yml").write_text("entries: []\n", encoding="utf-8")
    (root / "graphs" / "afterthought_conditions.yml").write_text("conditions: []\n", encoding="utf-8")
    (root / "graphs" / "event_afterthought_links.yml").write_text(
        yaml.safe_dump(
            {
                "policy": {
                    "eligibility_evaluation": "event_resolution",
                    "reveal_default": "ending_resolution",
                    "exclusive_group_rule": "at_most_one",
                },
                "links": [],
            },
            allow_unicode=True,
        ),
        encoding="utf-8",
    )
    return root


def run(root):
    return subprocess.run(
        [sys.executable, str(SCRIPT), "--root", str(root)],
        capture_output=True,
        text=True,
    )


def write_links(root, links):
    (root / "graphs" / "event_afterthought_links.yml").write_text(
        yaml.safe_dump(
            {
                "policy": {
                    "eligibility_evaluation": "event_resolution",
                    "reveal_default": "ending_resolution",
                    "exclusive_group_rule": "at_most_one",
                },
                "links": links,
            },
            allow_unicode=True,
        ),
        encoding="utf-8",
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
    (root / "reports" / "afterthought_triage.yml").write_text(
        yaml.safe_dump(
            {
                "entries": [
                    {
                        "event_id": "event_fixture",
                        "source_path": "events/event_fixture.yml",
                        "raw_reference": "설명만 있는 후일담 연결",
                        "classification": "descriptive_condition",
                        "candidate_afterthought_ids": [],
                        "evidence": ["fixture"],
                        "confidence": "medium",
                        "review_status": "approved_as_condition",
                        "condition_summary": "설명만 있는 후일담 연결",
                    }
                ]
            },
            allow_unicode=True,
        ),
        encoding="utf-8",
    )
    (root / "graphs" / "afterthought_conditions.yml").write_text(
        yaml.safe_dump(
            {
                "conditions": [
                    {
                        "event_id": "event_fixture",
                        "raw_reference": "설명만 있는 후일담 연결",
                        "condition_summary": "설명만 있는 후일담 연결",
                        "status": "approved_as_condition",
                    }
                ]
            },
            allow_unicode=True,
        ),
        encoding="utf-8",
    )
    result = run(root)
    assert result.returncode == 0
    report = json.loads(result.stdout)
    assert report["structural_errors"] == []
    assert report["semantic_errors"] == []
    assert report["unverifiable"] == []
    assert report["afterthought_triage"]["classification_counts"]["descriptive_condition"] == 1
    assert report["policy"]["warnings_and_unverifiable_only_do_not_fail"] is True


def approved_link(card="notion_0012", **extra):
    link = {
        "event_id": "event_fixture",
        "afterthought_id": card,
        "relation": "primary",
        "eligibility": {"all_of": ["event:event_fixture"]},
        "reveal_policy": "ending_resolution",
        "exclusive_group": "fixture_group",
        "priority": 10,
        "status": "approved",
    }
    link.update(extra)
    return link


def test_approved_explicit_link_happy_path(tmp_path):
    root = write_fixture(tmp_path)
    event = root / "events" / "event_fixture.yml"
    data = yaml.safe_load(event.read_text(encoding="utf-8"))
    data["notion_properties"]["후일담 카드 연결"] = "청류문의 후일"
    event.write_text(yaml.safe_dump(data, allow_unicode=True), encoding="utf-8")
    write_links(root, [approved_link()])
    result = run(root)
    assert result.returncode == 0, result.stdout
    report = json.loads(result.stdout)
    assert report["afterthought_overlays"]["approved_link_count"] == 1


def test_orphan_card_is_rejected(tmp_path):
    root = write_fixture(tmp_path)
    write_links(root, [approved_link("missing_card")])
    result = run(root)
    assert result.returncode != 0
    assert "afterthought link card missing" in result.stdout


def test_duplicate_link_is_rejected(tmp_path):
    root = write_fixture(tmp_path)
    write_links(root, [approved_link(), approved_link()])
    result = run(root)
    assert result.returncode != 0
    assert "duplicate event+afterthought link" in result.stdout


def test_exclusive_priority_tie_is_rejected(tmp_path):
    root = write_fixture(tmp_path)
    write_links(root, [approved_link(), approved_link("notion_0013", priority=10)])
    result = run(root)
    assert result.returncode != 0
    assert "exclusive-group priority tie" in result.stdout


def test_chain_cycle_is_rejected(tmp_path):
    root = write_fixture(tmp_path)
    a = approved_link("notion_0012", relation="chain", depends_on="notion_0013")
    b = approved_link("notion_0013", relation="chain", depends_on="notion_0012")
    write_links(root, [a, b])
    result = run(root)
    assert result.returncode != 0
    assert "afterthought chain cycle detected" in result.stdout


def test_pending_link_is_excluded_from_runtime_approved_count(tmp_path):
    root = write_fixture(tmp_path)
    write_links(root, [approved_link(status="draft")])
    result = run(root)
    assert result.returncode == 0, result.stdout
    report = json.loads(result.stdout)
    assert report["afterthought_overlays"]["approved_link_count"] == 0
    assert report["afterthought_overlays"]["unresolved_runtime_links"] == [["event_fixture", "notion_0012"]]

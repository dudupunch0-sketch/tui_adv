from __future__ import annotations

import json
import subprocess
import sys
from importlib.util import module_from_spec, spec_from_file_location
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCRIPT_PATH = ROOT / "scripts" / "export_web_data.py"
PRIVATE_SECRET_FIELDS = {"final_hint", "actual_ip_address", "office_location", "treasure_location"}


def _load_export_module():
    spec = spec_from_file_location("export_web_data", SCRIPT_PATH)
    assert spec is not None and spec.loader is not None
    module = module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def _missing_private_secret_fields(payload: object) -> bool:
    if isinstance(payload, dict):
        if PRIVATE_SECRET_FIELDS.intersection(payload):
            return False
        return all(_missing_private_secret_fields(value) for value in payload.values())
    if isinstance(payload, list):
        return all(_missing_private_secret_fields(value) for value in payload)
    return True


def test_export_web_data_builds_public_manifest_with_expected_counts():
    exporter = _load_export_module()

    bundle = exporter.build_web_data(ROOT)

    assert bundle["manifest"]["schema_version"] == 1
    assert bundle["manifest"]["counts"] == {
        "locations": 16,
        "items": 13,
        "encounters": 21,
        "endings": 13,
        "achievements": 11,
        "secrets": 3,
    }
    assert bundle["locations"][0]["id"] == "dev_desk"
    assert bundle["encounters"][0]["id"] == "ex_employee_messenger"
    assert bundle["secrets"][0]["id"] == "real_note_001"


def test_export_web_data_writes_generated_json_files(tmp_path):
    exporter = _load_export_module()
    out_dir = tmp_path / "generated"

    written = exporter.write_web_data(ROOT, out_dir)

    assert sorted(path.name for path in written) == [
        "achievements.json",
        "encounters.json",
        "endings.json",
        "items.json",
        "locations.json",
        "manifest.json",
        "secrets.example.json",
    ]
    manifest = json.loads((out_dir / "manifest.json").read_text(encoding="utf-8"))
    assert manifest["counts"]["encounters"] == 21
    secrets = json.loads((out_dir / "secrets.example.json").read_text(encoding="utf-8"))
    assert secrets[0]["final_hint_policy"] == "private_only"


def test_export_web_data_builds_renderer_neutral_content_bundle():
    exporter = _load_export_module()

    bundle = exporter.build_content_bundle(ROOT)

    assert bundle["schema_version"] == 1
    assert bundle["kind"] == "tui_adv.content_bundle"
    assert "runtime" not in bundle
    assert bundle["manifest"]["counts"]["locations"] == 16
    assert bundle["content"]["locations"][0]["id"] == "dev_desk"
    assert bundle["content"]["encounters"][0]["id"] == "ex_employee_messenger"
    assert not any(
        encounter["id"]
        in {
            "wuxia_commute_rift_arrival",
            "wuxia_heuksa_bang_first_fight",
            "wuxia_cheonggi_record_first_fragment",
            "wuxia_seo_harin_rescue",
            "wuxia_cheongryu_apprentice_entry",
            "wuxia_cheongryu_chore_sparring",
            "wuxia_cheongryu_raid_route_split",
            "wuxia_cheongryu_raid_wounded_fallback",
            "wuxia_baekdo_medicine_debt",
        }
        for encounter in bundle["content"]["encounters"]
    )
    assert _missing_private_secret_fields(bundle)


def test_export_web_data_builds_wuxia_storypack_preview_bundle():
    exporter = _load_export_module()

    bundle = exporter.build_storypack_preview_bundle(ROOT, "wuxia_jianghu_pack")

    assert bundle["schema_version"] == 1
    assert bundle["kind"] == "tui_adv.content_bundle"
    assert bundle["runtime"] == {
        "runtime_mode": "storypack_preview",
        "world_id": "wuxia_jianghu",
        "storypack_id": "wuxia_jianghu_pack",
        "default_location": "wuxia_commute_rift",
    }
    assert "storypack-previews/wuxia_jianghu_pack" in bundle["source"]
    assert bundle["manifest"]["counts"] == {
        "locations": 4,
        "items": 3,
        "encounters": 9,
        "endings": 1,
        "achievements": 2,
        "secrets": 0,
    }
    assert [location["id"] for location in bundle["content"]["locations"]] == [
        "wuxia_commute_rift",
        "jianghu_roadside",
        "jianghu_market_street",
        "cheongryu_outer_courtyard",
    ]
    encounter_ids = [encounter["id"] for encounter in bundle["content"]["encounters"]]
    assert encounter_ids == [
        "wuxia_commute_rift_arrival",
        "wuxia_heuksa_bang_first_fight",
        "wuxia_cheonggi_record_first_fragment",
        "wuxia_seo_harin_rescue",
        "wuxia_cheongryu_apprentice_entry",
        "wuxia_cheongryu_chore_sparring",
        "wuxia_cheongryu_raid_route_split",
        "wuxia_cheongryu_raid_wounded_fallback",
        "wuxia_baekdo_medicine_debt",
    ]
    first_fight = bundle["content"]["encounters"][1]
    assert first_fight["conditions"] == {
        "locations": ["jianghu_market_street"],
        "required_flags": ["wuxia_arrival_hidden"],
        "forbidden_flags": ["heuksa_bang_first_fight_resolved"],
    }
    assert first_fight["presentation"]["layout"] == "combat_intervention"
    assert first_fight["presentation"]["effect_cues"][0]["stable_terms"] == [
        "거리",
        "구두",
        "사원증",
    ]
    assert [choice["id"] for choice in first_fight["choices"]] == [
        "run_toward_open_street",
        "deescalate_with_words",
        "swing_commute_bag",
        "loosen_tie_and_drop_shoes",
        "crash_in_with_body",
    ]
    fallback = first_fight["choices"][0]
    assert fallback["outcome"]["add_flags"] == [
        "first_brawl_started",
        "heuksa_bang_first_fight_resolved",
        "first_brawl_survived",
    ]
    assert fallback["outcome"]["add_clues"] == [
        "violence_is_real",
        "open_street_escape_route",
    ]
    first_fragment = bundle["content"]["encounters"][2]
    assert first_fragment["conditions"] == {
        "locations": ["jianghu_market_street"],
        "required_flags": ["heuksa_bang_first_fight_resolved"],
        "forbidden_flags": ["cheonggi_record_first_fragment_resolved"],
    }
    assert first_fragment["presentation"]["layout"] == "cheonggi_record"
    assert first_fragment["presentation"]["effect_cues"][0]["stable_terms"] == [
        "업무수첩",
        "천기록",
        "실패 기록",
    ]
    assert [choice["id"] for choice in first_fragment["choices"]] == [
        "choose_guard_basics",
        "choose_keep_feet_moving",
        "choose_failure_log",
        "close_notebook_without_choice",
    ]
    fallback_fragment = first_fragment["choices"][-1]
    assert fallback_fragment["outcome"]["add_flags"] == [
        "cheonggi_record_awakened",
        "first_fragment_seen",
        "cheonggi_record_first_fragment_resolved",
        "cheonggi_record_caution",
    ]
    guard_choice = first_fragment["choices"][0]
    assert guard_choice["outcome"]["add_items"] == ["cheonggi_record_notebook"]
    assert guard_choice["outcome"]["add_clues"] == [
        "notebook_is_not_search",
        "fragments_are_training_directions",
    ]
    rescue = bundle["content"]["encounters"][3]
    assert rescue["conditions"] == {
        "locations": ["jianghu_market_street"],
        "required_flags": [
            "heuksa_bang_first_fight_resolved",
            "cheonggi_record_first_fragment_resolved",
        ],
        "forbidden_flags": ["seo_harin_rescue_resolved"],
    }
    assert rescue["presentation"]["layout"] == "rescue_and_investigation"
    assert rescue["presentation"]["effect_cues"][0]["stable_terms"] == [
        "서하린",
        "청류문",
        "감시",
    ]
    assert [choice["id"] for choice in rescue["choices"]] == [
        "tell_plain_truth",
        "ask_for_medical_help_first",
        "explain_company_and_commute",
        "show_cheonggi_record_page",
        "hide_employee_badge",
    ]
    fallback_rescue = rescue["choices"][0]
    assert fallback_rescue["outcome"]["add_flags"] == [
        "seo_harin_rescue_resolved",
        "seo_harin_intervened",
        "taken_under_watch",
        "outsider_claim_recorded",
        "truthful_outsider_claim",
    ]
    assert fallback_rescue["outcome"]["destination_id"] == "cheongryu_outer_courtyard"
    apprentice = bundle["content"]["encounters"][4]
    assert apprentice["conditions"] == {
        "locations": ["cheongryu_outer_courtyard"],
        "required_flags": ["seo_harin_rescue_resolved", "taken_under_watch"],
        "forbidden_flags": ["cheongryu_apprentice_entry_resolved"],
    }
    assert apprentice["presentation"]["layout"] == "cheongryu_apprenticeship"
    assert apprentice["presentation"]["effect_cues"][0]["stable_terms"] == [
        "청류문",
        "잡일",
        "수습생",
    ]
    assert [choice["id"] for choice in apprentice["choices"]] == [
        "accept_three_month_trial",
        "request_martial_training_immediately",
        "organize_chores_like_workflow",
        "inspect_archive_during_chore",
    ]
    fallback_apprentice = apprentice["choices"][0]
    assert fallback_apprentice["outcome"]["add_flags"] == [
        "cheongryu_apprentice_entry_resolved",
        "cheongryu_trial_started",
        "seo_harin_mentor_thread",
        "sect_debt_accepted",
        "chore_training_open",
    ]
    assert fallback_apprentice["outcome"]["add_items"] == ["work_chore_token"]
    assert fallback_apprentice["outcome"]["destination_id"] == "cheongryu_outer_courtyard"
    sparring = bundle["content"]["encounters"][5]
    assert sparring["conditions"] == {
        "locations": ["cheongryu_outer_courtyard"],
        "required_flags": [
            "cheongryu_apprentice_entry_resolved",
            "cheongryu_trial_started",
            "cheonggi_record_awakened",
            "first_fragment_seen",
        ],
        "forbidden_flags": ["cheongryu_chore_sparring_resolved"],
    }
    assert sparring["presentation"]["layout"] == "combat_intervention"
    assert sparring["presentation"]["effect_cues"][0]["stable_terms"] == [
        "균형",
        "호흡",
        "장작",
    ]
    assert [choice["id"] for choice in sparring["choices"]] == [
        "step_back_with_firewood",
        "let_shoulder_turn_with_push",
        "plant_bare_foot_in_dust",
        "ask_harin_what_changed",
    ]
    fallback_sparring = sparring["choices"][0]
    assert fallback_sparring["outcome"]["add_flags"] == [
        "cheongryu_chore_sparring_resolved",
        "chore_sparring_completed",
        "balance_training_noticed",
        "office_combat_model_reused",
    ]
    assert fallback_sparring["outcome"]["add_clues"] == [
        "balance_matters_more_than_force",
        "office_items_can_translate_to_training",
    ]
    assert fallback_sparring["outcome"]["destination_id"] == "cheongryu_outer_courtyard"
    raid = bundle["content"]["encounters"][6]
    assert raid["conditions"] == {
        "locations": ["cheongryu_outer_courtyard"],
        "required_flags": [
            "cheongryu_apprentice_entry_resolved",
            "cheongryu_trial_started",
            "cheonggi_record_awakened",
            "first_fragment_seen",
            "cheongryu_chore_sparring_resolved",
        ],
        "forbidden_flags": ["cheongryu_raid_route_split_resolved"],
    }
    assert raid["presentation"]["layout"] == "raid_route_pressure"
    assert raid["presentation"]["effect_cues"][0]["stable_terms"] == [
        "청류문",
        "백도맹",
        "천기록",
    ]
    assert [choice["id"] for choice in raid["choices"]] == [
        "evacuate_the_wounded_first",
        "defend_cheongryu_with_white_path",
        "trade_with_black_heaven",
        "follow_heavenly_archive",
    ]
    fallback_raid = raid["choices"][0]
    assert fallback_raid["outcome"]["add_flags"] == [
        "cheongryu_raid_route_split_resolved",
        "cheongryu_raid_survived",
        "route_commitment_pressure",
        "route_commitment_deferred",
        "wounded_saved_flag",
        "seo_harin_survived_raid",
    ]
    assert fallback_raid["outcome"]["add_clues"] == [
        "saving_people_delays_route_choice",
        "blood_moon_targets_cheonggi_record",
    ]
    assert fallback_raid["outcome"]["destination_id"] == "cheongryu_outer_courtyard"
    wounded_fallback = bundle["content"]["encounters"][7]
    assert wounded_fallback["conditions"] == {
        "locations": ["cheongryu_outer_courtyard"],
        "required_flags": [
            "cheongryu_raid_route_split_resolved",
            "route_commitment_deferred",
            "wounded_saved_flag",
            "cheongryu_raid_survived",
        ],
        "forbidden_flags": ["cheongryu_raid_wounded_fallback_resolved"],
    }
    assert wounded_fallback["presentation"]["layout"] == "wounded_fallback_route_pressure"
    assert wounded_fallback["presentation"]["effect_cues"][0]["stable_terms"] == [
        "부상자",
        "백도맹",
        "천기각",
    ]
    assert [choice["id"] for choice in wounded_fallback["choices"]] == [
        "stabilize_wounded_until_dawn",
        "ask_baekdo_for_medicine_not_command",
        "trade_black_heaven_bandages_for_exit",
        "follow_archive_triage_map",
    ]
    stabilize = wounded_fallback["choices"][0]
    assert stabilize["outcome"]["add_flags"] == [
        "cheongryu_raid_wounded_fallback_resolved",
        "deferred_route_reopened",
        "route_commitment_deferred",
        "wounded_shelter_stabilized",
        "survivor_roll_call_complete",
        "route_delay_cost_recorded",
    ]
    assert stabilize["outcome"]["add_clues"] == [
        "saving_people_changed_witnesses",
        "deferred_choice_is_still_choice",
    ]
    assert stabilize["outcome"]["destination_id"] == "cheongryu_outer_courtyard"
    baekdo_debt = bundle["content"]["encounters"][8]
    assert baekdo_debt["conditions"] == {
        "locations": ["cheongryu_outer_courtyard"],
        "required_flags": [
            "righteous_route_started",
            "cheongryu_rebuild_thread",
        ],
        "forbidden_flags": ["baekdo_medicine_debt_resolved"],
    }
    assert baekdo_debt["presentation"]["layout"] == "righteous_route_opener"
    assert baekdo_debt["presentation"]["speaker"] == "남궁서윤"
    assert baekdo_debt["presentation"]["effect_cues"][0]["stable_terms"] == [
        "약상자",
        "백도맹",
        "채무",
    ]
    assert [choice["id"] for choice in baekdo_debt["choices"]] == [
        "accept_medicine_with_written_debt",
        "ask_terms_before_opening_gate",
        "send_supplies_to_wounded_first",
        "compare_banner_to_record_margin",
    ]
    accept_debt = baekdo_debt["choices"][0]
    assert accept_debt["outcome"]["add_flags"] == [
        "baekdo_medicine_debt_resolved",
        "righteous_route_opened",
        "white_path_debt_recorded",
        "cheongryu_rebuild_supplies_secured",
        "namgung_seoyun_notice",
    ]
    assert accept_debt["outcome"]["add_clues"] == [
        "medicine_has_banner",
        "white_path_help_has_price",
        "qingliu_survival_needs_outside_help",
    ]
    assert accept_debt["outcome"]["destination_id"] == "cheongryu_outer_courtyard"
    assert "dev_desk" not in json.dumps(bundle, ensure_ascii=False)
    assert _missing_private_secret_fields(bundle)


def test_checked_in_wuxia_storypack_preview_bundle_is_up_to_date():
    exporter = _load_export_module()
    bundle_path = (
        ROOT
        / "crates"
        / "escape-core"
        / "fixtures"
        / "content"
        / "storypack-preview"
        / "wuxia_jianghu_pack.content.bundle.json"
    )
    web_bundle_path = (
        ROOT
        / "web"
        / "src"
        / "data"
        / "generated"
        / "storypack-preview"
        / "wuxia_jianghu_pack.content.bundle.json"
    )

    assert exporter.check_storypack_preview_bundle(ROOT, "wuxia_jianghu_pack", bundle_path) == []
    assert exporter.check_storypack_preview_bundle(ROOT, "wuxia_jianghu_pack", web_bundle_path) == []


def test_export_web_data_writes_and_checks_content_bundle(tmp_path):
    exporter = _load_export_module()
    bundle_path = tmp_path / "content.bundle.json"

    written = exporter.write_content_bundle(ROOT, bundle_path)

    assert written == bundle_path
    bundle = json.loads(bundle_path.read_text(encoding="utf-8"))
    assert bundle["kind"] == "tui_adv.content_bundle"
    assert bundle["manifest"]["counts"]["secrets"] == 3
    assert exporter.check_content_bundle(ROOT, bundle_path) == []
    bundle_path.write_text("{}\n", encoding="utf-8")
    assert exporter.check_content_bundle(ROOT, bundle_path) == [
        f"stale generated file: {bundle_path}"
    ]


def test_checked_in_content_bundle_is_up_to_date():
    exporter = _load_export_module()
    bundle_path = ROOT / "crates" / "escape-core" / "fixtures" / "content" / "content.bundle.json"

    assert exporter.check_content_bundle(ROOT, bundle_path) == []


def test_export_web_data_check_detects_stale_generated_files(tmp_path):
    exporter = _load_export_module()
    out_dir = tmp_path / "generated"

    errors = exporter.check_web_data(ROOT, out_dir)

    assert errors
    assert "missing generated file" in errors[0]


def test_export_web_data_cli_write_and_check_roundtrip(tmp_path):
    out_dir = tmp_path / "generated"
    rust_bundle_path = tmp_path / "rust" / "content.bundle.json"
    web_bundle_path = tmp_path / "web" / "content.bundle.json"

    write_result = subprocess.run(
        [
            sys.executable,
            str(SCRIPT_PATH),
            "--root",
            str(ROOT),
            "--out-dir",
            str(out_dir),
            "--bundle",
            str(rust_bundle_path),
            "--bundle",
            str(web_bundle_path),
            "--write",
        ],
        check=False,
        text=True,
        capture_output=True,
    )
    assert write_result.returncode == 0, write_result.stderr
    assert "wrote 7 web data files" in write_result.stdout
    assert f"wrote content bundle to {rust_bundle_path}" in write_result.stdout
    assert f"wrote content bundle to {web_bundle_path}" in write_result.stdout
    assert json.loads(rust_bundle_path.read_text(encoding="utf-8")) == json.loads(
        web_bundle_path.read_text(encoding="utf-8")
    )

    check_result = subprocess.run(
        [
            sys.executable,
            str(SCRIPT_PATH),
            "--root",
            str(ROOT),
            "--out-dir",
            str(out_dir),
            "--bundle",
            str(rust_bundle_path),
            "--bundle",
            str(web_bundle_path),
            "--check",
        ],
        check=False,
        text=True,
        capture_output=True,
    )
    assert check_result.returncode == 0, check_result.stdout + check_result.stderr
    assert "web data is up to date" in check_result.stdout
    assert check_result.stdout.count("content bundle is up to date") == 2


def test_export_web_data_cli_writes_and_checks_storypack_preview_bundles(tmp_path):
    rust_preview_bundle = tmp_path / "rust" / "storypack-preview" / "wuxia.bundle.json"
    web_preview_bundle = tmp_path / "web" / "storypack-preview" / "wuxia.bundle.json"

    write_result = subprocess.run(
        [
            sys.executable,
            str(SCRIPT_PATH),
            "--root",
            str(ROOT),
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--preview-bundle",
            str(rust_preview_bundle),
            "--preview-bundle",
            str(web_preview_bundle),
            "--write",
        ],
        check=False,
        text=True,
        capture_output=True,
    )
    assert write_result.returncode == 0, write_result.stdout + write_result.stderr
    assert f"wrote storypack preview bundle to {rust_preview_bundle}" in write_result.stdout
    assert f"wrote storypack preview bundle to {web_preview_bundle}" in write_result.stdout
    assert json.loads(rust_preview_bundle.read_text(encoding="utf-8")) == json.loads(
        web_preview_bundle.read_text(encoding="utf-8")
    )

    check_result = subprocess.run(
        [
            sys.executable,
            str(SCRIPT_PATH),
            "--root",
            str(ROOT),
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--preview-bundle",
            str(rust_preview_bundle),
            "--preview-bundle",
            str(web_preview_bundle),
            "--check",
        ],
        check=False,
        text=True,
        capture_output=True,
    )
    assert check_result.returncode == 0, check_result.stdout + check_result.stderr
    assert check_result.stdout.count("storypack preview bundle is up to date") == 2


def test_export_web_data_refuses_public_secret_final_hint(tmp_path):
    root = tmp_path / "repo"
    data_dir = root / "src" / "tui_adv" / "data"
    data_dir.mkdir(parents=True)
    for name, key in [
        ("locations", "locations"),
        ("items", "items"),
        ("encounters", "encounters"),
        ("endings", "endings"),
        ("achievements", "achievements"),
    ]:
        (data_dir / f"{name}.yaml").write_text(f"{key}: []\n", encoding="utf-8")
    (data_dir / "secrets.example.yaml").write_text(
        """
secrets:
  - id: unsafe
    title: unsafe
    final_hint: do not publish
""".strip()
        + "\n",
        encoding="utf-8",
    )
    exporter = _load_export_module()

    try:
        exporter.build_web_data(root)
    except ValueError as exc:
        assert "public secret unsafe has private-only field: final_hint" in str(exc)
    else:
        raise AssertionError("expected public final_hint to be rejected")

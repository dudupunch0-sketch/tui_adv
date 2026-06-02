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
            "wuxia_black_heaven_escape_price",
            "wuxia_heavenly_archive_previous_outsiders",
            "wuxia_wounded_shelter_dawn_offers",
            "wuxia_mumyeong_first_sighting",
            "wuxia_mumyeong_first_confrontation",
            "wuxia_mumyeong_copy_style_reveal",
            "wuxia_mumyeong_reads_orthodox_style",
            "wuxia_mumyeong_midgame_reunion",
            "wuxia_boss_first_appearance",
            "wuxia_mumyeong_request_for_aid",
            "wuxia_mumyeong_awakening",
            "wuxia_qingliu_attack_after_war",
            "wuxia_mumyeong_destroys_orthodox_sect",
            "wuxia_boss_recruits_mumyeong",
            "wuxia_mumyeong_departure_truth_summary",
            "wuxia_seoharin_empty_place",
            "wuxia_seoharin_left_meal",
            "wuxia_sado_final_phase_1_price_tag",
            "wuxia_sado_final_phase_2_weakpoint_control",
            "wuxia_sado_final_phase_3_outside_calculation",
            "wuxia_boss_resolution",
            "wuxia_mumyeong_resolution",
            "wuxia_seoharin_qingliu_resolution",
            "wuxia_cheongirok_resolution",
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
        "locations": 5,
        "items": 4,
        "encounters": 33,
        "endings": 1,
        "achievements": 2,
        "secrets": 0,
    }
    assert [location["id"] for location in bundle["content"]["locations"]] == [
        "wuxia_commute_rift",
        "jianghu_roadside",
        "jianghu_market_street",
        "cheongryu_outer_courtyard",
        "black_serpent_ledger_vault",
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
        "wuxia_black_heaven_escape_price",
        "wuxia_heavenly_archive_previous_outsiders",
        "wuxia_wounded_shelter_dawn_offers",
        "wuxia_mumyeong_first_sighting",
        "wuxia_mumyeong_first_confrontation",
        "wuxia_mumyeong_copy_style_reveal",
        "wuxia_mumyeong_reads_orthodox_style",
        "wuxia_mumyeong_midgame_reunion",
        "wuxia_boss_first_appearance",
        "wuxia_mumyeong_request_for_aid",
        "wuxia_mumyeong_awakening",
        "wuxia_qingliu_attack_after_war",
        "wuxia_mumyeong_destroys_orthodox_sect",
        "wuxia_boss_recruits_mumyeong",
        "wuxia_mumyeong_departure_truth_summary",
        "wuxia_seoharin_empty_place",
        "wuxia_seoharin_left_meal",
        "wuxia_sado_final_phase_1_price_tag",
        "wuxia_sado_final_phase_2_weakpoint_control",
        "wuxia_sado_final_phase_3_outside_calculation",
        "wuxia_boss_resolution",
        "wuxia_mumyeong_resolution",
        "wuxia_seoharin_qingliu_resolution",
        "wuxia_cheongirok_resolution",
    ]
    cheongirok = bundle["content"]["encounters"][32]
    assert cheongirok["conditions"] == {
        "locations": ["black_serpent_ledger_vault"],
        "required_flags": [
            "seoharin_qingliu_resolution_resolved",
            "mumyeong_resolution_resolved",
            "boss_resolution_resolved",
            "final_result_priority_applied_seeded",
            "final_combat_result_battle_victory_seeded",
            "final_state_routing_seeded",
        ],
        "forbidden_flags": ["cheongirok_resolution_resolved"],
    }
    assert cheongirok["presentation"]["layout"] == "cheongirok_resolution_seed"
    assert cheongirok["presentation"]["speaker"] == "천기록"
    assert cheongirok["presentation"]["effect_cues"][0]["stable_terms"] == [
        "천기록",
        "마지막 장",
        "빈칸",
        "기록자",
    ]
    assert [choice["id"] for choice in cheongirok["choices"]] == [
        "turn_the_last_page_without_question",
        "leave_blank_as_unpriced_place",
        "read_the_lines_that_align_like_ledger",
        "close_record_before_it_becomes_answer",
        "let_record_reflect_the_method",
    ]
    safe_last_page = cheongirok["choices"][0]
    assert safe_last_page["outcome"]["add_flags"] == [
        "cheongirok_resolution_resolved",
        "final_cheongirok_resolution_safe_high_use_seeded",
        "final_cheongirok_state_high_use_not_corruption_seeded",
        "final_epilogue_tianjilu_last_page_candidate_seeded",
        "final_epilogue_tianjilu_safe_high_use_variant_seeded",
    ]
    assert safe_last_page["outcome"]["add_clues"] == [
        "record_does_not_answer_questions",
        "high_use_is_not_corruption",
        "last_page_turns_without_identity_reveal",
    ]
    assert safe_last_page["outcome"]["destination_id"] == "black_serpent_ledger_vault"
    assert "add_items" not in safe_last_page["outcome"]
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
        "route_opener_resolved",
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
    black_heaven = bundle["content"]["encounters"][9]
    assert black_heaven["conditions"] == {
        "locations": ["cheongryu_outer_courtyard"],
        "required_flags": [
            "sapa_route_started",
            "dowol_debt",
        ],
        "forbidden_flags": ["black_heaven_escape_price_resolved"],
    }
    assert black_heaven["presentation"]["layout"] == "sapa_route_opener"
    assert black_heaven["presentation"]["speaker"] == "도월"
    assert black_heaven["presentation"]["effect_cues"][0]["stable_terms"] == [
        "탈출로",
        "흑천련",
        "값",
    ]
    assert [choice["id"] for choice in black_heaven["choices"]] == [
        "accept_dowol_marker_for_safehouse",
        "ask_who_collects_the_price",
        "keep_cheongryu_names_off_ledger",
        "map_exit_before_following_dowol",
    ]
    accept_marker = black_heaven["choices"][0]
    assert accept_marker["outcome"]["add_flags"] == [
        "black_heaven_escape_price_resolved",
        "sapa_route_opened",
        "route_opener_resolved",
        "black_heaven_safehouse_marked",
        "market_route_debt_recorded",
    ]
    assert accept_marker["outcome"]["add_clues"] == [
        "black_heaven_help_marks_debt",
        "survival_bargain_is_not_loyalty",
    ]
    assert accept_marker["outcome"]["destination_id"] == "cheongryu_outer_courtyard"
    heavenly_archive = bundle["content"]["encounters"][10]
    assert heavenly_archive["conditions"] == {
        "locations": ["cheongryu_outer_courtyard"],
        "required_flags": [
            "cheonggi_return_route_started",
            "cheonggi_record_targeted",
        ],
        "forbidden_flags": ["heavenly_archive_previous_outsiders_resolved"],
    }
    assert heavenly_archive["presentation"]["layout"] == "cheonggi_return_opener"
    assert heavenly_archive["presentation"]["speaker"] == "연소하"
    assert heavenly_archive["presentation"]["effect_cues"][0]["stable_terms"] == [
        "천기각",
        "이방인",
        "균열",
    ]
    assert [choice["id"] for choice in heavenly_archive["choices"]] == [
        "read_previous_outsider_margins",
        "ask_yeon_soha_what_not_to_read",
        "mark_current_worldline_without_answer",
        "compare_rift_terms_to_commute_memory",
    ]
    read_margins = heavenly_archive["choices"][0]
    assert read_margins["outcome"]["add_flags"] == [
        "heavenly_archive_previous_outsiders_resolved",
        "cheonggi_return_route_opened",
        "route_opener_resolved",
        "previous_outsiders_record_seen",
    ]
    assert read_margins["outcome"]["add_clues"] == [
        "archive_has_other_outsiders",
        "return_clue_is_not_return_method",
    ]
    assert read_margins["outcome"]["destination_id"] == "cheongryu_outer_courtyard"
    wounded_shelter = bundle["content"]["encounters"][11]
    assert wounded_shelter["conditions"] == {
        "locations": ["cheongryu_outer_courtyard"],
        "required_flags": [
            "cheongryu_raid_wounded_fallback_resolved",
            "route_commitment_deferred",
            "deferred_route_reopened",
            "wounded_shelter_stabilized",
        ],
        "forbidden_flags": ["wounded_shelter_dawn_offers_resolved"],
    }
    assert wounded_shelter["presentation"]["layout"] == "deferred_route_offer"
    assert wounded_shelter["presentation"]["speaker"] == "서하린"
    assert wounded_shelter["presentation"]["effect_cues"][0]["stable_terms"] == [
        "새벽",
        "부상자",
        "제안",
    ]
    assert [choice["id"] for choice in wounded_shelter["choices"]] == [
        "keep_wounded_shelter_until_noon",
        "accept_baekdo_medicine_after_roll_call",
        "send_word_to_dowol_for_quiet_exit",
        "show_archive_map_to_yeon_soha",
    ]
    keep_shelter = wounded_shelter["choices"][0]
    assert keep_shelter["outcome"]["add_flags"] == [
        "wounded_shelter_dawn_offers_resolved",
        "route_commitment_reopened",
        "wounded_shelter_until_noon",
        "deferred_offer_debt_recorded",
    ]
    assert keep_shelter["outcome"]["add_clues"] == [
        "saving_people_changed_witnesses",
        "care_is_not_route_escape",
        "dawn_shelter_keeps_names",
    ]
    assert keep_shelter["outcome"]["destination_id"] == "cheongryu_outer_courtyard"
    baekdo_reentry = wounded_shelter["choices"][1]
    assert baekdo_reentry["outcome"]["add_flags"] == [
        "wounded_shelter_dawn_offers_resolved",
        "route_commitment_reopened",
        "righteous_route_started",
        "cheongryu_rebuild_thread",
        "baekdo_medicine_debt",
    ]
    sapa_reentry = wounded_shelter["choices"][2]
    assert sapa_reentry["outcome"]["add_flags"] == [
        "wounded_shelter_dawn_offers_resolved",
        "route_commitment_reopened",
        "sapa_route_started",
        "dowol_debt",
        "black_heaven_escape_marker",
    ]
    cheonggi_reentry = wounded_shelter["choices"][3]
    assert cheonggi_reentry["outcome"]["add_flags"] == [
        "wounded_shelter_dawn_offers_resolved",
        "route_commitment_reopened",
        "cheonggi_return_route_started",
        "cheonggi_record_targeted",
        "heavenly_archive_triage_map_seen",
    ]
    mumyeong = bundle["content"]["encounters"][12]
    assert mumyeong["conditions"] == {
        "locations": ["cheongryu_outer_courtyard"],
        "required_flags": [
            "route_opener_resolved",
            "cheongryu_raid_survived",
            "cheongryu_trial_started",
            "first_fragment_seen",
        ],
        "forbidden_flags": ["mumyeong_first_sighting_resolved"],
    }
    assert mumyeong["presentation"]["layout"] == "midgame_rival_sighting"
    assert mumyeong["presentation"]["speaker"] == "서하린"
    assert mumyeong["presentation"]["effect_cues"][0]["stable_terms"] == [
        "무명",
        "청류문",
        "흑사방",
    ]
    assert [choice["id"] for choice in mumyeong["choices"]] == [
        "watch_the_stolen_qingliu_flow",
        "check_seo_harin_silence",
        "follow_black_serpent_runner",
        "pretend_not_to_see_the_form",
    ]
    observe = mumyeong["choices"][0]
    assert observe["outcome"]["add_flags"] == [
        "mumyeong_first_sighting_resolved",
        "midgame_continuity_started",
        "mumyeong_shadow_seen",
        "copied_qingliu_flow_noted",
    ]
    assert observe["outcome"]["add_clues"] == [
        "mumyeong_exists",
        "copied_flow_is_not_qingliu",
    ]
    assert observe["outcome"]["destination_id"] == "cheongryu_outer_courtyard"
    confrontation = bundle["content"]["encounters"][13]
    assert confrontation["conditions"] == {
        "locations": ["cheongryu_outer_courtyard"],
        "required_flags": [
            "mumyeong_first_sighting_resolved",
            "midgame_continuity_started",
            "cheongryu_raid_survived",
            "first_fragment_seen",
        ],
        "forbidden_flags": ["mumyeong_first_confrontation_resolved"],
    }
    assert confrontation["presentation"]["layout"] == "rival_first_confrontation"
    assert confrontation["presentation"]["speaker"] == "무명"
    assert confrontation["presentation"]["effect_cues"][0]["stable_terms"] == [
        "무명",
        "서하린",
        "청류문",
    ]
    assert [choice["id"] for choice in confrontation["choices"]] == [
        "meet_mumyeong_head_on",
        "endure_until_copy_flow_breaks",
        "watch_seo_harin_hold_back",
        "read_mumyeongs_copied_form",
        "do_not_provoke_mumyeong",
    ]
    endure = confrontation["choices"][1]
    assert endure["outcome"]["add_flags"] == [
        "mumyeong_first_confrontation_resolved",
        "mumyeong_rival_thread_opened",
        "copied_flow_weakness_noted",
    ]
    assert endure["outcome"]["add_clues"] == [
        "copy_style_has_gap",
        "copied_flow_is_not_qingliu",
    ]
    assert endure["outcome"]["destination_id"] == "cheongryu_outer_courtyard"
    copy_style = bundle["content"]["encounters"][14]
    assert copy_style["conditions"] == {
        "locations": ["cheongryu_outer_courtyard"],
        "required_flags": [
            "mumyeong_first_confrontation_resolved",
            "mumyeong_rival_thread_opened",
            "midgame_continuity_started",
        ],
        "forbidden_flags": ["mumyeong_copy_style_reveal_resolved"],
    }
    assert copy_style["presentation"]["layout"] == "copy_style_analysis"
    assert copy_style["presentation"]["speaker"] == "서하린"
    assert copy_style["presentation"]["effect_cues"][0]["stable_terms"] == [
        "무명",
        "청류안",
        "천기록",
    ]
    assert [choice["id"] for choice in copy_style["choices"]] == [
        "read_the_stolen_blade_path",
        "watch_mumyeongs_footwork",
        "listen_for_breath_mismatch",
        "wait_for_body_to_shudder",
    ]
    breath = copy_style["choices"][2]
    assert breath["outcome"]["add_flags"] == [
        "mumyeong_copy_style_reveal_resolved",
        "copy_style_hint_recorded",
        "copied_breath_mismatch_noted",
    ]
    assert breath["outcome"]["add_clues"] == [
        "breath_mismatch_marks_copy",
        "understanding_is_not_copying",
    ]
    assert breath["outcome"]["destination_id"] == "cheongryu_outer_courtyard"
    orthodox_style = bundle["content"]["encounters"][15]
    assert orthodox_style["conditions"] == {
        "locations": ["cheongryu_outer_courtyard"],
        "required_flags": [
            "mumyeong_copy_style_reveal_resolved",
            "copy_style_hint_recorded",
            "midgame_continuity_started",
            "first_fragment_seen",
        ],
        "forbidden_flags": ["mumyeong_reads_orthodox_style_resolved"],
    }
    assert orthodox_style["presentation"]["layout"] == "orthodox_style_trace"
    assert orthodox_style["presentation"]["speaker"] == "천기록"
    assert orthodox_style["presentation"]["effect_cues"][0]["stable_terms"] == [
        "현악문",
        "복호금쇄수",
        "무명",
    ]
    assert [choice["id"] for choice in orthodox_style["choices"]] == [
        "compare_copied_form_to_old_wound",
        "trace_qingliu_eye_variation",
        "reconstruct_mumyeongs_sightline",
        "stop_before_truth_becomes_accusation",
    ]
    reconstruct = orthodox_style["choices"][2]
    assert reconstruct["outcome"]["add_flags"] == [
        "mumyeong_reads_orthodox_style_resolved",
        "orthodox_style_trace_recorded",
        "mumyeong_sightline_reconstructed",
    ]
    assert reconstruct["outcome"]["add_clues"] == [
        "bokho_geumsaesu_name_recorded",
        "departure_truth_still_incomplete",
    ]
    assert reconstruct["outcome"]["destination_id"] == "cheongryu_outer_courtyard"
    midgame_reunion = bundle["content"]["encounters"][16]
    assert midgame_reunion["conditions"] == {
        "locations": ["cheongryu_outer_courtyard"],
        "required_flags": [
            "mumyeong_reads_orthodox_style_resolved",
            "orthodox_style_trace_recorded",
            "mumyeong_first_confrontation_resolved",
            "mumyeong_rival_thread_opened",
        ],
        "forbidden_flags": ["mumyeong_midgame_reunion_resolved"],
    }
    assert midgame_reunion["presentation"]["layout"] == "rival_reunion_trace"
    assert midgame_reunion["presentation"]["speaker"] == "무명"
    assert midgame_reunion["presentation"]["effect_cues"][0]["stable_terms"] == [
        "무명",
        "서하린",
        "현악문",
    ]
    assert [choice["id"] for choice in midgame_reunion["choices"]] == [
        "ask_why_seoharin_never_called_him_traitor",
        "show_the_hyeonakmun_trace_without_accusing",
        "point_out_the_copied_form_gap",
        "keep_blades_low_and_watch_his_answer",
    ]
    share_trace = midgame_reunion["choices"][1]
    assert share_trace["outcome"]["add_flags"] == [
        "mumyeong_midgame_reunion_resolved",
        "mumyeong_mirror_thread_deepened",
        "hyeonakmun_trace_shared_carefully",
    ]
    assert share_trace["outcome"]["add_clues"] == [
        "hyeonakmun_trace_shared_without_accusation",
        "boss_used_mumyeongs_wound",
    ]
    assert share_trace["outcome"]["destination_id"] == "cheongryu_outer_courtyard"
    boss = bundle["content"]["encounters"][17]
    assert boss["conditions"] == {
        "locations": ["cheongryu_outer_courtyard"],
        "required_flags": [
            "mumyeong_midgame_reunion_resolved",
            "mumyeong_mirror_thread_deepened",
            "cheongryu_raid_survived",
            "midgame_continuity_started",
        ],
        "forbidden_flags": ["boss_first_appearance_resolved"],
    }
    assert boss["presentation"]["layout"] == "boss_wall_pressure"
    assert boss["presentation"]["speaker"] == "흑사방주"
    assert boss["presentation"]["effect_cues"][0]["stable_terms"] == [
        "흑사방주",
        "무명",
        "청류문",
    ]
    assert [choice["id"] for choice in boss["choices"]] == [
        "read_the_boss_flow_and_fail_to_move",
        "pull_seo_harin_behind_broken_gate",
        "watch_mumyeong_answer_the_boss",
        "retreat_before_the_second_step",
    ]
    watch_mumyeong = boss["choices"][2]
    assert watch_mumyeong["outcome"]["add_flags"] == [
        "boss_first_appearance_resolved",
        "boss_wall_thread_opened",
        "black_serpent_core_pressure_opened",
        "mumyeong_reacts_to_boss_voice",
    ]
    assert watch_mumyeong["outcome"]["add_clues"] == [
        "mumyeong_follows_power_that_saw_his_wound",
        "boss_reads_people_not_forms",
    ]
    assert watch_mumyeong["outcome"]["destination_id"] == "cheongryu_outer_courtyard"
    request_for_aid = bundle["content"]["encounters"][18]
    assert request_for_aid["conditions"] == {
        "locations": ["cheongryu_outer_courtyard"],
        "required_flags": [
            "boss_first_appearance_resolved",
            "boss_wall_thread_opened",
            "black_serpent_core_pressure_opened",
            "mumyeong_mirror_thread_deepened",
            "orthodox_style_trace_recorded",
            "midgame_continuity_started",
        ],
        "forbidden_flags": ["mumyeong_request_for_aid_resolved"],
    }
    assert request_for_aid["presentation"]["layout"] == "failed_aid_records"
    assert request_for_aid["presentation"]["speaker"] == "천기록"
    assert request_for_aid["presentation"]["effect_cues"][0]["stable_terms"] == [
        "무명",
        "청류문",
        "정파",
    ]
    assert [choice["id"] for choice in request_for_aid["choices"]] == [
        "search_the_rejected_aid_letters",
        "follow_old_inn_rumors_about_mumyeong",
        "ask_seo_harin_what_help_never_came",
        "keep_the_failed_aid_record_unshown",
    ]
    rejected_letters = request_for_aid["choices"][0]
    assert rejected_letters["outcome"]["add_items"] == ["rejected_aid_letter_fragment"]
    assert rejected_letters["outcome"]["add_flags"] == [
        "mumyeong_request_for_aid_resolved",
        "mumyeong_failed_aid_thread_opened",
        "orthodox_hypocrisy_thread_opened",
        "rejected_aid_letters_read",
    ]
    assert rejected_letters["outcome"]["add_clues"] == [
        "mumyeong_tried_to_save_qingliu",
        "orthodox_refusal_broke_mumyeong",
    ]
    assert rejected_letters["outcome"]["destination_id"] == "cheongryu_outer_courtyard"
    awakening = bundle["content"]["encounters"][19]
    assert awakening["conditions"] == {
        "locations": ["cheongryu_outer_courtyard"],
        "required_flags": [
            "mumyeong_request_for_aid_resolved",
            "mumyeong_failed_aid_thread_opened",
            "orthodox_hypocrisy_thread_opened",
            "mumyeong_reads_orthodox_style_resolved",
            "orthodox_style_trace_recorded",
            "mumyeong_copy_style_reveal_resolved",
            "copy_style_hint_recorded",
            "midgame_continuity_started",
        ],
        "forbidden_flags": ["mumyeong_awakening_resolved"],
    }
    assert awakening["presentation"]["layout"] == "anger_copy_bloom"
    assert awakening["presentation"]["speaker"] == "천기록"
    assert awakening["presentation"]["effect_cues"][0]["stable_terms"] == [
        "무명",
        "카피",
        "분노",
    ]
    assert [choice["id"] for choice in awakening["choices"]] == [
        "compare_anger_to_copied_flow",
        "trace_awakening_from_failed_aid",
        "ask_what_the_copy_cost_him",
        "stop_before_calling_it_salvation",
    ]
    compare = awakening["choices"][0]
    assert compare["outcome"]["add_flags"] == [
        "mumyeong_awakening_resolved",
        "mumyeong_awakening_thread_opened",
        "copy_corruption_thread_opened",
    ]
    assert compare["outcome"]["add_clues"] == [
        "mumyeong_copy_bloomed_from_anger",
        "copy_is_wound_not_growth",
    ]
    assert compare["outcome"]["destination_id"] == "cheongryu_outer_courtyard"
    qingliu_attack = bundle["content"]["encounters"][20]
    assert qingliu_attack["conditions"] == {
        "locations": ["cheongryu_outer_courtyard"],
        "required_flags": [
            "mumyeong_awakening_resolved",
            "mumyeong_awakening_thread_opened",
            "copy_corruption_thread_opened",
            "mumyeong_request_for_aid_resolved",
            "mumyeong_failed_aid_thread_opened",
            "orthodox_hypocrisy_thread_opened",
            "mumyeong_reads_orthodox_style_resolved",
            "orthodox_style_trace_recorded",
            "midgame_continuity_started",
        ],
        "forbidden_flags": ["qingliu_attack_after_war_resolved"],
    }
    assert qingliu_attack["presentation"]["layout"] == "attack_trace_investigation"
    assert qingliu_attack["presentation"]["speaker"] == "천기록"
    assert qingliu_attack["presentation"]["effect_cues"][0]["stable_terms"] == [
        "청류문",
        "현악문",
        "복호금쇄수",
    ]
    assert [choice["id"] for choice in qingliu_attack["choices"]] == [
        "inspect_bokho_lock_scars",
        "compare_hyeonakmun_trace_to_qingliu_wounds",
        "ask_seo_harin_what_she_saw_afterward",
        "stop_before_replaying_the_attack",
    ]
    lock_scars = qingliu_attack["choices"][0]
    assert lock_scars["outcome"]["add_flags"] == [
        "qingliu_attack_after_war_resolved",
        "qingliu_attack_trace_confirmed",
        "hyeonakmun_attack_thread_opened",
    ]
    assert lock_scars["outcome"]["add_clues"] == [
        "bokho_geumsaesu_used_on_qingliu",
        "full_flashback_still_unopened",
    ]
    assert lock_scars["outcome"]["destination_id"] == "cheongryu_outer_courtyard"
    destroys_orthodox = bundle["content"]["encounters"][21]
    assert destroys_orthodox["conditions"] == {
        "locations": ["cheongryu_outer_courtyard"],
        "required_flags": [
            "qingliu_attack_after_war_resolved",
            "qingliu_attack_trace_confirmed",
            "hyeonakmun_attack_thread_opened",
            "mumyeong_awakening_resolved",
            "midgame_continuity_started",
        ],
        "forbidden_flags": ["mumyeong_destroys_orthodox_sect_resolved"],
    }
    assert destroys_orthodox["presentation"]["layout"] == "hyeonakmun_empty_gate_record"
    assert destroys_orthodox["presentation"]["speaker"] == "천기록"
    assert destroys_orthodox["presentation"]["effect_cues"][0]["stable_terms"] == [
        "현악문",
        "복호금쇄수",
        "무명",
    ]
    assert [choice["id"] for choice in destroys_orthodox["choices"]] == [
        "read_hyeonakmun_empty_gate_record",
        "trace_bokho_lock_to_mumyeong",
        "ask_why_seoharin_never_heard_full_story",
        "stop_before_counting_the_dead",
    ]
    read_record = destroys_orthodox["choices"][0]
    assert read_record["outcome"]["add_flags"] == [
        "mumyeong_destroys_orthodox_sect_resolved",
        "hyeonakmun_destruction_thread_opened",
        "departure_truth_thread_deepened",
    ]
    assert read_record["outcome"]["add_clues"] == [
        "hyeonakmun_was_destroyed_after_qingliu_attack",
        "destruction_is_consequence_not_salvation",
    ]
    assert read_record["outcome"]["destination_id"] == "cheongryu_outer_courtyard"
    boss_recruit = bundle["content"]["encounters"][22]
    assert boss_recruit["conditions"] == {
        "locations": ["cheongryu_outer_courtyard"],
        "required_flags": [
            "mumyeong_destroys_orthodox_sect_resolved",
            "hyeonakmun_destruction_thread_opened",
            "departure_truth_thread_deepened",
            "boss_first_appearance_resolved",
            "boss_wall_thread_opened",
            "black_serpent_core_pressure_opened",
            "midgame_continuity_started",
        ],
        "forbidden_flags": ["boss_recruits_mumyeong_resolved"],
    }
    assert boss_recruit["presentation"]["layout"] == "boss_recruitment_trace"
    assert boss_recruit["presentation"]["speaker"] == "천기록"
    assert boss_recruit["presentation"]["effect_cues"][0]["stable_terms"] == [
        "흑사방주",
        "무명",
        "현악문",
    ]
    assert [choice["id"] for choice in boss_recruit["choices"]] == [
        "trace_boss_offer_after_hyeonakmun",
        "read_mumyeong_choice_without_excusing_it",
        "search_black_serpent_recruitment_record",
        "stop_before_following_him_into_black_serpent",
    ]
    trace_offer = boss_recruit["choices"][0]
    assert trace_offer["outcome"]["add_flags"] == [
        "boss_recruits_mumyeong_resolved",
        "boss_recruitment_thread_opened",
        "boss_saw_mumyeongs_wound",
    ]
    assert trace_offer["outcome"]["add_clues"] == [
        "boss_recruited_mumyeong_after_hyeonakmun",
        "recruitment_was_not_salvation",
    ]
    assert trace_offer["outcome"]["destination_id"] == "cheongryu_outer_courtyard"
    price_tag = bundle["content"]["encounters"][26]
    assert price_tag["conditions"] == {
        "locations": ["cheongryu_outer_courtyard"],
        "required_flags": [
            "seoharin_left_meal_resolved",
            "seoharin_empty_place_resolved",
            "seoharin_axis_opened",
            "empty_place_remembered",
            "truth_delivery_still_unopened",
            "boss_recruits_mumyeong_resolved",
            "boss_recruitment_thread_opened",
            "boss_first_appearance_resolved",
            "black_serpent_core_pressure_opened",
            "sealed_departure_truth_summary_prepared",
            "midgame_continuity_started",
        ],
        "forbidden_flags": ["sado_final_phase_1_price_tag_resolved"],
    }
    assert price_tag["presentation"]["layout"] == "final_phase_price_tag"
    assert price_tag["presentation"]["speaker"] == "흑사방주"
    assert price_tag["presentation"]["effect_cues"][0]["stable_terms"] == [
        "흑사방주",
        "장부",
        "빚",
        "청류문",
    ]
    assert [choice["id"] for choice in price_tag["choices"]] == [
        "approach_sado_before_the_ledger",
        "burn_the_blackscale_ledger",
        "secure_the_blackscale_ledger",
        "ease_hostage_pressure_first",
    ]
    secure_ledger = price_tag["choices"][2]
    assert secure_ledger["outcome"]["add_flags"] == [
        "sado_final_phase_1_price_tag_resolved",
        "final_state_routing_seeded",
        "final_price_tag_ledger_secured",
        "final_network_ledger_secured_seeded",
        "final_evidence_strong_seeded",
        "final_item_logs_blackscale_ledger_seeded",
    ]
    assert secure_ledger["outcome"]["add_clues"] == [
        "item_blackscale_ledger_logged",
        "black_serpent_network_structure_seen",
        "alliance_silence_accountability_seeded",
    ]
    assert secure_ledger["outcome"]["destination_id"] == "black_serpent_ledger_vault"
    weakpoint = bundle["content"]["encounters"][27]
    assert weakpoint["conditions"] == {
        "locations": ["black_serpent_ledger_vault"],
        "required_flags": [
            "sado_final_phase_1_price_tag_resolved",
            "final_state_routing_seeded",
        ],
        "forbidden_flags": ["sado_final_phase_2_weakpoint_control_resolved"],
    }
    assert weakpoint["presentation"]["layout"] == "final_phase_weakpoint_control"
    assert weakpoint["presentation"]["speaker"] == "흑사방주"
    assert weakpoint["presentation"]["effect_cues"][0]["stable_terms"] == [
        "서하린",
        "무명",
        "천기록",
        "약점",
    ]
    assert [choice["id"] for choice in weakpoint["choices"]] == [
        "respond_to_seoharin_pressure",
        "return_flow_to_mumyeong",
        "read_dangerous_cheongirok_sentence",
        "focus_on_sado",
    ]
    return_flow = weakpoint["choices"][1]
    assert return_flow["outcome"]["add_flags"] == [
        "sado_final_phase_2_weakpoint_control_resolved",
        "final_phase_2_weakpoint_control_resolved",
        "final_mumyeong_salvation_partial_seeded",
        "final_successor_route_suppressed_seeded",
        "final_own_flow_choice_opened_seeded",
        "final_player_method_protected_as_person_seeded",
    ]
    assert return_flow["outcome"]["add_clues"] == [
        "mumyeong_flow_is_not_tool",
        "successor_logic_wavers",
        "stolen_form_can_stop",
    ]
    assert return_flow["outcome"]["destination_id"] == "black_serpent_ledger_vault"
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

from __future__ import annotations

import json
from pathlib import Path

from tui_adv.game.storypack_db import load_storypack_db, validate_storypack_db

ROOT = Path(__file__).resolve().parents[1]


def test_storypack_db_loads_office_wuxia_and_yageunmong_candidate_records():
    db = load_storypack_db(ROOT)

    assert sorted(db.storypacks) == ["isolation_pack", "wuxia_jianghu_pack", "yageunmong_pack"]
    assert db.storypacks["isolation_pack"].world_id == "office_apocalypse"
    assert db.storypacks["yageunmong_pack"].world_id == "office_dream"
    assert db.storypacks["yageunmong_pack"].name == "야근몽"
    assert "approval_system" in db.storypacks["yageunmong_pack"].main_surfaces
    assert "build_log" in db.storypacks["yageunmong_pack"].main_surfaces
    assert db.storypacks["wuxia_jianghu_pack"].world_id == "wuxia_jianghu"
    assert db.storypacks["wuxia_jianghu_pack"].name == "이구학지 — 천기록"
    assert "cheonggi_record" in db.storypacks["wuxia_jianghu_pack"].main_surfaces

    assert sorted(db.cards_by_storypack("isolation_pack")) == [
        "automatic_minutes_no_attendees",
        "delayed_cctv_next_action",
        "isolation_channel_mismatched_floor",
        "org_chart_missing_employee",
        "pantry_survivor_trace",
        "server_log_other_branch",
    ]
    assert sorted(db.cards_by_storypack("yageunmong_pack")) == [
        "yageunmong_awakening_fragment_choice",
        "yageunmong_clockout_gate_route_split",
        "yageunmong_clockout_gate_self",
        "yageunmong_coworker_meeting_room_rescue",
        "yageunmong_dead_project_server_log",
        "yageunmong_elevator_unapproved_floor",
        "yageunmong_late_night_desk_awake",
        "yageunmong_manager_approval_trap",
        "yageunmong_reality_anchor_pantry",
        "yageunmong_unapproved_meeting_room_loop",
        "yageunmong_unread_mail_wall",
        "yageunmong_wake_desk_aftermath",
    ]
    assert sorted(db.cards_by_storypack("wuxia_jianghu_pack")) == [
        "wuxia_baekdo_medicine_debt",
        "wuxia_black_heaven_escape_price",
        "wuxia_boss_first_appearance",
        "wuxia_boss_recruits_mumyeong",
        "wuxia_boss_resolution",
        "wuxia_cheonggi_record_first_fragment",
        "wuxia_cheongirok_resolution",
        "wuxia_cheongryu_apprentice_entry",
        "wuxia_cheongryu_chore_sparring",
        "wuxia_cheongryu_raid_route_split",
        "wuxia_cheongryu_raid_wounded_fallback",
        "wuxia_commute_rift_arrival",
        "wuxia_heavenly_archive_previous_outsiders",
        "wuxia_heuksa_bang_first_fight",
        "wuxia_mumyeong_awakening",
        "wuxia_mumyeong_copy_style_reveal",
        "wuxia_mumyeong_departure_truth_summary",
        "wuxia_mumyeong_destroys_orthodox_sect",
        "wuxia_mumyeong_first_confrontation",
        "wuxia_mumyeong_first_sighting",
        "wuxia_mumyeong_midgame_reunion",
        "wuxia_mumyeong_reads_orthodox_style",
        "wuxia_mumyeong_request_for_aid",
        "wuxia_mumyeong_resolution",
        "wuxia_qingliu_attack_after_war",
        "wuxia_sado_final_phase_1_price_tag",
        "wuxia_sado_final_phase_2_weakpoint_control",
        "wuxia_sado_final_phase_3_outside_calculation",
        "wuxia_seo_harin_rescue",
        "wuxia_seoharin_empty_place",
        "wuxia_seoharin_left_meal",
        "wuxia_seoharin_qingliu_resolution",
        "wuxia_wounded_shelter_dawn_offers",
    ]

    yageun_opening = db.encounter_cards["yageunmong_late_night_desk_awake"]
    assert yageun_opening.world_id == "office_dream"
    assert yageun_opening.storypack_id == "yageunmong_pack"
    assert "lucid_dream_awareness" in yageun_opening.phases
    assert "safe_observe" in [choice["role"] for choice in yageun_opening.choice_shapes]
    assert "yageunmong_started" in yageun_opening.outcome_hooks["possible_flags"]

    coworker_rescue = db.encounter_cards["yageunmong_coworker_meeting_room_rescue"]
    assert coworker_rescue.priority_class == "npc_relation"
    assert "meeting_minutes" in coworker_rescue.surfaces
    assert "safe_honesty" in [choice["role"] for choice in coworker_rescue.choice_shapes]
    assert "coworker_rescue_thread" in coworker_rescue.outcome_hooks["possible_relations"]

    yageun_route_split = db.encounter_cards["yageunmong_clockout_gate_route_split"]
    assert "wake_resolution" in yageun_route_split.phases
    assert "wake_up_route_started" in yageun_route_split.outcome_hooks["possible_route_flags"]

    arrival = db.encounter_cards["wuxia_commute_rift_arrival"]
    assert arrival.world_id == "wuxia_jianghu"
    assert arrival.storypack_id == "wuxia_jianghu_pack"
    assert arrival.priority_class == "main_forced"
    assert "market_arrival" in arrival.phases
    assert "safe_observe" in [choice["role"] for choice in arrival.choice_shapes]
    assert "wuxia_arrival_confirmed" in arrival.outcome_hooks["possible_flags"]

    black_heaven = db.encounter_cards["wuxia_black_heaven_escape_price"]
    assert black_heaven.world_id == "wuxia_jianghu"
    assert black_heaven.storypack_id == "wuxia_jianghu_pack"
    assert black_heaven.priority_class == "route_key"
    assert "route_commitment" in black_heaven.phases
    assert "safe_acceptance" in [choice["role"] for choice in black_heaven.choice_shapes]
    assert "sapa_route_opened" in black_heaven.outcome_hooks["possible_flags"]
    assert "black_heaven_bargain_has_teeth" in black_heaven.outcome_hooks["possible_clues"]

    heavenly_archive = db.encounter_cards["wuxia_heavenly_archive_previous_outsiders"]
    assert heavenly_archive.world_id == "wuxia_jianghu"
    assert heavenly_archive.storypack_id == "wuxia_jianghu_pack"
    assert heavenly_archive.priority_class == "route_key"
    assert "cheonggi_return" in heavenly_archive.phases
    assert "cheonggi_record" in heavenly_archive.surfaces
    assert "safe_reading" in [choice["role"] for choice in heavenly_archive.choice_shapes]
    assert "cheonggi_return_route_opened" in heavenly_archive.outcome_hooks["possible_flags"]
    assert "return_clue_is_not_return_method" in heavenly_archive.outcome_hooks["possible_clues"]

    wounded_shelter = db.encounter_cards["wuxia_wounded_shelter_dawn_offers"]
    assert wounded_shelter.world_id == "wuxia_jianghu"
    assert wounded_shelter.storypack_id == "wuxia_jianghu_pack"
    assert wounded_shelter.priority_class == "route_key"
    assert "route_commitment" in wounded_shelter.phases
    assert "sect_courtyard" in wounded_shelter.surfaces
    assert "safe_care" in [choice["role"] for choice in wounded_shelter.choice_shapes]
    assert "route_commitment_reopened" in wounded_shelter.outcome_hooks["possible_flags"]
    assert "offers_arrive_because_people_lived" in wounded_shelter.outcome_hooks["possible_clues"]

    mumyeong_sighting = db.encounter_cards["wuxia_mumyeong_first_sighting"]
    assert mumyeong_sighting.world_id == "wuxia_jianghu"
    assert mumyeong_sighting.storypack_id == "wuxia_jianghu_pack"
    assert mumyeong_sighting.priority_class == "route_key"
    assert "midgame_rival" in mumyeong_sighting.phases
    assert "sect_courtyard" in mumyeong_sighting.surfaces
    assert "safe_observe" in [choice["role"] for choice in mumyeong_sighting.choice_shapes]
    assert "midgame_continuity_started" in mumyeong_sighting.outcome_hooks["possible_flags"]
    assert "mumyeong_exists" in mumyeong_sighting.outcome_hooks["possible_clues"]

    mumyeong_confrontation = db.encounter_cards["wuxia_mumyeong_first_confrontation"]
    assert mumyeong_confrontation.world_id == "wuxia_jianghu"
    assert mumyeong_confrontation.storypack_id == "wuxia_jianghu_pack"
    assert mumyeong_confrontation.priority_class == "route_key"
    assert "rival_confrontation" in mumyeong_confrontation.phases
    assert "sect_courtyard" in mumyeong_confrontation.surfaces
    assert "safe_endure" in [choice["role"] for choice in mumyeong_confrontation.choice_shapes]
    assert (
        "mumyeong_first_confrontation_resolved"
        in mumyeong_confrontation.outcome_hooks["possible_flags"]
    )
    assert "winning_is_not_required" in mumyeong_confrontation.outcome_hooks["possible_clues"]

    copy_style_reveal = db.encounter_cards["wuxia_mumyeong_copy_style_reveal"]
    assert copy_style_reveal.world_id == "wuxia_jianghu"
    assert copy_style_reveal.storypack_id == "wuxia_jianghu_pack"
    assert copy_style_reveal.priority_class == "route_key"
    assert "copy_style_analysis" in copy_style_reveal.phases
    assert "cheonggi_record" in copy_style_reveal.surfaces
    assert "safe_observe" in [choice["role"] for choice in copy_style_reveal.choice_shapes]
    assert (
        "mumyeong_copy_style_reveal_resolved"
        in copy_style_reveal.outcome_hooks["possible_flags"]
    )
    assert "copy_is_surface_not_root" in copy_style_reveal.outcome_hooks["possible_clues"]

    orthodox_style = db.encounter_cards["wuxia_mumyeong_reads_orthodox_style"]
    assert orthodox_style.world_id == "wuxia_jianghu"
    assert orthodox_style.storypack_id == "wuxia_jianghu_pack"
    assert orthodox_style.priority_class == "route_key"
    assert "orthodox_style_trace" in orthodox_style.phases
    assert "cheonggi_record" in orthodox_style.surfaces
    assert "safe_observe" in [choice["role"] for choice in orthodox_style.choice_shapes]
    assert (
        "mumyeong_reads_orthodox_style_resolved"
        in orthodox_style.outcome_hooks["possible_flags"]
    )
    assert "bokho_geumsaesu_name_recorded" in orthodox_style.outcome_hooks["possible_clues"]

    midgame_reunion = db.encounter_cards["wuxia_mumyeong_midgame_reunion"]
    assert midgame_reunion.world_id == "wuxia_jianghu"
    assert midgame_reunion.storypack_id == "wuxia_jianghu_pack"
    assert midgame_reunion.priority_class == "route_key"
    assert "rival_reunion" in midgame_reunion.phases
    assert "sect_courtyard" in midgame_reunion.surfaces
    assert "safe_observe" in [choice["role"] for choice in midgame_reunion.choice_shapes]
    assert (
        "mumyeong_midgame_reunion_resolved"
        in midgame_reunion.outcome_hooks["possible_flags"]
    )
    assert "boss_used_mumyeongs_wound" in midgame_reunion.outcome_hooks["possible_clues"]

    boss_first_appearance = db.encounter_cards["wuxia_boss_first_appearance"]
    assert boss_first_appearance.world_id == "wuxia_jianghu"
    assert boss_first_appearance.storypack_id == "wuxia_jianghu_pack"
    assert boss_first_appearance.priority_class == "route_key"
    assert "boss_wall_pressure" in boss_first_appearance.phases
    assert "faction_negotiation" in boss_first_appearance.surfaces
    assert "safe_reposition" in [choice["role"] for choice in boss_first_appearance.choice_shapes]
    assert (
        "boss_first_appearance_resolved"
        in boss_first_appearance.outcome_hooks["possible_flags"]
    )
    assert "boss_reads_people_not_forms" in boss_first_appearance.outcome_hooks["possible_clues"]

    request_for_aid = db.encounter_cards["wuxia_mumyeong_request_for_aid"]
    assert request_for_aid.world_id == "wuxia_jianghu"
    assert request_for_aid.storypack_id == "wuxia_jianghu_pack"
    assert request_for_aid.priority_class == "route_key"
    assert "failed_aid_records" in request_for_aid.phases
    assert "faction_negotiation" in request_for_aid.surfaces
    assert "safe_reading" in [choice["role"] for choice in request_for_aid.choice_shapes]
    assert (
        "mumyeong_request_for_aid_resolved"
        in request_for_aid.outcome_hooks["possible_flags"]
    )
    assert "mumyeong_tried_to_save_qingliu" in request_for_aid.outcome_hooks[
        "possible_clues"
    ]

    qingliu_attack = db.encounter_cards["wuxia_qingliu_attack_after_war"]
    assert qingliu_attack.world_id == "wuxia_jianghu"
    assert qingliu_attack.storypack_id == "wuxia_jianghu_pack"
    assert qingliu_attack.priority_class == "route_key"
    assert "attack_trace_investigation" in qingliu_attack.phases
    assert "cheonggi_record" in qingliu_attack.surfaces
    assert "safe_observe" in [choice["role"] for choice in qingliu_attack.choice_shapes]
    assert (
        "qingliu_attack_trace_confirmed"
        in qingliu_attack.outcome_hooks["possible_flags"]
    )
    assert "bokho_geumsaesu_used_on_qingliu" in qingliu_attack.outcome_hooks[
        "possible_clues"
    ]

    destroys_orthodox = db.encounter_cards["wuxia_mumyeong_destroys_orthodox_sect"]
    assert destroys_orthodox.world_id == "wuxia_jianghu"
    assert destroys_orthodox.storypack_id == "wuxia_jianghu_pack"
    assert destroys_orthodox.priority_class == "route_key"
    assert "hyeonakmun_consequence_trace" in destroys_orthodox.phases
    assert "cheonggi_record" in destroys_orthodox.surfaces
    assert "safe_reading" in [
        choice["role"] for choice in destroys_orthodox.choice_shapes
    ]
    assert (
        "mumyeong_destroys_orthodox_sect_resolved"
        in destroys_orthodox.outcome_hooks["possible_flags"]
    )
    assert "destruction_is_consequence_not_salvation" in destroys_orthodox.outcome_hooks[
        "possible_clues"
    ]

    boss_recruits = db.encounter_cards["wuxia_boss_recruits_mumyeong"]
    assert boss_recruits.world_id == "wuxia_jianghu"
    assert boss_recruits.storypack_id == "wuxia_jianghu_pack"
    assert boss_recruits.priority_class == "route_key"
    assert "boss_recruitment_trace" in boss_recruits.phases
    assert "faction_negotiation" in boss_recruits.surfaces
    assert "safe_trace" in [choice["role"] for choice in boss_recruits.choice_shapes]
    assert (
        "boss_recruits_mumyeong_resolved"
        in boss_recruits.outcome_hooks["possible_flags"]
    )
    assert "recruitment_was_not_salvation" in boss_recruits.outcome_hooks[
        "possible_clues"
    ]

    departure_truth = db.encounter_cards["wuxia_mumyeong_departure_truth_summary"]
    assert departure_truth.world_id == "wuxia_jianghu"
    assert departure_truth.storypack_id == "wuxia_jianghu_pack"
    assert departure_truth.priority_class == "route_key"
    assert "sealed_departure_truth_summary" in departure_truth.phases
    assert "cheonggi_record" in departure_truth.surfaces
    assert "safe_trace" in [choice["role"] for choice in departure_truth.choice_shapes]
    assert (
        "sealed_departure_truth_summary_prepared"
        in departure_truth.outcome_hooks["possible_flags"]
    )
    assert "salvation_condition_seen_but_unmet" in departure_truth.outcome_hooks[
        "possible_clues"
    ]

    empty_place = db.encounter_cards["wuxia_seoharin_empty_place"]
    assert empty_place.world_id == "wuxia_jianghu"
    assert empty_place.storypack_id == "wuxia_jianghu_pack"
    assert empty_place.status == "implemented_in_storypack_preview"
    assert empty_place.priority_class == "npc_relation"
    assert "seoharin_empty_place_bridge" in empty_place.phases
    assert "training_chore" in empty_place.surfaces
    assert "safe_observe" in [choice["role"] for choice in empty_place.choice_shapes]
    assert "seoharin_axis_opened" in empty_place.outcome_hooks["possible_flags"]
    assert "empty_place_is_return_not_claim" in empty_place.outcome_hooks[
        "possible_clues"
    ]

    left_meal = db.encounter_cards["wuxia_seoharin_left_meal"]
    assert left_meal.world_id == "wuxia_jianghu"
    assert left_meal.storypack_id == "wuxia_jianghu_pack"
    assert left_meal.status == "implemented_in_storypack_preview"
    assert left_meal.priority_class == "npc_relation"
    assert "seoharin_belonging_bridge" in left_meal.phases
    assert "food" in left_meal.surfaces
    assert "daily_care" in left_meal.surfaces
    assert "safe_stop" in [choice["role"] for choice in left_meal.choice_shapes]
    assert "seoharin_axis_deepened" in left_meal.outcome_hooks["possible_flags"]
    assert "belonging_is_daily_care" in left_meal.outcome_hooks["possible_clues"]

    price_tag = db.encounter_cards["wuxia_sado_final_phase_1_price_tag"]
    assert price_tag.world_id == "wuxia_jianghu"
    assert price_tag.storypack_id == "wuxia_jianghu_pack"
    assert price_tag.status == "implemented_in_storypack_preview"
    assert price_tag.priority_class == "route_key"
    assert "price_tag_ledger_phase" in price_tag.phases
    assert "faction_negotiation" in price_tag.surfaces
    assert "safe_pressure_relief" in [choice["role"] for choice in price_tag.choice_shapes]
    assert "final_evidence_strong_seeded" in price_tag.outcome_hooks["possible_flags"]
    assert "item_blackscale_ledger_logged" in price_tag.outcome_hooks["possible_clues"]

    weakpoint = db.encounter_cards["wuxia_sado_final_phase_2_weakpoint_control"]
    assert weakpoint.world_id == "wuxia_jianghu"
    assert weakpoint.storypack_id == "wuxia_jianghu_pack"
    assert weakpoint.status == "implemented_in_storypack_preview"
    assert weakpoint.priority_class == "route_key"
    assert "weakpoint_control_phase" in weakpoint.phases
    assert "faction_negotiation" in weakpoint.surfaces
    assert "safe_pressure_relief" in [choice["role"] for choice in weakpoint.choice_shapes]
    assert "final_mumyeong_salvation_partial_seeded" in weakpoint.outcome_hooks["possible_flags"]
    assert "cheongirok_understanding_not_calculation" in weakpoint.outcome_hooks["possible_clues"]

    outside_calculation = db.encounter_cards[
        "wuxia_sado_final_phase_3_outside_calculation"
    ]
    assert outside_calculation.world_id == "wuxia_jianghu"
    assert outside_calculation.storypack_id == "wuxia_jianghu_pack"
    assert outside_calculation.status == "implemented_in_storypack_preview"
    assert outside_calculation.priority_class == "route_key"
    assert "outside_calculation_phase" in outside_calculation.phases
    assert "faction_negotiation" in outside_calculation.surfaces
    assert "safe_identity_return" in [
        choice["role"] for choice in outside_calculation.choice_shapes
    ]
    assert (
        "final_boss_resolution_true_route_candidate_seeded"
        in outside_calculation.outcome_hooks["possible_flags"]
    )
    assert (
        "sado_calculation_fails_to_price_waiting"
        in outside_calculation.outcome_hooks["possible_clues"]
    )

    boss_resolution = db.encounter_cards["wuxia_boss_resolution"]
    assert boss_resolution.world_id == "wuxia_jianghu"
    assert boss_resolution.storypack_id == "wuxia_jianghu_pack"
    assert boss_resolution.status == "implemented_in_storypack_preview"
    assert boss_resolution.priority_class == "route_key"
    assert "boss_resolution_seed" in boss_resolution.phases
    assert "faction_negotiation" in boss_resolution.surfaces
    assert "evidence_priority" in [
        choice["role"] for choice in boss_resolution.choice_shapes
    ]
    assert (
        "final_boss_resolution_true_route_confirmed_seeded"
        in boss_resolution.outcome_hooks["possible_flags"]
    )
    assert (
        "sado_defeat_does_not_save_mumyeong"
        in boss_resolution.outcome_hooks["possible_clues"]
    )

    mumyeong_resolution = db.encounter_cards["wuxia_mumyeong_resolution"]
    assert mumyeong_resolution.world_id == "wuxia_jianghu"
    assert mumyeong_resolution.storypack_id == "wuxia_jianghu_pack"
    assert mumyeong_resolution.status == "implemented_in_storypack_preview"
    assert mumyeong_resolution.priority_class == "route_key"
    assert "mumyeong_resolution_seed" in mumyeong_resolution.phases
    assert "faction_negotiation" in mumyeong_resolution.surfaces
    assert "safe_identity_return" in [
        choice["role"] for choice in mumyeong_resolution.choice_shapes
    ]
    assert (
        "final_mumyeong_resolution_own_flow_salvation_seeded"
        in mumyeong_resolution.outcome_hooks["possible_flags"]
    )
    assert (
        "mumyeong_salvation_is_not_return_to_qingliu"
        in mumyeong_resolution.outcome_hooks["possible_clues"]
    )

    seoharin_qingliu_resolution = db.encounter_cards[
        "wuxia_seoharin_qingliu_resolution"
    ]
    assert seoharin_qingliu_resolution.world_id == "wuxia_jianghu"
    assert seoharin_qingliu_resolution.storypack_id == "wuxia_jianghu_pack"
    assert seoharin_qingliu_resolution.status == "implemented_in_storypack_preview"
    assert seoharin_qingliu_resolution.priority_class == "route_key"
    assert "seoharin_qingliu_resolution_seed" in seoharin_qingliu_resolution.phases
    assert "sect_courtyard" in seoharin_qingliu_resolution.surfaces
    assert "faction_negotiation" in seoharin_qingliu_resolution.surfaces
    assert "safe_identity_return" in [
        choice["role"] for choice in seoharin_qingliu_resolution.choice_shapes
    ]
    assert (
        "final_epilogue_seoharin_open_gate_candidate_seeded"
        in seoharin_qingliu_resolution.outcome_hooks["possible_flags"]
    )
    assert (
        "open_gate_is_not_possession"
        in seoharin_qingliu_resolution.outcome_hooks["possible_clues"]
    )
    assert seoharin_qingliu_resolution.outcome_hooks["possible_items"] == []

    cheongirok_resolution = db.encounter_cards["wuxia_cheongirok_resolution"]
    assert cheongirok_resolution.world_id == "wuxia_jianghu"
    assert cheongirok_resolution.storypack_id == "wuxia_jianghu_pack"
    assert cheongirok_resolution.status == "implemented_in_storypack_preview"
    assert cheongirok_resolution.priority_class == "route_key"
    assert "cheongirok_resolution_seed" in cheongirok_resolution.phases
    assert "cheonggi_record" in cheongirok_resolution.surfaces
    assert "safe_reading" in [
        choice["role"] for choice in cheongirok_resolution.choice_shapes
    ]
    assert (
        "final_epilogue_tianjilu_safe_high_use_variant_seeded"
        in cheongirok_resolution.outcome_hooks["possible_flags"]
    )
    assert (
        "record_does_not_answer_questions"
        in cheongirok_resolution.outcome_hooks["possible_clues"]
    )
    assert cheongirok_resolution.outcome_hooks["possible_items"] == []


def test_storypack_db_public_files_validate_cleanly():
    assert validate_storypack_db(ROOT) == []


def test_storypack_db_validation_detects_reference_and_card_contract_errors(tmp_path):
    db_dir = tmp_path / "docs" / "content" / "storypack_db"
    db_dir.mkdir(parents=True)
    (db_dir / "storypacks.json").write_text(
        json.dumps(
            [
                {
                    "id": "office_pack",
                    "world_id": "office_apocalypse",
                    "status": "candidate",
                    "source_refs": [],
                    "name": "Office Pack",
                    "one_line": "office test",
                    "main_surfaces": ["messenger"],
                    "anomaly_types": ["mismatched_floor"],
                    "main_phases": ["opening_absence"],
                    "sensitive_topics": [],
                    "reusable_npc_slots": ["infra_interpreter"],
                    "ending_candidates": ["escape_alone"],
                    "main_spine_support": "supports the main spine",
                    "runtime_promotion_notes": "not runtime yet",
                }
            ],
            ensure_ascii=False,
            indent=2,
        )
        + "\n",
        encoding="utf-8",
    )
    (db_dir / "encounter_situations.json").write_text(
        json.dumps(
            [
                {
                    "id": "bad_cross_world_card",
                    "world_id": "wuxia_jianghu",
                    "storypack_id": "office_pack",
                    "status": "candidate",
                    "phase": ["opening_absence"],
                    "priority_class": "main_forced",
                    "location_tags": ["office"],
                    "surface": ["messenger"],
                    "anomaly_type": ["mismatched_floor"],
                    "pressure_type": ["sanity"],
                    "npc_slots": ["infra_interpreter"],
                    "candidate_characters": [],
                    "summary": "bad card",
                    "setup_text": "bad setup",
                    "choice_shapes": [
                        {
                            "id": "only_attack",
                            "role": "high_risk_attack",
                            "expected_costs": [],
                            "expected_gains": [],
                        }
                    ],
                    "outcome_hooks": {
                        "possible_flags": [],
                        "possible_clues": [],
                        "possible_items": [],
                    },
                    "main_spine_link": "",
                    "randomization_notes": "",
                    "promotion_notes": "",
                },
                {
                    "id": "missing_storypack_card",
                    "world_id": "office_apocalypse",
                    "storypack_id": "missing_pack",
                    "status": "candidate",
                    "phase": ["opening_absence"],
                    "priority_class": "unknown_priority",
                    "location_tags": ["office"],
                    "surface": ["unknown_surface"],
                    "anomaly_type": ["unknown_anomaly"],
                    "pressure_type": ["unknown_pressure"],
                    "npc_slots": ["unknown_slot"],
                    "candidate_characters": [],
                    "summary": "missing pack",
                    "setup_text": "missing pack",
                    "choice_shapes": [
                        {
                            "id": "wait",
                            "role": "safe_observe",
                            "expected_costs": [],
                            "expected_gains": ["minor_clue"],
                        }
                    ],
                    "outcome_hooks": {
                        "possible_flags": ["seen"],
                        "possible_clues": [],
                        "possible_items": [],
                    },
                    "main_spine_link": "links to spine",
                    "randomization_notes": "",
                    "promotion_notes": "",
                },
            ],
            ensure_ascii=False,
            indent=2,
        )
        + "\n",
        encoding="utf-8",
    )

    errors = validate_storypack_db(tmp_path)

    assert "card bad_cross_world_card world_id wuxia_jianghu does not match storypack office_pack world_id office_apocalypse" in errors
    assert "card bad_cross_world_card has no fallback/safe choice role" in errors
    assert "card bad_cross_world_card has no outcome hook" in errors
    assert "card bad_cross_world_card main_spine_link is required" in errors
    assert "card missing_storypack_card references unknown storypack_id: missing_pack" in errors
    assert "card missing_storypack_card has unknown priority_class: unknown_priority" in errors
    assert "card missing_storypack_card has unknown surface: unknown_surface" in errors
    assert "card missing_storypack_card has unknown anomaly_type: unknown_anomaly" in errors
    assert "card missing_storypack_card has unknown pressure_type: unknown_pressure" in errors
    assert "card missing_storypack_card has unknown npc_slot: unknown_slot" in errors

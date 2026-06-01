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
        "yageunmong_clockout_gate_self",
        "yageunmong_late_night_desk_awake",
        "yageunmong_manager_approval_trap",
        "yageunmong_reality_anchor_pantry",
        "yageunmong_unapproved_meeting_room_loop",
    ]
    assert sorted(db.cards_by_storypack("wuxia_jianghu_pack")) == [
        "wuxia_baekdo_medicine_debt",
        "wuxia_black_heaven_escape_price",
        "wuxia_boss_first_appearance",
        "wuxia_cheonggi_record_first_fragment",
        "wuxia_cheongryu_apprentice_entry",
        "wuxia_cheongryu_chore_sparring",
        "wuxia_cheongryu_raid_route_split",
        "wuxia_cheongryu_raid_wounded_fallback",
        "wuxia_commute_rift_arrival",
        "wuxia_heavenly_archive_previous_outsiders",
        "wuxia_heuksa_bang_first_fight",
        "wuxia_mumyeong_copy_style_reveal",
        "wuxia_mumyeong_first_confrontation",
        "wuxia_mumyeong_first_sighting",
        "wuxia_mumyeong_midgame_reunion",
        "wuxia_mumyeong_reads_orthodox_style",
        "wuxia_seo_harin_rescue",
        "wuxia_wounded_shelter_dawn_offers",
    ]

    yageun_opening = db.encounter_cards["yageunmong_late_night_desk_awake"]
    assert yageun_opening.world_id == "office_dream"
    assert yageun_opening.storypack_id == "yageunmong_pack"
    assert "lucid_dream_awareness" in yageun_opening.phases
    assert "safe_observe" in [choice["role"] for choice in yageun_opening.choice_shapes]
    assert "yageunmong_started" in yageun_opening.outcome_hooks["possible_flags"]

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

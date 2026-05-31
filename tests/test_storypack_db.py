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
        "wuxia_cheonggi_record_first_fragment",
        "wuxia_cheongryu_apprentice_entry",
        "wuxia_cheongryu_raid_route_split",
        "wuxia_commute_rift_arrival",
        "wuxia_heuksa_bang_first_fight",
        "wuxia_seo_harin_rescue",
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

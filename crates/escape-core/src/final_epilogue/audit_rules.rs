use super::audit::StateRule;

pub(super) const COMBAT_RESULT_RULES: &[StateRule] = &[
    StateRule {
        value: "battle_loss",
        flags: &["final_combat_result_battle_loss_seeded"],
    },
    StateRule {
        value: "battle_victory",
        flags: &["final_combat_result_battle_victory_seeded"],
    },
];

pub(super) const BOSS_RESOLUTION_ROUTE_RULES: &[StateRule] = &[
    StateRule {
        value: "corrupted_victory",
        flags: &[
            "final_boss_resolution_corrupted_candidate_seeded",
            "final_boss_resolution_corrupted_victory_seeded",
            "final_epilogue_candidates_corrupted_seeded",
        ],
    },
    StateRule {
        value: "true_route_victory",
        flags: &[
            "final_boss_resolution_true_route_candidate_seeded",
            "final_boss_resolution_true_route_confirmed_seeded",
            "final_epilogue_candidates_true_route_seeded",
        ],
    },
    StateRule {
        value: "mumyeong_unsaved_victory",
        flags: &[
            "final_boss_resolution_mumyeong_unsaved_victory_seeded",
            "final_epilogue_candidates_mumyeong_unsaved_seeded",
        ],
    },
    StateRule {
        value: "meaningful_victory",
        flags: &[
            "final_boss_resolution_true_or_meaningful_candidate_seeded",
            "final_boss_resolution_meaningful_candidate_seeded",
            "final_boss_resolution_meaningful_or_true_candidate_seeded",
            "final_boss_resolution_meaningful_victory_seeded",
            "final_epilogue_candidates_meaningful_seeded",
        ],
    },
    StateRule {
        value: "incomplete_victory",
        flags: &[
            "final_boss_resolution_incomplete_victory_seeded",
            "final_epilogue_candidates_incomplete_seeded",
        ],
    },
];

pub(super) const EVIDENCE_STATE_RULES: &[StateRule] = &[
    StateRule {
        value: "strong",
        flags: &[
            "final_evidence_strong_seeded",
            "final_evidence_strong_support_seeded",
            "final_evidence_strong_confirmed_seeded",
            "final_alliance_silence_strong_evidence_variant_seeded",
        ],
    },
    StateRule {
        value: "partial_or_strong",
        flags: &["final_evidence_partial_or_strong_seeded"],
    },
    StateRule {
        value: "partial",
        flags: &[
            "final_evidence_partial_seeded",
            "final_alliance_silence_partial_evidence_variant_seeded",
        ],
    },
    StateRule {
        value: "none_or_low",
        flags: &["final_evidence_none_or_low_seeded"],
    },
];

pub(super) const NETWORK_HANDLING_RULES: &[StateRule] = &[
    StateRule {
        value: "core_cut",
        flags: &[
            "final_network_core_cut_seeded",
            "final_network_core_network_cut_seeded",
        ],
    },
    StateRule {
        value: "accountability",
        flags: &[
            "final_network_ledger_secured_seeded",
            "final_network_accountability_seeded",
        ],
    },
    StateRule {
        value: "partially_destroyed",
        flags: &["final_network_partially_destroyed_seeded"],
    },
    StateRule {
        value: "residue_possible",
        flags: &["final_network_residue_possible_seeded"],
    },
    StateRule {
        value: "ignored",
        flags: &["final_network_ignored_seeded"],
    },
];

pub(super) const PRESSURE_STATE_RULES: &[StateRule] = &[
    StateRule {
        value: "eased",
        flags: &[
            "final_pressure_eased_seeded",
            "final_pressure_state_eased_confirmed_seeded",
        ],
    },
    StateRule {
        value: "partially_eased",
        flags: &["final_pressure_partially_eased_seeded"],
    },
    StateRule {
        value: "unresolved",
        flags: &[
            "final_pressure_unresolved_seeded",
            "final_black_serpent_pressure_unresolved_variant_seeded",
        ],
    },
];

pub(super) const SEOHARIN_AXIS_RULES: &[StateRule] = &[
    StateRule {
        value: "open_gate",
        flags: &[
            "final_seoharin_open_gate_candidate_seeded",
            "final_epilogue_seoharin_open_gate_candidate_seeded",
            "final_epilogue_seoharin_open_gate_candidate_reinforced_seeded",
            "final_epilogue_seoharin_open_gate_reinforced_seeded",
        ],
    },
    StateRule {
        value: "empty_place",
        flags: &[
            "final_seoharin_qingliu_resolution_empty_place_seeded",
            "final_epilogue_seoharin_empty_place_candidate_seeded",
            "final_epilogue_seoharin_empty_place_candidate_reinforced_seeded",
        ],
    },
    StateRule {
        value: "high_preserved",
        flags: &[
            "final_seoharin_axis_high_preserved_seeded",
            "final_seoharin_axis_high_seeded",
            "final_epilogue_seoharin_future_candidate_seeded",
        ],
    },
    StateRule {
        value: "closed_gate",
        flags: &[
            "final_seoharin_closed_gate_candidate_seeded",
            "final_epilogue_seoharin_closed_gate_candidate_seeded",
            "final_return_settlement_evasion_seeded",
            "final_epilogue_closed_gate_risk_seeded",
        ],
    },
    StateRule {
        value: "last_bowl",
        flags: &[
            "last_bowl_epilogue_seeded",
            "final_epilogue_seoharin_last_bowl_conditional_seeded",
        ],
    },
];

pub(super) const QINGLIU_REBUILD_RULES: &[StateRule] = &[
    StateRule {
        value: "high",
        flags: &[
            "final_qingliu_rebuild_high_candidate_seeded",
            "final_qingliu_future_high_candidate_seeded",
            "final_epilogue_qingliu_future_high_candidate_seeded",
            "final_epilogue_qingliu_restored_martial_art_candidate_seeded",
            "final_epilogue_qingliu_restored_martial_art_conditional_seeded",
        ],
    },
    StateRule {
        value: "partial",
        flags: &[
            "final_qingliu_rebuild_partial_seeded",
            "final_epilogue_qingliu_future_candidate_seeded",
        ],
    },
    StateRule {
        value: "weakened",
        flags: &[
            "final_epilogue_qingliu_future_weakened_variant_seeded",
            "final_epilogue_qingliu_future_dark_variant_seeded",
        ],
    },
];

pub(super) const MUMYEONG_SALVATION_RULES: &[StateRule] = &[
    StateRule {
        value: "own_flow_salvation",
        flags: &[
            "final_mumyeong_resolution_own_flow_salvation_seeded",
            "final_epilogue_mumyeong_stolen_forms_stopped_candidate_seeded",
        ],
    },
    StateRule {
        value: "relational_salvation",
        flags: &[
            "final_mumyeong_resolution_relational_salvation_seeded",
            "final_epilogue_mumyeong_unsent_apology_candidate_seeded",
        ],
    },
    StateRule {
        value: "substantial_candidate",
        flags: &["final_mumyeong_salvation_substantial_candidate_seeded"],
    },
    StateRule {
        value: "partial",
        flags: &["final_mumyeong_salvation_partial_seeded"],
    },
    StateRule {
        value: "incomplete",
        flags: &[
            "final_mumyeong_resolution_incomplete_salvation_seeded",
            "final_epilogue_mumyeong_second_wooden_sword_conditional_seeded",
        ],
    },
    StateRule {
        value: "end_of_stolen_forms",
        flags: &[
            "final_mumyeong_resolution_end_of_stolen_forms_seeded",
            "final_epilogue_mumyeong_end_of_stolen_forms_candidate_seeded",
        ],
    },
    StateRule {
        value: "black_serpent_successor",
        flags: &[
            "final_mumyeong_resolution_black_serpent_successor_seeded",
            "final_epilogue_mumyeong_black_serpent_new_scale_candidate_seeded",
        ],
    },
    StateRule {
        value: "corrupted_unsaved",
        flags: &[
            "final_mumyeong_resolution_corrupted_unsaved_seeded",
            "final_mumyeong_player_method_tool_use_seeded",
        ],
    },
];

pub(super) const SUCCESSOR_ROUTE_RULES: &[StateRule] = &[
    StateRule {
        value: "active_risk",
        flags: &[
            "final_successor_route_active_risk_seeded",
            "final_mumyeong_successor_route_active_seeded",
            "final_black_serpent_new_scale_candidate_seeded",
            "final_epilogue_mumyeong_black_serpent_new_scale_candidate_seeded",
        ],
    },
    StateRule {
        value: "weakened",
        flags: &["final_mumyeong_successor_route_weakened_seeded"],
    },
    StateRule {
        value: "suppressed",
        flags: &[
            "final_successor_route_suppressed_seeded",
            "final_successor_route_suppressed_confirmed_seeded",
            "final_mumyeong_successor_route_suppressed_seeded",
        ],
    },
];

pub(super) const OWN_FLOW_CHOICE_RULES: &[StateRule] = &[
    StateRule {
        value: "chosen",
        flags: &[
            "final_own_flow_choice_chosen_seeded",
            "final_mumyeong_own_flow_choice_confirmed_seeded",
            "final_mumyeong_resolution_own_flow_salvation_seeded",
        ],
    },
    StateRule {
        value: "opened",
        flags: &["final_own_flow_choice_opened_seeded"],
    },
    StateRule {
        value: "not_opened",
        flags: &["final_mumyeong_own_flow_not_opened_seeded"],
    },
];

pub(super) const TRUTH_STATE_RULES: &[StateRule] = &[
    StateRule {
        value: "not_forced",
        flags: &[
            "final_mumyeong_truth_state_not_forced_seeded",
            "truth_delivery_still_unopened",
        ],
    },
    StateRule {
        value: "partial",
        flags: &["final_mumyeong_truth_state_partial_seeded"],
    },
    StateRule {
        value: "sealed_summary_prepared",
        flags: &["sealed_departure_truth_summary_prepared"],
    },
];

pub(super) const CHEONGIROK_STATE_RULES: &[StateRule] = &[
    StateRule {
        value: "corruption_high",
        flags: &[
            "final_cheongirok_state_corruption_high_seeded",
            "final_cheongirok_state_corruption_high_confirmed_seeded",
            "final_cheongirok_resolution_corruption_variant_seeded",
            "final_epilogue_tianjilu_last_page_corruption_variant_seeded",
        ],
    },
    StateRule {
        value: "safe_high_use",
        flags: &[
            "final_cheongirok_state_high_use_seeded",
            "final_cheongirok_state_high_use_not_corruption_seeded",
            "final_cheongirok_resolution_safe_high_use_seeded",
            "final_epilogue_tianjilu_safe_high_use_variant_seeded",
        ],
    },
    StateRule {
        value: "blank_true_route_place",
        flags: &[
            "final_cheongirok_resolution_blank_place_seeded",
            "final_epilogue_tianjilu_true_route_variant_seeded",
        ],
    },
    StateRule {
        value: "low_use_silence",
        flags: &["final_cheongirok_resolution_low_use_silence_seeded"],
    },
    StateRule {
        value: "method_reflection",
        flags: &[
            "final_cheongirok_resolution_method_reflection_seeded",
            "final_player_method_reflected_not_judged_seeded",
        ],
    },
    StateRule {
        value: "corruption_risk",
        flags: &["final_cheongirok_corruption_risk_seeded"],
    },
];

pub(super) const PLAYER_METHOD_RULES: &[StateRule] = &[
    StateRule {
        value: "sado_style_calculation",
        flags: &[
            "final_player_method_sado_style_calculation_seeded",
            "final_player_method_sado_style_calculation_echo_seeded",
        ],
    },
    StateRule {
        value: "tool_use",
        flags: &[
            "final_player_method_used_as_tool_risk_seeded",
            "final_mumyeong_player_method_tool_use_seeded",
        ],
    },
    StateRule {
        value: "outside_calculation",
        flags: &[
            "final_player_method_outside_calculation_seeded",
            "final_player_method_outside_calculation_confirmed_seeded",
        ],
    },
    StateRule {
        value: "protected_as_person",
        flags: &[
            "final_player_method_protected_as_person_seeded",
            "final_player_method_protected_as_person_confirmed_seeded",
        ],
    },
    StateRule {
        value: "direct_boss_focus",
        flags: &["final_player_method_direct_boss_focus_seeded"],
    },
    StateRule {
        value: "reflected_not_judged",
        flags: &["final_player_method_reflected_not_judged_seeded"],
    },
];

pub(super) const ITEM_LOG_RULES: &[StateRule] = &[
    StateRule {
        value: "blackscale_ledger",
        flags: &["final_item_logs_blackscale_ledger_seeded"],
    },
    StateRule {
        value: "blank_ledger",
        flags: &["final_item_logs_blank_ledger_seen_seeded"],
    },
    StateRule {
        value: "unpriced_wooden_sword_condition",
        flags: &[
            "final_unpriced_wooden_sword_condition_raised_seeded",
            "final_unpriced_wooden_sword_condition_preserved_seeded",
        ],
    },
];

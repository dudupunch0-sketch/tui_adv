use crate::scene_page::BodyBlock;
use crate::state::GameState;
use std::collections::BTreeSet;

const FINAL_EPILOGUE_ENDING_ID: &str = "wuxia_final_epilogue_renderer_contract";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FinalResult {
    BattleLoss,
    CorruptedVictory,
    TrueRouteVictory,
    MumyeongUnsavedVictory,
    MeaningfulVictory,
    IncompleteVictory,
    BasicVictory,
}

impl FinalResult {
    fn key(self) -> &'static str {
        match self {
            Self::BattleLoss => "battle_loss",
            Self::CorruptedVictory => "corrupted_victory",
            Self::TrueRouteVictory => "true_route_victory",
            Self::MumyeongUnsavedVictory => "mumyeong_unsaved_victory",
            Self::MeaningfulVictory => "meaningful_victory",
            Self::IncompleteVictory => "incomplete_victory",
            Self::BasicVictory => "basic_victory",
        }
    }

    fn title(self) -> &'static str {
        match self {
            Self::BattleLoss => "패배 결산",
            Self::CorruptedVictory => "침식 승리 결산",
            Self::TrueRouteVictory => "계산식 밖의 승리",
            Self::MumyeongUnsavedVictory => "무명 비구원 승리",
            Self::MeaningfulVictory => "의미 있는 승리",
            Self::IncompleteVictory => "불완전한 승리",
            Self::BasicVictory => "기본 승리",
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MainEndingType {
    BattleLoss,
    Returnee,
    MurimOutsider,
    CheongryuDivineSword,
    WhitePathPrison,
    BlackNightGentleman,
    DebtorOfAllUnderHeaven,
}

impl MainEndingType {
    pub(crate) fn key(self) -> &'static str {
        match self {
            Self::BattleLoss => "battle_loss",
            Self::Returnee => "returnee",
            Self::MurimOutsider => "murim_outsider",
            Self::CheongryuDivineSword => "cheongryu_divine_sword",
            Self::WhitePathPrison => "white_path_prison",
            Self::BlackNightGentleman => "black_night_gentleman",
            Self::DebtorOfAllUnderHeaven => "debtor_of_all_under_heaven",
        }
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::BattleLoss => "패배 결산",
            Self::Returnee => "귀환자",
            Self::MurimOutsider => "무림 외지인",
            Self::CheongryuDivineSword => "청류 신검",
            Self::WhitePathPrison => "백도의 굴레",
            Self::BlackNightGentleman => "흑야의 협객",
            Self::DebtorOfAllUnderHeaven => "천하의 채무자",
        }
    }
}


#[derive(Clone, Debug)]
struct CardCandidate {
    id: &'static str,
    variant: &'static str,
    group: &'static str,
    consumed_seeds: Vec<String>,
    body: &'static str,
}

#[derive(Clone, Debug)]
struct SuppressedCard {
    id: &'static str,
    suppressed_by: &'static str,
    consumed_seeds: Vec<String>,
}

pub(crate) fn final_epilogue_body_blocks(state: &GameState, ending_id: &str) -> Vec<BodyBlock> {
    if ending_id != FINAL_EPILOGUE_ENDING_ID {
        return Vec::new();
    }

    let facts = FinalFacts::from_state(state);
    if !facts.has_required_preconditions() {
        return vec![BodyBlock {
            kind: "epilogue_contract_error".to_string(),
            text: "final_epilogue_contract: required preconditions missing; renderer must not recompute cards.".to_string(),
            source_id: Some(FINAL_EPILOGUE_ENDING_ID.to_string()),
        }];
    }

    let final_result = facts.final_result();
    let main_ending_type = facts.main_ending_type(final_result);
    let mut candidates = build_candidates(&facts, final_result);
    let suppressed = apply_suppress_rules(&facts, final_result, &mut candidates);

    let mut blocks = vec![BodyBlock {
        kind: "epilogue_result".to_string(),
        text: final_result_text(final_result, main_ending_type),
        source_id: Some(FINAL_EPILOGUE_ENDING_ID.to_string()),
    }];
    blocks.push(state_audit_block(&facts, final_result));
    blocks.extend(candidates.iter().map(card_block));
    blocks.extend(suppressed.iter().map(suppressed_block));
    blocks
}

struct FinalFacts<'a> {
    flags: BTreeSet<&'a str>,
    clues: BTreeSet<&'a str>,
}

impl<'a> FinalFacts<'a> {
    fn from_state(state: &'a GameState) -> Self {
        Self {
            flags: state.flags.iter().map(String::as_str).collect(),
            clues: state.clues.iter().map(String::as_str).collect(),
        }
    }

    fn has_required_preconditions(&self) -> bool {
        [
            "boss_resolution_resolved",
            "mumyeong_resolution_resolved",
            "seoharin_qingliu_resolution_resolved",
            "cheongirok_resolution_resolved",
            "black_serpent_aftermath_resolved",
            "final_result_priority_applied_seeded",
            "final_state_routing_seeded",
        ]
        .iter()
        .all(|flag| self.has_flag(flag))
            && (self.has_flag("final_combat_result_battle_victory_seeded")
                || self.has_flag("final_combat_result_battle_loss_seeded"))
    }

    fn has_flag(&self, flag: &str) -> bool {
        self.flags.contains(flag)
    }

    fn has_clue(&self, clue: &str) -> bool {
        self.clues.contains(clue)
    }

    fn has_any_flag(&self, flags: &[&str]) -> bool {
        flags.iter().any(|flag| self.has_flag(flag))
    }

    fn has_any_clue(&self, clues: &[&str]) -> bool {
        clues.iter().any(|clue| self.has_clue(clue))
    }

    fn consumed_flags(&self, flags: &[&str]) -> Vec<String> {
        flags
            .iter()
            .filter(|flag| self.has_flag(flag))
            .map(|flag| (*flag).to_string())
            .collect()
    }

    fn final_result(&self) -> FinalResult {
        if self.has_flag("final_combat_result_battle_loss_seeded") {
            return FinalResult::BattleLoss;
        }
        if self.has_any_flag(&[
            "final_boss_resolution_corrupted_victory_seeded",
            "final_epilogue_candidates_corrupted_seeded",
            "final_cheongirok_state_corruption_high_seeded",
            "final_cheongirok_state_corruption_high_confirmed_seeded",
        ]) && self.has_any_flag(&[
            "final_player_method_sado_style_calculation_seeded",
            "final_player_method_sado_style_calculation_echo_seeded",
            "final_mumyeong_player_method_tool_use_seeded",
        ]) {
            return FinalResult::CorruptedVictory;
        }
        if self.has_any_flag(&[
            "final_boss_resolution_true_route_confirmed_seeded",
            "final_epilogue_candidates_true_route_seeded",
        ]) {
            return FinalResult::TrueRouteVictory;
        }
        if self.has_any_flag(&[
            "final_boss_resolution_mumyeong_unsaved_victory_seeded",
            "final_epilogue_candidates_mumyeong_unsaved_seeded",
        ]) {
            return FinalResult::MumyeongUnsavedVictory;
        }
        if self.has_any_flag(&[
            "final_boss_resolution_meaningful_victory_seeded",
            "final_epilogue_candidates_meaningful_seeded",
        ]) {
            return FinalResult::MeaningfulVictory;
        }
        if self.has_any_flag(&[
            "final_boss_resolution_incomplete_victory_seeded",
            "final_epilogue_candidates_incomplete_seeded",
        ]) {
            return FinalResult::IncompleteVictory;
        }
        FinalResult::BasicVictory
    }

    fn main_ending_type(&self, final_result: FinalResult) -> MainEndingType {
        // Priority:
        // 1. battle_loss -> BattleLoss
        // 2. final_return_intent_honest_seeded -> Returnee
        // 3. final_settlement_intent_honest_seeded -> MurimOutsider
        // 4. TrueRouteVictory -> CheongryuDivineSword
        // 5. CorruptedVictory -> DebtorOfAllUnderHeaven
        // 6. sapa epilogue seeds -> BlackNightGentleman
        // 7. default -> WhitePathPrison
        if matches!(final_result, FinalResult::BattleLoss) {
            return MainEndingType::BattleLoss;
        }
        if self.has_flag("final_return_intent_honest_seeded") {
            return MainEndingType::Returnee;
        }
        if self.has_flag("final_settlement_intent_honest_seeded") {
            return MainEndingType::MurimOutsider;
        }
        if matches!(final_result, FinalResult::TrueRouteVictory) {
            return MainEndingType::CheongryuDivineSword;
        }
        if matches!(final_result, FinalResult::CorruptedVictory) {
            return MainEndingType::DebtorOfAllUnderHeaven;
        }
        if self.has_any_flag(&[
            "final_mumyeong_resolution_black_serpent_successor_seeded",
            "final_epilogue_mumyeong_black_serpent_new_scale_candidate_seeded",
            "final_black_serpent_new_scale_candidate_seeded",
            "final_player_method_sado_style_calculation_seeded",
            "final_player_method_sado_style_calculation_echo_seeded",
        ]) {
            return MainEndingType::BlackNightGentleman;
        }
        MainEndingType::WhitePathPrison
    }
}

#[derive(Clone, Debug)]
struct StateRule {
    value: &'static str,
    flags: &'static [&'static str],
}

#[derive(Clone, Debug)]
struct AuditEntry {
    key: &'static str,
    value: &'static str,
    status: &'static str,
    consumed_flags: Vec<String>,
    candidate_values: Vec<&'static str>,
}

fn state_audit_block(facts: &FinalFacts<'_>, final_result: FinalResult) -> BodyBlock {
    let mut text = format!(
        "audit_id: final_state_canonical_collapse\nowned_by: Rust GameCore\nsource_contract: wuxia_final_state_canonical_collapse_contract\nfinal_result_key: {}\nrouting_note: local final_*_seeded flags were collapsed before renderer display; renderer must not infer canonical final states.",
        final_result.key()
    );
    for entry in canonical_state_audit(facts, final_result) {
        text.push_str(&format!(
            "\ncanonical_state: {}\nvalue: {}\nstatus: {}\nconsumed_flags: {}\ncandidate_values: {}",
            entry.key,
            entry.value,
            entry.status,
            if entry.consumed_flags.is_empty() {
                "none".to_string()
            } else {
                seeds_text(&entry.consumed_flags)
            },
            if entry.candidate_values.is_empty() {
                "none".to_string()
            } else {
                entry.candidate_values.join(", ")
            }
        ));
    }
    BodyBlock {
        kind: "epilogue_state_audit".to_string(),
        text,
        source_id: Some("wuxia_final_state_canonical_collapse_contract".to_string()),
    }
}

fn canonical_state_audit(facts: &FinalFacts<'_>, final_result: FinalResult) -> Vec<AuditEntry> {
    let mut entries = vec![
        resolve_state("combat_result", facts, COMBAT_RESULT_RULES),
        resolve_state("boss_resolution_route", facts, BOSS_RESOLUTION_ROUTE_RULES),
        resolve_state("evidence_state", facts, EVIDENCE_STATE_RULES),
        resolve_state("network_handling", facts, NETWORK_HANDLING_RULES),
        resolve_state("pressure_state", facts, PRESSURE_STATE_RULES),
        resolve_state("seoharin_axis", facts, SEOHARIN_AXIS_RULES),
        resolve_state("qingliu_rebuild", facts, QINGLIU_REBUILD_RULES),
        resolve_state("mumyeong_salvation", facts, MUMYEONG_SALVATION_RULES),
        resolve_state("successor_route", facts, SUCCESSOR_ROUTE_RULES),
        resolve_state("own_flow_choice", facts, OWN_FLOW_CHOICE_RULES),
        resolve_state("truth_state", facts, TRUTH_STATE_RULES),
        resolve_state("cheongirok_state", facts, CHEONGIROK_STATE_RULES),
        resolve_state("player_method", facts, PLAYER_METHOD_RULES),
        resolve_state("item_logs", facts, ITEM_LOG_RULES),
    ];

    if matches!(final_result, FinalResult::BattleLoss)
        && entries[1].status == "missing"
        && facts.has_flag("final_combat_result_battle_loss_seeded")
    {
        entries[1] = AuditEntry {
            key: "boss_resolution_route",
            value: "not_reached_battle_loss",
            status: "derived_by_final_result_priority",
            consumed_flags: facts.consumed_flags(&["final_combat_result_battle_loss_seeded"]),
            candidate_values: vec!["not_reached_battle_loss"],
        };
    }

    entries
}

fn resolve_state(key: &'static str, facts: &FinalFacts<'_>, rules: &[StateRule]) -> AuditEntry {
    let mut consumed = BTreeSet::new();
    let mut candidate_values = Vec::new();
    for rule in rules {
        let rule_flags = facts.consumed_flags(rule.flags);
        if !rule_flags.is_empty() {
            candidate_values.push(rule.value);
            consumed.extend(rule_flags);
        }
    }

    if candidate_values.is_empty() {
        return AuditEntry {
            key,
            value: "missing",
            status: "missing",
            consumed_flags: Vec::new(),
            candidate_values,
        };
    }

    let value = candidate_values[0];
    let distinct_values = candidate_values
        .iter()
        .copied()
        .collect::<BTreeSet<&'static str>>();
    let status = if distinct_values.len() > 1 {
        "ambiguous_priority_applied"
    } else {
        "resolved"
    };

    AuditEntry {
        key,
        value,
        status,
        consumed_flags: consumed.into_iter().collect(),
        candidate_values,
    }
}

const COMBAT_RESULT_RULES: &[StateRule] = &[
    StateRule {
        value: "battle_loss",
        flags: &["final_combat_result_battle_loss_seeded"],
    },
    StateRule {
        value: "battle_victory",
        flags: &["final_combat_result_battle_victory_seeded"],
    },
];

const BOSS_RESOLUTION_ROUTE_RULES: &[StateRule] = &[
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

const EVIDENCE_STATE_RULES: &[StateRule] = &[
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

const NETWORK_HANDLING_RULES: &[StateRule] = &[
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

const PRESSURE_STATE_RULES: &[StateRule] = &[
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

const SEOHARIN_AXIS_RULES: &[StateRule] = &[
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

const QINGLIU_REBUILD_RULES: &[StateRule] = &[
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

const MUMYEONG_SALVATION_RULES: &[StateRule] = &[
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

const SUCCESSOR_ROUTE_RULES: &[StateRule] = &[
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

const OWN_FLOW_CHOICE_RULES: &[StateRule] = &[
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

const TRUTH_STATE_RULES: &[StateRule] = &[
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

const CHEONGIROK_STATE_RULES: &[StateRule] = &[
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

const PLAYER_METHOD_RULES: &[StateRule] = &[
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

const ITEM_LOG_RULES: &[StateRule] = &[
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

fn build_candidates(facts: &FinalFacts<'_>, final_result: FinalResult) -> Vec<CardCandidate> {
    let mut cards = Vec::new();

    if facts.has_any_flag(&[
        "final_return_intent_honest_seeded",
        "final_epilogue_return_absence_candidate_seeded",
    ]) {
        push_card(
            &mut cards,
            facts,
            "epilogue_wuxia_returned_commute",
            "honest_return",
            "return_settlement",
            &[
                "final_return_settlement_contract_seeded",
                "final_return_intent_honest_seeded",
                "final_epilogue_return_absence_candidate_seeded",
            ],
            "돌아온 출근길은 도망친 보상이 아니다. 소매 끝의 흙먼지와 빈 업무수첩 한 줄이 강호에 두고 온 자리를 기억한다.",
        );
    }
    if facts.has_any_flag(&[
        "final_settlement_intent_honest_seeded",
        "final_epilogue_qingliu_settlement_candidate_seeded",
    ]) {
        push_card(
            &mut cards,
            facts,
            "epilogue_wuxia_qingliu_settlement",
            "honest_settlement",
            "return_settlement",
            &[
                "final_return_settlement_contract_seeded",
                "final_settlement_intent_honest_seeded",
                "final_epilogue_qingliu_settlement_candidate_seeded",
            ],
            "청류문에 남은 외지인은 사원증을 태워 영웅이 되지 않는다. 낯선 단추 하나가 창고 상자에 남고, 아무도 그것에 가격을 붙이지 않는다.",
        );
    }
    if facts.has_any_flag(&[
        "final_return_settlement_uncertain_shared_seeded",
        "final_epilogue_empty_place_kept_open_seeded",
    ]) {
        push_card(
            &mut cards,
            facts,
            "epilogue_wuxia_empty_place_kept_open",
            "uncertain_shared",
            "return_settlement",
            &[
                "final_return_settlement_contract_seeded",
                "final_return_settlement_uncertain_shared_seeded",
                "final_epilogue_empty_place_kept_open_seeded",
            ],
            "아직 모른다는 대답은 회피가 아니었다. 빈자리는 귀환과 정착 어느 쪽도 미리 닫지 않는 약속으로 남는다.",
        );
    }
    if facts.has_any_flag(&[
        "final_return_settlement_evasion_seeded",
        "final_epilogue_closed_gate_risk_seeded",
    ]) {
        push_card(
            &mut cards,
            facts,
            "epilogue_wuxia_closed_gate_risk",
            "evasion_risk",
            "return_settlement",
            &[
                "final_return_settlement_contract_seeded",
                "final_return_settlement_evasion_seeded",
                "final_epilogue_closed_gate_risk_seeded",
            ],
            "말을 돌린 자리에는 닫힌 산문이 확정되지 않는다. 다만 기다림을 설명하지 않은 비용이 문고리에 남는다.",
        );
    }

    if matches!(final_result, FinalResult::BattleLoss) {
        push_card(
            &mut cards,
            facts,
            "epilogue_boss_black_serpent_banner",
            "battle_loss_residue",
            "boss_black_serpent",
            &[
                "final_combat_result_battle_loss_seeded",
                "final_black_serpent_banner_candidate_seeded",
                "final_black_serpent_banner_candidate_reinforced_seeded",
                "final_epilogue_boss_black_serpent_banner_conditional_seeded",
                "final_black_serpent_aftermath_banner_residue_seeded",
            ],
            "장터 입구에는 며칠 만에 다시 검은 깃발이 걸렸다.\n사람들은 놀라지 않았다.\n누가 이겼든,\n밤길에 값을 매기는 사람은 늘 필요하다는 듯이.\n표국의 말들은 그 깃발을 지나갈 때마다 걸음을 늦췄다.",
        );
        push_card(
            &mut cards,
            facts,
            "epilogue_wuxia_southern_market_rumor",
            "unresolved_debt",
            "boss_black_serpent",
            &[
                "final_combat_result_battle_loss_seeded",
                "final_southern_market_rumor_candidate_seeded",
                "final_southern_market_rumor_candidate_reinforced_seeded",
                "final_epilogue_southern_market_rumor_conditional_seeded",
                "final_black_serpent_aftermath_southern_market_rumor_seeded",
            ],
            "남쪽 장터에서는 흑사방 잔당이 다시 표국을 습격했다는 풍문이 돌았다.\n청류문 장로들은 그것을 작은 불씨라 불렀지만,\n장터 사람들은 안다.\n작은 불씨는 늘 누군가가 대수롭지 않게 넘긴 자리에서 살아난다는 것을.\n객잔의 취객들은 그 소문을 두고\n칼보다 빠른 것은 발이 아니라 방심이라고 웃었다.\n그러나 그 웃음은 오래 가지 않았다.",
        );
        push_card(
            &mut cards,
            facts,
            "epilogue_mumyeong_black_serpent_new_scale",
            "battle_loss_successor_pressure",
            "mumyeong",
            &[
                "final_combat_result_battle_loss_seeded",
                "final_black_serpent_new_scale_candidate_seeded",
                "final_mumyeong_successor_route_active_seeded",
                "final_epilogue_mumyeong_black_serpent_new_scale_candidate_seeded",
            ],
            "흑사방의 옛 깃발은 찢어졌지만,\n장터의 밤길은 조용해지지 않았다.\n사람들은 방주의 이름이 사라졌다고 말했다.\n그러나 새로 걷히는 통행세의 손짓은\n어딘가 청류문을 닮아 있었다.\n누군가는 그것을 두고 검은 뱀의 새 비늘이라 불렀다.",
        );
        push_card(
            &mut cards,
            facts,
            "epilogue_seoharin_closed_gate",
            "battle_loss_or_corruption",
            "seoharin_qingliu",
            &[
                "final_combat_result_battle_loss_seeded",
                "final_seoharin_closed_gate_candidate_seeded",
                "final_epilogue_seoharin_closed_gate_candidate_seeded",
            ],
            "산문은 닫혀 있었다.\n안에 있는 사람들은 안전했다.\n서하린은 그렇게 믿었다.\n바깥에서 부르는 소리는 들리지 않았다.\n아니, 들리지 않아야 했다.\n밤이 깊어질수록 산문 안쪽의 등불은 더 밝아졌다.\n누군가 문을 열어야 하지 않느냐고 묻자,\n서하린은 문빗장을 한 번 더 확인했다.\n\"나가면, 다시 돌아오지 않을 거야.\"\n그 말이 누구를 향한 것인지는 아무도 묻지 않았다.",
        );
        push_card(
            &mut cards,
            facts,
            "epilogue_tianjilu_last_page",
            "corruption_variant",
            "cheongirok",
            &[
                "final_combat_result_battle_loss_seeded",
                "final_epilogue_tianjilu_last_page_corruption_variant_seeded",
                "final_cheongirok_state_corruption_high_seeded",
                "final_cheongirok_state_corruption_high_confirmed_seeded",
            ],
            "마지막 장은 거의 비어 있었다.\n다만 한 줄만은 지워지지 않았다.\n기록자는 대답하지 않는다.\n다만 다음 장을 넘긴다.\n그 문장이 누구를 향한 것인지는 알 수 없었다.\n주인공을 향한 것인지,\n이전에 기록된 누군가를 향한 것인지,\n아니면 아직 기록되지 않은 이름을 향한 것인지도.\n기록서는 조용히 덮였다.\n하지만 마지막 장은 끝내 완전히 닫히지 않았다.",
        );
    }

    if matches!(
        final_result,
        FinalResult::BasicVictory
            | FinalResult::IncompleteVictory
            | FinalResult::MeaningfulVictory
            | FinalResult::TrueRouteVictory
            | FinalResult::CorruptedVictory
    ) || facts.has_any_flag(&[
        "final_broken_black_serpent_epilogue_candidate_seeded",
        "final_broken_black_serpent_epilogue_candidate_reinforced_seeded",
        "final_epilogue_boss_broken_black_serpent_variant_ready_seeded",
    ]) {
        push_card(
            &mut cards,
            facts,
            "epilogue_boss_broken_black_serpent",
            final_result.key(),
            "boss_black_serpent",
            &[
                "final_broken_black_serpent_epilogue_candidate_seeded",
                "final_broken_black_serpent_epilogue_candidate_reinforced_seeded",
                "final_epilogue_boss_broken_black_serpent_variant_ready_seeded",
            ],
            "흑사방의 깃발은 한동안 장터 바닥에 끌렸다.\n사람들은 이제 밤길이 안전해졌다고 말하지 않았다.\n다만 예전보다 조금 늦게 문을 닫았다.\n표국 장부의 붉은 표식은 하나씩 지워졌지만,\n빚 문서가 사라진 자리에는 오래 접힌 자국이 남았다.",
        );
    }

    push_optional_card(
        &mut cards,
        facts,
        "epilogue_boss_black_serpent_banner",
        "residue",
        "boss_black_serpent",
        &[
            "final_black_serpent_banner_candidate_seeded",
            "final_black_serpent_banner_candidate_reinforced_seeded",
            "final_epilogue_boss_black_serpent_banner_conditional_seeded",
            "final_black_serpent_aftermath_banner_residue_seeded",
        ],
        "장터 입구에는 며칠 만에 다시 검은 깃발이 걸렸다.\n사람들은 놀라지 않았다.\n누가 이겼든,\n밤길에 값을 매기는 사람은 늘 필요하다는 듯이.\n표국의 말들은 그 깃발을 지나갈 때마다 걸음을 늦췄다.",
    );
    push_optional_card(
        &mut cards,
        facts,
        "epilogue_wuxia_southern_market_rumor",
        "unresolved_debt",
        "boss_black_serpent",
        &[
            "final_southern_market_rumor_candidate_seeded",
            "final_southern_market_rumor_candidate_reinforced_seeded",
            "final_epilogue_southern_market_rumor_conditional_seeded",
            "final_black_serpent_aftermath_southern_market_rumor_seeded",
        ],
        "남쪽 장터에서는 흑사방 잔당이 다시 표국을 습격했다는 풍문이 돌았다.\n청류문 장로들은 그것을 작은 불씨라 불렀지만,\n장터 사람들은 안다.\n작은 불씨는 늘 누군가가 대수롭지 않게 넘긴 자리에서 살아난다는 것을.\n객잔의 취객들은 그 소문을 두고\n칼보다 빠른 것은 발이 아니라 방심이라고 웃었다.\n그러나 그 웃음은 오래 가지 않았다.",
    );
    if facts.has_any_flag(&[
        "final_alliance_silence_strong_evidence_variant_seeded",
        "final_alliance_silence_partial_evidence_variant_seeded",
        "final_epilogue_boss_alliance_silence_conditional_seeded",
        "final_black_serpent_aftermath_alliance_silence_seeded",
        "final_alliance_silence_responsibility_evasion_seeded",
    ]) {
        let variant = if facts.has_any_flag(&[
            "final_evidence_strong_seeded",
            "final_evidence_strong_support_seeded",
            "final_evidence_strong_confirmed_seeded",
            "final_alliance_silence_strong_evidence_variant_seeded",
            "final_alliance_silence_responsibility_evasion_seeded",
        ]) || facts
            .has_any_clue(&["strong_evidence_turns_silence_into_responsibility_evasion"])
        {
            "responsibility_evasion"
        } else {
            "private_document_or_partial_evidence"
        };
        push_card(
            &mut cards,
            facts,
            "epilogue_boss_alliance_silence",
            variant,
            "boss_black_serpent",
            &[
                "final_alliance_silence_strong_evidence_variant_seeded",
                "final_alliance_silence_partial_evidence_variant_seeded",
                "final_epilogue_boss_alliance_silence_conditional_seeded",
                "final_black_serpent_aftermath_alliance_silence_seeded",
                "final_alliance_silence_responsibility_evasion_seeded",
                "final_evidence_strong_seeded",
                "final_evidence_strong_support_seeded",
                "final_evidence_strong_confirmed_seeded",
            ],
            "무림맹은 공문을 보냈다.\n사건은 유감이나,\n공식 기록상 흑사방의 활동 범위는 확인되지 않았다고 했다.\n청류문 사람들은 그 문장을 세 번 읽고도 아무 말도 하지 않았다.\n서하린은 공문을 접어 장문인의 방 앞에 두었다.\n그날 청류문 수련장에는 아무도 구호를 외치지 않았다.",
        );
    }

    push_optional_card(
        &mut cards,
        facts,
        "epilogue_mumyeong_stolen_forms_stopped",
        "own_flow",
        "mumyeong",
        &[
            "final_mumyeong_resolution_own_flow_salvation_seeded",
            "final_epilogue_mumyeong_stolen_forms_stopped_candidate_seeded",
        ],
        "강호 어딘가에서 낯선 무인이 비무를 벌였다는 소문이 돌았다.\n그는 어느 문파의 초식도 끝까지 흉내 내지 않았다.\n첫 세 수는 남의 것이었지만,\n네 번째 수부터는 아무도 알아보지 못했다.\n그날 이후 사람들은 그를 어느 문파 출신인지로 묻지 않았다.\n다만 이상하게도,\n그가 떠난 자리에는 늘 물길처럼 휘어진 발자국이 남았다.",
    );
    push_optional_card(
        &mut cards,
        facts,
        "epilogue_mumyeong_second_wooden_sword",
        "candidate_not_payout",
        "mumyeong",
        &[
            "final_mumyeong_second_wooden_sword_candidate_seeded",
            "final_epilogue_mumyeong_second_wooden_sword_candidate_seeded",
            "final_epilogue_mumyeong_second_wooden_sword_conditional_seeded",
        ],
        "청류문 산문 밖에는 목검이 두 자루 놓였다.\n하나는 새 수습생의 것이었고,\n다른 하나는 오래전에 사라진 제자의 것이었다.\n서하린은 아무 말 없이 두 번째 목검에 묻은 흙을 털어냈다.\n문 안으로 들인 것은 아니었다.\n하지만 문밖에 그대로 두지도 않았다.",
    );
    push_optional_card(
        &mut cards,
        facts,
        "epilogue_mumyeong_unsent_apology",
        "not_forced_truth",
        "mumyeong",
        &[
            "final_epilogue_mumyeong_unsent_apology_candidate_seeded",
            "final_epilogue_mumyeong_unsent_apology_conditional_seeded",
        ],
        "청류문 산문 앞에는 접히지 않은 편지 한 장이 놓여 있었다.\n서하린은 그 편지를 오래 들여다보았지만,\n끝내 펼치지 않았다.\n글자가 없다는 걸 알면서도,\n그녀는 한동안 그 종이를 버리지 못했다.\n누군가는 사과가 늦으면 아무 의미가 없다고 했다.\n서하린은 그 말을 부정하지 않았다.\n다만 편지를 불태우지도 않았다.",
    );
    push_optional_card(
        &mut cards,
        facts,
        "epilogue_mumyeong_end_of_stolen_forms",
        "cost_trace",
        "mumyeong",
        &[
            "final_mumyeong_resolution_end_of_stolen_forms_seeded",
            "final_epilogue_mumyeong_end_of_stolen_forms_candidate_seeded",
        ],
        "그가 마지막으로 쓴 초식은 아무 문파의 것도 아니었다.\n검로는 검객의 것이었고,\n발은 보법가의 것이었고,\n호흡은 독공 수련자의 것이었다.\n몸은 그 모든 것을 견디지 못했다.\n쓰러진 자리에는 완성된 무공이 남지 않았다.\n다만 너무 많은 타인의 흔적이 한 몸에서 서로를 밀어내고 있었다.",
    );
    push_optional_card(
        &mut cards,
        facts,
        "epilogue_mumyeong_black_serpent_new_scale",
        "successor_route",
        "mumyeong",
        &[
            "final_black_serpent_new_scale_candidate_seeded",
            "final_mumyeong_successor_route_active_seeded",
            "final_epilogue_mumyeong_black_serpent_new_scale_candidate_seeded",
        ],
        "흑사방의 옛 깃발은 찢어졌지만,\n장터의 밤길은 조용해지지 않았다.\n사람들은 방주의 이름이 사라졌다고 말했다.\n그러나 새로 걷히는 통행세의 손짓은\n어딘가 청류문을 닮아 있었다.\n누군가는 그것을 두고 검은 뱀의 새 비늘이라 불렀다.",
    );
    push_optional_card(
        &mut cards,
        facts,
        "epilogue_mumyeong_new_shadow",
        "secondary_rumor",
        "mumyeong",
        &["final_epilogue_mumyeong_new_shadow_variant_seeded"],
        "흑사방 깃발 아래에 새 그림자가 섰다는 소문이 돌았다.\n그는 이름을 쓰지 않았고,\n어느 문파의 초식이든 한 번은 따라 했다.\n두 번째부터는 더 이상 따라 하는 것처럼 보이지 않았다.\n장터 사람들은 그를 두고 이렇게 말했다.\n\"검은 뱀에게 새 비늘이 돋았다.\"",
    );

    if matches!(final_result, FinalResult::TrueRouteVictory)
        || facts.has_flag("final_epilogue_seoharin_future_candidate_seeded")
    {
        push_card(
            &mut cards,
            facts,
            "epilogue_seoharin_future",
            "return_place_not_claim",
            "seoharin_qingliu",
            &["final_epilogue_seoharin_future_candidate_seeded"],
            "서하린은 여전히 청류문에 남아 있었다.\n떠난 사람들을 모두 붙잡지는 못했다.\n하지만 돌아오는 길을 없애지도 않았다.\n산문 옆에는 낡은 목검 하나가 더 걸렸다.\n누구의 것이냐고 묻는 수습생에게,\n서하린은 잠시 침묵하다가 말했다.\n\"비워둔 거야.\"\n그 말이 기다림인지,\n허락인지,\n아니면 오래된 습관인지는 아무도 묻지 않았다.",
        );
    }
    if matches!(final_result, FinalResult::TrueRouteVictory)
        || facts.has_any_flag(&[
            "final_epilogue_seoharin_empty_place_candidate_seeded",
            "final_epilogue_seoharin_empty_place_candidate_reinforced_seeded",
        ])
    {
        push_card(
            &mut cards,
            facts,
            "epilogue_seoharin_empty_place",
            "return_or_absence",
            "seoharin_qingliu",
            &[
                "final_epilogue_seoharin_empty_place_candidate_seeded",
                "final_epilogue_seoharin_empty_place_candidate_reinforced_seeded",
                "final_seoharin_axis_high_preserved_seeded",
                "final_unpriced_wooden_sword_condition_raised_seeded",
                "final_unpriced_wooden_sword_condition_preserved_seeded",
            ],
            "주인공은 돌아오지 않았다.\n그래도 서하린은 수련장 한쪽을 비워두었다.\n비가 오는 날이면 그 자리의 먼지는 다른 곳보다 늦게 말랐고,\n새 수습생들은 그곳에 물건을 두지 않았다.\n누군가 물었다.\n\"저 자리는 누구 겁니까?\"\n서하린은 목검 끈을 고쳐 매며 말했다.\n\"없는 사람 자리도, 자리야.\"\n그 뒤로 아무도 그 자리를 치우지 않았다.",
        );
    }
    if matches!(final_result, FinalResult::TrueRouteVictory)
        || facts.has_any_flag(&[
            "final_seoharin_open_gate_candidate_seeded",
            "final_epilogue_seoharin_open_gate_candidate_seeded",
            "final_epilogue_seoharin_open_gate_candidate_reinforced_seeded",
            "final_epilogue_seoharin_open_gate_reinforced_seeded",
        ])
    {
        push_card(
            &mut cards,
            facts,
            "epilogue_seoharin_open_gate",
            "not_possession",
            "seoharin_qingliu",
            &[
                "final_seoharin_open_gate_candidate_seeded",
                "final_epilogue_seoharin_open_gate_candidate_seeded",
                "final_epilogue_seoharin_open_gate_candidate_reinforced_seeded",
                "final_epilogue_seoharin_open_gate_reinforced_seeded",
            ],
            "무명은 돌아오지 않았다.\n적어도 그날은 그랬다.\n하지만 산문 앞의 흙은 쓸려 있지 않았다.\n비가 온 뒤에도 누군가 발자국이 남을 길을 고쳐 두었다.\n새 수습생이 물었다.\n\"저 길은 왜 막지 않습니까?\"\n서하린은 잠시 산 아래를 보았다.\n\"막아두면, 돌아오는 사람도 길을 잃어.\"\n그 말 뒤로 산문은 오래 열려 있었다.",
        );
    }
    push_optional_card(
        &mut cards,
        facts,
        "epilogue_seoharin_closed_gate",
        "sado_style_protection",
        "seoharin_qingliu",
        &[
            "final_seoharin_closed_gate_candidate_seeded",
            "final_epilogue_seoharin_closed_gate_candidate_seeded",
        ],
        "산문은 닫혀 있었다.\n안에 있는 사람들은 안전했다.\n서하린은 그렇게 믿었다.\n바깥에서 부르는 소리는 들리지 않았다.\n아니, 들리지 않아야 했다.\n밤이 깊어질수록 산문 안쪽의 등불은 더 밝아졌다.\n누군가 문을 열어야 하지 않느냐고 묻자,\n서하린은 문빗장을 한 번 더 확인했다.\n\"나가면, 다시 돌아오지 않을 거야.\"\n그 말이 누구를 향한 것인지는 아무도 묻지 않았다.",
    );
    push_optional_card(
        &mut cards,
        facts,
        "epilogue_seoharin_last_bowl",
        "conditional_absence",
        "seoharin_qingliu",
        &[
            "last_bowl_epilogue_seeded",
            "final_epilogue_seoharin_last_bowl_conditional_seeded",
        ],
        "서하린은 더 이상 밥을 남기지 않았다.\n식탁 끝의 빈 그릇은 어느 날부터 찬장 안으로 들어갔다.\n남은 음식을 버리는 일은 줄었고,\n청류문 부엌은 조금 더 조용해졌다.\n누군가 그릇 하나가 비었다고 말했지만,\n서하린은 대답하지 않았다.\n그날 저녁,\n그녀는 평소보다 오래 식탁을 닦았다.",
    );
    if matches!(
        final_result,
        FinalResult::MeaningfulVictory | FinalResult::TrueRouteVictory
    ) || facts.has_any_flag(&[
        "final_qingliu_future_high_candidate_seeded",
        "final_epilogue_qingliu_future_candidate_seeded",
        "final_epilogue_qingliu_future_high_candidate_seeded",
        "final_epilogue_qingliu_future_weakened_variant_seeded",
    ]) {
        let variant = if facts.has_flag("final_epilogue_qingliu_future_weakened_variant_seeded") {
            "weakened_but_flowing"
        } else {
            "poor_but_flowing"
        };
        push_card(
            &mut cards,
            facts,
            "epilogue_qingliu_future",
            variant,
            "seoharin_qingliu",
            &[
                "final_qingliu_future_high_candidate_seeded",
                "final_epilogue_qingliu_future_candidate_seeded",
                "final_epilogue_qingliu_future_high_candidate_seeded",
                "final_epilogue_qingliu_future_weakened_variant_seeded",
            ],
            "청류문 수련장에는 다시 사람 목소리가 들리기 시작했다.\n아직 가난했고,\n아직 지붕은 새고 있었다.\n하지만 더 이상 아무도 청류문이 끝났다고 말하지는 않았다.\n장문인의 방 앞에는 새 물동이가 놓였고,\n수련장 한쪽에는 이름 없는 목검이 몇 자루 늘어났다.\n강호는 여전히 거칠었지만,\n흐르던 물은 멈추지 않았다.",
        );
    }
    if matches!(final_result, FinalResult::TrueRouteVictory)
        || facts.has_any_flag(&[
            "final_epilogue_qingliu_restored_martial_art_candidate_seeded",
            "final_epilogue_qingliu_restored_martial_art_conditional_seeded",
        ])
    {
        push_card(
            &mut cards,
            facts,
            "epilogue_qingliu_restored_martial_art",
            "restored_flow",
            "seoharin_qingliu",
            &[
                "final_epilogue_qingliu_restored_martial_art_candidate_seeded",
                "final_epilogue_qingliu_restored_martial_art_conditional_seeded",
            ],
            "수련장 한쪽에는 새 비급이 아니라, 오래된 초식의 빈칸을 메운 종이들이 걸렸다.\n청류문 사람들은 그것을 복구라 부르지 않았다.\n잃어버린 흐름이 다시 물길을 찾았다고만 했다.\n장문인의 방 앞에는 여전히 물동이가 놓였고,\n지붕은 아직 비가 오면 샜다.\n하지만 새 수습생들은 더 이상 비어 있는 초식을 외우지 않았다.\n그들은 비어 있던 자리를 지나, 다음 흐름으로 발을 옮겼다.",
        );
    }

    if matches!(final_result, FinalResult::TrueRouteVictory) {
        push_card(
            &mut cards,
            facts,
            "epilogue_tianjilu_last_page",
            "true_route_blank_place",
            "cheongirok",
            &[
                "final_epilogue_tianjilu_true_route_variant_seeded",
                "final_unpriced_wooden_sword_condition_preserved_seeded",
                "final_unpriced_wooden_sword_condition_raised_seeded",
            ],
            "마지막 장은 거의 비어 있었다.\n다만 한 줄만은 지워지지 않았다.\n기록자는 대답하지 않는다.\n다만 다음 장을 넘긴다.\n그 문장이 누구를 향한 것인지는 알 수 없었다.\n주인공을 향한 것인지,\n이전에 기록된 누군가를 향한 것인지,\n아니면 아직 기록되지 않은 이름을 향한 것인지도.\n기록서는 조용히 덮였다.\n하지만 마지막 장은 끝내 완전히 닫히지 않았다.",
        );
    } else if matches!(final_result, FinalResult::CorruptedVictory)
        || facts.has_flag("final_epilogue_tianjilu_last_page_corruption_variant_seeded")
    {
        push_card(
            &mut cards,
            facts,
            "epilogue_tianjilu_last_page",
            "corruption_variant",
            "cheongirok",
            &[
                "final_epilogue_tianjilu_last_page_corruption_variant_seeded",
                "final_cheongirok_state_corruption_high_seeded",
                "final_cheongirok_state_corruption_high_confirmed_seeded",
            ],
            "마지막 장은 거의 비어 있었다.\n다만 한 줄만은 지워지지 않았다.\n기록자는 대답하지 않는다.\n다만 다음 장을 넘긴다.\n그 문장이 누구를 향한 것인지는 알 수 없었다.\n주인공을 향한 것인지,\n이전에 기록된 누군가를 향한 것인지,\n아니면 아직 기록되지 않은 이름을 향한 것인지도.\n기록서는 조용히 덮였다.\n하지만 마지막 장은 끝내 완전히 닫히지 않았다.",
        );
    } else if facts.has_any_flag(&[
        "final_epilogue_tianjilu_last_page_candidate_seeded",
        "final_epilogue_tianjilu_safe_high_use_variant_seeded",
        "final_cheongirok_resolution_low_use_silence_seeded",
    ]) {
        let variant = if facts.has_flag("final_epilogue_tianjilu_safe_high_use_variant_seeded") {
            "safe_high_use"
        } else {
            "low_use_silence"
        };
        push_card(
            &mut cards,
            facts,
            "epilogue_tianjilu_last_page",
            variant,
            "cheongirok",
            &[
                "final_epilogue_tianjilu_last_page_candidate_seeded",
                "final_epilogue_tianjilu_safe_high_use_variant_seeded",
                "final_cheongirok_resolution_low_use_silence_seeded",
            ],
            "마지막 장은 거의 비어 있었다.\n다만 한 줄만은 지워지지 않았다.\n기록자는 대답하지 않는다.\n다만 다음 장을 넘긴다.\n그 문장이 누구를 향한 것인지는 알 수 없었다.\n주인공을 향한 것인지,\n이전에 기록된 누군가를 향한 것인지,\n아니면 아직 기록되지 않은 이름을 향한 것인지도.\n기록서는 조용히 덮였다.\n하지만 마지막 장은 끝내 완전히 닫히지 않았다.",
        );
    }

    cards
}

fn push_optional_card(
    cards: &mut Vec<CardCandidate>,
    facts: &FinalFacts<'_>,
    id: &'static str,
    variant: &'static str,
    group: &'static str,
    seed_flags: &[&'static str],
    body: &'static str,
) {
    if facts.has_any_flag(seed_flags) {
        push_card(cards, facts, id, variant, group, seed_flags, body);
    }
}

fn push_card(
    cards: &mut Vec<CardCandidate>,
    facts: &FinalFacts<'_>,
    id: &'static str,
    variant: &'static str,
    group: &'static str,
    seed_flags: &[&'static str],
    body: &'static str,
) {
    if cards.iter().any(|card| card.id == id) {
        return;
    }
    cards.push(CardCandidate {
        id,
        variant,
        group,
        consumed_seeds: facts.consumed_flags(seed_flags),
        body,
    });
}

fn apply_suppress_rules(
    facts: &FinalFacts<'_>,
    final_result: FinalResult,
    candidates: &mut Vec<CardCandidate>,
) -> Vec<SuppressedCard> {
    let mut suppressed = Vec::new();

    if matches!(final_result, FinalResult::BattleLoss) {
        suppress_cards(
            candidates,
            &mut suppressed,
            &[
                "epilogue_boss_broken_black_serpent",
                "epilogue_seoharin_open_gate",
                "epilogue_mumyeong_stolen_forms_stopped",
            ],
            "battle_loss",
        );
    }

    if matches!(final_result, FinalResult::CorruptedVictory) {
        suppress_cards(
            candidates,
            &mut suppressed,
            &[
                "epilogue_seoharin_open_gate",
                "epilogue_seoharin_empty_place",
                "epilogue_mumyeong_stolen_forms_stopped",
            ],
            "corrupted_victory",
        );
    }

    if matches!(final_result, FinalResult::TrueRouteVictory) {
        suppress_cards(
            candidates,
            &mut suppressed,
            &[
                "epilogue_mumyeong_black_serpent_new_scale",
                "epilogue_mumyeong_new_shadow",
                "epilogue_seoharin_closed_gate",
                "epilogue_seoharin_last_bowl",
                "epilogue_boss_black_serpent_banner",
                "epilogue_wuxia_southern_market_rumor",
            ],
            "true_route_victory",
        );
    }

    if candidates
        .iter()
        .any(|card| card.id == "epilogue_seoharin_open_gate")
    {
        suppress_cards(
            candidates,
            &mut suppressed,
            &["epilogue_seoharin_closed_gate"],
            "open_gate_vs_closed_gate",
        );
    } else if candidates
        .iter()
        .any(|card| card.id == "epilogue_seoharin_closed_gate")
    {
        suppress_cards(
            candidates,
            &mut suppressed,
            &["epilogue_seoharin_open_gate"],
            "open_gate_vs_closed_gate",
        );
    }

    if candidates
        .iter()
        .any(|card| card.id == "epilogue_seoharin_empty_place")
    {
        suppress_cards(
            candidates,
            &mut suppressed,
            &["epilogue_seoharin_last_bowl"],
            "empty_place_vs_last_bowl",
        );
    }

    if candidates
        .iter()
        .any(|card| card.id == "epilogue_wuxia_closed_gate_risk")
    {
        suppress_cards(
            candidates,
            &mut suppressed,
            &[
                "epilogue_wuxia_returned_commute",
                "epilogue_wuxia_qingliu_settlement",
                "epilogue_wuxia_empty_place_kept_open",
            ],
            "return_settlement_evasion",
        );
    }

    if facts.has_any_flag(&[
        "final_black_serpent_banner_suppressed_candidate_seeded",
        "final_southern_market_rumor_suppressed_candidate_seeded",
        "final_black_serpent_aftermath_banner_suppressed_seeded",
        "final_pressure_eased_seeded",
        "final_pressure_state_eased_confirmed_seeded",
        "final_network_core_cut_seeded",
        "final_network_core_network_cut_seeded",
    ]) || facts.has_any_clue(&["true_route_can_suppress_banner_and_rumor"])
    {
        suppress_cards(
            candidates,
            &mut suppressed,
            &[
                "epilogue_boss_black_serpent_banner",
                "epilogue_wuxia_southern_market_rumor",
            ],
            "banner_rumor_suppressed_by_route_or_pressure",
        );
    }

    suppressed
}

fn suppress_cards(
    candidates: &mut Vec<CardCandidate>,
    suppressed: &mut Vec<SuppressedCard>,
    ids: &[&'static str],
    suppressed_by: &'static str,
) {
    let mut index = 0;
    while index < candidates.len() {
        if ids.contains(&candidates[index].id) {
            let card = candidates.remove(index);
            if !suppressed
                .iter()
                .any(|existing| existing.id == card.id && existing.suppressed_by == suppressed_by)
            {
                suppressed.push(SuppressedCard {
                    id: card.id,
                    suppressed_by,
                    consumed_seeds: card.consumed_seeds,
                });
            }
        } else {
            index += 1;
        }
    }
}

fn final_result_text(final_result: FinalResult, main_ending_type: MainEndingType) -> String {
    format!(
        "final_result_key: {}
result_title: {}
main_ending_type: {}
main_ending_label: {}
owned_by: Rust GameCore
routing_note: final_result_priority, seed consumption, suppress, and card ordering were resolved before renderer display.",
        final_result.key(),
        final_result.title(),
        main_ending_type.key(),
        main_ending_type.label()
    )
}
fn card_block(card: &CardCandidate) -> BodyBlock {
    BodyBlock {
        kind: "epilogue_card".to_string(),
        text: format!(
            "card_id: {}\nvariant: {}\ngroup: {}\nconsumed_seeds: {}\n{}",
            card.id,
            card.variant,
            card.group,
            seeds_text(&card.consumed_seeds),
            card.body
        ),
        source_id: Some(card.id.to_string()),
    }
}

fn suppressed_block(card: &SuppressedCard) -> BodyBlock {
    BodyBlock {
        kind: "epilogue_suppressed".to_string(),
        text: format!(
            "card_id: {}\nsuppressed_by: {}\nconsumed_seeds: {}",
            card.id,
            card.suppressed_by,
            seeds_text(&card.consumed_seeds)
        ),
        source_id: Some(card.id.to_string()),
    }
}

fn seeds_text(seeds: &[String]) -> String {
    if seeds.is_empty() {
        "implicit_by_final_result".to_string()
    } else {
        seeds.join(", ")
    }
}

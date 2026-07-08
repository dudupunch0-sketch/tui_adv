use crate::state::GameState;
use std::collections::BTreeSet;

use super::types::{FinalResult, MainEndingType};

pub(super) struct FinalFacts<'a> {
    flags: BTreeSet<&'a str>,
    clues: BTreeSet<&'a str>,
}

impl<'a> FinalFacts<'a> {
    pub(super) fn from_state(state: &'a GameState) -> Self {
        Self {
            flags: state.flags.iter().map(String::as_str).collect(),
            clues: state.clues.iter().map(String::as_str).collect(),
        }
    }

    pub(super) fn has_required_preconditions(&self) -> bool {
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

    pub(super) fn has_flag(&self, flag: &str) -> bool {
        self.flags.contains(flag)
    }

    fn has_clue(&self, clue: &str) -> bool {
        self.clues.contains(clue)
    }

    pub(super) fn has_any_flag(&self, flags: &[&str]) -> bool {
        flags.iter().any(|flag| self.has_flag(flag))
    }

    pub(super) fn has_any_clue(&self, clues: &[&str]) -> bool {
        clues.iter().any(|clue| self.has_clue(clue))
    }

    pub(super) fn consumed_flags(&self, flags: &[&str]) -> Vec<String> {
        flags
            .iter()
            .filter(|flag| self.has_flag(flag))
            .map(|flag| (*flag).to_string())
            .collect()
    }

    pub(super) fn final_result(&self) -> FinalResult {
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

    pub(super) fn main_ending_type(&self, final_result: FinalResult) -> MainEndingType {
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

use super::facts::FinalFacts;
use super::types::{CardCandidate, FinalResult, SuppressedCard};

pub(super) fn apply_suppress_rules(
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

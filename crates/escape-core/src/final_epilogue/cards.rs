use super::facts::FinalFacts;
use super::types::{CardCandidate, FinalResult};

const BODY_TIANJILU_LAST_PAGE: &str = "마지막 장은 거의 비어 있었다.\n다만 한 줄만은 지워지지 않았다.\n기록자는 대답하지 않는다.\n다만 다음 장을 넘긴다.\n그 문장이 누구를 향한 것인지는 알 수 없었다.\n주인공을 향한 것인지,\n이전에 기록된 누군가를 향한 것인지,\n아니면 아직 기록되지 않은 이름을 향한 것인지도.\n기록서는 조용히 덮였다.\n하지만 마지막 장은 끝내 완전히 닫히지 않았다.";
const BODY_BLACK_SERPENT_BANNER: &str = "장터 입구에는 며칠 만에 다시 검은 깃발이 걸렸다.\n사람들은 놀라지 않았다.\n누가 이겼든,\n밤길에 값을 매기는 사람은 늘 필요하다는 듯이.\n표국의 말들은 그 깃발을 지나갈 때마다 걸음을 늦췄다.";
const BODY_SOUTHERN_MARKET_RUMOR: &str = "남쪽 장터에서는 흑사방 잔당이 다시 표국을 습격했다는 풍문이 돌았다.\n청류문 장로들은 그것을 작은 불씨라 불렀지만,\n장터 사람들은 안다.\n작은 불씨는 늘 누군가가 대수롭지 않게 넘긴 자리에서 살아난다는 것을.\n객잔의 취객들은 그 소문을 두고\n칼보다 빠른 것은 발이 아니라 방심이라고 웃었다.\n그러나 그 웃음은 오래 가지 않았다.";
const BODY_MUMYEONG_NEW_SCALE: &str = "흑사방의 옛 깃발은 찢어졌지만,\n장터의 밤길은 조용해지지 않았다.\n사람들은 방주의 이름이 사라졌다고 말했다.\n그러나 새로 걷히는 통행세의 손짓은\n어딘가 청류문을 닮아 있었다.\n누군가는 그것을 두고 검은 뱀의 새 비늘이라 불렀다.";
const BODY_SEOHARIN_CLOSED_GATE: &str = "산문은 닫혀 있었다.\n안에 있는 사람들은 안전했다.\n서하린은 그렇게 믿었다.\n바깥에서 부르는 소리는 들리지 않았다.\n아니, 들리지 않아야 했다.\n밤이 깊어질수록 산문 안쪽의 등불은 더 밝아졌다.\n누군가 문을 열어야 하지 않느냐고 묻자,\n서하린은 문빗장을 한 번 더 확인했다.\n\"나가면, 다시 돌아오지 않을 거야.\"\n그 말이 누구를 향한 것인지는 아무도 묻지 않았다.";

pub(super) fn build_candidates(
    facts: &FinalFacts<'_>,
    final_result: FinalResult,
) -> Vec<CardCandidate> {
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
            BODY_BLACK_SERPENT_BANNER,
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
            BODY_SOUTHERN_MARKET_RUMOR,
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
            BODY_MUMYEONG_NEW_SCALE,
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
            BODY_SEOHARIN_CLOSED_GATE,
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
            BODY_TIANJILU_LAST_PAGE,
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
        BODY_BLACK_SERPENT_BANNER,
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
        BODY_SOUTHERN_MARKET_RUMOR,
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
        BODY_MUMYEONG_NEW_SCALE,
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
        BODY_SEOHARIN_CLOSED_GATE,
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
            BODY_TIANJILU_LAST_PAGE,
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
            BODY_TIANJILU_LAST_PAGE,
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
            BODY_TIANJILU_LAST_PAGE,
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

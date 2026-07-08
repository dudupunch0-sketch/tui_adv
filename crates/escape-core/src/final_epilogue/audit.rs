use crate::scene_page::BodyBlock;
use std::collections::BTreeSet;

use super::audit_rules::*;
use super::facts::FinalFacts;
use super::render::seeds_text;
use super::types::FinalResult;

pub(super) struct StateRule {
    pub(super) value: &'static str,
    pub(super) flags: &'static [&'static str],
}

#[derive(Clone, Debug)]
pub(super) struct AuditEntry {
    pub(super) key: &'static str,
    pub(super) value: &'static str,
    status: &'static str,
    consumed_flags: Vec<String>,
    candidate_values: Vec<&'static str>,
}

pub(super) fn state_audit_block(facts: &FinalFacts<'_>, final_result: FinalResult) -> BodyBlock {
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
        && facts.has_flag("final_combat_result_battle_loss_seeded")
    {
        if let Some(entry) = entries
            .iter_mut()
            .find(|entry| entry.key == "boss_resolution_route" && entry.status == "missing")
        {
            *entry = AuditEntry {
                key: "boss_resolution_route",
                value: "not_reached_battle_loss",
                status: "derived_by_final_result_priority",
                consumed_flags: facts.consumed_flags(&["final_combat_result_battle_loss_seeded"]),
                candidate_values: vec!["not_reached_battle_loss"],
            };
        }
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

    let Some(value) = candidate_values.first().copied() else {
        return AuditEntry {
            key,
            value: "missing",
            status: "missing",
            consumed_flags: Vec::new(),
            candidate_values,
        };
    };
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

use crate::scene_page::BodyBlock;

use super::types::{CardCandidate, FinalResult, MainEndingType, SuppressedCard};

pub(super) fn final_result_text(
    final_result: FinalResult,
    main_ending_type: MainEndingType,
) -> String {
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
pub(super) fn card_block(card: &CardCandidate) -> BodyBlock {
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

pub(super) fn suppressed_block(card: &SuppressedCard) -> BodyBlock {
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

pub(super) fn seeds_text(seeds: &[String]) -> String {
    if seeds.is_empty() {
        "implicit_by_final_result".to_string()
    } else {
        seeds.join(", ")
    }
}

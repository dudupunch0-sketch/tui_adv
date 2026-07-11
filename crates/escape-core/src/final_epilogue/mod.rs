use crate::scene_page::BodyBlock;
use crate::state::GameState;

mod audit;
mod audit_rules;
mod cards;
mod facts;
mod render;
mod suppress;
mod types;

use audit::state_audit_block;
use cards::build_candidates;
use facts::FinalFacts;
use render::{card_block, final_result_text, suppressed_block};
use suppress::apply_suppress_rules;

const FINAL_EPILOGUE_ENDING_ID: &str = "wuxia_final_epilogue_renderer_contract";

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

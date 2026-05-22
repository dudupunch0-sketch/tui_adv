use crate::effects::{printer_glyph_anomaly_cue, EffectCue};
use crate::state::GameState;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionView {
    pub id: String,
    pub label: String,
    pub cost_summary: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TurnView {
    pub location_id: String,
    pub encounter_id: Option<String>,
    pub title: String,
    pub body: String,
    pub actions: Vec<ActionView>,
    pub effect_cues: Vec<EffectCue>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionResult {
    pub encounter_id: String,
    pub action_id: String,
    pub state: GameState,
    pub logs: Vec<String>,
    pub effect_cues: Vec<EffectCue>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionError {
    UnknownAction(String),
}

impl std::fmt::Display for ActionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionError::UnknownAction(action_id) => {
                write!(formatter, "unknown action id: {action_id}")
            }
        }
    }
}

impl std::error::Error for ActionError {}

pub fn printer_turn_view(state: &GameState) -> TurnView {
    TurnView {
        location_id: state.location_id.clone(),
        encounter_id: Some("printer_prints_alone".to_string()),
        title: "복합기가 혼자 출력한다".to_string(),
        body: "꺼져 있던 복합기가 아직 고르지 않은 선택을 출력한다. 출력구 안쪽에서 종이가 밀려 나오지만, 날짜는 내일로 찍혀 있다.".to_string(),
        actions: vec![
            ActionView {
                id: "choice:wait_for_output".to_string(),
                label: "출력물이 안정될 때까지 기다린다".to_string(),
                cost_summary: Some("정신력 -2".to_string()),
            },
            ActionView {
                id: "choice:inspect_toner".to_string(),
                label: "토너 카트리지 안쪽 표식을 확인한다".to_string(),
                cost_summary: None,
            },
            ActionView {
                id: "choice:record_stable_terms".to_string(),
                label: "출력물의 안정 단어만 사원증 뒤에 적는다".to_string(),
                cost_summary: None,
            },
        ],
        effect_cues: vec![printer_glyph_anomaly_cue()],
    }
}

pub fn apply_printer_action(
    state: &GameState,
    action_id: &str,
) -> Result<ActionResult, ActionError> {
    match action_id {
        "choice:wait_for_output" => {
            let mut next_state = state.clone();
            next_state.turn += 1;
            next_state.player.sanity = (next_state.player.sanity - 2).max(0);
            next_state.add_flag_once("printer_secret_started");
            next_state.add_clue_once("copier_stable_terms");

            Ok(ActionResult {
                encounter_id: "printer_prints_alone".to_string(),
                action_id: action_id.to_string(),
                state: next_state,
                logs: vec![
                    "흔들리던 문장이 안정되고 비상계단, 토너, 접힌 방향이라는 단어만 남았다."
                        .to_string(),
                ],
                effect_cues: vec![printer_glyph_anomaly_cue()],
            })
        }
        "choice:inspect_toner" | "choice:record_stable_terms" => {
            let mut next_state = state.clone();
            next_state.turn += 1;
            next_state.add_flag_once("printer_secret_started");

            Ok(ActionResult {
                encounter_id: "printer_prints_alone".to_string(),
                action_id: action_id.to_string(),
                state: next_state,
                logs: vec!["복합기는 아직 같은 문장을 반복해서 밀어내고 있다.".to_string()],
                effect_cues: vec![printer_glyph_anomaly_cue()],
            })
        }
        other => Err(ActionError::UnknownAction(other.to_string())),
    }
}

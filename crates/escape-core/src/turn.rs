use crate::content::{
    ChoiceDef, ContentConditions, ContentIndex, EncounterDef, LocationDef, OutcomeDef, ResourceMap,
};
use crate::effects::{printer_glyph_anomaly_cue, EffectCue};
use crate::state::{GameState, PlayerState};

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContentTurnError {
    UnknownStateLocation(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContentActionError {
    UnknownStateLocation(String),
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

impl std::fmt::Display for ContentTurnError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentTurnError::UnknownStateLocation(location_id) => {
                write!(formatter, "unknown state location: {location_id}")
            }
        }
    }
}

impl std::error::Error for ContentTurnError {}

impl std::fmt::Display for ContentActionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentActionError::UnknownStateLocation(location_id) => {
                write!(formatter, "unknown state location: {location_id}")
            }
            ContentActionError::UnknownAction(action_id) => {
                write!(formatter, "unknown action id: {action_id}")
            }
        }
    }
}

impl std::error::Error for ContentActionError {}

pub fn content_turn_view(
    state: &GameState,
    content: &ContentIndex,
) -> Result<TurnView, ContentTurnError> {
    let location = content
        .location(&state.location_id)
        .ok_or_else(|| ContentTurnError::UnknownStateLocation(state.location_id.clone()))?;

    let Some(encounter) = current_content_encounter(content, state) else {
        return Ok(TurnView {
            location_id: state.location_id.clone(),
            encounter_id: None,
            title: location.name.clone(),
            body: location.description.clone(),
            actions: movement_action_views(location, content),
            effect_cues: Vec::new(),
        });
    };

    Ok(TurnView {
        location_id: state.location_id.clone(),
        encounter_id: Some(encounter.id.clone()),
        title: encounter.title.clone(),
        body: encounter.body.clone(),
        actions: encounter
            .choices
            .iter()
            .filter(|choice| choice_is_available(choice, state))
            .map(choice_action_view)
            .collect(),
        effect_cues: Vec::new(),
    })
}

pub fn apply_content_action(
    state: &GameState,
    content: &ContentIndex,
    action_id: &str,
) -> Result<ActionResult, ContentActionError> {
    let location = content
        .location(&state.location_id)
        .ok_or_else(|| ContentActionError::UnknownStateLocation(state.location_id.clone()))?;

    let Some(encounter) = current_content_encounter(content, state) else {
        return apply_movement_action(state, content, location, action_id);
    };
    let Some(choice_id) = action_id.strip_prefix("choice:") else {
        return Err(ContentActionError::UnknownAction(action_id.to_string()));
    };
    let Some(choice) = encounter
        .choices
        .iter()
        .find(|choice| choice.id == choice_id && choice_is_available(choice, state))
    else {
        return Err(ContentActionError::UnknownAction(action_id.to_string()));
    };

    let mut next_state = state.clone();
    next_state.turn += 1;
    apply_cost(&mut next_state.player, &choice.cost);
    apply_outcome(&mut next_state, &choice.outcome);
    next_state.add_seen_encounter_once(&encounter.id);

    Ok(ActionResult {
        encounter_id: encounter.id.clone(),
        action_id: action_id.to_string(),
        state: next_state,
        logs: choice.outcome.log.iter().cloned().collect(),
        effect_cues: Vec::new(),
    })
}

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

fn movement_action_views(location: &LocationDef, content: &ContentIndex) -> Vec<ActionView> {
    location
        .connections
        .iter()
        .map(|destination_id| ActionView {
            id: format!("move:{destination_id}"),
            label: content
                .location(destination_id)
                .map(|destination| destination.name.clone())
                .unwrap_or_else(|| destination_id.clone()),
            cost_summary: None,
        })
        .collect()
}

fn apply_movement_action(
    state: &GameState,
    content: &ContentIndex,
    location: &LocationDef,
    action_id: &str,
) -> Result<ActionResult, ContentActionError> {
    let Some(destination_id) = action_id.strip_prefix("move:") else {
        return Err(ContentActionError::UnknownAction(action_id.to_string()));
    };
    if !location
        .connections
        .iter()
        .any(|candidate| candidate == destination_id)
    {
        return Err(ContentActionError::UnknownAction(action_id.to_string()));
    }
    let Some(destination) = content.location(destination_id) else {
        return Err(ContentActionError::UnknownAction(action_id.to_string()));
    };

    let mut next_state = state.clone();
    next_state.turn += 1;
    next_state.location_id = destination_id.to_string();

    Ok(ActionResult {
        encounter_id: "movement".to_string(),
        action_id: action_id.to_string(),
        state: next_state,
        logs: vec![format!("{}로 이동했다.", destination.name)],
        effect_cues: Vec::new(),
    })
}

fn current_content_encounter<'a>(
    content: &'a ContentIndex,
    state: &GameState,
) -> Option<&'a EncounterDef> {
    content
        .encounters()
        .find(|encounter| encounter_is_available(encounter, state))
}

fn apply_cost(player: &mut PlayerState, cost: &ResourceMap) {
    for (resource, amount) in cost {
        apply_player_resource_delta(player, resource, -*amount);
    }
}

fn apply_outcome(state: &mut GameState, outcome: &OutcomeDef) {
    for (resource, amount) in &outcome.resources {
        apply_player_resource_delta(&mut state.player, resource, *amount);
    }
    for flag in &outcome.add_flags {
        state.add_flag_once(flag);
    }
    for clue in &outcome.add_clues {
        state.add_clue_once(clue);
    }
    if let Some(destination_id) = &outcome.destination_id {
        state.location_id = destination_id.clone();
    }
}

fn apply_player_resource_delta(player: &mut PlayerState, resource: &str, amount: i32) {
    match resource {
        "health" => player.health = clamp_resource(player.health + amount),
        "sanity" => player.sanity = clamp_resource(player.sanity + amount),
        "battery" => player.battery = clamp_resource(player.battery + amount),
        _ => {}
    }
}

fn clamp_resource(value: i32) -> i32 {
    value.clamp(0, 100)
}

fn encounter_is_available(encounter: &EncounterDef, state: &GameState) -> bool {
    !state
        .seen_encounters
        .iter()
        .any(|seen_encounter| seen_encounter == &encounter.id)
        && conditions_match(&encounter.conditions, state)
        && encounter
            .choices
            .iter()
            .any(|choice| choice_is_available(choice, state))
}

fn choice_is_available(choice: &ChoiceDef, state: &GameState) -> bool {
    conditions_match(&choice.conditions, state) && can_pay_cost(&choice.cost, &state.player)
}

fn conditions_match(conditions: &ContentConditions, state: &GameState) -> bool {
    (conditions.locations.is_empty() || conditions.locations.contains(&state.location_id))
        && conditions
            .required_flags
            .iter()
            .all(|flag| state.flags.contains(flag))
        && conditions
            .forbidden_flags
            .iter()
            .all(|flag| !state.flags.contains(flag))
        && conditions
            .required_clues
            .iter()
            .all(|clue| state.clues.contains(clue))
        && conditions
            .min_resources
            .iter()
            .all(|(resource, minimum)| player_resource(&state.player, resource) >= *minimum)
        && conditions
            .min_abilities
            .iter()
            .all(|(_ability, minimum)| *minimum <= 0)
}

fn can_pay_cost(cost: &ResourceMap, player: &PlayerState) -> bool {
    cost.iter()
        .all(|(resource, amount)| *amount <= 0 || player_resource(player, resource) >= *amount)
}

fn choice_action_view(choice: &ChoiceDef) -> ActionView {
    ActionView {
        id: format!("choice:{}", choice.id),
        label: choice.label.clone(),
        cost_summary: format_cost_summary(&choice.cost),
    }
}

fn format_cost_summary(cost: &ResourceMap) -> Option<String> {
    let parts = cost
        .iter()
        .filter(|(_resource, amount)| **amount != 0)
        .map(|(resource, amount)| {
            let sign = if *amount > 0 { "-" } else { "+" };
            format!("{} {}{}", resource_label(resource), sign, amount.abs())
        })
        .collect::<Vec<_>>();

    if parts.is_empty() {
        None
    } else {
        Some(parts.join(", "))
    }
}

fn player_resource(player: &PlayerState, resource: &str) -> i32 {
    match resource {
        "health" => player.health,
        "sanity" => player.sanity,
        "battery" => player.battery,
        _ => 0,
    }
}

fn resource_label(resource: &str) -> &str {
    match resource {
        "health" => "체력",
        "sanity" => "정신력",
        "battery" => "배터리",
        "hunger" => "허기",
        "thirst" => "갈증",
        other => other,
    }
}

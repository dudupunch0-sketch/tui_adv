use crate::content::{
    AchievementDef, ChoiceDef, ContentConditions, ContentIndex, EncounterDef, ItemDef, LocationDef,
    OutcomeDef, ResourceMap,
};
use crate::effects::{printer_glyph_anomaly_cue, EffectCue};
use crate::resources::{
    ACTION_PREFIX_CHOICE, ACTION_PREFIX_MOVE, ACTION_PREFIX_TRAIN, ACTION_PREFIX_USE,
    RESOURCE_BATTERY, RESOURCE_HEALTH, RESOURCE_HUNGER, RESOURCE_SANITY, RESOURCE_THIRST,
};
use crate::state::{CheckResolution, GameState, PlayerState};
use serde::Serialize;

const VALID_ABILITY_IDS: [&str; 6] = [
    "logic",
    "empathy",
    "volition",
    "composure",
    "interface",
    "physical",
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionView {
    pub id: String,
    pub label: String,
    pub cost_summary: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockedActionView {
    pub id: String,
    pub label: String,
    pub cost_summary: Option<String>,
    pub reasons: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TurnView {
    pub location_id: String,
    pub encounter_id: Option<String>,
    pub ending_id: Option<String>,
    pub title: String,
    pub body: String,
    pub actions: Vec<ActionView>,
    pub blocked_actions: Vec<BlockedActionView>,
    pub effect_cues: Vec<EffectCue>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ActionResult {
    pub encounter_id: String,
    pub action_id: String,
    pub state: GameState,
    pub logs: Vec<String>,
    pub effect_cues: Vec<EffectCue>,
    pub newly_unlocked_achievements: Vec<String>,
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

    if let Some(ending) = current_content_ending(content, state) {
        return Ok(TurnView {
            location_id: state.location_id.clone(),
            encounter_id: None,
            ending_id: Some(ending.id.clone()),
            title: ending.name.clone(),
            body: ending.text.clone(),
            actions: Vec::new(),
            blocked_actions: Vec::new(),
            effect_cues: Vec::new(),
        });
    }

    let item_actions = item_action_views(state, content);
    let Some(encounter) = current_content_encounter(content, state) else {
        let mut actions = movement_action_views(location, content);
        actions.extend(item_actions);
        return Ok(TurnView {
            location_id: state.location_id.clone(),
            encounter_id: None,
            ending_id: None,
            title: location.name.clone(),
            body: location.description.clone(),
            actions,
            blocked_actions: Vec::new(),
            effect_cues: Vec::new(),
        });
    };

    let stage = current_event_stage(encounter, state);
    let mut actions = if let Some(stage) = stage {
        if stage.kind == "choice" {
            stage
                .choices
                .iter()
                .filter_map(|choice_ref| {
                    encounter
                        .choices
                        .iter()
                        .find(|choice| choice.id == choice_ref.id)
                })
                .filter(|choice| choice_is_available(choice, state))
                .map(choice_action_view)
                .collect()
        } else {
            vec![ActionView {
                id: "event:continue".to_string(),
                label: "계속".to_string(),
                cost_summary: None,
            }]
        }
    } else {
        encounter
            .choices
            .iter()
            .filter(|choice| choice_is_available(choice, state))
            .map(choice_action_view)
            .collect()
    };
    actions.extend(item_actions);

    Ok(TurnView {
        location_id: state.location_id.clone(),
        encounter_id: Some(encounter.id.clone()),
        ending_id: None,
        title: encounter.title.clone(),
        body: stage
            .map(|stage| event_stage_text(stage, state))
            .filter(|body| !body.is_empty())
            .unwrap_or_else(|| encounter.body.clone()),
        actions,
        blocked_actions: encounter
            .choices
            .iter()
            .filter(|choice| {
                stage
                    .map(|stage| {
                        stage
                            .choices
                            .iter()
                            .any(|choice_ref| choice_ref.id == choice.id)
                    })
                    .unwrap_or(true)
            })
            .filter(|choice| !choice_is_available(choice, state))
            .map(|choice| blocked_choice_action_view(choice, state))
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

    if current_content_ending(content, state).is_some() {
        return Err(ContentActionError::UnknownAction(action_id.to_string()));
    }

    if action_id.starts_with(ACTION_PREFIX_TRAIN) {
        return apply_train_action(state, content, action_id);
    }

    if action_id.starts_with(ACTION_PREFIX_USE) {
        return apply_item_action(state, content, action_id);
    }

    let Some(encounter) = current_content_encounter(content, state) else {
        return apply_movement_action(state, content, location, action_id);
    };
    if encounter.event.is_some() && action_id == "event:continue" {
        return advance_event(state, encounter, action_id);
    }
    let Some(choice_id) = action_id.strip_prefix(ACTION_PREFIX_CHOICE) else {
        return Err(ContentActionError::UnknownAction(action_id.to_string()));
    };
    let Some(choice) = encounter
        .choices
        .iter()
        .find(|choice| choice.id == choice_id && choice_is_available(choice, state))
    else {
        return Err(ContentActionError::UnknownAction(action_id.to_string()));
    };
    if let Some(stage) = current_event_stage(encounter, state) {
        if stage.kind != "choice"
            || !stage
                .choices
                .iter()
                .any(|choice_ref| choice_ref.id == choice.id)
        {
            return Err(ContentActionError::UnknownAction(action_id.to_string()));
        }
    }

    let mut next_state = state.clone();
    next_state.last_check = None;
    let mut logs = Vec::new();
    apply_cost(&mut next_state.player, &choice.cost);
    logs.extend(apply_outcome(&mut next_state, content, &choice.outcome));
    if let Some(check) = &choice.check {
        let res = resolve_ability_check_with_content(
            state,
            content,
            check.ability.as_str(),
            check.difficulty,
        );
        let branch = if res.success {
            &check.success
        } else {
            &check.failure
        };
        next_state.last_check = Some(res);
        logs.extend(apply_outcome(&mut next_state, content, branch));
    }
    if let Some(event) = &encounter.event {
        let stage_index = effective_event_stage_index(encounter, state);
        let next_stage_id = event
            .stages
            .get(stage_index)
            .and_then(|stage| {
                stage
                    .choices
                    .iter()
                    .find(|choice_ref| choice_ref.id == choice.id)
            })
            .and_then(|choice_ref| choice_ref.next_stage_id.clone());
        next_state.active_event_id = Some(encounter.id.clone());
        if let Some(next_stage_id) = next_stage_id {
            if let Some((next_index, next_stage)) = event
                .stages
                .iter()
                .enumerate()
                .find(|(_, stage)| stage.id == next_stage_id)
            {
                if next_stage.kind == "result" {
                    // A direct ResultStage target gets its own cursor and a self-sentinel.
                    // advance_event uses that sentinel to avoid falling through to the
                    // next physical stage when this result has no explicit continuation.
                    next_state.event_stage_index = next_index;
                    next_state.event_next_stage_id = Some(next_stage.id.clone());
                } else {
                    // Legacy refs target the stage after the shared ResultStage.
                    next_state.event_stage_index = stage_index + 1;
                    next_state.event_next_stage_id = Some(next_stage_id);
                }
            } else {
                next_state.event_stage_index = stage_index + 1;
                next_state.event_next_stage_id = Some(next_stage_id);
            }
        } else {
            next_state.event_stage_index = stage_index + 1;
            next_state.event_next_stage_id = None;
        }
    }
    next_state.add_seen_encounter_once(&encounter.id);
    logs.extend(advance_turn(&mut next_state));
    if !logs.is_empty() {
        let combined_log = logs.join("\n");
        next_state.add_history_entry("action", &combined_log, Some(&encounter.id));
    }
    let newly_unlocked_achievements = unlock_achievements(&mut next_state, content);

    Ok(ActionResult {
        encounter_id: encounter.id.clone(),
        action_id: action_id.to_string(),
        state: next_state,
        logs,
        effect_cues: Vec::new(),
        newly_unlocked_achievements,
    })
}

pub fn printer_turn_view(state: &GameState) -> TurnView {
    TurnView {
        location_id: state.location_id.clone(),
        encounter_id: Some("printer_prints_alone".to_string()),
        ending_id: None,
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
        blocked_actions: Vec::new(),
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
                newly_unlocked_achievements: Vec::new(),
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
                newly_unlocked_achievements: Vec::new(),
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
            id: format!("{ACTION_PREFIX_MOVE}{destination_id}"),
            label: content
                .location(destination_id)
                .map(|destination| destination.name.clone())
                .unwrap_or_else(|| destination_id.clone()),
            cost_summary: None,
        })
        .collect()
}

fn item_action_views(state: &GameState, content: &ContentIndex) -> Vec<ActionView> {
    let mut actions = Vec::new();
    let mut seen = Vec::<String>::new();
    for item_id in &state.inventory {
        if seen.iter().any(|existing| existing == item_id) {
            continue;
        }
        seen.push(item_id.clone());
        let Some(item) = content.item(item_id) else {
            continue;
        };
        if !item.usable || item.use_effects.is_empty() {
            continue;
        }
        actions.push(ActionView {
            id: format!("{ACTION_PREFIX_USE}{}", item.id),
            label: item.name.clone(),
            cost_summary: None,
        });
    }
    actions
}

fn apply_movement_action(
    state: &GameState,
    content: &ContentIndex,
    location: &LocationDef,
    action_id: &str,
) -> Result<ActionResult, ContentActionError> {
    let Some(destination_id) = action_id.strip_prefix(ACTION_PREFIX_MOVE) else {
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
    next_state.last_check = None;
    next_state.location_id = destination_id.to_string();
    next_state.danger = (next_state.danger + destination.danger).max(0);
    let mut logs = vec![format!("{}로 이동했다.", destination.name)];
    logs.extend(advance_turn(&mut next_state));
    if !logs.is_empty() {
        let combined_log = logs.join("\n");
        next_state.add_history_entry("action", &combined_log, Some("movement"));
    }
    let newly_unlocked_achievements = unlock_achievements(&mut next_state, content);

    Ok(ActionResult {
        encounter_id: "movement".to_string(),
        action_id: action_id.to_string(),
        state: next_state,
        logs,
        effect_cues: Vec::new(),
        newly_unlocked_achievements,
    })
}

fn apply_item_action(
    state: &GameState,
    content: &ContentIndex,
    action_id: &str,
) -> Result<ActionResult, ContentActionError> {
    let Some(item_id) = action_id.strip_prefix(ACTION_PREFIX_USE) else {
        return Err(ContentActionError::UnknownAction(action_id.to_string()));
    };
    let Some(item) = content.item(item_id) else {
        return Err(ContentActionError::UnknownAction(action_id.to_string()));
    };
    if !state.inventory.iter().any(|candidate| candidate == item_id)
        || !item.usable
        || item.use_effects.is_empty()
    {
        return Err(ContentActionError::UnknownAction(action_id.to_string()));
    }

    let mut next_state = state.clone();
    next_state.last_check = None;
    for (resource, amount) in &item.use_effects {
        apply_player_resource_delta(&mut next_state.player, resource, *amount);
    }
    next_state.remove_inventory_item(item_id);
    let mut logs = vec![item_use_log(item)];
    logs.extend(advance_turn(&mut next_state));
    if !logs.is_empty() {
        let combined_log = logs.join("\n");
        next_state.add_history_entry("action", &combined_log, Some("item"));
    }
    let newly_unlocked_achievements = unlock_achievements(&mut next_state, content);

    Ok(ActionResult {
        encounter_id: "item".to_string(),
        action_id: action_id.to_string(),
        state: next_state,
        logs,
        effect_cues: Vec::new(),
        newly_unlocked_achievements,
    })
}

fn apply_train_action(
    state: &GameState,
    content: &ContentIndex,
    action_id: &str,
) -> Result<ActionResult, ContentActionError> {
    let Some(ability_id) = action_id.strip_prefix(ACTION_PREFIX_TRAIN) else {
        return Err(ContentActionError::UnknownAction(action_id.to_string()));
    };
    let Some(leveling) = content
        .runtime
        .as_ref()
        .and_then(|runtime| runtime.leveling.as_ref())
    else {
        return Err(ContentActionError::UnknownAction(action_id.to_string()));
    };
    if available_stat_points(state, content) == 0 || !VALID_ABILITY_IDS.contains(&ability_id) {
        return Err(ContentActionError::UnknownAction(action_id.to_string()));
    }
    let current = state.player.abilities.get(ability_id).copied().unwrap_or(0);
    if current >= 5 || leveling.thresholds.is_empty() {
        return Err(ContentActionError::UnknownAction(action_id.to_string()));
    }

    let mut next_state = state.clone();
    next_state.last_check = None;
    next_state
        .player
        .abilities
        .insert(ability_id.to_string(), current + 1);
    next_state.spent_stat_points += 1;
    let log = format!("+ {} 수련 1", ability_label(ability_id));
    next_state.add_history_entry("action", &log, Some("training"));
    Ok(ActionResult {
        encounter_id: "training".to_string(),
        action_id: action_id.to_string(),
        state: next_state,
        logs: vec![log],
        effect_cues: Vec::new(),
        newly_unlocked_achievements: Vec::new(),
    })
}

fn item_use_log(item: &ItemDef) -> String {
    item.use_log
        .clone()
        .unwrap_or_else(|| format!("{}을 사용했다.", item.name))
}

/// Returns the collapse encounter iff the bundle declares collapse runtime meta,
/// the player's health has hit 0 or below, and the gate's `used_flag` has not
/// yet been set on this state. Nothing else influences this decision — no
/// content-flag coupling beyond `used_flag` itself.
fn collapse_gate_pending<'a>(
    content: &'a ContentIndex,
    state: &GameState,
) -> Option<&'a crate::content::EncounterDef> {
    let collapse = content.runtime.as_ref()?.collapse.as_ref()?;
    if state.player.health > 0 {
        return None;
    }
    if state.flags.iter().any(|f| f == &collapse.used_flag) {
        return None;
    }
    content.encounter(&collapse.encounter_id)
}

fn current_content_ending<'a>(
    content: &'a ContentIndex,
    state: &GameState,
) -> Option<&'a crate::content::EndingDef> {
    if collapse_gate_pending(content, state).is_some() {
        return None;
    }

    content
        .endings()
        .filter(|ending| conditions_match(&ending.conditions, state))
        .max_by_key(|ending| ending.priority)
}

fn current_content_encounter<'a>(
    content: &'a ContentIndex,
    state: &GameState,
) -> Option<&'a EncounterDef> {
    if let Some(active_event_id) = &state.active_event_id {
        if let Some(encounter) = content.encounter(active_event_id) {
            if encounter.event.is_some() {
                return Some(encounter);
            }
        }
    }
    if let Some(enc) = collapse_gate_pending(content, state) {
        return Some(enc);
    }

    // The reward-pipeline cards are authored as a chronological early-game
    // sequence. ContentIndex intentionally keeps encounters in a BTreeMap for
    // stable lookup, so apply an explicit local priority before the generic
    // lexical scan to keep the story order independent of map ordering.
    const REWARD_PIPELINE_ORDER: [&str; 7] = [
        "wuxia_cheongryu_first_night_shelter",
        "wuxia_cheongryu_first_breathing_lesson",
        "wuxia_cheongryu_training_first_failure",
        "wuxia_cheongryu_medicine_errand",
        "wuxia_cheongryu_raid_omen",
        "wuxia_cheongryu_gate_patrol_first_trouble",
        "wuxia_seoharin_hides_training_injury",
    ];
    if let Some(encounter) = REWARD_PIPELINE_ORDER
        .iter()
        .filter_map(|id| content.encounter(id))
        .find(|encounter| encounter_is_available(encounter, state))
    {
        return Some(encounter);
    }

    content
        .encounters()
        .find(|encounter| encounter_is_available(encounter, state))
}

fn effective_event_stage_index(encounter: &EncounterDef, state: &GameState) -> usize {
    if state.active_event_id.as_deref() == Some(encounter.id.as_str()) {
        state.event_stage_index
    } else {
        0
    }
}

fn current_event_stage<'a>(
    encounter: &'a EncounterDef,
    state: &GameState,
) -> Option<&'a crate::content::EventStageDef> {
    encounter
        .event
        .as_ref()?
        .stages
        .get(effective_event_stage_index(encounter, state))
}

fn event_stage_text(stage: &crate::content::EventStageDef, state: &GameState) -> String {
    stage
        .visible_blocks(state.last_check.as_ref().map(|check| check.success))
        .filter_map(|block| block.text.as_deref())
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn advance_event(
    state: &GameState,
    encounter: &EncounterDef,
    action_id: &str,
) -> Result<ActionResult, ContentActionError> {
    let event = encounter.event.as_ref().expect("event checked by caller");
    let index = effective_event_stage_index(encounter, state);
    let stage = event
        .stages
        .get(index)
        .ok_or_else(|| ContentActionError::UnknownAction(action_id.to_string()))?;
    if stage.kind == "choice" {
        return Err(ContentActionError::UnknownAction(action_id.to_string()));
    }
    let direct_result_target =
        stage.kind == "result" && state.event_next_stage_id.as_deref() == Some(stage.id.as_str());
    let target = if stage.kind == "result" {
        if direct_result_target {
            stage.next_stage_id.as_ref()
        } else {
            state
                .event_next_stage_id
                .as_ref()
                .or(stage.next_stage_id.as_ref())
        }
    } else {
        stage.next_stage_id.as_ref()
    };
    let next_index = if direct_result_target && target.is_none() {
        event.stages.len()
    } else {
        target
            .and_then(|id| {
                event
                    .stages
                    .iter()
                    .position(|candidate| &candidate.id == id)
            })
            .unwrap_or(index + 1)
    };
    let mut next_state = state.clone();
    next_state.last_check = None;
    next_state.active_event_id = Some(encounter.id.clone());
    next_state.event_stage_index = next_index;
    if stage.kind == "result" {
        next_state.event_next_stage_id = None;
    }
    if next_index >= event.stages.len() {
        next_state.active_event_id = None;
        next_state.event_stage_index = 0;
        next_state.event_next_stage_id = None;
        next_state.add_seen_encounter_once(&encounter.id);
    }
    Ok(ActionResult {
        encounter_id: encounter.id.clone(),
        action_id: action_id.to_string(),
        state: next_state,
        logs: Vec::new(),
        effect_cues: Vec::new(),
        newly_unlocked_achievements: Vec::new(),
    })
}

fn apply_cost(player: &mut PlayerState, cost: &ResourceMap) {
    for (resource, amount) in cost {
        apply_player_resource_delta(player, resource, cost_delta(resource, *amount));
    }
}

fn apply_outcome(
    state: &mut GameState,
    content: &ContentIndex,
    outcome: &OutcomeDef,
) -> Vec<String> {
    let points_before = earned_stat_points(state.experience, content);
    for (resource, amount) in &outcome.resources {
        apply_player_resource_delta(&mut state.player, resource, *amount);
    }
    state.danger = (state.danger + outcome.danger).max(0);

    let mut delta_logs = Vec::new();

    // Resource deltas
    for (resource, amount) in &outcome.resources {
        if *amount > 0 {
            delta_logs.push(format!("+ {} {}", resource_label(resource), amount));
        } else if *amount < 0 {
            delta_logs.push(format!("- {} {}", resource_label(resource), -amount));
        }
    }

    // Item deltas
    for item_id in &outcome.remove_items {
        state.remove_inventory_item(item_id);
        let name = content
            .item(item_id)
            .map(|item| item.name.as_str())
            .unwrap_or(item_id);
        delta_logs.push(format!("- {}", name));
    }
    for item_id in &outcome.add_items {
        state.add_inventory_once(item_id);
        let name = content
            .item(item_id)
            .map(|item| item.name.as_str())
            .unwrap_or(item_id);
        delta_logs.push(format!("+ {}", name));
    }

    for flag in &outcome.remove_flags {
        state.remove_flag(flag);
    }
    for flag in &outcome.add_flags {
        state.add_flag_once(flag);
    }
    for clue in &outcome.add_clues {
        state.add_clue_once(clue);
    }
    for insight_id in &outcome.add_insights {
        if state.insights.iter().any(|owned| owned == insight_id) {
            continue;
        }
        state.insights.push(insight_id.clone());
        let name = content
            .insight(insight_id)
            .map(|insight| insight.name.as_str())
            .unwrap_or(insight_id);
        delta_logs.push(format!("+ 기연: {name}"));
    }
    for skill_id in &outcome.add_skills {
        if state.skills.iter().any(|owned| owned == skill_id) {
            continue;
        }
        state.skills.push(skill_id.clone());
        let name = content
            .skill(skill_id)
            .map(|skill| skill.name.as_str())
            .unwrap_or(skill_id);
        delta_logs.push(format!("+ 스킬: {name}"));
    }
    for title_id in &outcome.add_titles {
        if state.titles.iter().any(|owned| owned == title_id) {
            continue;
        }
        state.titles.push(title_id.clone());
        let name = content
            .title(title_id)
            .map(|title| title.name.as_str())
            .unwrap_or(title_id);
        delta_logs.push(format!("+ 칭호: {name}"));
    }
    for (key, delta) in &outcome.relationship_deltas {
        *state.relationships.entry(key.clone()).or_default() += delta;
        delta_logs.push(format!("관계 방향 기록: {key}"));
    }
    if let Some(destination_id) = &outcome.destination_id {
        state.location_id = destination_id.clone();
    }

    // Trait deltas
    if let Some(new_trait_id) = &outcome.set_trait {
        if let Some(prev_trait_id) = &state.trait_id {
            let prev_name = content
                .trait_def(prev_trait_id)
                .map(|t| t.name.as_str())
                .unwrap_or(prev_trait_id);
            delta_logs.push(format!("- 특성: {}", prev_name));
        }
        state.trait_id = Some(new_trait_id.clone());
        let new_name = content
            .trait_def(new_trait_id)
            .map(|t| t.name.as_str())
            .unwrap_or(new_trait_id);
        delta_logs.push(format!("+ 특성: {}", new_name));
    }

    // Experience deltas
    if let Some(exp_delta) = outcome.experience {
        let new_exp = (state.experience as i32 + exp_delta).max(0) as u32;
        state.experience = new_exp;
        if exp_delta > 0 {
            delta_logs.push(format!("+ 경험 {}", exp_delta));
        } else if exp_delta < 0 {
            delta_logs.push(format!("- 경험 {}", -exp_delta));
        }
    }

    let points_after = earned_stat_points(state.experience, content);
    if points_after > points_before {
        delta_logs.push(format!("+ 수련 기회 {}", points_after - points_before));
    }

    let mut logs: Vec<String> = outcome.log.iter().cloned().collect();
    logs.extend(delta_logs);
    logs
}

fn earned_stat_points(experience: u32, content: &ContentIndex) -> u32 {
    content
        .runtime
        .as_ref()
        .and_then(|runtime| runtime.leveling.as_ref())
        .map(|leveling| {
            leveling
                .thresholds
                .iter()
                .filter(|threshold| **threshold <= experience)
                .count() as u32
        })
        .unwrap_or(0)
}

pub fn available_stat_points(state: &GameState, content: &ContentIndex) -> u32 {
    earned_stat_points(state.experience, content).saturating_sub(state.spent_stat_points)
}

fn advance_turn(state: &mut GameState) -> Vec<String> {
    state.turn += 1;
    apply_player_resource_delta(&mut state.player, RESOURCE_HUNGER, 1);
    apply_player_resource_delta(&mut state.player, RESOURCE_THIRST, 2);

    if state.player.hunger >= 100 {
        apply_player_resource_delta(&mut state.player, RESOURCE_HEALTH, -2);
    }
    if state.player.thirst >= 100 {
        apply_player_resource_delta(&mut state.player, RESOURCE_HEALTH, -4);
        apply_player_resource_delta(&mut state.player, RESOURCE_SANITY, -2);
    }

    let mut logs = Vec::new();
    if state.player.thirst >= 60
        && !state
            .flags
            .iter()
            .any(|flag| flag == "pressure_thirst_warning_seen")
    {
        state.add_flag_once("pressure_thirst_warning_seen");
        logs.push("목이 마르자 가장 가까운 정수기 물소리가 한 박자 늦게 따라온다.".to_string());
    }
    if state.player.sanity > 0
        && state.player.sanity < 40
        && !state
            .flags
            .iter()
            .any(|flag| flag == "pressure_low_sanity_warning_seen")
    {
        state.add_flag_once("pressure_low_sanity_warning_seen");
        logs.push("선택지 문장이 화면 가장자리에서 흐려지기 시작했다.".to_string());
    }

    logs
}

fn unlock_achievements(state: &mut GameState, content: &ContentIndex) -> Vec<String> {
    let mut newly_unlocked = Vec::new();
    for achievement in content.achievements() {
        if achievement_unlocked(achievement, state)
            && state.add_unlocked_achievement_once(&achievement.id)
        {
            newly_unlocked.push(achievement.id.clone());
        }
    }
    newly_unlocked
}

fn achievement_unlocked(achievement: &AchievementDef, state: &GameState) -> bool {
    conditions_match(&achievement.conditions, state)
}

pub fn resolve_ability_check(
    state: &GameState,
    ability_id: &str,
    difficulty: i32,
) -> CheckResolution {
    resolve_ability_check_with_bonus(state, ability_id, difficulty, 0)
}

pub fn resolve_ability_check_with_content(
    state: &GameState,
    content: &ContentIndex,
    ability_id: &str,
    difficulty: i32,
) -> CheckResolution {
    resolve_ability_check_with_bonus(
        state,
        ability_id,
        difficulty,
        insight_bonus(state, content, ability_id),
    )
}

fn resolve_ability_check_with_bonus(
    state: &GameState,
    ability_id: &str,
    difficulty: i32,
    insight_bonus: i32,
) -> CheckResolution {
    let (first, second) = roll_2d6(&format!(
        "{}:{}:{}:{}",
        state.seed, state.turn, ability_id, difficulty
    ));
    let ability_value = player_ability(&state.player, ability_id);
    let total = first + second + ability_value + insight_bonus;
    let success = total >= difficulty;
    CheckResolution {
        ability_id: ability_id.to_string(),
        ability_label: crate::ability_label(ability_id).to_string(),
        dice: (first, second),
        ability_value,
        insight_bonus,
        difficulty,
        total,
        success,
    }
}

pub fn insight_bonus(state: &GameState, content: &ContentIndex, ability_id: &str) -> i32 {
    ordered_unique_insights(state, content)
        .into_iter()
        .filter_map(|insight| insight.check_bonus.as_ref())
        .filter(|bonus| bonus.ability == ability_id)
        .map(|bonus| bonus.bonus)
        .sum()
}

pub(crate) fn ordered_unique_insights<'a>(
    state: &'a GameState,
    content: &'a ContentIndex,
) -> Vec<&'a crate::content::InsightDef> {
    let mut seen = std::collections::BTreeSet::new();
    state
        .insights
        .iter()
        .filter(|id| seen.insert(id.as_str()))
        .filter_map(|id| content.insight(id))
        .collect()
}

fn roll_2d6(seed: &str) -> (i32, i32) {
    let hash = fnv1a_32(seed);
    ((hash % 6 + 1) as i32, ((hash / 6) % 6 + 1) as i32)
}

fn fnv1a_32(value: &str) -> u32 {
    let mut hash = 2_166_136_261u32;
    for byte in value.bytes() {
        hash ^= u32::from(byte);
        hash = hash.wrapping_mul(16_777_619);
    }
    hash
}

fn cost_delta(resource: &str, amount: i32) -> i32 {
    if matches!(resource, RESOURCE_HUNGER | RESOURCE_THIRST) {
        amount
    } else {
        -amount
    }
}

fn encounter_is_available(encounter: &EncounterDef, state: &GameState) -> bool {
    (encounter.repeatable
        || !state
            .seen_encounters
            .iter()
            .any(|seen_encounter| seen_encounter == &encounter.id))
        && conditions_match(&encounter.conditions, state)
        && encounter
            .choices
            .iter()
            .any(|choice| choice_is_available(choice, state))
}

fn choice_is_available(choice: &ChoiceDef, state: &GameState) -> bool {
    choice_unavailable_reasons(choice, state).is_empty()
}

fn choice_unavailable_reasons(choice: &ChoiceDef, state: &GameState) -> Vec<String> {
    let mut reasons = conditions_unavailable_reasons(&choice.conditions, state);
    reasons.extend(cost_unavailable_reasons(&choice.cost, &state.player));
    reasons
}

fn conditions_match(conditions: &ContentConditions, state: &GameState) -> bool {
    conditions_unavailable_reasons(conditions, state).is_empty()
}

fn conditions_unavailable_reasons(
    conditions: &ContentConditions,
    state: &GameState,
) -> Vec<String> {
    let mut reasons = Vec::new();
    if !conditions.locations.is_empty() && !conditions.locations.contains(&state.location_id) {
        reasons.push("현재 위치 조건 불일치".to_string());
    }
    if !conditions.disaster_types.is_empty()
        && !conditions.disaster_types.contains(&state.disaster_type)
    {
        reasons.push("재난 유형 조건 불일치".to_string());
    }
    for item in &conditions.required_items {
        if !state.inventory.contains(item) {
            reasons.push(format!("필요 아이템 없음: {item}"));
        }
    }
    for flag in &conditions.required_flags {
        if !state.flags.contains(flag) {
            reasons.push(format!("필요 플래그 없음: {flag}"));
        }
    }
    for flag in &conditions.forbidden_flags {
        if state.flags.contains(flag) {
            reasons.push(format!("이미 발생한 플래그: {flag}"));
        }
    }
    for clue in &conditions.required_clues {
        if !state.clues.contains(clue) {
            reasons.push(format!("필요 단서 없음: {clue}"));
        }
    }
    for (resource, minimum) in &conditions.min_resources {
        let current = player_resource(&state.player, resource);
        if current < *minimum {
            reasons.push(format!(
                "{} 부족: {current}/{minimum}",
                resource_label(resource)
            ));
        }
    }
    for (resource, maximum) in &conditions.max_resources {
        let current = player_resource(&state.player, resource);
        if current > *maximum {
            reasons.push(format!(
                "{} 초과: {current}/{maximum}",
                resource_label(resource)
            ));
        }
    }
    for (ability, minimum) in &conditions.min_abilities {
        let current = player_ability(&state.player, ability);
        if current < *minimum {
            reasons.push(format!("능력 조건 미충족: {ability} >= {minimum}"));
        }
    }
    if let Some(min_exp) = conditions.min_experience {
        if state.experience < min_exp {
            reasons.push(format!("경험 부족: {}/{}", state.experience, min_exp));
        }
    }
    reasons
}

fn cost_unavailable_reasons(cost: &ResourceMap, player: &PlayerState) -> Vec<String> {
    cost.iter()
        .filter(|(resource, amount)| {
            is_spendable_resource(resource)
                && **amount > 0
                && player_resource(player, resource) < **amount
        })
        .map(|(resource, amount)| {
            let current = player_resource(player, resource);
            format!("{} 부족: {current}/{amount}", resource_label(resource))
        })
        .collect()
}

fn is_spendable_resource(resource: &str) -> bool {
    matches!(
        resource,
        RESOURCE_HEALTH | RESOURCE_SANITY | RESOURCE_BATTERY
    )
}

fn choice_action_view(choice: &ChoiceDef) -> ActionView {
    ActionView {
        id: format!("{ACTION_PREFIX_CHOICE}{}", choice.id),
        label: choice.label.clone(),
        cost_summary: format_cost_summary(&choice.cost),
    }
}

fn blocked_choice_action_view(choice: &ChoiceDef, state: &GameState) -> BlockedActionView {
    BlockedActionView {
        id: format!("{ACTION_PREFIX_CHOICE}{}", choice.id),
        label: choice.label.clone(),
        cost_summary: format_cost_summary(&choice.cost),
        reasons: choice_unavailable_reasons(choice, state),
    }
}

fn format_cost_summary(cost: &ResourceMap) -> Option<String> {
    let parts = cost
        .iter()
        .filter(|(_resource, amount)| **amount != 0)
        .map(|(resource, amount)| {
            let delta = cost_delta(resource, *amount);
            let sign = if delta > 0 { "+" } else { "-" };
            format!("{} {}{}", resource_label(resource), sign, delta.abs())
        })
        .collect::<Vec<_>>();

    if parts.is_empty() {
        None
    } else {
        Some(parts.join(", "))
    }
}

fn apply_player_resource_delta(player: &mut PlayerState, resource: &str, amount: i32) {
    match resource {
        RESOURCE_HEALTH => player.health = clamp_resource(player.health + amount),
        RESOURCE_SANITY => player.sanity = clamp_resource(player.sanity + amount),
        RESOURCE_BATTERY => player.battery = clamp_resource(player.battery + amount),
        RESOURCE_HUNGER => player.hunger = clamp_resource(player.hunger + amount),
        RESOURCE_THIRST => player.thirst = clamp_resource(player.thirst + amount),
        _ => {}
    }
}

fn clamp_resource(value: i32) -> i32 {
    value.clamp(0, 100)
}

fn player_resource(player: &PlayerState, resource: &str) -> i32 {
    match resource {
        RESOURCE_HEALTH => player.health,
        RESOURCE_SANITY => player.sanity,
        RESOURCE_BATTERY => player.battery,
        RESOURCE_HUNGER => player.hunger,
        RESOURCE_THIRST => player.thirst,
        _ => 0,
    }
}

fn player_ability(player: &PlayerState, ability: &str) -> i32 {
    player.abilities.get(ability).copied().unwrap_or(0)
}

fn resource_label(resource: &str) -> &str {
    match resource {
        RESOURCE_HEALTH => "체력",
        RESOURCE_SANITY => "정신력",
        RESOURCE_BATTERY => "배터리",
        RESOURCE_HUNGER => "허기",
        RESOURCE_THIRST => "갈증",
        other => other,
    }
}

pub fn ability_label(ability: &str) -> &str {
    match ability {
        "logic" => "논리",
        "empathy" => "공감",
        "volition" => "의지",
        "composure" => "평정",
        "interface" => "인터페이스",
        "physical" => "신체",
        other => other,
    }
}

/// P(2d6 + ability >= difficulty)를 백분율로 계산한다.
/// 2d6의 분포는 고정 표를 따른다.
/// 실제 굴림 `roll_2d6`는 seed/turn 기반 결정론 해시이지만,
/// 표기 확률은 시드 전체에 대한 사전 확률(prior probability)이며,
/// 이는 레퍼런스의 사전 공개 문법과 부합한다.
pub fn ability_check_success_percent(ability: i32, difficulty: i32) -> f32 {
    let need = difficulty - ability;
    if need <= 2 {
        return 100.0;
    }
    if need > 12 {
        return 0.0;
    }
    let count = match need {
        3 => 35,
        4 => 33,
        5 => 30,
        6 => 26,
        7 => 21,
        8 => 15,
        9 => 10,
        10 => 6,
        11 => 3,
        12 => 1,
        _ => unreachable!(),
    };
    ((count as f32 / 36.0 * 100.0 * 10.0).round()) / 10.0
}

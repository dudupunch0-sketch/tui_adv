use crate::content::{
    AchievementDef, ChoiceDef, ContentConditions, ContentIndex, EncounterDef, ItemDef, LocationDef,
    OutcomeDef, ResourceMap,
};
use crate::effects::{printer_glyph_anomaly_cue, EffectCue};
use crate::state::{GameState, PlayerState};
use serde::Serialize;

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

    let mut actions = encounter
        .choices
        .iter()
        .filter(|choice| choice_is_available(choice, state))
        .map(choice_action_view)
        .collect::<Vec<_>>();
    actions.extend(item_actions);

    Ok(TurnView {
        location_id: state.location_id.clone(),
        encounter_id: Some(encounter.id.clone()),
        ending_id: None,
        title: encounter.title.clone(),
        body: encounter.body.clone(),
        actions,
        blocked_actions: encounter
            .choices
            .iter()
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

    if action_id.starts_with("use:") {
        return apply_item_action(state, content, action_id);
    }

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
    let mut logs = Vec::new();
    apply_cost(&mut next_state.player, &choice.cost);
    logs.extend(apply_outcome(&mut next_state, &choice.outcome));
    if let Some(check) = &choice.check {
        let branch = if ability_check_succeeds(state, check.ability.as_str(), check.difficulty) {
            &check.success
        } else {
            &check.failure
        };
        logs.extend(apply_outcome(&mut next_state, branch));
    }
    next_state.add_seen_encounter_once(&encounter.id);
    logs.extend(advance_turn(&mut next_state));
    for log in &logs {
        next_state.add_history_entry("action", log, Some(&encounter.id));
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
            id: format!("move:{destination_id}"),
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
            id: format!("use:{}", item.id),
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
    next_state.location_id = destination_id.to_string();
    next_state.danger = (next_state.danger + destination.danger).max(0);
    let mut logs = vec![format!("{}로 이동했다.", destination.name)];
    logs.extend(advance_turn(&mut next_state));
    for log in &logs {
        next_state.add_history_entry("action", log, Some("movement"));
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
    let Some(item_id) = action_id.strip_prefix("use:") else {
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
    for (resource, amount) in &item.use_effects {
        apply_player_resource_delta(&mut next_state.player, resource, *amount);
    }
    next_state.remove_inventory_item(item_id);
    let mut logs = vec![item_use_log(item)];
    logs.extend(advance_turn(&mut next_state));
    for log in &logs {
        next_state.add_history_entry("action", log, Some("item"));
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

fn item_use_log(item: &ItemDef) -> String {
    item.use_log
        .clone()
        .unwrap_or_else(|| format!("{}을 사용했다.", item.name))
}

fn current_content_ending<'a>(
    content: &'a ContentIndex,
    state: &GameState,
) -> Option<&'a crate::content::EndingDef> {
    content
        .endings()
        .filter(|ending| conditions_match(&ending.conditions, state))
        .max_by_key(|ending| ending.priority)
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
        apply_player_resource_delta(player, resource, cost_delta(resource, *amount));
    }
}

fn apply_outcome(state: &mut GameState, outcome: &OutcomeDef) -> Vec<String> {
    for (resource, amount) in &outcome.resources {
        apply_player_resource_delta(&mut state.player, resource, *amount);
    }
    state.danger = (state.danger + outcome.danger).max(0);
    for item in &outcome.remove_items {
        state.remove_inventory_item(item);
    }
    for item in &outcome.add_items {
        state.add_inventory_once(item);
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
    if let Some(destination_id) = &outcome.destination_id {
        state.location_id = destination_id.clone();
    }
    outcome.log.iter().cloned().collect()
}

fn advance_turn(state: &mut GameState) -> Vec<String> {
    state.turn += 1;
    apply_player_resource_delta(&mut state.player, "hunger", 1);
    apply_player_resource_delta(&mut state.player, "thirst", 2);

    if state.player.hunger >= 100 {
        apply_player_resource_delta(&mut state.player, "health", -2);
    }
    if state.player.thirst >= 100 {
        apply_player_resource_delta(&mut state.player, "health", -4);
        apply_player_resource_delta(&mut state.player, "sanity", -2);
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

fn ability_check_succeeds(state: &GameState, ability: &str, difficulty: i32) -> bool {
    let (first, second) = roll_2d6(&format!(
        "{}:{}:{}:{}",
        state.seed, state.turn, ability, difficulty
    ));
    first + second + player_ability(&state.player, ability) >= difficulty
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
    if matches!(resource, "hunger" | "thirst") {
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
    matches!(resource, "health" | "sanity" | "battery")
}

fn choice_action_view(choice: &ChoiceDef) -> ActionView {
    ActionView {
        id: format!("choice:{}", choice.id),
        label: choice.label.clone(),
        cost_summary: format_cost_summary(&choice.cost),
    }
}

fn blocked_choice_action_view(choice: &ChoiceDef, state: &GameState) -> BlockedActionView {
    BlockedActionView {
        id: format!("choice:{}", choice.id),
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
        "health" => player.health = clamp_resource(player.health + amount),
        "sanity" => player.sanity = clamp_resource(player.sanity + amount),
        "battery" => player.battery = clamp_resource(player.battery + amount),
        "hunger" => player.hunger = clamp_resource(player.hunger + amount),
        "thirst" => player.thirst = clamp_resource(player.thirst + amount),
        _ => {}
    }
}

fn clamp_resource(value: i32) -> i32 {
    value.clamp(0, 100)
}

fn player_resource(player: &PlayerState, resource: &str) -> i32 {
    match resource {
        "health" => player.health,
        "sanity" => player.sanity,
        "battery" => player.battery,
        "hunger" => player.hunger,
        "thirst" => player.thirst,
        _ => 0,
    }
}

fn player_ability(player: &PlayerState, ability: &str) -> i32 {
    player.abilities.get(ability).copied().unwrap_or(0)
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

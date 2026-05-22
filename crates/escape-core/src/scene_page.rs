use crate::content::{ContentIndex, EncounterDef, LocationDef, PresentationEffectCue};
use crate::effects::EffectCue;
use crate::state::{GameHistoryEntry, GameState, PlayerState};
use crate::turn::{content_turn_view, ActionView, BlockedActionView, ContentTurnError, TurnView};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SceneMode {
    Encounter,
    Movement,
    Ending,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScenePage {
    pub mode: SceneMode,
    pub title: String,
    pub location: SceneLocation,
    pub chapter_label: String,
    pub status_summary: StatusSummary,
    pub body_blocks: Vec<BodyBlock>,
    pub dialogue_entries: Vec<DialogueEntry>,
    pub visual: SceneVisual,
    pub actions: Vec<SceneAction>,
    pub blocked_actions: Vec<SceneBlockedAction>,
    pub history_entries: Vec<HistoryEntry>,
    pub inventory_summary: InventorySummary,
    pub achievement_summary: AchievementSummary,
    pub pressure_cues: Vec<PressureCue>,
    pub effect_cues: Vec<SceneEffectCue>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SceneLocation {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusSummary {
    pub turn: u32,
    pub danger: i32,
    pub resources: Vec<ResourceStatus>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceStatus {
    pub id: String,
    pub label: String,
    pub band: String,
    pub text: String,
    pub value: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BodyBlock {
    pub kind: String,
    pub text: String,
    pub source_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogueEntry {
    pub speaker: String,
    pub text: String,
    pub source_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SceneVisual {
    pub id: String,
    pub kind: String,
    pub alt: String,
    pub source_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SceneAction {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub cost_text: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SceneBlockedAction {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub cost_text: Option<String>,
    pub reasons: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub kind: String,
    pub text: String,
    pub source_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventorySummary {
    pub items: Vec<String>,
    pub overflow_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AchievementSummary {
    pub unlocked: Vec<String>,
    pub newly_unlocked: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PressureCue {
    pub kind: String,
    pub severity: String,
    pub message: String,
    pub resource_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SceneEffectCue {
    pub kind: String,
    pub source: String,
    pub intensity: f32,
    pub stable_terms: Vec<String>,
    pub distortion: String,
    pub duration_hint_ms: Option<u32>,
    pub fallback_text: Option<String>,
}

pub fn scene_page_from_content(
    state: &GameState,
    content: &ContentIndex,
) -> Result<ScenePage, ContentTurnError> {
    let view = content_turn_view(state, content)?;
    let location = content
        .location(&state.location_id)
        .ok_or_else(|| ContentTurnError::UnknownStateLocation(state.location_id.clone()))?;

    let encounter = view
        .encounter_id
        .as_deref()
        .and_then(|encounter_id| content.encounter(encounter_id));

    Ok(scene_page_from_turn_view(state, location, encounter, &view))
}

fn scene_page_from_turn_view(
    state: &GameState,
    location: &LocationDef,
    encounter: Option<&EncounterDef>,
    view: &TurnView,
) -> ScenePage {
    let source_id = view
        .encounter_id
        .as_ref()
        .cloned()
        .unwrap_or_else(|| location.id.clone());
    let mode = if view.encounter_id.is_some() {
        SceneMode::Encounter
    } else {
        SceneMode::Movement
    };
    let default_visual_kind = if view.encounter_id.is_some() {
        "encounter"
    } else {
        "location"
    };
    let presentation = encounter.and_then(|encounter| encounter.presentation.as_ref());
    let visual_id = presentation
        .and_then(|presentation| presentation.visual_id.as_deref())
        .map(str::to_string)
        .unwrap_or_else(|| format!("{default_visual_kind}:{source_id}"));
    let visual_kind = presentation
        .and_then(|presentation| presentation.layout.as_deref())
        .unwrap_or(default_visual_kind)
        .to_string();
    let dialogue_entries = presentation
        .and_then(|presentation| presentation.speaker.as_ref())
        .map(|speaker| {
            vec![DialogueEntry {
                speaker: speaker.clone(),
                text: view.body.clone(),
                source_id: Some(source_id.clone()),
            }]
        })
        .unwrap_or_default();

    ScenePage {
        mode,
        title: view.title.clone(),
        location: SceneLocation {
            id: location.id.clone(),
            name: location.name.clone(),
            description: location.description.clone(),
        },
        chapter_label: format!("격리 {}턴", state.turn),
        status_summary: status_summary(state),
        body_blocks: vec![BodyBlock {
            kind: "narration".to_string(),
            text: view.body.clone(),
            source_id: Some(source_id.clone()),
        }],
        dialogue_entries,
        visual: SceneVisual {
            id: visual_id,
            kind: visual_kind,
            alt: view.title.clone(),
            source_id: Some(source_id),
        },
        actions: view.actions.iter().map(scene_action).collect(),
        blocked_actions: view
            .blocked_actions
            .iter()
            .map(scene_blocked_action)
            .collect(),
        history_entries: state.history.iter().map(history_entry).collect(),
        inventory_summary: InventorySummary {
            items: Vec::new(),
            overflow_count: 0,
        },
        achievement_summary: AchievementSummary {
            unlocked: Vec::new(),
            newly_unlocked: Vec::new(),
        },
        pressure_cues: pressure_cues(&state.player),
        effect_cues: presentation_effect_cues(presentation, view),
    }
}

fn status_summary(state: &GameState) -> StatusSummary {
    let resources = vec![
        health_status(state.player.health),
        sanity_status(state.player.sanity),
        battery_status(state.player.battery),
    ];
    let warnings = pressure_cues(&state.player)
        .iter()
        .map(|cue| cue.message.clone())
        .collect();

    StatusSummary {
        turn: state.turn,
        danger: state.danger,
        resources,
        warnings,
    }
}

fn health_status(value: i32) -> ResourceStatus {
    let (band, text) = if value <= 20 {
        ("critical", "붕괴 직전")
    } else if value <= 50 {
        ("warning", "불안정")
    } else {
        ("normal", "정상 범위")
    };
    resource_status("health", "신체 반응", band, text, value)
}

fn sanity_status(value: i32) -> ResourceStatus {
    let (band, text) = if value <= 20 {
        ("critical", "붕괴 직전")
    } else if value <= 30 {
        ("warning", "불안정")
    } else {
        ("normal", "안정")
    };
    resource_status("sanity", "집중도", band, text, value)
}

fn battery_status(value: i32) -> ResourceStatus {
    let band = if value <= 10 {
        "critical"
    } else if value <= 20 {
        "warning"
    } else {
        "normal"
    };
    resource_status("battery", "단말기 전원", band, &format!("{value}%"), value)
}

fn resource_status(id: &str, label: &str, band: &str, text: &str, value: i32) -> ResourceStatus {
    ResourceStatus {
        id: id.to_string(),
        label: label.to_string(),
        band: band.to_string(),
        text: text.to_string(),
        value,
    }
}

fn pressure_cues(player: &PlayerState) -> Vec<PressureCue> {
    let mut cues = Vec::new();
    if player.sanity <= 30 {
        cues.push(PressureCue {
            kind: "low_sanity".to_string(),
            severity: severity_for_low_resource(player.sanity, 20),
            message: "집중도가 흔들리고 있습니다. 일부 기록이 다르게 보일 수 있습니다.".to_string(),
            resource_id: "sanity".to_string(),
        });
    }
    if player.battery <= 20 {
        cues.push(PressureCue {
            kind: "low_battery".to_string(),
            severity: severity_for_low_resource(player.battery, 10),
            message: "단말기 전원이 낮습니다. 전력 행동이 제한될 수 있습니다.".to_string(),
            resource_id: "battery".to_string(),
        });
    }
    cues
}

fn severity_for_low_resource(value: i32, critical_at: i32) -> String {
    if value <= critical_at {
        "critical"
    } else {
        "warning"
    }
    .to_string()
}

fn scene_action(action: &ActionView) -> SceneAction {
    SceneAction {
        id: action.id.clone(),
        label: action.label.clone(),
        kind: action_kind(&action.id).to_string(),
        cost_text: action.cost_summary.clone(),
    }
}

fn scene_blocked_action(action: &BlockedActionView) -> SceneBlockedAction {
    SceneBlockedAction {
        id: action.id.clone(),
        label: action.label.clone(),
        kind: action_kind(&action.id).to_string(),
        cost_text: action.cost_summary.clone(),
        reasons: action.reasons.clone(),
    }
}

fn history_entry(entry: &GameHistoryEntry) -> HistoryEntry {
    HistoryEntry {
        kind: entry.kind.clone(),
        text: entry.text.clone(),
        source_id: entry.source_id.clone(),
    }
}

fn presentation_effect_cues(
    presentation: Option<&crate::content::PresentationDef>,
    view: &TurnView,
) -> Vec<SceneEffectCue> {
    if let Some(presentation) = presentation {
        if !presentation.effect_cues.is_empty() {
            return presentation
                .effect_cues
                .iter()
                .map(scene_effect_cue_from_presentation)
                .collect();
        }
    }

    view.effect_cues
        .iter()
        .map(scene_effect_cue_from_turn)
        .collect()
}

fn scene_effect_cue_from_presentation(cue: &PresentationEffectCue) -> SceneEffectCue {
    SceneEffectCue {
        kind: cue.kind.clone(),
        source: cue.source.clone(),
        intensity: cue.intensity,
        stable_terms: cue.stable_terms.clone(),
        distortion: cue.distortion.clone(),
        duration_hint_ms: cue.duration_hint_ms,
        fallback_text: cue.fallback_text.clone(),
    }
}

fn scene_effect_cue_from_turn(cue: &EffectCue) -> SceneEffectCue {
    match cue {
        EffectCue::GlyphAnomaly(details) => SceneEffectCue {
            kind: "glyph_anomaly".to_string(),
            source: details.source.clone(),
            intensity: f32::from(details.intensity) / 100.0,
            stable_terms: details.stable_terms.clone(),
            distortion: details.distortion.clone(),
            duration_hint_ms: None,
            fallback_text: None,
        },
    }
}

fn action_kind(action_id: &str) -> &str {
    if action_id.starts_with("choice:") {
        "choice"
    } else if action_id.starts_with("move:") {
        "move"
    } else if action_id.starts_with("use:") {
        "use"
    } else {
        "unknown"
    }
}

mod content;
mod effects;
mod save;
mod scene_page;
mod state;
mod turn;

pub use content::{
    index_content_bundle, load_content_bundle, validate_content_bundle, AbilityCheckDef,
    AchievementDef, ChoiceDef, ContentBundle, ContentBundleError, ContentConditions, ContentIndex,
    ContentIndexError, ContentManifest, ContentSections, EncounterDef, EndingDef, ItemDef,
    LocationDef, OutcomeDef, PresentationDef, PresentationEffectCue, PublicSecretDef, ResourceMap,
    CONTENT_BUNDLE_KIND, CONTENT_BUNDLE_SCHEMA_VERSION,
};
pub use effects::{printer_glyph_anomaly_cue, EffectCue, GlyphAnomalyCue};
pub use save::{load_state, save_state, SaveEnvelope, SaveError, SAVE_SCHEMA_VERSION};
pub use scene_page::{
    scene_page_from_content, AchievementSummary, BodyBlock, DialogueEntry, HistoryEntry,
    InventorySummary, PressureCue, ResourceStatus, SceneAction, SceneBlockedAction, SceneEffectCue,
    SceneLocation, SceneMode, ScenePage, SceneVisual, StatusSummary,
};
pub use state::{GameState, NewGameError, PlayerState, DEFAULT_START_LOCATION_ID};
pub use turn::{
    ActionError, ActionResult, ActionView, BlockedActionView, ContentActionError, ContentTurnError,
    TurnView,
};

pub fn new_game(seed: u64) -> GameState {
    GameState::new_printer_scene(seed)
}

pub fn new_game_from_content(seed: u64, content: &ContentIndex) -> Result<GameState, NewGameError> {
    GameState::new_from_content(seed, content)
}

pub fn new_game_from_content_at(
    seed: u64,
    content: &ContentIndex,
    start_location_id: &str,
) -> Result<GameState, NewGameError> {
    GameState::new_from_content_at(seed, content, start_location_id)
}

pub fn turn_view(state: &GameState) -> TurnView {
    turn::printer_turn_view(state)
}

pub fn turn_view_from_content(
    state: &GameState,
    content: &ContentIndex,
) -> Result<TurnView, ContentTurnError> {
    turn::content_turn_view(state, content)
}

pub fn apply_action_from_content(
    state: &GameState,
    content: &ContentIndex,
    action_id: &str,
) -> Result<ActionResult, ContentActionError> {
    turn::apply_content_action(state, content, action_id)
}

pub fn apply_action(state: &GameState, action_id: &str) -> Result<ActionResult, ActionError> {
    turn::apply_printer_action(state, action_id)
}

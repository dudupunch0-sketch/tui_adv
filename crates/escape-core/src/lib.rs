mod content;
mod effects;
mod save;
mod state;
mod turn;

pub use content::{
    load_content_bundle, validate_content_bundle, ContentBundle, ContentBundleError,
    ContentManifest, ContentSections, CONTENT_BUNDLE_KIND, CONTENT_BUNDLE_SCHEMA_VERSION,
};
pub use effects::{printer_glyph_anomaly_cue, EffectCue, GlyphAnomalyCue};
pub use save::{load_state, save_state, SaveEnvelope, SaveError, SAVE_SCHEMA_VERSION};
pub use state::{GameState, PlayerState};
pub use turn::{ActionError, ActionResult, ActionView, TurnView};

pub fn new_game(seed: u64) -> GameState {
    GameState::new_printer_scene(seed)
}

pub fn turn_view(state: &GameState) -> TurnView {
    turn::printer_turn_view(state)
}

pub fn apply_action(state: &GameState, action_id: &str) -> Result<ActionResult, ActionError> {
    turn::apply_printer_action(state, action_id)
}

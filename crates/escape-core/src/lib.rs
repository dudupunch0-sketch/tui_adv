mod combat_conclusion;
mod combat_contract;
mod combat_execution;
mod combat_hex;
mod combat_opportunity;
mod combat_resolution;
mod combat_simulation;
mod combat_spectator;
mod combat_state;
mod content;
mod effects;
mod final_epilogue;
mod resources;
mod save;
mod scene_page;
mod state;
mod turn;

pub use combat_conclusion::{
    conclude as conclude_combat, side_all_defeated, CombatCombatantReport, CombatConclusionError,
    CombatConclusionOutcome, CombatConclusionReason, CombatConclusionReport,
    CombatConclusionRequest, CombatTerminationPolicy,
};
pub use combat_contract::{
    CombatContractError, CombatEffectRef, CombatManifest, CombatRngNamespace,
    CombatSimulationVersion, SuppressedCombatEffect,
};
pub use combat_execution::{
    execute as execute_combat, CombatExecutionError, CombatExecutionRequest, CombatExecutionResult,
    CombatLogEvent, CombatLogImportance, CombatLogTag, CombatPresentationSpeed, CombatProvenance,
    CombatRunMode,
};
pub use combat_hex::{range, ring, HexCoord, HexError};
pub use combat_opportunity::{
    CombatDetectionLevel, CombatDetectionThresholds, CombatInterventionBudget, CombatObserver,
    CombatOpportunityCandidate, CombatOpportunityCatalog, CombatOpportunityContext,
    CombatOpportunityDefinition, CombatOpportunityError, CombatOpportunityEvaluation,
    CombatOpportunityInstance, CombatResponseDefinition, CombatResponseOption,
};
pub use combat_resolution::{
    resolve as resolve_combat, CombatAttackDefinition, CombatAttackEffect, CombatAttackOutcome,
    CombatDefenseProfile, CombatResolutionCombatant, CombatResolutionError, CombatResolutionFrame,
    CombatResolutionLogEvent, CombatResolutionLogTag, CombatResolutionRequest,
    CombatResolutionResult, CombatResolutionState,
};
pub use combat_simulation::{
    CombatFacing, CombatMoveIntent, CombatMoveMode, CombatPosition, CombatRolePreset,
    CombatRoleWeights, CombatSide, CombatSimulation, CombatSimulationConfig, CombatSimulationError,
    CombatSimulationInput, CombatSimulationParticipant, CombatTargetFallback, CombatTargetPolicy,
    CombatTargetPreference, CombatTickFrame,
};
pub use combat_spectator::{
    spectate as spectate_combat, CombatSpectatorCue, CombatSpectatorError, CombatSpectatorFrame,
    CombatSpectatorLogEntry, CombatSpectatorPage, CombatSpectatorPiece, CombatSpectatorRequest,
    CombatSpectatorView,
};
pub use combat_state::{
    CombatConclusion, CombatEffectCatalog, CombatEffectCategory, CombatEffectDecision,
    CombatEffectDefinition, CombatEffectInstance, CombatInitialStateProjection,
    CombatPreCombatInput, CombatState, CombatStateError, CombatantState, EffectLifetime,
    EffectPhase, EffectStacking, EffectVisibility, EnvironmentState, PersistentCombatStatus,
    Posture, RelationshipState, TeamFormationState, WeaponControl,
};
pub use content::{
    index_content_bundle, load_content_bundle, validate_content_bundle, AbilityCheckDef,
    AchievementDef, CheckBonusDef, ChoiceDef, ContentBlockDef, ContentBundle, ContentBundleError,
    ContentConditions, ContentIndex, ContentIndexError, ContentManifest, ContentSections,
    EncounterCombatDef, EncounterCombatKind, EncounterDef, EndingDef, EventChoiceRef, EventDef,
    EventStageDef, InsightDef, ItemDef, LevelingMetadata, LocationDef, OutcomeDef, PresentationDef,
    PresentationEffectCue, PublicSecretDef, ResourceMap, RewardDef, RuntimeMetadata, TraitDef,
    CONTENT_BUNDLE_KIND, CONTENT_BUNDLE_SCHEMA_VERSION,
};
pub use effects::{printer_glyph_anomaly_cue, EffectCue, GlyphAnomalyCue};
pub use save::{load_state, save_state, SaveEnvelope, SaveError, SAVE_SCHEMA_VERSION};
pub use scene_page::{
    scene_page_from_content, AchievementSummary, BodyBlock, DialogueEntry, HistoryEntry,
    InsightStatus, InventorySummary, ItemDetail, PressureCue, ResourceStatus, RewardStatus,
    SceneAction, SceneBlockedAction, SceneContentItem, SceneEffectCue, SceneLocation, SceneMode,
    ScenePage, SceneVisual, StatusSummary,
};
pub use state::{GameState, NewGameError, PlayerState, DEFAULT_START_LOCATION_ID};
pub use turn::{
    ability_check_success_percent, ability_label, available_stat_points, insight_bonus,
    resolve_ability_check, resolve_ability_check_with_content, ActionError, ActionResult,
    ActionView, BlockedActionView, ContentActionError, ContentTurnError, TurnView,
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

use escape_core::{
    apply_action_from_content, index_content_bundle, load_content_bundle, load_state,
    new_game_from_content, save_state, scene_page_from_content, ContentIndex, GameState,
    SaveEnvelope,
};
use serde::Serialize;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn new_game_json(seed: u64, content_bundle_json: &str) -> Result<String, String> {
    let content = content_index_from_json(content_bundle_json)?;
    let state = new_game_from_content(seed, &content).map_err(|error| error.to_string())?;
    to_json(&state, "state")
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn scene_page_json(state_json: &str, content_bundle_json: &str) -> Result<String, String> {
    let state = state_from_json(state_json)?;
    let content = content_index_from_json(content_bundle_json)?;
    let page = scene_page_from_content(&state, &content).map_err(|error| error.to_string())?;
    to_json(&page, "scene page")
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn apply_action_json(
    state_json: &str,
    content_bundle_json: &str,
    action_id: &str,
) -> Result<String, String> {
    let state = state_from_json(state_json)?;
    let content = content_index_from_json(content_bundle_json)?;
    let result = apply_action_from_content(&state, &content, action_id)
        .map_err(|error| error.to_string())?;
    to_json(&result, "action result")
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn save_state_json(state_json: &str) -> Result<String, String> {
    let state = state_from_json(state_json)?;
    let envelope = save_state(&state);
    to_json(&envelope, "save envelope")
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub fn load_state_json(save_json: &str) -> Result<String, String> {
    let envelope: SaveEnvelope =
        serde_json::from_str(save_json).map_err(|error| format!("invalid save JSON: {error}"))?;
    let state = load_state(&envelope).map_err(|error| error.to_string())?;
    to_json(&state, "state")
}

fn content_index_from_json(content_bundle_json: &str) -> Result<ContentIndex, String> {
    let bundle = load_content_bundle(content_bundle_json).map_err(|error| error.to_string())?;
    index_content_bundle(&bundle).map_err(|error| error.to_string())
}

fn state_from_json(state_json: &str) -> Result<GameState, String> {
    serde_json::from_str(state_json).map_err(|error| format!("invalid state JSON: {error}"))
}

fn to_json<T: Serialize>(value: &T, label: &str) -> Result<String, String> {
    serde_json::to_string(value).map_err(|error| format!("failed to serialize {label}: {error}"))
}

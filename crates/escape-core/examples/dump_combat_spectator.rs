//! Dump the real `ScenePage.combat` spectator payload as JSON.
//!
//! Renderer prototypes must be driven by frames the core actually produced,
//! not by hand-written coordinates: a board that looks good on invented data
//! proves nothing about the board that ships. This example resolves the one
//! authored systemic combat encounter
//! (`wuxia_combat_spectator_preview_bout`, gated behind
//! `combat_spectator_preview_unlocked`) and prints the resulting
//! `CombatSpectatorPage` to stdout.
//!
//! ```bash
//! cargo run -p escape-core --example dump_combat_spectator -- 2 > frames.json
//! ```
//!
//! The optional argument is the run seed (default 2). The payload is
//! deterministic for a given seed, so the same command always produces the
//! same bytes -- which is what lets a captured prototype frame be compared
//! before and after a renderer change.

use escape_core::{
    index_content_bundle, load_content_bundle, new_game_from_content_at, scene_page_from_content,
};

const WUXIA_BUNDLE: &str =
    include_str!("../fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json");
const SPECTATOR_LOCATION_ID: &str = "cheongryu_outer_courtyard";
const SPECTATOR_GATE_FLAG: &str = "combat_spectator_preview_unlocked";

fn main() {
    let seed: u64 = std::env::args()
        .nth(1)
        .and_then(|raw| raw.parse().ok())
        .unwrap_or(2);

    let bundle = load_content_bundle(WUXIA_BUNDLE).expect("wuxia preview bundle should load");
    let index = index_content_bundle(&bundle).expect("wuxia preview bundle should index");
    let mut state = new_game_from_content_at(seed, &index, SPECTATOR_LOCATION_ID)
        .expect("game should start at the Qingliu outer courtyard");
    state.flags.push(SPECTATOR_GATE_FLAG.to_string());

    let page = scene_page_from_content(&state, &index).expect("scene page should render");
    let combat = page
        .combat
        .expect("the gated systemic combat encounter should fill ScenePage.combat");

    println!(
        "{}",
        serde_json::to_string_pretty(&combat).expect("spectator page should serialize")
    );
}

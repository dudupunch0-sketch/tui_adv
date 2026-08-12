use escape_core::{
    index_content_bundle, load_content_bundle, new_game_from_content_at, scene_page_from_content,
};

const WUXIA_BUNDLE: &str =
    include_str!("../fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json");
const SPECTATOR_LOCATION_ID: &str = "cheongryu_outer_courtyard";
const SPECTATOR_GATE_FLAG: &str = "combat_spectator_preview_unlocked";
const EXPECTED_FIXTURE: &[u8] =
    include_bytes!("../fixtures/combat/wuxia_combat_spectator_preview_bout.seed-2.combat.json");

#[test]
fn producer_output_matches_checked_in_combat_fixture_byte_for_byte() {
    let bundle = load_content_bundle(WUXIA_BUNDLE).expect("wuxia preview bundle should load");
    let index = index_content_bundle(&bundle).expect("wuxia preview bundle should index");
    let mut state = new_game_from_content_at(2, &index, SPECTATOR_LOCATION_ID)
        .expect("game should start at the Qingliu outer courtyard");
    state.flags.push(SPECTATOR_GATE_FLAG.to_string());

    let page = scene_page_from_content(&state, &index).expect("scene page should render");
    let combat = page
        .combat
        .expect("the gated systemic combat encounter should fill ScenePage.combat");

    let mut producer_bytes = serde_json::to_string_pretty(&combat)
        .expect("spectator page should serialize")
        .into_bytes();
    producer_bytes.push(b'\n');

    assert_eq!(producer_bytes, EXPECTED_FIXTURE);
}

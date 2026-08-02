use escape_core::{
    index_content_bundle, load_content_bundle, new_game_from_content, scene_page_from_content,
    CombatConclusionOutcome, CombatConclusionReason, CombatConclusionReport,
    CombatSimulationVersion, CombatSpectatorPage, CombatSpectatorView,
};
use serde_json::Value;

const CONTENT_BUNDLE: &str = include_str!("../fixtures/content/content.bundle.json");

/// Assembles a `CombatSpectatorView` directly (no simulation run, no RNG) — this
/// slice only wires existing structs together, it never recomputes resolution.
fn sample_view() -> CombatSpectatorView {
    CombatSpectatorView {
        simulation_version: CombatSimulationVersion::new("v1").unwrap(),
        resolution_fingerprint: "resolution-fp".into(),
        tick_millis: 100,
        frames: vec![],
        core_log: vec![],
        full_log: vec![],
        fingerprint: "view-fp".into(),
    }
}

fn sample_report() -> CombatConclusionReport {
    CombatConclusionReport {
        resolution_fingerprint: "resolution-fp".into(),
        outcome: CombatConclusionOutcome::AllyVictory,
        reason: CombatConclusionReason::AllEnemiesDefeated,
        decisive_tick: Some(3),
        active_allies: 1,
        active_enemies: 0,
        survivor_ids: vec!["a".into()],
        defeated_ids: vec!["e".into()],
        removed_combat_effect_ids: vec![],
        retained_effect_ids: vec![],
        duration_millis: 300,
        combatants: vec![],
        top_damage_dealt_id: None,
        top_damage_taken_id: None,
        fingerprint: "report-fp".into(),
    }
}

fn sample_page_with_report() -> CombatSpectatorPage {
    CombatSpectatorPage {
        view: sample_view(),
        report: Some(sample_report()),
    }
}

fn content_backed_page() -> escape_core::ScenePage {
    let bundle = load_content_bundle(CONTENT_BUNDLE).expect("content bundle should load");
    let index = index_content_bundle(&bundle).expect("content bundle should index");
    let state = new_game_from_content(123, &index).expect("content-backed game should start");
    scene_page_from_content(&state, &index).expect("scene page should render")
}

/// 1 & 2: `scene_page_from_content`'s `combat` is always `None` in this slice (no
/// authoring opens combat yet — Wave 3 Step 2), and a `None` combat must not add a
/// `"combat"` key to the serialized JSON, so existing ScenePage JSON stays
/// byte-identical.
#[test]
fn content_backed_scene_page_has_no_combat_producer_and_no_combat_key_in_json() {
    let page = content_backed_page();
    assert!(page.combat.is_none());

    let value = serde_json::to_value(&page).expect("ScenePage should serialize");
    assert!(
        value.as_object().unwrap().get("combat").is_none(),
        "combat key must not appear when ScenePage.combat is None: {value}"
    );
}

/// 3: once `combat` is filled in (as a future producer will do in Wave 3 Step 2),
/// the `"combat"` key appears and carries `view.simulation_version` alongside
/// `view.fingerprint`, satisfying the fingerprint/version pairing contract.
#[test]
fn filled_combat_serializes_with_simulation_version_alongside_fingerprint() {
    let mut page = content_backed_page();
    page.combat = Some(sample_page_with_report());

    let value = serde_json::to_value(&page).expect("ScenePage should serialize");
    let combat = value
        .get("combat")
        .expect("combat key must appear once ScenePage.combat is Some");
    assert_eq!(
        combat["view"]["simulation_version"],
        Value::String("v1".into())
    );
    assert_eq!(
        combat["view"]["fingerprint"],
        Value::String("view-fp".into())
    );
}

/// 4: a `Some(...)` ScenePage round-trips through serde without loss.
#[test]
fn filled_combat_scene_page_round_trips_through_serde() {
    let mut page = content_backed_page();
    page.combat = Some(sample_page_with_report());

    let json = serde_json::to_string(&page).expect("ScenePage should serialize to string");
    let restored: escape_core::ScenePage =
        serde_json::from_str(&json).expect("ScenePage should deserialize");
    assert_eq!(restored, page);
}

/// 5: pre-existing ScenePage JSON (no "combat" key at all) still deserializes
/// cleanly into `combat: None` — this is the additive-optional guarantee old save
/// data / old serialized pages depend on.
#[test]
fn scene_page_json_missing_combat_key_deserializes_to_none_without_error() {
    let page = content_backed_page();
    let mut value = serde_json::to_value(&page).expect("ScenePage should serialize");
    assert!(value.as_object().unwrap().get("combat").is_none());

    // Simulate genuinely old JSON that predates this field by round-tripping
    // through a value that has no "combat" key at all (belt-and-suspenders: the
    // producer already omits it, this asserts deserialization tolerates that).
    value.as_object_mut().unwrap().remove("combat");
    let restored: escape_core::ScenePage =
        serde_json::from_value(value).expect("ScenePage without combat key should deserialize");
    assert!(restored.combat.is_none());
}

/// 6: `CombatSpectatorPage.report` being `None` must not add a `"report"` key
/// (the "conclusion report only appears once combat has actually concluded" rule).
#[test]
fn combat_spectator_page_with_no_report_omits_report_key() {
    let page = CombatSpectatorPage {
        view: sample_view(),
        report: None,
    };
    let value = serde_json::to_value(&page).expect("CombatSpectatorPage should serialize");
    assert!(
        value.as_object().unwrap().get("report").is_none(),
        "report key must not appear when CombatSpectatorPage.report is None: {value}"
    );
}

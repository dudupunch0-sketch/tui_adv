use escape_core::{
    apply_action_from_content, index_content_bundle, load_content_bundle, new_game_from_content_at,
    scene_page_from_content, turn_view_from_content,
};
use serde_json::json;

const WUXIA_PREVIEW_BUNDLE: &str =
    include_str!("../fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json");
const OFFICE_BUNDLE: &str = include_str!("../fixtures/content/content.bundle.json");

const SUCCESS_TEXT: &str = "민첩하게 큰길로 물러섰다";
const FAILURE_TEXT: &str = "비틀거리다 몽둥이에 쓸리며";

fn staged_entry_state(
    index: &escape_core::ContentIndex,
    encounter_id: &str,
    location_id: &str,
    flags: &[&str],
    seed: u64,
) -> escape_core::GameState {
    let mut state = new_game_from_content_at(seed, index, location_id).unwrap();
    state.active_event_id = Some(encounter_id.to_string());
    state.flags = flags.iter().map(|flag| (*flag).to_string()).collect();
    state
}

/// seed 1..=256을 훑어서 `wuxia_heuksa_bang_first_fight`의
/// `run_toward_open_street` 선택 이후 성공/실패 결과 state를 각각 하나씩 찾는다.
fn heuksa_result_states(
    index: &escape_core::ContentIndex,
) -> (escape_core::GameState, escape_core::GameState) {
    let mut success_state = None;
    let mut failure_state = None;

    for seed in 1..=256 {
        let state = staged_entry_state(
            index,
            "wuxia_heuksa_bang_first_fight",
            "jianghu_market_street",
            &["wuxia_arrival_hidden"],
            seed,
        );
        let choice_state = apply_action_from_content(&state, index, "event:continue")
            .unwrap()
            .state;
        let result_state =
            apply_action_from_content(&choice_state, index, "choice:run_toward_open_street")
                .unwrap()
                .state;
        if result_state.last_check.as_ref().unwrap().success {
            if success_state.is_none() {
                success_state = Some(result_state);
            }
        } else if failure_state.is_none() {
            failure_state = Some(result_state);
        }
        if success_state.is_some() && failure_state.is_some() {
            break;
        }
    }

    (
        success_state.expect("seed sweep should find a successful check"),
        failure_state.expect("seed sweep should find a failed check"),
    )
}

#[test]
fn result_stage_turn_view_body_keeps_only_matching_branch() {
    let bundle = load_content_bundle(WUXIA_PREVIEW_BUNDLE).unwrap();
    let index = index_content_bundle(&bundle).unwrap();
    let (success_state, failure_state) = heuksa_result_states(&index);

    let success_body = turn_view_from_content(&success_state, &index).unwrap().body;
    assert!(success_body.contains(SUCCESS_TEXT));
    assert!(!success_body.contains(FAILURE_TEXT));

    let failure_body = turn_view_from_content(&failure_state, &index).unwrap().body;
    assert!(failure_body.contains(FAILURE_TEXT));
    assert!(!failure_body.contains(SUCCESS_TEXT));
}

#[test]
fn result_stage_scene_page_body_blocks_and_dialogue_keep_only_matching_branch() {
    let bundle = load_content_bundle(WUXIA_PREVIEW_BUNDLE).unwrap();
    let index = index_content_bundle(&bundle).unwrap();
    let (success_state, failure_state) = heuksa_result_states(&index);

    let success_page = scene_page_from_content(&success_state, &index).unwrap();
    let success_body_texts: Vec<_> = success_page
        .body_blocks
        .iter()
        .map(|block| block.text.as_str())
        .collect();
    let success_dialogue_texts: Vec<_> = success_page
        .dialogue_entries
        .iter()
        .map(|entry| entry.text.as_str())
        .collect();
    assert!(success_body_texts.iter().any(|t| t.contains(SUCCESS_TEXT)));
    assert!(!success_body_texts.iter().any(|t| t.contains(FAILURE_TEXT)));
    assert!(success_dialogue_texts
        .iter()
        .any(|t| t.contains(SUCCESS_TEXT)));
    assert!(!success_dialogue_texts
        .iter()
        .any(|t| t.contains(FAILURE_TEXT)));

    let failure_page = scene_page_from_content(&failure_state, &index).unwrap();
    let failure_body_texts: Vec<_> = failure_page
        .body_blocks
        .iter()
        .map(|block| block.text.as_str())
        .collect();
    let failure_dialogue_texts: Vec<_> = failure_page
        .dialogue_entries
        .iter()
        .map(|entry| entry.text.as_str())
        .collect();
    assert!(failure_body_texts.iter().any(|t| t.contains(FAILURE_TEXT)));
    assert!(!failure_body_texts.iter().any(|t| t.contains(SUCCESS_TEXT)));
    assert!(failure_dialogue_texts
        .iter()
        .any(|t| t.contains(FAILURE_TEXT)));
    assert!(!failure_dialogue_texts
        .iter()
        .any(|t| t.contains(SUCCESS_TEXT)));
}

/// office fixture(`content.bundle.json`)의 `printer_prints_alone` encounter에
/// `serde_json`으로 `event`를 주입한다. fixture 파일 자체는 수정하지 않는다.
fn office_event_bundle() -> escape_core::ContentBundle {
    let mut bundle = load_content_bundle(OFFICE_BUNDLE).unwrap();
    let encounter = bundle
        .content
        .encounters
        .iter_mut()
        .find(|value| value["id"] == "printer_prints_alone")
        .unwrap();
    encounter["event"] = json!({"stages": [
        {"id":"opening","kind":"story","blocks":[
            {"kind":"narration","text":"첫 문단"},
            {"kind":"illustration","visual_id":"printer-event.png","alt":"출력 중인 복합기","placeholder":true}
        ]},
        {"id":"decision","kind":"choice","blocks":[{"kind":"system","text":"무엇을 할까?"}],
         "choices":[{"id":"read_printout","next_stage_id":"result"}]},
        {"id":"result","kind":"result","blocks":[
            {"kind":"result_summary","text":"공통 결과 문장"},
            {"kind":"narration","text":"성공 분기 결과","branch":"success"},
            {"kind":"narration","text":"실패 분기 결과","branch":"failure"}
        ]}
    ]});
    bundle
}

#[test]
fn result_stage_without_check_resolution_keeps_only_common_blocks() {
    let bundle = office_event_bundle();
    let index = index_content_bundle(&bundle).unwrap();
    let mut state = new_game_from_content_at(3, &index, "printer_area").unwrap();
    state.active_event_id = Some("printer_prints_alone".to_string());
    state.event_stage_index = 2; // "result" stage, reached without a check resolution
    state.last_check = None;

    let body = turn_view_from_content(&state, &index).unwrap().body;
    assert!(body.contains("공통 결과 문장"));
    assert!(!body.contains("성공 분기 결과"));
    assert!(!body.contains("실패 분기 결과"));

    let page = scene_page_from_content(&state, &index).unwrap();
    let body_texts: Vec<_> = page
        .body_blocks
        .iter()
        .map(|block| block.text.as_str())
        .collect();
    assert!(body_texts.iter().any(|t| t.contains("공통 결과 문장")));
    assert!(!body_texts.iter().any(|t| t.contains("성공 분기 결과")));
    assert!(!body_texts.iter().any(|t| t.contains("실패 분기 결과")));
}

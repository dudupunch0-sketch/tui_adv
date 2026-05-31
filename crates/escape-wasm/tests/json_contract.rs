use escape_wasm::{
    apply_action_json, load_state_json, new_game_json, save_state_json, scene_page_json,
};
use serde_json::Value;

const CONTENT_BUNDLE: &str = include_str!("../../escape-core/fixtures/content/content.bundle.json");

const WUXIA_PREVIEW_BUNDLE: &str = r#"
{
  "schema_version": 1,
  "kind": "tui_adv.content_bundle",
  "source": "src/tui_adv/storypack-previews/wuxia_jianghu_pack/*.yaml",
  "runtime": {
    "runtime_mode": "storypack_preview",
    "world_id": "wuxia_jianghu",
    "storypack_id": "wuxia_jianghu_pack",
    "default_location": "wuxia_commute_rift"
  },
  "manifest": {
    "schema_version": 1,
    "source": "src/tui_adv/storypack-previews/wuxia_jianghu_pack/*.yaml",
    "counts": {
      "locations": 2,
      "items": 1,
      "encounters": 1,
      "endings": 1,
      "achievements": 1,
      "secrets": 0
    }
  },
  "content": {
    "locations": [
      {
        "id": "wuxia_commute_rift",
        "name": "출근길 균열",
        "description": "출근길 손잡이가 종소리처럼 흔들리며 강호의 흙먼지로 이어진다.",
        "connections": ["jianghu_roadside"],
        "tags": ["wuxia", "start", "rift"],
        "danger": 0
      },
      {
        "id": "jianghu_roadside",
        "name": "강호 초입",
        "description": "낯선 흙길과 대나무 숲 사이로 청류문 표식이 희미하게 보인다.",
        "connections": ["wuxia_commute_rift"],
        "tags": ["wuxia", "transit"],
        "danger": 1
      }
    ],
    "items": [
      {
        "id": "commuter_badge",
        "name": "사원증",
        "description": "현대 회사원의 이름과 사진이 남은 얇은 플라스틱 카드.",
        "type": "clue",
        "tags": ["identity", "wuxia_preview"]
      }
    ],
    "encounters": [
      {
        "id": "wuxia_commute_rift_arrival",
        "title": "출근길 균열",
        "body": "눈을 뜨자 출근복 차림 그대로 낯선 관도 한복판에 서 있다. 손에는 아직 사원증이 걸려 있고, 멀리서 청류문 수련생들의 발소리가 다가온다.",
        "presentation": {
          "visual_id": "wuxia_commute_rift",
          "speaker": "천기록",
          "layout": "storypack_preview",
          "effect_cues": [
            {
              "kind": "glyph_anomaly",
              "source": "commute_rift",
              "intensity": 0.58,
              "stable_terms": ["사원증", "출근복", "천기록"],
              "distortion": "hanja_then_badge_scan",
              "duration_hint_ms": 1400,
              "fallback_text": "한자 획 사이로 '사원증', '출근복', '천기록'만 현대식 글꼴처럼 남는다."
            }
          ]
        },
        "conditions": { "locations": ["wuxia_commute_rift"] },
        "choices": [
          {
            "id": "grip_employee_badge",
            "label": "사원증을 쥐고 현재의 나를 붙든다",
            "outcome": {
              "add_items": ["commuter_badge"],
              "add_clues": ["employee_badge_in_jianghu"],
              "add_flags": ["wuxia_arrival_grounded"],
              "log": "사원증의 플라스틱 모서리가 손바닥에 남아, 이 몸이 아직 현대의 나임을 붙든다."
            }
          },
          {
            "id": "follow_roadside_dust",
            "label": "흙먼지가 흐르는 쪽으로 몸을 숨긴다",
            "outcome": {
              "destination_id": "jianghu_roadside",
              "add_flags": ["wuxia_arrival_hidden"],
              "log": "출근복 자락을 여미고 흙먼지의 흐름을 따라 관도 가장자리로 물러섰다."
            }
          }
        ]
      }
    ],
    "endings": [
      {
        "id": "wuxia_preview_grounded",
        "name": "강호의 사원증",
        "kind": "preview",
        "priority": 10,
        "conditions": { "required_flags": ["wuxia_arrival_grounded"] },
        "text": "사원증은 부적처럼 남았다. 아직 돌아갈 길은 없지만, 천기록은 당신을 이방인으로 기억하기 시작한다."
      }
    ],
    "achievements": [
      {
        "id": "wuxia_first_arrival",
        "name": "강호 출근",
        "description": "출근복 그대로 강호에 떨어진 첫 순간을 붙든다.",
        "conditions": { "required_flags": ["wuxia_arrival_grounded"] }
      }
    ],
    "secrets": []
  }
}
"#;

#[test]
fn json_boundary_creates_scene_page_applies_action_and_roundtrips_save() {
    let state_json = new_game_json(123, CONTENT_BUNDLE).expect("new game should serialize");
    let state: Value = serde_json::from_str(&state_json).expect("state JSON should parse");
    assert_eq!(state["seed"], 123);
    assert_eq!(state["location_id"], "dev_desk");
    assert_eq!(state["turn"], 0);
    assert_eq!(state["player"]["hunger"], 0);
    assert_eq!(state["player"]["thirst"], 0);

    let page_json =
        scene_page_json(&state_json, CONTENT_BUNDLE).expect("scene page should serialize");
    let page: Value = serde_json::from_str(&page_json).expect("page JSON should parse");
    assert_eq!(page["mode"], "encounter");
    assert_eq!(page["title"], "퇴사자의 메신저");
    assert_eq!(page["actions"][0]["id"], "choice:check_message");
    assert_eq!(page["status_summary"]["resources"][3]["id"], "hunger");
    assert_eq!(page["status_summary"]["resources"][4]["id"], "thirst");

    let result_json = apply_action_json(&state_json, CONTENT_BUNDLE, "choice:check_message")
        .expect("action result should serialize");
    let result: Value =
        serde_json::from_str(&result_json).expect("action result JSON should parse");
    assert_eq!(result["encounter_id"], "ex_employee_messenger");
    assert_eq!(result["action_id"], "choice:check_message");
    assert_eq!(result["state"]["turn"], 1);
    assert_eq!(result["state"]["player"]["battery"], 97);
    assert_eq!(result["state"]["player"]["hunger"], 1);
    assert_eq!(result["state"]["player"]["thirst"], 2);
    assert_eq!(result["logs"][0], "퇴사자의 메시지를 확인했다.");
    assert_eq!(
        result["newly_unlocked_achievements"][0],
        "first_signal_received"
    );

    let next_state_json = serde_json::to_string(&result["state"]).expect("state should stringify");
    let next_page_json = scene_page_json(&next_state_json, CONTENT_BUNDLE)
        .expect("next scene page should serialize");
    let next_page: Value =
        serde_json::from_str(&next_page_json).expect("next page JSON should parse");
    assert_eq!(next_page["mode"], "movement");
    assert_eq!(
        next_page["history_entries"][0]["text"],
        "퇴사자의 메시지를 확인했다."
    );
    assert_eq!(
        next_page["achievement_summary"]["unlocked"][0],
        "first_signal_received"
    );

    let save_json = save_state_json(&next_state_json).expect("save envelope should serialize");
    let save: Value = serde_json::from_str(&save_json).expect("save envelope JSON should parse");
    assert_eq!(save["schema_version"], 1);
    assert_eq!(save["state"]["turn"], 1);

    let restored_json = load_state_json(&save_json).expect("save envelope should load");
    let restored: Value =
        serde_json::from_str(&restored_json).expect("restored state should parse");
    assert_eq!(restored, result["state"]);
}

#[test]
fn json_boundary_uses_storypack_preview_default_location() {
    let state_json =
        new_game_json(123, WUXIA_PREVIEW_BUNDLE).expect("preview new game should serialize");
    let state: Value = serde_json::from_str(&state_json).expect("state JSON should parse");
    assert_eq!(state["location_id"], "wuxia_commute_rift");

    let page_json = scene_page_json(&state_json, WUXIA_PREVIEW_BUNDLE)
        .expect("preview scene page should serialize");
    let page: Value = serde_json::from_str(&page_json).expect("page JSON should parse");
    assert_eq!(page["mode"], "encounter");
    assert_eq!(page["title"], "출근길 균열");
    assert_eq!(page["location"]["id"], "wuxia_commute_rift");
    assert_eq!(page["visual"]["kind"], "storypack_preview");
    assert_eq!(page["actions"][0]["id"], "choice:grip_employee_badge");

    let result_json = apply_action_json(
        &state_json,
        WUXIA_PREVIEW_BUNDLE,
        "choice:grip_employee_badge",
    )
    .expect("preview action result should serialize");
    let result: Value =
        serde_json::from_str(&result_json).expect("action result JSON should parse");
    assert_eq!(result["encounter_id"], "wuxia_commute_rift_arrival");
    assert_eq!(result["state"]["flags"][0], "wuxia_arrival_grounded");
    assert_eq!(
        result["newly_unlocked_achievements"][0],
        "wuxia_first_arrival"
    );
}

#[test]
fn json_boundary_reports_user_facing_errors() {
    let state_json = new_game_json(123, CONTENT_BUNDLE).expect("new game should serialize");

    let malformed_state_error = scene_page_json("{not json", CONTENT_BUNDLE)
        .expect_err("malformed state should be a normal error");
    assert!(malformed_state_error.contains("invalid state JSON"));

    let unknown_action_error = apply_action_json(&state_json, CONTENT_BUNDLE, "choice:missing")
        .expect_err("unknown action should be a normal error");
    assert!(unknown_action_error.contains("unknown action id: choice:missing"));

    let malformed_save_error =
        load_state_json("[]").expect_err("malformed save should be rejected");
    assert!(malformed_save_error.contains("invalid save JSON"));
}

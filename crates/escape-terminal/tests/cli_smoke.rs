use escape_wasm::{apply_action_json, new_game_json, scene_page_json};
use serde_json::Value;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

#[test]
fn executable_smoke_prints_printer_scene_from_core() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args(["--scene", "printer", "--seed", "123", "--smoke"])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("escape-terminal"));
    assert!(stdout.contains("Rust GameCore"));
    assert!(stdout.contains("printer_prints_alone"));
    assert!(stdout.contains("EffectCue::GlyphAnomaly"));
    assert!(stdout.contains("비상계단"));
    assert!(stdout.contains("1. 출력물이 안정될 때까지 기다린다"));
}

#[test]
fn content_smoke_prints_seeded_content_turn_from_bundle() {
    let bundle_path = content_bundle_path();
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--content-bundle",
            bundle_path.to_str().expect("bundle path should be UTF-8"),
            "--seed",
            "123",
            "--smoke",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("scene: content"));
    assert!(stdout.contains("== Turn 0 =="));
    assert!(stdout.contains("location: dev_desk"));
    assert!(stdout.contains("encounter: ex_employee_messenger"));
    assert!(stdout.contains("[퇴사자의 메신저]"));
    assert!(stdout.contains("1. choice:check_message / 메시지를 확인한다 / 배터리 -3, 정신력 -2"));
}

#[test]
fn content_scripted_actions_walk_from_encounter_to_movement_turn() {
    let bundle_path = content_bundle_path();
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--content-bundle",
            bundle_path.to_str().expect("bundle path should be UTF-8"),
            "--seed",
            "123",
            "--smoke",
            "--action",
            "choice:check_message",
            "--action",
            "move:dev_office",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("== Turn 0 =="));
    assert!(stdout.contains("encounter: ex_employee_messenger"));
    assert!(stdout.contains("executed: choice:check_message / 메시지를 확인한다"));
    assert!(stdout.contains("- 퇴사자의 메시지를 확인했다."));
    assert!(stdout.contains("== Turn 1 =="));
    assert!(stdout.contains("encounter: none"));
    assert!(stdout.contains("1. move:dev_office / 개발팀 사무실"));
    assert!(stdout.contains("executed: move:dev_office / 개발팀 사무실"));
    assert!(stdout.contains("- 개발팀 사무실로 이동했다."));
    assert!(stdout.contains("== Turn 2 =="));
    assert!(stdout.contains("location: dev_office"));
}

#[test]
fn content_script_rejects_action_that_is_not_available_in_current_turn() {
    let bundle_path = content_bundle_path();
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--content-bundle",
            bundle_path.to_str().expect("bundle path should be UTF-8"),
            "--seed",
            "123",
            "--smoke",
            "--action",
            "move:dev_office",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        !output.status.success(),
        "expected unavailable action to fail"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("action 'move:dev_office' is not available in current turn"));
}

#[test]
fn content_tui_smoke_renders_start_encounter_panel() {
    let bundle_path = content_bundle_path();
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--content-bundle",
            bundle_path.to_str().expect("bundle path should be UTF-8"),
            "--seed",
            "123",
            "--tui-smoke",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[SuperLightTUI Snapshot]"));
    assert!(stdout.contains("ESCAPE OFFICE // SuperLightTUI HORROR EDITION"));
    assert!(stdout.contains("[상태]"));
    assert!(stdout.contains("[비주얼]"));
    assert!(stdout.contains("glyphfx signal:"));
    assert!(stdout.contains("위치: 내 자리 (dev_desk)"));
    assert!(stdout.contains("[현재 인카운터]"));
    assert!(stdout.contains("퇴사자의 메신저"));
    assert!(stdout.contains("1. choice:check_message / 메시지를 확인한다 / 배터리 -3, 정신력 -2"));
    assert!(stdout.contains("[잠긴 선택지]"));
    assert!(stdout.contains("choice:trace_packet_delay / [인터페이스] 알림 지연 시간을 역추적한다"));
    assert!(stdout.contains("능력 조건 미충족: interface >= 4"));
    assert!(stdout.contains("[현재 행동]"));
    assert!(!stdout.contains("== Turn 0 =="));
}

#[test]
fn content_tui_smoke_renders_final_movement_panel_after_scripted_actions() {
    let bundle_path = content_bundle_path();
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--content-bundle",
            bundle_path.to_str().expect("bundle path should be UTF-8"),
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:check_message",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[SuperLightTUI Snapshot]"));
    assert!(stdout.contains("ESCAPE OFFICE // SuperLightTUI HORROR EDITION"));
    assert!(stdout.contains("턴: 1"));
    assert!(stdout.contains("위치: 내 자리 (dev_desk)"));
    assert!(stdout.contains("체력: 100  정신력: 98  배터리: 97"));
    assert!(stdout.contains("[현재 행동]"));
    assert!(stdout.contains("1. move:dev_office / 개발팀 사무실"));
    assert!(stdout.contains("[최근 로그]"));
    assert!(stdout.contains("- 퇴사자의 메시지를 확인했다."));
    assert!(!stdout.contains("[현재 인카운터]"));
    assert!(!stdout.contains("== Turn 1 =="));
}

#[test]
fn content_tui_smoke_renders_printer_visual_card_with_stable_glyphfx_terms() {
    let bundle_path = content_bundle_path();
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--content-bundle",
            bundle_path.to_str().expect("bundle path should be UTF-8"),
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:check_message",
            "--action",
            "move:dev_office",
            "--action",
            "move:printer_area",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("╭─ VISUAL CARD ─"));
    assert!(stdout.contains("visual id: printer_anomaly"));
    assert!(stdout.contains("layout: anomaly_object"));
    assert!(stdout.contains("alt: 복합기가 혼자 출력한다"));
    assert!(stdout.contains("glyphfx signal: glyph_anomaly [#######---] 72% reflow_then_stabilize"));
    assert!(stdout.contains("stable terms: 비상계단 / 토너 / 접힌 방향"));
    assert!(stdout.contains("fallback: 출력물의 깨진 글자 사이로 '비상계단'이 선명하게 남는다."));
}

#[test]
fn content_play_mode_prints_turn_input_hint_and_invalid_range() {
    let bundle_path = content_bundle_path();
    let output = run_escape_terminal_with_input(
        &[
            "--scene",
            "content",
            "--content-bundle",
            bundle_path.to_str().expect("bundle path should be UTF-8"),
            "--seed",
            "123",
            "--play",
        ],
        "99\nq\n",
    );

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("입력 안내: 1-3 또는 action id, q/quit 종료"));
    assert!(stdout.contains("잘못된 입력: 99 (사용 가능한 번호: 1-3 또는 action id)"));
}

#[test]
fn content_tui_smoke_action_ids_match_wasm_scene_page_after_scripted_action() {
    let bundle_path = content_bundle_path();
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--content-bundle",
            bundle_path.to_str().expect("bundle path should be UTF-8"),
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:check_message",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let terminal_action_ids =
        action_ids_from_terminal_snapshot(&String::from_utf8_lossy(&output.stdout));

    let bundle_json = std::fs::read_to_string(&bundle_path).expect("bundle should be readable");
    let state_json = new_game_json(123, &bundle_json).expect("wasm new_game should serialize");
    let result_json = apply_action_json(&state_json, &bundle_json, "choice:check_message")
        .expect("wasm action result should serialize");
    let result: Value = serde_json::from_str(&result_json).expect("action result should parse");
    let next_state_json = serde_json::to_string(&result["state"]).expect("state should serialize");
    let scene_json =
        scene_page_json(&next_state_json, &bundle_json).expect("wasm scene page should serialize");
    let scene: Value = serde_json::from_str(&scene_json).expect("scene page should parse");
    let wasm_action_ids: Vec<String> = scene["actions"]
        .as_array()
        .expect("actions should be an array")
        .iter()
        .map(|action| {
            action["id"]
                .as_str()
                .expect("id should be a string")
                .to_string()
        })
        .collect();

    assert_eq!(terminal_action_ids, wasm_action_ids);
    assert_eq!(terminal_action_ids, vec!["move:dev_office".to_string()]);
}

#[test]
fn content_play_mode_accepts_numbered_input_and_quit() {
    let bundle_path = content_bundle_path();
    let output = run_escape_terminal_with_input(
        &[
            "--scene",
            "content",
            "--content-bundle",
            bundle_path.to_str().expect("bundle path should be UTF-8"),
            "--seed",
            "123",
            "--play",
        ],
        "1\n1\nq\n",
    );

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("escape-terminal / 직접 플레이"));
    assert!(stdout.contains("입력: 번호 또는 action id"));
    assert!(stdout.contains("[현재 인카운터]"));
    assert!(stdout.contains("선택 실행: 메시지를 확인한다"));
    assert!(stdout.contains("이동 실행: 개발팀 사무실"));
    assert!(stdout.contains("위치: 개발팀 사무실 (dev_office)"));
    assert!(stdout.contains("게임을 종료한다"));
    assert!(!String::from_utf8_lossy(&output.stderr).contains("Traceback"));
}

#[test]
fn content_play_mode_accepts_stable_action_id_input() {
    let bundle_path = content_bundle_path();
    let output = run_escape_terminal_with_input(
        &[
            "--scene",
            "content",
            "--content-bundle",
            bundle_path.to_str().expect("bundle path should be UTF-8"),
            "--seed",
            "123",
            "--play",
        ],
        "choice:ignore_phone\nq\n",
    );

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("선택 실행: 무시하고 휴대폰을 엎어둔다"));
    assert!(stdout.contains("휴대폰을 엎어두자 알림음이 한 박자 늦게 멈췄다."));
    assert!(stdout.contains("게임을 종료한다"));
}

#[test]
fn content_play_mode_rejects_invalid_input_without_exiting() {
    let bundle_path = content_bundle_path();
    let output = run_escape_terminal_with_input(
        &[
            "--scene",
            "content",
            "--content-bundle",
            bundle_path.to_str().expect("bundle path should be UTF-8"),
            "--seed",
            "123",
            "--play",
        ],
        "99\nq\n",
    );

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("잘못된 입력: 99"));
    assert!(stdout.contains("게임을 종료한다"));
}

fn content_bundle_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../escape-core/fixtures/content/content.bundle.json")
}

fn action_ids_from_terminal_snapshot(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            let (_, rest) = trimmed.split_once(". ")?;
            let (action_id, _) = rest.split_once(" / ")?;
            if action_id.starts_with("choice:") || action_id.starts_with("move:") {
                Some(action_id.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn run_escape_terminal_with_input(args: &[&str], input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("escape-terminal executable should start");

    child
        .stdin
        .as_mut()
        .expect("stdin should be piped")
        .write_all(input.as_bytes())
        .expect("input should be written");

    child
        .wait_with_output()
        .expect("escape-terminal output should be captured")
}

use std::path::PathBuf;
use std::process::Command;

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

fn content_bundle_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../escape-core/fixtures/content/content.bundle.json")
}

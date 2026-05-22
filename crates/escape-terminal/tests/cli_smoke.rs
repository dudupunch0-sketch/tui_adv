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

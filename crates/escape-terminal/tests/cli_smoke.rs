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
fn content_smoke_defaults_to_wuxia_storypack_when_no_bundle_is_given() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args(["--scene", "content", "--seed", "123", "--smoke"])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("scene: content"));
    assert!(stdout.contains("location: wuxia_commute_rift"));
    assert!(stdout.contains("encounter: wuxia_commute_rift_arrival"));
    assert!(stdout.contains("[출근길 균열]"));
    assert!(!stdout.contains("location: dev_desk"));
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
fn content_tui_smoke_renders_wuxia_storypack_preview_arrival() {
    let bundle_path = wuxia_preview_bundle_path();
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
    assert!(stdout.contains("이구학지 - 천기록 // SuperLightTUI STORYBOOK"));
    assert!(stdout.contains("위치: 출근길 균열 (wuxia_commute_rift)"));
    assert!(stdout.contains("[현재 인카운터]"));
    assert!(stdout.contains("출근길 균열"));
    assert!(stdout.contains("visual id: wuxia_commute_rift"));
    assert!(stdout.contains("layout: storypack_preview"));
    assert!(stdout.contains("stable terms: 사원증 / 출근복 / 천기록"));
    assert!(stdout.contains("choice:grip_employee_badge / 사원증을 쥐고 현재의 나를 붙든다"));
    assert!(stdout.contains("choice:follow_roadside_dust / 흙먼지가 흐르는 쪽으로 몸을 숨긴다"));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_renders_wuxia_storypack_preview_first_fight() {
    let bundle_path = wuxia_preview_bundle_path();
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
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 강호 시장 거리 (jianghu_market_street)"));
    assert!(stdout.contains("[현재 인카운터]"));
    assert!(stdout.contains("흑사방 첫 난투"));
    assert!(stdout.contains("visual id: wuxia_heuksa_bang_first_fight"));
    assert!(stdout.contains("layout: combat_intervention"));
    assert!(stdout.contains("stable terms: 거리 / 구두 / 사원증"));
    assert!(stdout.contains("choice:run_toward_open_street / 큰길 쪽으로 비틀거리며 물러난다"));
    assert!(stdout.contains("choice:deescalate_with_words / 말로 시간을 벌며 사원증을 감춘다"));
    assert!(stdout.contains("choice:swing_commute_bag / 출근 가방을 방패처럼 휘두른다"));
    assert!(stdout.contains(
        "choice:loosen_tie_and_drop_shoes / 넥타이를 풀고 구두를 벗어 움직임을 회복한다"
    ));
    assert!(stdout.contains("choice:crash_in_with_body / 어깨로 들이받고 넘어지듯 버틴다"));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_launches_wuxia_storypack_preview_by_opt_in_flag() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 강호 시장 거리 (jianghu_market_street)"));
    assert!(stdout.contains("흑사방 첫 난투"));
    assert!(stdout.contains("choice:run_toward_open_street / 큰길 쪽으로 비틀거리며 물러난다"));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_cheonggi_record_first_fragment() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 강호 시장 거리 (jianghu_market_street)"));
    assert!(stdout.contains("천기록 첫 편린"));
    assert!(stdout.contains("visual id: wuxia_cheonggi_record_first_fragment"));
    assert!(stdout.contains("layout: cheonggi_record"));
    assert!(stdout.contains("stable terms: 업무수첩 / 천기록 / 실패 기록"));
    assert!(stdout.contains("choice:choose_guard_basics / '호신 자세의 기본' 문장을 붙든다"));
    assert!(stdout.contains("choice:choose_keep_feet_moving / '발을 멈추지 않는 법'을 남긴다"));
    assert!(stdout.contains("choice:choose_failure_log / '실패 기록'을 받아들인다"));
    assert!(stdout.contains("choice:close_notebook_without_choice / 수첩을 덮고 호흡부터 고른다"));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_seo_harin_rescue() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 강호 시장 거리 (jianghu_market_street)"));
    assert!(stdout.contains("서하린의 개입"));
    assert!(stdout.contains("visual id: wuxia_seo_harin_rescue"));
    assert!(stdout.contains("layout: rescue_and_investigation"));
    assert!(stdout.contains("stable terms: 서하린 / 청류문 / 감시"));
    assert!(stdout.contains("choice:tell_plain_truth / 있는 그대로 길을 잃은 외지인이라고 말한다"));
    assert!(stdout
        .contains("choice:ask_for_medical_help_first / 설명보다 치료와 안전한 곳을 먼저 부탁한다"));
    assert!(stdout.contains(
        "choice:explain_company_and_commute / 회사와 출근길을 최대한 논리적으로 설명한다"
    ));
    assert!(stdout.contains(
        "choice:show_cheonggi_record_page / 방금 떠오른 천기록의 글자를 조심스럽게 보여준다"
    ));
    assert!(stdout.contains("choice:hide_employee_badge / 사원증과 수첩을 품 안으로 숨긴다"));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_cheongryu_apprentice_entry() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("청류문 임시 수습생 등록"));
    assert!(stdout.contains("visual id: wuxia_cheongryu_apprentice_entry"));
    assert!(stdout.contains("layout: cheongryu_apprenticeship"));
    assert!(stdout.contains("stable terms: 청류문 / 잡일 / 수습생"));
    assert!(stdout
        .contains("choice:accept_three_month_trial / 석 달 동안 잡일과 수습 조건을 받아들인다"));
    assert!(stdout.contains(
        "choice:request_martial_training_immediately / 지금 당장 무공을 가르쳐 달라고 요구한다"
    ));
    assert!(stdout.contains(
        "choice:organize_chores_like_workflow / 회사식 업무 분해로 잡일 동선을 정리한다"
    ));
    assert!(stdout
        .contains("choice:inspect_archive_during_chore / 서고 정리 중 잠긴 낡은 장부를 살핀다"));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_cheongryu_chore_sparring() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("청류문 장작 마당 첫 겨루기"));
    assert!(stdout.contains("visual id: wuxia_cheongryu_chore_sparring"));
    assert!(stdout.contains("layout: combat_intervention"));
    assert!(stdout.contains("stable terms: 균형 / 호흡 / 장작"));
    assert!(
        stdout.contains("choice:step_back_with_firewood / 장작을 떨어뜨리지 않고 반걸음 물러난다")
    );
    assert!(stdout
        .contains("choice:let_shoulder_turn_with_push / 밀리는 힘을 거스르지 않고 어깨를 돌린다"));
    assert!(
        stdout.contains("choice:plant_bare_foot_in_dust / 흙먼지에 발을 박아 미끄러짐을 멈춘다")
    );
    assert!(
        stdout.contains("choice:ask_harin_what_changed / 방금 왜 덜 밀렸는지 서하린에게 묻는다")
    );
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_cheongryu_raid_route_split() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
            "--action",
            "choice:step_back_with_firewood",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("청류문 습격과 갈라지는 길"));
    assert!(stdout.contains("visual id: wuxia_cheongryu_raid_route_split"));
    assert!(stdout.contains("layout: raid_route_pressure"));
    assert!(stdout.contains("stable terms: 청류문 / 백도맹 / 천기록"));
    assert!(
        stdout.contains("choice:evacuate_the_wounded_first / 부상자를 먼저 빼내고 선택을 미룬다")
    );
    assert!(stdout.contains(
        "choice:defend_cheongryu_with_white_path / 백도맹 지원을 받아 청류문을 방어한다"
    ));
    assert!(stdout.contains("choice:trade_with_black_heaven / 흑천련 도월과 거래해 탈출로를 산다"));
    assert!(stdout
        .contains("choice:follow_heavenly_archive / 천기각 기록관을 따라 천기록의 출처를 쫓는다"));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_cheongryu_raid_wounded_fallback() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
            "--action",
            "choice:step_back_with_firewood",
            "--action",
            "choice:evacuate_the_wounded_first",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("부상자 피난처와 미뤄진 선택"));
    assert!(stdout.contains("visual id: wuxia_cheongryu_raid_wounded_fallback"));
    assert!(stdout.contains("layout: wounded_fallback_route_pressure"));
    assert!(stdout.contains("stable terms: 부상자 / 백도맹 / 천기각"));
    assert!(stdout.contains(
        "choice:stabilize_wounded_until_dawn / 새벽까지 부상자를 안정시키고 명단을 맞춘다"
    ));
    assert!(stdout.contains(
        "choice:ask_baekdo_for_medicine_not_command / 백도맹에 명령이 아니라 약과 호위를 요청한다"
    ));
    assert!(stdout.contains(
        "choice:trade_black_heaven_bandages_for_exit / 흑천련의 붕대와 탈출로를 거래한다"
    ));
    assert!(stdout
        .contains("choice:follow_archive_triage_map / 천기각 기록관의 부상자 동선 지도를 따른다"));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_baekdo_medicine_debt() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
            "--action",
            "choice:step_back_with_firewood",
            "--action",
            "choice:defend_cheongryu_with_white_path",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("백도맹 약상자와 청류문의 채무"));
    assert!(stdout.contains("visual id: wuxia_baekdo_medicine_debt"));
    assert!(stdout.contains("layout: righteous_route_opener"));
    assert!(stdout.contains("stable terms: 약상자 / 백도맹 / 채무"));
    assert!(stdout.contains(
        "choice:accept_medicine_with_written_debt / 채무 문서를 남기고 약상자와 호위를 받는다"
    ));
    assert!(stdout.contains(
        "choice:ask_terms_before_opening_gate / 산문을 열기 전에 백도맹의 조건을 묻는다"
    ));
    assert!(stdout.contains(
        "choice:send_supplies_to_wounded_first / 약과 식량을 장문 명부보다 부상자에게 먼저 돌린다"
    ));
    assert!(stdout.contains(
        "choice:compare_banner_to_record_margin / 백도맹 깃발과 천기록 여백의 문장을 비교한다"
    ));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_black_heaven_escape_price() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
            "--action",
            "choice:step_back_with_firewood",
            "--action",
            "choice:trade_with_black_heaven",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("흑천련 탈출로의 값"));
    assert!(stdout.contains("visual id: wuxia_black_heaven_escape_price"));
    assert!(stdout.contains("layout: sapa_route_opener"));
    assert!(stdout.contains("stable terms: 탈출로 / 흑천련 / 값"));
    assert!(stdout.contains(
        "choice:accept_dowol_marker_for_safehouse / 도월의 표식을 받고 임시 은신처와 탈출로를 얻는다"
    ));
    assert!(stdout.contains(
        "choice:ask_who_collects_the_price / 누가 언제 어떤 방식으로 값을 받는지 먼저 묻는다"
    ));
    assert!(stdout.contains(
        "choice:keep_cheongryu_names_off_ledger / 청류문 사람들의 이름은 흑천련 장부에 남기지 않는다"
    ));
    assert!(stdout.contains(
        "choice:map_exit_before_following_dowol / 따라가기 전에 탈출로와 추적선을 먼저 기록한다"
    ));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_heavenly_archive_previous_outsiders() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
            "--action",
            "choice:step_back_with_firewood",
            "--action",
            "choice:follow_heavenly_archive",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("천기각 이전 이방인 기록"));
    assert!(stdout.contains("visual id: wuxia_heavenly_archive_previous_outsiders"));
    assert!(stdout.contains("layout: cheonggi_return_opener"));
    assert!(stdout.contains("stable terms: 천기각 / 이방인 / 균열"));
    assert!(stdout.contains(
        "choice:read_previous_outsider_margins / 이전 이방인의 여백 기록을 조용히 읽는다"
    ));
    assert!(stdout.contains(
        "choice:ask_yeon_soha_what_not_to_read / 연소하에게 무엇을 읽으면 안 되는지 먼저 묻는다"
    ));
    assert!(stdout.contains(
        "choice:mark_current_worldline_without_answer / 정답을 요구하지 않고 현재 세계선의 흔적만 표시한다"
    ));
    assert!(stdout.contains(
        "choice:compare_rift_terms_to_commute_memory / 서고의 균열 용어를 출근길 기억과 비교한다"
    ));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_wounded_shelter_dawn_offers() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
            "--action",
            "choice:step_back_with_firewood",
            "--action",
            "choice:evacuate_the_wounded_first",
            "--action",
            "choice:stabilize_wounded_until_dawn",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("부상자 피난처의 새벽 제안"));
    assert!(stdout.contains("visual id: wuxia_wounded_shelter_dawn_offers"));
    assert!(stdout.contains("layout: deferred_route_offer"));
    assert!(stdout.contains("stable terms: 새벽 / 부상자 / 제안"));
    assert!(stdout.contains("choice:keep_wounded_shelter_until_noon / 정오까지 피난처를 더 지킨다"));
    assert!(stdout.contains(
        "choice:accept_baekdo_medicine_after_roll_call / 생존자 점호 뒤 백도맹 약상자를 받는다"
    ));
    assert!(stdout
        .contains("choice:send_word_to_dowol_for_quiet_exit / 도월에게 조용한 퇴로를 부탁한다"));
    assert!(stdout.contains(
        "choice:show_archive_map_to_yeon_soha / 연소하에게 피난처 지도의 접힌 부분을 보인다"
    ));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_mumyeong_first_sighting() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
            "--action",
            "choice:step_back_with_firewood",
            "--action",
            "choice:defend_cheongryu_with_white_path",
            "--action",
            "choice:accept_medicine_with_written_debt",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("무명 첫 목격"));
    assert!(stdout.contains("visual id: wuxia_mumyeong_first_sighting"));
    assert!(stdout.contains("layout: midgame_rival_sighting"));
    assert!(stdout.contains("stable terms: 무명 / 청류문 / 흑사방"));
    assert!(stdout.contains(
        "choice:watch_the_stolen_qingliu_flow / 훔쳐 쓴 청류문식 흐름을 끝까지 관찰한다"
    ));
    assert!(stdout.contains("choice:check_seo_harin_silence / 서하린이 이름을 삼키는 순간을 본다"));
    assert!(
        stdout.contains("choice:follow_black_serpent_runner / 흑사방 심부름꾼의 뒤를 짧게 쫓는다")
    );
    assert!(
        stdout.contains("choice:pretend_not_to_see_the_form / 못 본 척하고 외원 순찰을 계속한다")
    );
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_mumyeong_first_confrontation() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
            "--action",
            "choice:step_back_with_firewood",
            "--action",
            "choice:defend_cheongryu_with_white_path",
            "--action",
            "choice:accept_medicine_with_written_debt",
            "--action",
            "choice:watch_the_stolen_qingliu_flow",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("무명 첫 대치"));
    assert!(stdout.contains("visual id: wuxia_mumyeong_first_confrontation"));
    assert!(stdout.contains("layout: rival_first_confrontation"));
    assert!(stdout.contains("stable terms: 무명 / 서하린 / 청류문"));
    assert!(stdout.contains("choice:meet_mumyeong_head_on / 무명과 정면으로 맞선다"));
    assert!(stdout.contains(
        "choice:endure_until_copy_flow_breaks / 버티며 카피한 흐름이 끊기는 순간을 기다린다"
    ));
    assert!(
        stdout.contains("choice:watch_seo_harin_hold_back / 서하린이 왜 끼어들지 않는지 살핀다")
    );
    assert!(stdout
        .contains("choice:read_mumyeongs_copied_form / 무명의 초식이 어디서 어긋나는지 읽는다"));
    assert!(stdout.contains("choice:do_not_provoke_mumyeong / 도발하지 않고 물러설 거리를 만든다"));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_mumyeong_copy_style_reveal() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
            "--action",
            "choice:step_back_with_firewood",
            "--action",
            "choice:defend_cheongryu_with_white_path",
            "--action",
            "choice:accept_medicine_with_written_debt",
            "--action",
            "choice:watch_the_stolen_qingliu_flow",
            "--action",
            "choice:endure_until_copy_flow_breaks",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("무명의 카피 무공 공개"));
    assert!(stdout.contains("visual id: wuxia_mumyeong_copy_style_reveal"));
    assert!(stdout.contains("layout: copy_style_analysis"));
    assert!(stdout.contains("stable terms: 무명 / 청류안 / 천기록"));
    assert!(stdout
        .contains("choice:read_the_stolen_blade_path / 훔쳐 쓴 검로가 어디서 꺾이는지 읽는다"));
    assert!(stdout
        .contains("choice:watch_mumyeongs_footwork / 무명의 보법이 땅을 밀어내는 방식을 본다"));
    assert!(stdout
        .contains("choice:listen_for_breath_mismatch / 거리를 두고 호흡이 어긋나는 박자를 듣는다"));
    assert!(stdout.contains(
        "choice:wait_for_body_to_shudder / 몸에 맞지 않는 초식이 반동을 내는 순간까지 기다린다"
    ));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_mumyeong_reads_orthodox_style() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
            "--action",
            "choice:step_back_with_firewood",
            "--action",
            "choice:defend_cheongryu_with_white_path",
            "--action",
            "choice:accept_medicine_with_written_debt",
            "--action",
            "choice:watch_the_stolen_qingliu_flow",
            "--action",
            "choice:endure_until_copy_flow_breaks",
            "--action",
            "choice:listen_for_breath_mismatch",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("무명의 정파 무공 간파"));
    assert!(stdout.contains("visual id: wuxia_mumyeong_reads_orthodox_style"));
    assert!(stdout.contains("layout: orthodox_style_trace"));
    assert!(stdout.contains("stable terms: 현악문 / 복호금쇄수 / 무명"));
    assert!(stdout.contains(
        "choice:compare_copied_form_to_old_wound / 카피된 초식과 오래된 상처의 각도를 맞춰 본다"
    ));
    assert!(stdout.contains(
        "choice:trace_qingliu_eye_variation / 청류안 계열의 시선이 어디서 비틀렸는지 추적한다"
    ));
    assert!(stdout.contains(
        "choice:reconstruct_mumyeongs_sightline / 무명이 그날 보았을 시선을 따라 재구성한다"
    ));
    assert!(stdout.contains(
        "choice:stop_before_truth_becomes_accusation / 진실이 추궁이 되기 전에 기록을 덮는다"
    ));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_mumyeong_midgame_reunion() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
            "--action",
            "choice:step_back_with_firewood",
            "--action",
            "choice:defend_cheongryu_with_white_path",
            "--action",
            "choice:accept_medicine_with_written_debt",
            "--action",
            "choice:watch_the_stolen_qingliu_flow",
            "--action",
            "choice:endure_until_copy_flow_breaks",
            "--action",
            "choice:listen_for_breath_mismatch",
            "--action",
            "choice:reconstruct_mumyeongs_sightline",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("무명 중반 재회"));
    assert!(stdout.contains("visual id: wuxia_mumyeong_midgame_reunion"));
    assert!(stdout.contains("layout: rival_reunion_trace"));
    assert!(stdout.contains("stable terms: 무명 / 서하린 / 현악문"));
    assert!(stdout.contains(
        "choice:ask_why_seoharin_never_called_him_traitor / 서하린이 왜 그를 배신자라 부르지 않았는지 묻는다"
    ));
    assert!(stdout.contains(
        "choice:show_the_hyeonakmun_trace_without_accusing / 현악문 흔적을 추궁이 아니라 기록으로 보여 준다"
    ));
    assert!(stdout.contains(
        "choice:point_out_the_copied_form_gap / 훔친 초식과 이해한 흐름이 갈라지는 틈을 짚는다"
    ));
    assert!(stdout.contains(
        "choice:keep_blades_low_and_watch_his_answer / 칼끝을 낮추고 대답 대신 반응을 본다"
    ));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_boss_first_appearance() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
            "--action",
            "choice:step_back_with_firewood",
            "--action",
            "choice:defend_cheongryu_with_white_path",
            "--action",
            "choice:accept_medicine_with_written_debt",
            "--action",
            "choice:watch_the_stolen_qingliu_flow",
            "--action",
            "choice:endure_until_copy_flow_breaks",
            "--action",
            "choice:listen_for_breath_mismatch",
            "--action",
            "choice:reconstruct_mumyeongs_sightline",
            "--action",
            "choice:show_the_hyeonakmun_trace_without_accusing",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("보스 첫 등장"));
    assert!(stdout.contains("visual id: wuxia_boss_first_appearance"));
    assert!(stdout.contains("layout: boss_wall_pressure"));
    assert!(stdout.contains("stable terms: 흑사방주 / 무명 / 청류문"));
    assert!(stdout.contains(
        "choice:read_the_boss_flow_and_fail_to_move / 보스의 흐름을 읽지만 몸이 늦는 것을 인정한다"
    ));
    assert!(stdout
        .contains("choice:pull_seo_harin_behind_broken_gate / 서하린을 부서진 산문 뒤로 물린다"));
    assert!(stdout.contains(
        "choice:watch_mumyeong_answer_the_boss / 무명이 보스의 말에 어떻게 반응하는지 본다"
    ));
    assert!(stdout
        .contains("choice:retreat_before_the_second_step / 두 번째 걸음 전에 물러나 살아남는다"));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_mumyeong_request_for_aid() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
            "--action",
            "choice:step_back_with_firewood",
            "--action",
            "choice:defend_cheongryu_with_white_path",
            "--action",
            "choice:accept_medicine_with_written_debt",
            "--action",
            "choice:watch_the_stolen_qingliu_flow",
            "--action",
            "choice:endure_until_copy_flow_breaks",
            "--action",
            "choice:listen_for_breath_mismatch",
            "--action",
            "choice:reconstruct_mumyeongs_sightline",
            "--action",
            "choice:show_the_hyeonakmun_trace_without_accusing",
            "--action",
            "choice:watch_mumyeong_answer_the_boss",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("무명의 도움 요청"));
    assert!(stdout.contains("visual id: wuxia_mumyeong_request_for_aid"));
    assert!(stdout.contains("layout: failed_aid_records"));
    assert!(stdout.contains("stable terms: 무명 / 청류문 / 정파"));
    assert!(stdout.contains(
        "choice:search_the_rejected_aid_letters / 거절당한 도움 요청 서찰을 찾아 읽는다"
    ));
    assert!(stdout
        .contains("choice:follow_old_inn_rumors_about_mumyeong / 객잔에 남은 무명 소문을 좇는다"));
    assert!(stdout.contains(
        "choice:ask_seo_harin_what_help_never_came / 서하린에게 오지 않았던 도움을 묻는다"
    ));
    assert!(stdout.contains(
        "choice:keep_the_failed_aid_record_unshown / 실패한 도움 요청 기록을 아직 보여주지 않는다"
    ));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_mumyeong_awakening() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
            "--action",
            "choice:step_back_with_firewood",
            "--action",
            "choice:defend_cheongryu_with_white_path",
            "--action",
            "choice:accept_medicine_with_written_debt",
            "--action",
            "choice:watch_the_stolen_qingliu_flow",
            "--action",
            "choice:endure_until_copy_flow_breaks",
            "--action",
            "choice:listen_for_breath_mismatch",
            "--action",
            "choice:reconstruct_mumyeongs_sightline",
            "--action",
            "choice:show_the_hyeonakmun_trace_without_accusing",
            "--action",
            "choice:watch_mumyeong_answer_the_boss",
            "--action",
            "choice:search_the_rejected_aid_letters",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("무명의 각성"));
    assert!(stdout.contains("visual id: wuxia_mumyeong_awakening"));
    assert!(stdout.contains("layout: anger_copy_bloom"));
    assert!(stdout.contains("stable terms: 무명 / 카피 / 분노"));
    assert!(stdout.contains(
        "choice:compare_anger_to_copied_flow / 분노가 베껴 낸 흐름과 복사한 초식을 비교한다"
    ));
    assert!(stdout.contains(
        "choice:trace_awakening_from_failed_aid / 도움 요청 실패가 각성으로 이어진 흔적을 좇는다"
    ));
    assert!(stdout.contains(
        "choice:ask_what_the_copy_cost_him / 그 카피가 무명에게 무엇을 빼앗았는지 묻는다"
    ));
    assert!(stdout.contains(
        "choice:stop_before_calling_it_salvation / 이것을 아직 구원이라고 부르지 않는다"
    ));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_qingliu_attack_after_war() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
            "--action",
            "choice:step_back_with_firewood",
            "--action",
            "choice:defend_cheongryu_with_white_path",
            "--action",
            "choice:accept_medicine_with_written_debt",
            "--action",
            "choice:watch_the_stolen_qingliu_flow",
            "--action",
            "choice:endure_until_copy_flow_breaks",
            "--action",
            "choice:listen_for_breath_mismatch",
            "--action",
            "choice:reconstruct_mumyeongs_sightline",
            "--action",
            "choice:show_the_hyeonakmun_trace_without_accusing",
            "--action",
            "choice:watch_mumyeong_answer_the_boss",
            "--action",
            "choice:search_the_rejected_aid_letters",
            "--action",
            "choice:compare_anger_to_copied_flow",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("무너져가는 청류문 습격의 흔적"));
    assert!(stdout.contains("visual id: wuxia_qingliu_attack_after_war"));
    assert!(stdout.contains("layout: attack_trace_investigation"));
    assert!(stdout.contains("stable terms: 청류문 / 현악문 / 복호금쇄수"));
    assert!(stdout.contains(
        "choice:inspect_bokho_lock_scars / 자물쇠와 문틀에 남은 복호금쇄수 자국을 살핀다"
    ));
    assert!(stdout.contains(
        "choice:compare_hyeonakmun_trace_to_qingliu_wounds / 현악문 흔적과 청류문 상처의 결을 대조한다"
    ));
    assert!(stdout.contains(
        "choice:ask_seo_harin_what_she_saw_afterward / 서하린이 습격 뒤에 무엇을 보았는지 조심스레 묻는다"
    ));
    assert!(stdout.contains(
        "choice:stop_before_replaying_the_attack / 습격을 다시 재생하기 전에 기록을 덮는다"
    ));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_mumyeong_destroys_orthodox_sect() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
            "--action",
            "choice:step_back_with_firewood",
            "--action",
            "choice:defend_cheongryu_with_white_path",
            "--action",
            "choice:accept_medicine_with_written_debt",
            "--action",
            "choice:watch_the_stolen_qingliu_flow",
            "--action",
            "choice:endure_until_copy_flow_breaks",
            "--action",
            "choice:listen_for_breath_mismatch",
            "--action",
            "choice:reconstruct_mumyeongs_sightline",
            "--action",
            "choice:show_the_hyeonakmun_trace_without_accusing",
            "--action",
            "choice:watch_mumyeong_answer_the_boss",
            "--action",
            "choice:search_the_rejected_aid_letters",
            "--action",
            "choice:compare_anger_to_copied_flow",
            "--action",
            "choice:inspect_bokho_lock_scars",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("비어 버린 현악문 산문"));
    assert!(stdout.contains("visual id: wuxia_mumyeong_destroys_orthodox_sect"));
    assert!(stdout.contains("layout: hyeonakmun_empty_gate_record"));
    assert!(stdout.contains("stable terms: 현악문 / 복호금쇄수 / 무명"));
    assert!(stdout.contains(
        "choice:read_hyeonakmun_empty_gate_record / 빈 현악문 산문에 남은 기록을 읽는다"
    ));
    assert!(stdout.contains(
        "choice:trace_bokho_lock_to_mumyeong / 복호금쇄수 흔적이 무명의 분노로 되돌아간 길을 대조한다"
    ));
    assert!(stdout.contains(
        "choice:ask_why_seoharin_never_heard_full_story / 왜 서하린이 전체 이야기를 듣지 못했는지 조심스럽게 묻는다"
    ));
    assert!(stdout.contains(
        "choice:stop_before_counting_the_dead / 죽은 사람의 수를 세기 전에 기록을 덮는다"
    ));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_boss_recruits_mumyeong() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
            "--action",
            "choice:step_back_with_firewood",
            "--action",
            "choice:defend_cheongryu_with_white_path",
            "--action",
            "choice:accept_medicine_with_written_debt",
            "--action",
            "choice:watch_the_stolen_qingliu_flow",
            "--action",
            "choice:endure_until_copy_flow_breaks",
            "--action",
            "choice:listen_for_breath_mismatch",
            "--action",
            "choice:reconstruct_mumyeongs_sightline",
            "--action",
            "choice:show_the_hyeonakmun_trace_without_accusing",
            "--action",
            "choice:watch_mumyeong_answer_the_boss",
            "--action",
            "choice:search_the_rejected_aid_letters",
            "--action",
            "choice:compare_anger_to_copied_flow",
            "--action",
            "choice:inspect_bokho_lock_scars",
            "--action",
            "choice:read_hyeonakmun_empty_gate_record",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("흑사방 보스의 스카웃 흔적"));
    assert!(stdout.contains("visual id: wuxia_boss_recruits_mumyeong"));
    assert!(stdout.contains("layout: boss_recruitment_trace"));
    assert!(stdout.contains("stable terms: 흑사방주 / 무명 / 현악문"));
    assert!(stdout.contains(
        "choice:trace_boss_offer_after_hyeonakmun / 현악문 뒤에 이어진 보스의 제안을 추적한다"
    ));
    assert!(stdout.contains(
        "choice:read_mumyeong_choice_without_excusing_it / 무명의 선택을 변명하지 않고 해석한다"
    ));
    assert!(stdout.contains(
        "choice:search_black_serpent_recruitment_record / 흑사방의 스카웃 기록을 찾는다"
    ));
    assert!(stdout.contains(
        "choice:stop_before_following_him_into_black_serpent / 흑사방 안쪽까지 따라가기 전에 기록을 멈춘다"
    ));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_mumyeong_departure_truth_summary() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
            "--action",
            "choice:step_back_with_firewood",
            "--action",
            "choice:defend_cheongryu_with_white_path",
            "--action",
            "choice:accept_medicine_with_written_debt",
            "--action",
            "choice:watch_the_stolen_qingliu_flow",
            "--action",
            "choice:endure_until_copy_flow_breaks",
            "--action",
            "choice:listen_for_breath_mismatch",
            "--action",
            "choice:reconstruct_mumyeongs_sightline",
            "--action",
            "choice:show_the_hyeonakmun_trace_without_accusing",
            "--action",
            "choice:watch_mumyeong_answer_the_boss",
            "--action",
            "choice:search_the_rejected_aid_letters",
            "--action",
            "choice:compare_anger_to_copied_flow",
            "--action",
            "choice:inspect_bokho_lock_scars",
            "--action",
            "choice:read_hyeonakmun_empty_gate_record",
            "--action",
            "choice:trace_boss_offer_after_hyeonakmun",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("봉해 둔 이탈의 진실"));
    assert!(stdout.contains("visual id: wuxia_mumyeong_departure_truth_summary"));
    assert!(stdout.contains("layout: sealed_departure_truth_summary"));
    assert!(stdout.contains("stable terms: 무명 / 서하린 / 현악문 / 흑사방주"));
    assert!(stdout.contains(
        "choice:assemble_departure_truth_without_delivering / 전하지 않은 채 이탈의 진실을 정리한다"
    ));
    assert!(stdout.contains(
        "choice:compare_failed_aid_to_recruitment_offer / 거절당한 도움 요청과 보스의 제안을 나란히 놓는다"
    ));
    assert!(stdout.contains(
        "choice:ask_seoharin_what_she_is_ready_to_hear / 서하린이 무엇을 들을 준비가 되었는지 묻는다"
    ));
    assert!(stdout.contains(
        "choice:seal_truth_until_mumyeong_faces_it / 무명이 마주하기 전까지 진실을 봉해 둔다"
    ));
    assert!(!stdout.contains("dev_desk"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_seoharin_empty_place() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
            "--action",
            "choice:step_back_with_firewood",
            "--action",
            "choice:defend_cheongryu_with_white_path",
            "--action",
            "choice:accept_medicine_with_written_debt",
            "--action",
            "choice:watch_the_stolen_qingliu_flow",
            "--action",
            "choice:endure_until_copy_flow_breaks",
            "--action",
            "choice:listen_for_breath_mismatch",
            "--action",
            "choice:reconstruct_mumyeongs_sightline",
            "--action",
            "choice:show_the_hyeonakmun_trace_without_accusing",
            "--action",
            "choice:watch_mumyeong_answer_the_boss",
            "--action",
            "choice:search_the_rejected_aid_letters",
            "--action",
            "choice:compare_anger_to_copied_flow",
            "--action",
            "choice:inspect_bokho_lock_scars",
            "--action",
            "choice:read_hyeonakmun_empty_gate_record",
            "--action",
            "choice:trace_boss_offer_after_hyeonakmun",
            "--action",
            "choice:assemble_departure_truth_without_delivering",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("비워둔 자리"));
    assert!(stdout.contains("visual id: wuxia_seoharin_empty_place"));
    assert!(stdout.contains("layout: empty_place_memory"));
    assert!(stdout.contains("stable terms: 서하린 / 무명 / 청류문 / 목검"));
    assert!(stdout
        .contains("choice:ask_who_kept_the_empty_place / 누가 이 빈 자리를 지켜 왔는지 묻는다"));
    assert!(stdout
        .contains("choice:leave_the_place_unclaimed / 그 자리를 누구의 것이라고 부르지 않는다"));
    assert!(stdout.contains(
        "choice:set_down_the_work_notebook_briefly / 업무수첩을 잠시 내려놓고 목검을 본다"
    ));
    assert!(stdout.contains(
        "choice:step_back_without_naming_mumyeong / 무명의 이름을 꺼내지 않고 한 걸음 물러난다"
    ));
    assert!(!stdout.contains("told_seoharin_truth"));
    assert!(!stdout.contains("item_unpriced_wooden_sword"));
}

#[test]
fn content_tui_smoke_reaches_wuxia_seoharin_left_meal() {
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--storypack-preview",
            "wuxia_jianghu_pack",
            "--seed",
            "123",
            "--tui-smoke",
            "--action",
            "choice:follow_roadside_dust",
            "--action",
            "move:jianghu_market_street",
            "--action",
            "choice:run_toward_open_street",
            "--action",
            "choice:choose_failure_log",
            "--action",
            "choice:tell_plain_truth",
            "--action",
            "choice:accept_three_month_trial",
            "--action",
            "choice:step_back_with_firewood",
            "--action",
            "choice:defend_cheongryu_with_white_path",
            "--action",
            "choice:accept_medicine_with_written_debt",
            "--action",
            "choice:watch_the_stolen_qingliu_flow",
            "--action",
            "choice:endure_until_copy_flow_breaks",
            "--action",
            "choice:listen_for_breath_mismatch",
            "--action",
            "choice:reconstruct_mumyeongs_sightline",
            "--action",
            "choice:show_the_hyeonakmun_trace_without_accusing",
            "--action",
            "choice:watch_mumyeong_answer_the_boss",
            "--action",
            "choice:search_the_rejected_aid_letters",
            "--action",
            "choice:compare_anger_to_copied_flow",
            "--action",
            "choice:inspect_bokho_lock_scars",
            "--action",
            "choice:read_hyeonakmun_empty_gate_record",
            "--action",
            "choice:trace_boss_offer_after_hyeonakmun",
            "--action",
            "choice:assemble_departure_truth_without_delivering",
            "--action",
            "choice:set_down_the_work_notebook_briefly",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("위치: 청류문 외곽 마당 (cheongryu_outer_courtyard)"));
    assert!(stdout.contains("남겨둔 밥"));
    assert!(stdout.contains("visual id: wuxia_seoharin_left_meal"));
    assert!(stdout.contains("layout: left_meal_memory"));
    assert!(stdout.contains("stable terms: 서하린 / 밥그릇 / 청류문 / 귀환"));
    assert!(stdout.contains("choice:eat_the_left_meal_quietly / 말없이 남겨 둔 밥을 먹는다"));
    assert!(stdout.contains("choice:thank_seoharin_for_the_bowl / 서하린에게 고맙다고 말한다"));
    assert!(stdout
        .contains("choice:joke_about_who_ordered_extra_rice / 누가 밥을 더 지었냐고 농담한다"));
    assert!(stdout.contains("choice:pass_without_eating_the_meal / 먹지 않고 지나친다"));
    assert!(!stdout.contains("told_seoharin_truth"));
    assert!(!stdout.contains("item_unpriced_wooden_sword"));
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
fn content_tui_smoke_renders_escape_aftermath_ending_panel() {
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
            "move:hallway",
            "--action",
            "move:emergency_stairs",
            "--action",
            "choice:align_breathing_floor",
            "--action",
            "choice:solve_distorted_floor",
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
    assert!(stdout.contains("격리 6턴 · 엔딩"));
    assert!(stdout.contains("퇴근 성공"));
    assert!(stdout.contains("[POST-ESCAPE REPORT]"));
    assert!(stdout.contains("survivor_count: 1"));
    assert!(stdout.contains("evidence_level: 0"));
    assert!(stdout.contains("company_response: denial"));
    assert!(stdout.contains("employee_status: access_revoked"));
    assert!(stdout.contains("risk_level: ongoing"));
    assert!(stdout.contains("ENDING: 정문 밖"));
    assert!(action_ids_from_terminal_snapshot(&stdout).is_empty());
    assert!(!stdout.contains("final_hint"));
    assert!(!stdout.contains("treasure_location"));
}

#[test]
fn content_tui_smoke_renders_schema_less_combat_intervention_panel() {
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
            "move:supply_closet",
            "--action",
            "choice:brace_for_supply_scuffle",
        ])
        .output()
        .expect("escape-terminal executable should run");

    assert!(
        output.status.success(),
        "expected success, stderr was: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[현재 인카운터]"));
    assert!(stdout.contains("물품창고 자동 난투"));
    assert!(stdout.contains("visual id: supply_closet_scuffle"));
    assert!(stdout.contains("layout: combat_intervention"));
    assert!(stdout.contains("stable terms: 거리 / 균형 / 소화기 핀"));
    assert!(
        stdout.contains("choice:hook_cart_to_cabinet / 캐비닛 손잡이에 카트를 걸어 거리를 만든다")
    );
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
fn content_app_smoke_renders_full_screen_frame_with_tick_raw_draw_glyphfx() {
    let bundle_path = content_bundle_path();
    let output = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args([
            "--scene",
            "content",
            "--content-bundle",
            bundle_path.to_str().expect("bundle path should be UTF-8"),
            "--seed",
            "123",
            "--app-smoke",
            "--tick",
            "7",
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
    assert!(stdout.contains("[SuperLightTUI App Smoke]"));
    assert!(stdout.contains("app loop: full-screen SuperLightTUI frame"));
    assert!(stdout.contains("tick: 7"));
    assert!(stdout.contains("[RAW-DRAW GLYPHFX LAYER]"));
    assert!(stdout.contains("raw-draw glyphfx tick=7"));
    assert!(stdout.contains("stable terms: 비상계단 / 토너 / 접힌 방향"));
    assert!(stdout.contains("fallback: 출력물의 깨진 글자 사이로 '비상계단'이 선명하게 남는다."));
    assert!(stdout.contains("입력: 번호 1-3 · q 종료 · ? 도움말"));
}

#[test]
fn content_app_smoke_tick_changes_glyphfx_frame_without_losing_stable_terms() {
    let bundle_path = content_bundle_path();
    let args = |tick: &str| {
        vec![
            "--scene".to_string(),
            "content".to_string(),
            "--content-bundle".to_string(),
            bundle_path
                .to_str()
                .expect("bundle path should be UTF-8")
                .to_string(),
            "--seed".to_string(),
            "123".to_string(),
            "--app-smoke".to_string(),
            "--tick".to_string(),
            tick.to_string(),
            "--action".to_string(),
            "choice:check_message".to_string(),
            "--action".to_string(),
            "move:dev_office".to_string(),
            "--action".to_string(),
            "move:printer_area".to_string(),
        ]
    };
    let frame_a = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args(args("1"))
        .output()
        .expect("escape-terminal executable should run");
    let frame_b = Command::new(env!("CARGO_BIN_EXE_escape-terminal"))
        .args(args("2"))
        .output()
        .expect("escape-terminal executable should run");

    assert!(frame_a.status.success());
    assert!(frame_b.status.success());
    let stdout_a = String::from_utf8_lossy(&frame_a.stdout);
    let stdout_b = String::from_utf8_lossy(&frame_b.stdout);
    let wave_a = raw_glyphfx_wave(&stdout_a);
    let wave_b = raw_glyphfx_wave(&stdout_b);
    assert_ne!(wave_a, wave_b, "tick should alter raw-draw GlyphFX cells");
    assert!(stdout_a.contains("stable terms: 비상계단 / 토너 / 접힌 방향"));
    assert!(stdout_b.contains("stable terms: 비상계단 / 토너 / 접힌 방향"));
    assert!(stdout_a.contains("fallback: 출력물의 깨진 글자 사이로 '비상계단'이 선명하게 남는다."));
    assert!(stdout_b.contains("fallback: 출력물의 깨진 글자 사이로 '비상계단'이 선명하게 남는다."));
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

fn wuxia_preview_bundle_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json",
    )
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

fn raw_glyphfx_wave(stdout: &str) -> &str {
    stdout
        .lines()
        .find_map(|line| {
            line.trim()
                .strip_prefix("raw-draw glyphfx tick=")?
                .split_once(' ')
                .map(|(_, wave)| wave)
        })
        .expect("raw-draw GlyphFX wave should be rendered")
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

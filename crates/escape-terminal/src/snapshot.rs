use super::*;
use escape_core::{
    CombatConclusionOutcome, CombatConclusionReason, CombatConclusionReport, CombatSide,
    CombatSpectatorCue, CombatSpectatorLogEntry, CombatSpectatorPiece, CombatSpectatorView,
};
use std::collections::BTreeMap;

pub(crate) fn render_scene_page_snapshot(page: &ScenePage, logs: &[String]) -> String {
    let mut lines = Vec::new();
    lines.push(scene_page_terminal_title(page).to_string());
    lines.push(format!(
        "{} · {}",
        page.chapter_label,
        scene_mode_label(&page.mode)
    ));
    lines.push("[상태]".to_string());
    lines.push(format!("턴: {}", page.status_summary.turn));
    lines.push(format!(
        "위치: {} ({})",
        page.location.name, page.location.id
    ));
    lines.push(format!(
        "체력: {}  정신력: {}  배터리: {}  허기: {}  갈증: {}  위험도: {}",
        resource_value(page, "health"),
        resource_value(page, "sanity"),
        resource_value(page, "battery"),
        resource_value(page, "hunger"),
        resource_value(page, "thirst"),
        page.status_summary.danger
    ));
    for warning in &page.status_summary.warnings {
        lines.push(format!("! {warning}"));
    }

    if page
        .content_stream
        .iter()
        .any(|item| item.stage_id.is_some())
    {
        render_ordered_content_stream(&mut lines, page);
        render_combat_section(&mut lines, page);
        render_snapshot_tail(&mut lines, page, logs);
        return lines.join("\n");
    }

    lines.push("[비주얼]".to_string());
    for line in scene_visual_card_lines(page) {
        lines.push(line);
    }

    if matches!(page.mode, SceneMode::Encounter) {
        lines.push("[현재 인카운터]".to_string());
        lines.push(page.title.clone());
        render_scene_body(&mut lines, page);
    }

    lines.push("[현재 행동]".to_string());
    if !matches!(page.mode, SceneMode::Encounter) {
        lines.push(page.title.clone());
        render_scene_body(&mut lines, page);
    }
    for (index, action) in page.actions.iter().enumerate() {
        lines.push(scene_action_line(index + 1, action));
    }
    if !page.blocked_actions.is_empty() {
        lines.push("[잠긴 선택지]".to_string());
        for action in &page.blocked_actions {
            lines.push(scene_blocked_action_line(action));
            lines.push(format!("   이유: {}", action.reasons.join(", ")));
        }
    }

    render_combat_section(&mut lines, page);
    render_snapshot_logs(&mut lines, logs);
    lines.join("\n")
}

fn render_snapshot_tail(lines: &mut Vec<String>, page: &ScenePage, logs: &[String]) {
    if !page.blocked_actions.is_empty() {
        lines.push("[잠긴 선택지]".to_string());
        for action in &page.blocked_actions {
            lines.push(scene_blocked_action_line(action));
            lines.push(format!("   이유: {}", action.reasons.join(", ")));
        }
    }
    render_snapshot_logs(lines, logs);
}

fn render_snapshot_logs(lines: &mut Vec<String>, logs: &[String]) {
    lines.push("[최근 로그]".to_string());
    if logs.is_empty() {
        lines.push("- 아직 기록된 로그가 없다.".to_string());
    } else {
        for log in logs {
            lines.push(format!("- {log}"));
        }
    }
}

fn render_ordered_content_stream(lines: &mut Vec<String>, page: &ScenePage) {
    if matches!(page.mode, SceneMode::Encounter) {
        lines.push("[현재 인카운터]".to_string());
    }
    lines.push(page.title.clone());
    if !page
        .content_stream
        .iter()
        .any(|item| item.kind == "illustration")
    {
        // Terminal snapshots retain the legacy encounter visual anchor while a
        // staged cursor is waiting for input; Web still renders content_stream
        // strictly in authored order.
        lines.push("[일러스트]".to_string());
        lines.extend(scene_visual_card_lines(page));
    }
    for item in &page.content_stream {
        render_content_item(lines, item, page);
    }
}

fn render_content_item(lines: &mut Vec<String>, item: &SceneContentItem, page: &ScenePage) {
    match item.kind.as_str() {
        "illustration" => {
            lines.push("[일러스트]".to_string());
            if item.placeholder || item.visual_id.is_none() {
                lines.push(format!(
                    "[NO IMAGE] {}",
                    item.alt.as_deref().unwrap_or("no image")
                ));
            } else {
                lines.push(format!(
                    "visual id: {}",
                    item.visual_id.as_deref().unwrap_or_default()
                ));
                lines.push(format!("alt: {}", item.alt.as_deref().unwrap_or_default()));
                lines.extend(glyphfx_card_lines(&page.effect_cues));
            }
        }
        "choice" | "continue" => {
            lines.push("[선택]".to_string());
            for (index, action) in item.actions.iter().enumerate() {
                lines.push(scene_action_line(index + 1, action));
            }
        }
        "dialogue" => {
            if let Some(text) = item.text.as_deref() {
                if let Some(speaker) = item.speaker.as_deref() {
                    lines.push(format!("{speaker}: {text}"));
                } else {
                    lines.push(text.to_string());
                }
            }
        }
        "result_summary" => push_wrapped_content(lines, "[결과]", item.text.as_deref()),
        "document" | "cheongirok" => push_wrapped_content(lines, "[기록]", item.text.as_deref()),
        "system" => push_wrapped_content(lines, "[시스템]", item.text.as_deref()),
        _ => push_wrapped_content(lines, "[이야기]", item.text.as_deref()),
    }
}

fn push_wrapped_content(lines: &mut Vec<String>, heading: &str, text: Option<&str>) {
    lines.push(heading.to_string());
    for source_line in text.unwrap_or_default().lines() {
        lines.extend(wrap_terminal_body_line(source_line, 76));
    }
}

pub(crate) fn render_scene_body(lines_buf: &mut Vec<String>, page: &ScenePage) {
    for entry in &page.dialogue_entries {
        lines_buf.push(format!("{}: {}", entry.speaker, entry.text));
    }
    for block in &page.body_blocks {
        if let Some(heading) = scene_body_block_heading(block) {
            lines_buf.push(heading.to_string());
        }
        let lines = compact_terminal_body_block_lines(block);
        for line in lines {
            for wrapped in wrap_terminal_body_line(line, 76) {
                lines_buf.push(wrapped);
            }
        }
    }
}

pub(crate) fn compact_terminal_body_block_lines(block: &BodyBlock) -> Vec<&str> {
    match block.kind.as_str() {
        "epilogue_result" => block
            .text
            .lines()
            .filter(|line| {
                line.starts_with("final_result_key:")
                    || line.starts_with("result_title:")
                    || line.starts_with("owned_by:")
            })
            .collect(),
        "epilogue_card" => block
            .text
            .lines()
            .filter(|line| line.starts_with("card_id:"))
            .collect(),
        "epilogue_state_audit" => block
            .text
            .lines()
            .filter(|line| {
                line.starts_with("audit_id:")
                    || line.starts_with("source_contract:")
                    || line.starts_with("final_result_key:")
                    || line.starts_with("canonical_state: combat_result")
                    || line.starts_with("canonical_state: boss_resolution_route")
            })
            .collect(),
        "epilogue_suppressed" => block
            .text
            .lines()
            .filter(|line| line.starts_with("card_id:") || line.starts_with("suppressed_by:"))
            .collect(),
        _ => block.text.lines().collect(),
    }
}

pub(crate) fn scene_body_block_heading(block: &BodyBlock) -> Option<&'static str> {
    match block.kind.as_str() {
        "epilogue_result" => Some("[결산 판정]"),
        "epilogue_state_audit" => Some("[결산 상태 감사]"),
        "epilogue_suppressed" => Some("[억제된 후일담 후보]"),
        "epilogue_contract_error" => Some("[후일담 계약 오류]"),
        _ => None,
    }
}

pub(crate) fn wrap_terminal_body_line(line: &str, max_chars: usize) -> Vec<String> {
    let mut rows = Vec::new();
    let mut remaining = line.trim_end();
    while remaining.chars().count() > max_chars {
        let mut last_space_byte = None;
        let mut hard_break_byte = remaining.len();
        for (char_count, (byte_index, ch)) in remaining.char_indices().enumerate() {
            if char_count >= max_chars {
                hard_break_byte = byte_index;
                break;
            }
            if ch.is_whitespace() {
                last_space_byte = Some(byte_index);
            }
        }
        let break_byte = last_space_byte
            .filter(|byte_index| *byte_index > 0)
            .unwrap_or(hard_break_byte);
        rows.push(remaining[..break_byte].trim_end().to_string());
        remaining = remaining[break_byte..].trim_start();
    }
    if rows.is_empty() || !remaining.is_empty() {
        rows.push(remaining.to_string());
    }
    rows
}

pub(crate) fn scene_visual_card_lines(page: &ScenePage) -> Vec<String> {
    let mut lines = vec![
        "╭─ VISUAL CARD ─────────────────────────╮".to_string(),
        format!("│ visual id: {}", page.visual.id),
        format!("│ layout: {}", page.visual.kind),
        format!("│ alt: {}", page.visual.alt),
    ];
    lines.extend(glyphfx_card_lines(&page.effect_cues));
    lines.push("╰────────────────────────────────────────╯".to_string());
    lines
}

pub(crate) fn scene_page_terminal_title(page: &ScenePage) -> &'static str {
    if is_wuxia_scene_page(page) {
        "이구학지 - 천기록 // TERMINAL STORYBOOK"
    } else {
        "ESCAPE OFFICE // TERMINAL EDITION"
    }
}

pub(crate) fn is_wuxia_scene_page(page: &ScenePage) -> bool {
    page.location.id.starts_with("wuxia_")
        || page.visual.id.contains("wuxia")
        || page
            .visual
            .source_id
            .as_deref()
            .is_some_and(|source_id| source_id.contains("wuxia"))
}

pub(crate) fn render_turn_view_snapshot(
    view: &TurnView,
    state: &GameState,
    location_name: &str,
    logs: &[String],
) -> String {
    let mut lines = Vec::new();
    lines.push("ESCAPE OFFICE // TERMINAL EDITION".to_string());
    lines.push("legacy printer scene · TurnView bridge".to_string());
    lines.push("[상태]".to_string());
    lines.push(format!("턴: {}", state.turn));
    lines.push(format!("위치: {location_name} ({})", state.location_id));
    lines.push(format!(
        "체력: {}  정신력: {}  배터리: {}  위험도: {}",
        state.player.health, state.player.sanity, state.player.battery, state.danger
    ));
    lines.push("[비주얼]".to_string());
    lines.push(glyphfx_turn_line(&view.effect_cues));

    if view.encounter_id.is_some() {
        lines.push("[현재 인카운터]".to_string());
        lines.push(view.title.clone());
        lines.push(view.body.clone());
    }

    lines.push("[현재 행동]".to_string());
    if view.encounter_id.is_none() {
        lines.push(view.title.clone());
        lines.push(view.body.clone());
    }
    for (index, action) in view.actions.iter().enumerate() {
        lines.push(turn_action_line(index + 1, action));
    }
    if !view.blocked_actions.is_empty() {
        lines.push("[잠긴 선택지]".to_string());
        for action in &view.blocked_actions {
            lines.push(turn_blocked_action_line(action));
            lines.push(format!("   이유: {}", action.reasons.join(", ")));
        }
    }

    lines.push("[최근 로그]".to_string());
    if logs.is_empty() {
        lines.push("- 아직 기록된 로그가 없다.".to_string());
    } else {
        for log in logs {
            lines.push(format!("- {log}"));
        }
    }
    lines.join("\n")
}

pub(crate) fn scene_mode_label(mode: &SceneMode) -> &'static str {
    match mode {
        SceneMode::Encounter => "인카운터",
        SceneMode::Movement => "이동",
        SceneMode::Ending => "엔딩",
    }
}

pub(crate) fn resource_value(page: &ScenePage, id: &str) -> i32 {
    page.status_summary
        .resources
        .iter()
        .find(|resource| resource.id == id)
        .map(|resource| resource.value)
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Wave 3 Step 1d-1: terminal 관전 렌더러.
//
// Hard invariant: this module only *formats* `ScenePage.combat`. It never
// calls `resolve_combat` / `conclude_combat` / `spectate_combat` and never
// recomputes damage totals, outcomes, or cue derivation — those are all
// already decided by `escape-core` (정본 13 "감독형 관전·전략 피드백 시스템").
// ---------------------------------------------------------------------------

// -- P1: 로그 템플릿 테이블 ---------------------------------------------------

/// hundredths 정수를 반올림 정수로 표시한다 (정본 11 §8 "UI는 정수 반올림으로
/// 표시한다"). 부동소수점을 쓰지 않아 결과가 플랫폼과 무관하게 결정론적이다.
fn round_hundredths_to_int(value_hundredths: i64) -> i64 {
    let sign: i64 = if value_hundredths < 0 { -1 } else { 1 };
    let magnitude = value_hundredths.unsigned_abs();
    let rounded = (magnitude + 50) / 100;
    sign * rounded as i64
}

/// 6개 template id -> 한국어 문장. 알 수 없는 id는 조용히 버리지 않고
/// `template_id` 자체를 노출하는 fallback 줄을 만든다 (정본: 로그 계약 위반 금지).
fn combat_log_template_line(entry: &CombatSpectatorLogEntry) -> String {
    let actor = entry.actor_id.as_str();
    let target = entry.target_id.as_deref();
    match entry.template_id.as_str() {
        "combat.log.move_intent" => match target {
            Some(target) => format!("{actor} 이동 의도 (목표 {target})"),
            None => format!("{actor} 이동 의도"),
        },
        "combat.log.target_selection" => match target {
            Some(target) => format!("{actor} → 목표 지정: {target}"),
            None => format!("{actor} 목표 지정 (대상 없음)"),
        },
        "combat.log.collision" => match target {
            Some(target) => format!("{actor} × {target} 충돌"),
            None => format!("{actor} 충돌 (대상 없음)"),
        },
        "combat.log.damage_applied" => {
            let value = entry
                .value_hundredths
                .map(round_hundredths_to_int)
                .unwrap_or(0);
            match target {
                Some(target) => format!("{actor} → {target} 피해 {value}"),
                None => format!("{actor} 피해 {value} (대상 없음)"),
            }
        }
        "combat.log.effect_applied" => {
            let effect = entry.effect_id.as_deref().unwrap_or("(효과 id 없음)");
            match target {
                Some(target) => format!("{actor} → {target} 효과 적용 [{effect}]"),
                None => format!("{actor} 효과 적용 [{effect}] (대상 없음)"),
            }
        }
        "combat.log.effect_applied_hidden" => match target {
            Some(target) => format!("{actor} → {target} 효과 적용 [정체불명]"),
            None => format!("{actor} 효과 적용 [정체불명] (대상 없음)"),
        },
        other => format!(
            "{actor} → {} 알 수 없는 사건 [template_id={other}]",
            target.unwrap_or("(대상 없음)")
        ),
    }
}

// -- P2: 체스말 보드 ----------------------------------------------------------

/// Board render caps (정본: 스케일 축소 대신 좌표 목록으로 대체한다). Width/height
/// are measured as `max - min` over the last frame's piece positions.
const COMBAT_BOARD_MAX_WIDTH: i64 = 32;
const COMBAT_BOARD_MAX_HEIGHT: i64 = 16;

/// cue 5종 -> 텍스트 표식 대응표 (정본 13의 연출 문법을 문자 표식으로 옮긴 것).
/// 이 표는 `docs/dev/TUI_Layout.md`의 동일 표와 짝을 이룬다 — 한쪽만 고치지 말 것.
///
/// | cue              | 정본 연출 의미      | 표식 |
/// |------------------|--------------------|------|
/// | Attack           | 짧은 전진/복귀       | `>`  |
/// | Hit              | 밀림/진동           | `<`  |
/// | Evade            | 측면 이동           | `~`  |
/// | BalanceBroken    | 흔들림/기울어짐      | `!`  |
/// | Incapacitated    | 흐려짐/표식         | `x`  |
fn combat_cue_symbol(cue: CombatSpectatorCue) -> char {
    match cue {
        CombatSpectatorCue::Attack => '>',
        CombatSpectatorCue::Hit => '<',
        CombatSpectatorCue::Evade => '~',
        CombatSpectatorCue::BalanceBroken => '!',
        CombatSpectatorCue::Incapacitated => 'x',
    }
}

const COMBAT_LEGEND_LINE: &str =
    "표기: A/E=아군/적(생존) a/e=아군/적(비활성) · > 공격 · < 피격 · ~ 회피 · ! 균형붕괴 · x 전투불능";

/// 말 한 개의 보드 토큰: 진영/생존 문자 + cue 표식들 (core가 정한 순서, Attack ->
/// Hit -> Evade -> BalanceBroken -> Incapacitated 그대로 이어붙인다).
fn combat_piece_token(piece: &CombatSpectatorPiece) -> String {
    let side_char = match (piece.side, piece.active) {
        (CombatSide::Ally, true) => 'A',
        (CombatSide::Ally, false) => 'a',
        (CombatSide::Enemy, true) => 'E',
        (CombatSide::Enemy, false) => 'e',
    };
    let mut token = String::new();
    token.push(side_char);
    for cue in &piece.cues {
        token.push(combat_cue_symbol(*cue));
    }
    token
}

/// `view.frames`의 마지막 프레임만 그린다 (정적 스냅샷이므로 결착 시점이
/// 가장 정보량이 많다 — 정본: 시간 조작 금지, 애니메이션 없음).
fn render_combat_board(lines: &mut Vec<String>, view: &CombatSpectatorView) {
    lines.push("[전투 판]".to_string());
    let Some(frame) = view.frames.last() else {
        lines.push("- 표시할 프레임이 없다.".to_string());
        return;
    };
    let elapsed_millis = u64::from(frame.tick) * u64::from(view.tick_millis);
    lines.push(format!("tick {} · 경과 {elapsed_millis}ms", frame.tick));

    if frame.pieces.is_empty() {
        lines.push("- 표시할 말이 없다 (전투원 0명).".to_string());
        return;
    }

    let mut sorted_pieces: Vec<&CombatSpectatorPiece> = frame.pieces.iter().collect();
    sorted_pieces.sort_by(|a, b| a.id.cmp(&b.id));

    let (mut min_x, mut max_x, mut min_y, mut max_y) = {
        let first = &sorted_pieces[0];
        (
            first.position.x,
            first.position.x,
            first.position.y,
            first.position.y,
        )
    };
    for piece in &sorted_pieces[1..] {
        min_x = min_x.min(piece.position.x);
        max_x = max_x.max(piece.position.x);
        min_y = min_y.min(piece.position.y);
        max_y = max_y.max(piece.position.y);
    }
    let x_span = i64::from(max_x) - i64::from(min_x);
    let y_span = i64::from(max_y) - i64::from(min_y);

    if x_span > COMBAT_BOARD_MAX_WIDTH || y_span > COMBAT_BOARD_MAX_HEIGHT {
        lines.push(format!(
            "- 보드 범위(폭 {x_span}, 높이 {y_span})가 상한(폭 {COMBAT_BOARD_MAX_WIDTH}, 높이 {COMBAT_BOARD_MAX_HEIGHT})을 넘어 좌표 목록으로 대체한다 (스케일 축소는 하지 않는다)."
        ));
        for piece in &sorted_pieces {
            lines.push(format!(
                "- {} {} @ ({}, {})",
                combat_piece_token(piece),
                piece.id,
                piece.position.x,
                piece.position.y
            ));
        }
        return;
    }

    let mut cell_tokens: BTreeMap<(i32, i32), Vec<String>> = BTreeMap::new();
    for piece in &sorted_pieces {
        cell_tokens
            .entry((piece.position.y, piece.position.x))
            .or_default()
            .push(combat_piece_token(piece));
    }
    for y in min_y..=max_y {
        let mut row = format!("y={y:>4}:");
        for x in min_x..=max_x {
            let cell = cell_tokens
                .get(&(y, x))
                .map(|tokens| tokens.join("/"))
                .unwrap_or_else(|| "·".to_string());
            row.push(' ');
            row.push_str(&cell);
        }
        lines.push(row);
    }
    lines.push(COMBAT_LEGEND_LINE.to_string());
}

// -- P3: 핵심 로그 -------------------------------------------------------------

/// terminal 화면에 노출하는 핵심 로그 줄 수 상한. 넘으면 생략 개수를 명시한다
/// (조용한 truncation 금지).
const COMBAT_CORE_LOG_LIMIT: usize = 20;

/// `core_log`만 문장화한다. `full_log`는 개수만 표시한다 (정본 07: 전체 로그는
/// 일시정지/전투 종료 뒤 별도 열람 — 이 slice는 전체 로그 열람 UI를 만들지 않는다).
fn render_combat_core_log(lines: &mut Vec<String>, view: &CombatSpectatorView) {
    lines.push("[전투 로그]".to_string());
    lines.push(format!(
        "전체 로그 {}건 (일시정지 또는 전투 종료 후 별도 열람, 이 화면은 개수만 표시)",
        view.full_log.len()
    ));
    if view.core_log.is_empty() {
        lines.push("- 핵심 로그가 없다.".to_string());
        return;
    }
    let total = view.core_log.len();
    let shown = total.min(COMBAT_CORE_LOG_LIMIT);
    for entry in &view.core_log[..shown] {
        lines.push(format!("- {}", combat_log_template_line(entry)));
    }
    if total > shown {
        lines.push(format!("- …(생략 {}줄)", total - shown));
    }
}

// -- P4: 보고서 ----------------------------------------------------------------

fn combat_outcome_label(outcome: CombatConclusionOutcome) -> &'static str {
    match outcome {
        CombatConclusionOutcome::InProgress => "진행 중",
        CombatConclusionOutcome::AllyVictory => "아군 승리",
        CombatConclusionOutcome::EnemyVictory => "적 승리",
        CombatConclusionOutcome::MutualDefeat => "양측 전멸",
        CombatConclusionOutcome::Stalemate => "무승부",
    }
}

fn combat_reason_label(reason: CombatConclusionReason) -> &'static str {
    match reason {
        CombatConclusionReason::NoTerminalCondition => "종료 조건 없음",
        CombatConclusionReason::AllEnemiesDefeated => "적 전멸",
        CombatConclusionReason::AllAlliesDefeated => "아군 전멸",
        CombatConclusionReason::BothSidesDefeated => "양측 전멸",
        CombatConclusionReason::MaxTicksReached => "최대 tick 도달",
    }
}

/// P4: `combat.report`가 `Some`일 때만 호출된다 (호출부: `render_combat_section`).
/// 금지: 전략 평가·핵심 전환점·자동 원인 분석·전략 조언·종합 MVP·이전 전투 비교 —
/// 이 함수는 `CombatConclusionReport` 필드를 그대로 옮기기만 한다.
fn render_combat_report(
    lines: &mut Vec<String>,
    view: &CombatSpectatorView,
    report: &CombatConclusionReport,
) {
    lines.push("[전투 보고서]".to_string());
    // fingerprint를 표시하면 반드시 simulation_version과 같은 줄에 둔다 (인덱스 계약).
    lines.push(format!(
        "시뮬레이션 버전: {} · 지문: {}",
        view.simulation_version.as_str(),
        report.fingerprint
    ));
    lines.push(format!("결과: {}", combat_outcome_label(report.outcome)));
    lines.push(format!("사유: {}", combat_reason_label(report.reason)));
    lines.push(format!("전투 시간: {}ms", report.duration_millis));
    lines.push(format!(
        "생존: {}",
        if report.survivor_ids.is_empty() {
            "없음".to_string()
        } else {
            report.survivor_ids.join(", ")
        }
    ));
    lines.push(format!(
        "전투불능: {}",
        if report.defeated_ids.is_empty() {
            "없음".to_string()
        } else {
            report.defeated_ids.join(", ")
        }
    ));
    // 발생하지 않은 항목은 숨긴다: None이면 줄 자체를 만들지 않는다.
    if let Some(top_dealt) = &report.top_damage_dealt_id {
        lines.push(format!("최대 피해를 가한 전투원: {top_dealt}"));
    }
    if let Some(top_taken) = &report.top_damage_taken_id {
        lines.push(format!("최대 피해를 받은 전투원: {top_taken}"));
    }
    if report.combatants.is_empty() {
        lines.push("- 전투원 상세 기록 없음.".to_string());
    } else {
        for combatant in &report.combatants {
            lines.push(format!(
                "- {}: 가한 피해 {} · 받은 피해 {} · 처치 {} · 전투불능 {}",
                combatant.id,
                round_hundredths_to_int(combatant.damage_dealt_hundredths),
                round_hundredths_to_int(combatant.damage_taken_hundredths),
                combatant.kills,
                if combatant.incapacitated {
                    "예"
                } else {
                    "아니오"
                }
            ));
        }
    }
}

// -- P5: 스냅샷 통합 -------------------------------------------------------------

/// `page.combat`이 `None`이면 아무 줄도 추가하지 않는다 — 호출부에서 이
/// 함수를 부르기 전/후 로직을 바꾸지 않으므로 `None`일 때 스냅샷 출력은 이
/// 함수를 추가하기 전과 바이트 단위로 동일하다 (`tests::combat_section_*` 참고).
fn render_combat_section(lines: &mut Vec<String>, page: &ScenePage) {
    let Some(combat) = &page.combat else {
        return;
    };
    render_combat_board(lines, &combat.view);
    render_combat_core_log(lines, &combat.view);
    if let Some(report) = &combat.report {
        render_combat_report(lines, &combat.view, report);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use escape_core::CombatLogImportance;

    fn test_log_entry(
        template_id: &str,
        actor: &str,
        target: Option<&str>,
        value_hundredths: Option<i64>,
        effect_id: Option<&str>,
    ) -> CombatSpectatorLogEntry {
        CombatSpectatorLogEntry {
            tick: 1,
            sequence: 0,
            template_id: template_id.to_string(),
            importance: CombatLogImportance::Important,
            actor_id: actor.to_string(),
            target_id: target.map(|t| t.to_string()),
            value_hundredths,
            effect_id: effect_id.map(|e| e.to_string()),
        }
    }

    // -- P1: 로그 템플릿 테이블 --------------------------------------------

    #[test]
    fn template_move_intent_renders() {
        let entry = test_log_entry("combat.log.move_intent", "ally_1", None, None, None);
        assert_eq!(combat_log_template_line(&entry), "ally_1 이동 의도");
    }

    #[test]
    fn template_target_selection_renders() {
        let entry = test_log_entry(
            "combat.log.target_selection",
            "ally_1",
            Some("enemy_1"),
            None,
            None,
        );
        assert_eq!(
            combat_log_template_line(&entry),
            "ally_1 → 목표 지정: enemy_1"
        );
    }

    #[test]
    fn template_collision_renders() {
        let entry = test_log_entry(
            "combat.log.collision",
            "ally_1",
            Some("enemy_1"),
            None,
            None,
        );
        assert_eq!(combat_log_template_line(&entry), "ally_1 × enemy_1 충돌");
    }

    #[test]
    fn template_damage_applied_rounds_hundredths() {
        let entry = test_log_entry(
            "combat.log.damage_applied",
            "ally_1",
            Some("enemy_1"),
            Some(1050),
            None,
        );
        assert_eq!(combat_log_template_line(&entry), "ally_1 → enemy_1 피해 11");
    }

    #[test]
    fn template_effect_applied_shows_effect_id() {
        let entry = test_log_entry(
            "combat.log.effect_applied",
            "ally_1",
            Some("enemy_1"),
            None,
            Some("burn"),
        );
        assert_eq!(
            combat_log_template_line(&entry),
            "ally_1 → enemy_1 효과 적용 [burn]"
        );
    }

    #[test]
    fn template_effect_applied_hidden_masks_effect_id() {
        let entry = test_log_entry(
            "combat.log.effect_applied_hidden",
            "ally_1",
            Some("enemy_1"),
            None,
            None,
        );
        let line = combat_log_template_line(&entry);
        assert_eq!(line, "ally_1 → enemy_1 효과 적용 [정체불명]");
        assert!(!line.contains("burn"));
    }

    #[test]
    fn template_unknown_id_falls_back_and_exposes_id() {
        let entry = test_log_entry(
            "combat.log.made_up_event",
            "ally_1",
            Some("enemy_1"),
            None,
            None,
        );
        let line = combat_log_template_line(&entry);
        assert!(line.contains("combat.log.made_up_event"));
        assert!(line.contains("알 수 없는 사건"));
    }

    #[test]
    fn round_hundredths_rounds_half_up_both_signs() {
        assert_eq!(round_hundredths_to_int(1050), 11);
        assert_eq!(round_hundredths_to_int(1049), 10);
        assert_eq!(round_hundredths_to_int(1000), 10);
        assert_eq!(round_hundredths_to_int(-1050), -11);
    }

    // -- P2: 체스말 보드 ----------------------------------------------------
    use escape_core::{
        CombatFacing, CombatPosition, CombatSimulationVersion, CombatSpectatorFrame,
    };

    fn test_piece(
        id: &str,
        side: CombatSide,
        x: i32,
        y: i32,
        active: bool,
        cues: Vec<CombatSpectatorCue>,
    ) -> CombatSpectatorPiece {
        CombatSpectatorPiece {
            id: id.to_string(),
            side,
            position: CombatPosition { x, y },
            facing: CombatFacing { x: 1, y: 0 },
            active,
            cues,
        }
    }

    fn test_view(
        frames: Vec<CombatSpectatorFrame>,
        core_log: Vec<CombatSpectatorLogEntry>,
        full_log: Vec<CombatSpectatorLogEntry>,
    ) -> CombatSpectatorView {
        CombatSpectatorView {
            simulation_version: CombatSimulationVersion::new("v-test").expect("valid version"),
            resolution_fingerprint: "res-fp".to_string(),
            tick_millis: 100,
            frames,
            core_log,
            full_log,
            fingerprint: "view-fp".to_string(),
        }
    }

    #[test]
    fn board_renders_last_frame_with_tick_and_elapsed_time() {
        let frames = vec![
            CombatSpectatorFrame {
                tick: 0,
                pieces: vec![test_piece(
                    "ally_1",
                    CombatSide::Ally,
                    0,
                    0,
                    true,
                    Vec::new(),
                )],
            },
            CombatSpectatorFrame {
                tick: 3,
                pieces: vec![
                    test_piece("ally_1", CombatSide::Ally, 0, 0, true, Vec::new()),
                    test_piece("enemy_1", CombatSide::Enemy, 2, 1, true, Vec::new()),
                ],
            },
        ];
        let view = test_view(frames, Vec::new(), Vec::new());
        let mut lines = Vec::new();
        render_combat_board(&mut lines, &view);
        let text = lines.join("\n");
        assert!(text.contains("tick 3"));
        assert!(text.contains("경과 300ms"));
        assert!(text.contains(COMBAT_LEGEND_LINE));
        assert!(!text.contains("좌표 목록"));
    }

    #[test]
    fn board_exceeding_caps_falls_back_to_coordinate_list() {
        let frames = vec![CombatSpectatorFrame {
            tick: 1,
            pieces: vec![
                test_piece("ally_1", CombatSide::Ally, 0, 0, true, Vec::new()),
                test_piece("enemy_1", CombatSide::Enemy, 40, 0, true, Vec::new()),
            ],
        }];
        let view = test_view(frames, Vec::new(), Vec::new());
        let mut lines = Vec::new();
        render_combat_board(&mut lines, &view);
        let text = lines.join("\n");
        assert!(text.contains("좌표 목록으로 대체"));
        assert!(text.contains("ally_1 @ (0, 0)"));
        assert!(text.contains("enemy_1 @ (40, 0)"));
    }

    #[test]
    fn board_shows_all_five_cue_symbols() {
        let all_cues = vec![
            CombatSpectatorCue::Attack,
            CombatSpectatorCue::Hit,
            CombatSpectatorCue::Evade,
            CombatSpectatorCue::BalanceBroken,
            CombatSpectatorCue::Incapacitated,
        ];
        let piece = test_piece("ally_1", CombatSide::Ally, 0, 0, true, all_cues);
        let token = combat_piece_token(&piece);
        assert_eq!(token, "A><~!x");
    }

    #[test]
    fn board_handles_empty_pieces_without_panic() {
        let frames = vec![CombatSpectatorFrame {
            tick: 5,
            pieces: Vec::new(),
        }];
        let view = test_view(frames, Vec::new(), Vec::new());
        let mut lines = Vec::new();
        render_combat_board(&mut lines, &view);
        let text = lines.join("\n");
        assert!(text.contains("말이 없다"));
    }

    #[test]
    fn board_handles_no_frames_without_panic() {
        let view = test_view(Vec::new(), Vec::new(), Vec::new());
        let mut lines = Vec::new();
        render_combat_board(&mut lines, &view);
        let text = lines.join("\n");
        assert!(text.contains("프레임이 없다"));
    }

    // -- P3: 핵심 로그 --------------------------------------------------------

    fn test_log_entry_seq(sequence: u32) -> CombatSpectatorLogEntry {
        CombatSpectatorLogEntry {
            tick: 1,
            sequence,
            template_id: "combat.log.move_intent".to_string(),
            importance: CombatLogImportance::Important,
            actor_id: "ally_1".to_string(),
            target_id: None,
            value_hundredths: None,
            effect_id: None,
        }
    }

    #[test]
    fn core_log_shows_full_log_count_only() {
        let full_log: Vec<_> = (0..7).map(test_log_entry_seq).collect();
        let view = test_view(Vec::new(), Vec::new(), full_log);
        let mut lines = Vec::new();
        render_combat_core_log(&mut lines, &view);
        let text = lines.join("\n");
        assert!(text.contains("전체 로그 7건"));
        assert!(text.contains("핵심 로그가 없다"));
    }

    #[test]
    fn core_log_truncates_and_states_omitted_count() {
        let core_log: Vec<_> = (0..25).map(test_log_entry_seq).collect();
        let view = test_view(Vec::new(), core_log, Vec::new());
        let mut lines = Vec::new();
        render_combat_core_log(&mut lines, &view);
        let shown = lines
            .iter()
            .filter(|line| line.starts_with("- ally_1"))
            .count();
        assert_eq!(shown, COMBAT_CORE_LOG_LIMIT);
        assert!(lines.iter().any(|line| line.contains("생략 5줄")));
    }

    // -- P4: 보고서 -----------------------------------------------------------
    use escape_core::CombatCombatantReport;

    fn base_report() -> CombatConclusionReport {
        CombatConclusionReport {
            resolution_fingerprint: "res-fp".to_string(),
            outcome: CombatConclusionOutcome::AllyVictory,
            reason: CombatConclusionReason::AllEnemiesDefeated,
            decisive_tick: Some(3),
            active_allies: 1,
            active_enemies: 0,
            survivor_ids: vec!["ally_1".to_string()],
            defeated_ids: vec!["enemy_1".to_string()],
            removed_combat_effect_ids: Vec::new(),
            retained_effect_ids: Vec::new(),
            duration_millis: 300,
            combatants: vec![CombatCombatantReport {
                id: "ally_1".to_string(),
                damage_dealt_hundredths: 1050,
                damage_taken_hundredths: 200,
                kills: 1,
                incapacitated: false,
            }],
            top_damage_dealt_id: Some("ally_1".to_string()),
            top_damage_taken_id: Some("enemy_1".to_string()),
            fingerprint: "report-fp".to_string(),
        }
    }

    #[test]
    fn report_hides_highlight_lines_when_none() {
        let mut report = base_report();
        report.top_damage_dealt_id = None;
        report.top_damage_taken_id = None;
        let view = test_view(Vec::new(), Vec::new(), Vec::new());
        let mut lines = Vec::new();
        render_combat_report(&mut lines, &view, &report);
        let text = lines.join("\n");
        assert!(!text.contains("최대 피해를 가한"));
        assert!(!text.contains("최대 피해를 받은"));
    }

    #[test]
    fn report_shows_highlight_lines_when_some() {
        let report = base_report();
        let view = test_view(Vec::new(), Vec::new(), Vec::new());
        let mut lines = Vec::new();
        render_combat_report(&mut lines, &view, &report);
        let text = lines.join("\n");
        assert!(text.contains("최대 피해를 가한 전투원: ally_1"));
        assert!(text.contains("최대 피해를 받은 전투원: enemy_1"));
    }

    #[test]
    fn report_lists_one_row_per_combatant() {
        let mut report = base_report();
        report.combatants.push(CombatCombatantReport {
            id: "ally_2".to_string(),
            damage_dealt_hundredths: 0,
            damage_taken_hundredths: 500,
            kills: 0,
            incapacitated: true,
        });
        let view = test_view(Vec::new(), Vec::new(), Vec::new());
        let mut lines = Vec::new();
        render_combat_report(&mut lines, &view, &report);
        let rows = lines
            .iter()
            .filter(|line| line.starts_with("- ally") || line.starts_with("- enemy"))
            .count();
        assert_eq!(rows, report.combatants.len());
    }

    #[test]
    fn report_contains_no_forbidden_phrases() {
        let report = base_report();
        let view = test_view(Vec::new(), Vec::new(), Vec::new());
        let mut lines = Vec::new();
        render_combat_report(&mut lines, &view, &report);
        let text = lines.join("\n");
        for forbidden in [
            "MVP",
            "전략 평가",
            "전환점",
            "원인 분석",
            "전략 조언",
            "이전 전투",
        ] {
            assert!(
                !text.contains(forbidden),
                "forbidden phrase leaked: {forbidden}"
            );
        }
    }

    #[test]
    fn report_fingerprint_shares_line_with_simulation_version() {
        let report = base_report();
        let view = test_view(Vec::new(), Vec::new(), Vec::new());
        let mut lines = Vec::new();
        render_combat_report(&mut lines, &view, &report);
        let version_line = lines
            .iter()
            .find(|line| line.contains("시뮬레이션 버전"))
            .expect("simulation version line present");
        assert!(version_line.contains(&report.fingerprint));
    }

    // -- P5: 스냅샷 통합 -------------------------------------------------------
    use escape_core::{
        AchievementSummary, CombatSpectatorPage, InventorySummary, SceneLocation, SceneVisual,
        StatusSummary,
    };

    fn minimal_scene_page(combat: Option<CombatSpectatorPage>) -> ScenePage {
        ScenePage {
            mode: SceneMode::Movement,
            title: "테스트 장면".to_string(),
            location: SceneLocation {
                id: "loc_1".to_string(),
                name: "테스트 위치".to_string(),
                description: String::new(),
            },
            chapter_label: "1장".to_string(),
            status_summary: StatusSummary {
                turn: 1,
                danger: 0,
                resources: Vec::new(),
                warnings: Vec::new(),
            },
            body_blocks: Vec::new(),
            content_stream: Vec::new(),
            dialogue_entries: Vec::new(),
            visual: SceneVisual {
                id: "visual_1".to_string(),
                kind: "none".to_string(),
                alt: String::new(),
                source_id: None,
            },
            actions: Vec::new(),
            blocked_actions: Vec::new(),
            history_entries: Vec::new(),
            inventory_summary: InventorySummary {
                items: Vec::new(),
                overflow_count: 0,
            },
            inventory_details: Vec::new(),
            achievement_summary: AchievementSummary {
                unlocked: Vec::new(),
                newly_unlocked: Vec::new(),
            },
            pressure_cues: Vec::new(),
            effect_cues: Vec::new(),
            character_summary: None,
            progression: None,
            content_labels: None,
            check_result: None,
            insights: Vec::new(),
            skills: Vec::new(),
            titles: Vec::new(),
            combat,
        }
    }

    #[test]
    fn combat_section_adds_nothing_when_combat_is_none() {
        let mut lines = Vec::new();
        render_combat_section(&mut lines, &minimal_scene_page(None));
        assert!(lines.is_empty());
    }

    #[test]
    fn scene_snapshot_unchanged_bytes_when_combat_is_none() {
        let page = minimal_scene_page(None);
        let with_no_combat_path = render_scene_page_snapshot(&page, &[]);
        assert!(!with_no_combat_path.contains("[전투 판]"));
        assert!(!with_no_combat_path.contains("[전투 로그]"));
        assert!(!with_no_combat_path.contains("[전투 보고서]"));
        // Determinism: same input renders identical bytes every time.
        assert_eq!(with_no_combat_path, render_scene_page_snapshot(&page, &[]));
    }

    #[test]
    fn scene_snapshot_includes_combat_sections_in_order_when_present() {
        let combat = CombatSpectatorPage {
            view: test_view(
                vec![CombatSpectatorFrame {
                    tick: 1,
                    pieces: vec![test_piece(
                        "ally_1",
                        CombatSide::Ally,
                        0,
                        0,
                        true,
                        Vec::new(),
                    )],
                }],
                vec![test_log_entry(
                    "combat.log.move_intent",
                    "ally_1",
                    None,
                    None,
                    None,
                )],
                Vec::new(),
            ),
            report: Some(base_report()),
        };
        let page = minimal_scene_page(Some(combat));
        let output = render_scene_page_snapshot(&page, &[]);
        let board_pos = output.find("[전투 판]").expect("board section present");
        let log_pos = output
            .find("[전투 로그]")
            .expect("core log section present");
        let report_pos = output
            .find("[전투 보고서]")
            .expect("report section present");
        let recent_log_pos = output
            .find("[최근 로그]")
            .expect("recent log section present");
        assert!(board_pos < log_pos);
        assert!(log_pos < report_pos);
        assert!(report_pos < recent_log_pos);
    }

    #[test]
    fn scene_snapshot_omits_report_section_when_combat_in_progress() {
        let combat = CombatSpectatorPage {
            view: test_view(
                vec![CombatSpectatorFrame {
                    tick: 1,
                    pieces: vec![test_piece(
                        "ally_1",
                        CombatSide::Ally,
                        0,
                        0,
                        true,
                        Vec::new(),
                    )],
                }],
                Vec::new(),
                Vec::new(),
            ),
            report: None,
        };
        let page = minimal_scene_page(Some(combat));
        let output = render_scene_page_snapshot(&page, &[]);
        assert!(output.contains("[전투 판]"));
        assert!(!output.contains("[전투 보고서]"));
    }
}

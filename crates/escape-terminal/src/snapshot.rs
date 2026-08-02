use super::*;
use escape_core::CombatSpectatorLogEntry;

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
}

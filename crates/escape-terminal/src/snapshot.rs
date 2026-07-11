use super::*;

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

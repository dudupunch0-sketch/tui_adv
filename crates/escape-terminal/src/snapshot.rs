use super::*;

pub(crate) fn render_scene_page_app_frame(page: &ScenePage, logs: &[String], tick: u64) -> String {
    let mut backend = slt::TestBackend::new(120, 40);
    backend.render(|ui| render_scene_page_app(ui, page, logs, tick));
    backend.to_string_trimmed()
}
pub(crate) fn render_scene_page_app(
    ui: &mut slt::Context,
    page: &ScenePage,
    logs: &[String],
    tick: u64,
) {
    let _ = ui.col(|ui| {
        ui.text(scene_page_terminal_title(page));
        ui.text("app loop: full-screen SuperLightTUI frame");
        ui.text(format!("tick: {tick}"));
        ui.text(format!(
            "{} · {} · {} ({})",
            page.chapter_label,
            scene_mode_label(&page.mode),
            page.location.name,
            page.location.id
        ));
        ui.text(format!(
            "진단: 체력 {} · 정신력 {} · 배터리 {} · 허기 {} · 갈증 {} · 위험도 {}",
            resource_value(page, "health"),
            resource_value(page, "sanity"),
            resource_value(page, "battery"),
            resource_value(page, "hunger"),
            resource_value(page, "thirst"),
            page.status_summary.danger
        ));

        ui.text("[STORY PAGE]");
        ui.text(format!("visual: {} / {}", page.visual.id, page.visual.kind));
        ui.text(format!("alt: {}", page.visual.alt));
        ui.container().w(110).h(7).draw_with(
            RawGlyphFxFrame {
                tick,
                effect_cues: page.effect_cues.clone(),
            },
            draw_raw_glyphfx,
        );

        if matches!(page.mode, SceneMode::Encounter) {
            ui.text("[현재 인카운터]");
        } else {
            ui.text("[현재 행동]");
        }
        ui.text(page.title.as_str());
        render_scene_body(ui, page);

        ui.text("[선택지]");
        for (index, action) in page.actions.iter().enumerate() {
            ui.text(scene_action_line(index + 1, action));
        }
        if !page.blocked_actions.is_empty() {
            ui.text("[잠긴 선택지]");
            for action in &page.blocked_actions {
                ui.text(scene_blocked_action_line(action));
                ui.text(format!("   이유: {}", action.reasons.join(", ")));
            }
        }
        ui.text(app_input_hint_for_scene_actions(&page.actions));

        ui.text("[최근 로그]");
        if logs.is_empty() {
            ui.text("- 아직 기록된 로그가 없다.");
        } else {
            for log in logs.iter().rev().take(5).rev() {
                ui.text(format!("- {log}"));
            }
        }
    });
}
pub(crate) fn app_input_hint_for_scene_actions(actions: &[SceneAction]) -> String {
    format!(
        "입력: 번호 {} · q 종료 · ? 도움말",
        scene_action_number_range(actions)
    )
}
pub(crate) fn render_scene_page_snapshot(page: &ScenePage, logs: &[String]) -> String {
    let mut backend = slt::TestBackend::new(120, 36);
    backend.render(|ui| render_scene_page(ui, page, logs));
    backend.to_string_trimmed()
}
pub(crate) fn render_scene_page(ui: &mut slt::Context, page: &ScenePage, logs: &[String]) {
    let _ = ui.col(|ui| {
        ui.text(scene_page_terminal_title(page));
        ui.text(format!(
            "{} · {}",
            page.chapter_label,
            scene_mode_label(&page.mode)
        ));
        ui.text("[상태]");
        ui.text(format!("턴: {}", page.status_summary.turn));
        ui.text(format!(
            "위치: {} ({})",
            page.location.name, page.location.id
        ));
        ui.text(format!(
            "체력: {}  정신력: {}  배터리: {}  허기: {}  갈증: {}  위험도: {}",
            resource_value(page, "health"),
            resource_value(page, "sanity"),
            resource_value(page, "battery"),
            resource_value(page, "hunger"),
            resource_value(page, "thirst"),
            page.status_summary.danger
        ));
        for warning in &page.status_summary.warnings {
            ui.text(format!("! {warning}"));
        }

        ui.text("[비주얼]");
        for line in scene_visual_card_lines(page) {
            ui.text(line);
        }

        if matches!(page.mode, SceneMode::Encounter) {
            ui.text("[현재 인카운터]");
            ui.text(page.title.as_str());
            render_scene_body(ui, page);
        }

        ui.text("[현재 행동]");
        if !matches!(page.mode, SceneMode::Encounter) {
            ui.text(page.title.as_str());
            render_scene_body(ui, page);
        }
        for (index, action) in page.actions.iter().enumerate() {
            ui.text(scene_action_line(index + 1, action));
        }
        if !page.blocked_actions.is_empty() {
            ui.text("[잠긴 선택지]");
            for action in &page.blocked_actions {
                ui.text(scene_blocked_action_line(action));
                ui.text(format!("   이유: {}", action.reasons.join(", ")));
            }
        }

        ui.text("[최근 로그]");
        if logs.is_empty() {
            ui.text("- 아직 기록된 로그가 없다.");
        } else {
            for log in logs {
                ui.text(format!("- {log}"));
            }
        }
    });
}
pub(crate) fn render_scene_body(ui: &mut slt::Context, page: &ScenePage) {
    for entry in &page.dialogue_entries {
        ui.text(format!("{}: {}", entry.speaker, entry.text));
    }
    for block in &page.body_blocks {
        if let Some(heading) = scene_body_block_heading(block) {
            ui.text(heading);
        }
        let lines = compact_terminal_body_block_lines(block);
        for line in lines {
            for wrapped in wrap_terminal_body_line(line, 76) {
                ui.text(wrapped);
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
        "이구학지 - 천기록 // SuperLightTUI STORYBOOK"
    } else {
        "ESCAPE OFFICE // SuperLightTUI HORROR EDITION"
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
    let mut backend = slt::TestBackend::new(120, 32);
    backend.render(|ui| {
        let _ = ui.col(|ui| {
            ui.text("ESCAPE OFFICE // SuperLightTUI HORROR EDITION");
            ui.text("legacy printer scene · TurnView bridge");
            ui.text("[상태]");
            ui.text(format!("턴: {}", state.turn));
            ui.text(format!("위치: {location_name} ({})", state.location_id));
            ui.text(format!(
                "체력: {}  정신력: {}  배터리: {}  위험도: {}",
                state.player.health, state.player.sanity, state.player.battery, state.danger
            ));
            ui.text("[비주얼]");
            ui.text(glyphfx_turn_line(&view.effect_cues));

            if view.encounter_id.is_some() {
                ui.text("[현재 인카운터]");
                ui.text(view.title.as_str());
                ui.text(view.body.as_str());
            }

            ui.text("[현재 행동]");
            if view.encounter_id.is_none() {
                ui.text(view.title.as_str());
                ui.text(view.body.as_str());
            }
            for (index, action) in view.actions.iter().enumerate() {
                ui.text(turn_action_line(index + 1, action));
            }
            if !view.blocked_actions.is_empty() {
                ui.text("[잠긴 선택지]");
                for action in &view.blocked_actions {
                    ui.text(turn_blocked_action_line(action));
                    ui.text(format!("   이유: {}", action.reasons.join(", ")));
                }
            }

            ui.text("[최근 로그]");
            if logs.is_empty() {
                ui.text("- 아직 기록된 로그가 없다.");
            } else {
                for log in logs {
                    ui.text(format!("- {log}"));
                }
            }
        });
    });
    backend.to_string_trimmed()
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

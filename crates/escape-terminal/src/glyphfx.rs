use super::*;

#[derive(Clone)]
pub(crate) struct RawGlyphFxFrame {
    pub(crate) tick: u64,
    pub(crate) effect_cues: Vec<SceneEffectCue>,
}
pub(crate) fn draw_raw_glyphfx(buf: &mut slt::Buffer, rect: slt::Rect, frame: &RawGlyphFxFrame) {
    let lines = raw_glyphfx_lines(frame);
    for (index, line) in lines.iter().enumerate() {
        if index >= rect.height as usize {
            break;
        }
        buf.set_string(rect.x, rect.y + index as u32, line, slt::Style::new());
    }
}
pub(crate) fn raw_glyphfx_lines(frame: &RawGlyphFxFrame) -> Vec<String> {
    let mut lines = vec![
        "[RAW-DRAW GLYPHFX LAYER]".to_string(),
        format!(
            "raw-draw glyphfx tick={} {}",
            frame.tick,
            glyphfx_tick_wave(frame.tick)
        ),
    ];

    if frame.effect_cues.is_empty() {
        lines.push("raw-draw glyphfx idle · no EffectCue".to_string());
        return lines;
    }

    for cue in &frame.effect_cues {
        lines.push(format!(
            "cue: {} source={} intensity={} distortion={}",
            cue.kind, cue.source, cue.intensity, cue.distortion
        ));
        if !cue.stable_terms.is_empty() {
            lines.push(format!("stable terms: {}", cue.stable_terms.join(" / ")));
        }
        if let Some(fallback) = &cue.fallback_text {
            lines.push(format!("fallback: {fallback}"));
        }
    }
    lines
}
pub(crate) fn glyphfx_tick_wave(tick: u64) -> String {
    const CELLS: [char; 5] = ['·', '░', '▒', '▓', '▒'];
    (0..24)
        .map(|offset| CELLS[((tick as usize) + offset) % CELLS.len()])
        .collect()
}
pub(crate) fn glyphfx_card_lines(effect_cues: &[SceneEffectCue]) -> Vec<String> {
    if effect_cues.is_empty() {
        return vec!["│ glyphfx signal: idle · terminal-native fallback".to_string()];
    }

    let mut lines = Vec::new();
    for cue in effect_cues {
        let percent = glyphfx_intensity_percent(cue.intensity);
        lines.push(format!(
            "│ glyphfx signal: {} [{}] {}% {}",
            cue.kind,
            glyphfx_meter(percent),
            percent,
            cue.distortion
        ));
        if !cue.stable_terms.is_empty() {
            lines.push(format!("│ stable terms: {}", cue.stable_terms.join(" / ")));
        }
        if let Some(fallback) = &cue.fallback_text {
            lines.push(format!("│ fallback: {fallback}"));
        }
    }
    lines
}
pub(crate) fn glyphfx_intensity_percent(intensity: f32) -> u32 {
    (intensity.clamp(0.0, 1.0) * 100.0).round() as u32
}
pub(crate) fn glyphfx_meter(percent: u32) -> String {
    let filled = (percent / 10).min(10) as usize;
    format!("{}{}", "#".repeat(filled), "-".repeat(10 - filled))
}
pub(crate) fn glyphfx_turn_line(effect_cues: &[EffectCue]) -> String {
    if effect_cues.is_empty() {
        return "GlyphFX: terminal-native fallback idle".to_string();
    }
    let cues = effect_cues
        .iter()
        .map(|cue| match cue {
            EffectCue::GlyphAnomaly(details) => format!(
                "{}:{} {}",
                cue.kind_label(),
                details.intensity,
                details.distortion
            ),
        })
        .collect::<Vec<_>>()
        .join(" | ");
    format!("GlyphFX: {cues}")
}

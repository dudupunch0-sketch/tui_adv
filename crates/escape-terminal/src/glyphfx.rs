use super::*;

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EffectCue {
    GlyphAnomaly(GlyphAnomalyCue),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GlyphAnomalyCue {
    pub source: String,
    pub intensity: u8,
    pub stable_terms: Vec<String>,
    pub distortion: String,
}

impl EffectCue {
    pub fn kind_label(&self) -> &'static str {
        match self {
            EffectCue::GlyphAnomaly(_) => "EffectCue::GlyphAnomaly",
        }
    }
}

pub fn printer_glyph_anomaly_cue() -> EffectCue {
    EffectCue::GlyphAnomaly(GlyphAnomalyCue {
        source: "copier_output".to_string(),
        intensity: 72,
        stable_terms: vec![
            "비상계단".to_string(),
            "토너".to_string(),
            "접힌 방향".to_string(),
        ],
        distortion: "reflow_then_stabilize".to_string(),
    })
}

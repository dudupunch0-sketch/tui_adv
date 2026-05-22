use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EffectCue {
    GlyphAnomaly(GlyphAnomalyCue),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
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

impl Serialize for EffectCue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            EffectCue::GlyphAnomaly(details) => {
                let mut state = serializer.serialize_struct("EffectCue", 7)?;
                state.serialize_field("kind", "glyph_anomaly")?;
                state.serialize_field("source", &details.source)?;
                state.serialize_field("intensity", &(f64::from(details.intensity) / 100.0))?;
                state.serialize_field("stable_terms", &details.stable_terms)?;
                state.serialize_field("distortion", &details.distortion)?;
                state.serialize_field("duration_hint_ms", &Option::<u32>::None)?;
                state.serialize_field("fallback_text", &Option::<String>::None)?;
                state.end()
            }
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

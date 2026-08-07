use serde::{Deserialize, Serialize};
use std::fmt;

/// The one `simulation_version` this build's judgement actually implements.
///
/// 정본 03: determinism is only promised *within* a simulation version. A
/// single binary implementing two versions' worth of judgement at once is
/// out of scope (T0 §4-1) — so there is exactly one current value, not a
/// supported-list. Bumping it is a breaking change owned by whichever slice
/// changes the judgement (e.g. T1's hex coordinate swap), never this one.
///
/// v2 -> v3 (`fable_combat_hex_t1b1_step1_2608071921.md` §4-4): `{x,y}`
/// euclidean `CombatPosition`/`CombatFacing` became `{q,r}` axial `HexCoord`,
/// changing both the wire representation and the meaning of `attack_range`/
/// `support_range`/`speed_per_tick` (hex distance and tile count, not
/// euclidean units). T0's enforcement (index-time + runtime) is what makes
/// this bump safe to make here.
pub const CURRENT_SIMULATION_VERSION: &str = "v3";

/// Rejects `version` unless it equals [`CURRENT_SIMULATION_VERSION`].
///
/// Callers decide *where* this applies (simulation entry, index-time
/// authoring) — this only decides *whether* a given value is acceptable.
/// Deliberately not called from [`CombatSimulationVersion::new`] or its
/// `Deserialize` impl: archived records carrying an old version string must
/// keep deserializing without error (T0 §4-2).
pub fn ensure_supported_simulation_version(
    version: &CombatSimulationVersion,
) -> Result<(), CombatContractError> {
    if version.as_str() == CURRENT_SIMULATION_VERSION {
        Ok(())
    } else {
        Err(CombatContractError::UnsupportedSimulationVersion(
            version.as_str().to_string(),
        ))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CombatSimulationVersion(String);

impl CombatSimulationVersion {
    pub fn new(value: impl Into<String>) -> Result<Self, CombatContractError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(CombatContractError::EmptySimulationVersion);
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl fmt::Display for CombatSimulationVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatRngNamespace {
    StoryResolution,
    EncounterComposition,
    ActualCombat,
    ForecastEnsemble,
    CosmeticPresentation,
}
impl CombatRngNamespace {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::StoryResolution => "story_resolution",
            Self::EncounterComposition => "encounter_composition",
            Self::ActualCombat => "actual_combat",
            Self::ForecastEnsemble => "forecast_ensemble",
            Self::CosmeticPresentation => "cosmetic_presentation",
        }
    }
    pub fn derive_seed(self, base_seed: u64, version: &CombatSimulationVersion) -> u64 {
        fnv1a_64(format!("{}:{}:{}", version, self.as_str(), base_seed).as_bytes())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatEffectRef {
    pub id: String,
    pub reason: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuppressedCombatEffect {
    pub id: String,
    pub reason: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatManifest {
    pub simulation_version: CombatSimulationVersion,
    pub actual_seed: u64,
    pub world_state_fingerprint: String,
    pub applied_effects: Vec<CombatEffectRef>,
    pub suppressed_effects: Vec<SuppressedCombatEffect>,
    pub combatant_ids: Vec<String>,
    pub placement_ids: Vec<String>,
    pub environment_ids: Vec<String>,
    #[serde(default)]
    pub team_ids: Vec<String>,
    pub rule_ids: Vec<String>,
    pub public_info_ids: Vec<String>,
}
impl CombatManifest {
    pub fn validate(&self) -> Result<(), CombatContractError> {
        if self.simulation_version.as_str().trim().is_empty() {
            return Err(CombatContractError::EmptySimulationVersion);
        }
        if self.world_state_fingerprint.trim().is_empty() {
            return Err(CombatContractError::EmptyWorldStateFingerprint);
        }
        for effect in &self.applied_effects {
            ensure_id(&effect.id)?;
            if effect.reason.trim().is_empty() {
                return Err(CombatContractError::EmptyEffectReason(effect.id.clone()));
            }
        }
        for effect in &self.suppressed_effects {
            ensure_id(&effect.id)?;
            if effect.reason.trim().is_empty() {
                return Err(CombatContractError::EmptySuppressionReason(
                    effect.id.clone(),
                ));
            }
        }
        for id in self
            .combatant_ids
            .iter()
            .chain(&self.placement_ids)
            .chain(&self.environment_ids)
            .chain(&self.team_ids)
            .chain(&self.rule_ids)
            .chain(&self.public_info_ids)
        {
            ensure_id(id)?;
        }
        Ok(())
    }
    pub fn canonical_json(&self) -> Result<String, CombatContractError> {
        self.validate()?;
        let mut canonical = self.clone();
        canonical
            .applied_effects
            .sort_by(|a, b| a.id.cmp(&b.id).then(a.reason.cmp(&b.reason)));
        canonical
            .suppressed_effects
            .sort_by(|a, b| a.id.cmp(&b.id).then(a.reason.cmp(&b.reason)));
        for ids in [
            &mut canonical.combatant_ids,
            &mut canonical.placement_ids,
            &mut canonical.environment_ids,
            &mut canonical.team_ids,
            &mut canonical.rule_ids,
            &mut canonical.public_info_ids,
        ] {
            ids.sort();
        }
        serde_json::to_string(&canonical)
            .map_err(|e| CombatContractError::Serialization(e.to_string()))
    }
    pub fn fingerprint(&self) -> Result<String, CombatContractError> {
        Ok(format!(
            "{:016x}",
            fnv1a_64(self.canonical_json()?.as_bytes())
        ))
    }
    pub fn derived_seed(&self, namespace: CombatRngNamespace) -> Result<u64, CombatContractError> {
        self.validate()?;
        let fingerprint = self.fingerprint()?;
        Ok(fnv1a_64(
            format!(
                "{}:{}:{}:{}",
                self.simulation_version,
                namespace.as_str(),
                self.actual_seed,
                fingerprint
            )
            .as_bytes(),
        ))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CombatContractError {
    EmptySimulationVersion,
    EmptyWorldStateFingerprint,
    EmptyStableId,
    EmptySuppressionReason(String),
    EmptyEffectReason(String),
    Serialization(String),
    /// A `simulation_version` other than [`CURRENT_SIMULATION_VERSION`] was
    /// used to attempt judgement (not merely to deserialize a past record).
    /// Carries the received value; the expected value is
    /// [`CURRENT_SIMULATION_VERSION`] and is added by `Display`.
    UnsupportedSimulationVersion(String),
}
impl fmt::Display for CombatContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySimulationVersion => write!(f, "simulation version must not be empty"),
            Self::EmptyWorldStateFingerprint => {
                write!(f, "world state fingerprint must not be empty")
            }
            Self::EmptyStableId => write!(f, "stable combat ID must not be empty"),
            Self::EmptySuppressionReason(id) => {
                write!(f, "suppressed effect '{id}' requires a reason")
            }
            Self::EmptyEffectReason(id) => write!(f, "applied effect '{id}' requires a reason"),
            Self::Serialization(msg) => write!(f, "combat contract serialization failed: {msg}"),
            Self::UnsupportedSimulationVersion(received) => write!(
                f,
                "unsupported simulation version '{received}' (this build implements '{CURRENT_SIMULATION_VERSION}')"
            ),
        }
    }
}
impl std::error::Error for CombatContractError {}
fn ensure_id(id: &str) -> Result<(), CombatContractError> {
    if id.trim().is_empty() {
        Err(CombatContractError::EmptyStableId)
    } else {
        Ok(())
    }
}
fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

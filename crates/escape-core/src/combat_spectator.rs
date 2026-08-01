use crate::{
    CombatEffectCatalog, CombatFacing, CombatLogImportance, CombatPosition, CombatResolutionResult,
    CombatSide, CombatSimulationParticipant,
};
use serde::{Deserialize, Serialize};

/// 정본 13의 "공용 연출 문법". renderer가 이 종류만 보고 표현을 고른다.
/// 값은 판정 결과에서 파생되며, 이 enum이 애니메이션·CSS·색을 지정하지 않는다.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatSpectatorCue {
    Attack,
    Hit,
    Evade,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatSpectatorPiece {
    pub id: String,
    pub side: CombatSide,
    pub position: CombatPosition,
    pub facing: CombatFacing,
    pub active: bool,
    #[serde(default)]
    pub cues: Vec<CombatSpectatorCue>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatSpectatorFrame {
    pub tick: u32,
    #[serde(default)]
    pub pieces: Vec<CombatSpectatorPiece>,
}

/// 등록된 사건 태그 -> 템플릿 id. 문장은 renderer가 소유한다.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatSpectatorLogEntry {
    pub tick: u32,
    pub sequence: u32,
    pub template_id: String,
    pub importance: CombatLogImportance,
    pub actor_id: String,
    #[serde(default)]
    pub target_id: Option<String>,
    #[serde(default)]
    pub value_hundredths: Option<i64>,
    #[serde(default)]
    pub effect_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatSpectatorView {
    pub resolution_fingerprint: String,
    pub tick_millis: u32,
    #[serde(default)]
    pub frames: Vec<CombatSpectatorFrame>,
    #[serde(default)]
    pub core_log: Vec<CombatSpectatorLogEntry>,
    #[serde(default)]
    pub full_log: Vec<CombatSpectatorLogEntry>,
    pub fingerprint: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatSpectatorRequest {
    pub resolution: CombatResolutionResult,
    #[serde(default)]
    pub participants: Vec<CombatSimulationParticipant>,
    pub catalog: CombatEffectCatalog,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CombatSpectatorError {
    UnknownParticipant(String),
}
impl std::fmt::Display for CombatSpectatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for CombatSpectatorError {}

pub fn spectate(
    request: &CombatSpectatorRequest,
) -> Result<CombatSpectatorView, CombatSpectatorError> {
    let mut view = CombatSpectatorView {
        resolution_fingerprint: request.resolution.fingerprint.clone(),
        // NOTE(wave3-step1a WP-1): `CombatResolutionResult`/`CombatExecutionResult`에는
        // tick 길이(ms)를 담는 필드가 없다. 플랜의 파생 규칙에도 명시되어 있지 않다.
        // 렌더러 소비는 Step 1c/1d 소관이라 이 slice에서는 0으로 둔다 (보고서 참고).
        tick_millis: 0,
        frames: Vec::new(),
        core_log: Vec::new(),
        full_log: Vec::new(),
        fingerprint: String::new(),
    };
    view.fingerprint = fingerprint(&view);
    Ok(view)
}

fn fingerprint<T: Serialize>(value: &T) -> String {
    format!(
        "{:016x}",
        fnv(&serde_json::to_vec(value).unwrap_or_default())
    )
}
fn fnv(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

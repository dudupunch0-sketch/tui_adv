use crate::{
    CombatAttackOutcome, CombatEffectCatalog, CombatEffectDefinition, CombatFacing, CombatLogEvent,
    CombatLogImportance, CombatLogTag, CombatPosition, CombatResolutionCombatant,
    CombatResolutionLogEvent, CombatResolutionLogTag, CombatResolutionResult, CombatSide,
    CombatSimulationParticipant, EffectVisibility,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

const TEMPLATE_MOVE_INTENT: &str = "combat.log.move_intent";
const TEMPLATE_TARGET_SELECTION: &str = "combat.log.target_selection";
const TEMPLATE_COLLISION: &str = "combat.log.collision";
const TEMPLATE_DAMAGE_APPLIED: &str = "combat.log.damage_applied";
const TEMPLATE_EFFECT_APPLIED: &str = "combat.log.effect_applied";
const TEMPLATE_EFFECT_APPLIED_HIDDEN: &str = "combat.log.effect_applied_hidden";

/// 정본 13의 "공용 연출 문법". renderer가 이 종류만 보고 표현을 고른다.
/// 값은 판정 결과에서 파생되며, 이 enum이 애니메이션·CSS·색을 지정하지 않는다.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CombatSpectatorCue {
    Attack,
    Hit,
    Evade,
    BalanceBroken,
    Incapacitated,
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
    /// `resolution.execution.provenance`가 없거나(구 JSON) `tick_millis`가 0이면
    /// 화면 시간을 시뮬레이션 시간에 맞출 수 없다. 값을 지어내지 않고 에러를 낸다.
    MissingProvenance,
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
    let tick_millis = request
        .resolution
        .execution
        .provenance
        .as_ref()
        .map(|p| p.tick_millis)
        .filter(|millis| *millis > 0)
        .ok_or(CombatSpectatorError::MissingProvenance)?;
    let participants: BTreeMap<&str, &CombatSimulationParticipant> = request
        .participants
        .iter()
        .map(|p| (p.id.as_str(), p))
        .collect();

    let outcomes_by_tick: BTreeMap<u32, &Vec<CombatAttackOutcome>> = request
        .resolution
        .frames
        .iter()
        .map(|frame| (frame.tick, &frame.outcomes))
        .collect();
    let combatants_by_tick: BTreeMap<u32, &Vec<CombatResolutionCombatant>> = request
        .resolution
        .frames
        .iter()
        .map(|frame| (frame.tick, &frame.combatants))
        .collect();

    let mut frames = Vec::with_capacity(request.resolution.execution.frames.len());
    for tick_frame in &request.resolution.execution.frames {
        let outcomes = outcomes_by_tick.get(&tick_frame.tick).copied();
        let combatants_snapshot = combatants_by_tick.get(&tick_frame.tick).copied();
        let mut pieces = Vec::with_capacity(tick_frame.positions.len());
        for (id, position) in &tick_frame.positions {
            let participant = participants
                .get(id.as_str())
                .ok_or_else(|| CombatSpectatorError::UnknownParticipant(id.clone()))?;
            pieces.push(CombatSpectatorPiece {
                id: id.clone(),
                side: participant.side,
                position: *position,
                facing: participant.facing,
                active: participant.active,
                cues: cues_for(id, outcomes, combatants_snapshot),
            });
        }
        frames.push(CombatSpectatorFrame {
            tick: tick_frame.tick,
            pieces,
        });
    }

    let full_log = build_log(request);
    let core_log = full_log
        .iter()
        .filter(|entry| entry.importance >= CombatLogImportance::Important)
        .cloned()
        .collect();

    let mut view = CombatSpectatorView {
        resolution_fingerprint: request.resolution.fingerprint.clone(),
        tick_millis,
        frames,
        core_log,
        full_log,
        fingerprint: String::new(),
    };
    view.fingerprint = fingerprint(&view);
    Ok(view)
}

enum LogSource<'a> {
    Execution(&'a CombatLogEvent),
    Resolution(&'a CombatResolutionLogEvent),
}

/// `execution.full_log`(`CombatLogEvent`)와 `resolution.full_log`(`CombatResolutionLogEvent`)를
/// tick -> sequence -> 실행로그 우선 순서로 합병하고, 등록된 태그 -> 템플릿 id 표로 옮긴다.
///
/// 누설 차단 (정본 5번 문장):
/// - `AttackRoll`/`EffectSuppressed`는 숨은 판정·억제 사유를 담고 있어 아예 제외한다.
/// - `EffectApplied`의 `effect_id`가 `catalog`에서 `Hidden`/`Conditional`이거나
///   `catalog`에 아예 없으면 `effect_id`를 마스킹하고 템플릿을 `_hidden` 변형으로 바꾼다.
fn build_log(request: &CombatSpectatorRequest) -> Vec<CombatSpectatorLogEntry> {
    let catalog: BTreeMap<&str, &CombatEffectDefinition> = request
        .catalog
        .effects
        .iter()
        .map(|effect| (effect.id.as_str(), effect))
        .collect();

    let mut merged: Vec<(u32, u32, u8, LogSource)> = Vec::new();
    for event in &request.resolution.execution.full_log {
        merged.push((event.tick, event.sequence, 0, LogSource::Execution(event)));
    }
    for event in &request.resolution.full_log {
        if matches!(
            event.tag,
            CombatResolutionLogTag::AttackRoll | CombatResolutionLogTag::EffectSuppressed
        ) {
            continue;
        }
        merged.push((event.tick, event.sequence, 1, LogSource::Resolution(event)));
    }
    merged.sort_by(|a, b| (a.0, a.1, a.2).cmp(&(b.0, b.1, b.2)));

    merged
        .into_iter()
        .map(|(tick, sequence, _, source)| match source {
            LogSource::Execution(event) => CombatSpectatorLogEntry {
                tick,
                sequence,
                template_id: match event.tag {
                    CombatLogTag::MoveIntent => TEMPLATE_MOVE_INTENT,
                    CombatLogTag::TargetSelection => TEMPLATE_TARGET_SELECTION,
                }
                .to_string(),
                importance: event.importance,
                actor_id: event.actor_id.clone(),
                target_id: event.target_id.clone(),
                value_hundredths: None,
                effect_id: None,
            },
            LogSource::Resolution(event) => {
                let (template_id, value_hundredths, effect_id) = match event.tag {
                    CombatResolutionLogTag::Collision => (TEMPLATE_COLLISION, None, None),
                    CombatResolutionLogTag::DamageApplied => {
                        (TEMPLATE_DAMAGE_APPLIED, Some(event.value_hundredths), None)
                    }
                    CombatResolutionLogTag::EffectApplied => {
                        let visible = event
                            .effect_id
                            .as_deref()
                            .and_then(|id| catalog.get(id))
                            .is_some_and(|def| matches!(def.visibility, EffectVisibility::Public));
                        if visible {
                            (TEMPLATE_EFFECT_APPLIED, None, event.effect_id.clone())
                        } else {
                            (TEMPLATE_EFFECT_APPLIED_HIDDEN, None, None)
                        }
                    }
                    CombatResolutionLogTag::AttackRoll
                    | CombatResolutionLogTag::EffectSuppressed => {
                        unreachable!("filtered out above before merge/sort")
                    }
                };
                CombatSpectatorLogEntry {
                    tick,
                    sequence,
                    template_id: template_id.to_string(),
                    importance: event.importance,
                    actor_id: event.actor_id.clone(),
                    target_id: Some(event.target_id.clone()),
                    value_hundredths,
                    effect_id,
                }
            }
        })
        .collect()
}

/// 정본 13의 공용 연출 문법 5규칙을 적용한다: Attack/Hit/Evade(outcome 기반) +
/// BalanceBroken/Incapacitated(같은 tick의 전투원 스냅샷 기반) 외 cue는 만들지 않는다.
/// 규칙 밖 조합(예: hit && damage == 0)은 어떤 cue도 만들지 않는다.
///
/// BalanceBroken/Incapacitated는 이전 tick과 비교하지 않는다 — "지금 그 상태인가"만
/// 본다. 스냅샷이 없거나(구 JSON의 additive-optional 빈 Vec 포함) 그 tick에 해당 id가
/// 없으면 두 cue를 붙이지 않는다. 에러가 아니다.
fn cues_for(
    id: &str,
    outcomes: Option<&Vec<CombatAttackOutcome>>,
    combatants_snapshot: Option<&Vec<CombatResolutionCombatant>>,
) -> Vec<CombatSpectatorCue> {
    let mut cues: BTreeSet<CombatSpectatorCue> = BTreeSet::new();
    if let Some(outcomes) = outcomes {
        for outcome in outcomes {
            if outcome.actor_id == id {
                cues.insert(CombatSpectatorCue::Attack);
            }
            if outcome.target_id == id {
                if outcome.hit && outcome.damage_hundredths > 0 {
                    cues.insert(CombatSpectatorCue::Hit);
                }
                if outcome.in_range && !outcome.hit {
                    cues.insert(CombatSpectatorCue::Evade);
                }
            }
        }
    }
    if let Some(snapshot) = combatants_snapshot {
        if let Some(combatant) = snapshot.iter().find(|c| c.id == id) {
            if combatant.balance_hundredths <= 0 {
                cues.insert(CombatSpectatorCue::BalanceBroken);
            }
            if combatant.current_health_hundredths <= 0 {
                cues.insert(CombatSpectatorCue::Incapacitated);
            }
        }
    }
    cues.into_iter().collect()
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

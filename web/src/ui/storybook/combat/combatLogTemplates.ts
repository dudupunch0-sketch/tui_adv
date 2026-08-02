import type { CombatSpectatorLogEntry } from '../../../core/types';

// ---------------------------------------------------------------------------
// Wave 3 Step 1d-2 — 로그 템플릿 테이블 (renderer 소유).
//
// Hard invariant: this module only *formats* a `CombatSpectatorLogEntry`.
// It never derives cues, never filters by importance, never recomputes
// damage — escape-core has already decided all of that.
//
// 6개 template id의 문장 형식은
// `crates/escape-terminal/src/snapshot.rs::combat_log_template_line`의 표와
// 글자 단위로 맞춘다 (두 렌더러가 같은 사건을 다르게 부르지 않도록, 정본 13).
//
// 한 가지 의도적인 차이: terminal 쪽은 `combat.log.damage_applied`에서
// `value_hundredths`가 `None`이면 `unwrap_or(0)`로 0을 채운다. 이 슬라이스의
// 플랜(I6)은 그 대신 "(수치 없음)"을 명시적으로 쓰라고 지시한다 — 없는 값을
// 0으로 때우지 않는다. 그 경로는 terminal 쪽에서도 테스트로 고정돼 있지 않다.
// ---------------------------------------------------------------------------

const NO_TARGET_MARKER = '(대상 없음)';
const NO_VALUE_MARKER = '(수치 없음)';
const NO_EFFECT_ID_MARKER = '(효과 id 없음)';
const HIDDEN_EFFECT_MARKER = '정체불명';

/** 6개 등록된 template_id + fallback을 문장으로 옮긴다. 알 수 없는 id는
 * 조용히 버리지 않고 template_id 자체를 노출하는 fallback 줄을 만든다. */
export function combatLogTemplateLine(entry: CombatSpectatorLogEntry): string {
  const actor = entry.actor_id;
  const target = entry.target_id ?? null;
  switch (entry.template_id) {
    case 'combat.log.move_intent':
      return target ? `${actor} 이동 의도 (목표 ${target})` : `${actor} 이동 의도`;
    case 'combat.log.target_selection':
      return target ? `${actor} → 목표 지정: ${target}` : `${actor} 목표 지정 ${NO_TARGET_MARKER}`;
    case 'combat.log.collision':
      return target ? `${actor} × ${target} 충돌` : `${actor} 충돌 ${NO_TARGET_MARKER}`;
    case 'combat.log.damage_applied': {
      const value = formatValueHundredths(entry.value_hundredths);
      return target ? `${actor} → ${target} 피해 ${value}` : `${actor} 피해 ${value} ${NO_TARGET_MARKER}`;
    }
    case 'combat.log.effect_applied': {
      const effect = entry.effect_id ?? NO_EFFECT_ID_MARKER;
      return target
        ? `${actor} → ${target} 효과 적용 [${effect}]`
        : `${actor} 효과 적용 [${effect}] ${NO_TARGET_MARKER}`;
    }
    case 'combat.log.effect_applied_hidden':
      return target
        ? `${actor} → ${target} 효과 적용 [${HIDDEN_EFFECT_MARKER}]`
        : `${actor} 효과 적용 [${HIDDEN_EFFECT_MARKER}] ${NO_TARGET_MARKER}`;
    default:
      return `${actor} → ${target ?? NO_TARGET_MARKER} 알 수 없는 사건 [template_id=${entry.template_id}]`;
  }
}

/** 등록된 6개 template id 집합. `data-log-unknown` 판정에 쓰인다 (WP4). */
export const KNOWN_COMBAT_LOG_TEMPLATE_IDS: readonly string[] = [
  'combat.log.move_intent',
  'combat.log.target_selection',
  'combat.log.collision',
  'combat.log.damage_applied',
  'combat.log.effect_applied',
  'combat.log.effect_applied_hidden',
];

export function isKnownCombatLogTemplateId(templateId: string): boolean {
  return KNOWN_COMBAT_LOG_TEMPLATE_IDS.includes(templateId);
}

function formatValueHundredths(valueHundredths: number | null | undefined): string {
  if (valueHundredths === null || valueHundredths === undefined) return NO_VALUE_MARKER;
  return String(roundHundredthsToInt(valueHundredths));
}

/** hundredths 정수 -> 반올림 정수 (정본 11 §8: "표시할 때 정수 반올림").
 * Rust `round_hundredths_to_int`와 동일한 half-up 반올림(부호 보존)이다. */
export function roundHundredthsToInt(valueHundredths: number): number {
  const sign = valueHundredths < 0 ? -1 : 1;
  const magnitude = Math.abs(valueHundredths);
  const rounded = Math.floor((magnitude + 50) / 100);
  return sign * rounded;
}

// ---------------------------------------------------------------------------
// Wave 3 Step 1d-3 — data-driven combat playback keyframes.
//
// This module turns an already-projected per-tick frame list into CSS
// `@keyframes` text. It is a pure string generator: it never recomputes a
// gameplay judgment (no cue derivation, no re-deciding what happened at a
// tick — escape-core already decided all of that, 정본 13) and it never
// widens or narrows the board projection itself (that stays in
// `renderCombatStage.ts`, which passes already-projected board-percent x/y
// per tick).
//
// Hard invariants enforced here (see
// `fable_combat_wave3_step1d3_2608021755.md` §3 for the source of truth):
//
// - I1 (screen time = simulation time): the generated `animation-duration`
//   is exactly `(frameCount - 1) * tickMillis` ms, and keyframe k sits at
//   exactly `k / (frameCount - 1) * 100%`. No easing/padding is added to the
//   total length. `frameCount <= 1` (or zero tracks, or `tickMillis <= 0`,
//   which core would already have rejected via `MissingProvenance`) yields
//   an empty string — never a division by zero, never `NaN`/`Infinity`.
// - I2 (no time manipulation): the whole block uses a single linear overall
//   timeline (`animation-timing-function: linear` at the animation level).
//   No `animation-play-state: paused` is ever emitted.
// - I3 (reduced-motion parity): every declaration this module emits is
//   wrapped in one `@media (prefers-reduced-motion: no-preference) { ... }`
//   block. Nothing is emitted outside it.
// - I4 (zero recomputation): the only math here is the display conversion
//   from absolute per-tick board-percent coordinates to a translate offset
//   *relative to the final frame* (so the reduced-motion resting position —
//   the last frame — and the animation's `100%` keyframe are the same
//   position without depending on `animation-fill-mode`).
// - I5 (determinism): no `Math.random()`, no `Date.now()`, no reliance on
//   object/array iteration order for identity. `@keyframes` names are
//   derived from the piece id's own content (sanitize + djb2 hash), so two
//   different ids that sanitize to the same string never collide, and the
//   name does not depend on the id's position in `tracks`.
// - I9 (performance / allowed properties): only `translate`, `opacity`, and
//   `filter` are ever animated. Never `left`/`top` (1d-2 already fixed the
//   piece's box position with those; motion rides on top via `translate`).
//
// WP3 — cue presentation grammar (정본 13's 5-cue vocabulary, §4-3 of the
// plan). **`balance_broken` animates `rotate`, which I9's allow-list above
// does not include.** This paragraph used to claim the opposite — that I9
// won and the cue was a `translate`-only wobble — and stayed that way after
// the tilt was implemented, so the comment contradicted the file it
// documents. Corrected 2026-08-07 to describe what the code actually does.
//
// The contradiction is real and unresolved, not a wording slip: §4-3's table
// asks for "rotate 흔들림 유지" while §3 I9, a Hard invariant, restricts every
// animated property to `translate`/`opacity`/`filter` with no stated
// exception. One of the two has to give. Widening the allow-list is a
// presentation-grammar decision, so it belongs to the cue-vocabulary track
// rather than to whichever slice happens to notice — see
// `docs/design/Combat_Hex_Rework_Development_Plan.md` §9. Until that is
// decided, do not "restore" this comment to the old claim and do not silently
// drop the tilt; either move would hide the open question again.
//
// The other four cues stay within `translate`/`opacity`/`filter`:
// - `attack`: a brief lunge toward `piece.facing` (never a direction this
//   module invents — §4-3 explicitly authorizes `facing` only for this cue,
//   and only when it is not `(0, 0)`) then a return to the un-perturbed
//   path, inserted as one extra keyframe stop at the midpoint of the tick
//   interval where the cue occurs.
// - `hit`: a small damped two-beat judder (translate only), not tied to any
//   direction — no field gives a "which way was this piece knocked"
//   judgment, and inventing one would violate rule 2.
// - `evade`: a brief lateral (board Y-axis — 정본 09: "측면: 화면 위·아래")
//   nudge and return. The sign of the nudge is a fixed, symmetric
//   presentation choice (not a gameplay judgment core makes), documented at
//   its call site.
// - `incapacitated`: sustained `opacity`/`filter` dimming, reusing the exact
//   values 1d-2 already uses for `[data-active="false"]` (I7: no new
//   literal). "Sustained" here means exactly what the current frame's own
//   `cues` array says — this module does not carry the cue forward from an
//   earlier frame if a later frame's `cues` array omits it (that would be
//   inventing persistence the data does not state, rule 2).
// ---------------------------------------------------------------------------

import type { CombatSpectatorCue, HexCoord } from '../../../core/types';

export interface PieceMotionFrame {
  /** Board-percent x/y for this tick, projected with the SAME min/max-over-
   * all-frames range used for every other tick of this playback (that
   * expansion is `renderCombatStage.ts`'s job, not this module's). */
  x: number;
  y: number;
  /** This tick's cue set, exactly as core decided it (WP3). Omit (or leave
   * empty) for pure positional motion with no presentation flourish. */
  cues?: CombatSpectatorCue[];
  /** This piece's own facing at this tick, as a raw hex direction (never a
   * screen vector) — the only input the `attack` lunge direction may use
   * (§4-3). Omit, or `(q: 0, r: 0)`, to skip the lunge. This module converts
   * it to a screen vector itself (`hexFacingToScreenVector`), right where it
   * normalizes — nothing upstream pre-converts it. */
  facing?: HexCoord;
}

export interface PieceMotionTrack {
  pieceId: string;
  /** One entry per `view.frames` index. Every track passed into the same
   * `buildCombatMotionCss` call must have the same length (`frameCount`). */
  frames: PieceMotionFrame[];
}

export interface CombatMotionInput {
  /** Comes straight from `view.tick_millis` (core-provided; core already
   * rejects a missing/zero value with `MissingProvenance` before this
   * renders, so this module does not invent a fallback). */
  tickMillis: number;
  tracks: PieceMotionTrack[];
}

export interface CombatMotionResult {
  /** Full `@media (prefers-reduced-motion: no-preference) { ... }` block:
   * one `@keyframes` + one animation-applying rule per piece. Empty string
   * when there is nothing to animate. */
  css: string;
  /** Exactly `(frameCount - 1) * tickMillis`, or `0` when there is nothing
   * to animate. */
  durationMillis: number;
  /** `pieceId -> generated @keyframes name`, so callers needing to
   * cross-reference the name (tests, or future WP3/WP4 wiring) do not have
   * to re-derive it by hand. */
  keyframeNames: ReadonlyMap<string, string>;
}

/** CSS identifiers may only contain `[a-zA-Z0-9_-]` (escapes aside, which we
 * do not want to have to reason about downstream). Anything else collapses
 * to `-`. This alone is not collision-safe (`a<b` and `a>b` both collapse to
 * `a-b`), so `keyframeNameForPiece` appends a content hash of the *original*
 * id on top of it. */
function sanitizeForCssIdentifierFragment(raw: string): string {
  return raw.replace(/[^a-zA-Z0-9_-]/g, '-');
}

/** djb2 string hash, folded to an unsigned 32-bit int and rendered base-36.
 * Deterministic and a pure function of the string's content — never of its
 * position in any array (I5). Not cryptographic; only needs to make
 * sanitize() collisions astronomically unlikely for a per-encounter cast
 * (I9's stated upper bound is 12 combatants). */
function djb2Hash(raw: string): string {
  let hash = 5381;
  for (let i = 0; i < raw.length; i += 1) {
    hash = (hash * 33 + raw.charCodeAt(i)) >>> 0;
  }
  return hash.toString(36);
}

/** Deterministic, collision-safe `@keyframes` name for a piece id. Always
 * starts with a letter (the `combat-piece-` prefix), so a pieceId that
 * starts with a digit is never a problem either. */
export function keyframeNameForPiece(pieceId: string): string {
  return `combat-piece-${sanitizeForCssIdentifierFragment(pieceId)}-${djb2Hash(pieceId)}`;
}

/** Round to 4 decimal places and stringify without a trailing `.0000` tail,
 * so the same numeric input always serializes to the same string (I5) and
 * float noise (e.g. `2.0000000000000004`) never leaks into the CSS. */
function formatNumber(value: number): string {
  const rounded = Math.round(value * 10000) / 10000;
  // `-0` must render as `0`, not `-0` (String(-0) already does this in JS,
  // this comment just documents that it is intentional, not an oversight).
  return String(rounded);
}

function formatPercentStop(value: number): string {
  return formatNumber(value);
}

export function buildCombatMotionCss(input: CombatMotionInput): CombatMotionResult {
  const keyframeNames = new Map<string, string>();
  const frameCount = input.tracks[0]?.frames.length ?? 0;

  if (frameCount <= 1 || input.tracks.length === 0 || input.tickMillis <= 0) {
    return { css: '', durationMillis: 0, keyframeNames };
  }

  const durationMillis = (frameCount - 1) * input.tickMillis;

  const blocks: string[] = [];
  for (const track of input.tracks) {
    // The generated CSS is embedded in a `<style>` element (WP2 —
    // `renderCombatStage.ts`), which HTML parses as *raw text*: the parser
    // ends the element at the first literal `</style` substring regardless
    // of any CSS-level quoting. The `@keyframes` name is already a sanitized
    // hash (never contains the raw id), but the animation rule's attribute
    // selector must match the piece's real `data-piece-id` value verbatim,
    // so it cannot be sanitized the same way. If a piece id could break out
    // of the `<style>` element this way, skip animating that piece rather
    // than emitting unsafe markup — the piece still renders correctly at
    // its static (last-frame) position, just without motion (I2 also
    // forbids inventing a "safe" substitute id).
    if (/<\/style/i.test(track.pieceId)) continue;
    const name = keyframeNameForPiece(track.pieceId);
    keyframeNames.set(track.pieceId, name);
    blocks.push(buildPieceKeyframesBlock(name, track.frames));
    blocks.push(buildPieceAnimationRule(track.pieceId, name, durationMillis));
  }

  if (blocks.length === 0) {
    return { css: '', durationMillis: 0, keyframeNames };
  }

  const css = `@media (prefers-reduced-motion: no-preference) {\n${blocks.join('\n')}\n}`;
  return { css, durationMillis, keyframeNames };
}

// -- WP3: cue presentation grammar constants --------------------------------
//
// All magnitudes below are presentation constants (how far a purely
// decorative flourish reaches), not claimed simulation facts — the same
// category as the existing `BOARD_INSET_PERCENT` in renderCombatStage.ts.
// Units are `cqw`/`cqh` points (numeric, unitless here; the unit suffix is
// added where the value is written into the CSS text).

/** `balance_broken`: alternating side-to-side translate wobble plus an
 * alternating tilt. 정본 13은 이 cue를 "흔들림/기울어짐"으로 정의하므로 둘 다
 * 필요하다 — 흔들림만 쓰면 `hit`의 진동과 구별되지 않아 공용 문법이 무너진다.
 * 기울어짐은 `rotate`로 표현한다: `translate`와 같은 개별 transform 속성이라
 * 컴포지터 스레드에서 처리되고 레이아웃을 만들지 않는다(플랜 I9의 허용
 * 목록에서 빠져 있었던 것은 누락이며, 오케스트레이터가 정본에 맞춰 허용으로
 * 바로잡았다). 부호를 tick 짝수/홀수로 번갈아 주어 한쪽으로 흐르지 않고
 * 휘청이는 것으로 읽히게 한다. */
const BALANCE_WOBBLE_MAGNITUDE = 1.5;
const BALANCE_TILT_DEGREES = 7;

/** `attack`/`evade`: a single lunge-then-return point at the midpoint of the
 * tick interval where the cue occurs. */
const LUNGE_FRACTION = 0.5;
const ATTACK_LUNGE_MAGNITUDE = 4;
const EVADE_LUNGE_MAGNITUDE = 4;

/** `hit`: a damped two-beat judder (knockback + decaying rebound), not tied
 * to any direction — no field tells the renderer which way a piece was
 * struck, and inventing one would violate rule 2. */
const HIT_JUDDER_FRACTION_1 = 0.3;
const HIT_JUDDER_FRACTION_2 = 0.65;
const HIT_JUDDER_MAGNITUDE = 3;

/** `incapacitated`: reuses 1d-2's existing dimming values for
 * `[data-active="false"]` verbatim (I7: no new literal). */
const INCAPACITATED_OPACITY = 0.55;
const INCAPACITATED_FILTER = 'saturate(0.4)';

interface Contribution {
  dx: number;
  dy: number;
}

/** flat-top axial `(q, r)` -> unit-scale screen vector — the exact formula
 * `renderCombatStage.ts`'s `axialToScreen` uses (§4-2 of
 * `fable_combat_hex_t1b2_step1_2608072024.md`), duplicated here rather than
 * imported because that module imports *from* this one (`buildCombatMotionCss`)
 * and a hex direction needs the same axis-shape fix a hex position does
 * before it can be normalized. Only `q`/`r` ever feed this — never
 * `facing.x`/`facing.y`, which do not exist on `HexCoord`. */
function hexFacingToScreenVector(q: number, r: number): { x: number; y: number } {
  return { x: 1.5 * q, y: Math.sqrt(3) * (r + q / 2) };
}

/** cue-driven translate contribution at one fraction *within* the tick
 * interval `[k, k+1)` — added on top of the natural (linearly interpolated)
 * position, so the base position track is never distorted, only briefly
 * perturbed. `cues`/`facing` are frame k's own (the tick where the cue was
 * decided to have happened). */
function lungeContributionAtFraction(
  fraction: number,
  cues: CombatSpectatorCue[],
  facing: HexCoord | undefined,
): Contribution {
  let dx = 0;
  let dy = 0;
  if (fraction === LUNGE_FRACTION) {
    if (cues.includes('attack') && facing && (facing.q !== 0 || facing.r !== 0)) {
      // §4-3: facing is a hex direction now, not a screen vector — convert
      // it the same way a hex position is converted (§4-2) before
      // normalizing. The zero-vector guard above stays even though Rust now
      // restricts facing to the 6 hex directions (never `(0, 0)`) — this
      // renderer does not become code that trusts its input.
      const screen = hexFacingToScreenVector(facing.q, facing.r);
      const length = Math.hypot(screen.x, screen.y);
      dx += (screen.x / length) * ATTACK_LUNGE_MAGNITUDE;
      dy += (screen.y / length) * ATTACK_LUNGE_MAGNITUDE;
    }
    if (cues.includes('evade')) {
      // Lateral = board Y axis (정본 09: "측면: 화면 위·아래"). The sign is a
      // fixed, symmetric presentation choice, not a gameplay judgment — core
      // does not say which of the two lateral directions a piece evaded
      // toward, and this module does not invent one.
      dy += -EVADE_LUNGE_MAGNITUDE;
    }
  }
  if (fraction === HIT_JUDDER_FRACTION_1 && cues.includes('hit')) {
    dx += HIT_JUDDER_MAGNITUDE;
  }
  if (fraction === HIT_JUDDER_FRACTION_2 && cues.includes('hit')) {
    dx += -HIT_JUDDER_MAGNITUDE * 0.5;
  }
  return { dx, dy };
}

/** One `@keyframes` rule: for each tick k, translate offset *relative to the
 * final frame* (final frame's own offset is therefore always `0 0`, matching
 * the reduced-motion resting position — see the module comment for why),
 * plus (WP3) any cue-driven translate/opacity/filter perturbation for that
 * tick, using only the current frame's own `cues`/`facing` (never inferred
 * from neighboring ticks). */
function buildPieceKeyframesBlock(name: string, frames: PieceMotionFrame[]): string {
  const frameCount = frames.length;
  const last = frames[frameCount - 1];

  // Base (pre-cue) translate offset per tick, relative to the final frame,
  // plus the balance_broken wobble (sustained per-tick, not a lunge-return,
  // so it belongs at the real tick stops rather than the interval
  // micro-stops below).
  const baseDx: number[] = [];
  const baseDy: number[] = [];
  for (let k = 0; k < frameCount; k += 1) {
    const cues = frames[k].cues ?? [];
    const wobble = cues.includes('balance_broken') ? (k % 2 === 0 ? BALANCE_WOBBLE_MAGNITUDE : -BALANCE_WOBBLE_MAGNITUDE) : 0;
    baseDx.push(frames[k].x - last.x + wobble);
    baseDy.push(frames[k].y - last.y);
  }

  // 감광 속성은 이 트랙에 `incapacitated` cue가 한 번이라도 있을 때만
  // 내보낸다. 항상 내보내면 애니메이션이 `[data-active="false"]`의 정적 감광
  // (opacity .55 / saturate(.4))을 재생 내내 `opacity: 1`로 덮어써서 비참전
  // 말이 멀쩡하게 보인다.
  const trackHasIncapacitated = frames.some((f) => (f.cues ?? []).includes('incapacitated'));

  type Stop = {
    percent: number;
    dx: number;
    dy: number;
    tilt: number;
    opacity: number | null;
    filter: string | null;
  };
  const stops: Stop[] = [];

  for (let k = 0; k < frameCount; k += 1) {
    const cues = frames[k].cues ?? [];
    const incapacitated = cues.includes('incapacitated');
    const tilt = cues.includes('balance_broken')
      ? k % 2 === 0
        ? BALANCE_TILT_DEGREES
        : -BALANCE_TILT_DEGREES
      : 0;
    stops.push({
      percent: (k / (frameCount - 1)) * 100,
      dx: baseDx[k],
      dy: baseDy[k],
      tilt,
      opacity: trackHasIncapacitated ? (incapacitated ? INCAPACITATED_OPACITY : 1) : null,
      filter: trackHasIncapacitated ? (incapacitated ? INCAPACITATED_FILTER : 'none') : null,
    });

    if (k < frameCount - 1) {
      for (const fraction of [HIT_JUDDER_FRACTION_1, LUNGE_FRACTION, HIT_JUDDER_FRACTION_2]) {
        const contribution = lungeContributionAtFraction(fraction, cues, frames[k].facing);
        if (contribution.dx === 0 && contribution.dy === 0) continue;
        const natural = {
          dx: baseDx[k] + (baseDx[k + 1] - baseDx[k]) * fraction,
          dy: baseDy[k] + (baseDy[k + 1] - baseDy[k]) * fraction,
        };
        stops.push({
          percent: ((k + fraction) / (frameCount - 1)) * 100,
          dx: natural.dx + contribution.dx,
          dy: natural.dy + contribution.dy,
          tilt,
          opacity: trackHasIncapacitated ? (incapacitated ? INCAPACITATED_OPACITY : 1) : null,
          filter: trackHasIncapacitated ? (incapacitated ? INCAPACITATED_FILTER : 'none') : null,
        });
      }
    }
  }

  const stopLines = stops.map((s) => {
    const declarations = [
      `translate: calc(-50% + ${formatNumber(s.dx)}cqw) calc(-50% + ${formatNumber(s.dy)}cqh)`,
    ];
    // `rotate`는 균형 붕괴 cue에만 쓴다. 0도일 때도 명시해야 기울어진 stop에서
    // 기울지 않은 stop으로 되돌아온다 (누락하면 브라우저가 기존 값을 유지하지
    // 않고 보간 대상에서 빠져 흔들림이 한 방향으로 남는다).
    if (stops.some((other) => other.tilt !== 0)) {
      declarations.push(`rotate: ${formatNumber(s.tilt)}deg`);
    }
    if (s.opacity !== null) declarations.push(`opacity: ${s.opacity}`);
    if (s.filter !== null) declarations.push(`filter: ${s.filter}`);
    return `  ${formatPercentStop(s.percent)}% { ${declarations.join('; ')}; }`;
  });
  return `  @keyframes ${name} {\n${stopLines.join('\n')}\n  }`;
}

/** `translate: -50% -50%` is the piece's own centering rule from 1d-2
 * (`web/src/styles/storybook.css` `.combat-board__piece`), unchanged by this
 * slice. This animation's own `100%` keyframe evaluates to
 * `calc(-50% + 0cqw) calc(-50% + 0cqh)` — numerically identical to that
 * static rule — so the animation's resting value and the `reduce`-media
 * resting value are the same position without relying on
 * `animation-fill-mode` semantics for anything other than *holding* it
 * there (`both` below) after playback ends. */
function buildPieceAnimationRule(pieceId: string, keyframeName: string, durationMillis: number): string {
  return `  .combat-board__piece[data-piece-id="${escapeAttributeSelectorValue(
    pieceId,
  )}"] { animation: ${keyframeName} ${durationMillis}ms linear both; }`;
}

/** `data-piece-id` values are already HTML-escaped by `renderCombatStage.ts`
 * when they appear in markup; here we are writing a CSS *attribute
 * selector* value, which has its own escaping rules (a `"` inside a
 * double-quoted attribute selector must be escaped, backslashes must be
 * escaped first so they are not read as introducing a new escape). */
function escapeAttributeSelectorValue(value: string): string {
  return (
    value
      .replace(/\\/g, '\\\\')
      .replace(/"/g, '\\"')
      // CSS 문자열에는 raw 개행을 넣을 수 없다 — 넣으면 그 규칙과 뒤따르는
      // 규칙까지 파싱이 깨진다. `\A` 코드포인트 이스케이프로 바꾸고, 뒤에
      // 오는 문자가 16진수로 이어 읽히지 않도록 공백 종결자를 붙인다.
      .replace(/\r\n?|\n/g, '\\A ')
      .replace(/\f/g, '\\C ')
  );
}

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
// plan): a **known deviation from the plan's illustrative table**, called
// out explicitly because instructed to report every such deviation. §4-3's
// table lists "rotate 흔들림 유지" as the suggested implementation for
// `balance_broken`, but §3 I9 — listed under "Hard invariants" — restricts
// every animated property to `translate`/`opacity`/`filter` with no stated
// exception. Since I9 is the Hard invariant and §4-3 is non-binding
// implementation guidance, I9 wins: `balance_broken` is rendered as a
// `translate`-based side-to-side wobble (a stand-in for "흔들림", the shake
// half of "흔들림/기울어짐") rather than a `rotate` tilt. The "기울어짐"
// (tilt) half of that cue's description is not reproduced — CSS has no
// tilt-without-`rotate` primitive, and inventing a fake tilt via `skew`
// (also not in the I9 allow-list) would repeat the same problem.
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

import type { CombatSpectatorCue } from '../../../core/types';

export interface PieceMotionFrame {
  /** Board-percent x/y for this tick, projected with the SAME min/max-over-
   * all-frames range used for every other tick of this playback (that
   * expansion is `renderCombatStage.ts`'s job, not this module's). */
  x: number;
  y: number;
  /** This tick's cue set, exactly as core decided it (WP3). Omit (or leave
   * empty) for pure positional motion with no presentation flourish. */
  cues?: CombatSpectatorCue[];
  /** This piece's own facing at this tick — the only input the `attack`
   * lunge direction may use (§4-3). Omit, or `(0, 0)`, to skip the lunge. */
  facing?: { x: number; y: number };
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

/** `balance_broken`: alternating side-to-side translate wobble (I9 note
 * above — stands in for "흔들림", not "기울어짐"). Applied on every tick
 * where the cue is present, alternating sign by tick parity so it reads as
 * a shake rather than a one-way drift. */
const BALANCE_WOBBLE_MAGNITUDE = 1.5;

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

/** cue-driven translate contribution at one fraction *within* the tick
 * interval `[k, k+1)` — added on top of the natural (linearly interpolated)
 * position, so the base position track is never distorted, only briefly
 * perturbed. `cues`/`facing` are frame k's own (the tick where the cue was
 * decided to have happened). */
function lungeContributionAtFraction(
  fraction: number,
  cues: CombatSpectatorCue[],
  facing: { x: number; y: number } | undefined,
): Contribution {
  let dx = 0;
  let dy = 0;
  if (fraction === LUNGE_FRACTION) {
    if (cues.includes('attack') && facing && (facing.x !== 0 || facing.y !== 0)) {
      const length = Math.hypot(facing.x, facing.y);
      dx += (facing.x / length) * ATTACK_LUNGE_MAGNITUDE;
      dy += (facing.y / length) * ATTACK_LUNGE_MAGNITUDE;
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

  type Stop = { percent: number; dx: number; dy: number; opacity: number; filter: string };
  const stops: Stop[] = [];

  for (let k = 0; k < frameCount; k += 1) {
    const cues = frames[k].cues ?? [];
    const incapacitated = cues.includes('incapacitated');
    stops.push({
      percent: (k / (frameCount - 1)) * 100,
      dx: baseDx[k],
      dy: baseDy[k],
      opacity: incapacitated ? INCAPACITATED_OPACITY : 1,
      filter: incapacitated ? INCAPACITATED_FILTER : 'none',
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
          opacity: incapacitated ? INCAPACITATED_OPACITY : 1,
          filter: incapacitated ? INCAPACITATED_FILTER : 'none',
        });
      }
    }
  }

  const stopLines = stops.map(
    (s) =>
      `  ${formatPercentStop(s.percent)}% { translate: calc(-50% + ${formatNumber(s.dx)}cqw) calc(-50% + ${formatNumber(
        s.dy,
      )}cqh); opacity: ${s.opacity}; filter: ${s.filter}; }`,
  );
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
  return value.replace(/\\/g, '\\\\').replace(/"/g, '\\"');
}

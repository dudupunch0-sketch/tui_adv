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
//   piece's box position with those; motion rides on top via `translate`),
//   and — see the WP3 comment further down for why this module does *not*
//   use `rotate` even though §4-3 of the plan suggests it for
//   `balance_broken`: I9 is listed as a Hard invariant and takes precedence
//   over that non-invariant implementation-direction table.
// ---------------------------------------------------------------------------

export interface PieceMotionFrame {
  /** Board-percent x/y for this tick, projected with the SAME min/max-over-
   * all-frames range used for every other tick of this playback (that
   * expansion is `renderCombatStage.ts`'s job, not this module's). */
  x: number;
  y: number;
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
    const name = keyframeNameForPiece(track.pieceId);
    keyframeNames.set(track.pieceId, name);
    blocks.push(buildPieceKeyframesBlock(name, track.frames));
    blocks.push(buildPieceAnimationRule(track.pieceId, name, durationMillis));
  }

  const css = `@media (prefers-reduced-motion: no-preference) {\n${blocks.join('\n')}\n}`;
  return { css, durationMillis, keyframeNames };
}

/** One `@keyframes` rule: for each tick k, translate offset *relative to the
 * final frame* (final frame's own offset is therefore always `0 0`, matching
 * the reduced-motion resting position — see the module comment for why). */
function buildPieceKeyframesBlock(name: string, frames: PieceMotionFrame[]): string {
  const frameCount = frames.length;
  const last = frames[frameCount - 1];
  const stops: string[] = [];
  for (let k = 0; k < frameCount; k += 1) {
    const percent = (k / (frameCount - 1)) * 100;
    const dx = frames[k].x - last.x;
    const dy = frames[k].y - last.y;
    stops.push(
      `  ${formatPercentStop(percent)}% { translate: calc(-50% + ${formatNumber(dx)}cqw) calc(-50% + ${formatNumber(
        dy,
      )}cqh); }`,
    );
  }
  return `  @keyframes ${name} {\n${stops.join('\n')}\n  }`;
}

/** `translate: -50% -50%` is the piece's own centering rule from 1d-2
 * (`web/src/styles/storybook.css` `.combat-board__piece`). WP2 replaces that
 * static declaration with the `calc(-50% + var(--dx))`-shaped one so the
 * animation (which sets the *inline* `translate` via this named animation)
 * and the reduced-motion resting rule agree on the same formula. This
 * function only emits the piece's animation-applying rule, not the
 * `--dx`/`--dy` custom-property rule itself — WP2 owns wiring `--piece-x`/
 * `--piece-y` (unchanged) plus this animation onto the same selector. */
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

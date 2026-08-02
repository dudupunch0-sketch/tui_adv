import { describe, expect, it } from 'vitest';

import { buildCombatMotionCss, keyframeNameForPiece } from './combatMotion';
import type { CombatMotionInput, PieceMotionTrack } from './combatMotion';

// ---------------------------------------------------------------------------
// Wave 3 Step 1d-3 — WP1. combatMotion.ts generates CSS `@keyframes` from an
// already-projected frame list. It never derives cues, never recomputes
// gameplay judgments — see the module-level comment in combatMotion.ts for
// the invariant list (I1/I2/I4/I5/I9 from
// fable_combat_wave3_step1d3_2608021755.md).
// ---------------------------------------------------------------------------

function track(pieceId: string, points: Array<{ x: number; y: number }>): PieceMotionTrack {
  return { pieceId, frames: points.map((p) => ({ x: p.x, y: p.y })) };
}

function extractPercentOffsets(css: string, keyframeName: string): number[] {
  const blockMatch = new RegExp(`@keyframes ${keyframeName} \\{([\\s\\S]*?)\\n\\}`).exec(css);
  expect(blockMatch).not.toBeNull();
  const body = blockMatch![1];
  const percents = [...body.matchAll(/(-?\d+(?:\.\d+)?)% \{/g)].map((m) => Number(m[1]));
  return percents;
}

describe('buildCombatMotionCss — I1: screen time equals simulation time', () => {
  it('produces a duration of exactly (frames.length - 1) * tickMillis', () => {
    const input: CombatMotionInput = {
      tickMillis: 250,
      tracks: [track('ally_1', [{ x: 20, y: 50 }, { x: 40, y: 50 }, { x: 60, y: 50 }, { x: 80, y: 50 }])],
    };
    const result = buildCombatMotionCss(input);
    expect(result.durationMillis).toBe(3 * 250);
    // The `animation` shorthand carries the duration (see
    // `buildPieceAnimationRule`); assert on the shorthand form actually
    // emitted rather than a longhand `animation-duration:` declaration.
    expect(result.css).toContain(`${3 * 250}ms linear`);
  });

  it('places tick k at exactly k / (frames.length - 1) * 100% (no eased/padded offsets)', () => {
    const points = [{ x: 10, y: 10 }, { x: 20, y: 30 }, { x: 30, y: 70 }, { x: 40, y: 90 }, { x: 50, y: 10 }];
    const input: CombatMotionInput = { tickMillis: 100, tracks: [track('ally_1', points)] };
    const result = buildCombatMotionCss(input);
    const name = result.keyframeNames.get('ally_1')!;
    const offsets = extractPercentOffsets(result.css, name);
    const frameCount = points.length;
    for (let k = 0; k < frameCount; k += 1) {
      const expected = (k / (frameCount - 1)) * 100;
      expect(offsets).toContainEqual(expect.closeTo(expected, 4));
    }
  });

  it('returns an empty string and 0 duration for a single frame (no division by zero, no NaN)', () => {
    const input: CombatMotionInput = { tickMillis: 100, tracks: [track('ally_1', [{ x: 50, y: 50 }])] };
    const result = buildCombatMotionCss(input);
    expect(result.css).toBe('');
    expect(result.durationMillis).toBe(0);
    expect(result.css).not.toContain('NaN');
    expect(result.css).not.toContain('Infinity');
  });

  it('returns an empty string for zero frames', () => {
    const input: CombatMotionInput = { tickMillis: 100, tracks: [track('ally_1', [])] };
    const result = buildCombatMotionCss(input);
    expect(result.css).toBe('');
    expect(result.durationMillis).toBe(0);
  });

  it('returns an empty string when there are no tracks at all', () => {
    const input: CombatMotionInput = { tickMillis: 100, tracks: [] };
    const result = buildCombatMotionCss(input);
    expect(result.css).toBe('');
    expect(result.durationMillis).toBe(0);
  });
});

describe('buildCombatMotionCss — I5: deterministic output', () => {
  it('produces byte-identical CSS for the same input across repeated calls', () => {
    const input: CombatMotionInput = {
      tickMillis: 120,
      tracks: [
        track('ally_1', [{ x: 20, y: 50 }, { x: 30, y: 55 }, { x: 40, y: 50 }]),
        track('enemy_1', [{ x: 80, y: 50 }, { x: 70, y: 45 }, { x: 60, y: 50 }]),
      ],
    };
    const a = buildCombatMotionCss(input);
    const b = buildCombatMotionCss(input);
    expect(a.css).toBe(b.css);
    expect(a.durationMillis).toBe(b.durationMillis);
  });
});

describe('buildCombatMotionCss — I3: wrapped in prefers-reduced-motion: no-preference', () => {
  it('wraps the entire generated block in the media query, nothing leaks outside it', () => {
    const input: CombatMotionInput = {
      tickMillis: 100,
      tracks: [track('ally_1', [{ x: 20, y: 50 }, { x: 40, y: 50 }])],
    };
    const result = buildCombatMotionCss(input);
    expect(result.css.trim().startsWith('@media (prefers-reduced-motion: no-preference)')).toBe(true);
  });
});

describe('buildCombatMotionCss — I9: only translate/opacity/filter are animated', () => {
  it('never emits a `left:` or `top:` declaration inside the generated keyframes', () => {
    const input: CombatMotionInput = {
      tickMillis: 100,
      tracks: [track('ally_1', [{ x: 20, y: 50 }, { x: 80, y: 50 }])],
    };
    const result = buildCombatMotionCss(input);
    expect(result.css).not.toMatch(/[^-]\bleft:/);
    expect(result.css).not.toMatch(/[^-]\btop:/);
  });
});

describe('buildCombatMotionCss — <style>-embedding safety', () => {
  it('skips animating a piece id that could break out of a <style> raw-text element', () => {
    const input: CombatMotionInput = {
      tickMillis: 100,
      tracks: [track('</style><script>alert(1)</script>', [{ x: 20, y: 50 }, { x: 40, y: 50 }])],
    };
    const result = buildCombatMotionCss(input);
    expect(result.css).toBe('');
    expect(result.keyframeNames.size).toBe(0);
  });

  it('still animates the other pieces when one piece id is unsafe', () => {
    const input: CombatMotionInput = {
      tickMillis: 100,
      tracks: [
        track('</style>', [{ x: 20, y: 50 }, { x: 40, y: 50 }]),
        track('ally_1', [{ x: 60, y: 50 }, { x: 70, y: 50 }]),
      ],
    };
    const result = buildCombatMotionCss(input);
    expect(result.keyframeNames.has('</style>')).toBe(false);
    expect(result.keyframeNames.has('ally_1')).toBe(true);
    expect(result.css).toContain(keyframeNameForPiece('ally_1'));
  });
});

describe('keyframeNameForPiece — CSS-identifier safety', () => {
  it('sanitizes dangerous characters and never collides for distinct ids that sanitize to the same string', () => {
    const nameA = keyframeNameForPiece('a<b');
    const nameB = keyframeNameForPiece('a>b');
    const nameC = keyframeNameForPiece('a b');
    expect(nameA).not.toBe(nameB);
    expect(nameA).not.toBe(nameC);
    expect(nameB).not.toBe(nameC);
    // A valid CSS identifier: letters/digits/hyphen/underscore only, and this
    // module always prefixes with a letter so a leading digit in pieceId is
    // never a problem.
    for (const name of [nameA, nameB, nameC]) {
      expect(name).toMatch(/^[a-zA-Z_-][a-zA-Z0-9_-]*$/);
    }
  });

  it('is deterministic for the same id', () => {
    expect(keyframeNameForPiece('wuxia_spectator_bout_ally')).toBe(
      keyframeNameForPiece('wuxia_spectator_bout_ally'),
    );
  });

  it('produces distinct names for two different pieces in the same call, derived from id content not array order', () => {
    const input: CombatMotionInput = {
      tickMillis: 100,
      tracks: [
        track('ally_1', [{ x: 20, y: 50 }, { x: 40, y: 50 }]),
        track('ally_2', [{ x: 60, y: 50 }, { x: 70, y: 50 }]),
      ],
    };
    const result = buildCombatMotionCss(input);
    const nameAlly1 = result.keyframeNames.get('ally_1')!;
    const nameAlly2 = result.keyframeNames.get('ally_2')!;
    expect(nameAlly1).not.toBe(nameAlly2);
    expect(nameAlly1).toBe(keyframeNameForPiece('ally_1'));
    expect(nameAlly2).toBe(keyframeNameForPiece('ally_2'));
  });
});

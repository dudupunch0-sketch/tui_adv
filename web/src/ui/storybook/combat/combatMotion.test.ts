import { describe, expect, it } from 'vitest';

import type { CombatSpectatorCue, HexCoord } from '../../../core/types';
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

// WP3 helper: like `track`, but each point may carry a cue set / facing so
// tests can pin the cue presentation grammar. `facing` is now a raw hex
// direction (`{ q, r }`, WP1's §4-1 rename) rather than `{ x, y }`.
function cueTrack(
  pieceId: string,
  points: Array<{
    x: number;
    y: number;
    cues?: CombatSpectatorCue[];
    facing?: HexCoord;
  }>,
): PieceMotionTrack {
  return { pieceId, frames: points };
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

  it('a 10-frame / 100ms fixture sums to exactly 900ms (explicit verification number)', () => {
    const points = Array.from({ length: 10 }, (_, i) => ({ x: i * 5, y: 50 }));
    const input: CombatMotionInput = { tickMillis: 100, tracks: [track('ally_1', points)] };
    const result = buildCombatMotionCss(input);
    expect(result.durationMillis).toBe(900);
    expect(result.css).toContain('900ms linear');
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

// ---------------------------------------------------------------------------
// Wave 3 Step 1d-3 — WP3: cue presentation grammar (정본 13's 5-cue
// vocabulary). See the module-level "WP3" comment in combatMotion.ts for why
// `balance_broken` gets a translate wobble rather than the `rotate` tilt
// §4-3's table suggests (I9 — a Hard invariant — restricts animated
// properties to translate/opacity/filter; this is a deliberate,
// explicitly-reported deviation from that non-invariant table).
// ---------------------------------------------------------------------------
describe('buildCombatMotionCss — WP3 cue grammar: attack lunges toward facing, then returns', () => {
  it('attack_lunge_direction_follows_the_hex_facing: the lunge direction is the flat-top screen vector for the hex facing, not the raw q/r pair', () => {
    const input: CombatMotionInput = {
      tickMillis: 100,
      tracks: [cueTrack('ally_1', [{ x: 0, y: 50, cues: ['attack'], facing: { q: 1, r: 0 } }, { x: 10, y: 50 }])],
    };
    const { css } = buildCombatMotionCss(input);
    // natural midpoint (linear interp of -10 -> 0) is -5. Facing (q=1, r=0)
    // is one of `HexCoord::NEIGHBOR_DIRECTIONS` (flat-top axial "east"); its
    // screen vector via §4-2's formula (px = 1.5q, py = sqrt(3)*(r + q/2)) is
    // (1.5, 0.8660254), which normalizes to (0.8660254, 0.5) and scales by
    // the fixed lunge magnitude (4) to (3.4641016, 2). This is a changed
    // expected value from the pre-hex `facing: { x: 1, y: 0 }` fixture
    // (which used to add +4 on x only, since a cartesian unit vector along x
    // needs no axis conversion) — the hex direction (1, 0) is not a screen
    // unit vector, so both components of the lunge now carry contribution.
    expect(css).toMatch(/50% \{ translate: calc\(-50% \+ -1\.5359cqw\) calc\(-50% \+ 2cqh\)/);
  });

  it('zero_facing_still_produces_no_lunge: omits the lunge stop entirely when the hex facing is (q: 0, r: 0) — never guesses a direction', () => {
    // Rust now rejects a zero facing before this ever runs (T1-b1), but this
    // renderer keeps its own guard regardless — it does not become code that
    // trusts its input.
    const input: CombatMotionInput = {
      tickMillis: 100,
      tracks: [cueTrack('ally_1', [{ x: 0, y: 50, cues: ['attack'], facing: { q: 0, r: 0 } }, { x: 10, y: 50 }])],
    };
    const { css } = buildCombatMotionCss(input);
    expect(css).not.toMatch(/50% \{/);
  });

  it('omits the lunge stop when facing is absent altogether', () => {
    const input: CombatMotionInput = {
      tickMillis: 100,
      tracks: [cueTrack('ally_1', [{ x: 0, y: 50, cues: ['attack'] }, { x: 10, y: 50 }])],
    };
    const { css } = buildCombatMotionCss(input);
    expect(css).not.toMatch(/50% \{/);
  });

  it('never inserts a lunge stop for the very last frame (no next tick to return within)', () => {
    const input: CombatMotionInput = {
      tickMillis: 100,
      tracks: [
        cueTrack('ally_1', [
          { x: 0, y: 50 },
          { x: 10, y: 50, cues: ['attack'], facing: { q: 1, r: 0 } },
        ]),
      ],
    };
    const { css } = buildCombatMotionCss(input);
    expect(css).not.toMatch(/50% \{/);
  });
});

describe('buildCombatMotionCss — WP3 cue grammar: evade nudges laterally (board Y axis) then returns', () => {
  it('inserts a midpoint stop with a Y-only offset, independent of facing', () => {
    const input: CombatMotionInput = {
      tickMillis: 100,
      tracks: [cueTrack('ally_1', [{ x: 20, y: 20, cues: ['evade'] }, { x: 20, y: 40 }])],
    };
    const { css } = buildCombatMotionCss(input);
    // baseDy: k=0 -> 20-40=-20, k=1 -> 0. natural midpoint = -10. evade adds
    // -4 more (a fixed, symmetric presentation constant) -> -14. dx is
    // untouched (evade never touches the ally/enemy axis).
    expect(css).toMatch(/50% \{ translate: calc\(-50% \+ 0cqw\) calc\(-50% \+ -14cqh\)/);
  });
});

describe('buildCombatMotionCss — WP3 cue grammar: hit is a damped two-beat judder, not a direction', () => {
  it('inserts two extra stops (30% and 65% of the interval) for a hit cue', () => {
    const input: CombatMotionInput = {
      tickMillis: 100,
      tracks: [cueTrack('ally_1', [{ x: 20, y: 50, cues: ['hit'] }, { x: 20, y: 50 }])],
    };
    const { css } = buildCombatMotionCss(input);
    expect(css).toMatch(/30% \{/);
    expect(css).toMatch(/65% \{/);
  });

  it('does not use piece.facing for the hit direction (only attack is allowed to)', () => {
    const withFacing: CombatMotionInput = {
      tickMillis: 100,
      tracks: [cueTrack('ally_1', [{ x: 20, y: 50, cues: ['hit'], facing: { q: 1, r: 0 } }, { x: 20, y: 50 }])],
    };
    const withoutFacing: CombatMotionInput = {
      tickMillis: 100,
      tracks: [cueTrack('ally_1', [{ x: 20, y: 50, cues: ['hit'] }, { x: 20, y: 50 }])],
    };
    expect(buildCombatMotionCss(withFacing).css).toBe(buildCombatMotionCss(withoutFacing).css);
  });
});

describe('buildCombatMotionCss — WP3 cue grammar: balance_broken is a wobble plus a tilt', () => {
  it('alternates a small translate offset by tick parity while the cue is present, at the real tick stops (not a lunge-return)', () => {
    const input: CombatMotionInput = {
      tickMillis: 100,
      tracks: [
        cueTrack('ally_1', [
          { x: 20, y: 50, cues: ['balance_broken'] },
          { x: 20, y: 50, cues: ['balance_broken'] },
          { x: 20, y: 50 },
        ]),
      ],
    };
    const { css } = buildCombatMotionCss(input);
    expect(css).toMatch(/0% \{ translate: calc\(-50% \+ 1\.5cqw\)/);
    expect(css).toMatch(/50% \{ translate: calc\(-50% \+ -1\.5cqw\)/);
    expect(css).toMatch(/100% \{ translate: calc\(-50% \+ 0cqw\)/);
  });

  it('tilts by alternating degrees while the cue is present (정본 13: 흔들림/기울어짐)', () => {
    const input: CombatMotionInput = {
      tickMillis: 100,
      tracks: [
        cueTrack('ally_1', [
          { x: 20, y: 50, cues: ['balance_broken'] },
          { x: 20, y: 50, cues: ['balance_broken'] },
          { x: 20, y: 50 },
        ]),
      ],
    };
    const { css } = buildCombatMotionCss(input);
    expect(css).toMatch(/0% \{[^}]*rotate: 7deg;/);
    expect(css).toMatch(/50% \{[^}]*rotate: -7deg;/);
    // 기울지 않은 stop도 0deg를 명시해야 한쪽으로 기운 채 남지 않는다.
    expect(css).toMatch(/100% \{[^}]*rotate: 0deg;/);
  });

  it('emits no rotate at all when no frame carries balance_broken', () => {
    const input: CombatMotionInput = {
      tickMillis: 100,
      tracks: [
        cueTrack('ally_1', [
          { x: 20, y: 50, cues: ['hit'] },
          { x: 30, y: 50 },
        ]),
      ],
    };
    expect(buildCombatMotionCss(input).css).not.toMatch(/rotate/);
  });
});

describe('buildCombatMotionCss — dimming must not override the static [data-active=false] dim', () => {
  it('omits opacity/filter entirely when no frame carries incapacitated', () => {
    const input: CombatMotionInput = {
      tickMillis: 100,
      tracks: [
        cueTrack('ally_1', [
          { x: 20, y: 50, cues: ['attack'], facing: { q: 1, r: 0 } },
          { x: 30, y: 50 },
        ]),
      ],
    };
    const { css } = buildCombatMotionCss(input);
    // `opacity: 1`을 매 stop에 내보내면 재생 내내 정적 감광을 덮어써서
    // 비참전 말이 멀쩡하게 보인다.
    expect(css).not.toMatch(/opacity:/);
    expect(css).not.toMatch(/filter:/);
  });
});

describe('buildCombatMotionCss — WP3 cue grammar: incapacitated dims opacity/filter, following only the current frame', () => {
  it('reuses the exact 1d-2 dimming values (0.55 opacity, saturate(0.4) filter)', () => {
    const input: CombatMotionInput = {
      tickMillis: 100,
      tracks: [cueTrack('ally_1', [{ x: 20, y: 50 }, { x: 20, y: 50, cues: ['incapacitated'] }])],
    };
    const { css } = buildCombatMotionCss(input);
    expect(css).toMatch(/100% \{[^}]*opacity: 0\.55;[^}]*filter: saturate\(0\.4\);/);
    expect(css).toMatch(/0% \{[^}]*opacity: 1;[^}]*filter: none;/);
  });

  it('does not sustain the dim after a later frame stops listing the cue (never infers persistence)', () => {
    const input: CombatMotionInput = {
      tickMillis: 100,
      tracks: [
        cueTrack('ally_1', [
          { x: 20, y: 50, cues: ['incapacitated'] },
          { x: 20, y: 50 },
        ]),
      ],
    };
    const { css } = buildCombatMotionCss(input);
    expect(css).toMatch(/0% \{[^}]*opacity: 0\.55;/);
    expect(css).toMatch(/100% \{[^}]*opacity: 1;/);
  });
});

describe('buildCombatMotionCss — WP3: only compositor-thread properties (I9)', () => {
  it('never emits a layout property (left/top/width/height/margin) or scale, even for a fully cue-laden fixture', () => {
    const input: CombatMotionInput = {
      tickMillis: 100,
      tracks: [
        cueTrack('ally_1', [
          { x: 20, y: 50, cues: ['attack', 'hit', 'evade', 'balance_broken', 'incapacitated'], facing: { q: 1, r: 1 } },
          { x: 30, y: 40 },
        ]),
      ],
    };
    const { css } = buildCombatMotionCss(input);
    expect(css).not.toMatch(/[^-]\bleft:/);
    expect(css).not.toMatch(/[^-]\btop:/);
    expect(css).not.toMatch(/\bwidth:/);
    expect(css).not.toMatch(/\bheight:/);
    expect(css).not.toMatch(/\bmargin/);
    expect(css).not.toMatch(/\bscale\b/);
  });

  it('escapes a newline in a piece id so the attribute selector cannot break the stylesheet', () => {
    const input: CombatMotionInput = {
      tickMillis: 100,
      tracks: [
        cueTrack('ally\n_1', [
          { x: 20, y: 50 },
          { x: 30, y: 50 },
        ]),
      ],
    };
    const { css } = buildCombatMotionCss(input);
    const selectorLine = css.split('\n').find((line) => line.includes('data-piece-id'));
    expect(selectorLine).toBeDefined();
    expect(selectorLine).toContain('\\A ');
    expect(selectorLine).not.toContain('ally\n');
  });
});

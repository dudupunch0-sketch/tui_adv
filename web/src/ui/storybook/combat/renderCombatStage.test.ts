import { describe, expect, it } from 'vitest';

import type {
  CombatSpectatorCue,
  CombatSpectatorFrame,
  CombatSpectatorPiece,
  CombatSpectatorView,
} from '../../../core/types';
import { renderCombatBoard } from './renderCombatStage';

function piece(overrides: Partial<CombatSpectatorPiece> = {}): CombatSpectatorPiece {
  return {
    id: 'ally_1',
    side: 'ally',
    position: { x: 0, y: 0 },
    facing: { x: 1, y: 0 },
    active: true,
    cues: [],
    ...overrides,
  };
}

function frame(tick: number, pieces: CombatSpectatorPiece[]): CombatSpectatorFrame {
  return { tick, pieces };
}

function view(overrides: Partial<CombatSpectatorView> = {}): CombatSpectatorView {
  return {
    simulation_version: 'v-test',
    resolution_fingerprint: 'res-fp',
    tick_millis: 100,
    frames: [],
    core_log: [],
    full_log: [],
    fingerprint: 'view-fp',
    ...overrides,
  };
}

describe('renderCombatBoard', () => {
  it('renders only the last frame — earlier frame coordinates never appear', () => {
    const html = renderCombatBoard(
      view({
        frames: [
          frame(0, [piece({ id: 'ally_1', position: { x: 99, y: 99 } })]),
          frame(3, [piece({ id: 'ally_1', position: { x: 5, y: 5 } })]),
        ],
      }),
    );
    // Only frame 3's piece is a single piece so span === 0 -> 50% center;
    // the point is the earlier frame's distinguishing coordinate (99) never
    // leaks into the projection math anywhere in the output.
    expect(html).not.toContain('99');
    expect(html).toContain('data-piece-id="ally_1"');
  });

  it('centers pieces at 50% when the coordinate span is zero, with no NaN/Infinity', () => {
    const html = renderCombatBoard(
      view({
        frames: [
          frame(1, [
            piece({ id: 'ally_1', position: { x: 7, y: 7 } }),
            piece({ id: 'ally_2', position: { x: 7, y: 7 }, side: 'ally' }),
          ]),
        ],
      }),
    );
    expect(html).toContain('--piece-x: 50%');
    expect(html).toContain('--piece-y: 50%');
    expect(html).not.toContain('NaN');
    expect(html).not.toContain('Infinity');
  });

  it('shows all 5 cue symbols with matching data-cue attributes', () => {
    const cues: CombatSpectatorCue[] = ['attack', 'hit', 'evade', 'balance_broken', 'incapacitated'];
    const html = renderCombatBoard(
      view({ frames: [frame(1, [piece({ cues })])] }),
    );
    for (const cue of cues) {
      expect(html).toContain(`data-cue-${cue}="true"`);
      expect(html).toContain(`data-cue="${cue}"`);
    }
    // The 5 glyphs from I11's cue table.
    expect(html).toContain('攻');
    expect(html).toContain('打');
    expect(html).toContain('避');
    expect(html).toContain('傾');
    expect(html).toContain('倒');
  });

  it('handles an empty frames array without throwing, and says so', () => {
    expect(() => renderCombatBoard(view({ frames: [] }))).not.toThrow();
    const html = renderCombatBoard(view({ frames: [] }));
    expect(html).toContain('표시할 프레임이 없다');
  });

  it('handles a frame with zero pieces without throwing, and says so', () => {
    const html = renderCombatBoard(view({ frames: [frame(1, [])] }));
    expect(html).toContain('표시할 말이 없다');
  });

  it('includes every piece id, side, and coordinate in the semantic alternative table', () => {
    const html = renderCombatBoard(
      view({
        frames: [
          frame(2, [
            piece({ id: 'ally_1', side: 'ally', position: { x: 0, y: 3 } }),
            piece({ id: 'enemy_1', side: 'enemy', position: { x: 10, y: 3 } }),
          ]),
        ],
      }),
    );
    expect(html).toContain('<table');
    expect(html).toContain('ally_1');
    expect(html).toContain('enemy_1');
    expect(html).toContain('아군');
    expect(html).toContain('적군');
    expect(html).toContain('0');
    expect(html).toContain('10');
  });

  it('escapes a piece id containing markup', () => {
    const html = renderCombatBoard(
      view({ frames: [frame(1, [piece({ id: '<script>alert(1)</script>' })])] }),
    );
    expect(html).not.toContain('<script>alert(1)</script>');
    expect(html).toContain('&lt;script&gt;');
  });
});

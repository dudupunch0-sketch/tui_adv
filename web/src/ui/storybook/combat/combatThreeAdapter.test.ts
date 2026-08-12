import fixture from '../../../../../crates/escape-core/fixtures/combat/wuxia_combat_spectator_preview_bout.seed-2.combat.json';
import { describe, expect, it } from 'vitest';
import { adaptCombatForThree, axialToWorld, combatVisualSeedHex, type CombatAdapterDiagnostic } from './combatThreeAdapter';

const bounds = { minQ: 0, maxQ: 6, minR: 0, maxR: 5 };
const options = { boardBounds: bounds };

function combat(overrides: Record<string, unknown> = {}): Record<string, unknown> {
  return {
    view: {
      simulation_version: 'v3', resolution_fingerprint: 'resolution', fingerprint: 'view', tick_millis: 100,
      frames: [{ tick: 0, pieces: [{ id: 'ally', side: 'ally', position: { q: 1, r: 0 }, facing: { q: 1, r: 0 }, active: true, cues: ['attack'] }] }],
      ...overrides,
    },
  };
}

function codes(result: { diagnostics: readonly CombatAdapterDiagnostic[] }): string[] { return result.diagnostics.map((diagnostic) => `${diagnostic.code}@${diagnostic.path}`); }

describe('combatThreeAdapter', () => {
  it('returns absent before validating options', () => {
    expect(adaptCombatForThree(undefined, { boardBounds: null as never })).toEqual({ kind: 'absent', diagnostics: [] });
    expect(adaptCombatForThree(null, { boardBounds: null as never })).toEqual({ kind: 'absent', diagnostics: [] });
  });

  it('normalizes the producer-owned fixture without changing order', () => {
    const result = adaptCombatForThree(fixture, options);
    expect(result.kind).toBe('ready');
    if (result.kind !== 'ready') return;
    expect(result.replay.frames).toHaveLength(8);
    expect(result.replay.frames[0]?.pieces.map((piece) => piece.id)).toEqual(['wuxia_spectator_bout_ally', 'wuxia_spectator_bout_challenger']);
    expect(result.replay.frames[0]?.pieces[0]?.cues[0]).toMatchObject({ type: 'attack', ordinal: 0, seedHex: expect.stringMatching(/^[0-9a-f]{16}$/) });
  });

  it('computes the canonical JSON UTF-8 FNV-1a64 golden', () => {
    expect(combatVisualSeedHex(['v3', '1978f44e79dd23d1', 'bb9240dfbc6e72b0', 1, 'wuxia_spectator_bout_ally', 'attack', 0])).toBe('fba89b551ab6959d');
  });

  it('projects flat-top axial coordinates exactly', () => {
    expect(axialToWorld({ q: 2, r: 3 }, 4)).toEqual({ x: 12, z: 4 * Math.sqrt(3) * 4 });
  });

  it('accumulates sibling errors in canonical input order and deduplicates', () => {
    const result = adaptCombatForThree(combat({ simulation_version: '', resolution_fingerprint: '', fingerprint: '', tick_millis: 0, frames: [{ tick: -1, pieces: [{ id: '', side: 'bad', position: { q: 1.2, r: 0 }, facing: { q: 0, r: 0 }, active: 'yes', cues: ['bad', 'bad'] }] }] }), options);
    expect(result.kind).toBe('fallback');
    expect(codes(result)).toEqual([
      'INVALID_SIMULATION_VERSION@$.view.simulation_version', 'INVALID_RESOLUTION_FINGERPRINT@$.view.resolution_fingerprint', 'INVALID_VIEW_FINGERPRINT@$.view.fingerprint', 'INVALID_TICK_MILLIS@$.view.tick_millis',
      'INVALID_FRAME_TICK@$.view.frames[0].tick', 'INVALID_PIECE_ID@$.view.frames[0].pieces[0].id', 'INVALID_POSITION@$.view.frames[0].pieces[0].position', 'INVALID_FACING@$.view.frames[0].pieces[0].facing', 'INVALID_SIDE@$.view.frames[0].pieces[0].side', 'INVALID_ACTIVE@$.view.frames[0].pieces[0].active', 'INVALID_CUE@$.view.frames[0].pieces[0].cues[0]', 'INVALID_CUE@$.view.frames[0].pieces[0].cues[1]',
    ]);
  });

  it('stops only invalid containers and preserves valid siblings', () => {
    const result = adaptCombatForThree(combat({ frames: [{ tick: 0, pieces: 'bad' }, { tick: 1, pieces: [] }] }), options);
    expect(result.kind).toBe('fallback');
    expect(codes(result)).toEqual(['INVALID_PIECES@$.view.frames[0].pieces']);
  });

  it('rejects invalid bounds without producing position warnings', () => {
    const result = adaptCombatForThree(combat({ frames: [{ tick: 0, pieces: [{ id: 'a', side: 'ally', position: { q: 99, r: 99 }, facing: { q: 1, r: 0 }, active: true, cues: [] }] }] }), { boardBounds: { minQ: 4, maxQ: 1, minR: 0, maxR: 5 } });
    expect(result.kind).toBe('fallback');
    expect(codes(result)).toEqual(['INVALID_BOARD_BOUNDS@$options.boardBounds']);
  });

  it('reports duplicate ids and non-monotonic ticks in deterministic paths', () => {
    const result = adaptCombatForThree(combat({ frames: [{ tick: 2, pieces: [{ id: 'a', side: 'ally', position: { q: 1, r: 0 }, facing: { q: 1, r: 0 }, active: true, cues: [] }, { id: 'a', side: 'enemy', position: { q: 2, r: 0 }, facing: { q: -1, r: 0 }, active: true, cues: [] }] }, { tick: 1, pieces: [] }] }), options);
    expect(result.kind).toBe('fallback');
    expect(codes(result)).toEqual(['DUPLICATE_PIECE_ID@$.view.frames[0].pieces[1].id', 'NON_MONOTONIC_FRAME_TICK@$.view.frames[1].tick']);
  });

  it('accepts empty arrays and more than twelve participants', () => {
    const pieces = Array.from({ length: 13 }, (_, index) => ({ id: `p${index}`, side: 'ally', position: { q: index, r: 0 }, facing: { q: 1, r: 0 }, active: true, cues: [] }));
    const result = adaptCombatForThree(combat({ frames: [{ tick: 0, pieces }, { tick: 1 }] }), options);
    expect(result.kind).toBe('ready');
    if (result.kind === 'ready') expect(result.replay.frames[0]?.pieces).toHaveLength(13);
  });

  it('reports warnings last without clamping or dropping positions', () => {
    const result = adaptCombatForThree(combat({ frames: [{ tick: 0, pieces: [
      { id: 'a', side: 'ally', position: { q: 7, r: 0 }, facing: { q: 1, r: 0 }, active: true, cues: [] },
      { id: 'b', side: 'enemy', position: { q: 7, r: 0 }, facing: { q: -1, r: 0 }, active: false, cues: [] },
    ] }] }), options);
    expect(result.kind).toBe('ready');
    if (result.kind === 'ready') expect(result.replay.frames[0]?.pieces.map((piece) => piece.position.q)).toEqual([7, 7]);
    expect(codes(result)).toEqual(['OUT_OF_BOUNDS@$.view.frames[0].pieces[0].position', 'OUT_OF_BOUNDS@$.view.frames[0].pieces[1].position', 'DUPLICATE_OCCUPANCY@$.view.frames[0].pieces[1].position']);
  });

  it('does not mutate input', () => {
    const input = combat();
    const before = JSON.stringify(input);
    adaptCombatForThree(input, options);
    expect(JSON.stringify(input)).toBe(before);
  });
});

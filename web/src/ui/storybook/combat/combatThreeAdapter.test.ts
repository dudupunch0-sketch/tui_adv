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
    expect(result.replay).toMatchObject({ simulationVersion: 'v3', resolutionFingerprint: '1978f44e79dd23d1', viewFingerprint: 'bb9240dfbc6e72b0', tickMillis: 100 });
    expect(result.replay.frames.map((frame) => frame.tick)).toEqual([1, 2, 3, 4, 5, 6, 7, 8]);
    expect(result.replay.frames[0]?.pieces.map((piece) => piece.id)).toEqual(['wuxia_spectator_bout_ally', 'wuxia_spectator_bout_challenger']);
    expect(result.replay.frames[0]?.pieces[0]?.cues).toMatchObject([{ type: 'attack', ordinal: 0, seedHex: 'fba89b551ab6959d' }, { type: 'hit', ordinal: 1 }]);
    expect(result.replay.frames[0]?.pieces[0]?.position).toEqual({ q: 1, r: 0 });
    expect(axialToWorld(result.replay.frames[0]?.pieces[0]?.position ?? { q: 0, r: 0 }, 1)).toEqual({ x: 1.5, z: Math.sqrt(3) / 2 });
  });

  it('computes the canonical JSON UTF-8 FNV-1a64 golden', () => {
    const tuple = ['v3', '1978f44e79dd23d1', 'bb9240dfbc6e72b0', 1, 'wuxia_spectator_bout_ally', 'attack', 0] as const;
    expect(JSON.stringify(tuple)).toBe('["v3","1978f44e79dd23d1","bb9240dfbc6e72b0",1,"wuxia_spectator_bout_ally","attack",0]');
    expect(Array.from(new TextEncoder().encode(JSON.stringify(tuple))).map((byte) => byte.toString(16).padStart(2, '0')).join('')).toBe('5b227633222c2231393738663434653739646432336431222c2262623932343064666263366537326230222c312c2277757869615f737065637461746f725f626f75745f616c6c79222c2261747461636b222c305d');
    expect(combatVisualSeedHex(tuple)).toBe('fba89b551ab6959d');
  });

  it('changes only the seed for the cue ordinal that changed', () => {
    const first = adaptCombatForThree(combat({ frames: [{ tick: 0, pieces: [{ id: 'ally', side: 'ally', position: { q: 1, r: 0 }, facing: { q: 1, r: 0 }, active: true, cues: ['attack', 'hit'] }] }] }), options);
    const second = adaptCombatForThree(combat({ frames: [{ tick: 0, pieces: [{ id: 'ally', side: 'ally', position: { q: 1, r: 0 }, facing: { q: 1, r: 0 }, active: true, cues: ['evade', 'hit'] }] }] }), options);
    expect(first.kind).toBe('ready');
    expect(second.kind).toBe('ready');
    if (first.kind !== 'ready' || second.kind !== 'ready') return;
    const a = first.replay.frames[0]?.pieces[0]?.cues ?? [];
    const b = second.replay.frames[0]?.pieces[0]?.cues ?? [];
    expect(a[0]?.seedHex).not.toBe(b[0]?.seedHex);
    expect(a[0]?.ordinal).toBe(b[0]?.ordinal);
    expect(a[0]?.seedHex).toBe(combatVisualSeedHex(['v3', 'resolution', 'view', 0, 'ally', 'attack', 0]));
    expect(a[1]?.seedHex).toBe(b[1]?.seedHex);
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

  it.each([
    ['frames', { frames: undefined }, 'INVALID_FRAMES', '$.view.frames'],
    ['pieces', { frames: [{ tick: 0, pieces: null }] }, 'INVALID_PIECES', '$.view.frames[0].pieces'],
    ['cues', { frames: [{ tick: 0, pieces: [{ id: 'a', side: 'ally', position: { q: 1, r: 0 }, facing: { q: 1, r: 0 }, active: true, cues: null }] }] }, 'INVALID_CUES', '$.view.frames[0].pieces[0].cues'],
  ])('treats present undefined/null containers as hard errors (%s)', (_label, overrides, code, path) => {
    const result = adaptCombatForThree(combat(overrides), options);
    expect(result.kind).toBe('fallback');
    expect(codes(result)).toContain(`${code}@${path}`);
  });

  it('keeps warnings after hard errors and returns no replay', () => {
    const result = adaptCombatForThree(combat({ tick_millis: 0, frames: [{ tick: 0, pieces: [{ id: 'a', side: 'ally', position: { q: 7, r: 0 }, facing: { q: 1, r: 0 }, active: true, cues: [] }, { id: 'b', side: 'enemy', position: { q: 7, r: 0 }, facing: { q: -1, r: 0 }, active: true, cues: [] }] }] }), options);
    expect(result.kind).toBe('fallback');
    expect(result.diagnostics.at(-1)).toMatchObject({ code: 'DUPLICATE_OCCUPANCY', severity: 'warning', path: '$.view.frames[0].pieces[1].position' });
    expect(result).not.toHaveProperty('replay');
  });

  it('ignores malformed unconsumed logs and report', () => {
    const result = adaptCombatForThree({ ...combat(), core_log: null, full_log: 'bad', report: 12 }, options);
    expect(result.kind).toBe('ready');
  });

  it('covers every hard diagnostic with sanitized non-throwing results', () => {
    const cases: Array<[string, unknown, string, string]> = [
      ['root', [], 'INVALID_COMBAT_OBJECT', '$'], ['view', { view: [] }, 'INVALID_VIEW_OBJECT', '$.view'],
      ['frame', combat({ frames: [null] }), 'INVALID_FRAME', '$.view.frames[0]'], ['tick', combat({ frames: [{ tick: -1 }] }), 'INVALID_FRAME_TICK', '$.view.frames[0].tick'],
      ['nonmonotonic', combat({ frames: [{ tick: 1 }, { tick: 1 }] }), 'NON_MONOTONIC_FRAME_TICK', '$.view.frames[1].tick'], ['pieces', combat({ frames: [{ tick: 0, pieces: {} }] }), 'INVALID_PIECES', '$.view.frames[0].pieces'],
      ['piece', combat({ frames: [{ tick: 0, pieces: [null] }] }), 'INVALID_PIECE', '$.view.frames[0].pieces[0]'], ['id', combat({ frames: [{ tick: 0, pieces: [{ id: '' }] }] }), 'INVALID_PIECE_ID', '$.view.frames[0].pieces[0].id'],
      ['position', combat({ frames: [{ tick: 0, pieces: [{ id: 'a', position: {} }] }] }), 'INVALID_POSITION', '$.view.frames[0].pieces[0].position'], ['facing', combat({ frames: [{ tick: 0, pieces: [{ id: 'a', position: { q: 0, r: 0 }, facing: { q: 0, r: 0 } }] }] }), 'INVALID_FACING', '$.view.frames[0].pieces[0].facing'],
      ['side', combat({ frames: [{ tick: 0, pieces: [{ id: 'a', position: { q: 0, r: 0 }, facing: { q: 1, r: 0 }, side: 'x' }] }] }), 'INVALID_SIDE', '$.view.frames[0].pieces[0].side'], ['active', combat({ frames: [{ tick: 0, pieces: [{ id: 'a', position: { q: 0, r: 0 }, facing: { q: 1, r: 0 }, side: 'ally', active: 1 }] }] }), 'INVALID_ACTIVE', '$.view.frames[0].pieces[0].active'],
      ['cue', combat({ frames: [{ tick: 0, pieces: [{ id: 'a', position: { q: 0, r: 0 }, facing: { q: 1, r: 0 }, side: 'ally', active: true, cues: ['x'] }] }] }), 'INVALID_CUE', '$.view.frames[0].pieces[0].cues[0]'], ['options', combat(), 'INVALID_BOARD_BOUNDS', '$options.boardBounds'],
    ];
    for (const [_label, input, code, path] of cases) {
      const result = adaptCombatForThree(input, _label === 'options' ? { boardBounds: { minQ: 4, maxQ: 1, minR: 0, maxR: 5 } } : options);
      expect(result.kind).toBe('fallback');
      expect(result.diagnostics.some((item) => item.code === code && item.severity === 'error' && item.path === path)).toBe(true);
      expect(result.diagnostics.every((item) => Object.keys(item).sort().join(',') === 'code,path,severity')).toBe(true);
    }
  });

  it('preserves frame, piece, and cue input order', () => {
    const result = adaptCombatForThree(combat({ frames: [{ tick: 2, pieces: [{ id: 'b', side: 'enemy', position: { q: 2, r: 0 }, facing: { q: -1, r: 0 }, active: true, cues: ['hit', 'evade'] }, { id: 'a', side: 'ally', position: { q: 1, r: 0 }, facing: { q: 1, r: 0 }, active: true, cues: ['attack'] }] }, { tick: 3, pieces: [] }] }), options);
    expect(result.kind).toBe('ready');
    if (result.kind === 'ready') expect(result.replay.frames.map((frame) => [frame.tick, frame.pieces.map((piece) => [piece.id, piece.cues.map((cue) => cue.type)])])).toEqual([[2, [['b', ['hit', 'evade']], ['a', ['attack']]]], [3, []]]);
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

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

  it('contains hostile access traps at their nearest contract boundary', () => {
    const throwingOptions = { get boardBounds(): never { throw new Error('secret'); } } as unknown as { boardBounds: typeof bounds };
    expect(() => adaptCombatForThree(combat(), throwingOptions)).not.toThrow();
    expect(codes(adaptCombatForThree(combat(), throwingOptions))).toEqual(['INVALID_BOARD_BOUNDS@$options.boardBounds']);
    const throwingView = { get view(): never { throw new Error('secret'); } };
    expect(() => adaptCombatForThree(throwingView, options)).not.toThrow();
    expect(codes(adaptCombatForThree(throwingView, options))).toEqual(['INVALID_VIEW_OBJECT@$.view']);
    const throwingFrames = { view: { get frames(): never { throw new Error('secret'); } } };
    expect(codes(adaptCombatForThree(throwingFrames, options))).toEqual(['INVALID_FRAMES@$.view.frames']);
    const throwingPieces = { view: { frames: [{ get pieces(): never { throw new Error('secret'); } }] } };
    expect(codes(adaptCombatForThree(throwingPieces, options))).toEqual(['INVALID_PIECES@$.view.frames[0].pieces']);
    const throwingCues = { view: { frames: [{ pieces: [{ get cues(): never { throw new Error('secret'); } }] }] } };
    expect(codes(adaptCombatForThree(throwingCues, options))).toEqual(['INVALID_CUES@$.view.frames[0].pieces[0].cues']);
    const revokedBounds = Proxy.revocable({ ...bounds }, {}); revokedBounds.revoke();
    expect(codes(adaptCombatForThree(combat(), { boardBounds: revokedBounds.proxy }))).toEqual(['INVALID_BOARD_BOUNDS@$options.boardBounds']);
    const revokedView = Proxy.revocable({ ...combat().view as Record<string, unknown> }, {}); revokedView.revoke();
    expect(codes(adaptCombatForThree({ view: revokedView.proxy }, options))).toEqual(['INVALID_VIEW_OBJECT@$.view']);
    const nestedPosition = { get q(): never { throw new Error('secret'); }, r: 0 };
    expect(codes(adaptCombatForThree(combat({ frames: [{ tick: 0, pieces: [{ id: 'a', side: 'ally', position: nestedPosition, facing: { q: 1, r: 0 }, active: true, cues: [] }] }] }), options))).toEqual(['INVALID_POSITION@$.view.frames[0].pieces[0].position']);
  });

  it('diagnoses sparse frame, piece, and cue holes', () => {
    const frames: unknown[] = []; frames.length = 1;
    const pieces: unknown[] = []; pieces.length = 1;
    const cues: unknown[] = []; cues.length = 1;
    const frameHole = adaptCombatForThree({ view: { ...(combat().view as Record<string, unknown>), frames } }, options);
    expect(codes(frameHole)).toContain('INVALID_FRAME@$.view.frames[0]');
    const pieceHole = adaptCombatForThree(combat({ frames: [{ tick: 0, pieces }] }), options);
    expect(codes(pieceHole)).toContain('INVALID_PIECE@$.view.frames[0].pieces[0]');
    const cueHole = adaptCombatForThree(combat({ frames: [{ tick: 0, pieces: [{ id: 'a', side: 'ally', position: { q: 1, r: 0 }, facing: { q: 1, r: 0 }, active: true, cues }] }] }), options);
    expect(codes(cueHole)).toContain('INVALID_CUE@$.view.frames[0].pieces[0].cues[0]');
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

  it('preserves every producer frame, participant field, cue ordinal, and projection', () => {
    const result = adaptCombatForThree(fixture, options);
    expect(result.kind).toBe('ready');
    if (result.kind !== 'ready') return;
    expect(result.replay.frames.map((frame) => ({ tick: frame.tick, pieces: frame.pieces.map((piece) => ({ id: piece.id, side: piece.side, position: piece.position, facing: piece.facing, active: piece.active, cues: piece.cues.map((cue) => ({ type: cue.type, ordinal: cue.ordinal })) })) }))).toEqual(fixture.view.frames.map((frame) => ({ tick: frame.tick, pieces: frame.pieces.map((piece) => ({ id: piece.id, side: piece.side, position: piece.position, facing: piece.facing, active: piece.active, cues: piece.cues.map((type, ordinal) => ({ type, ordinal })) })) })));
    for (const frame of fixture.view.frames) for (const piece of frame.pieces) {
      const normalized = result.replay.frames.find((candidate) => candidate.tick === frame.tick)?.pieces.find((candidate) => candidate.id === piece.id);
      expect(axialToWorld(normalized?.position ?? piece.position, 1)).toEqual(axialToWorld(piece.position, 1));
    }
  });

  it('computes the canonical JSON UTF-8 FNV-1a64 golden', () => {
    const tuple = ['v3', '1978f44e79dd23d1', 'bb9240dfbc6e72b0', 1, 'wuxia_spectator_bout_ally', 'attack', 0] as const;
    expect(JSON.stringify(tuple)).toBe('["v3","1978f44e79dd23d1","bb9240dfbc6e72b0",1,"wuxia_spectator_bout_ally","attack",0]');
    expect(Array.from(new TextEncoder().encode(JSON.stringify(tuple))).map((byte) => byte.toString(16).padStart(2, '0')).join('')).toBe('5b227633222c2231393738663434653739646432336431222c2262623932343064666263366537326230222c312c2277757869615f737065637461746f725f626f75745f616c6c79222c2261747461636b222c305d');
    expect(combatVisualSeedHex(tuple)).toBe('fba89b551ab6959d');
  });

  it('changes only the seed for the same attack cue when its ordinal changes', () => {
    const first = adaptCombatForThree(combat({ frames: [{ tick: 0, pieces: [{ id: 'ally', side: 'ally', position: { q: 1, r: 0 }, facing: { q: 1, r: 0 }, active: true, cues: ['attack'] }, { id: 'enemy', side: 'enemy', position: { q: 2, r: 0 }, facing: { q: -1, r: 0 }, active: true, cues: ['hit'] }] }] }), options);
    const second = adaptCombatForThree(combat({ frames: [{ tick: 0, pieces: [{ id: 'ally', side: 'ally', position: { q: 1, r: 0 }, facing: { q: 1, r: 0 }, active: true, cues: ['evade', 'attack'] }, { id: 'enemy', side: 'enemy', position: { q: 2, r: 0 }, facing: { q: -1, r: 0 }, active: true, cues: ['hit'] }] }] }), options);
    expect(first.kind).toBe('ready');
    expect(second.kind).toBe('ready');
    if (first.kind !== 'ready' || second.kind !== 'ready') return;
    const originalAttack = first.replay.frames[0]?.pieces[0]?.cues[0];
    const shiftedAttack = second.replay.frames[0]?.pieces[0]?.cues[1];
    expect(originalAttack).toMatchObject({ type: 'attack', ordinal: 0 });
    expect(shiftedAttack).toMatchObject({ type: 'attack', ordinal: 1 });
    expect(originalAttack?.seedHex).not.toBe(shiftedAttack?.seedHex);
    expect(first.replay.frames[0]?.pieces[1]?.cues[0]?.seedHex).toBe(second.replay.frames[0]?.pieces[1]?.cues[0]?.seedHex);
  });

  it('projects flat-top axial coordinates exactly', () => {
    expect(axialToWorld({ q: 2, r: 3 }, 4)).toEqual({ x: 12, z: 4 * Math.sqrt(3) * 4 });
  });

  it('accumulates sibling errors in canonical input order', () => {
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
    const input = combat();
    const view = input.view as Record<string, unknown>;
    view.core_log = null;
    view.full_log = 'bad';
    const result = adaptCombatForThree({ ...input, report: 12 }, options);
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

  it('defaults each absent frames, pieces, and cues field independently', () => {
    const noFrames = adaptCombatForThree({ view: { simulation_version: 'v3', resolution_fingerprint: 'r', fingerprint: 'f', tick_millis: 100 } }, options);
    expect(noFrames.kind).toBe('ready');
    if (noFrames.kind === 'ready') expect(noFrames.replay.frames).toEqual([]);
    const noPieces = adaptCombatForThree(combat({ frames: [{ tick: 0 }] }), options);
    expect(noPieces.kind).toBe('ready');
    if (noPieces.kind === 'ready') expect(noPieces.replay.frames[0]?.pieces).toEqual([]);
    const noCues = adaptCombatForThree(combat({ frames: [{ tick: 0, pieces: [{ id: 'a', side: 'ally', position: { q: 1, r: 0 }, facing: { q: 1, r: 0 }, active: true }] }] }), options);
    expect(noCues.kind).toBe('ready');
    if (noCues.kind === 'ready') expect(noCues.replay.frames[0]?.pieces[0]?.cues).toEqual([]);
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


  it('handles revoked containers at exact boundaries', () => {
    const revoked = (value: object) => { const r = Proxy.revocable(value, {}); r.revoke(); return r.proxy; };
    const cases: Array<[unknown, string]> = [
      [{ view: { ...combat().view as Record<string, unknown>, frames: revoked([]) } }, 'INVALID_FRAMES@$.view.frames'],
      [{ view: { ...combat().view as Record<string, unknown>, frames: [revoked({})] } }, 'INVALID_FRAME@$.view.frames[0]'],
      [combat({ frames: [{ tick: 0, pieces: revoked([]) }] }), 'INVALID_PIECES@$.view.frames[0].pieces'],
      [combat({ frames: [{ tick: 0, pieces: [revoked({})] }] }), 'INVALID_PIECE@$.view.frames[0].pieces[0]'],
      [combat({ frames: [{ tick: 0, pieces: [{ id: 'a', side: 'ally', position: { q: 1, r: 0 }, facing: { q: 1, r: 0 }, active: true, cues: revoked([]) }] }] }), 'INVALID_CUES@$.view.frames[0].pieces[0].cues'],
    ];
    for (const [input, expected] of cases) {
      const result = adaptCombatForThree(input, options);
      expect(result.kind).toBe('fallback');
      expect(codes(result)).toContain(expected);
    }
  });

  it('maps array length traps to container diagnostics', () => {
    const trap = (value: object) => new Proxy(value, { get(target, property, receiver) { if (property === 'length') throw new Error('length'); return Reflect.get(target, property, receiver); } });
    expect(codes(adaptCombatForThree(combat({ frames: trap([]) }), options))).toContain('INVALID_FRAMES@$.view.frames');
    expect(codes(adaptCombatForThree(combat({ frames: [{ tick: 0, pieces: trap([]) }] }), options))).toContain('INVALID_PIECES@$.view.frames[0].pieces');
    expect(codes(adaptCombatForThree(combat({ frames: [{ tick: 0, pieces: [{ id: 'a', side: 'ally', position: { q: 1, r: 0 }, facing: { q: 1, r: 0 }, active: true, cues: trap([]) }] }] }), options))).toContain('INVALID_CUES@$.view.frames[0].pieces[0].cues');
  });

  it('reads stateful consumed getters exactly once', () => {
    let minQReads = 0; let versionReads = 0; let positionReads = 0; let qReads = 0;
    const board = { get minQ() { if (++minQReads > 1) throw new Error(); return 0; }, maxQ: 6, minR: 0, maxR: 5 };
    const position = { get q() { if (++qReads > 1) throw new Error(); return 1; }, r: 0 };
    const input = { view: { get simulation_version() { if (++versionReads > 1) throw new Error(); return 'v3'; }, resolution_fingerprint: 'r', fingerprint: 'f', tick_millis: 100, frames: [{ tick: 0, pieces: [{ id: 'a', side: 'ally', get position() { if (++positionReads > 1) throw new Error(); return position; }, facing: { q: 1, r: 0 }, active: true, cues: [] }] }] } };
    const result = adaptCombatForThree(input, { boardBounds: board });
    expect(result.kind).toBe('ready');
    expect(minQReads).toBe(1); expect(versionReads).toBe(1); expect(positionReads).toBe(1); expect(qReads).toBe(1);
  });
});

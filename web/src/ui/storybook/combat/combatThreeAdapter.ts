export interface CombatBoardBounds {
  minQ: number;
  maxQ: number;
  minR: number;
  maxR: number;
}

export interface CombatAdapterOptions { boardBounds: CombatBoardBounds }

export type CombatAdapterDiagnosticCode =
  | 'INVALID_COMBAT_OBJECT' | 'INVALID_VIEW_OBJECT'
  | 'INVALID_SIMULATION_VERSION' | 'INVALID_RESOLUTION_FINGERPRINT'
  | 'INVALID_VIEW_FINGERPRINT' | 'INVALID_TICK_MILLIS'
  | 'INVALID_FRAMES' | 'INVALID_FRAME' | 'INVALID_FRAME_TICK'
  | 'NON_MONOTONIC_FRAME_TICK' | 'INVALID_PIECES'
  | 'INVALID_PIECE' | 'INVALID_PIECE_ID' | 'DUPLICATE_PIECE_ID'
  | 'INVALID_POSITION' | 'INVALID_FACING' | 'INVALID_SIDE'
  | 'INVALID_ACTIVE' | 'INVALID_CUES' | 'INVALID_CUE'
  | 'INVALID_BOARD_BOUNDS' | 'OUT_OF_BOUNDS' | 'DUPLICATE_OCCUPANCY';

export interface CombatAdapterDiagnostic {
  code: CombatAdapterDiagnosticCode;
  severity: 'error' | 'warning';
  path: string;
}

export type NormalizedCombatCueType = 'attack' | 'hit' | 'evade' | 'balance_broken' | 'incapacitated';
export interface NormalizedCombatCoord { q: number; r: number }
export interface NormalizedCombatCue {
  type: NormalizedCombatCueType;
  ordinal: number;
  seedHex: string;
}
export interface NormalizedCombatPiece {
  id: string;
  side: 'ally' | 'enemy';
  position: NormalizedCombatCoord;
  facing: NormalizedCombatCoord;
  active: boolean;
  cues: readonly NormalizedCombatCue[];
}
export interface NormalizedCombatFrame {
  tick: number;
  pieces: readonly NormalizedCombatPiece[];
}
export interface NormalizedCombatReplay {
  simulationVersion: string;
  resolutionFingerprint: string;
  viewFingerprint: string;
  tickMillis: number;
  frames: readonly NormalizedCombatFrame[];
}

export type CombatAdapterResult =
  | { kind: 'absent'; diagnostics: readonly [] }
  | { kind: 'fallback'; diagnostics: readonly CombatAdapterDiagnostic[] }
  | { kind: 'ready'; replay: NormalizedCombatReplay; diagnostics: readonly CombatAdapterDiagnostic[] };

const CUES = new Set<NormalizedCombatCueType>(['attack', 'hit', 'evade', 'balance_broken', 'incapacitated']);
const FACING_DIRECTIONS = new Set(['1,0', '1,-1', '0,-1', '-1,0', '-1,1', '0,1']);
const FNV_OFFSET = 0xcbf29ce484222325n;
const FNV_PRIME = 0x100000001b3n;

type RecordValue = Record<string, unknown>;

function isRecord(value: unknown): value is RecordValue {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function isSafeInteger(value: unknown): value is number {
  return typeof value === 'number' && Number.isSafeInteger(value);
}

function nonEmptyString(value: unknown): value is string {
  return typeof value === 'string' && value.length > 0;
}

function diagnostic(code: CombatAdapterDiagnosticCode, severity: 'error' | 'warning', path: string): CombatAdapterDiagnostic {
  return { code, severity, path };
}

function addDiagnostic(
  diagnostics: CombatAdapterDiagnostic[],
  seen: Set<string>,
  code: CombatAdapterDiagnosticCode,
  severity: 'error' | 'warning',
  path: string,
): void {
  const key = `${code}|${path}`;
  if (!seen.has(key)) {
    seen.add(key);
    diagnostics.push(diagnostic(code, severity, path));
  }
}

function validBounds(value: unknown): value is CombatBoardBounds {
  if (!isRecord(value)) return false;
  return isSafeInteger(value.minQ) && isSafeInteger(value.maxQ)
    && isSafeInteger(value.minR) && isSafeInteger(value.maxR)
    && value.minQ <= value.maxQ && value.minR <= value.maxR;
}

function validCoord(value: unknown): value is NormalizedCombatCoord {
  return isRecord(value) && isSafeInteger(value.q) && isSafeInteger(value.r);
}

function validFacing(value: unknown): value is NormalizedCombatCoord {
  return validCoord(value) && FACING_DIRECTIONS.has(`${value.q},${value.r}`);
}

function utf8(value: string): Uint8Array {
  return new TextEncoder().encode(value);
}

export function combatVisualSeedHex(
  tuple: readonly [string, string, string, number, string, NormalizedCombatCueType, number],
): string {
  const bytes = utf8(JSON.stringify(tuple));
  let hash = FNV_OFFSET;
  for (const byte of bytes) {
    hash = BigInt.asUintN(64, (hash ^ BigInt(byte)) * FNV_PRIME);
  }
  return hash.toString(16).padStart(16, '0');
}

export function axialToWorld(coord: Readonly<NormalizedCombatCoord>, size: number): { x: number; z: number } {
  return {
    x: size * 1.5 * coord.q,
    z: size * Math.sqrt(3) * (coord.r + coord.q / 2),
  };
}

export function adaptCombatForThree(combat: unknown, options: CombatAdapterOptions): CombatAdapterResult {
  if (combat === undefined || combat === null) return { kind: 'absent', diagnostics: [] };

  const diagnostics: CombatAdapterDiagnostic[] = [];
  const seenDiagnostics = new Set<string>();
  const addError = (code: CombatAdapterDiagnosticCode, path: string) => addDiagnostic(diagnostics, seenDiagnostics, code, 'error', path);
  const addWarning = (code: CombatAdapterDiagnosticCode, path: string) => addDiagnostic(diagnostics, seenDiagnostics, code, 'warning', path);

  const boundsValid = validBounds(options?.boardBounds);
  if (!boundsValid) addError('INVALID_BOARD_BOUNDS', '$options.boardBounds');
  if (!isRecord(combat)) {
    addError('INVALID_COMBAT_OBJECT', '$');
    return { kind: 'fallback', diagnostics };
  }
  const view = combat.view;
  if (!isRecord(view)) {
    addError('INVALID_VIEW_OBJECT', '$.view');
    return { kind: 'fallback', diagnostics };
  }

  const simulationVersion = nonEmptyString(view.simulation_version) ? view.simulation_version : undefined;
  if (!simulationVersion) addError('INVALID_SIMULATION_VERSION', '$.view.simulation_version');
  const resolutionFingerprint = nonEmptyString(view.resolution_fingerprint) ? view.resolution_fingerprint : undefined;
  if (!resolutionFingerprint) addError('INVALID_RESOLUTION_FINGERPRINT', '$.view.resolution_fingerprint');
  const viewFingerprint = nonEmptyString(view.fingerprint) ? view.fingerprint : undefined;
  if (!viewFingerprint) addError('INVALID_VIEW_FINGERPRINT', '$.view.fingerprint');
  const tickMillis = isSafeInteger(view.tick_millis) && view.tick_millis > 0 ? view.tick_millis : undefined;
  if (tickMillis === undefined) addError('INVALID_TICK_MILLIS', '$.view.tick_millis');

  const rawFrames = Object.prototype.hasOwnProperty.call(view, 'frames') ? view.frames : [];
  if (!Array.isArray(rawFrames)) {
    addError('INVALID_FRAMES', '$.view.frames');
  }

  const frames: NormalizedCombatFrame[] = [];
  let previousValidTick: number | undefined;
  if (Array.isArray(rawFrames)) {
    rawFrames.forEach((rawFrame, frameIndex) => {
      const framePath = `$.view.frames[${frameIndex}]`;
      if (!isRecord(rawFrame)) {
        addError('INVALID_FRAME', framePath);
        return;
      }
      const tick = isSafeInteger(rawFrame.tick) && rawFrame.tick >= 0 ? rawFrame.tick : undefined;
      if (tick === undefined) addError('INVALID_FRAME_TICK', `${framePath}.tick`);
      else if (previousValidTick !== undefined && tick <= previousValidTick) addError('NON_MONOTONIC_FRAME_TICK', `${framePath}.tick`);
      else if (tick !== undefined) previousValidTick = tick;

      const rawPieces = Object.prototype.hasOwnProperty.call(rawFrame, 'pieces') ? rawFrame.pieces : [];
      if (!Array.isArray(rawPieces)) {
        addError('INVALID_PIECES', `${framePath}.pieces`);
        return;
      }
      const pieces: NormalizedCombatPiece[] = [];
      const ids = new Set<string>();
      rawPieces.forEach((rawPiece, pieceIndex) => {
        const piecePath = `${framePath}.pieces[${pieceIndex}]`;
        if (!isRecord(rawPiece)) {
          addError('INVALID_PIECE', piecePath);
          return;
        }
        const id = nonEmptyString(rawPiece.id) ? rawPiece.id : undefined;
        if (!id) addError('INVALID_PIECE_ID', `${piecePath}.id`);
        else if (ids.has(id)) addError('DUPLICATE_PIECE_ID', `${piecePath}.id`);
        else ids.add(id);
        const position = validCoord(rawPiece.position) ? { q: rawPiece.position.q, r: rawPiece.position.r } : undefined;
        if (!position) addError('INVALID_POSITION', `${piecePath}.position`);
        const facing = validFacing(rawPiece.facing) ? { q: rawPiece.facing.q, r: rawPiece.facing.r } : undefined;
        if (!facing) addError('INVALID_FACING', `${piecePath}.facing`);
        const side = rawPiece.side === 'ally' || rawPiece.side === 'enemy' ? rawPiece.side : undefined;
        if (!side) addError('INVALID_SIDE', `${piecePath}.side`);
        const active = typeof rawPiece.active === 'boolean' ? rawPiece.active : undefined;
        if (active === undefined) addError('INVALID_ACTIVE', `${piecePath}.active`);
        const rawCues = Object.prototype.hasOwnProperty.call(rawPiece, 'cues') ? rawPiece.cues : [];
        if (!Array.isArray(rawCues)) {
          addError('INVALID_CUES', `${piecePath}.cues`);
          return;
        }
        const cues: NormalizedCombatCue[] = [];
        rawCues.forEach((rawCue, cueIndex) => {
          const cuePath = `${piecePath}.cues[${cueIndex}]`;
          if (typeof rawCue !== 'string' || !CUES.has(rawCue as NormalizedCombatCueType)) {
            addError('INVALID_CUE', cuePath);
            return;
          }
          if (simulationVersion && resolutionFingerprint && viewFingerprint && tick !== undefined && id) {
            const cueType = rawCue as NormalizedCombatCueType;
            cues.push({ type: cueType, ordinal: cueIndex, seedHex: combatVisualSeedHex([simulationVersion, resolutionFingerprint, viewFingerprint, tick, id, cueType, cueIndex]) });
          }
        });
        if (id && position && facing && side && active !== undefined) pieces.push({ id, side, position, facing, active, cues });
      });
      if (tick !== undefined) frames.push({ tick, pieces });
    });
  }

  if (boundsValid && Array.isArray(rawFrames)) {
    rawFrames.forEach((rawFrame, frameIndex) => {
      if (!isRecord(rawFrame) || !Array.isArray(rawFrame.pieces)) return;
      const occupancy = new Set<string>();
      rawFrame.pieces.forEach((rawPiece, pieceIndex) => {
        if (!isRecord(rawPiece) || !validCoord(rawPiece.position)) return;
        const positionPath = `$.view.frames[${frameIndex}].pieces[${pieceIndex}].position`;
        const key = `${rawPiece.position.q},${rawPiece.position.r}`;
        if (rawPiece.position.q < options.boardBounds.minQ || rawPiece.position.q > options.boardBounds.maxQ || rawPiece.position.r < options.boardBounds.minR || rawPiece.position.r > options.boardBounds.maxR) addWarning('OUT_OF_BOUNDS', positionPath);
        if (occupancy.has(key)) addWarning('DUPLICATE_OCCUPANCY', positionPath);
        occupancy.add(key);
      });
    });
  }

  if (diagnostics.some((item) => item.severity === 'error')) return { kind: 'fallback', diagnostics };

  const replaySimulationVersion = simulationVersion as string;
  const replayResolutionFingerprint = resolutionFingerprint as string;
  const replayViewFingerprint = viewFingerprint as string;
  const replayTickMillis = tickMillis as number;

  return {
    kind: 'ready',
    replay: { simulationVersion: replaySimulationVersion, resolutionFingerprint: replayResolutionFingerprint, viewFingerprint: replayViewFingerprint, tickMillis: replayTickMillis, frames },
    diagnostics,
  };
}

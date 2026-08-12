/**
 * The combat play surface — hex board, two rigged fighters, and the numbers
 * the HUD is allowed to say.
 *
 * Everything here is derived from `combat-frames.json` (real `ScenePage.combat`
 * output). Nothing is invented: the occupied cells, the movement trail, the
 * cue set per tick, the log lines and the 기력 figures all come out of the
 * fixture. The only authored things are (a) which *context* hexes to draw
 * around the occupied ones, (b) the camera, and (c) the Korean wording for the
 * six registered `template_id`s.
 */

import { BONE_INDEX, HIP_HEIGHT, createRobe, createStrand, poseFor, signalsFor, solveSkeleton, stepRobe, stepStrand } from './rig.js';
import { bladeSegmentWorld, drawCharacter2D, drawClash } from './rig2d.js';

// ---------------------------------------------------------------------------
// Palette. One table for this module; no colour literal at a draw site.
// ---------------------------------------------------------------------------

export const STAGE = {
  voidNear: '#0a0c12',
  voidFar: '#141926',
  tile: '#38424f',
  tileFar: '#232c38',
  tileLit: '#4b5866',
  tileEdge: '#5d6b7a',
  tileEdgeSoft: 'rgba(150,170,190,0.20)',
  groundGlow: 'rgba(120,160,190,0.10)',
  ally: '#63c8b2',
  allyDim: 'rgba(99,200,178,0.20)',
  enemy: '#d1566a',
  enemyDim: 'rgba(209,86,106,0.20)',
  trailDot: 'rgba(190,205,220,0.34)',
  gold: '#d8b26a',
  haze: 'rgba(216,178,106,0.05)',
};

// ---------------------------------------------------------------------------
// Deterministic randomness. No `Math.random()` anywhere in this prototype.
// ---------------------------------------------------------------------------

export function fnv1a(text) {
  let h = 0x811c9dc5;
  for (let i = 0; i < text.length; i += 1) {
    h ^= text.charCodeAt(i);
    h = Math.imul(h, 0x01000193);
  }
  return h >>> 0;
}

export function mulberry32(seed) {
  let s = seed >>> 0;
  return () => {
    s = (s + 0x6d2b79f5) >>> 0;
    let t = s;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

// ---------------------------------------------------------------------------
// Board geometry
// ---------------------------------------------------------------------------

/**
 * flat-top axial `(q, r)` -> unit-scale screen vector.
 *
 * Identical to `web/src/ui/storybook/combat/renderCombatStage.ts`. `q`/`r` are
 * two axes 60° apart, not screen x/y, and normalising them directly shears the
 * board — two pieces at equal hex distance land at unequal screen distances.
 */
export function axialToScreen(q, r) {
  return { x: 1.5 * q, y: Math.sqrt(3) * (r + q / 2) };
}

export function hexDistance(a, b) {
  const dq = a.q - b.q;
  const dr = a.r - b.r;
  return (Math.abs(dq) + Math.abs(dq + dr) + Math.abs(dr)) / 2;
}

/**
 * The context field: every hex within `radius` of a cell the fixture actually
 * occupies at some point. Showing context hexes is allowed; inventing occupied
 * ones is not, so this only ever *adds empty ground* around real positions.
 */
export function buildField(view, radius = 2) {
  const occupied = [];
  const seen = new Set();
  for (const frame of view.frames) {
    for (const p of frame.pieces) {
      const key = `${p.position.q},${p.position.r}`;
      if (!seen.has(key)) {
        seen.add(key);
        occupied.push({ q: p.position.q, r: p.position.r });
      }
    }
  }
  const cells = [];
  const added = new Set();
  for (let q = -4; q <= 9; q += 1) {
    for (let r = -5; r <= 5; r += 1) {
      const d = Math.min(...occupied.map((o) => hexDistance({ q, r }, o)));
      if (d > radius) continue;
      const key = `${q},${r}`;
      if (added.has(key)) continue;
      added.add(key);
      cells.push({ q, r, ring: d });
    }
  }
  return { cells, occupied };
}

/**
 * Fit the field into a pixel rect.
 *
 * `squash` is the camera: a uniform vertical compression that reads as looking
 * down at the ground plane at an angle. It is applied *after* the production
 * axial conversion, so relative cell layout is untouched — it is a camera, not
 * a different coordinate system.
 */
export function layoutBoard(field, rect, squash = 0.62, opts = {}) {
  const { fit = 'cover', zoom = 1, focus = null } = opts;
  const pts = field.cells.map((c) => axialToScreen(c.q, c.r));
  const minX = Math.min(...pts.map((p) => p.x));
  const maxX = Math.max(...pts.map((p) => p.x));
  const minY = Math.min(...pts.map((p) => p.y));
  const maxY = Math.max(...pts.map((p) => p.y));
  // +1 hex of margin on each axis so the outer ring of tiles is not clipped.
  const spanX = maxX - minX + 2;
  const spanY = (maxY - minY) * squash + 2 * squash * Math.sqrt(3);
  // The field is inherently wide (the two fighters close along one row) and
  // the board is portrait, so `contain` leaves two thirds of the board empty.
  // `cover` treats the board as a camera looking at the duel: the field runs
  // off both sides and the edges are faded, which is what a shipped tactical
  // screen does.
  const s = (fit === 'cover' ? Math.max(rect.w / spanX, rect.h / spanY) : Math.min(rect.w / spanX, rect.h / spanY)) * zoom;
  const anchor = focus ? axialToScreen(focus.q, focus.r) : { x: (minX + maxX) / 2, y: (minY + maxY) / 2 };
  const cx = rect.x + rect.w / 2 - anchor.x * s;
  const cy = rect.y + rect.h / 2 - anchor.y * s * squash;
  return {
    s,
    squash,
    toPx(q, r) {
      const p = axialToScreen(q, r);
      return { x: cx + p.x * s, y: cy + p.y * s * squash };
    },
  };
}

function hexPath(ctx, cx, cy, s, squash) {
  ctx.beginPath();
  for (let i = 0; i < 6; i += 1) {
    const a = (Math.PI / 3) * i;
    const x = cx + Math.cos(a) * s;
    const y = cy + Math.sin(a) * s * squash;
    if (i === 0) ctx.moveTo(x, y);
    else ctx.lineTo(x, y);
  }
  ctx.closePath();
}

// ---------------------------------------------------------------------------
// Board painting
// ---------------------------------------------------------------------------

export function drawBoard(ctx, rect, field, layout, frame, view) {
  ctx.save();
  ctx.beginPath();
  ctx.rect(rect.x, rect.y, rect.w, rect.h);
  ctx.clip();

  const bg = ctx.createLinearGradient(0, rect.y, 0, rect.y + rect.h);
  bg.addColorStop(0, STAGE.voidFar);
  bg.addColorStop(1, STAGE.voidNear);
  ctx.fillStyle = bg;
  ctx.fillRect(rect.x, rect.y, rect.w, rect.h);

  // A single warm haze centred on the duel: the stage needs one light source
  // or the board reads as a flat diagram.
  const focus = layout.toPx(2.5, 0);
  const haze = ctx.createRadialGradient(focus.x, focus.y, 0, focus.x, focus.y, layout.s * 6);
  haze.addColorStop(0, STAGE.haze);
  haze.addColorStop(1, 'rgba(0,0,0,0)');
  ctx.fillStyle = haze;
  ctx.fillRect(rect.x, rect.y, rect.w, rect.h);

  const cells = [...field.cells].sort((a, b) => layout.toPx(a.q, a.r).y - layout.toPx(b.q, b.r).y);
  const occupiedNow = new Map(frame.pieces.map((p) => [`${p.position.q},${p.position.r}`, p]));

  // Cells the fixture passed through earlier in the bout but has left.
  const visited = new Set();
  for (const f of view.frames) {
    if (f.tick > frame.tick) break;
    for (const p of f.pieces) visited.add(`${p.position.q},${p.position.r}:${p.side}`);
  }

  for (const cell of cells) {
    const c = layout.toPx(cell.q, cell.r);
    const depth = 1 - Math.min(1, (c.y - rect.y) / rect.h);
    hexPath(ctx, c.x, c.y, layout.s * 0.94, layout.squash);
    const g = ctx.createLinearGradient(c.x, c.y - layout.s * layout.squash, c.x, c.y + layout.s * layout.squash);
    g.addColorStop(0, cell.ring === 0 ? STAGE.tileLit : STAGE.tile);
    g.addColorStop(1, STAGE.tileFar);
    ctx.globalAlpha = 0.34 + 0.66 * (1 - cell.ring / 2.6) - depth * 0.12;
    ctx.fillStyle = g;
    ctx.fill();
    ctx.globalAlpha = 0.5 + 0.5 * (1 - cell.ring / 2.6);
    ctx.strokeStyle = STAGE.tileEdgeSoft;
    ctx.lineWidth = 1;
    ctx.stroke();
    ctx.globalAlpha = 1;

    // Top-edge bevel: one lit segment along the two upper faces. Without it
    // the tiles read as flat holes cut in the background rather than as a
    // surface catching the same light the figures do.
    ctx.save();
    ctx.beginPath();
    const s94 = layout.s * 0.94;
    ctx.moveTo(c.x - s94 * 0.5, c.y - s94 * 0.866 * layout.squash);
    ctx.lineTo(c.x + s94 * 0.5, c.y - s94 * 0.866 * layout.squash);
    ctx.lineTo(c.x + s94, c.y);
    ctx.strokeStyle = STAGE.tileEdge;
    ctx.globalAlpha = 0.18 * (1 - cell.ring / 3);
    ctx.lineWidth = 1.4;
    ctx.stroke();
    ctx.restore();

    // Movement trail: a small mark on a cell this side stood on earlier.
    for (const side of ['ally', 'enemy']) {
      if (!visited.has(`${cell.q},${cell.r}:${side}`)) continue;
      if (occupiedNow.has(`${cell.q},${cell.r}`)) continue;
      ctx.beginPath();
      if (side === 'ally') {
        ctx.arc(c.x, c.y, layout.s * 0.075, 0, Math.PI * 2);
      } else {
        const t = layout.s * 0.08;
        ctx.moveTo(c.x - t, c.y - t * layout.squash);
        ctx.lineTo(c.x + t, c.y + t * layout.squash);
        ctx.moveTo(c.x + t, c.y - t * layout.squash);
        ctx.lineTo(c.x - t, c.y + t * layout.squash);
      }
      ctx.strokeStyle = STAGE.trailDot;
      ctx.fillStyle = STAGE.trailDot;
      ctx.lineWidth = 1.4;
      if (side === 'ally') ctx.fill();
      else ctx.stroke();
    }
  }

  // Occupied-cell markers. Shape carries the side: the orthodox school gets a
  // continuous double ring, the gang a broken one. Colour is redundant.
  for (const piece of frame.pieces) {
    const c = layout.toPx(piece.position.q, piece.position.r);
    const isAlly = piece.side === 'ally';
    const glow = ctx.createRadialGradient(c.x, c.y, 0, c.x, c.y, layout.s * 1.2);
    glow.addColorStop(0, isAlly ? STAGE.allyDim : STAGE.enemyDim);
    glow.addColorStop(1, 'rgba(0,0,0,0)');
    ctx.fillStyle = glow;
    ctx.beginPath();
    ctx.ellipse(c.x, c.y, layout.s * 1.2, layout.s * 1.2 * layout.squash, 0, 0, Math.PI * 2);
    ctx.fill();

    ctx.strokeStyle = isAlly ? STAGE.ally : STAGE.enemy;
    ctx.lineWidth = 1.8;
    ctx.setLineDash(isAlly ? [] : [layout.s * 0.2, layout.s * 0.14]);
    hexPath(ctx, c.x, c.y, layout.s * 0.94, layout.squash);
    ctx.stroke();
    if (isAlly) {
      ctx.globalAlpha = 0.5;
      hexPath(ctx, c.x, c.y, layout.s * 0.78, layout.squash);
      ctx.stroke();
      ctx.globalAlpha = 1;
    }
    ctx.setLineDash([]);
  }

  // The camera is framed to give a raised blade headroom, so on ticks where
  // nobody has one raised the top of the board is empty tiles. Sinking the far
  // rows into darkness turns that from empty board into distance. Applied
  // here, before the figures, so it never dims a blade held up into it.
  const far = ctx.createLinearGradient(0, rect.y, 0, rect.y + rect.h * 0.46);
  far.addColorStop(0, 'rgba(7,9,14,0.90)');
  far.addColorStop(1, 'rgba(7,9,14,0)');
  ctx.fillStyle = far;
  ctx.fillRect(rect.x, rect.y, rect.w, rect.h * 0.46);
  ctx.restore();
}

/**
 * Fade the left and right board edges. With a `cover` fit the field runs past
 * the frame, and a hard vertical cut through a row of hexes reads as a bug;
 * a fade reads as the camera's edge.
 */
export function drawEdgeFade(ctx, rect) {
  ctx.save();
  const g = ctx.createLinearGradient(rect.x, 0, rect.x + rect.w, 0);
  g.addColorStop(0, STAGE.voidNear);
  g.addColorStop(0.13, 'rgba(10,12,18,0)');
  g.addColorStop(0.87, 'rgba(10,12,18,0)');
  g.addColorStop(1, STAGE.voidNear);
  ctx.fillStyle = g;
  ctx.fillRect(rect.x, rect.y, rect.w, rect.h);
  ctx.restore();
}

/** A vignette over the whole stage, drawn after the figures. */
export function drawVignette(ctx, rect) {
  ctx.save();
  const g = ctx.createRadialGradient(
    rect.x + rect.w / 2,
    rect.y + rect.h * 0.46,
    rect.w * 0.22,
    rect.x + rect.w / 2,
    rect.y + rect.h * 0.46,
    rect.w * 0.86,
  );
  g.addColorStop(0, 'rgba(0,0,0,0)');
  g.addColorStop(1, 'rgba(4,6,10,0.62)');
  ctx.fillStyle = g;
  ctx.fillRect(rect.x, rect.y, rect.w, rect.h);
  ctx.restore();
}

// ---------------------------------------------------------------------------
// Fighters
// ---------------------------------------------------------------------------

const PHASE = 0.62;
/** Earlier phases inside the *same* tick, for the cut trail. Kept close to
 *  the sampled phase — anything further back reaches into the windup, where
 *  the tip is a whole body away and the streak stops belonging to the cut. */
const TRAIL_PHASES = [-0.13, -0.09, -0.055, -0.025];

/**
 * Where inside the tick to sample *this* combatant.
 *
 * Both fighters carry `attack` on the same tick, and sampling both at the
 * same phase produced two mirrored figures mid-swing — a symmetry that reads
 * as a diagram, not as an exchange. The core already orders them: within a
 * tick the ally's entries carry sequences 1–2 and the challenger's 3–5, so
 * the ally acts first. Reading that order out of the log (rather than
 * inventing a stagger) puts one fighter into follow-through while the other
 * is still driving the cut.
 */
export function phaseFor(view, tick, pieceId) {
  const order = [];
  for (const e of view.core_log) {
    if (e.tick !== tick) continue;
    if (!order.includes(e.actor_id)) order.push(e.actor_id);
  }
  const idx = order.indexOf(pieceId);
  if (idx < 0) return PHASE;
  // First actor: mid-strike, blade driving down and forward into the target.
  // Second actor: still winding up, blade high. The pair reads as cause and
  // answer rather than as two copies of the same frame.
  return idx === 0 ? 0.6 : 0.33;
}

/**
 * Robe state is integrated in tick order and cached, so a still at t4 carries
 * the cloth motion the ticks before it produced. Rendering frames out of order
 * would silently change the hem.
 */
export function clothUpTo(view, targetIndex, cache) {
  if (cache.has(targetIndex)) return cache.get(targetIndex);
  const robes = new Map();
  const hairs = new Map();
  for (const piece of view.frames[0].pieces) {
    robes.set(piece.id, createRobe(4, 8.5));
    // Only 청류문 검수 wears her hair loose; 흑사방 도객's is bound, and a bound
    // head has nothing for a cloth solver to do.
    if (piece.side === 'ally') hairs.set(piece.id, createStrand(4, 8.5));
  }
  for (let i = 0; i <= targetIndex; i += 1) {
    const frame = view.frames[i];
    const prev = i > 0 ? view.frames[i - 1] : null;
    for (const piece of frame.pieces) {
      const s = signalsFor(piece, frame, prev, view);
      const pose = poseFor(s, phaseFor(view, frame.tick, piece.id));
      const sweep = -(s.attack * 1.5 + s.hit * 0.9 + s.closing * 1.2);
      stepRobe(robes.get(piece.id), pose.hipX, HIP_HEIGHT + pose.hipY, sweep);
      const hair = hairs.get(piece.id);
      if (hair) {
        const head = solveSkeleton(pose)[BONE_INDEX.head];
        // Hair carries more of the swing than the hem does and settles faster.
        stepStrand(hair, head.baseX, head.baseY + 1.5, sweep * 1.35);
      }
    }
  }
  const copy = (m) =>
    new Map([...m].map(([k, v]) => [k, { pts: v.pts.map((p) => ({ ...p })), segLen: v.segLen, segments: v.segments }]));
  const snapshot = { robes: copy(robes), hairs: copy(hairs) };
  cache.set(targetIndex, snapshot);
  return snapshot;
}

/**
 * Draw both fighters on their real cells, back to front.
 *
 * `charScale` is expressed in hex radii so the figures keep their relation to
 * the board whatever size the board ends up.
 */
export function drawFighters(ctx, view, frameIndex, layout, robeCache, charHeightInHexRadii = 2.05) {
  const frame = view.frames[frameIndex];
  const prev = frameIndex > 0 ? view.frames[frameIndex - 1] : null;
  const cloth = clothUpTo(view, frameIndex, robeCache);
  const scale = (layout.s * charHeightInHexRadii) / 100;

  const placed = frame.pieces
    .map((piece) => {
      const signals = signalsFor(piece, frame, prev, view);
      const phase = phaseFor(view, frame.tick, piece.id);
      const pose = poseFor(signals, phase);
      const px = layout.toPx(piece.position.q, piece.position.r);
      return { piece, signals, pose, px, phase };
    })
    .sort((a, b) => a.px.y - b.px.y);

  for (const item of placed) {
    const { piece, signals, pose, px, phase } = item;
    const trailJoints = signals.attack && !signals.down
      ? TRAIL_PHASES.map((d) => solvedAt(signals, phase + d))
      : [];
    drawCharacter2D(ctx, {
      cx: px.x,
      groundY: px.y + layout.s * layout.squash * 0.18,
      scale,
      side: piece.side,
      pose,
      robe: cloth.robes.get(piece.id),
      hair: cloth.hairs.get(piece.id),
      signals,
      phase,
      seed: fnv1a(piece.id) % 97,
      trailJoints,
    });
    item.groundY = px.y + layout.s * layout.squash * 0.18;
    item.scale = scale;
  }

  return placed;
}

/** The cut trail wants solved skeletons from earlier in the same tick, so the
 *  pose is re-solved at an earlier phase rather than faked with an arc. */
function solvedAt(signals, phase) {
  return solveSkeleton(poseFor(signals, Math.max(0, Math.min(1, phase))));
}

/**
 * If both fighters swung on the same tick and their blades actually cross,
 * that is a parry — flare it, once, exactly where the two edges meet.
 *
 * The first version compared blade *tips* and never fired: in a real exchange
 * the tips end up on opposite sides of the pair while the blades cross near
 * their middles. Intersecting the two grip→tip segments finds the contact
 * point the picture already shows, and returns nothing when there isn't one —
 * so the flare can never appear where the blades are not meeting.
 */
export function drawClashIfAny(ctx, placed, layout) {
  if (placed.length !== 2) return null;
  const [a, b] = placed;
  if (!a.signals.attack || !b.signals.attack || a.signals.down || b.signals.down) return null;

  const segs = placed.map((it) => {
    const seg = bladeSegmentWorld(it.pose, it.piece.side);
    const toScreen = (p) => ({
      x: it.px.x + p.x * it.scale * it.signals.facing,
      y: it.groundY - p.y * it.scale,
    });
    return { a: toScreen(seg.grip), b: toScreen(seg.tip) };
  });
  const hit = segmentIntersection(segs[0].a, segs[0].b, segs[1].a, segs[1].b);
  if (!hit) return null;
  drawClash(ctx, hit.x, hit.y, layout.s * 0.34, mulberry32(fnv1a(`clash:${a.signals.tick}`)));
  return hit;
}

function segmentIntersection(p1, p2, p3, p4) {
  const d = (p2.x - p1.x) * (p4.y - p3.y) - (p2.y - p1.y) * (p4.x - p3.x);
  if (Math.abs(d) < 1e-6) return null;
  const t = ((p3.x - p1.x) * (p4.y - p3.y) - (p3.y - p1.y) * (p4.x - p3.x)) / d;
  const u = ((p3.x - p1.x) * (p2.y - p1.y) - (p3.y - p1.y) * (p2.x - p1.x)) / d;
  if (t < 0 || t > 1 || u < 0 || u > 1) return null;
  return { x: p1.x + t * (p2.x - p1.x), y: p1.y + t * (p2.y - p1.y) };
}

// ---------------------------------------------------------------------------
// Text derived from the fixture
// ---------------------------------------------------------------------------

/** Display names for the two real combatant ids. The ids stay visible in the
 *  identity cards; these are the reading names the log uses. */
export const COMBATANT_NAMES = {
  wuxia_spectator_bout_ally: { name: '청류문 검수', house: '청류문', side: 'ally' },
  wuxia_spectator_bout_challenger: { name: '흑사방 도객', house: '흑사방', side: 'enemy' },
};

function nameOf(id) {
  return COMBATANT_NAMES[id]?.name ?? id;
}

function roundHundredths(v) {
  const sign = v < 0 ? -1 : 1;
  return sign * Math.floor((Math.abs(v) + 50) / 100);
}

/**
 * One log entry -> one natural Korean sentence.
 *
 * The six registered `template_id`s and nothing else; an unknown id surfaces
 * its own id rather than being silently dropped, exactly as the production
 * template table does.
 */
export function logSentence(entry) {
  const actor = nameOf(entry.actor_id);
  const target = entry.target_id ? nameOf(entry.target_id) : null;
  switch (entry.template_id) {
    case 'combat.log.target_selection':
      return target ? `${actor}, ${target}에게 검끝을 겨눈다.` : `${actor}, 겨눌 상대가 없다.`;
    case 'combat.log.move_intent':
      return target ? `${actor}, 간격을 끊고 ${target}의 품으로 파고든다.` : `${actor}, 간격을 끊는다.`;
    case 'combat.log.collision':
      return target ? `${actor}의 검이 ${target}의 검과 맞부딪친다.` : `${actor}의 검이 허공을 친다.`;
    case 'combat.log.damage_applied': {
      const v = entry.value_hundredths;
      if (v === null || v === undefined) return `${actor}의 일격이 ${target ?? '허공'}에 닿는다.`;
      return `${actor}의 일격 — ${target ?? '허공'}, 기력 ${roundHundredths(v)} 깎인다.`;
    }
    case 'combat.log.effect_applied':
      return `${actor}, ${target ?? '허공'}에게 [${entry.effect_id ?? '효과 id 없음'}]을 건다.`;
    case 'combat.log.effect_applied_hidden':
      return `${actor}, ${target ?? '허공'}에게 정체불명의 수를 건다.`;
    default:
      return `${actor} — 알 수 없는 사건 [${entry.template_id}]`;
  }
}

/** The log window shown for a tick: this tick's entries, plus enough of the
 *  previous ones to fill the panel. Ordered exactly as the core emitted them. */
export function logWindow(view, tick, size = 7) {
  const upTo = view.core_log.filter((e) => e.tick <= tick);
  return upTo.slice(Math.max(0, upTo.length - size));
}

/**
 * 기력 remaining for each combatant at a tick, summed from the real
 * `damage_applied` entries. The denominator is that combatant's own total
 * damage-taken figure from the report, which is the number the bout actually
 * ended on — so "0 남음" at t8 is the fixture's own arithmetic, not a guess.
 */
export function vitalsAt(combat, tick) {
  const out = new Map();
  for (const c of combat.report.combatants) {
    out.set(c.id, { total: c.damage_taken_hundredths, taken: 0 });
  }
  for (const e of combat.view.full_log) {
    if (e.tick > tick) continue;
    if (e.template_id !== 'combat.log.damage_applied') continue;
    if (!e.target_id || !out.has(e.target_id)) continue;
    out.get(e.target_id).taken += e.value_hundredths ?? 0;
  }
  const result = new Map();
  for (const [id, v] of out) {
    const remain = Math.max(0, v.total - v.taken);
    result.set(id, {
      remain: roundHundredths(remain),
      max: roundHundredths(v.total),
      ratio: v.total > 0 ? remain / v.total : 0,
    });
  }
  return result;
}

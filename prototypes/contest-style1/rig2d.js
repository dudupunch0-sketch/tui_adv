/**
 * Style 1 — 2D bone rig drawn as an inked silhouette (Canvas 2D).
 *
 * Consumes the solved skeleton from `rig.js` and draws a body around it. The
 * drawing vocabulary is deliberately small: offset polylines around bone
 * chains, one continuous garment built from the verlet hem, one keyline, one
 * rim light. There are no facial features — partly because the art policy asks
 * for them to be absent, and partly because a silhouette that has to sell a
 * pose is *helped* by not having a face competing with it.
 *
 * What makes this read as a body rather than a stick figure:
 *   1. **Near/far limb pairs.** The far arm and leg are drawn first in a
 *      darker tone. Two-tone depth is what separates a fighting silhouette
 *      from a diagram.
 *   2. **One garment, not a stack of parts.** Shoulders, waist and skirt are a
 *      single closed outline whose spine runs chest → hips → verlet hem, so
 *      the figure has one continuous contour instead of visible seams.
 *   3. **A light direction.** Every cloth shape gets a trailing shade lobe and
 *      a leading rim. Flat fills read as paper cut-outs at this size.
 *   4. **Cloth with inertia.** The hem carries motion from the previous tick,
 *      so a lunge drags the robe behind it.
 *
 * Side is carried by **shape** before colour (accessibility contract):
 *
 *   | | 청류문 (ally) | 흑사방 (enemy) |
 *   |---|---|---|
 *   | head | bound topknot + streaming ribbon | wide 삿갓, face in its shadow |
 *   | blade | straight 검, disc guard, tassel | curved 도, ring pommel |
 *   | hem | long, smooth, ankle-deep | short, ragged, knee-cut |
 *   | sleeve | wide flared cuff | tight bound forearm wrap |
 *
 * Colour only reinforces that split; it never carries it alone.
 */

import { BONE_INDEX, HIP_HEIGHT, RAD, expressionFor, solveSkeleton } from './rig.js';

// ---------------------------------------------------------------------------
// Palette. One table for the whole module; no colour literal at a draw site.
// ---------------------------------------------------------------------------

export const PAL2D = {
  ally: {
    // Muted and low-chroma on purpose. A saturated robe next to a delicately
    // shaded face flattens the face — the robe wins the eye and the modelling
    // in the skin stops registering. Warm metal is the only bright accent.
    cloth: '#5b8b83',
    clothLit: '#8ab5ab',
    clothShade: '#2c4a49',
    clothDeep: '#1b2f30',
    ink: '#07171a',
    sash: '#ddd0ad',
    sashShade: '#a8966f',
    trim: '#a7c9bf',
    metal: '#dbe4ea',
    metalShade: '#8d9ca7',
    hilt: '#2a1c15',
    // Face. The shadow tone is *warmer* than the lit one, not merely darker —
    // a straight darkening reads as dirt on the cheek rather than as a
    // turning form.
    skin: '#f0d3b1',
    skinMid: '#e0b891',
    skinShade: '#c08e69',
    skinDeep: '#9a6a4b',
    lash: '#231a1d',
    brow: '#3c2e27',
    sclera: '#f9f3ec',
    scleraShade: '#dccec0',
    irisTop: '#173f45',
    irisMid: '#33706e',
    irisLit: '#7ab5a6',
    irisRim: '#b7e3d3',
    pupil: '#0d0b0c',
    glint: '#ffffff',
    lip: '#b8776a',
    lipLit: '#f0c8b4',
    mouthDark: '#5b3033',
    blush: '#dd8a72',
    trimMetal: '#d8b26a',
    hair: '#101b20',
    hairLit: '#2c4048',
  },
  enemy: {
    // 흑사방 reads as "black", but a genuinely black robe collapses into a
    // night board. The cloth is a desaturated plum two steps above the
    // background, and the rim light does the rest.
    cloth: '#3e3441',
    clothLit: '#665a6c',
    clothShade: '#211a25',
    clothDeep: '#110d14',
    ink: '#060305',
    sash: '#8a3540',
    sashShade: '#521f28',
    trim: '#a75c68',
    metal: '#dae3e9',
    metalShade: '#7f8d97',
    hilt: '#1a0e11',
    skin: '#dcb68f',
    skinMid: '#c69b73',
    skinShade: '#a2774f',
    skinDeep: '#7a5636',
    lash: '#150e10',
    brow: '#2a1c1a',
    sclera: '#f3eae0',
    scleraShade: '#cfbfb1',
    irisTop: '#452812',
    irisMid: '#8a6533',
    irisLit: '#cfa464',
    irisRim: '#f0d3a0',
    pupil: '#0a0708',
    glint: '#fff6e2',
    lip: '#a06052',
    lipLit: '#e4b6a2',
    mouthDark: '#422121',
    blush: '#c4705c',
    trimMetal: '#c39b52',
    hair: '#0a0709',
    hairLit: '#251b21',
  },
  shared: {
    shadow: 'rgba(3,7,11,0.44)',
    trail: '#e6f6ff',
    spark: '#ffe6a6',
    clash: '#fff3cf',
    downWash: 'rgba(6,10,14,0.34)',
    brimShadow: 'rgba(20,8,12,0.72)',
  },
};

/**
 * Permanent per-character set of the features. Not a signal — the Black
 * Serpent's sneer survives every state, the way a face does.
 */
export const FACE_BIAS = {
  // 청류문 검수 is cold and composed, and that is her *neutral* — lids a shade
  // low, brows level, mouth a flat line. Setting it here rather than inside
  // `alert` means it survives every state, so her `hit` and `incapacitated`
  // read all the harder against it.
  ally: { lid: 0.05, browOuter: -0.04, mouthCurve: -0.02 },
  // 흑사방 도객 carries a slight upswept sneer in the same permanent way.
  enemy: { browInner: -0.12, browOuter: -0.2, mouthCurve: -0.18, squint: 0.1 },
};

/**
 * A monochrome palette, for the silhouette check.
 *
 * Side has to survive colour being removed — that is the accessibility
 * contract, and rendering it is the only way to know it holds.
 */
export function silhouettePalette() {
  const flat = {};
  for (const key of Object.keys(PAL2D.ally)) flat[key] = '#0c0c10';
  return flat;
}

// ---------------------------------------------------------------------------
// Geometry helpers
// ---------------------------------------------------------------------------

/**
 * Offset a polyline to both sides by a per-point half-width.
 *
 * The normal points *forward* in character space (+X) for a chain that runs
 * downward, which is what lets callers talk about a garment's "front" and
 * "back" edge rather than about an arbitrary left/right.
 */
function offsetPolyline(pts, widths) {
  const front = [];
  const back = [];
  for (let i = 0; i < pts.length; i += 1) {
    const prev = pts[Math.max(0, i - 1)];
    const next = pts[Math.min(pts.length - 1, i + 1)];
    let dx = next.x - prev.x;
    let dy = next.y - prev.y;
    const len = Math.hypot(dx, dy) || 1;
    dx /= len;
    dy /= len;
    const w = (widths[Math.min(i, widths.length - 1)] ?? widths[widths.length - 1]) / 2;
    front.push({ x: pts[i].x - dy * w, y: pts[i].y + dx * w });
    back.push({ x: pts[i].x + dy * w, y: pts[i].y - dx * w });
  }
  return { front, back };
}

function polyPath(ctx, pts) {
  ctx.beginPath();
  ctx.moveTo(pts[0].x, pts[0].y);
  for (let i = 1; i < pts.length; i += 1) ctx.lineTo(pts[i].x, pts[i].y);
  ctx.closePath();
}

/**
 * Build a tapered polygon through a polyline.
 *
 * Caps are flat, closed straight across. An earlier version used `ctx.arc()`
 * for rounded ends, which appends a *full circle subpath* rather than an end
 * cap — the first capture came back with a hard ring stamped at every joint.
 * Limbs overlap at the joints anyway, so flat caps are invisible.
 */
function taperedPath(ctx, pts, widths) {
  const { front, back } = offsetPolyline(pts, widths);
  polyPath(ctx, [...front, ...back.slice().reverse()]);
}

function fillInk(ctx, fill, ink, width) {
  ctx.fillStyle = fill;
  ctx.fill();
  if (ink && width > 0) {
    ctx.strokeStyle = ink;
    ctx.lineWidth = width;
    ctx.lineJoin = 'round';
    ctx.stroke();
  }
}

function chain(joints, names) {
  const first = joints[BONE_INDEX[names[0]]];
  const pts = [{ x: first.baseX, y: first.baseY }];
  for (const n of names) pts.push({ x: joints[BONE_INDEX[n]].tipX, y: joints[BONE_INDEX[n]].tipY });
  return pts;
}

function dirOf(joint) {
  const a = joint.angle * RAD;
  return { x: Math.sin(a), y: Math.cos(a), a };
}

function lerpPt(a, b, t) {
  return { x: a.x + (b.x - a.x) * t, y: a.y + (b.y - a.y) * t };
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

/**
 * Draw one character.
 *
 * The canvas transform maps rig units to pixels with +Y up and the ground
 * plane at the character's feet, which lets every number in `rig.js` stay in
 * one readable unit system. `facing === -1` mirrors in X, so "forward" is
 * always +X inside this module.
 *
 * `opts.trailJoints` is an array of *earlier-phase* solved skeletons for this
 * same tick, supplied by the caller. The cut trail is drawn through the blade
 * tip at those samples rather than as a synthetic arc — which is the only way
 * to guarantee the trail belongs to this character's actual swing.
 */
export function drawCharacter2D(ctx, opts) {
  const { cx, groundY, scale, side, pose, robe, hair, signals, phase, seed = 0, trailJoints = [] } = opts;
  const pal = opts.pal ?? PAL2D[side];
  const joints = solveSkeleton(pose);
  // The face reads the same signals the body does, through the same
  // STATES → MODIFIERS blend. The renderer is handed channel values, never a
  // cue array — exactly as it is handed joint angles rather than a pose name.
  const expr = opts.expr ?? expressionFor(signals, phase, FACE_BIAS[side]);

  ctx.save();
  ctx.translate(cx, groundY);
  ctx.scale(scale * signals.facing, -scale);
  ctx.lineJoin = 'round';
  ctx.lineCap = 'butt';
  // A defeated combatant desaturates and drops in value. This is the same
  // rule the production board already applies to an incapacitated piece
  // (`.combat-board__piece[data-cue-incapacitated]`: opacity 0.55,
  // saturate 0.4) — expressed here as a canvas filter so the whole figure
  // dims as one shape instead of each part fading through the others.
  if (signals.down) ctx.filter = 'saturate(0.5) brightness(0.78)';

  drawShadow(ctx, pose, signals);
  // Behind everything: 흑사방's 삿갓 slung on her back, and 청류문's loose hair.
  if (side === 'enemy') drawSlungSatgat(ctx, joints, pal);
  if (side === 'ally' && hair) drawLooseHair(ctx, joints, hair, pal);
  drawLeg(ctx, joints, pal, 'Far');
  drawArm(ctx, joints, pal, side, 'Far');
  drawLeg(ctx, joints, pal, 'Near');
  drawGarment(ctx, joints, robe, pal, side, signals, seed);
  drawSash(ctx, joints, pal, side, signals);
  drawCollar(ctx, joints, pal);
  // Only the strike leaves a cut. During the windup the blade is travelling
  // too — but a trail there captured as a wide grey wedge hanging off a
  // raised blade, because a rising blade's trail points *up*, out of the
  // figure and out of the frame. A cut trail belongs to the cut.
  if (signals.attack && !signals.down && phase >= 0.42) drawCutTrail(ctx, joints, trailJoints, side);
  drawArm(ctx, joints, pal, side, 'Near');
  if (signals.down) drawDroppedBlade(ctx, pal, side, seed);
  else drawBlade(ctx, joints, pal, side);
  drawHead(ctx, joints, pal, side, expr);

  ctx.restore();
}

// ---------------------------------------------------------------------------
// Body parts
// ---------------------------------------------------------------------------

function drawShadow(ctx, pose, signals) {
  const spread = signals.down ? 21 : 12.5;
  ctx.save();
  ctx.beginPath();
  ctx.ellipse(pose.hipX * 0.42, 1.2, spread, signals.down ? 4.2 : 3.2, 0, 0, Math.PI * 2);
  ctx.fillStyle = PAL2D.shared.shadow;
  ctx.fill();
  ctx.restore();
}

/** Legs, boots. `Far` renders in the shade tone so the pair reads as depth. */
function drawLeg(ctx, joints, pal, which) {
  const far = which === 'Far';
  const fill = far ? pal.clothShade : pal.cloth;
  const leg = chain(joints, [`leg${which}Thigh`, `leg${which}Shin`, `leg${which}Foot`]);
  taperedPath(ctx, leg.slice(0, 3), far ? [9.6, 7.2, 5.4] : [10.4, 7.8, 5.8]);
  fillInk(ctx, fill, pal.ink, 1.1);

  // Boot: a separate wedge so the ankle has a visible break rather than the
  // trouser simply continuing into the ground.
  const shin = joints[BONE_INDEX[`leg${which}Shin`]];
  const foot = joints[BONE_INDEX[`leg${which}Foot`]];
  const ankle = { x: shin.tipX, y: shin.tipY };
  const toe = { x: foot.tipX, y: foot.tipY };
  taperedPath(ctx, [{ x: ankle.x, y: ankle.y + 5 }, ankle, lerpPt(ankle, toe, 0.55), toe], [7.4, 7, 6, 3.4]);
  fillInk(ctx, far ? pal.clothDeep : pal.ink, pal.ink, 0.9);
}

/**
 * One arm: upper + forearm, plus the sleeve treatment for this side. The wide
 * flared 장삼 sleeve is the genre's most recognisable silhouette cue and costs
 * one extra polygon; the enemy gets a bound forearm wrap instead, so the two
 * arms differ in outline even at thumbnail size.
 */
function drawArm(ctx, joints, pal, side, which) {
  const far = which === 'Far';
  const upper = joints[BONE_INDEX[`arm${which}Up`]];
  const fore = joints[BONE_INDEX[`arm${which}Fore`]];
  const hand = joints[BONE_INDEX[`arm${which}Hand`]];
  const fill = far ? pal.clothShade : pal.cloth;
  const lit = far ? pal.clothShade : pal.clothLit;

  const arm = chain(joints, [`arm${which}Up`, `arm${which}Fore`, `arm${which}Hand`]);
  taperedPath(ctx, arm, [8.0, 6.2, 4.9, 3.7]);
  fillInk(ctx, fill, pal.ink, 1.1);

  // Sleeve treatments are near-side only. On the far arm the extra shape ends
  // in a flat cap that pokes out from behind the robe, and both captures came
  // back with a small hard-edged block hanging off each fighter's waist. The
  // far arm is a depth cue, not a costume detail.
  if (far) {
    ctx.beginPath();
    ctx.ellipse(hand.baseX, hand.baseY, 2.5, 2.2, 0, 0, Math.PI * 2);
    fillInk(ctx, pal.clothShade, pal.ink, 0.8);
    return;
  }

  if (side === 'ally') {
    // Flared cuff: widens from shoulder toward the elbow and hangs past it.
    const cuffEnd = lerpPt({ x: fore.baseX, y: fore.baseY }, { x: fore.tipX, y: fore.tipY }, 0.42);
    const drape = { x: cuffEnd.x - 1.2, y: cuffEnd.y - 6 };
    taperedPath(
      ctx,
      [{ x: upper.baseX, y: upper.baseY }, { x: fore.baseX, y: fore.baseY }, cuffEnd, drape],
      [9.0, 10.4, 12.4, 7.8],
    );
    fillInk(ctx, lit, pal.ink, 1.2);
    // Cuff band. Without a terminator the sleeve merged into the robe and the
    // hanging arm captured as a pale bib across the chest.
    ctx.beginPath();
    ctx.moveTo(cuffEnd.x, cuffEnd.y);
    ctx.lineTo(drape.x, drape.y);
    ctx.strokeStyle = pal.sash;
    ctx.lineWidth = 2.6;
    ctx.stroke();
  } else {
    // Bound wrap: a short tapered over-sleeve on the upper arm, then three
    // tight bands down the forearm. The first version drew the over-sleeve as
    // a near-constant-width quad, which read as a shoulder pad rather than as
    // a sleeve.
    taperedPath(
      ctx,
      [
        { x: upper.baseX, y: upper.baseY },
        lerpPt({ x: upper.baseX, y: upper.baseY }, { x: fore.baseX, y: fore.baseY }, 0.5),
        lerpPt({ x: upper.baseX, y: upper.baseY }, { x: fore.baseX, y: fore.baseY }, 0.88),
      ],
      [11, 9.2, 6.8],
    );
    fillInk(ctx, lit, pal.ink, 1.1);
    // Two bands, in the sash's shadow tone. Three in the bright tone captured
    // as a striped bar that detached from the arm at a distance.
    for (let i = 0; i < 2; i += 1) {
      const t = 0.3 + i * 0.28;
      const c = lerpPt({ x: fore.baseX, y: fore.baseY }, { x: fore.tipX, y: fore.tipY }, t);
      const n = dirOf(fore);
      ctx.beginPath();
      ctx.moveTo(c.x - n.y * 3.0, c.y + n.x * 3.0);
      ctx.lineTo(c.x + n.y * 3.0, c.y - n.x * 3.0);
      ctx.strokeStyle = pal.sashShade;
      ctx.lineWidth = 1.5;
      ctx.stroke();
    }
  }

  ctx.beginPath();
  ctx.ellipse(hand.baseX, hand.baseY, 3.1, 2.7, 0, 0, Math.PI * 2);
  fillInk(ctx, pal.skin, pal.ink, 1);
}

/**
 * The garment — torso and skirt as one closed outline.
 *
 * The spine runs chest tip → hips → verlet hem, so the skirt inherits the
 * cloth simulation while the shoulders stay rigid on the chest bone. Drawing
 * torso and skirt as two shapes (the first version) left a visible seam
 * exactly at the waist, where the eye is already being pulled by the sash.
 */
function drawGarment(ctx, joints, robe, pal, side, signals, seed) {
  const chest = joints[BONE_INDEX.chest];
  const hips = joints[BONE_INDEX.hips];
  const hem = robe.pts.slice(1);

  const spine = [
    { x: chest.tipX, y: chest.tipY },
    { x: chest.baseX, y: chest.baseY },
    { x: hips.baseX, y: hips.baseY },
    ...hem,
  ];
  // Ally: long smooth 장삼. Enemy: cut short at the knee, so the two hems
  // differ in outline as well as in tone.
  // Narrow shoulders, a real waist, and a wide flare below it — both figures
  // are women in their early twenties. The hem is where they diverge.
  const base =
    side === 'ally'
      ? [18.5, 17.5, 13.6, 20.5, 27, 32.5]
      : [19.5, 18, 13.0, 19, 24, 25.5];
  // A kneeling figure's skirt pools. Without this the down pose only made the
  // figure *shorter* — still a vertical column, which captured as "standing,
  // smaller" rather than as "on its knees". The horizontal spread at the
  // ground is what sells the kneel.
  const widths = signals.down
    ? base.map((w, i) => (i < 3 ? w : w * (1.18 + (i - 3) * 0.14)))
    : base;

  const { front, back } = offsetPolyline(spine, widths);
  const hemEdge = hemLine(front[front.length - 1], back[back.length - 1], side, seed);

  polyPath(ctx, [...front, ...hemEdge, ...back.slice().reverse()]);
  ctx.save();
  ctx.clip();

  // Base fill plus a vertical gradient: the cloth is lit from above-front, so
  // the hem sits in its own shadow without needing a second shape.
  const grad = ctx.createLinearGradient(0, HIP_HEIGHT + 26, 0, 0);
  grad.addColorStop(0, pal.clothLit);
  grad.addColorStop(0.42, pal.cloth);
  grad.addColorStop(1, pal.clothShade);
  ctx.fillStyle = grad;
  ctx.fillRect(-60, -10, 120, 130);

  // Trailing shade lobe: everything behind the spine, offset forward slightly
  // so the lit band sits on the leading edge where the eye expects it.
  const lobe = spine.map((p, i) => ({ x: p.x - widths[Math.min(i, widths.length - 1)] * 0.16, y: p.y }));
  polyPath(ctx, [...lobe, ...back.slice().reverse()]);
  ctx.fillStyle = pal.clothShade;
  ctx.globalAlpha = 0.55;
  ctx.fill();
  ctx.globalAlpha = 1;

  // Front placket: the 교임 cross-collar seam, one line that gives the shape
  // an inside.
  ctx.beginPath();
  ctx.moveTo(front[0].x - 3, front[0].y - 1);
  for (let i = 1; i < spine.length; i += 1) {
    const t = i / (spine.length - 1);
    const p = lerpPt(spine[i], front[i], 0.42 + t * 0.25);
    ctx.lineTo(p.x, p.y);
  }
  ctx.strokeStyle = pal.clothLit;
  ctx.globalAlpha = 0.5;
  ctx.lineWidth = 1.5;
  ctx.stroke();
  ctx.globalAlpha = 1;
  ctx.restore();

  // Keyline last, over the clipped fills, so the contour stays crisp.
  polyPath(ctx, [...front, ...hemEdge, ...back.slice().reverse()]);
  ctx.strokeStyle = pal.ink;
  ctx.lineWidth = 1.5;
  ctx.stroke();

  // Leading rim light. Two-tone cloth plus a rim is what stops a dark enemy
  // from collapsing into the board behind it.
  ctx.beginPath();
  ctx.moveTo(front[0].x, front[0].y);
  for (let i = 1; i < front.length; i += 1) ctx.lineTo(front[i].x, front[i].y);
  ctx.strokeStyle = pal.clothLit;
  ctx.globalAlpha = signals.down ? 0.3 : 0.7;
  ctx.lineWidth = 1.6;
  ctx.stroke();
  ctx.globalAlpha = 1;
}

/**
 * The hem edge, front point to back point.
 *
 * A straight cut across reads as a cardboard silhouette. The ally's hem is a
 * shallow smooth wave, the enemy's is a torn zigzag — the same seeded
 * function, two amplitudes and two shapes.
 */
function hemLine(frontPt, backPt, side, seed) {
  const out = [];
  const steps = side === 'ally' ? 5 : 6;
  for (let i = 1; i < steps; i += 1) {
    const t = i / steps;
    const p = lerpPt(frontPt, backPt, t);
    const wave =
      side === 'ally'
        ? Math.sin(t * Math.PI * 1.6 + seed * 0.7) * 2.4 - Math.sin(t * Math.PI) * 1.6
        : (i % 2 === 0 ? 3.4 : -1.6) + Math.sin(seed + i) * 0.9;
    out.push({ x: p.x, y: p.y - wave });
  }
  return out;
}

/**
 * The waist band — the one bright horizontal on the figure.
 *
 * The band rides the spine, but the hanging ends are drawn in *world* space.
 * Drawn in the rotated waist frame they swung upward whenever the torso
 * pitched forward, which at t8 produced a beige spar sticking out of the
 * kneeling figure's back. Cloth hangs with gravity, not with the pelvis.
 */
function drawSash(ctx, joints, pal, side, signals) {
  const hips = joints[BONE_INDEX.hips];
  const spine = joints[BONE_INDEX.spine];
  ctx.save();
  ctx.translate(hips.baseX, hips.baseY);
  ctx.rotate(-Math.atan2(spine.tipX - spine.baseX, spine.tipY - spine.baseY));
  ctx.beginPath();
  ctx.moveTo(-7.9, -0.2);
  ctx.lineTo(7.9, 1.4);
  ctx.lineTo(7.7, 5.0);
  ctx.lineTo(-8.1, 4.0);
  ctx.closePath();
  fillInk(ctx, pal.sash, pal.ink, 1);
  ctx.beginPath();
  ctx.moveTo(-8.0, 0.9);
  ctx.lineTo(7.8, 2.5);
  ctx.strokeStyle = pal.sashShade;
  ctx.lineWidth = 1.1;
  ctx.stroke();
  ctx.restore();

  // Hanging ends. Ally: two ribbons. Enemy: one short tassel.
  const kx = hips.baseX + (side === 'ally' ? -2.5 : 3);
  const ky = hips.baseY + 1;
  const ends = side === 'ally' ? [[-3.5, -13.5, 1.7], [-1, -10, 1.2]] : [[3.5, -9, 1.9]];
  ctx.lineCap = 'round';
  for (const [dx, dy, w] of ends) {
    ctx.beginPath();
    ctx.moveTo(kx, ky);
    ctx.quadraticCurveTo(kx + dx * 0.4, ky + dy * 0.55, kx + dx, ky + dy * (signals.down ? 0.8 : 1));
    ctx.strokeStyle = side === 'ally' ? pal.sash : pal.trim;
    ctx.lineWidth = w;
    ctx.stroke();
  }
  ctx.lineCap = 'butt';
}

/** A small collar wedge so the head sits on shoulders instead of floating. */
function drawCollar(ctx, joints, pal) {
  const chest = joints[BONE_INDEX.chest];
  const neck = joints[BONE_INDEX.neck];
  const d = dirOf(neck);
  ctx.beginPath();
  ctx.moveTo(chest.tipX - d.y * 7.0, chest.tipY + d.x * 7.0);
  ctx.lineTo(chest.tipX + d.x * 3.2 - d.y * 3.6, chest.tipY + d.y * 3.2 + d.x * 3.6);
  ctx.lineTo(chest.tipX + d.x * 3.2 + d.y * 3.9, chest.tipY + d.y * 3.2 - d.x * 3.9);
  ctx.lineTo(chest.tipX + d.y * 7.2, chest.tipY - d.x * 7.2);
  ctx.closePath();
  fillInk(ctx, pal.clothShade, pal.ink, 1.1);
}

// ---------------------------------------------------------------------------
// Weapons
// ---------------------------------------------------------------------------

/**
 * The blade. Side changes the *shape*, not just the palette:
 * 청류문 carries a straight double-edged 검 with a disc guard and a tassel,
 * 흑사방 a curved single-edged 도 with a ring pommel.
 */
function drawBlade(ctx, joints, pal, side) {
  const hand = joints[BONE_INDEX.armNearHand];
  const d = dirOf(hand);
  ctx.save();
  ctx.translate(hand.baseX, hand.baseY);
  ctx.rotate(-d.a);
  if (side === 'ally') drawJian(ctx, pal);
  else drawDao(ctx, pal);
  ctx.restore();
}

/**
 * Total bend of the 도, convex edge forward (+X). Shallow on purpose: at
 * 0.42 rad with a mid-blade bulge the silhouette read as a sickle. The blade
 * tip helper below must use the same number or the cut trail detaches from
 * the blade.
 */
const DAO_CURVE = 0.24;

/** Local frame: origin at the grip, blade runs along +Y, forward is +X. */
function drawJian(ctx, pal) {
  const L = 47;
  ctx.beginPath();
  ctx.moveTo(-2.9, 5);
  ctx.lineTo(-2.1, L * 0.74);
  ctx.lineTo(0, L);
  ctx.lineTo(2.1, L * 0.74);
  ctx.lineTo(2.9, 5);
  ctx.closePath();
  fillInk(ctx, pal.metal, pal.ink, 1);

  // Fuller: one line down the spine so the blade has a section.
  ctx.beginPath();
  ctx.moveTo(0, 6.5);
  ctx.lineTo(0, L * 0.86);
  ctx.strokeStyle = pal.metalShade;
  ctx.lineWidth = 0.9;
  ctx.stroke();

  // Disc guard.
  ctx.beginPath();
  ctx.ellipse(0, 4.6, 5.4, 1.9, 0, 0, Math.PI * 2);
  fillInk(ctx, pal.metalShade, pal.ink, 1);

  // Grip + pommel.
  ctx.beginPath();
  ctx.rect(-1.9, -7.2, 3.8, 11.6);
  fillInk(ctx, pal.hilt, pal.ink, 1);
  ctx.beginPath();
  ctx.ellipse(0, -7.6, 2.8, 1.7, 0, 0, Math.PI * 2);
  fillInk(ctx, pal.metalShade, pal.ink, 1);

  // Tassel — three strands falling off the pommel.
  for (let i = 0; i < 3; i += 1) {
    const spread = (i - 1) * 2.4;
    ctx.beginPath();
    ctx.moveTo(spread * 0.3, -9);
    ctx.quadraticCurveTo(spread, -14, spread * 1.5 - 2, -19 - i);
    ctx.strokeStyle = pal.trim;
    ctx.lineWidth = 1.3;
    ctx.lineCap = 'round';
    ctx.stroke();
  }
  ctx.lineCap = 'butt';
}

function drawDao(ctx, pal) {
  const L = 45;
  const curve = DAO_CURVE;
  const spine = [];
  const edge = [];
  for (let i = 0; i <= 8; i += 1) {
    const t = i / 8;
    const a = curve * t * t;
    const x = Math.sin(a) * L * t * 1.06;
    const y = 5 + Math.cos(a) * (L - 5) * t;
    const w = (2.9 + 1.7 * Math.sin(t * Math.PI * 0.7)) * (1 - t * t * 0.8);
    spine.push({ x: x - Math.cos(a) * w * 0.35, y: y + Math.sin(a) * w * 0.35 });
    edge.push({ x: x + Math.cos(a) * w, y: y - Math.sin(a) * w });
  }
  polyPath(ctx, [...edge, ...spine.slice().reverse()]);
  fillInk(ctx, pal.metal, pal.ink, 1);

  ctx.beginPath();
  ctx.moveTo(spine[0].x, spine[0].y);
  for (let i = 1; i < spine.length; i += 1) ctx.lineTo(spine[i].x, spine[i].y);
  ctx.strokeStyle = pal.metalShade;
  ctx.lineWidth = 1.1;
  ctx.stroke();

  // Oval guard, angled — reads differently from the ally's flat disc. Kept
  // small: at 4.6 units it read as a grey donut sitting on the other
  // fighter's chest.
  ctx.beginPath();
  ctx.ellipse(0.4, 4.2, 3.4, 1.8, 0.35, 0, Math.PI * 2);
  fillInk(ctx, pal.metalShade, pal.ink, 0.9);

  ctx.beginPath();
  ctx.rect(-1.8, -6.2, 3.6, 10.4);
  fillInk(ctx, pal.hilt, pal.ink, 1);

  // Ring pommel.
  ctx.beginPath();
  ctx.arc(0, -8.2, 2.1, 0, Math.PI * 2);
  ctx.strokeStyle = pal.metalShade;
  ctx.lineWidth = 1.3;
  ctx.stroke();
}

/** The blade a fallen combatant is no longer holding. Lies flat on the ground. */
function drawDroppedBlade(ctx, pal, side, seed) {
  ctx.save();
  // In front of the kneeling figure, lying along the ground and foreshortened
  // — a blade at full length on the floor spans a whole hex and stops reading
  // as a dropped weapon.
  ctx.translate(5 + (seed % 3), 1.6);
  // The two blades are dropped one hex apart and both point forward, so a
  // shared angle put the ally's tip across the enemy's. Spreading them keeps
  // each weapon inside its owner's cell.
  ctx.rotate(-Math.PI * 0.5 + (side === 'ally' ? 0.46 : 0.06));
  ctx.scale(0.66, 0.66);
  if (side === 'ally') drawJian(ctx, pal);
  else drawDao(ctx, pal);
  ctx.restore();
}

/**
 * 청류문 검수's hair, below the shoulders — the same verlet chain the hem uses,
 * anchored at the nape.
 *
 * This is the single strongest silhouette cue she has. 산발 means unbound, and
 * a long unbound mass that swings with the body is something a bone rig gets
 * almost free once the cloth solver exists; a static curve in the same place
 * reads as a cape. The clumps split at the tip rather than ending in one
 * blunt edge, and two strands escape and curl outward.
 */
function drawLooseHair(ctx, joints, hair, pal) {
  const head = joints[BONE_INDEX.head];
  const pts = hair.pts;
  const widths = [11.5, 17.5, 18, 14, 8];
  const spine = [{ x: head.baseX, y: head.baseY + 2.5 }, ...pts.slice(1)].map((p, i) => ({
    x: p.x - (i === 0 ? 0 : 1.6 + i * 1.7),
    y: p.y,
  }));
  const { front, back } = offsetPolyline(spine, widths);

  // Main mass, split into two clumps at the tip so the end is not a blunt cut.
  const tip = spine[spine.length - 1];
  polyPath(ctx, [
    ...front,
    { x: tip.x + 1.6, y: tip.y - 4.6 },
    { x: tip.x - 0.4, y: tip.y - 1.4 },
    { x: tip.x - 2.8, y: tip.y - 5.6 },
    ...back.slice().reverse(),
  ]);
  ctx.fillStyle = pal.hair;
  ctx.fill();

  // One highlight band down the mass.
  ctx.beginPath();
  ctx.moveTo(spine[0].x - 2, spine[0].y);
  for (let i = 1; i < spine.length; i += 1) {
    const t = i / (spine.length - 1);
    ctx.lineTo(spine[i].x - 2.5 + t * 1.5, spine[i].y);
  }
  ctx.strokeStyle = pal.hairLit;
  ctx.globalAlpha = 0.4;
  ctx.lineWidth = 2.4;
  ctx.lineCap = 'round';
  ctx.stroke();
  ctx.globalAlpha = 1;
  ctx.lineCap = 'butt';

  // Escaped strands curling outward off the mass.
  const mid = spine[Math.max(1, Math.floor(spine.length / 2))];
  ctx.fillStyle = pal.hair;
  for (const [ax, ay, bx, by, cxx, cyy, w] of [
    [back[1].x, back[1].y, mid.x - 9, mid.y + 3, mid.x - 13, mid.y - 4, 1.2],
    [back[2].x, back[2].y, tip.x - 8, tip.y + 5, tip.x - 12, tip.y - 2, 1.0],
  ]) {
    ctx.beginPath();
    ctx.moveTo(ax, ay);
    ctx.quadraticCurveTo(bx, by, cxx, cyy);
    ctx.quadraticCurveTo(bx + w * 2, by - w * 2, ax + w, ay - w);
    ctx.closePath();
    ctx.fill();
  }
}

/**
 * The 삿갓, pushed off the head and hanging down her back on its cord.
 *
 * The hat used to sit on 흑사방 도객's head with her face inside its shadow.
 * Her face is now specified to be visible, but the hat was carrying real
 * silhouette load — worn on the back is the standard wuxia answer and keeps
 * the cue while freeing the face. Drawn edge-on and foreshortened, because a
 * disc seen from behind the shoulder is an ellipse, not a circle.
 */
function drawSlungSatgat(ctx, joints, pal) {
  const chest = joints[BONE_INDEX.chest];
  const d = dirOf(chest);
  const cx = chest.baseX + (chest.tipX - chest.baseX) * 0.55 - d.y * 11.5;
  const cy = chest.baseY + (chest.tipY - chest.baseY) * 0.55 + d.x * 11.5;
  ctx.save();
  ctx.translate(cx, cy);
  ctx.rotate(-d.a + 0.42);
  ctx.beginPath();
  ctx.ellipse(0, 0, 6.4, 15.5, 0, 0, Math.PI * 2);
  fillInk(ctx, pal.clothShade, pal.ink, 1.2);
  ctx.beginPath();
  ctx.ellipse(-2.0, 0, 2.2, 13.5, 0, 0, Math.PI * 2);
  ctx.fillStyle = pal.clothLit;
  ctx.globalAlpha = 0.5;
  ctx.fill();
  ctx.globalAlpha = 1;
  ctx.beginPath();
  ctx.ellipse(0.4, 0, 1.7, 4.6, 0, 0, Math.PI * 2);
  ctx.strokeStyle = pal.trimMetal;
  ctx.globalAlpha = 0.5;
  ctx.lineWidth = 0.6;
  ctx.stroke();
  ctx.globalAlpha = 1;
  ctx.restore();
}

// ---------------------------------------------------------------------------
// Head and face
//
// ## The line-weight decision
//
// This renderer is an *inked silhouette*: every filled shape gets a hard
// keyline. The faces these characters need are painterly — form carried by
// value and soft edges, very little contrast, no outline. Those two do not
// coexist by accident; a soft face dropped onto a hard-inked body reads as two
// drawings glued together.
//
// The resolution taken here is the comic one: **line weight varies by
// importance.** Cloth, steel and boots keep their full keyline, because that
// is what makes the figure hold together as a silhouette at 18 px. Skin gets
// *no* keyline at all — the head's edge is carried by value against the hair,
// the collar and the dark board behind it, and all of the modelling inside it
// is soft gradient. Hair keeps an edge only because hair is nearly black
// anyway. The eyes are the exception that proves it: the lash is heavy and
// hard, but it is a *feature*, not a contour.
//
// The two rejected options, for the record: going soft everywhere would cost
// the body its silhouette read at phone scale, which is the whole premise of
// the style; going fully cel on the face would be coherent but would throw
// away the delicacy these two characters are specified to have.
//
// ## Construction
//
//   1. **The eye is separable parts**, drawn in order: a large, round-ish
//      aperture; the iris, dark at the top under the lid's shadow, lighter
//      toward the bottom, with a bright rim along its *lower* edge; a small
//      pupil; one hard specular at the upper inner iris plus a soft
//      counter-glint; a soft-edged upper lash heaviest at its outer third; and
//      a lower lid line that deliberately does not close the shape.
//   2. **Skin is nearly flat**, with value only under the jaw, under the
//      hairline, a warm blush on the cheek, and a small highlight on the lower
//      lip.
//   3. **Brows are thin and low contrast**, close to the eye. Their angle
//      carries most of the emotion, with the upper lid.
//   4. **Hair is chunky clumps with outward-curling escaped strands.** For
//      청류문 검수 that is the whole point — 산발, unbound and a little wild, is
//      the cue that separates her from a bound-haired opponent in pure black.
//      흑사방 도객 is the deliberate opposite: slicked back, one high tight
//      ponytail, nothing loose.
//   5. **The nose is a shadow, not an outline. The mouth is small.**
//
// Every part is driven by the expression channels `rig.js::expressionFor`
// produces from the same SIGNALS the body uses. Nothing here is placed per
// frame: `attack` furrows the brow and sets the jaw, `hit` drives the lids
// down and the brow up, `incapacitated` goes slack.
//
// Local face space: origin at the head's centre, +X is the facing direction,
// +Y is up, and the skull spans roughly y ∈ [-10, +13]. Callers scale into it.
// ---------------------------------------------------------------------------

/** Half-height of the skull in face space; the scale contract for callers. */
const FACE_UNIT = 11.5;

/**
 * Per-character facial structure. Both are 미형 and both are women in their
 * early twenties, so the difference is in the *angles*, not in a caricature:
 * 청류문 검수 is sharp and level, 흑사방 도객 is upswept.
 */
const FACE_SHAPE = {
  ally: { eyeTilt: 0.1, browTilt: 0.02, lash: 1, lip: 1 },
  enemy: { eyeTilt: 0.2, browTilt: 0.09, lash: 0.95, lip: 1.06 },
};

/** Near eye (viewer side) and far eye (nose side, compressed by the turn). */
const EYES = {
  near: { cx: 0.6, cy: 1.3, w: 6.4, h: 5.0, weight: 1 },
  far: { cx: 6.9, cy: 1.8, w: 3.6, h: 4.4, weight: 0.62 },
};

function skullPath(ctx) {
  ctx.beginPath();
  ctx.moveTo(-8.0, 3.0);
  ctx.bezierCurveTo(-8.4, 8.8, -4.4, 12.2, 1.0, 12.2);
  ctx.bezierCurveTo(5.4, 12.2, 8.2, 8.6, 8.5, 4.0);
  ctx.bezierCurveTo(8.6, 2.4, 8.3, 1.4, 8.8, 0.3);
  ctx.bezierCurveTo(10.0, -1.5, 9.5, -2.5, 8.1, -3.1);
  ctx.bezierCurveTo(8.5, -4.1, 8.3, -4.8, 7.5, -5.4);
  ctx.bezierCurveTo(7.8, -6.8, 6.4, -8.4, 4.2, -9.2);
  ctx.bezierCurveTo(1.9, -9.9, -1.3, -8.7, -3.9, -6.5);
  ctx.bezierCurveTo(-6.2, -4.4, -7.6, -1.2, -8.0, 3.0);
  ctx.closePath();
}

/** Sample a cubic bezier, so a lid line can taper instead of being a stroke. */
function sampleBezier(p0, c0, c1, p1, n) {
  const out = [];
  for (let i = 0; i <= n; i += 1) {
    const t = i / n;
    const u = 1 - t;
    out.push({
      x: u * u * u * p0.x + 3 * u * u * t * c0.x + 3 * u * t * t * c1.x + t * t * t * p1.x,
      y: u * u * u * p0.y + 3 * u * u * t * c0.y + 3 * u * t * t * c1.y + t * t * t * p1.y,
    });
  }
  return out;
}

/**
 * One eye.
 *
 * `lid`/`squint`/`widen` move the lid *curve* rather than selecting a
 * different drawing, which is what lets the expression states blend
 * continuously instead of popping.
 */
function drawEye(ctx, pal, e, g, shape, detail) {
  const halfW = g.w / 2;
  const h = g.h;
  const tilt = shape.eyeTilt;
  const inner = { x: g.cx + halfW, y: g.cy - h * 0.12 - tilt * h * 0.3 };
  const outer = { x: g.cx - halfW, y: g.cy + h * 0.04 + tilt * h * 0.32 };
  const loBot = g.cy - h * 0.46 + e.squint * h * 0.38;
  const upTop = Math.max(
    loBot + h * 0.05,
    g.cy + h * (0.5 + e.widen * 0.16) - e.lid * h * 0.9,
  );
  const c0 = { x: g.cx + halfW * 0.5, y: upTop };
  const c1 = { x: g.cx - halfW * 0.34, y: upTop + tilt * h * 0.2 };

  ctx.beginPath();
  ctx.moveTo(inner.x, inner.y);
  ctx.bezierCurveTo(c0.x, c0.y, c1.x, c1.y, outer.x, outer.y);
  ctx.bezierCurveTo(g.cx - halfW * 0.44, loBot, g.cx + halfW * 0.36, loBot, inner.x, inner.y);
  ctx.closePath();
  const sg = ctx.createLinearGradient(0, upTop, 0, loBot);
  sg.addColorStop(0, pal.scleraShade);
  sg.addColorStop(0.5, pal.sclera);
  ctx.fillStyle = sg;
  ctx.fill();

  ctx.save();
  ctx.clip();

  const ir = h * 0.6;
  const ix = g.cx + g.w * 0.04 + e.gaze * g.w * 0.14;
  const iy = g.cy + h * 0.02;
  ctx.beginPath();
  ctx.ellipse(ix, iy, ir, ir, 0, 0, Math.PI * 2);
  const ig = ctx.createLinearGradient(0, iy + ir, 0, iy - ir);
  ig.addColorStop(0, pal.irisLit);
  ig.addColorStop(0.42, pal.irisMid);
  ig.addColorStop(1, pal.irisTop);
  ctx.fillStyle = ig;
  ctx.fill();

  // Bright rim along the *bottom* edge of the iris — light passing through it.
  ctx.beginPath();
  ctx.ellipse(ix, iy + ir * 0.1, ir * 0.94, ir * 0.94, 0, Math.PI * 0.16, Math.PI * 0.84);
  ctx.strokeStyle = pal.irisRim;
  ctx.globalAlpha = 0.75;
  ctx.lineWidth = h * 0.12;
  ctx.stroke();
  ctx.globalAlpha = 1;

  ctx.beginPath();
  ctx.ellipse(ix, iy, ir, ir, 0, 0, Math.PI * 2);
  ctx.strokeStyle = pal.lash;
  ctx.globalAlpha = 0.3;
  ctx.lineWidth = h * 0.09;
  ctx.stroke();
  ctx.globalAlpha = 1;

  ctx.beginPath();
  ctx.ellipse(ix, iy, ir * 0.4, ir * 0.44, 0, 0, Math.PI * 2);
  ctx.fillStyle = pal.pupil;
  ctx.fill();

  // Lid shadow riding under the lash, soft.
  ctx.beginPath();
  ctx.moveTo(inner.x, inner.y);
  ctx.bezierCurveTo(c0.x, c0.y, c1.x, c1.y, outer.x, outer.y);
  ctx.strokeStyle = pal.skinDeep;
  ctx.globalAlpha = 0.16;
  ctx.lineWidth = h * 0.22;
  ctx.stroke();
  ctx.globalAlpha = 1;

  ctx.beginPath();
  ctx.ellipse(ix + ir * 0.32, iy + ir * 0.38, ir * 0.32, ir * 0.26, -0.45, 0, Math.PI * 2);
  ctx.fillStyle = pal.glint;
  ctx.fill();
  if (detail === 'full') {
    ctx.beginPath();
    ctx.ellipse(ix - ir * 0.42, iy - ir * 0.46, ir * 0.17, ir * 0.14, 0, 0, Math.PI * 2);
    ctx.globalAlpha = 0.45;
    ctx.fill();
    ctx.globalAlpha = 1;
  }
  ctx.restore();

  if (detail === 'full') {
    ctx.beginPath();
    ctx.moveTo(outer.x + halfW * 0.24, outer.y - h * 0.1);
    ctx.quadraticCurveTo(g.cx - halfW * 0.05, loBot - h * 0.02, g.cx + halfW * 0.4, g.cy - h * 0.24);
    ctx.strokeStyle = pal.skinDeep;
    ctx.globalAlpha = 0.26;
    ctx.lineWidth = h * 0.07;
    ctx.lineCap = 'round';
    ctx.stroke();
    ctx.globalAlpha = 1;
    ctx.lineCap = 'butt';
  }

  // Upper lash: a tapered mass heaviest at its outer third, softer than the
  // hard blade the earlier version used.
  const lid = sampleBezier(inner, c0, c1, outer, 10);
  const lw = h * (0.17 * g.weight + 0.04) * shape.lash;
  const widths = lid.map((_, i) => {
    const t = i / (lid.length - 1);
    return lw * (0.3 + 1.15 * Math.sin(Math.PI * Math.min(1, t * 0.82 + 0.09)) ** 1.5);
  });
  taperedPath(ctx, lid, widths);
  ctx.fillStyle = pal.lash;
  ctx.fill();

  const spikes = g.weight > 0.9 ? 3 : 2;
  for (let i = 0; i < spikes; i += 1) {
    const t = 0.84 + i * 0.06;
    const base = lid[Math.min(lid.length - 1, Math.round(t * (lid.length - 1)))];
    const len = h * (0.3 - i * 0.055);
    ctx.beginPath();
    ctx.moveTo(base.x + lw * 0.22, base.y - lw * 0.1);
    ctx.lineTo(base.x - len * 0.95, base.y + len * 0.66);
    ctx.lineTo(base.x - lw * 0.3, base.y + lw * 0.35);
    ctx.closePath();
    ctx.fill();
  }
}

/** Thin, soft-edged, close to the eye. Angle does the emotional work. */
function drawBrow(ctx, pal, e, g, shape) {
  const baseY = g.cy + g.h * 0.72;
  const innerX = g.cx + g.w * 0.46;
  const outerX = g.cx - g.w * 0.56;
  const innerY = baseY + e.browInner * g.h * 0.34 - shape.browTilt * g.h;
  const outerY = baseY + g.h * 0.08 + e.browOuter * g.h * 0.28 + shape.browTilt * g.h;
  const mid = { x: (innerX + outerX) / 2, y: (innerY + outerY) / 2 + g.h * 0.12 };
  taperedPath(
    ctx,
    [{ x: innerX, y: innerY }, mid, { x: outerX, y: outerY }],
    [g.h * 0.125, g.h * 0.105, g.h * 0.04],
  );
  ctx.fillStyle = pal.brow;
  ctx.globalAlpha = 0.6;
  ctx.fill();
  ctx.globalAlpha = 1;
}

/** Nearly absent: a shadow mark and a nostril, no outline anywhere. */
function drawNose(ctx, pal) {
  ctx.beginPath();
  ctx.moveTo(8.2, 0.7);
  ctx.quadraticCurveTo(9.5, -1.3, 7.9, -2.5);
  ctx.strokeStyle = pal.skinShade;
  ctx.globalAlpha = 0.5;
  ctx.lineWidth = 0.55;
  ctx.lineCap = 'round';
  ctx.stroke();
  ctx.beginPath();
  ctx.ellipse(7.6, -2.75, 0.48, 0.3, 0.35, 0, Math.PI * 2);
  ctx.fillStyle = pal.skinDeep;
  ctx.globalAlpha = 0.42;
  ctx.fill();
  ctx.globalAlpha = 1;
  ctx.lineCap = 'butt';
}

/** Small, with a highlight on the lower lip. */
function drawMouth(ctx, pal, e, shape, detail) {
  const cx = 5.2;
  const cy = -5.7;
  const w = 3.4 * shape.lip * (1 - e.jaw * 0.12);
  const open = e.mouthOpen;
  const curve = e.mouthCurve;

  if (open > 0.06) {
    ctx.beginPath();
    ctx.moveTo(cx - w * 0.44, cy + 0.1);
    ctx.quadraticCurveTo(cx, cy + 0.3, cx + w * 0.44, cy - 0.15);
    ctx.quadraticCurveTo(cx + w * 0.05, cy - open * 2.7, cx - w * 0.44, cy + 0.1);
    ctx.closePath();
    ctx.fillStyle = pal.mouthDark;
    ctx.fill();
  }

  ctx.beginPath();
  ctx.moveTo(cx - w / 2, cy + curve * 0.5);
  ctx.quadraticCurveTo(cx, cy - curve * 1.0 - open * 0.3, cx + w / 2, cy + curve * 0.28);
  ctx.strokeStyle = pal.mouthDark;
  ctx.globalAlpha = 0.72;
  ctx.lineWidth = 0.5;
  ctx.lineCap = 'round';
  ctx.stroke();
  ctx.globalAlpha = 1;
  ctx.lineCap = 'butt';

  if (detail === 'full') {
    ctx.beginPath();
    ctx.ellipse(cx + w * 0.06, cy - 0.85 - open * 1.5, w * 0.24, 0.34, -0.1, 0, Math.PI * 2);
    ctx.fillStyle = pal.lipLit;
    ctx.globalAlpha = 0.5;
    ctx.fill();
    ctx.globalAlpha = 1;
  }
}

/**
 * Skin. No keyline, and nearly flat.
 *
 * Value lives in four places only: under the jaw, under the hairline, a warm
 * blush on the cheek, and the lip highlight above. An earlier version ran a
 * terminator diagonally across the cheek and every face came back reading as a
 * mask lit from stage left.
 */
function fillSkin(ctx, pal) {
  skullPath(ctx);
  ctx.fillStyle = pal.skin;
  ctx.fill();

  ctx.save();
  skullPath(ctx);
  ctx.clip();

  const j = ctx.createLinearGradient(0, -9.2, 0, -3.0);
  j.addColorStop(0, pal.skinShade);
  j.addColorStop(1, 'rgba(0,0,0,0)');
  ctx.globalAlpha = 0.4;
  ctx.fillStyle = j;
  ctx.fillRect(-10, -10, 22, 7);

  const b = ctx.createLinearGradient(0, 12.4, 0, 5.2);
  b.addColorStop(0, pal.skinShade);
  b.addColorStop(1, 'rgba(0,0,0,0)');
  ctx.globalAlpha = 0.34;
  ctx.fillStyle = b;
  ctx.fillRect(-10, 5, 22, 8);

  ctx.globalAlpha = 0.2;
  ctx.fillStyle = pal.blush;
  ctx.beginPath();
  ctx.ellipse(3.2, -1.5, 4.2, 1.7, -0.12, 0, Math.PI * 2);
  ctx.fill();
  ctx.globalAlpha = 0.12;
  ctx.beginPath();
  ctx.ellipse(7.2, 0.1, 1.6, 1.2, 0, 0, Math.PI * 2);
  ctx.fill();
  ctx.globalAlpha = 1;
  ctx.restore();
}

/**
 * An escaped strand: a clump that leaves the mass and curls outward at the tip.
 * Several of these are what "unbound and a little wild" actually looks like.
 */
function curlStrand(ctx, pal, x0, y0, cx1, cy1, x1, y1, curlX, curlY, w) {
  ctx.beginPath();
  ctx.moveTo(x0, y0);
  ctx.quadraticCurveTo(cx1, cy1, x1, y1);
  ctx.quadraticCurveTo(curlX, curlY, x1 + (curlX - x1) * 0.35, y1 + (curlY - y1) * 1.15);
  ctx.quadraticCurveTo(cx1 + w, cy1 + w * 0.5, x0 + w, y0);
  ctx.closePath();
  ctx.fillStyle = pal.hair;
  ctx.fill();
}

/**
 * 청류문 검수 — 산발. Chunky clumps with escaped strands curling outward, no
 * band, no knot, nothing tied. The long mass down her back is drawn by the
 * body renderer from a verlet chain; this is the part that frames the face.
 */
function drawHairAlly(ctx, pal, detail) {
  ctx.fillStyle = pal.hair;

  // Crown mass, hairline dipping low and unevenly at the front.
  ctx.beginPath();
  ctx.moveTo(-8.6, -3.0);
  ctx.bezierCurveTo(-10.4, 5.0, -6.4, 12.9, 0.6, 13.3);
  ctx.bezierCurveTo(5.8, 13.6, 9.0, 9.6, 9.1, 3.4);
  ctx.bezierCurveTo(8.4, 6.4, 6.2, 7.6, 3.4, 6.6);
  ctx.bezierCurveTo(1.4, 5.9, -0.4, 7.0, -2.2, 6.4);
  ctx.bezierCurveTo(-4.4, 5.7, -5.6, 7.2, -7.0, 6.4);
  ctx.bezierCurveTo(-8.4, 5.4, -8.2, 0.6, -8.6, -3.0);
  ctx.closePath();
  ctx.fill();

  if (detail === 'full') {
    ctx.beginPath();
    ctx.moveTo(-5.4, 10.0);
    ctx.quadraticCurveTo(0.4, 14.0, 6.4, 10.0);
    ctx.quadraticCurveTo(0.8, 12.4, -4.8, 9.2);
    ctx.closePath();
    ctx.fillStyle = pal.hairLit;
    ctx.globalAlpha = 0.5;
    ctx.fill();
    ctx.globalAlpha = 1;
    ctx.fillStyle = pal.hair;
  }

  // Front clump falling across the far brow.
  ctx.beginPath();
  ctx.moveTo(1.6, 12.4);
  ctx.bezierCurveTo(5.6, 9.2, 8.4, 5.4, 8.8, 0.4);
  ctx.bezierCurveTo(7.6, 5.0, 4.6, 8.4, 0.4, 10.8);
  ctx.closePath();
  ctx.fill();

  // Side clump in front of the ear, ending in a hook.
  ctx.beginPath();
  ctx.moveTo(-6.4, 8.6);
  ctx.bezierCurveTo(-10.2, 3.0, -10.6, -3.6, -9.0, -8.6);
  ctx.bezierCurveTo(-7.2, -6.8, -6.8, -3.6, -7.2, -1.2);
  ctx.bezierCurveTo(-6.4, -3.0, -5.4, 3.2, -4.4, 8.0);
  ctx.closePath();
  ctx.fill();

  // Escaped strands, curling outward and breaking the outline.
  if (detail === 'full') {
    curlStrand(ctx, pal, 3.6, 11.4, 7.6, 9.0, 8.8, 5.0, 10.8, 3.2, 0.5);
    curlStrand(ctx, pal, -6.6, 9.4, -10.0, 7.2, -11.0, 3.0, -13.0, 1.4, 0.5);
    curlStrand(ctx, pal, -8.0, 4.0, -10.8, 0.6, -10.6, -3.6, -12.6, -6.0, 0.45);
  } else {
    curlStrand(ctx, pal, -6.6, 9.4, -10.0, 7.2, -11.0, 3.0, -13.0, 1.4, 0.9);
    curlStrand(ctx, pal, 3.6, 11.4, 7.6, 9.0, 8.8, 5.0, 10.8, 3.2, 0.9);
  }
}

/**
 * 흑사방 도객 — the deliberate opposite: slicked back and pinned, one high
 * tight ponytail, nothing escaping. In pure black that reads as a smooth skull
 * with a single hard spike off the back, against the ally's ragged corona.
 */
function drawHairEnemyBack(ctx, pal) {
  ctx.fillStyle = pal.hair;
  // The tail only. The crown goes on *top* of the skin — drawing the whole
  // head of hair before `fillSkin` painted the crown out and every capture
  // came back with a bald assassin.
  ctx.beginPath();
  ctx.moveTo(-6.4, 8.2);
  ctx.bezierCurveTo(-11.0, 8.6, -15.4, 4.6, -18.6, -3.4);
  ctx.bezierCurveTo(-14.4, 3.0, -10.6, 5.6, -6.0, 5.6);
  ctx.closePath();
  ctx.fill();
  ctx.beginPath();
  ctx.moveTo(-6.2, 8.4);
  ctx.bezierCurveTo(-9.6, 8.0, -12.6, 5.0, -14.8, 0.6);
  ctx.bezierCurveTo(-11.8, 4.4, -9.2, 6.0, -6.0, 6.2);
  ctx.closePath();
  ctx.fill();
}

function drawHairEnemyFront(ctx, pal, detail) {
  ctx.fillStyle = pal.hair;

  // Slicked crown: a clean unbroken sweep, hairline high and even.
  ctx.beginPath();
  ctx.moveTo(-8.4, -2.4);
  ctx.bezierCurveTo(-9.4, 5.6, -5.6, 12.7, 0.8, 13.0);
  ctx.bezierCurveTo(5.6, 13.2, 8.6, 9.2, 8.8, 3.8);
  ctx.bezierCurveTo(8.2, 6.8, 5.6, 8.0, 2.4, 7.6);
  ctx.bezierCurveTo(-1.2, 7.2, -5.2, 7.6, -7.6, 6.2);
  ctx.bezierCurveTo(-8.4, 4.4, -8.2, 0.4, -8.4, -2.4);
  ctx.closePath();
  ctx.fill();

  if (detail === 'full') {
    ctx.beginPath();
    ctx.moveTo(-5.0, 10.2);
    ctx.quadraticCurveTo(0.6, 13.6, 6.2, 10.4);
    ctx.quadraticCurveTo(0.8, 12.2, -4.4, 9.6);
    ctx.closePath();
    ctx.fillStyle = pal.hairLit;
    ctx.globalAlpha = 0.45;
    ctx.fill();
    ctx.globalAlpha = 1;
    ctx.fillStyle = pal.hair;
  }

  ctx.beginPath();
  ctx.ellipse(-6.6, 6.9, 1.5, 1.9, 0.3, 0, Math.PI * 2);
  ctx.fillStyle = pal.trimMetal;
  ctx.globalAlpha = 0.9;
  ctx.fill();
  ctx.globalAlpha = 1;
}

/**
 * The whole face, in face space. Callers set up the transform.
 *
 * `detail` drops the parts that are sub-pixel on the board figure — the
 * counter-glint, the lower-lid line, the nose, the lip highlight, most of the
 * escaped strands. It never changes a shape, so the 18 px board head and the
 * portrait are the same face rather than two drawings that resemble each other.
 */
export function drawFace2D(ctx, pal, side, e, detail = 'full') {
  const shape = FACE_SHAPE[side];

  ctx.beginPath();
  ctx.moveTo(-3.2, -6.0);
  ctx.lineTo(3.0, -6.9);
  ctx.lineTo(3.8, -17);
  ctx.lineTo(-4.0, -17);
  ctx.closePath();
  ctx.fillStyle = pal.skinShade;
  ctx.fill();

  if (side === 'enemy') drawHairEnemyBack(ctx, pal);

  fillSkin(ctx, pal);

  ctx.beginPath();
  ctx.ellipse(-6.4, -1.2, 1.05, 1.7, -0.22, 0, Math.PI * 2);
  ctx.fillStyle = pal.skinMid;
  ctx.fill();
  ctx.beginPath();
  ctx.ellipse(-6.5, -1.4, 0.45, 0.95, -0.22, 0, Math.PI * 2);
  ctx.fillStyle = pal.skinShade;
  ctx.globalAlpha = 0.7;
  ctx.fill();
  ctx.globalAlpha = 1;

  // Features clipped to the skull: this is what sells the three-quarter turn,
  // because the far eye is *cut* by the front contour instead of floating
  // clear of it.
  ctx.save();
  skullPath(ctx);
  ctx.clip();
  if (detail === 'full') drawNose(ctx, pal);
  drawEye(ctx, pal, e, EYES.far, shape, detail);
  drawEye(ctx, pal, e, EYES.near, shape, detail);
  drawBrow(ctx, pal, e, EYES.far, shape);
  drawBrow(ctx, pal, e, EYES.near, shape);
  drawMouth(ctx, pal, e, shape, detail);
  ctx.restore();

  if (side === 'ally') drawHairAlly(ctx, pal, detail);
  else drawHairEnemyFront(ctx, pal, detail);
}

/**
 * Head, positioned on the skeleton. The face is drawn in its own unit system
 * and scaled into the head bone's, so one geometry serves the ~18 px board
 * figure and the portrait card.
 */
function drawHead(ctx, joints, pal, side, expr) {
  const head = joints[BONE_INDEX.head];
  const d = dirOf(head);
  const cx = head.baseX + d.x * 4.2;
  const cy = head.baseY + d.y * 4.2;
  ctx.save();
  ctx.translate(cx, cy);
  ctx.rotate(-d.a);
  const k = 5.6 / FACE_UNIT;
  ctx.scale(k, k);
  drawFace2D(ctx, pal, side, expr, 'small');
  ctx.restore();
}

/**
 * A portrait cut-in: the same rig face, drawn big enough to actually read.
 *
 * This is the honest answer to the scale problem. On a 390 px phone the board
 * head is ~18 px tall, and a face drawn there is a rumour no matter how well
 * it is constructed. The genre already has the convention — a portrait beside
 * the vitals — and it costs nothing here, because the portrait and the board
 * head are the same function fed the same channels.
 */
export function drawPortrait2D(ctx, w, h, side, expr, opts = {}) {
  const { mirror = false } = opts;
  const pal = PAL2D[side];
  ctx.save();
  ctx.beginPath();
  ctx.rect(0, 0, w, h);
  ctx.clip();

  const bg = ctx.createLinearGradient(0, 0, 0, h);
  bg.addColorStop(0, side === 'ally' ? '#25403c' : '#33222e');
  bg.addColorStop(1, side === 'ally' ? '#101d1e' : '#150e13');
  ctx.fillStyle = bg;
  ctx.fillRect(0, 0, w, h);

  // Frame on the head, not on the canvas: the visible head spans about 30
  // units and wants ~78% of the frame with its midpoint centred.
  const scale = h / 38;
  ctx.translate(w * (mirror ? 0.46 : 0.54), h * 0.63);
  ctx.scale(mirror ? -scale : scale, -scale);
  ctx.lineJoin = 'round';

  // Shoulders, so the head is not a floating bust. Cloth keeps its keyline —
  // the line-weight split is between skin and everything else, not between
  // portrait and board.
  ctx.beginPath();
  ctx.moveTo(-15, -21);
  ctx.bezierCurveTo(-12, -12.6, -5.4, -9.4, -3.4, -8.4);
  ctx.lineTo(3.2, -9.4);
  ctx.bezierCurveTo(6.4, -10.4, 12, -13.2, 15, -21);
  ctx.closePath();
  fillInk(ctx, pal.cloth, pal.ink, 0.5);
  ctx.beginPath();
  ctx.moveTo(-3.4, -8.4);
  ctx.lineTo(0.4, -14.4);
  ctx.lineTo(3.2, -9.4);
  ctx.closePath();
  ctx.fillStyle = pal.clothShade;
  ctx.fill();
  ctx.beginPath();
  ctx.moveTo(-4.2, -8.2);
  ctx.lineTo(0.4, -15.0);
  ctx.lineTo(4.0, -9.2);
  ctx.strokeStyle = pal.trimMetal;
  ctx.globalAlpha = 0.8;
  ctx.lineWidth = 0.4;
  ctx.stroke();
  ctx.globalAlpha = 1;

  drawFace2D(ctx, pal, side, expr, 'full');
  ctx.restore();
}

// ---------------------------------------------------------------------------
// Cues
// ---------------------------------------------------------------------------

/**
 * 검로 — the arc the blade has just travelled through.
 *
 * Built from the character's *own* earlier-phase skeletons rather than from a
 * synthetic arc. The first version swept 2.1 rad at shoulder-plus-blade radius
 * and drew concentric rings clean across the frame, which read as background
 * decoration instead of as one cut. Sampling the real tip path keeps the trail
 * inside the figure's swing by construction, and it tapers to nothing at the
 * oldest sample so it never competes with the blade itself.
 */
function drawCutTrail(ctx, joints, trailJoints, side) {
  if (trailJoints.length < 2) return;
  const now = bladeTip(joints, side);
  // The current tip is deliberately *not* a trail sample. Including it centred
  // the ribbon's widest section on the tip, so half of it stuck out past the
  // point and read as a detached white shard floating off the blade.
  const all = trailJoints.map((js) => bladeTip(js, side));

  // Hard length budget. Even sampled from the real swing, the windup phases
  // put the tip a whole body-height away, and the resulting streak read as
  // background decoration in the first capture — the exact defect the arc
  // version had. A cut is legible over about a third of a body height.
  const BUDGET = 30;
  const tips = all.filter((p) => Math.hypot(p.x - now.x, p.y - now.y) <= BUDGET);
  if (tips.length < 2) return;

  const widths = tips.map((_, i) => 0.5 + (i / (tips.length - 1)) ** 1.4 * 5.4);
  taperedPath(ctx, tips, widths);
  ctx.save();
  const grad = ctx.createLinearGradient(tips[0].x, tips[0].y, tips[tips.length - 1].x, tips[tips.length - 1].y);
  grad.addColorStop(0, 'rgba(230,246,255,0)');
  grad.addColorStop(1, PAL2D.shared.trail);
  ctx.fillStyle = grad;
  ctx.globalAlpha = 0.5;
  ctx.fill();
  ctx.restore();
}

/** Where this side's blade tip sits for a given solved skeleton. */
function bladeTip(joints, side) {
  const hand = joints[BONE_INDEX.armNearHand];
  const d = dirOf(hand);
  const L = side === 'ally' ? 47 : 45;
  if (side === 'ally') return { x: hand.baseX + d.x * L, y: hand.baseY + d.y * L };
  // The 도 tip is offset forward by its curve; use the same numbers as drawDao.
  const a = DAO_CURVE;
  const lx = Math.sin(a) * L * 1.06;
  const ly = 5 + Math.cos(a) * (L - 5);
  return { x: hand.baseX + d.x * ly + d.y * lx, y: hand.baseY + d.y * ly - d.x * lx };
}

/** Grip and tip of this side's blade in rig space, for the caller's geometry. */
export function bladeSegmentWorld(pose, side) {
  const joints = solveSkeleton(pose);
  const hand = joints[BONE_INDEX.armNearHand];
  return { grip: { x: hand.baseX, y: hand.baseY }, tip: bladeTip(joints, side) };
}

/*
 * There is deliberately no per-body hit decal.
 *
 * Two versions of one existed: nine scattered spokes (which read as debris
 * pasted on the character) and, after that, a single crescent chip on the
 * ribs. The crescent survived one capture and failed the next for the same
 * underlying reason: at the size this figure is actually shown, a bright mark
 * on the torso is an *icon*, and the brief for this surface is that cues are
 * carried by the pose. `hit` already lands as an additive recoil modifier in
 * `rig.js` — the head snaps back, the hips shift off the line — and the
 * exchange itself is marked once, between the blades, by `drawClash`.
 */

/**
 * The clash flare, drawn in *screen* space between two blades that met on the
 * same tick. It belongs to the exchange rather than to either character, which
 * is why it is not part of `drawCharacter2D`.
 */
export function drawClash(ctx, x, y, radius, rand) {
  ctx.save();
  ctx.translate(x, y);
  const glow = ctx.createRadialGradient(0, 0, 0, 0, 0, radius * 1.4);
  glow.addColorStop(0, 'rgba(255,248,226,0.42)');
  glow.addColorStop(0.4, 'rgba(255,232,170,0.10)');
  glow.addColorStop(1, 'rgba(255,232,170,0)');
  ctx.beginPath();
  ctx.arc(0, 0, radius * 1.4, 0, Math.PI * 2);
  ctx.fillStyle = glow;
  ctx.fill();

  // Four shards on one axis, not a starburst. A starburst reads as an
  // explosion; steel on steel throws sparks along the line of the two edges.
  for (let i = 0; i < 4; i += 1) {
    const ang = -0.35 + rand() * 0.7 + (i % 2 === 0 ? 0 : Math.PI);
    const d0 = radius * (0.12 + rand() * 0.1);
    const d1 = d0 + radius * (0.3 + rand() * 0.36);
    const w = radius * 0.045;
    const nx = -Math.sin(ang);
    const ny = Math.cos(ang);
    ctx.beginPath();
    ctx.moveTo(Math.cos(ang) * d0 + nx * w, Math.sin(ang) * d0 + ny * w);
    ctx.lineTo(Math.cos(ang) * d1, Math.sin(ang) * d1);
    ctx.lineTo(Math.cos(ang) * d0 - nx * w, Math.sin(ang) * d0 - ny * w);
    ctx.closePath();
    ctx.fillStyle = PAL2D.shared.clash;
    ctx.globalAlpha = 0.4 + rand() * 0.3;
    ctx.fill();
  }
  ctx.globalAlpha = 0.9;
  ctx.beginPath();
  ctx.ellipse(0, 0, radius * 0.11, radius * 0.11, 0, 0, Math.PI * 2);
  ctx.fillStyle = PAL2D.shared.clash;
  ctx.fill();
  ctx.restore();
}

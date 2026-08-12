/**
 * Style 1 — 2D bone rig drawn as an inked silhouette (Canvas 2D).
 *
 * Consumes the solved skeleton from `rig.js` and draws a body around it. The
 * drawing vocabulary is deliberately small: tapered polygons along bone
 * chains, one filled robe built from the verlet hem, one keyline. There are no
 * facial features — partly because the art policy asks for them to be absent,
 * and partly because a silhouette that has to sell a pose is *helped* by not
 * having a face competing with it.
 *
 * The three things that make this read as a body rather than a stick figure:
 *   1. **Near/far limb pairs.** The far arm and leg are drawn first in a
 *      darker tone. Two-tone depth is what separates a fighting silhouette
 *      from a diagram.
 *   2. **Taper.** Limbs are wider at the root than at the tip, and sleeves
 *      flare the other way. Constant-width limbs read as tubes.
 *   3. **Cloth with inertia.** The hem carries motion from the previous tick,
 *      so a lunge drags the robe behind it.
 */

import { BONE_INDEX, HIP_HEIGHT, solveSkeleton } from './rig.js';

/** One palette per side. No colour literal appears at a draw site. */
export const PAL2D = {
  ally: {
    robe: '#2f7069',
    robeLit: '#3d8c83',
    robeShade: '#1b4744',
    ink: '#0c1a1c',
    sash: '#d8c79c',
    trim: '#8fd0c4',
    blade: '#d3dde2',
    skin: '#e8c9a8',
  },
  enemy: {
    robe: '#6d2632',
    robeLit: '#8a3340',
    robeShade: '#421620',
    ink: '#180a0e',
    sash: '#31262b',
    trim: '#c8697a',
    blade: '#d3dde2',
    skin: '#dcb493',
  },
  shadow: 'rgba(8,12,18,0.30)',
  spark: '#ffe9a8',
  trail: 'rgba(226,240,246,0.55)',
};

/**
 * Build a tapered polygon through a polyline.
 *
 * `widths` is per-point, so a limb can narrow toward the wrist while a sleeve
 * widens toward the cuff. The path is closed with round caps at both ends.
 */
function taperedPath(ctx, pts, widths) {
  const left = [];
  const right = [];
  for (let i = 0; i < pts.length; i += 1) {
    const prev = pts[Math.max(0, i - 1)];
    const next = pts[Math.min(pts.length - 1, i + 1)];
    let dx = next.x - prev.x;
    let dy = next.y - prev.y;
    const len = Math.hypot(dx, dy) || 1;
    dx /= len;
    dy /= len;
    const nx = -dy;
    const ny = dx;
    const w = widths[i] / 2;
    left.push({ x: pts[i].x + nx * w, y: pts[i].y + ny * w });
    right.push({ x: pts[i].x - nx * w, y: pts[i].y - ny * w });
  }
  // Caps are flat, closed straight across. An earlier version used `ctx.arc()`
  // here for rounded ends, which appends a *full circle subpath* rather than an
  // end cap — the first capture came back with a hard ring stamped at every
  // joint. Limbs overlap at the joints anyway, so flat caps are invisible.
  ctx.beginPath();
  ctx.moveTo(left[0].x, left[0].y);
  for (let i = 1; i < left.length; i += 1) ctx.lineTo(left[i].x, left[i].y);
  for (let i = right.length - 1; i >= 0; i -= 1) ctx.lineTo(right[i].x, right[i].y);
  ctx.closePath();
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
  const pts = [{ x: joints[BONE_INDEX[names[0]]].baseX, y: joints[BONE_INDEX[names[0]]].baseY }];
  for (const n of names) pts.push({ x: joints[BONE_INDEX[n]].tipX, y: joints[BONE_INDEX[n]].tipY });
  return pts;
}

/**
 * Draw one character.
 *
 * The canvas transform is set up so rig units map to pixels with +Y up and the
 * ground plane at the character's feet, which lets every number in `rig.js`
 * stay in one readable unit system.
 */
export function drawCharacter2D(ctx, cx, groundY, scale, side, pose, robe, signals, phase) {
  const pal = PAL2D[side];
  const joints = solveSkeleton(pose);

  ctx.save();
  ctx.translate(cx, groundY);
  ctx.scale(scale * signals.facing, -scale);

  drawShadow(ctx, pose, pal, signals);
  drawFarLimbs(ctx, joints, pal);
  drawRobe(ctx, joints, robe, pal, signals);
  drawTorso(ctx, joints, pal);
  drawNearLeg(ctx, joints, pal);
  drawNearArm(ctx, joints, pal, signals, phase);
  drawHead(ctx, joints, pal, side);
  ctx.restore();
}

function drawShadow(ctx, pose, pal, signals) {
  const spread = signals.down ? 20 : 13;
  ctx.save();
  ctx.beginPath();
  ctx.ellipse(pose.hipX * 0.4, 1.5, spread, 3.4, 0, 0, Math.PI * 2);
  ctx.fillStyle = PAL2D.shadow;
  ctx.fill();
  ctx.restore();
}

function drawFarLimbs(ctx, joints, pal) {
  const arm = chain(joints, ['armFarUp', 'armFarFore', 'armFarHand']);
  taperedPath(ctx, arm, [8.2, 6.6, 9.5, 4.6]);
  fillInk(ctx, pal.robeShade, pal.ink, 1.1);

  const leg = chain(joints, ['legFarThigh', 'legFarShin', 'legFarFoot']);
  taperedPath(ctx, leg, [10.5, 8.2, 6.4, 4.6]);
  fillInk(ctx, pal.robeShade, pal.ink, 1.1);
}

/**
 * The robe is the silhouette. It runs from the chest down through the verlet
 * hem, widening as it falls, and is drawn as one closed shape so the hem edge
 * is a single continuous line rather than a stack of segments.
 */
function drawRobe(ctx, joints, robe, pal, signals) {
  const chest = joints[BONE_INDEX.chest];
  const hips = joints[BONE_INDEX.hips];
  const pts = robe.pts;

  const widths = [12, 15.5, 18.5, 21, 23.5];
  const left = [];
  const right = [];
  const spine = [{ x: chest.tipX, y: chest.tipY }, { x: hips.baseX, y: hips.baseY }, ...pts.slice(1)];

  for (let i = 0; i < spine.length; i += 1) {
    const prev = spine[Math.max(0, i - 1)];
    const next = spine[Math.min(spine.length - 1, i + 1)];
    let dx = next.x - prev.x;
    let dy = next.y - prev.y;
    const len = Math.hypot(dx, dy) || 1;
    dx /= len;
    dy /= len;
    const w = (widths[Math.min(i, widths.length - 1)] ?? 23) / 2;
    left.push({ x: spine[i].x - dy * w, y: spine[i].y + dx * w });
    right.push({ x: spine[i].x + dy * w, y: spine[i].y - dx * w });
  }

  ctx.beginPath();
  ctx.moveTo(left[0].x, left[0].y);
  for (let i = 1; i < left.length; i += 1) ctx.lineTo(left[i].x, left[i].y);
  for (let i = right.length - 1; i >= 0; i -= 1) ctx.lineTo(right[i].x, right[i].y);
  ctx.closePath();
  fillInk(ctx, pal.robe, pal.ink, 1.4);

  // Front fold: one lifted line down the robe so the shape has an inside.
  ctx.beginPath();
  ctx.moveTo(spine[0].x, spine[0].y);
  for (let i = 1; i < spine.length; i += 1) {
    const t = i / (spine.length - 1);
    ctx.lineTo(spine[i].x + t * 3.5, spine[i].y);
  }
  ctx.strokeStyle = pal.robeLit;
  ctx.lineWidth = 1.6;
  ctx.stroke();
}

function drawTorso(ctx, joints, pal) {
  const torso = chain(joints, ['spine', 'chest']);
  taperedPath(ctx, torso, [15, 15.5, 13.5]);
  fillInk(ctx, pal.robe, pal.ink, 1.4);

  // Sash at the waist — the one bright band on the figure, and the reason the
  // torso does not read as a single slab.
  const hips = joints[BONE_INDEX.hips];
  const spine = joints[BONE_INDEX.spine];
  ctx.save();
  ctx.translate(hips.baseX, hips.baseY);
  ctx.rotate(-Math.atan2(spine.tipX - spine.baseX, spine.tipY - spine.baseY));
  ctx.beginPath();
  ctx.rect(-8, -1, 16, 5.2);
  fillInk(ctx, pal.sash, pal.ink, 1);
  ctx.restore();
}

function drawNearLeg(ctx, joints, pal) {
  const leg = chain(joints, ['legNearThigh', 'legNearShin', 'legNearFoot']);
  taperedPath(ctx, leg, [11.5, 9, 7, 5]);
  fillInk(ctx, pal.robe, pal.ink, 1.3);

  const foot = joints[BONE_INDEX.legNearFoot];
  taperedPath(
    ctx,
    [
      { x: foot.baseX, y: foot.baseY },
      { x: foot.tipX, y: foot.tipY },
    ],
    [7.5, 5],
  );
  fillInk(ctx, pal.ink, pal.ink, 0);
}

/**
 * The blade arm, its sleeve, and the sword. The sleeve is a second tapered
 * shape over the forearm that widens toward the cuff — the wide sleeve is the
 * genre's most recognisable silhouette cue and costs one extra polygon.
 */
function drawNearArm(ctx, joints, pal, signals, phase) {
  const hand = joints[BONE_INDEX.armNearHand];

  if (signals.attack && !signals.down) drawSwordTrail(ctx, joints, phase);

  const arm = chain(joints, ['armNearUp', 'armNearFore', 'armNearHand']);
  taperedPath(ctx, arm, [9.5, 7.5, 6, 4.4]);
  fillInk(ctx, pal.robe, pal.ink, 1.3);

  const upper = joints[BONE_INDEX.armNearUp];
  const fore = joints[BONE_INDEX.armNearFore];
  taperedPath(
    ctx,
    [
      { x: upper.baseX, y: upper.baseY },
      { x: fore.baseX, y: fore.baseY },
      { x: fore.baseX + (fore.tipX - fore.baseX) * 0.55, y: fore.baseY + (fore.tipY - fore.baseY) * 0.55 },
    ],
    [11.5, 13.5, 16],
  );
  fillInk(ctx, pal.robeLit, pal.ink, 1.3);

  ctx.beginPath();
  ctx.arc(hand.baseX, hand.baseY, 2.9, 0, Math.PI * 2);
  fillInk(ctx, pal.skin, pal.ink, 1);

  drawSword(ctx, joints, pal, signals);
}

function drawSword(ctx, joints, pal, signals) {
  const hand = joints[BONE_INDEX.armNearHand];
  const a = (hand.angle * Math.PI) / 180;
  const dx = Math.sin(a);
  const dy = Math.cos(a);
  const nx = -dy;
  const ny = dx;
  const gx = hand.baseX;
  const gy = hand.baseY;
  const bladeLen = 46;
  const tipX = gx + dx * bladeLen;
  const tipY = gy + dy * bladeLen;

  // Guard.
  ctx.beginPath();
  ctx.moveTo(gx + nx * 5 + dx * 4, gy + ny * 5 + dy * 4);
  ctx.lineTo(gx - nx * 5 + dx * 4, gy - ny * 5 + dy * 4);
  ctx.lineWidth = 2.2;
  ctx.strokeStyle = pal.ink;
  ctx.stroke();

  // Blade: a long triangle rather than a line, so it has a spine and an edge.
  ctx.beginPath();
  ctx.moveTo(gx + nx * 2.2 + dx * 4, gy + ny * 2.2 + dy * 4);
  ctx.lineTo(tipX, tipY);
  ctx.lineTo(gx - nx * 2.2 + dx * 4, gy - ny * 2.2 + dy * 4);
  ctx.closePath();
  fillInk(ctx, pal.blade, pal.ink, 1.1);

  // Grip.
  ctx.beginPath();
  ctx.moveTo(gx - dx * 7, gy - dy * 7);
  ctx.lineTo(gx + dx * 4, gy + dy * 4);
  ctx.lineWidth = 4.2;
  ctx.lineCap = 'round';
  ctx.strokeStyle = pal.ink;
  ctx.stroke();
}

/**
 * 검로 — the arc the blade has just travelled through.
 *
 * Kept deliberately short. The first capture swept 2.1 radians at
 * shoulder-plus-blade radius, which drew concentric rings clean across the
 * frame and read as background decoration rather than as one cut. A cut is
 * legible at well under a quarter turn, and it has to sit inside the figure's
 * own bounding box to belong to the figure.
 */
function drawSwordTrail(ctx, joints, phase) {
  const shoulder = joints[BONE_INDEX.armNearUp];
  const hand = joints[BONE_INDEX.armNearHand];
  const cx = shoulder.baseX;
  const cy = shoulder.baseY;
  // Radius follows the *hand*, not the blade tip. Adding the blade length put
  // the arc outside the figure entirely, where it stopped reading as this
  // character's cut and started reading as background decoration.
  const r = Math.hypot(hand.baseX - cx, hand.baseY - cy) + 6;
  const end = Math.atan2(hand.baseX - cx, hand.baseY - cy);
  const sweep = 0.55 * Math.min(1, phase / 0.62);

  ctx.save();
  for (let i = 0; i < 2; i += 1) {
    ctx.beginPath();
    ctx.arc(cx, cy, r - i * 7, -(end - sweep) + Math.PI / 2, -end + Math.PI / 2, false);
    ctx.strokeStyle = PAL2D.trail;
    ctx.globalAlpha = 0.34 - i * 0.14;
    ctx.lineWidth = 4 - i * 1.6;
    ctx.lineCap = 'round';
    ctx.stroke();
  }
  ctx.restore();
}

/**
 * Head and headwear. No face: the art policy asks for it, and a blank head
 * keeps attention on the pose. Side is carried by silhouette — the orthodox
 * school wears a bound topknot, the Black Serpent a low hood.
 */
function drawHead(ctx, joints, pal, side) {
  const head = joints[BONE_INDEX.head];
  const a = (head.angle * Math.PI) / 180;
  const cx = head.baseX + Math.sin(a) * 4.5;
  const cy = head.baseY + Math.cos(a) * 4.5;

  ctx.save();
  ctx.translate(cx, cy);
  ctx.rotate(-a);

  ctx.beginPath();
  ctx.ellipse(0, 0, 5.2, 6.1, 0, 0, Math.PI * 2);
  fillInk(ctx, pal.skin, pal.ink, 1.2);

  if (side === 'ally') {
    // Bound topknot plus a trailing ribbon.
    ctx.beginPath();
    ctx.ellipse(-1.2, 4.4, 5.6, 3.4, 0, Math.PI, Math.PI * 2);
    fillInk(ctx, pal.ink, pal.ink, 0);
    ctx.beginPath();
    ctx.ellipse(-0.6, 8.2, 2.6, 3.1, 0, 0, Math.PI * 2);
    fillInk(ctx, pal.ink, pal.ink, 0);
    ctx.beginPath();
    ctx.moveTo(-3.4, 6.5);
    ctx.quadraticCurveTo(-12, 5.4, -15.5, -1.5);
    ctx.strokeStyle = pal.trim;
    ctx.lineWidth = 1.8;
    ctx.lineCap = 'round';
    ctx.stroke();
  } else {
    // Low hood: one shape that swallows the crown and the nape.
    ctx.beginPath();
    ctx.moveTo(6.2, 1.2);
    ctx.quadraticCurveTo(5.4, 8.4, -1.4, 8.2);
    ctx.quadraticCurveTo(-8.4, 7.6, -7.4, -2.4);
    ctx.quadraticCurveTo(-4, 2.2, 1.2, 1.6);
    ctx.closePath();
    fillInk(ctx, pal.ink, pal.ink, 0);
  }
  ctx.restore();
}

/** A small impact burst where a hit landed. Seeded scatter lives in the caller. */
export function drawImpact2D(ctx, cx, groundY, scale, signals, rand) {
  if (!signals.hit) return;
  ctx.save();
  ctx.translate(cx, groundY);
  ctx.scale(scale * signals.facing, -scale);
  // Five short marks in a forward-biased fan, not a starburst. The first
  // capture scattered nine long spokes over the whole torso, which read as
  // debris pasted on top of the character rather than as a strike landing.
  const ox = 7;
  const oy = HIP_HEIGHT + 14;
  for (let i = 0; i < 5; i += 1) {
    const ang = -0.9 + rand() * 1.8;
    const d0 = 7 + rand() * 3;
    const d1 = d0 + 4 + rand() * 5;
    ctx.beginPath();
    ctx.moveTo(ox + Math.cos(ang) * d0, oy + Math.sin(ang) * d0);
    ctx.lineTo(ox + Math.cos(ang) * d1, oy + Math.sin(ang) * d1);
    ctx.strokeStyle = PAL2D.spark;
    ctx.globalAlpha = 0.4 + rand() * 0.3;
    ctx.lineWidth = 1.4;
    ctx.lineCap = 'round';
    ctx.stroke();
  }
  ctx.restore();
}

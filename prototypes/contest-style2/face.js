/**
 * Procedural faces — a Canvas2D painter, renderer-agnostic like `rig.js`.
 *
 * This module knows nothing about Three.js. It paints an expression into a 2D
 * context laid out in **equirectangular sphere UV**, which is what lets the
 * features sit on the head as a decal rather than on a flat card floating in
 * front of it — no seam, no parallax when the head turns.
 *
 * Why painted rather than modelled: at this poly budget geometry eyes read as
 * beads. Every feature that carries emotion here — the weight of the upper
 * lash, the value drop across the iris, a 2px specular — is a *drawing*
 * problem, and the honest place to solve a drawing problem is a canvas.
 * (Ink Tide does the same thing for every texture it ships.)
 *
 * Construction rules, in priority order:
 *   1. The upper lash line is much heavier than the lower. It is the single
 *      strongest read at distance.
 *   2. The iris runs dark at the top to light at the bottom — the lid casts
 *      onto it. A flat iris reads as a printed dot.
 *   3. Sclera stays visible on both sides of the iris, and is never white; a
 *      warm off-grey keeps the eye inside the skin's value range.
 *   4. Brows are thin and low contrast. They steer the emotion; they do not
 *      announce it.
 *   5. The nose is a shadow, not an outline. The mouth is small.
 *   6. Hair is a few grouped masses with one highlight band, overlapping the
 *      brow asymmetrically. Never strands.
 */

/** One palette table. No colour literal appears below this object. */
export const FACE_PAL = {
  ally: {
    sclera: '#e7ded2',
    scleraShade: '#c2b4a6',
    lash: '#1e1519',
    lashSoft: '#3a2a2b',
    irisTop: '#16323d',
    irisBot: '#8fd0c8',
    irisRing: '#12222a',
    irisRim: '#bff0e4',
    pupil: '#0d161c',
    spec: '#fffdf6',
    brow: '#4b3728',
    browSoft: '#6b5240',
    lipLine: '#8d5347',
    lipLow: '#c98874',
    lipHi: '#ffe6d8',
    mouthDark: '#3f2220',
    noseShade: '#b47f5c',
    blush: '#cf8468',
    socket: '#a9714f',
    hair: '#191520',
    hairLit: '#3b3448',
    jaw: '#a86a4c',
    overhang: '#8c5334',
  },
  enemy: {
    sclera: '#ddd2c4',
    scleraShade: '#a9998a',
    lash: '#150f12',
    lashSoft: '#2e2022',
    irisTop: '#331f08',
    irisBot: '#e0ad55',
    irisRing: '#1d1207',
    irisRim: '#ffdda0',
    pupil: '#0a0806',
    spec: '#fff8e6',
    brow: '#31241c',
    browSoft: '#4a382c',
    lipLine: '#7a463d',
    lipLow: '#a97260',
    lipHi: '#ffdccb',
    mouthDark: '#2e1917',
    noseShade: '#96674a',
    blush: '#a86a52',
    socket: '#8a5539',
    hair: '#100d13',
    hairLit: '#2b2533',
    jaw: '#8a5b41',
    overhang: '#6b4029',
  },
};

/**
 * Sphere-UV projector.
 *
 * `s` is the lateral offset across the face and `h` the height, both in the
 * same units as the skull radius. The result is a pixel in an equirectangular
 * texture whose +X meridian (the direction the character faces) sits at u=0.5.
 *
 * Drawing a face in flat pixels and hoping is how features end up sliding off
 * the cheekbone; the mapping is cheap and exact, so it is done properly.
 */
export function faceProjector(width, height, radius, aspect) {
  // The skull mesh is an egg, not a ball — a head that is as wide as it is tall
  // reads as an infant no matter what is painted on it. Scaling the mesh does
  // not move its UVs, so the drawing has to divide the scale back out or every
  // feature drifts as soon as the proportions are retuned.
  const ay = (aspect && aspect.y) || 1;
  const az = (aspect && aspect.z) || 1;
  const proj = (sv, hv) => {
    const s = sv / az;
    const h = hv / ay;
    const hh = Math.max(-radius * 0.999, Math.min(radius * 0.999, h));
    const theta = Math.acos(hh / radius);
    const sinT = Math.max(1e-3, Math.sin(theta));
    const t = Math.max(-0.999, Math.min(0.999, s / (radius * sinT)));
    return { x: width * (0.5 - Math.asin(t) / (2 * Math.PI)), y: (height * theta) / Math.PI };
  };
  // With width = 2 × height the texture is isotropic, so one scalar converts a
  // head-unit length to pixels in both axes.
  proj.px = height / (Math.PI * radius);
  return proj;
}

// ---------------------------------------------------------------------------
// Small drawing helpers
// ---------------------------------------------------------------------------

function tracePoly(ctx, pts, proj) {
  pts.forEach((p, i) => {
    const q = proj(p[0], p[1]);
    if (i === 0) ctx.moveTo(q.x, q.y);
    else ctx.lineTo(q.x, q.y);
  });
}

function sample(fn, n) {
  const out = [];
  for (let i = 0; i <= n; i += 1) out.push(fn(-1 + (2 * i) / n));
  return out;
}

/** Almond profile. The exponent keeps the corners sharp and the middle full. */
function almond(u) {
  return Math.pow(Math.max(0, 1 - u * u), 0.58);
}

function withAlpha(hex, a) {
  const n = parseInt(hex.slice(1), 16);
  return `rgba(${(n >> 16) & 255}, ${(n >> 8) & 255}, ${n & 255}, ${a})`;
}

// ---------------------------------------------------------------------------
// Feature geometry, in head units
// ---------------------------------------------------------------------------

// All heights are measured from the centre of the skull sphere, in skull radii
// scaled by the mesh aspect. The eye line sits just below centre — on a head,
// not on a face card — and the chin follows at roughly −3.4.
const EYE = {
  lateral: 1.84,
  height: -0.34,
  halfWidth: 1.16,
  lidTop: 0.74,
  lidBottom: 0.6,
  iris: 0.66,
  /** Outer corner lift. This is most of what separates sharp from doll-like. */
  tilt: 0.2,
};

// Brow close to the eye — the gap is where an anxious, childish read comes
// from — and the mouth well down toward the chin.
const BROW = { height: 0.72, halfWidth: 1.36, lateral: 1.92 };
const NOSE = { height: -1.9, spread: 0.34 };
const MOUTH = { height: -3.05, halfWidth: 0.64 };

// ---------------------------------------------------------------------------

/**
 * Paint one expression.
 *
 * `expression` is exactly the object `rig.js`'s `expressionFor` returns — this
 * function invents nothing about the emotion, it only draws it.
 */
export function drawFace(ctx, opts) {
  const { width, height, radius, side, expression: e, gazeLateral = 0, aspect } = opts;
  const pal = FACE_PAL[side];
  const proj = faceProjector(width, height, radius, aspect);
  const px = proj.px;

  ctx.clearRect(0, 0, width, height);
  ctx.lineCap = 'round';
  ctx.lineJoin = 'round';

  // Order matters. Painted form first (the head's material is nearly unlit, so
  // this *is* the modelling), then features on top of it, then hair last so it
  // overhangs the brow.
  drawFormShading(ctx, proj, px, pal, e);
  drawSocketShadow(ctx, proj, px, pal, e);
  drawCheek(ctx, proj, px, pal, e);
  for (const dir of [-1, 1]) drawEye(ctx, proj, px, pal, e, dir, gazeLateral);
  for (const dir of [-1, 1]) drawBrow(ctx, proj, px, pal, e, dir);
  drawNose(ctx, proj, px, pal, e);
  drawMouth(ctx, proj, px, pal, e);
  drawHair(ctx, proj, px, pal, side);
}


/**
 * The painted form.
 *
 * The head's material contributes almost no NdotL on purpose, so every piece
 * of modelling on this face is here: a warm wash under the jaw, and the
 * overhang the hair casts across the brow. Two shapes, both soft, both warm —
 * which is the note that separates skin from a grey sphere.
 */
function drawFormShading(ctx, proj, px, pal, e) {
  // Under the jaw and chin.
  const chin = proj(0, -4.55);
  const jg = ctx.createRadialGradient(chin.x, chin.y + px * 0.4, px * 0.2, chin.x, chin.y + px * 0.4, px * 2.6);
  jg.addColorStop(0, withAlpha(pal.jaw, 0.62));
  jg.addColorStop(0.5, withAlpha(pal.jaw, 0.3));
  jg.addColorStop(1, withAlpha(pal.jaw, 0));
  ctx.fillStyle = jg;
  ctx.beginPath();
  ctx.arc(chin.x, chin.y + px * 0.4, px * 2.6, 0, Math.PI * 2);
  ctx.fill();

  // Overhang from the hair across the top of the brow.
  const brow = proj(0, 2.4);
  const og = ctx.createLinearGradient(brow.x, proj(0, 3.9).y, brow.x, proj(0, 0.9).y);
  og.addColorStop(0, withAlpha(pal.overhang, 0.62));
  og.addColorStop(1, withAlpha(pal.overhang, 0));
  ctx.fillStyle = og;
  ctx.beginPath();
  tracePoly(
    ctx,
    sample((u) => [u * 3.5, 4.2], 8),
    proj,
  );
  const lower = sample((u) => [-u * 3.5, 0.85 + 0.55 * (1 - u * u)], 12);
  tracePoly(ctx, lower, proj);
  ctx.closePath();
  ctx.fill();

  // Temple sides, so the head does not read as a flat disc from the front.
  // Radial, not a polygon: the polygon version drew a hard vertical seam down
  // the cheek that survived two capture rounds before anyone saw it.
  for (const dir of [-1, 1]) {
    const c = proj(3.9 * dir, -0.4);
    const tg = ctx.createRadialGradient(c.x, c.y, 0, c.x, c.y, px * 2.4);
    tg.addColorStop(0, withAlpha(pal.jaw, 0.4));
    tg.addColorStop(1, withAlpha(pal.jaw, 0));
    ctx.fillStyle = tg;
    ctx.beginPath();
    ctx.ellipse(c.x, c.y, px * 1.5, px * 2.4, 0, 0, Math.PI * 2);
    ctx.fill();
  }
}

/**
 * A warm shadow in the socket, laid down before the eye.
 *
 * This is the one place the "soft two-tone, shadow side warmer" note is
 * carried by the texture rather than the shader: an eye set straight onto flat
 * skin reads pasted on, and the 3D ramp is too coarse to seat it.
 */
function drawSocketShadow(ctx, proj, px, pal, e) {
  for (const dir of [-1, 1]) {
    const cx = EYE.lateral * dir;
    const g = ctx.createRadialGradient(
      proj(cx, EYE.height + 0.25).x,
      proj(cx, EYE.height + 0.25).y,
      px * 0.2,
      proj(cx, EYE.height + 0.25).x,
      proj(cx, EYE.height + 0.25).y,
      px * 1.6,
    );
    g.addColorStop(0, withAlpha(pal.socket, 0.26 + 0.12 * e.tension));
    g.addColorStop(1, withAlpha(pal.socket, 0));
    ctx.fillStyle = g;
    ctx.beginPath();
    ctx.arc(proj(cx, EYE.height + 0.25).x, proj(cx, EYE.height + 0.25).y, px * 1.6, 0, Math.PI * 2);
    ctx.fill();
  }
}

function lidCurves(e, dir) {
  const open = Math.max(0, 1 - e.lidUpper * 1.14 + e.eyeWide * 0.26);
  const topH = EYE.lidTop * (1 + 0.24 * e.eyeWide) * open;
  const botH = EYE.lidBottom * (1 - 0.62 * Math.max(0, e.lidLower));
  // The lid peak sits slightly inboard; a symmetric peak reads as a cartoon.
  const skew = -0.18 * dir;
  const cant = (u) => EYE.tilt * u; // outer corner rides up, inner drops
  const top = (u) => [
    (EYE.lateral + u * EYE.halfWidth) * dir,
    EYE.height + cant(u) + topH * almond(Math.max(-1, Math.min(1, u - skew))),
  ];
  const bottom = (u) => [
    (EYE.lateral + u * EYE.halfWidth) * dir,
    EYE.height + cant(u) - botH * almond(Math.max(-1, Math.min(1, u + skew * 0.5))),
  ];
  return { top, bottom, open };
}

function drawEye(ctx, proj, px, pal, e, dir, gazeLateral) {
  const { top, bottom, open } = lidCurves(e, dir);

  if (open < 0.09) {
    // Shut. One weighted arc — the shape a closed eye actually makes.
    ctx.strokeStyle = pal.lash;
    ctx.lineWidth = px * 0.15 * e.browWeight;
    ctx.beginPath();
    tracePoly(
      ctx,
      sample((u) => [(EYE.lateral + u * EYE.halfWidth) * dir, EYE.height - 0.06 + 0.13 * almond(u)], 14),
      proj,
    );
    ctx.stroke();
    return;
  }

  const lidPath = () => {
    ctx.beginPath();
    tracePoly(ctx, sample(top, 16), proj);
    const back = sample(bottom, 16).reverse();
    tracePoly(ctx, back, proj);
    ctx.closePath();
  };

  // Sclera — warm off-grey, darker toward the outer corner where the lid
  // shadow falls.
  lidPath();
  const c0 = proj((EYE.lateral - EYE.halfWidth) * dir, EYE.height);
  const c1 = proj((EYE.lateral + EYE.halfWidth) * dir, EYE.height);
  const sg = ctx.createLinearGradient(c0.x, c0.y, c1.x, c1.y);
  sg.addColorStop(0, pal.sclera);
  sg.addColorStop(1, pal.scleraShade);
  ctx.fillStyle = sg;
  ctx.fill();

  // Iris, clipped to the lid opening so the upper lid genuinely cuts it.
  ctx.save();
  lidPath();
  ctx.clip();

  const ix = EYE.lateral * dir + gazeLateral * 0.42;
  const iy = EYE.height + e.gazeY * 0.2;
  const ic = proj(ix, iy);
  const ir = EYE.iris * px;

  const ig = ctx.createLinearGradient(ic.x, ic.y - ir, ic.x, ic.y + ir);
  ig.addColorStop(0, pal.irisTop);
  ig.addColorStop(0.55, pal.irisTop);
  ig.addColorStop(1, pal.irisBot);
  ctx.fillStyle = ig;
  ctx.beginPath();
  ctx.arc(ic.x, ic.y, ir, 0, Math.PI * 2);
  ctx.fill();

  ctx.strokeStyle = pal.irisRing;
  ctx.lineWidth = ir * 0.22;
  ctx.beginPath();
  ctx.arc(ic.x, ic.y, ir * 0.92, 0, Math.PI * 2);
  ctx.stroke();

  ctx.fillStyle = pal.pupil;
  ctx.beginPath();
  ctx.arc(ic.x, ic.y + ir * 0.05, ir * 0.42, 0, Math.PI * 2);
  ctx.fill();

  // Contact shadow from the upper lid across the top of the iris.
  ctx.fillStyle = withAlpha(pal.irisRing, 0.45);
  ctx.beginPath();
  ctx.ellipse(ic.x, ic.y - ir * 1.0, ir * 1.25, ir * 0.62, 0, 0, Math.PI * 2);
  ctx.fill();

  ctx.strokeStyle = withAlpha(pal.irisRim, 0.85);
  ctx.lineWidth = ir * 0.16;
  ctx.beginPath();
  ctx.arc(ic.x, ic.y, ir * 0.86, Math.PI * 0.18, Math.PI * 0.82);
  ctx.stroke();

  // Two hard speculars: a large one high on the inner side of the iris, and a
  // much smaller one low and opposite. The pair is what stops an iris reading
  // as a printed dot — one alone reads as a sticker.
  const inner = -dir;
  ctx.fillStyle = pal.spec;
  ctx.beginPath();
  ctx.arc(ic.x + inner * ir * 0.34, ic.y - ir * 0.38, ir * 0.26, 0, Math.PI * 2);
  ctx.fill();
  ctx.beginPath();
  ctx.arc(ic.x - inner * ir * 0.36, ic.y + ir * 0.42, ir * 0.11, 0, Math.PI * 2);
  ctx.fill();
  ctx.restore();

  // Lid crease, well above the lash and barely there.
  ctx.strokeStyle = withAlpha(pal.lashSoft, 0.3);
  ctx.lineWidth = px * 0.042;
  ctx.beginPath();
  tracePoly(
    ctx,
    sample((u) => {
      const [x, y] = top(u * 0.82);
      return [x, y + 0.2];
    }, 12),
    proj,
  );
  ctx.stroke();

  // Lower lid: thin, soft, and deliberately short at both ends so the eye
  // shape stays open. A lower line that meets the upper one at both corners
  // turns the eye into a closed bead.
  ctx.strokeStyle = withAlpha(pal.lashSoft, 0.55);
  ctx.lineWidth = px * 0.05;
  ctx.beginPath();
  tracePoly(
    ctx,
    sample(bottom, 16).filter((_, i, a) => i > a.length * 0.22 && i < a.length * 0.88),
    proj,
  );
  ctx.stroke();

  // Upper lash: the heavy one. Drawn as a filled wedge rather than a stroke so
  // it can thicken toward the outer corner, which is what gives an eye its
  // direction.
  const lashW = (u) => px * (0.06 + 0.085 * Math.pow((u + 1) / 2, 1.3)) * e.browWeight;
  ctx.fillStyle = pal.lash;
  ctx.beginPath();
  tracePoly(ctx, sample(top, 18), proj);
  const upper = sample(top, 18).reverse();
  upper.forEach((p, i) => {
    const u = 1 - (2 * i) / (upper.length - 1);
    const q = proj(p[0], p[1]);
    ctx.lineTo(q.x, q.y - lashW(u));
  });
  ctx.closePath();
  ctx.fill();

  // Three lash spikes flicking up past the outer corner, each shorter and
  // steeper than the last. This is the detail that gives an eye a direction.
  const spikes = [
    { at: 0.92, len: 0.3, lift: 0.22, w: 0.06 },
    { at: 1.02, len: 0.22, lift: 0.26, w: 0.045 },
  ];
  ctx.strokeStyle = pal.lash;
  for (const sp of spikes) {
    const root = top(sp.at * 1.0);
    const a = proj(root[0], root[1]);
    const b = proj(root[0] + sp.len * dir, root[1] + sp.lift);
    ctx.lineWidth = px * sp.w * e.browWeight;
    ctx.beginPath();
    ctx.moveTo(a.x, a.y);
    ctx.lineTo(b.x, b.y);
    ctx.stroke();
  }
}

/** Thin, low contrast, and doing most of the emotional work. */
function drawBrow(ctx, proj, px, pal, e, dir) {
  const inner = e.browInner;
  const outer = e.browOuter;
  const pts = sample((u) => {
    const t = (u + 1) / 2; // 0 inner → 1 outer
    const lift = inner * (1 - t) + outer * t;
    const arch = 0.2 * almond(u - 0.15 * dir);
    return [(BROW.lateral + u * BROW.halfWidth) * dir, BROW.height + arch + lift * 0.5];
  }, 16);

  // A tapered fill, not a stroke of constant width. A brow of even weight
  // reads as a painted-on bar; the taper is most of what makes it hair.
  const weight = (u) => {
    const t = (u + 1) / 2;
    return px * (0.055 + 0.13 * Math.pow(1 - t, 1.15)) * e.browWeight;
  };
  ctx.fillStyle = withAlpha(pal.brow, 0.94);
  ctx.beginPath();
  pts.forEach((p, i) => {
    const u = -1 + (2 * i) / (pts.length - 1);
    const q = proj(p[0], p[1]);
    const y = q.y - weight(u) * 0.5;
    if (i === 0) ctx.moveTo(q.x, y);
    else ctx.lineTo(q.x, y);
  });
  [...pts].reverse().forEach((p, i) => {
    const u = 1 - (2 * i) / (pts.length - 1);
    const q = proj(p[0], p[1]);
    ctx.lineTo(q.x, q.y + weight(u) * 0.5);
  });
  ctx.closePath();
  ctx.fill();

  ctx.strokeStyle = withAlpha(pal.browSoft, 0.45);
  ctx.lineWidth = px * 0.05;
  ctx.beginPath();
  tracePoly(ctx, pts.slice(0, Math.floor(pts.length * 0.5)), proj);
  ctx.stroke();

  // Tension crease between the brows — only when they are genuinely pulled.
  if (inner < -0.3) {
    ctx.strokeStyle = withAlpha(pal.noseShade, Math.min(0.5, -inner * 0.45));
    ctx.lineWidth = px * 0.055;
    ctx.beginPath();
    tracePoly(
      ctx,
      sample((u) => [(0.42 + u * 0.06) * dir, BROW.height - 0.28 + u * 0.24], 6),
      proj,
    );
    ctx.stroke();
  }
}

/** A shadow, not an outline. */
function drawNose(ctx, proj, px, pal, e) {
  const a = 0.42 + 0.2 * e.tension;
  ctx.fillStyle = withAlpha(pal.noseShade, a);
  for (const dir of [-1, 1]) {
    const p = proj(NOSE.spread * dir, NOSE.height);
    ctx.beginPath();
    ctx.ellipse(p.x, p.y, px * 0.14, px * 0.085, 0, 0, Math.PI * 2);
    ctx.fill();
  }
  const top = proj(-0.22, NOSE.height + 1.5);
  const bot = proj(-0.3, NOSE.height + 0.1);
  const g = ctx.createLinearGradient(top.x, top.y, bot.x, bot.y);
  g.addColorStop(0, withAlpha(pal.noseShade, 0));
  g.addColorStop(1, withAlpha(pal.noseShade, a * 0.8));
  ctx.strokeStyle = g;
  ctx.lineWidth = px * 0.17;
  ctx.beginPath();
  ctx.moveTo(top.x, top.y);
  ctx.lineTo(bot.x, bot.y);
  ctx.stroke();
}

function drawMouth(ctx, proj, px, pal, e) {
  const hw = MOUTH.halfWidth * e.mouthWidth;
  const open = Math.max(0, e.mouthOpen);
  const curve = e.mouthCurve;

  // `mouthCurve` is signed: negative lifts the middle relative to the corners,
  // which is a grimace. The first pass added a separate corner-lift term on top
  // and every expression came out smirking — a fighter mid-exchange smiling at
  // its opponent, which is exactly the sort of thing only a capture catches.
  const line = (u) => [u * hw, MOUTH.height - curve * 0.2 * (1 - u * u)];

  if (open > 0.08) {
    const drop = 0.1 + open * 0.42;
    ctx.fillStyle = pal.mouthDark;
    ctx.beginPath();
    tracePoly(ctx, sample(line, 12), proj);
    const back = sample((u) => {
      const [x, y] = line(-u);
      return [x, y - drop * almond(u)];
    }, 12);
    tracePoly(ctx, back, proj);
    ctx.closePath();
    ctx.fill();

    ctx.strokeStyle = withAlpha(pal.lipLow, 0.8);
    ctx.lineWidth = px * 0.07;
    ctx.beginPath();
    tracePoly(
      ctx,
      sample((u) => {
        const [x, y] = line(u);
        return [x, y - drop * almond(u) - 0.04];
      }, 12),
      proj,
    );
    ctx.stroke();
  }

  ctx.strokeStyle = withAlpha(pal.lipLine, 0.9);
  ctx.lineWidth = px * (0.07 + 0.03 * e.tension);
  ctx.beginPath();
  tracePoly(ctx, sample(line, 14), proj);
  ctx.stroke();

  // Lower-lip highlight: one short soft mark, slightly off centre.
  const lipHi = proj(-0.16 * hw, MOUTH.height - 0.34);
  const lg = ctx.createRadialGradient(lipHi.x, lipHi.y, 0, lipHi.x, lipHi.y, px * 0.34);
  lg.addColorStop(0, withAlpha(pal.lipHi, 0.6));
  lg.addColorStop(1, withAlpha(pal.lipHi, 0));
  ctx.fillStyle = lg;
  ctx.beginPath();
  ctx.ellipse(lipHi.x, lipHi.y, px * 0.34, px * 0.15, 0, 0, Math.PI * 2);
  ctx.fill();

  if (open <= 0.08) {
    ctx.strokeStyle = withAlpha(pal.lipLow, 0.55);
    ctx.lineWidth = px * 0.07;
    ctx.beginPath();
    tracePoly(
      ctx,
      sample((u) => {
        const [x, y] = line(u * 0.8);
        return [x, y - 0.14];
      }, 10),
      proj,
    );
    ctx.stroke();
  }
}

/**
 * Blush, and it is doing more work than its size suggests.
 *
 * One soft warm ellipse tucked directly under each lower lid, continued across
 * the nose bridge. Without it a face with correct features still reads as a
 * mask; with it the same face reads as skin with blood behind it.
 */
function drawCheek(ctx, proj, px, pal, e) {
  const a = 0.2 + 0.18 * e.tension;
  for (const dir of [-1, 1]) {
    const c = proj(EYE.lateral * dir * 0.94, EYE.height - 0.86);
    const g = ctx.createRadialGradient(c.x, c.y, 0, c.x, c.y, px * 1.0);
    g.addColorStop(0, withAlpha(pal.blush, a));
    g.addColorStop(0.6, withAlpha(pal.blush, a * 0.45));
    g.addColorStop(1, withAlpha(pal.blush, 0));
    ctx.save();
    ctx.fillStyle = g;
    ctx.beginPath();
    ctx.ellipse(c.x, c.y, px * 1.25, px * 0.6, 0, 0, Math.PI * 2);
    ctx.fill();
    ctx.restore();
  }
  const bridge = proj(0, EYE.height - 0.7);
  const bg = ctx.createRadialGradient(bridge.x, bridge.y, 0, bridge.x, bridge.y, px * 0.95);
  bg.addColorStop(0, withAlpha(pal.blush, a * 0.72));
  bg.addColorStop(1, withAlpha(pal.blush, 0));
  ctx.fillStyle = bg;
  ctx.beginPath();
  ctx.ellipse(bridge.x, bridge.y, px * 0.95, px * 0.4, 0, 0, Math.PI * 2);
  ctx.fill();
}

/**
 * Hair as grouped masses.
 *
 * Three shapes, deliberately asymmetric, each overlapping the brow line, with
 * one highlight band across the largest. Strands at this scale turn to noise;
 * masses keep reading down to a 40px portrait.
 */
function drawHair(ctx, proj, px, pal, side) {
  // Both fighters wear hair over the brow, so both get a painted fringe: it is
  // what joins the geometry shell to the forehead. 산발 is parted off-centre
  // and uneven; the bound head is a clean centre part.
  const locks =
    side === 'ally'
      ? [
          { s0: -4.4, s1: -0.15, top: 2.5, dip: 0.62, bow: 0.5, soft: 3.0 },
          { s0: -1.5, s1: 2.6, top: 2.6, dip: 1.18, bow: -0.34, soft: 3.6 },
          { s0: 1.5, s1: 4.5, top: 2.45, dip: 0.5, bow: 0.44, soft: 2.8 },
        ]
      : [
          { s0: -4.3, s1: -0.25, top: 2.55, dip: 1.05, bow: 0.34, soft: 3.8 },
          { s0: 0.25, s1: 4.3, top: 2.55, dip: 1.05, bow: 0.34, soft: 3.8 },
        ];

  // A solid cap above the fringe first. The geometry shell's rim and the
  // painted locks are two different curves, and the capture showed bare skin
  // biting through between them in notches wherever they disagreed.
  ctx.fillStyle = pal.hair;
  ctx.beginPath();
  tracePoly(
    ctx,
    sample((u) => [u * 7.0, 1.7 + 0.6 * (1 - u * u)], 18),
    proj,
  );
  tracePoly(
    ctx,
    sample((u) => [-u * 7.0, 6.8], 8),
    proj,
  );
  ctx.closePath();
  ctx.fill();

  locks.forEach((l, i) => {
    ctx.fillStyle = pal.hair;
    ctx.beginPath();
    tracePoly(
      ctx,
      sample((u) => {
        const t = (u + 1) / 2;
        const s = l.s0 + (l.s1 - l.s0) * t;
        const edge = l.dip + (l.top - l.dip) * Math.pow(Math.abs(u), l.soft) + l.bow * (1 - u * u) * 0.5;
        return [s, edge];
      }, 18),
      proj,
    );
    const capTop = sample((u) => [l.s1 + (l.s0 - l.s1) * ((u + 1) / 2), 5.2], 6);
    tracePoly(ctx, capTop, proj);
    ctx.closePath();
    ctx.fill();

    if (i === 1) {
      ctx.strokeStyle = withAlpha(pal.hairLit, 0.9);
      ctx.lineWidth = px * 0.16;
      ctx.beginPath();
      tracePoly(
        ctx,
        sample((u) => {
          const t = (u + 1) / 2;
          const s = l.s0 + 0.25 + (l.s1 - l.s0 - 0.5) * t;
          return [s, l.dip + 0.62 + 0.34 * (1 - u * u)];
        }, 12),
        proj,
      );
      ctx.stroke();
    }
  });
}

/**
 * Shared character rig — skeleton, pose states, modifiers, IK.
 *
 * This module is **renderer-agnostic on purpose**. It is the prototype's
 * central claim: the part of Ink Tide's rider that is expensive to invent is
 * not the geometry, it is `riderAnim.ts`'s four-layer structure —
 *
 *     SIGNALS  → STATES → MODIFIERS → IK
 *
 * — and that structure is dimension-independent. `rig2d.js` draws the solved
 * skeleton as a silhouette on a canvas; `rig3d.js` feeds the *same* joint
 * angles into a Three.js hierarchy. Neither renderer computes a pose.
 *
 * Conventions
 *   - Character space: +X is the direction the character faces, +Y is up.
 *   - A bone angle of 0 points along +Y. Positive angles rotate from +Y toward
 *     +X, i.e. "forward". Angles are stored in degrees and converted once.
 *   - The bind pose IS the guard stance, so every pose state writes small
 *     deltas rather than absolute skeletons.
 *   - Side view: limbs come in `near`/`far` pairs. The far limb is drawn
 *     darker by the 2D renderer and offset in Z by the 3D one, which is what
 *     makes a two-dimensional fighting silhouette read as a body rather than
 *     as a stick figure.
 */

const RAD = Math.PI / 180;

// ---------------------------------------------------------------------------
// Skeleton
// ---------------------------------------------------------------------------

/**
 * `attach` is the fraction along the parent bone where this bone's base sits
 * (0 = parent base, 1 = parent tip). Arms hang off the chest tip, legs off the
 * hip base — hard-coding pixel offsets instead is what makes a rig break the
 * moment proportions are retuned.
 */
export const BONES = [
  { name: 'hips', parent: null, len: 0, bind: 0, attach: 0 },
  // Both combatants are women in their early twenties. The build lives partly
  // here and partly in the renderer's width tables: shorter torso, smaller
  // head, lighter arms. The *legs* are deliberately untouched — `applyFootIK`
  // solves against their lengths and `HIP_HEIGHT`, and retuning those is how
  // you get a rig that floats above the floor on one tick and sinks through
  // it on the next.
  { name: 'spine', parent: 'hips', len: 12.6, bind: -4, attach: 0 },
  { name: 'chest', parent: 'spine', len: 11.2, bind: -2, attach: 1 },
  { name: 'neck', parent: 'chest', len: 4.2, bind: 5, attach: 1 },
  { name: 'head', parent: 'neck', len: 8.2, bind: 1, attach: 1 },

  { name: 'armNearUp', parent: 'chest', len: 13.8, bind: 150, attach: 0.92 },
  { name: 'armNearFore', parent: 'armNearUp', len: 13.0, bind: -62, attach: 1 },
  { name: 'armNearHand', parent: 'armNearFore', len: 5, bind: -6, attach: 1 },

  { name: 'armFarUp', parent: 'chest', len: 13.8, bind: 163, attach: 0.92 },
  { name: 'armFarFore', parent: 'armFarUp', len: 13.0, bind: -38, attach: 1 },
  { name: 'armFarHand', parent: 'armFarFore', len: 5, bind: -4, attach: 1 },

  { name: 'legNearThigh', parent: 'hips', len: 21, bind: 172, attach: 0 },
  { name: 'legNearShin', parent: 'legNearThigh', len: 20, bind: 15, attach: 1 },
  { name: 'legNearFoot', parent: 'legNearShin', len: 7, bind: -96, attach: 1 },

  { name: 'legFarThigh', parent: 'hips', len: 21, bind: 192, attach: 0 },
  { name: 'legFarShin', parent: 'legFarThigh', len: 20, bind: 13, attach: 1 },
  { name: 'legFarFoot', parent: 'legFarShin', len: 7, bind: -101, attach: 1 },
];

export const BONE_INDEX = Object.fromEntries(BONES.map((b, i) => [b.name, i]));
export const BONE_COUNT = BONES.length;

/** Hip height in rig units; the ground plane is y = 0. */
export const HIP_HEIGHT = 51;

export function newPose() {
  return { a: new Float32Array(BONE_COUNT), hipX: 0, hipY: 0, roll: 0 };
}

function setA(pose, name, deg) {
  pose.a[BONE_INDEX[name]] = deg;
}

function addA(pose, name, deg) {
  pose.a[BONE_INDEX[name]] += deg;
}

// ---------------------------------------------------------------------------
// 1. SIGNALS — raw spectator state to the few smoothed quantities a pose keys
//    against. The renderer never sees a `cue` array; it sees these.
// ---------------------------------------------------------------------------

/**
 * `prev` is the previous tick's frame, used for the one genuinely temporal
 * signal (approach speed). Everything else is derived from this tick alone, so
 * a frame can be rendered in isolation — a property the capture harness needs.
 */
export function signalsFor(piece, frame, prevFrame, view) {
  const cues = new Set(piece.cues);
  const other = frame.pieces.find((p) => p.id !== piece.id) ?? null;
  const dist = other ? hexDistance(piece.position, other.position) : 3;

  let closing = 0;
  if (prevFrame && other) {
    const prevSelf = prevFrame.pieces.find((p) => p.id === piece.id);
    const prevOther = prevFrame.pieces.find((p) => p.id !== piece.id);
    if (prevSelf && prevOther) {
      closing = hexDistance(prevSelf.position, prevOther.position) - dist;
    }
  }

  return {
    tick: frame.tick,
    // A phase that advances with the tick so idle motion (breathing, robe
    // sway) is not frozen identically on every captured frame.
    t: (frame.tick * view.tick_millis) / 1000,
    attack: cues.has('attack') ? 1 : 0,
    hit: cues.has('hit') ? 1 : 0,
    evade: cues.has('evade') ? 1 : 0,
    stagger: cues.has('balance_broken') ? 1 : 0,
    down: cues.has('incapacitated') ? 1 : 0,
    dist,
    closing,
    /** Facing sign in the 2D plane: +1 draws to the right, -1 mirrors. */
    facing: piece.facing.q >= 0 ? 1 : -1,
  };
}

function hexDistance(a, b) {
  const dq = a.q - b.q;
  const dr = a.r - b.r;
  return (Math.abs(dq) + Math.abs(dq + dr) + Math.abs(dr)) / 2;
}

// ---------------------------------------------------------------------------
// 2. STATES — whole-body poses. Each writes an *absolute* pose; the blender
//    below mixes them by weight and renormalises, so a new state can be added
//    without touching the others.
// ---------------------------------------------------------------------------

/** Neutral 江湖 guard: weight back, blade-side forearm up, shoulders square. */
function poseGuard(p, s) {
  const breath = Math.sin(s.t * 2.1) * 1.4;
  setA(p, 'spine', breath * 0.5);
  setA(p, 'chest', breath * 0.7);
  setA(p, 'neck', -breath * 0.4);
  setA(p, 'armNearUp', -6 + breath * 0.6);
  setA(p, 'armNearFore', -4);
  setA(p, 'armFarUp', 4);
  setA(p, 'armFarFore', -8);
  p.hipY = -1.5;
}

/**
 * A downward diagonal cut, parameterised by phase so the still can be taken at
 * the most legible instant rather than at whatever the clock happened to be.
 * 0–0.35 windup, 0.35–0.65 strike, 0.65–1 follow-through.
 */
function poseAttack(p, s, phase) {
  const wind = smoothstep(0, 0.35, phase) * (1 - smoothstep(0.35, 0.6, phase));
  const strike = smoothstep(0.3, 0.62, phase);
  const follow = smoothstep(0.62, 1, phase);

  // Torso drives the cut: rotate back on the windup, forward through it.
  setA(p, 'spine', -13 * wind + 17 * strike - 5 * follow);
  setA(p, 'chest', -9 * wind + 15 * strike - 4 * follow);
  setA(p, 'neck', 5 * wind - 5 * strike);
  setA(p, 'head', 3 * wind - 6 * strike);

  // Blade arm: high and cocked, then extended forward and down.
  setA(p, 'armNearUp', -95 * wind - 44 * strike + 12 * follow);
  setA(p, 'armNearFore', 34 * wind + 46 * strike - 10 * follow);
  setA(p, 'armNearHand', -12 * wind + 16 * strike);

  // Off arm counterbalances backward — the detail that stops a swing reading
  // as a puppet waving one limb.
  setA(p, 'armFarUp', 26 * wind + 40 * strike);
  setA(p, 'armFarFore', -22 * wind - 34 * strike);

  // Rear leg drives, front leg receives the weight.
  setA(p, 'legNearThigh', -8 * wind - 22 * strike + 6 * follow);
  setA(p, 'legNearShin', 4 * wind + 16 * strike);
  setA(p, 'legFarThigh', 10 * wind + 24 * strike);
  setA(p, 'legFarShin', -6 * wind - 20 * strike);

  p.hipX = -2.5 * wind + 7 * strike + 2 * follow;
  p.hipY = -2 * wind - 4.5 * strike;
}

/** Weight dumped onto the rear leg, torso pulled off the line of attack. */
function poseEvade(p, s, phase) {
  const k = Math.sin(Math.PI * clamp01(phase));
  setA(p, 'spine', -16 * k);
  setA(p, 'chest', -12 * k);
  setA(p, 'neck', 9 * k);
  setA(p, 'head', 5 * k);
  setA(p, 'armNearUp', -30 * k);
  setA(p, 'armNearFore', -20 * k);
  setA(p, 'armFarUp', 34 * k);
  setA(p, 'armFarFore', -26 * k);
  setA(p, 'legNearThigh', 22 * k);
  setA(p, 'legNearShin', -26 * k);
  setA(p, 'legFarThigh', -14 * k);
  setA(p, 'legFarShin', 10 * k);
  p.hipX = -9 * k;
  p.hipY = -3 * k;
}

/** Balance broken: the body is no longer over its feet and knows it. */
function poseStagger(p, s, phase) {
  const k = smoothstep(0, 0.5, phase);
  setA(p, 'spine', 20 * k);
  setA(p, 'chest', 14 * k);
  setA(p, 'neck', -16 * k);
  setA(p, 'head', -10 * k);
  setA(p, 'armNearUp', -58 * k);
  setA(p, 'armNearFore', -34 * k);
  setA(p, 'armFarUp', -46 * k);
  setA(p, 'armFarFore', -40 * k);
  setA(p, 'legNearThigh', -30 * k);
  setA(p, 'legNearShin', 34 * k);
  setA(p, 'legFarThigh', 16 * k);
  setA(p, 'legFarShin', -8 * k);
  p.hipX = 6 * k;
  p.hipY = -6 * k;
  p.roll = 12 * k;
}

/** Down. Driven to both knees, torso folded over, weapon gone. */
function poseDown(p, s, phase) {
  // Two rewrites. The first read as a low guard, because the hips were still
  // half-height and the arms were still held out in front — the two things
  // that say "this body is still fighting". The second put the hips on the
  // floor, and the capture came back as an unreadable heap of cloth: a rig
  // whose whole silhouette vocabulary is vertical has nothing left to read
  // once the spine is horizontal.
  //
  // A driven-to-the-knees collapse is the pose that survives this renderer.
  // The shins fold flat along the ground, the hips sit back on the heels, the
  // spine pitches ~50° forward and the head hangs — and the figure still has
  // a standing figure's readable outline while saying, unmistakably, that it
  // is finished.
  const k = smoothstep(0, 0.7, phase);
  setA(p, 'spine', 30 * k);
  setA(p, 'chest', 16 * k);
  setA(p, 'neck', -26 * k);
  setA(p, 'head', -16 * k);
  // Arms hang past the torso rather than reaching — a braced arm still reads
  // as intent.
  setA(p, 'armNearUp', -14 * k);
  setA(p, 'armNearFore', 40 * k);
  setA(p, 'armFarUp', -26 * k);
  setA(p, 'armFarFore', 48 * k);
  // Thighs near-vertical so the knees touch down, shins folded back flat.
  setA(p, 'legNearThigh', 2 * k);
  setA(p, 'legNearShin', 73 * k);
  setA(p, 'legNearFoot', 104 * k);
  setA(p, 'legFarThigh', -6 * k);
  setA(p, 'legFarShin', 63 * k);
  setA(p, 'legFarFoot', 109 * k);
  p.hipX = -3 * k;
  p.hipY = -HIP_HEIGHT * 0.52 * k;
  p.roll = 9 * k;
}

const STATES = [
  { name: 'guard', fn: poseGuard },
  { name: 'attack', fn: poseAttack },
  { name: 'evade', fn: poseEvade },
  { name: 'stagger', fn: poseStagger },
  { name: 'down', fn: poseDown },
];

/** State weights from signals. `down` dominates everything — it is terminal. */
function stateWeights(s) {
  if (s.down) return { guard: 0, attack: 0, evade: 0, stagger: 0, down: 1 };
  const w = { guard: 1, attack: 0, evade: 0, stagger: 0, down: 0 };
  if (s.attack) {
    w.attack = 1;
    w.guard = 0.15;
  }
  if (s.evade) {
    w.evade = 1;
    w.guard = 0.1;
  }
  if (s.stagger) {
    w.stagger = 1.2;
    w.guard = 0.05;
    w.attack *= 0.4;
  }
  return w;
}

// ---------------------------------------------------------------------------
// 2b. EXPRESSION — the same four-layer structure, applied to the face.
//
// The face is not a decoration bolted onto the renderer: it is another
// consumer of the same SIGNALS, blended by the same STATES → MODIFIERS rule.
// A renderer receives channel values, never a `cue` array, exactly as it
// receives joint angles rather than a pose name.
//
// `hit` is a modifier here for the identical reason it is one in the body
// layer: in this fixture both combatants are struck on the tick they strike,
// and a face that stopped attacking the instant it was hit would contradict
// the body it sits on.
// ---------------------------------------------------------------------------

/**
 * Expression channels. All are 0..1 except the signed ones, which are -1..1.
 *
 *   browInner   signed. Negative furrows the inner brow down (anger, focus);
 *               positive lifts it (pain, fear).
 *   browOuter   signed. Same convention, outer end.
 *   lid         0 open, 1 shut. Drives the upper lid down over the iris.
 *   widen       Pulls the upper lid *up*, showing sclera above the iris.
 *   squint      Raises the lower lid.
 *   mouthCurve  signed. Negative is a downturn.
 *   mouthOpen   0..1.
 *   jaw         Clench. Adds the masseter shadow and narrows the mouth.
 *   gaze        signed, forward/back along the facing direction.
 */
export function newExpression() {
  return {
    browInner: 0,
    browOuter: 0,
    lid: 0,
    widen: 0,
    squint: 0,
    mouthCurve: 0,
    mouthOpen: 0,
    jaw: 0,
    gaze: 0,
  };
}

/** Steady, watching. The face equivalent of the guard stance. */
function exprAlert(e) {
  e.browInner = -0.08;
  e.lid = 0.06;
  e.mouthCurve = -0.12;
  e.gaze = 0.08;
}

/** Committed to a cut: brow down and in, lower lid up, jaw set. */
function exprFocus(e) {
  e.browInner = -0.9;
  e.browOuter = -0.4;
  e.lid = 0.12;
  e.squint = 0.42;
  e.mouthCurve = -0.5;
  e.jaw = 1;
  e.gaze = 0.18;
}

/** Slipping a strike: eyes wide, brow up, mouth tight. Authored, not fired by
 *  this fixture — the same gap the body's `evade` state has. */
function exprEvade(e) {
  e.browInner = 0.45;
  e.browOuter = 0.2;
  e.widen = 0.7;
  e.mouthCurve = -0.2;
  e.jaw = 0.4;
  e.gaze = -0.15;
}

/** Balance gone: strain rather than pain. Also unfired by this fixture. */
function exprStrain(e) {
  e.browInner = 0.55;
  e.browOuter = -0.1;
  e.squint = 0.35;
  e.mouthOpen = 0.5;
  e.mouthCurve = -0.35;
  e.jaw = 0.6;
}

/** Down. Eyes closed, everything slack — no clenched jaw, no held brow. */
function exprSlack(e) {
  e.browInner = 0.18;
  e.browOuter = 0.12;
  e.lid = 1;
  e.mouthOpen = 0.26;
  e.mouthCurve = -0.15;
  e.jaw = 0;
  e.gaze = -0.3;
}

const EXPR_STATES = [
  { name: 'alert', fn: exprAlert },
  { name: 'focus', fn: exprFocus },
  { name: 'evade', fn: exprEvade },
  { name: 'strain', fn: exprStrain },
  { name: 'slack', fn: exprSlack },
];

function exprWeights(s) {
  if (s.down) return { alert: 0, focus: 0, evade: 0, strain: 0, slack: 1 };
  const w = { alert: 1, focus: 0, evade: 0, strain: 0, slack: 0 };
  if (s.attack) {
    w.focus = 1;
    w.alert = 0.15;
  }
  if (s.evade) {
    w.evade = 1;
    w.alert = 0.1;
  }
  if (s.stagger) {
    w.strain = 1.2;
    w.alert = 0.05;
    w.focus *= 0.4;
  }
  return w;
}

/** Taking a blow: eyes screwed shut, brow driven up, mouth open. Additive. */
function modFlinch(e, s, phase) {
  if (!s.hit) return;
  const k = Math.sin(Math.PI * clamp01(phase)) * (1 - s.down * 0.85);
  e.lid += 0.6 * k;
  e.squint += 0.55 * k;
  e.browInner += 0.6 * k;
  e.browOuter += 0.25 * k;
  e.mouthOpen += 0.5 * k;
  e.mouthCurve -= 0.3 * k;
  e.jaw += 0.3 * k;
}

const _exprScratch = EXPR_STATES.map(() => newExpression());
const EXPR_KEYS = Object.keys(newExpression());

/**
 * Build the face for one piece on one tick.
 *
 * `bias` is a *character* constant, not a signal — a permanent set of the
 * features that distinguishes one fighter from another (the Black Serpent's
 * sneer, say). It is added after the blend so it survives every state.
 */
export function expressionFor(signals, phase = 0.5, bias = null) {
  const weights = exprWeights(signals);
  const out = newExpression();

  let total = 0;
  EXPR_STATES.forEach((state, i) => {
    const w = weights[state.name] ?? 0;
    if (w <= 0) return;
    const buf = _exprScratch[i];
    for (const k of EXPR_KEYS) buf[k] = 0;
    state.fn(buf, signals, phase);
    for (const k of EXPR_KEYS) out[k] += buf[k] * w;
    total += w;
  });
  if (total > 0) for (const k of EXPR_KEYS) out[k] /= total;

  modFlinch(out, signals, phase);
  if (bias) for (const k of EXPR_KEYS) out[k] += bias[k] ?? 0;

  out.lid = clamp01(out.lid);
  out.widen = clamp01(out.widen);
  out.squint = clamp01(out.squint);
  out.mouthOpen = clamp01(out.mouthOpen);
  out.jaw = clamp01(out.jaw);
  out.browInner = clampSigned(out.browInner);
  out.browOuter = clampSigned(out.browOuter);
  out.mouthCurve = clampSigned(out.mouthCurve);
  out.gaze = clampSigned(out.gaze);
  return out;
}

function clampSigned(v) {
  return v < -1 ? -1 : v > 1 ? 1 : v;
}

// ---------------------------------------------------------------------------
// 3. MODIFIERS — additive layers on top of the blend.
// ---------------------------------------------------------------------------

/**
 * Being hit is *not* a whole-body state: it lands on top of whatever the
 * character was already doing, which is why it is a modifier. Modelling it as
 * a state would make a character stop attacking the instant it took a hit,
 * and the real exchange in this fixture has both happening on the same tick.
 */
function modHitRecoil(p, s, phase) {
  if (!s.hit) return;
  const k = Math.sin(Math.PI * clamp01(phase)) * (1 - s.down * 0.6);
  addA(p, 'spine', -9 * k);
  addA(p, 'chest', -7 * k);
  addA(p, 'neck', -13 * k);
  addA(p, 'head', -9 * k);
  addA(p, 'armFarUp', -12 * k);
  addA(p, 'legNearThigh', 6 * k);
  p.hipX += -4.5 * k;
}

/** Head leads the body — a small lag that reads as intent. */
function modHeadLead(p, s) {
  addA(p, 'head', -p.a[BONE_INDEX.chest] * 0.22);
  addA(p, 'neck', -p.a[BONE_INDEX.chest] * 0.12);
}

// ---------------------------------------------------------------------------
// 4. IK — plant the feet.
// ---------------------------------------------------------------------------

/**
 * Two-bone IK, solved in world space against the ground plane.
 *
 * This is the layer that sells it, and it is the same reason Ink Tide solves
 * the rider's hands onto the handlebars: whatever the torso does, the feet
 * stay where the ground is. Without it, every torso rotation slides the
 * character through the floor and the whole rig reads as a puppet.
 */
function solveFootIK(joints, pose, thighName, shinName, footName) {
  const thigh = joints[BONE_INDEX[thighName]];
  const shin = joints[BONE_INDEX[shinName]];
  const foot = joints[BONE_INDEX[footName]];
  const ankle = { x: shin.tipX, y: shin.tipY };
  if (ankle.y >= 0) return null;

  // Target: same horizontal position, lifted to the ground plane.
  return { x: ankle.x, y: 0, thigh, shin, foot };
}

function twoBoneIK(baseX, baseY, len1, len2, targetX, targetY, bendSign) {
  const dx = targetX - baseX;
  const dy = targetY - baseY;
  const dist = Math.max(1e-4, Math.min(Math.hypot(dx, dy), len1 + len2 - 1e-3));
  const base = Math.atan2(dx, dy); // 0 = +Y, matching the rig convention
  const cosA = (len1 * len1 + dist * dist - len2 * len2) / (2 * len1 * dist);
  const a = Math.acos(Math.max(-1, Math.min(1, cosA)));
  const cosB = (len1 * len1 + len2 * len2 - dist * dist) / (2 * len1 * len2);
  const b = Math.acos(Math.max(-1, Math.min(1, cosB)));
  return {
    upper: (base + a * bendSign) / RAD,
    lower: (-(Math.PI - b) * bendSign) / RAD,
  };
}

// ---------------------------------------------------------------------------
// Solver
// ---------------------------------------------------------------------------

/**
 * Resolve a pose into world-space joints.
 *
 * Returns one entry per bone: base point, tip point, and world angle. Both
 * renderers consume exactly this — the 2D one strokes silhouettes along the
 * segments, the 3D one hangs meshes off the same angles.
 */
export function solveSkeleton(pose) {
  const joints = new Array(BONE_COUNT);
  for (let i = 0; i < BONE_COUNT; i += 1) {
    const bone = BONES[i];
    let baseX;
    let baseY;
    let parentAngle;
    if (bone.parent === null) {
      baseX = pose.hipX;
      baseY = HIP_HEIGHT + pose.hipY;
      parentAngle = pose.roll;
    } else {
      const parent = joints[BONE_INDEX[bone.parent]];
      baseX = parent.baseX + (parent.tipX - parent.baseX) * bone.attach;
      baseY = parent.baseY + (parent.tipY - parent.baseY) * bone.attach;
      parentAngle = parent.angle;
    }
    const angle = parentAngle + bone.bind + pose.a[i];
    const rad = angle * RAD;
    joints[i] = {
      name: bone.name,
      baseX,
      baseY,
      angle,
      len: bone.len,
      tipX: baseX + Math.sin(rad) * bone.len,
      tipY: baseY + Math.cos(rad) * bone.len,
    };
  }
  return joints;
}

/** Re-solve the two legs so the feet rest on y = 0 where they would sink. */
function applyFootIK(pose) {
  for (const side of ['Near', 'Far']) {
    let joints = solveSkeleton(pose);
    const target = solveFootIK(joints, pose, `leg${side}Thigh`, `leg${side}Shin`, `leg${side}Foot`);
    if (!target) continue;
    const thigh = joints[BONE_INDEX[`leg${side}Thigh`]];
    const solved = twoBoneIK(
      thigh.baseX,
      thigh.baseY,
      BONES[BONE_INDEX[`leg${side}Thigh`]].len,
      BONES[BONE_INDEX[`leg${side}Shin`]].len,
      target.x,
      target.y,
      1,
    );
    const hipsAngle = pose.roll;
    pose.a[BONE_INDEX[`leg${side}Thigh`]] = solved.upper - hipsAngle - BONES[BONE_INDEX[`leg${side}Thigh`]].bind;
    pose.a[BONE_INDEX[`leg${side}Shin`]] = solved.lower - BONES[BONE_INDEX[`leg${side}Shin`]].bind;
  }
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

const _scratch = STATES.map(() => newPose());

/**
 * Build the final pose for one piece on one tick.
 *
 * `phase` is where inside the tick to sample (0..1). Stills use a fixed value
 * so a capture is reproducible; a real playback would advance it with the
 * clock.
 */
export function poseFor(signals, phase = 0.5) {
  const weights = stateWeights(signals);
  const out = newPose();

  let total = 0;
  STATES.forEach((state, i) => {
    const w = weights[state.name] ?? 0;
    if (w <= 0) return;
    const buf = _scratch[i];
    buf.a.fill(0);
    buf.hipX = 0;
    buf.hipY = 0;
    buf.roll = 0;
    state.fn(buf, signals, phase);
    for (let b = 0; b < BONE_COUNT; b += 1) out.a[b] += buf.a[b] * w;
    out.hipX += buf.hipX * w;
    out.hipY += buf.hipY * w;
    out.roll += buf.roll * w;
    total += w;
  });

  if (total > 0) {
    for (let b = 0; b < BONE_COUNT; b += 1) out.a[b] /= total;
    out.hipX /= total;
    out.hipY /= total;
    out.roll /= total;
  }

  modHitRecoil(out, signals, phase);
  modHeadLead(out, signals);
  if (!signals.down) applyFootIK(out);

  return out;
}

// ---------------------------------------------------------------------------
// Robe — a verlet chain, integrated across ticks.
// ---------------------------------------------------------------------------

/**
 * Wide sleeves and a long 장삼 are the silhouette of this genre, and a hem
 * that only ever hangs straight down throws that away. The chain is
 * integrated in tick order with a fixed step, so the capture stays
 * reproducible while the cloth still carries motion from the pose before it.
 */
/**
 * A strand chain — the same verlet solver the hem uses, anchored somewhere
 * else. 청류문 검수 wears her hair 산발 (loose and unbound), and loose hair that
 * only ever hangs straight down throws away the one silhouette cue that
 * separates her from a bound-haired opponent at board scale. It is the same
 * integrator, so it inherits the same determinism.
 */
export function createStrand(segments = 3, segLen = 6) {
  return createRobe(segments, segLen);
}

export const stepStrand = stepRobe;

export function createRobe(segments = 4, segLen = 11) {
  const pts = [];
  for (let i = 0; i <= segments; i += 1) {
    pts.push({ x: 0, y: HIP_HEIGHT - i * segLen, px: 0, py: HIP_HEIGHT - i * segLen });
  }
  return { pts, segLen, segments };
}

export function stepRobe(robe, anchorX, anchorY, sweep, steps = 6, dt = 1 / 60) {
  for (let s = 0; s < steps; s += 1) {
    robe.pts[0].x = anchorX;
    robe.pts[0].y = anchorY;
    robe.pts[0].px = anchorX;
    robe.pts[0].py = anchorY;
    for (let i = 1; i < robe.pts.length; i += 1) {
      const p = robe.pts[i];
      // Sweep is clamped and damped hard. Unclamped, a fast closing tick threw
      // the hem out sideways and the robe read as a flag on a pole rather than
      // as cloth hanging off hips.
      const drag = Math.max(-1.6, Math.min(1.6, sweep)) * dt * 9;
      const vx = (p.x - p.px) * 0.82 + drag;
      const vy = (p.y - p.py) * 0.82;
      p.px = p.x;
      p.py = p.y;
      p.x += vx;
      p.y += vy - 130 * dt * dt * 30;
    }
    for (let k = 0; k < 3; k += 1) {
      for (let i = 1; i < robe.pts.length; i += 1) {
        const a = robe.pts[i - 1];
        const b = robe.pts[i];
        const dx = b.x - a.x;
        const dy = b.y - a.y;
        const d = Math.hypot(dx, dy) || 1e-4;
        const diff = (d - robe.segLen) / d;
        if (i - 1 > 0) {
          a.x += dx * diff * 0.5;
          a.y += dy * diff * 0.5;
          b.x -= dx * diff * 0.5;
          b.y -= dy * diff * 0.5;
        } else {
          b.x -= dx * diff;
          b.y -= dy * diff;
        }
      }
    }
  }
  return robe;
}

// ---------------------------------------------------------------------------

export function clamp01(v) {
  return v < 0 ? 0 : v > 1 ? 1 : v;
}

export function smoothstep(edge0, edge1, x) {
  const t = clamp01((x - edge0) / (edge1 - edge0 || 1e-6));
  return t * t * (3 - 2 * t);
}

export { RAD };

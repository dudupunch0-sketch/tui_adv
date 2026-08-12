/**
 * Style 2 — low-poly 3D, cel-shaded, fixed camera (Three.js).
 *
 * The Ink Tide approach transplanted: every mesh is assembled from primitives
 * in code, lighting is quantised diffuse rather than PBR, and silhouettes are
 * drawn with an inverted hull pushed along the normal at a *constant screen
 * width*. Nothing here is fetched — no textures, no HDRIs, no models.
 *
 * It is fed by the same `rig.js` pose the other style uses. Bone angles map to
 * `rotation.z` on a group hierarchy; the only degree of freedom 3D gets for
 * free is the Y rotation that turns a fighter toward its hex neighbour.
 *
 * Hard rules this module keeps
 *   - No `MeshStandardMaterial` / `MeshPhysicalMaterial`, no env maps, no
 *     shadow maps. Diffuse is three hard bands between three *authored*
 *     colours (not a multiply), so a dark robe keeps its form instead of
 *     sliding wholesale into the shadow band — the exact failure the first
 *     capture pass of this prototype hit.
 *   - Speculars are drawn, not computed: a hard step on a Blinn term.
 *   - Every colour comes from `PAL3D` below. No literals at a draw site.
 *   - Sides differ by **shape** first: 청류문 is tall, topknotted, straight
 *     jian, long hem; 흑사방 is broad, 삿갓-hatted, mantled, curved dao.
 *     Colour only reinforces what the silhouette already says.
 */

import * as THREE from 'three';
import { mergeGeometries } from './node_modules/three/examples/jsm/utils/BufferGeometryUtils.js';
import { BONES, BONE_INDEX, HIP_HEIGHT, poseFor, solveSkeleton } from './rig.js';
import { drawFace } from './face.js';

const RAD = Math.PI / 180;
const SQRT3 = Math.sqrt(3);

// ---------------------------------------------------------------------------
// Palette — the only place a colour literal may appear in this module.
// ---------------------------------------------------------------------------

export const PAL3D = {
  /** Light rig. Value does the modelling; hue does the mood. */
  light: {
    key: 0xffe7c4, // warm low sun, from screen-left
    bounce: 0x35538c, // cool sky fill that tints the shadow band
    rim: 0xa8dcff, // cold back rim separating figure from board
    skinShadow: 0xb0684a, // the shadow side of skin goes warm, not merely dark
    spec: 0xfff4dd,
  },

  world: {
    tile: 0x39414f,
    tilePath: 0x454e5f, // the row the bout is actually fought along
    tileEdge: 0x272e3b, // outer ring, falls away into the dark
    tileInk: 0x171c26,
    plinth: 0x191e28,
    allyCell: 0x2a5f66,
    enemyCell: 0x5e2a37,
    shadow: 0x121620,
    impact: 0xffe2a8,
    ridgeNear: 0x1e2536,
    ridgeFar: 0x2b3349,
    sky: 0x161b26,
    disc: 0x515c7a, // the pale disc behind the ridges
    poleWood: 0x3a3242,
    bannerAlly: 0x2f7d76,
    bannerEnemy: 0x8c3446,
    bannerInk: 0x14171f,
  },

  ally: {
    robe: 0x466f6d,
    under: 0x2f4f50,
    trim: 0x8ab3aa,
    collar: 0xd7ddd8,
    sash: 0xbda884,
    ink: 0x0d1216,
    headInk: 0x3a2b28,
    skin: 0xd8b393,
    leather: 0x4a3a2c,
    hair: 0x241d24,
    hairLit: 0x413546,
    steel: 0x9aabbe,
    steelInk: 0x1d2534,
    steelEdge: 0xf2f7fb,
    trail: 0xa9e8dc,
    cordAccent: 0xbfe6d8,
  },

  enemy: {
    robe: 0x37303f,
    under: 0x241f2c,
    trim: 0x7b3746,
    collar: 0x5e2b37,
    sash: 0x6d3040,
    ink: 0x090709,
    headInk: 0x33221f,
    skin: 0xc09b78,
    leather: 0x2c2026,
    hair: 0x141119,
    hairLit: 0x322a3c,
    steel: 0x8d9cb0,
    steelInk: 0x171d29,
    steelEdge: 0xe8eef5,
    trail: 0xf0a8b4,
    cordAccent: 0xd2a35a,
  },
};

// ---------------------------------------------------------------------------
// Shared uniforms
//
// The outline width and the key direction are shared *uniform objects*, not
// copies. Every material references the same instance, so `tuneForCamera` can
// keep every silhouette at the same pixel width without walking the scene.
// ---------------------------------------------------------------------------

const U_LIGHT = { value: new THREE.Vector3(-0.52, 0.62, 0.58).normalize() };
const U_RIM = { value: new THREE.Vector3(0.34, 0.30, -0.89).normalize() };
/** Multiply by view-space depth to get one pixel of screen offset. */
const U_PIXEL = { value: 0.0016 };

/**
 * Lock the inverted hull to true pixels.
 *
 * Uniformly scaling a mesh is the tempting shortcut and it fails: a wrist gets
 * a hairline and a torso gets a slab. The offset has to be along the normal in
 * view space, scaled by depth — and the scale factor is a property of the
 * projection, so it is computed once here rather than eyeballed per part.
 */
export function tuneForCamera(camera, pixelHeight) {
  U_PIXEL.value = (2 * Math.tan((camera.fov * RAD) / 2)) / pixelHeight;
}

/**
 * Author colours in sRGB, shade in linear, write back to sRGB.
 *
 * A raw `ShaderMaterial` does not get Three's output-colour-space conversion,
 * so writing a linear colour straight to `gl_FragColor` lands it in an sRGB
 * framebuffer uncorrected — every value roughly halves. The first capture of
 * this screen was uniformly near-black for exactly this reason; the board read
 * as a hole. Bands are computed in linear (where a midpoint is a midpoint) and
 * encoded once at the end.
 */
const SRGB_GLSL = /* glsl */ `
  vec3 toSRGB(vec3 c) {
    c = max(c, vec3(0.0));
    return mix(pow(c, vec3(0.41666)) * 1.055 - 0.055, c * 12.92, step(c, vec3(0.0031308)));
  }
`;

/**
 * `THREE.Color(hex)` already lands in the renderer's linear working space —
 * calling `convertSRGBToLinear()` on top of it converts twice and costs about
 * half a stop. `calibrate.html` measures this: 0x808080 round-trips to 128
 * through this path and to 55 through the double-converted one.
 */
function lin(hex) {
  return new THREE.Color(hex);
}

/**
 * Palette arithmetic happens in sRGB, deliberately.
 *
 * `new THREE.Color(hex)` decodes to linear, and mixing a near-black toward a
 * bright key *in linear* lifts it enormously once it is re-encoded: the boots
 * and the 흑사방 robe both came out tan in a capture because an 11% mix moved
 * a 0x0b value to 0x61. Loading the hex with `LinearSRGBColorSpace` keeps the
 * components as the authored bytes, so a 11% mix is 11% of the way as an
 * illustrator would mean it.
 */
function rawColor(hex) {
  return new THREE.Color().setHex(hex, THREE.LinearSRGBColorSpace);
}

function mixCol(aHex, bHex, t) {
  return rawColor(aHex).lerp(rawColor(bHex), t).getHex(THREE.LinearSRGBColorSpace);
}

function scaleCol(hex, k) {
  const c = rawColor(hex);
  c.r = Math.min(1, c.r * k);
  c.g = Math.min(1, c.g * k);
  c.b = Math.min(1, c.b * k);
  return c.getHex(THREE.LinearSRGBColorSpace);
}

// ---------------------------------------------------------------------------
// Cel material
// ---------------------------------------------------------------------------

/**
 * Three hard bands between three authored colours.
 *
 * The naive cel shader multiplies the base colour by a band factor. On a dark
 * robe that collapses: 0.56 × already-dark is mud, and the whole garment lands
 * in one indistinguishable value. So the bands are *interpolation targets*
 * derived per material — the shadow leans cool and does not go below a floor,
 * the light leans warm — which keeps a 12%-value robe legible.
 *
 * Band edges are deliberately uneven. A wide lit band with a late, narrow
 * shadow reads as illustration; even thirds read as a technical demo.
 */
function celMaterial(colorHex, opts = {}) {
  const {
    shadowMix = 0.22,
    shadowDim = 0.54,
    litMix = 0.11,
    litGain = 1.04,
    edgeHi = 0.44,
    edgeLo = 0.02,
    rim = 0.30,
    rimSharp = 4.2,
    spec = 0,
    specSharp = 46,
    specCut = 0.62,
    side = THREE.FrontSide,
  } = opts;

  const shadowHex = mixCol(scaleCol(colorHex, shadowDim), PAL3D.light.bounce, shadowMix);
  const litHex = mixCol(scaleCol(colorHex, litGain), PAL3D.light.key, litMix);

  return new THREE.ShaderMaterial({
    side,
    uniforms: {
      uMid: { value: lin(colorHex) },
      uLit: { value: lin(litHex) },
      uDark: { value: lin(shadowHex) },
      uRimCol: { value: lin(PAL3D.light.rim) },
      uSpecCol: { value: lin(PAL3D.light.spec) },
      uLight: U_LIGHT,
      uRimDir: U_RIM,
      uEdge: { value: new THREE.Vector2(edgeHi, edgeLo) },
      uRim: { value: rim },
      uRimSharp: { value: rimSharp },
      uSpec: { value: spec },
      uSpecSharp: { value: specSharp },
      uSpecCut: { value: specCut },
    },
    vertexShader: /* glsl */ `
      varying vec3 vN;
      varying vec3 vV;
      void main() {
        vN = normalize(normalMatrix * normal);
        vec4 mv = modelViewMatrix * vec4(position, 1.0);
        vV = mv.xyz;
        gl_Position = projectionMatrix * mv;
      }
    `,
    fragmentShader:
      SRGB_GLSL +
      /* glsl */ `
      uniform vec3 uMid, uLit, uDark, uRimCol, uSpecCol, uLight, uRimDir;
      uniform vec2 uEdge;
      uniform float uRim, uRimSharp, uSpec, uSpecSharp, uSpecCut;
      varying vec3 vN;
      varying vec3 vV;
      void main() {
        vec3 n = normalize(vN);
        if (!gl_FrontFacing) n = -n;
        vec3 l = normalize((viewMatrix * vec4(uLight, 0.0)).xyz);
        vec3 r = normalize((viewMatrix * vec4(uRimDir, 0.0)).xyz);
        vec3 v = normalize(-vV);
        float ndl = dot(n, l);

        // Three bands. Hard steps: no smoothstep anywhere on the terminator.
        vec3 col = ndl > uEdge.x ? uLit : (ndl > uEdge.y ? uMid : uDark);

        // Drawn specular: a hard-cut Blinn lobe, not an energy term.
        if (uSpec > 0.0) {
          vec3 h = normalize(l + v);
          float s = pow(max(dot(n, h), 0.0), uSpecSharp);
          col = mix(col, uSpecCol, step(uSpecCut, s) * uSpec);
        }

        // Rim: also hard-stepped, and gated to the back-light hemisphere so it
        // outlines the figure instead of haloing it uniformly.
        float fres = pow(1.0 - max(dot(n, v), 0.0), uRimSharp);
        float gate = step(0.05, dot(n, r));
        col = mix(col, uRimCol, step(0.48, fres) * gate * uRim);

        gl_FragColor = vec4(toSRGB(col), 1.0);
      }
    `,
  });
}

function outlineMaterial(colorHex, widthPx) {
  return new THREE.ShaderMaterial({
    side: THREE.BackSide,
    uniforms: {
      uColor: { value: lin(colorHex) },
      uWidth: { value: widthPx },
      uPixel: U_PIXEL,
    },
    vertexShader: /* glsl */ `
      uniform float uWidth;
      uniform float uPixel;
      void main() {
        vec3 n = normalize(normalMatrix * normal);
        vec4 mv = modelViewMatrix * vec4(position, 1.0);
        mv.xyz += n * uWidth * uPixel * (-mv.z);
        gl_Position = projectionMatrix * mv;
      }
    `,
    fragmentShader:
      SRGB_GLSL +
      /* glsl */ `
      uniform vec3 uColor;
      void main() { gl_FragColor = vec4(toSRGB(uColor), 1.0); }
    `,
  });
}

/** Skull radius. Shared by the mesh and by `face.js`'s UV projector. */
export const HEAD_RADIUS = 5.3;
/** Mesh scale of the skull. `face.js` divides it back out of every feature. */
export const HEAD_ASPECT = { y: 1.06, z: 0.88 };
const FACE_TEX = { w: 1024, h: 512 };

/**
 * Skin is very nearly unlit, and that is the whole trick.
 *
 * Everything else here is a hard three-band ladder. Run that ladder across a
 * face and no texture can save it: the terminator cuts the eye line in half
 * and the head reads as a faceted object wearing a decal. So the head keeps
 * only a whisper of NdotL — a wide, soft, warm-shifted two-tone that never
 * goes properly dark — and **all** the modelling that matters (jaw shadow,
 * the overhang under the hair, the blush under the lower lid) is *painted*
 * into the texture by `face.js`.
 *
 * That is how anime-styled 3D normally resolves this, and it stays inside the
 * no-PBR rule: the shading is drawn, and drawn shading is authored shading.
 */
function skinMaterial(colorHex, faceTexture, opts) {
  const o = opts || {};
  const warmMix = o.warmMix === undefined ? 0.4 : o.warmMix;
  const shadowDim = o.shadowDim === undefined ? 0.86 : o.shadowDim;
  const litMix = o.litMix === undefined ? 0.08 : o.litMix;
  const quantise = o.quantise === undefined ? 0 : o.quantise;
  const rim = o.rim === undefined ? 0.2 : o.rim;

  const shadowHex = mixCol(scaleCol(colorHex, shadowDim), PAL3D.light.skinShadow, warmMix);
  const litHex = mixCol(colorHex, PAL3D.light.key, litMix);

  return new THREE.ShaderMaterial({
    uniforms: {
      uSkin: { value: lin(colorHex) },
      uLit: { value: lin(litHex) },
      uDark: { value: lin(shadowHex) },
      uRimCol: { value: lin(PAL3D.light.rim) },
      uFace: { value: faceTexture },
      uLight: U_LIGHT,
      uRimDir: U_RIM,
      uQuant: { value: quantise },
      uRim: { value: rim },
    },
    vertexShader: /* glsl */ `
      varying vec3 vN;
      varying vec3 vV;
      varying vec2 vUvF;
      void main() {
        vN = normalize(normalMatrix * normal);
        vUvF = uv;
        vec4 mv = modelViewMatrix * vec4(position, 1.0);
        vV = mv.xyz;
        gl_Position = projectionMatrix * mv;
      }
    `,
    fragmentShader:
      SRGB_GLSL +
      /* glsl */ `
      uniform vec3 uSkin, uLit, uDark, uRimCol, uLight, uRimDir;
      uniform sampler2D uFace;
      uniform float uQuant, uRim;
      varying vec3 vN;
      varying vec3 vV;
      varying vec2 vUvF;
      vec3 toLinear(vec3 c) {
        return mix(pow((c + 0.055) / 1.055, vec3(2.4)), c / 12.92, step(c, vec3(0.04045)));
      }
      void main() {
        vec3 n = normalize(vN);
        if (!gl_FrontFacing) n = -n;
        vec3 l = normalize((viewMatrix * vec4(uLight, 0.0)).xyz);
        vec3 v = normalize(-vV);
        float ndl = dot(n, l);

        // A ShaderMaterial gets no automatic sRGB decode, so the canvas decal
        // is linearised here before it meets a linear skin colour.
        vec4 face = texture2D(uFace, vUvF);
        vec3 decal = toLinear(face.rgb);

        // Deliberately far too wide to be a terminator. Across a head-sized
        // sphere this is a gentle lean, not a light/shade split.
        float t = smoothstep(-0.75, 0.85, ndl);
        t = mix(t, floor(t * 3.0 + 0.5) / 3.0, uQuant);

        vec3 warmTint = uDark / max(uSkin, vec3(0.002));
        vec3 coolTint = uLit / max(uSkin, vec3(0.002));
        vec3 base = mix(uSkin, decal, face.a);
        vec3 col = base * mix(warmTint, coolTint, t);

        float fres = pow(1.0 - max(dot(n, v), 0.0), 3.6);
        float gate = step(0.05, dot(n, normalize((viewMatrix * vec4(uRimDir, 0.0)).xyz)));
        col = mix(col, uRimCol, smoothstep(0.42, 0.74, fres) * gate * uRim);

        gl_FragColor = vec4(toSRGB(col), 1.0);
      }
    `,
  });
}

/**
 * One canvas + texture per character. Repainting a single 1024×512 canvas per
 * tick is cheap; caching a texture per expression is not, and the expression
 * is continuous anyway.
 */
let BLANK_DECAL = null;
function blankDecal() {
  if (!BLANK_DECAL) {
    const c = document.createElement('canvas');
    c.width = c.height = 2;
    BLANK_DECAL = new THREE.CanvasTexture(c);
    BLANK_DECAL.colorSpace = THREE.NoColorSpace;
  }
  return BLANK_DECAL;
}

function createFaceSurface(side) {
  const canvas = document.createElement('canvas');
  canvas.width = FACE_TEX.w;
  canvas.height = FACE_TEX.h;
  const ctx = canvas.getContext('2d');
  const texture = new THREE.CanvasTexture(canvas);
  texture.colorSpace = THREE.NoColorSpace;
  texture.minFilter = THREE.LinearMipmapLinearFilter;
  texture.magFilter = THREE.LinearFilter;
  texture.anisotropy = 8;
  return { canvas, ctx, texture, side };
}

/**
 * Repaint a character's face from a solved expression.
 *
 * `gazeLateral` is the eyeline cheat, made explicit: the head is turned toward
 * the camera so the face can be seen at all, and the irises are pushed back
 * the other way so the fighter is still looking at the fighter it is fighting.
 */
export function updateFace3D(character, expression, gazeLateral) {
  drawFace(character.face.ctx, {
    width: FACE_TEX.w,
    height: FACE_TEX.h,
    radius: HEAD_RADIUS,
    aspect: HEAD_ASPECT,
    side: character.side,
    expression,
    gazeLateral: gazeLateral || 0,
  });
  character.face.texture.needsUpdate = true;
}

/**
 * Wipe the decal, leaving a plain skin-toned head.
 *
 * The head material composites the face *under* its lighting, so an empty
 * decal is not a stub — it is the same shader with nothing drawn on it, and
 * the head keeps its shape, its outline and its soft warm ramp.
 */
export function clearFace3D(character) {
  const { ctx, canvas } = character.face;
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  character.face.texture.needsUpdate = true;
}

/**
 * Turn the head toward the viewer.
 *
 * The board is watched from a fixed seat and both fighters face along the
 * engagement row, which puts every face in pure profile — the one angle at
 * which none of the work above is visible. Splitting a clamped yaw across neck
 * and head is the standard cheat and it is a cheat; naming it here is cheaper
 * than pretending the camera found this angle on its own.
 */
export function applyHeadTurn(character, yawRad, pitchRad) {
  const y = Math.max(-0.62, Math.min(0.62, yawRad));
  character.groups.neck.rotation.y = y * 0.38;
  character.groups.head.rotation.y = y * 0.62;
  character.groups.head.rotation.x = pitchRad || 0;
}

/** Flat-shade: low-poly wants facets, and facets need unshared normals. */
function flat(geo) {
  const g = geo.index ? geo.toNonIndexed() : geo;
  g.computeVertexNormals();
  return g;
}

/** A mesh plus its inverted-hull silhouette, as one group. */
function inked(geometry, colorHex, inkHex, widthPx = 2.2, opts = {}) {
  const group = new THREE.Group();
  const g = flat(geometry);
  group.add(new THREE.Mesh(g, celMaterial(colorHex, opts)));
  group.add(new THREE.Mesh(g, outlineMaterial(inkHex, widthPx)));
  return group;
}

// ---------------------------------------------------------------------------
// Geometry primitives
// ---------------------------------------------------------------------------

/** A closed tapered segment along +Y, `len` tall, sitting on the origin. */
function limbGeo(rTop, rBottom, len, radial = 6) {
  const g = new THREE.CylinderGeometry(rTop, rBottom, len, radial, 1, false);
  g.translate(0, len / 2, 0);
  return g;
}

function boxGeo(w, h, d, y = 0, x = 0, z = 0) {
  const g = new THREE.BoxGeometry(w, h, d);
  g.translate(x, y, z);
  return g;
}

/** Flat-top hex prism: corners on the ±X axis, flats facing ±Z. */
function hexPrismGeo(radius, height) {
  const g = new THREE.CylinderGeometry(radius, radius, height, 6, 1, false);
  g.rotateY(Math.PI / 6);
  g.translate(0, height / 2, 0);
  return g;
}

/**
 * A blade swept along an arc.
 *
 * Straight (`curve = 0`) gives the 청류문 jian; a positive curve gives the
 * 흑사방 dao. The cross-section is a flattened diamond so the blade catches a
 * band edge down its length instead of reading as a flat card — with a hard
 * cel ramp that ridge line *is* the highlight.
 */
function bladeGeo({ len, width, thick, curve = 0, taper = 0.45, edgeBias = 0, segments = 7 }) {
  const pos = [];
  const rings = [];
  for (let i = 0; i <= segments; i += 1) {
    const u = i / segments;
    const w = (width * (1 - taper * u * u)) / 2;
    const t = (thick * (1 - 0.55 * u)) / 2;
    const ang = curve * u;
    const y = curve === 0 ? len * u : (len / curve) * Math.sin(ang);
    const z = curve === 0 ? 0 : (len / curve) * (1 - Math.cos(ang));
    const c = Math.cos(ang);
    const s = Math.sin(ang);
    // Local cross-section axes, rotated with the arc so the section stays
    // perpendicular to the spine.
    const ez = (dz) => ({ y: y - s * dz, z: z + c * dz });
    const a = ez(w); // cutting edge
    const b = ez(-w); // spine
    const m = ez(-w * edgeBias); // ridge, offset toward the spine on a dao
    rings.push([
      { x: 0, ...a },
      { x: t, ...m },
      { x: 0, ...b },
      { x: -t, ...m },
    ]);
  }
  const quad = (p, q, r, s) => pos.push(p.x, p.y, p.z, q.x, q.y, q.z, r.x, r.y, r.z, p.x, p.y, p.z, r.x, r.y, r.z, s.x, s.y, s.z);
  for (let i = 0; i < segments; i += 1) {
    const A = rings[i];
    const B = rings[i + 1];
    for (let k = 0; k < 4; k += 1) {
      const k2 = (k + 1) % 4;
      // Ring-then-rise, not rise-then-ring. The other order winds the quads
      // backwards, `computeVertexNormals` points every blade normal inward,
      // the front faces get culled, and what survives is the BackSide outline
      // hull — a solid ink blade with a bright rim. It looked deliberate.
      quad(A[k], A[k2], B[k2], B[k]);
    }
  }
  const base = rings[0];
  quad(base[3], base[2], base[1], base[0]);
  const g = new THREE.BufferGeometry();
  g.setAttribute('position', new THREE.Float32BufferAttribute(pos, 3));
  g.computeVertexNormals();
  return g;
}

// ---------------------------------------------------------------------------
// Character
// ---------------------------------------------------------------------------

/**
 * How far each limb sits off the sagittal plane. The rig solves a side view;
 * without this the near and far limbs are coincident and a fighting pose reads
 * as a stick figure rather than as a body with depth.
 */
const LIMB_Z = {
  armNearUp: 4.6,
  armFarUp: -4.6,
  legNearThigh: 3.4,
  legFarThigh: -3.4,
};

const KIT = {
  ally: {
    hemLen: 26,
    hemTop: 9.6,
    hemBottom: 17.6,
    shoulder: 8.0,
    blade: { len: 41, width: 4.4, thick: 2.5, curve: 0, taper: 0.5, edgeBias: 0 },
  },
  enemy: {
    hemLen: 18,
    hemTop: 9.9,
    hemBottom: 16.4,
    shoulder: 8.4,
    blade: { len: 39, width: 5.8, thick: 3.0, curve: 0.52, taper: 0.34, edgeBias: 0.45 },
  },
};

export function buildCharacter3D(side) {
  const pal = PAL3D[side];
  const kit = KIT[side];
  const root = new THREE.Group();
  const groups = {};
  const face = createFaceSurface(side);

  const skin = {
    spine: () => torsoGroup(pal, kit, side),
    chest: () => chestGroup(pal, kit, side),
    neck: () => inked(limbGeo(2.1, 2.6, 4, 6), pal.under, pal.ink, 1.6),
    head: () => headGroup(side, pal, face),
    armNearUp: () => inked(limbGeo(2.7, 3.7, 15, 6), pal.robe, pal.ink),
    armNearFore: () => sleeveGroup(pal, side),
    armNearHand: () => handGroup(pal),
    armFarUp: () => inked(limbGeo(2.7, 3.7, 15, 6), pal.robe, pal.ink),
    armFarFore: () => sleeveGroup(pal, side),
    armFarHand: () => handGroup(pal),
    legNearThigh: () => inked(limbGeo(3.5, 4.7, 21, 6), pal.under, pal.ink),
    legNearShin: () => shinGroup(pal, side),
    legNearFoot: () => footGroup(pal),
    legFarThigh: () => inked(limbGeo(3.5, 4.7, 21, 6), pal.under, pal.ink),
    legFarShin: () => shinGroup(pal, side),
    legFarFoot: () => footGroup(pal),
  };

  for (const bone of BONES) {
    const g = new THREE.Group();
    groups[bone.name] = g;
    if (bone.parent === null) {
      root.add(g);
    } else {
      const parentBone = BONES[BONE_INDEX[bone.parent]];
      g.position.set(0, parentBone.len * bone.attach, LIMB_Z[bone.name] ?? 0);
      groups[bone.parent].add(g);
    }
    const make = skin[bone.name];
    if (make) g.add(make());
  }

  // The hem hangs from the hips as its own body so it does not inherit knee
  // rotation — the same reason a real 장삼 is not stitched to the shins. It is
  // deliberately short of the ankle: the first version of this prototype used
  // a 46-unit cone, swallowed both legs, and the figure read as a bell.
  const robe = new THREE.Group();
  const hemGeo = flat(hemGeometry(kit.hemTop, kit.hemBottom, kit.hemLen, side));
  robe.add(new THREE.Mesh(hemGeo, celMaterial(pal.robe, { rim: 0.62 })));
  robe.add(new THREE.Mesh(hemGeo, outlineMaterial(pal.ink, 2.4)));
  groups.hips.add(robe);

  groups.hips.add(sashGroup(pal, side));

  const sword = swordGroup(pal, kit);
  groups.armNearHand.add(sword);

  const tip = new THREE.Object3D();
  tip.position.set(0, kit.blade.len * 0.94, 0);
  sword.add(tip);
  const hilt = new THREE.Object3D();
  hilt.position.set(0, kit.blade.len * 0.68, 0);
  sword.add(hilt);

  return { side, root, groups, robe, sword, tip, hilt, face, depthSign: 1 };
}

/** A hem that flares and is scalloped at the bottom, not a smooth lampshade. */
function hemGeometry(rTop, rBottom, len, side) {
  const seg = 12;
  const g = new THREE.CylinderGeometry(rTop, rBottom, len, seg, 2, false);
  const p = g.attributes.position;
  const lift = side === 'ally' ? 1.6 : 2.8;
  for (let i = 0; i < p.count; i += 1) {
    const y = p.getY(i);
    if (y < -len / 2 + 0.01) {
      const a = Math.atan2(p.getZ(i), p.getX(i));
      p.setY(i, y + Math.abs(Math.sin(a * 3)) * lift);
    }
  }
  g.translate(0, -len / 2 + 0.5, 0);
  return g;
}

function torsoGroup(pal, kit, side) {
  const g = new THREE.Group();
  g.add(inked(limbGeo(6.4, 6.0, 13, 7), pal.robe, pal.ink));
  return g;
}

/** Chest, collar, and the piece that makes each school's build differ. */
function chestGroup(pal, kit, side) {
  const g = new THREE.Group();
  g.add(inked(limbGeo(5.0, 6.6, 12, 7), pal.robe, pal.ink));

  // 깃 — the crossed collar. Two thin slabs meeting in a V at the sternum;
  // reads at thumbnail size and is the clearest "this is a robe" signal.
  // 깃 — the crossed collar sits on the chest FRONT (+X) and the two halves
  // separate along the lateral axis (±Z), meeting in a V at the sternum.
  const collarHalf = (zSign) => {
    const b = new THREE.BoxGeometry(2.0, 12.0, 2.2);
    b.rotateX(-0.30 * zSign);
    b.translate(4.4, 5.8, 3.9 * zSign);
    return b;
  };
  const collarA = collarHalf(1);
  const collarB = collarHalf(-1);
  g.add(inked(mergeGeometries([flat(collarA), flat(collarB)]), pal.collar, pal.ink, 1.6));

  if (side === 'ally') {
    // A light shoulder yoke: narrow, square, upright.
    const yoke = new THREE.CylinderGeometry(kit.shoulder, kit.shoulder * 0.86, 5.4, 8, 1, false);
    yoke.scale(1, 1, 1.32);
    yoke.translate(0, 10.4, 0);
    g.add(inked(yoke, pal.under, pal.ink, 2.0));
  } else {
    // 흑사방: a heavy mantle over both shoulders. Broad, hunched, and it
    // reads as a different species of fighter in pure black.
    const mantle = new THREE.CylinderGeometry(kit.shoulder * 0.6, kit.shoulder * 1.12, 6.4, 9, 1, false);
    mantle.scale(1, 1, 1.16);
    mantle.translate(0, 9.4, 0);
    g.add(inked(mantle, pal.under, pal.ink, 2.2, { rim: 0.4 }));
    // 삿갓 slung on its cord between the shoulder blades.
    //
    // Worn properly it swallowed the face, and this fighter's face has to be
    // visible. Slung back is a real look, it keeps the silhouette cue that
    // says 흑사방 at a glance, and it puts a hard disc behind the shoulders
    // that the 청류문 outline has nothing like.
    const hat = new THREE.Group();
    const brim = new THREE.ConeGeometry(10.2, 4.4, 12, 1, false);
    brim.rotateZ(-Math.PI * 0.44);
    hat.add(inked(brim, pal.hair, pal.ink, 2.2, { rim: 0.18, shadowDim: 0.86, shadowMix: 0.08, litMix: 0.08 }));
    const hatRing = new THREE.TorusGeometry(9.7, 0.32, 4, 14);
    hatRing.rotateY(Math.PI / 2);
    hatRing.rotateZ(-Math.PI * 0.44 + Math.PI / 2);
    hat.add(inked(hatRing, pal.cordAccent, pal.ink, 1.1, { spec: 0.7 }));
    hat.position.set(-6.4, 4.0, 0);
    hat.rotation.z = -0.18;
    g.add(hat);
    const hatCord = boxGeo(0.7, 11.0, 0.7, 9.0, -3.4, 0);
    hatCord.rotateZ(-0.5);
    g.add(inked(hatCord, pal.cordAccent, pal.ink, 1.0, { spec: 0.4 }));

    const clasp = new THREE.CylinderGeometry(1.9, 1.9, 1.4, 6, 1, false);
    clasp.rotateZ(Math.PI / 2);
    clasp.translate(6.2, 11.2, 0);
    g.add(inked(clasp, pal.cordAccent, pal.ink, 1.4, { spec: 0.5 }));
  }
  return g;
}

/**
 * Hair, as chunky clumps with strands that escape the mass.
 *
 * This is the hardest thing this style has to do and the place it is most
 * likely to lose to a 2D renderer. The first attempt made every strand a thin
 * tapering spike launched along the scalp normal, and the capture was a head
 * with black straw radiating off it — a spider, not a fighter.
 *
 * What actually works is three layers:
 *   1. a **shell** that carries the mass and the parting,
 *   2. **clumps** — short, thick, blunt-tipped cards that hang off the shell
 *      and curl outward only at the very end,
 *   3. a handful of thin **flyaways** that break the outline.
 *
 * Each card is a cross of two quads. A single card vanishes edge-on, and hair
 * that disappears as the head turns is worse than no hair; the cross costs one
 * extra quad strip and never disappears.
 */
function strandGeometry(spec, seg = 6) {
  const { root, dir, len, drop, sweep, curl = 0, w0, w1, twist = 0 } = spec;
  const pos = [];
  const push = (a, b, c) => pos.push(a.x, a.y, a.z, b.x, b.y, b.z, c.x, c.y, c.z);

  const at = (t) => ({
    // Straight off the scalp, bending into gravity, with the outward curl
    // held back to t³ so it only shows in the last third of the length.
    x: root[0] + dir[0] * len * t + sweep[0] * len * t * t + curl * len * t * t * t,
    y: root[1] + dir[1] * len * t - drop * len * t * t + curl * len * t * t * t * 0.5,
    z: root[2] + dir[2] * len * t + sweep[2] * len * t * t,
  });

  for (let plane = 0; plane < 2; plane += 1) {
    const ang = twist + plane * Math.PI * 0.5;
    const ax = { x: Math.cos(ang), z: Math.sin(ang) };
    const rings = [];
    for (let i = 0; i <= seg; i += 1) {
      const t = i / seg;
      const c = at(t);
      // Widest a third of the way down, blunt at the tip. A card that tapers
      // linearly to a point is a spike; hair ends in a wedge.
      const shape = 1 - 0.78 * Math.pow(Math.max(0, t - 0.25) / 0.75, 1.5);
      const w = (w0 * shape + (w1 - w0) * 0.15 * t) * 0.5;
      rings.push([
        { x: c.x + ax.x * w, y: c.y, z: c.z + ax.z * w },
        { x: c.x - ax.x * w, y: c.y, z: c.z - ax.z * w },
      ]);
    }
    for (let i = 0; i < seg; i += 1) {
      const [a0, a1] = rings[i];
      const [b0, b1] = rings[i + 1];
      push(a0, b0, b1);
      push(a0, b1, a1);
    }
  }
  const g = new THREE.BufferGeometry();
  g.setAttribute('position', new THREE.Float32BufferAttribute(pos, 3));
  g.computeVertexNormals();
  return g;
}

/**
 * Place a clump by where it leaves the skull rather than by a raw position, so
 * retuning the head shape does not detach the hair from it.
 *
 * `fall` is the fraction of the launch direction that is simply *down*. It is
 * the parameter that was missing: at 0 the clumps fly off along the scalp
 * normal, which is the spider.
 */
function strandFrom({ az, el, len, back, drop, flare, curl, w0, w1, twist, fall = 0.72, radius = 5.2, centre = 4.2 }) {
  const ca = Math.cos(az);
  const sa = Math.sin(az);
  const ce = Math.cos(el);
  const se = Math.sin(el);
  const root = [ca * ce * radius * 0.9, centre + se * radius * 1.04, sa * ce * radius * 0.88];
  const n = [ca * ce, se, sa * ce];
  const dir = [
    n[0] * (1 - fall) * 0.5 - back,
    n[1] * (1 - fall) * 0.5 - fall,
    n[2] * (1 - fall) * 0.5 + flare * 0.4,
  ];
  const m = Math.hypot(dir[0], dir[1], dir[2]) || 1;
  return {
    root,
    dir: [dir[0] / m, dir[1] / m, dir[2] / m],
    len,
    drop,
    sweep: [-back * 0.4, 0, flare * 0.5],
    curl: curl || 0,
    w0,
    w1,
    twist: twist || 0,
  };
}

function hairMass(pal, specs, widthPx = 1.4) {
  const parts = specs.map((sp) => strandGeometry(strandFrom(sp)));
  const merged = mergeGeometries(parts);
  const g = new THREE.Group();
  // Near-black cards are almost all grazing angle, so bounce and rim take them
  // over completely unless both are cut right back — an earlier capture had
  // pale blue chopsticks sticking out of a head. The lit band is allowed a
  // little more range here so a clump reads as a volume, not a hole.
  g.add(
    new THREE.Mesh(
      merged,
      celMaterial(pal.hair, { rim: 0.12, shadowMix: 0.07, shadowDim: 0.68, litMix: 0.09, litGain: 1.5, edgeHi: 0.3 }),
    ),
  );
  g.add(new THREE.Mesh(merged, outlineMaterial(pal.ink, widthPx)));
  return g;
}

/**
 * The shell that carries the mass, so the clumps have something to hang off.
 *
 * A single sphere cap cannot do this job: its rim is a level circle, so a rim
 * low enough to reach the nape at the back is also low enough to cover the
 * eyes at the front. The capture of that version was a fighter wearing a
 * swimming cap pulled down over her face. So the shell is two pieces — a crown
 * that stops above the brow, and a nape panel spanning only the back half.
 */
function hairShell(pal, { back = -0.3, drop = 4.2, crown = 0.46, nape = 0.72, scale = [0.94, 1.08, 0.93], radius = 5.9 }) {
  const opts = { rim: 0.12, shadowMix: 0.07, shadowDim: 0.7, litMix: 0.09, litGain: 1.4 };
  const g = new THREE.Group();

  const top = new THREE.SphereGeometry(radius, 16, 8, 0, Math.PI * 2, 0, Math.PI * crown);
  top.scale(scale[0], scale[1], scale[2]);
  top.translate(back, drop, 0);
  g.add(inked(top, pal.hair, pal.ink, 1.6, opts));

  // Back half only: phi = 2π is −X, so a π-wide span centred there is the nape.
  const rear = new THREE.SphereGeometry(radius, 12, 7, Math.PI * 1.5, Math.PI, Math.PI * (crown - 0.08), Math.PI * (nape - crown + 0.08));
  rear.scale(scale[0], scale[1], scale[2]);
  rear.translate(back, drop, 0);
  g.add(inked(rear, pal.hair, pal.ink, 1.6, opts));

  return g;
}

/**
 * 산발 — unbound, loose, a little wild.
 *
 * A shell that reaches the nape, nine clumps hanging past the jaw with an
 * outward curl at the tips, and five thin flyaways crossing the outline.
 */
function looseHair(pal) {
  const rand = mulberry(0x5eed1a17);
  const g = new THREE.Group();
  g.add(hairShell(pal, { back: -0.28, drop: 4.15, crown: 0.44, nape: 0.78, scale: [0.95, 1.12, 0.95], radius: 5.95 }));

  const specs = [];
  // Back and sides: thick, blunt, hanging to below the jaw.
  const clumps = [
    { az: 1.52, el: 0.06 }, { az: 1.95, el: 0.0 }, { az: 2.38, el: 0.08 },
    { az: 2.8, el: 0.18 }, { az: Math.PI, el: 0.26 },
  ];
  for (const c of clumps) {
    for (const sgn of [1, -1]) {
      if (sgn === -1 && Math.abs(c.az - Math.PI) < 0.01) continue;
      const j = rand();
      specs.push({
        az: c.az * sgn,
        el: c.el + (rand() - 0.5) * 0.12,
        len: 14 + j * 6,
        back: 0.08 + rand() * 0.14,
        drop: 0.3 + rand() * 0.18,
        flare: (0.08 + rand() * 0.16) * sgn,
        curl: (0.18 + rand() * 0.2) * sgn,
        w0: 1.9 + rand() * 0.9,
        w1: 1.2,
        twist: rand() * Math.PI,
        fall: 0.7,
      });
    }
  }
  // Face-framing locks in front of the ears, shorter and narrower.
  specs.push(
    { az: 1.3, el: 0.28, len: 12, back: -0.04, drop: 0.28, flare: 0.1, curl: 0.1, w0: 1.5, w1: 0.9, twist: 0.4, fall: 0.84 },
    { az: -1.3, el: 0.32, len: 10.5, back: -0.02, drop: 0.3, flare: -0.1, curl: -0.09, w0: 1.35, w1: 0.8, twist: 1.1, fall: 0.84 },
  );
  // Flyaways: thin, longer, curling hard out of the mass.
  specs.push(
    { az: 2.45, el: 0.72, len: 11, back: 0.3, drop: 0.06, flare: 0.4, curl: 0.62, w0: 0.9, w1: 0.5, twist: 0.7, fall: 0.2 },
    { az: -2.15, el: 0.66, len: 8.5, back: 0.24, drop: 0.12, flare: -0.5, curl: -0.68, w0: 1.15, w1: 0.62, twist: 1.4, fall: 0.28 },
    { az: 2.95, el: 0.95, len: 8, back: 0.18, drop: 0.04, flare: 0.16, curl: 0.62, w0: 1.1, w1: 0.6, twist: 0.2, fall: 0.16 },
    { az: -2.75, el: 0.4, len: 10, back: 0.3, drop: 0.2, flare: -0.28, curl: -0.56, w0: 1.2, w1: 0.65, twist: 2.0, fall: 0.38 },
    { az: 1.95, el: 0.5, len: 8.5, back: 0.12, drop: 0.16, flare: 0.34, curl: 0.6, w0: 1.05, w1: 0.58, twist: 2.6, fall: 0.42 },
  );
  g.add(hairMass(pal, specs, 1.3));
  return g;
}

/**
 * The opposite: bound, high, and severe.
 *
 * A slicked shell with nothing loose at the temples, a tie high on the crown,
 * and one heavy tail. In pure black this silhouette is a smooth skull with a
 * single spike off the back — exactly the read 산발 does not have.
 */
function boundHair(pal) {
  const g = new THREE.Group();
  g.add(hairShell(pal, { back: -0.2, drop: 4.2, crown: 0.42, nape: 0.68, scale: [0.93, 1.04, 0.92], radius: 5.76 }));

  const tie = new THREE.CylinderGeometry(1.6, 1.8, 1.6, 8, 1, false);
  tie.rotateZ(0.6);
  tie.translate(-4.2, 8.2, 0);
  g.add(inked(tie, pal.cordAccent, pal.ink, 1.2, { spec: 0.6 }));

  const rand = mulberry(0x13579bdf);
  const specs = [];
  for (let i = 0; i < 5; i += 1) {
    specs.push({
      az: Math.PI + (rand() - 0.5) * 0.3,
      el: 0.7 + (rand() - 0.5) * 0.12,
      len: 23 + rand() * 6,
      back: 0.5,
      drop: 0.42 + rand() * 0.18,
      flare: (rand() - 0.5) * 0.2,
      curl: (rand() - 0.5) * 0.14,
      w0: 1.9,
      w1: 1.15,
      twist: rand() * Math.PI,
      fall: 0.34,
      radius: 4.5,
    });
  }
  // Two short escapees only — a perfectly sealed head reads as plastic.
  specs.push(
    { az: 2.72, el: 0.42, len: 7.5, back: 0.36, drop: 0.1, flare: 0.3, curl: 0.44, w0: 1.0, w1: 0.55, twist: 0.9, fall: 0.34 },
    { az: -2.72, el: 0.5, len: 6.8, back: 0.34, drop: 0.12, flare: -0.28, curl: -0.42, w0: 0.95, w1: 0.5, twist: 1.7, fall: 0.36 },
  );
  g.add(hairMass(pal, specs, 1.2));
  return g;
}

/**
 * The head.
 *
 * The skull is the only mesh in the character that keeps smooth normals. Every
 * other part is flat-shaded because low-poly wants facets; a face wants a
 * continuous surface for the decal to sit on, and faceting a head that carries
 * a painted eye puts a hard edge through the iris.
 *
 * The no-face rule this prototype started under covers the protagonist. These
 * two are NPC combatants, so they get faces — and the faces are driven from
 * `expressionFor`, not placed per image.
 */
function headGroup(side, pal, face) {
  const g = new THREE.Group();

  const skull = new THREE.SphereGeometry(HEAD_RADIUS, 22, 16);
  skull.scale(0.9, HEAD_ASPECT.y, HEAD_ASPECT.z);
  skull.translate(0, 4.2, 0);
  g.add(new THREE.Mesh(skull, skinMaterial(pal.skin, face.texture)));
  // Hairline-thin ink on the head, and only enough to seat it against the
  // background. The face is painted with soft edges and low contrast; a full
  // 2px keyline around it fights the drawing exactly the way a cel band does.
  // Cloth and steel keep the full-weight hull.
  g.add(new THREE.Mesh(skull, outlineMaterial(pal.headInk, 0.8)));

  // Ears use the *flat* skin ramp too. Built with `inked` they picked up the
  // hard three-band cel material and read as grey lumps stuck to a lit face.
  for (const zs of [-1, 1]) {
    const ear = flat(
      (() => {
        const e = new THREE.SphereGeometry(0.9, 6, 5);
        e.scale(0.45, 1.35, 0.7);
        e.translate(-1.2, 3.8, 4.15 * zs);
        return e;
      })(),
    );
    g.add(new THREE.Mesh(ear, skinMaterial(pal.skin, blankDecal())));
  }

  if (side === 'ally') {
    g.add(looseHair(pal));
  } else {
    g.add(boundHair(pal));
  }
  return g;
}

/**
 * Wide sleeve over the forearm.
 *
 * The forearm inside it is under-robe coloured, not skin: an earlier pass made
 * it skin-toned and every capture read the sleeve as a bare arm with a cuff
 * floating around it.
 */
function sleeveGroup(pal, side) {
  const g = new THREE.Group();
  g.add(inked(limbGeo(3.0, 3.7, 14, 6), pal.robe, pal.ink, 2));
  const flare = side === 'ally' ? 8.0 : 6.4;
  const cuff = new THREE.CylinderGeometry(flare, 4.2, 12.6, 9, 1, false);
  cuff.translate(0, 6.6, 0);
  g.add(inked(cuff, pal.robe, pal.ink, 2.2, { rim: 0.6 }));
  const band = new THREE.CylinderGeometry(flare * 1.02, flare * 0.98, 1.8, 9, 1, false);
  band.translate(0, 12.2, 0);
  g.add(inked(band, pal.trim, pal.ink, 1.5));
  return g;
}

function handGroup(pal) {
  const g = new THREE.Group();
  const hand = new THREE.SphereGeometry(2.4, 7, 5);
  hand.scale(1, 1.2, 0.86);
  hand.translate(0, 2, 0);
  g.add(inked(hand, pal.skin, pal.ink, 1.8));
  return g;
}

function shinGroup(pal, side) {
  const g = new THREE.Group();
  g.add(inked(limbGeo(2.7, 3.6, 20, 6), pal.under, pal.ink));
  if (side === 'enemy') {
    // Leg wraps — the short-hemmed gang fighter shows leg, so the leg needs
    // its own reading rather than a bare tube.
    for (let i = 0; i < 3; i += 1) {
      const w = new THREE.CylinderGeometry(3.45 + i * 0.22, 3.62 + i * 0.22, 1.15, 7, 1, false);
      w.translate(0, 4.2 + i * 4.2, 0);
      g.add(inked(w, i === 1 ? pal.cordAccent : pal.trim, pal.ink, 1.3, { spec: i === 1 ? 0.6 : 0 }));
    }
  } else {
    const w = new THREE.CylinderGeometry(4.1, 4.3, 3.0, 7, 1, false);
    w.translate(0, 2.4, 0);
    g.add(inked(w, pal.sash, pal.ink, 1.4));
  }
  return g;
}

function footGroup(pal) {
  const g = new THREE.Group();
  const boot = boxGeo(4.6, 9.6, 5.6, 3.6, -0.7, 0);
  g.add(inked(boot, pal.ink, pal.ink, 1.8, { shadowDim: 0.82 }));
  return g;
}

function sashGroup(pal, side) {
  const g = new THREE.Group();
  const belt = new THREE.CylinderGeometry(7.6, 8.2, 5.6, 10, 1, false);
  belt.translate(0, -1.4, 0);
  g.add(inked(belt, pal.sash, pal.ink, 1.8));
  const knot = new THREE.SphereGeometry(2.8, 7, 5);
  knot.scale(1.1, 0.9, 1);
  knot.translate(8.4, -1.6, 0);
  g.add(inked(knot, pal.sash, pal.ink, 1.6));
  const len = side === 'ally' ? 22 : 13;
  const tail = boxGeo(1.2, len, 2.4, -len / 2 - 2, 8.6, 0);
  g.add(inked(tail, pal.sash, pal.ink, 1.5));
  return g;
}

function swordGroup(pal, kit) {
  const g = new THREE.Group();
  g.rotation.y = Math.PI / 2;
  const blade = bladeGeo(kit.blade);
  blade.translate(0, 6.2, 0);
  g.add(inked(blade, pal.steel, pal.steelInk, 2.8, { spec: 0.5, specSharp: 44, specCut: 0.66, rim: 0.55, shadowDim: 0.62, litGain: 1.06, litMix: 0.07 }));

  const guard =
    kit.blade.curve === 0
      ? boxGeo(2.4, 2.0, 11.5, 5.2)
      : (() => {
          const d = new THREE.CylinderGeometry(5.4, 5.4, 1.8, 8, 1, false);
          d.scale(1, 1, 0.62);
          d.translate(0, 5.4, 0);
          return d;
        })();
  g.add(inked(guard, pal.cordAccent, pal.ink, 1.6, { spec: 0.5 }));

  const grip = new THREE.CylinderGeometry(1.5, 1.7, 9, 7, 1, false);
  grip.translate(0, 0.4, 0);
  g.add(inked(grip, pal.ink, pal.ink, 1.6, { shadowDim: 0.8 }));

  if (kit.blade.curve === 0) {
    const pommel = new THREE.SphereGeometry(1.9, 7, 5);
    pommel.translate(0, -4.2, 0);
    g.add(inked(pommel, pal.cordAccent, pal.ink, 1.4, { spec: 0.5 }));
    const tassel = boxGeo(1.4, 11, 1.4, -10.5, 0, 0);
    g.add(inked(tassel, pal.trim, pal.ink, 1.4));
  } else {
    const ring = new THREE.TorusGeometry(2.6, 0.7, 4, 8);
    ring.rotateY(Math.PI / 2);
    ring.translate(0, -5.4, 0);
    g.add(inked(ring, pal.cordAccent, pal.ink, 1.4, { spec: 0.5 }));
  }
  return g;
}

// ---------------------------------------------------------------------------
// Pose application
// ---------------------------------------------------------------------------

/**
 * `rig.js` measures a bone angle from +Y toward +X; a Three rotation about Z
 * by `-angle` does exactly that, so no conversion is needed and both styles
 * consume identical numbers.
 */
export function applyPose3D(character, pose, signals) {
  for (const bone of BONES) {
    const g = character.groups[bone.name];
    const local = bone.bind + pose.a[BONE_INDEX[bone.name]];
    g.rotation.z = -local * RAD;
  }
  character.groups.hips.position.set(pose.hipX, HIP_HEIGHT + pose.hipY, 0);
  character.groups.hips.rotation.z = -pose.roll * RAD;
  if (signals && character.root.userData.placed !== true) {
    character.root.rotation.y = signals.facing >= 0 ? 0 : Math.PI;
  }
  character.robe.rotation.z = (-pose.hipX * 0.6 - pose.roll * 0.4) * RAD;
}

export function poseCharacter3D(character, pose, signals) {
  solveSkeleton(pose);
  applyPose3D(character, pose, signals);
}

/** Flip which limb pair sits toward the camera when a fighter faces away. */
export function setDepthSign(character, sign) {
  if (character.depthSign === sign) return;
  character.depthSign = sign;
  for (const [name, z] of Object.entries(LIMB_Z)) {
    character.groups[name].position.z = z * sign;
  }
}

// ---------------------------------------------------------------------------
// Hex board
// ---------------------------------------------------------------------------

/** Flat-top axial → world, matching production's `1.5*q, sqrt(3)*(r + q/2)`. */
export function axialToWorld(q, r, size) {
  return { x: 1.5 * size * q, z: SQRT3 * size * (r + q / 2) };
}

export function hexDistanceAxial(a, b) {
  const dq = a.q - b.q;
  const dr = a.r - b.r;
  return (Math.abs(dq) + Math.abs(dq + dr) + Math.abs(dr)) / 2;
}

function mulberry(seed) {
  let s = seed >>> 0;
  return () => {
    s = (s + 0x6d2b79f5) >>> 0;
    let t = s;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

/**
 * The board.
 *
 * Tiles are merged into one geometry per class, so the whole arena is four
 * draw pairs rather than one per cell. The classes are not decoration: `path`
 * is exactly the row the fixture's two fighters occupy across all eight ticks,
 * and `ally`/`enemy` are the cells they stand on this tick — the board states
 * what the data says, and nothing more.
 */
export function buildBoard3D({ size, center, radius, occupied, path, seed = 0x9e3779b9 }) {
  const rand = mulberry(seed);
  const group = new THREE.Group();
  const buckets = { tile: [], tilePath: [], allyCell: [], enemyCell: [], tileEdge: [] };
  const gap = size * 0.055;
  const tops = new Map();

  for (let q = center.q - radius - 1; q <= center.q + radius + 1; q += 1) {
    for (let r = center.r - radius - 1; r <= center.r + radius + 1; r += 1) {
      const d = hexDistanceAxial({ q, r }, center);
      if (d > radius) continue;
      const key = `${q},${r}`;
      const edge = d >= radius - 0.5;
      const h = edge ? 4.0 : 7.0 + rand() * 1.3;
      const geo = hexPrismGeo(size - gap, h);
      const { x, z } = axialToWorld(q, r, size);
      geo.translate(x, 0, z);
      const bucket = occupied.get(key) ? (occupied.get(key) === 'ally' ? 'allyCell' : 'enemyCell') : edge ? 'tileEdge' : path.has(key) ? 'tilePath' : 'tile';
      buckets[bucket].push(flat(geo));
      tops.set(key, { x, z, y: h });
    }
  }

  for (const [name, list] of Object.entries(buckets)) {
    if (!list.length) continue;
    const merged = mergeGeometries(list);
    const hex = PAL3D.world[name];
    group.add(new THREE.Mesh(merged, celMaterial(hex, { rim: 0.14, litMix: 0.05, litGain: 1.0, edgeHi: 0.5, shadowDim: 0.66, shadowMix: 0.5 })));
    group.add(new THREE.Mesh(merged, outlineMaterial(PAL3D.world.tileInk, name === 'tileEdge' ? 1.6 : 2.2)));
  }

  // A plinth under the whole arena so the board sits on something instead of
  // floating in the void.
  const span = size * (radius + 1.35) * 1.72;
  const plinth = new THREE.CylinderGeometry(span, span * 0.9, 22, 6, 1, false);
  plinth.rotateY(Math.PI / 6);
  const c = axialToWorld(center.q, center.r, size);
  plinth.translate(c.x, -11.6, c.z);
  group.add(new THREE.Mesh(flat(plinth), celMaterial(PAL3D.world.plinth, { rim: 0.14, litMix: 0.06, shadowDim: 0.72 })));
  group.add(new THREE.Mesh(flat(plinth), outlineMaterial(PAL3D.world.tileInk, 2.4)));

  return { group, tops, size };
}

/**
 * A contact shadow, drawn rather than rendered.
 *
 * Two hard rings, no penumbra. A shadow map here would put a soft gradient
 * under a figure whose every other edge is a hard step, and the whole frame
 * would stop reading as illustration.
 */
export function buildContactShadow(radius) {
  const g = new THREE.Group();
  for (const [k, alpha] of [
    [1.0, 0.22],
    [0.55, 0.18],
  ]) {
    const disc = new THREE.CircleGeometry(radius * k, 14);
    disc.rotateX(-Math.PI / 2);
    const m = new THREE.Mesh(
      disc,
      new THREE.MeshBasicMaterial({
        color: lin(PAL3D.world.shadow),
        transparent: true,
        opacity: alpha,
        depthWrite: false,
      }),
    );
    m.renderOrder = 2;
    g.add(m);
  }
  return g;
}

// ---------------------------------------------------------------------------
// Blade trail
// ---------------------------------------------------------------------------

/**
 * The afterimage of the cut, sampled from the rig itself.
 *
 * The character is re-posed at a handful of earlier phases and the sword's
 * hilt/tip positions are recorded; the ribbon is the surface those two points
 * sweep. It cannot drift from the pose because it *is* the pose, evaluated a
 * few frames back — and the opacity is quantised into three steps so it reads
 * as drawn smear rather than as a particle glow.
 */
export function buildBladeTrail(character, signals, phase, span = 0.035, samples = 4) {
  const pts = [];
  const scratchPose = { a: null };
  for (let i = 0; i < samples; i += 1) {
    const p = phase - span * (1 - i / (samples - 1));
    if (p < 0) {
      pts.push(null);
      continue;
    }
    const pose = poseFor(signals, p);
    applyPose3D(character, pose, null);
    character.root.updateMatrixWorld(true);
    pts.push({
      a: character.hilt.getWorldPosition(new THREE.Vector3()),
      b: character.tip.getWorldPosition(new THREE.Vector3()),
    });
  }
  void scratchPose;

  const pos = [];
  const fade = [];
  for (let i = 0; i < samples - 1; i += 1) {
    const A = pts[i];
    const B = pts[i + 1];
    if (!A || !B) continue;
    const f0 = i / (samples - 1);
    const f1 = (i + 1) / (samples - 1);
    pos.push(A.a.x, A.a.y, A.a.z, A.b.x, A.b.y, A.b.z, B.b.x, B.b.y, B.b.z);
    fade.push(f0, f0, f1);
    pos.push(A.a.x, A.a.y, A.a.z, B.b.x, B.b.y, B.b.z, B.a.x, B.a.y, B.a.z);
    fade.push(f0, f1, f1);
  }
  if (!pos.length) return null;

  const geo = new THREE.BufferGeometry();
  geo.setAttribute('position', new THREE.Float32BufferAttribute(pos, 3));
  geo.setAttribute('aFade', new THREE.Float32BufferAttribute(fade, 1));
  const mesh = new THREE.Mesh(
    geo,
    new THREE.ShaderMaterial({
      side: THREE.DoubleSide,
      transparent: true,
      depthWrite: false,
      uniforms: { uColor: { value: lin(PAL3D[character.side].trail) } },
      vertexShader: /* glsl */ `
        attribute float aFade;
        varying float vFade;
        void main() {
          vFade = aFade;
          gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
        }
      `,
      fragmentShader:
        SRGB_GLSL +
        /* glsl */ `
        uniform vec3 uColor;
        varying float vFade;
        void main() {
          // Three steps, not a ramp: a drawn smear, not a particle glow.
          float a = floor(clamp(vFade, 0.0, 0.999) * 3.0) / 3.0;
          if (a <= 0.001) discard;
          gl_FragColor = vec4(toSRGB(uColor), a * 0.28);
        }
      `,
    }),
  );
  mesh.renderOrder = 3;
  return mesh;
}

// ---------------------------------------------------------------------------
// Impact
// ---------------------------------------------------------------------------

/**
 * One hard chevron at the point of contact, not a shower of debris.
 *
 * The 2D pass of this prototype put nine sparks on the torso and every capture
 * read them as fragments glued to the character. A single graphic mark, sized
 * to the figure, says "struck" without competing with the pose.
 */
export function buildImpactMark(radius, colorHex) {
  const g = new THREE.Group();
  const mat = new THREE.MeshBasicMaterial({
    color: lin(colorHex),
    transparent: true,
    opacity: 0.92,
    side: THREE.DoubleSide,
    depthWrite: false,
  });
  // A crescent whose thickness tapers to nothing at both ends — the shape a
  // blade leaves. Concentric constant-width arcs, which is what this drew
  // first, are the shape a wifi icon leaves; the capture was unambiguous.
  const stroke = (r, halfWidth, arc, steps = 14) => {
    const shape = new THREE.Shape();
    const at = (i, sign) => {
      const t = i / steps;
      const a = -arc / 2 + arc * t;
      const w = halfWidth * Math.pow(Math.sin(Math.PI * t), 0.65) * sign;
      return [Math.cos(a) * (r + w), Math.sin(a) * (r + w)];
    };
    shape.moveTo(...at(0, 1));
    for (let i = 1; i <= steps; i += 1) shape.lineTo(...at(i, 1));
    for (let i = steps; i >= 0; i -= 1) shape.lineTo(...at(i, -1));
    shape.closePath();
    const m = new THREE.Mesh(new THREE.ShapeGeometry(shape), mat);
    m.renderOrder = 4;
    return m;
  };
  g.add(stroke(radius, radius * 0.2, 1.9));
  const minor = stroke(radius * 1.5, radius * 0.075, 1.1);
  minor.rotation.z = 0.24;
  g.add(minor);
  return g;
}

// ---------------------------------------------------------------------------
// Environment
// ---------------------------------------------------------------------------

function ridgeGeometry(width, height, seed, teeth) {
  const rand = mulberry(seed);
  const pts = [];
  const n = teeth * 2;
  for (let i = 0; i <= n; i += 1) {
    const x = -width / 2 + (width * i) / n;
    const peak = i % 2 === 1;
    const y = peak ? height * (0.66 + rand() * 0.34) : height * (0.26 + rand() * 0.16);
    pts.push(new THREE.Vector2(x, y));
  }
  const shape = new THREE.Shape();
  shape.moveTo(-width / 2, -height);
  for (const p of pts) shape.lineTo(p.x, p.y);
  shape.lineTo(width / 2, -height);
  shape.closePath();
  return new THREE.ShapeGeometry(shape);
}

/**
 * Backdrop: two ridge lines and a pale disc, all unlit flat fills.
 *
 * They are `MeshBasicMaterial` on purpose — a lit backdrop competes with the
 * figures for the eye, and these exist only to give the arena a horizon and to
 * stop the board floating in a void.
 */
export function buildBackdrop3D({ center, size, seed = 0x1f2b3c }) {
  const g = new THREE.Group();
  const c = axialToWorld(center.q, center.r, size);
  const flatMat = (hex, opacity = 1) =>
    new THREE.MeshBasicMaterial({ color: lin(hex), transparent: opacity < 1, opacity, depthWrite: false });

  const disc = new THREE.CircleGeometry(size * 2.1, 28);
  const discMesh = new THREE.Mesh(disc, flatMat(PAL3D.world.disc, 0.5));
  discMesh.position.set(c.x - size * 3.0, size * 3.4, c.z - size * 15);
  discMesh.renderOrder = -6;
  g.add(discMesh);

  const far = new THREE.Mesh(ridgeGeometry(size * 17, size * 3.0, seed, 5), flatMat(PAL3D.world.ridgeFar));
  far.position.set(c.x, -size * 1.5, c.z - size * 14);
  far.renderOrder = -5;
  g.add(far);

  const near = new THREE.Mesh(ridgeGeometry(size * 13, size * 2.0, seed ^ 0x5bf03635, 4), flatMat(PAL3D.world.ridgeNear));
  near.position.set(c.x, -size * 1.9, c.z - size * 9);
  near.renderOrder = -4;
  g.add(near);

  return g;
}

/** Faction standards at the arena edge — scale reference and identity. */
export function buildBanner3D(side, height = 96) {
  const pal = PAL3D[side];
  const g = new THREE.Group();
  const pole = new THREE.CylinderGeometry(1.5, 2.0, height, 6, 1, false);
  pole.translate(0, height / 2, 0);
  g.add(inked(pole, PAL3D.world.poleWood, PAL3D.world.bannerInk, 2.0));

  const cloth = new THREE.PlaneGeometry(13, 46, 1, 5);
  const p = cloth.attributes.position;
  for (let i = 0; i < p.count; i += 1) {
    // A slack curve rather than a rigid card; a flag that is dead flat reads
    // as a UI element pasted into the scene.
    p.setZ(i, Math.sin((p.getY(i) / 46) * 3.2) * 2.4 + (p.getX(i) / 13) * 1.4);
  }
  cloth.rotateY(Math.PI / 2);
  cloth.translate(3.2, height - 30, 0);
  const clothGeo = flat(cloth);
  const bannerHex = side === 'ally' ? PAL3D.world.bannerAlly : PAL3D.world.bannerEnemy;
  g.add(new THREE.Mesh(clothGeo, celMaterial(bannerHex, { side: THREE.DoubleSide, rim: 0.5 })));

  const finial = new THREE.ConeGeometry(2.6, 7, 6, 1, false);
  finial.translate(0, height + 3, 0);
  g.add(inked(finial, pal.cordAccent, PAL3D.world.bannerInk, 1.6, { spec: 0.6 }));
  return g;
}

// ---------------------------------------------------------------------------
// Scene
// ---------------------------------------------------------------------------

/**
 * Fixed camera. Fixed because the spectator board never moves its viewpoint —
 * which is exactly the property that makes a 3D pipeline affordable here: the
 * framing can be authored once, like a stage, instead of surviving arbitrary
 * orbit.
 */
export function createBoardCamera({ width, height, target, yawDeg, pitchDeg, dist, fov = 26 }) {
  const camera = new THREE.PerspectiveCamera(fov, width / height, 1, 6000);
  const yaw = yawDeg * RAD;
  const pitch = pitchDeg * RAD;
  camera.position.set(
    target.x + Math.sin(yaw) * Math.cos(pitch) * dist,
    target.y + Math.sin(pitch) * dist,
    target.z + Math.cos(yaw) * Math.cos(pitch) * dist,
  );
  camera.lookAt(target.x, target.y, target.z);
  return camera;
}

/**
 * Legacy entry point kept so the original comparison sheet still renders. The
 * play screen builds its scene from the board helpers above instead.
 */
export function createScene3D(width, height) {
  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(32, width / height, 1, 2000);
  camera.position.set(142, 104, 252);
  camera.lookAt(0, 40, 0);
  tuneForCamera(camera, height * 2);

  const ground = new THREE.Mesh(
    new THREE.CircleGeometry(150, 40),
    new THREE.MeshBasicMaterial({ color: lin(PAL3D.world.plinth), transparent: true, opacity: 0.45 }),
  );
  ground.rotation.x = -Math.PI / 2;
  scene.add(ground);

  return { scene, camera };
}


/**
 * A camera framed on one character's head.
 *
 * The board camera is fixed and broadside, which is correct for reading a
 * duel and useless for reading a face. Rather than compromise the board shot,
 * the HUD gets its own render of the same head from three-quarter front — a
 * portrait cut-in, which is what the genre does anyway.
 */
export function createPortraitCamera(width, height, character, opts) {
  const o = opts || {};
  const dist = o.dist === undefined ? 30 : o.dist;
  const fov = o.fov === undefined ? 28 : o.fov;
  const yawOffset = (o.yawOffsetDeg === undefined ? 30 : o.yawOffsetDeg) * RAD;
  const pitch = (o.pitchDeg === undefined ? 8 : o.pitchDeg) * RAD;
  const lift = o.lift === undefined ? 1.5 : o.lift;

  character.root.updateMatrixWorld(true);
  const head = character.groups.head.getWorldPosition(new THREE.Vector3());
  head.y += lift;

  // The direction the character is facing, in world space, swung around by
  // `yawOffset` so the camera sits off the nose rather than on it.
  const yaw = character.root.rotation.y + character.groups.head.rotation.y + yawOffset;
  const dir = new THREE.Vector3(Math.cos(yaw), 0, -Math.sin(yaw)).normalize();

  const camera = new THREE.PerspectiveCamera(fov, width / height, 0.5, 400);
  camera.position.set(
    head.x + dir.x * Math.cos(pitch) * dist,
    head.y + Math.sin(pitch) * dist,
    head.z + dir.z * Math.cos(pitch) * dist,
  );
  camera.lookAt(head.x, head.y, head.z);
  return camera;
}

export { RAD as RAD3D };

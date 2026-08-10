/**
 * Style 2 — low-poly 3D character, cel-shaded, fixed camera (Three.js).
 *
 * This is the Ink Tide approach transplanted: geometry assembled from
 * primitives in code, quantised diffuse instead of PBR, and an inverted-hull
 * outline pushed along the normal at a constant screen width.
 *
 * It is fed by the **same** `rig.js` pose used by `rig2d.js`. Bone angles map
 * to `rotation.z` on a group hierarchy; the only extra degree of freedom 3D
 * gets for free is the Y rotation that turns the character to face its hex
 * neighbour.
 *
 * One honest simplification against Ink Tide's rider: parts are separate
 * meshes in a hierarchy rather than one merged, vertex-skinned geometry.
 * Ink Tide merged because its perf contract caps a rider at 4 draw calls;
 * this prototype renders one still figure, so the merge would buy nothing and
 * cost the clarity of the comparison. Shipping this style *would* need the
 * merge — that is real work this prototype does not price in.
 */

import * as THREE from 'three';
import { BONES, BONE_INDEX, HIP_HEIGHT, solveSkeleton } from './rig.js';

const RAD = Math.PI / 180;

export const PAL3D = {
  ally: {
    robe: 0x3d8f85,
    robeLit: 0x58b0a4,
    trim: 0x8fd0c4,
    sash: 0xd8c79c,
    ink: 0x08161a,
    skin: 0xe8c9a8,
    blade: 0xd3dde2,
  },
  enemy: {
    robe: 0x8e3441,
    robeLit: 0xb04a58,
    trim: 0xc8697a,
    sash: 0x31262b,
    ink: 0x140809,
    skin: 0xdcb493,
    blade: 0xd3dde2,
  },
};

/** Shared key-light direction. Inconsistent light direction across parts is
 *  the fastest way to break a flat-shaded look, so it lives here once. */
const LIGHT_DIR = new THREE.Vector3(-0.42, 0.72, 0.55).normalize();

/**
 * Cel material: three hard bands through NdotL plus a fresnel rim.
 *
 * The bands are *not* evenly spaced. A wide lit band with a narrow, late
 * shadow band reads as illustration; even spacing reads as a technical demo.
 * The shadow band also adds a cool bounce rather than simply darkening, so
 * the terminator shifts hue instead of just value.
 */
function celMaterial(colorHex) {
  const base = new THREE.Color(colorHex).convertSRGBToLinear();
  return new THREE.ShaderMaterial({
    uniforms: {
      uColor: { value: base },
      uLight: { value: LIGHT_DIR.clone() },
      uBounce: { value: new THREE.Color(0x3c6ea8).convertSRGBToLinear() },
    },
    vertexShader: /* glsl */ `
      varying vec3 vNormalV;
      varying vec3 vViewPos;
      void main() {
        vNormalV = normalize(normalMatrix * normal);
        vec4 mv = modelViewMatrix * vec4(position, 1.0);
        vViewPos = mv.xyz;
        gl_Position = projectionMatrix * mv;
      }
    `,
    fragmentShader: /* glsl */ `
      uniform vec3 uColor;
      uniform vec3 uLight;
      uniform vec3 uBounce;
      varying vec3 vNormalV;
      varying vec3 vViewPos;
      void main() {
        vec3 n = normalize(vNormalV);
        vec3 l = normalize((viewMatrix * vec4(uLight, 0.0)).xyz);
        float ndl = dot(n, l);

        float band;
        float bounce;
        if (ndl > 0.26)      { band = 1.05; bounce = 0.0; }
        else if (ndl > -0.06) { band = 0.78; bounce = 0.20; }
        else                 { band = 0.56; bounce = 0.38; }

        vec3 col = uColor * band + uBounce * bounce * 0.35;

        vec3 v = normalize(-vViewPos);
        float rim = pow(1.0 - max(dot(n, v), 0.0), 3.2);
        col += vec3(0.42, 0.58, 0.7) * rim * 0.34;

        gl_FragColor = vec4(col, 1.0);
      }
    `,
  });
}

/**
 * Inverted-hull outline at a constant screen-space width.
 *
 * Scaling the mesh uniformly is the tempting shortcut and it fails: a thin
 * limb gets a hairline and a thick torso gets a slab. Offsetting along the
 * normal in view space, scaled by depth, keeps the line the same number of
 * pixels wherever it lands.
 */
function outlineMaterial(colorHex, widthPx) {
  return new THREE.ShaderMaterial({
    uniforms: {
      uColor: { value: new THREE.Color(colorHex).convertSRGBToLinear() },
      uWidth: { value: widthPx },
    },
    side: THREE.BackSide,
    vertexShader: /* glsl */ `
      uniform float uWidth;
      void main() {
        vec3 n = normalize(normalMatrix * normal);
        vec4 mv = modelViewMatrix * vec4(position, 1.0);
        mv.xyz += n * uWidth * (-mv.z) * 0.0032;
        gl_Position = projectionMatrix * mv;
      }
    `,
    fragmentShader: /* glsl */ `
      uniform vec3 uColor;
      void main() { gl_FragColor = vec4(uColor, 1.0); }
    `,
  });
}

/** A mesh plus its outline hull, added as one group. */
function inkedMesh(geometry, colorHex, inkHex, widthPx = 2.4) {
  const group = new THREE.Group();
  group.add(new THREE.Mesh(geometry, celMaterial(colorHex)));
  group.add(new THREE.Mesh(geometry, outlineMaterial(inkHex, widthPx)));
  return group;
}

/** A tapered segment along +Y, `len` tall, sitting on the origin. */
function limbGeometry(rTop, rBottom, len, radial = 7) {
  const g = new THREE.CylinderGeometry(rTop, rBottom, len, radial, 1, false);
  g.translate(0, len / 2, 0);
  return g;
}

/**
 * Build the hierarchy. Every bone becomes a group positioned at its
 * attachment point on the parent and rotated by its bind angle; `applyPose3D`
 * then writes only the pose delta, so the rest pose is expressed once here.
 */
export function buildCharacter3D(side) {
  const pal = PAL3D[side];
  const root = new THREE.Group();
  const groups = {};

  const skin = {
    spine: () => inkedMesh(limbGeometry(6.6, 7.4, 13), pal.robe, pal.ink),
    chest: () => inkedMesh(limbGeometry(5.8, 7.2, 12), pal.robe, pal.ink),
    neck: () => inkedMesh(limbGeometry(2.4, 2.8, 4), pal.skin, pal.ink, 2),
    head: () => headGroup(side, pal),
    armNearUp: () => inkedMesh(limbGeometry(3.2, 4.2, 15), pal.robe, pal.ink),
    armNearFore: () => sleeveGroup(pal),
    armNearHand: () => handGroup(pal, true),
    armFarUp: () => inkedMesh(limbGeometry(3.2, 4.2, 15), pal.robe, pal.ink),
    armFarFore: () => sleeveGroup(pal),
    armFarHand: () => handGroup(pal, false),
    legNearThigh: () => inkedMesh(limbGeometry(4.2, 5.4, 21), pal.robe, pal.ink),
    legNearShin: () => inkedMesh(limbGeometry(3.2, 4.2, 20), pal.robe, pal.ink),
    legNearFoot: () => inkedMesh(limbGeometry(2.6, 3.4, 7), pal.ink, pal.ink, 2),
    legFarThigh: () => inkedMesh(limbGeometry(4.2, 5.4, 21), pal.robe, pal.ink),
    legFarShin: () => inkedMesh(limbGeometry(3.2, 4.2, 20), pal.robe, pal.ink),
    legFarFoot: () => inkedMesh(limbGeometry(2.6, 3.4, 7), pal.ink, pal.ink, 2),
  };

  for (const bone of BONES) {
    const g = new THREE.Group();
    groups[bone.name] = g;
    if (bone.parent === null) {
      root.add(g);
    } else {
      const parentBone = BONES[BONE_INDEX[bone.parent]];
      g.position.set(0, parentBone.len * bone.attach, 0);
      groups[bone.parent].add(g);
    }
    const make = skin[bone.name];
    if (make) g.add(make());
  }

  // The robe hangs from the hips as its own cone so it does not inherit leg
  // rotation — the same reason a real 장삼 is not attached to the knees.
  const robe = new THREE.Group();
  const robeGeo = new THREE.CylinderGeometry(10, 17.5, 30, 14, 1, true);
  robeGeo.translate(0, -15, 0);
  const robeMesh = new THREE.Mesh(robeGeo, celMaterial(pal.robe));
  robeMesh.material.side = THREE.DoubleSide;
  robe.add(robeMesh);
  robe.add(new THREE.Mesh(robeGeo, outlineMaterial(pal.ink, 2.4)));
  groups.hips.add(robe);

  const sash = inkedMesh(limbGeometry(8.6, 8.9, 5), pal.sash, pal.ink, 1.8);
  sash.position.y = -1;
  groups.hips.add(sash);

  const sword = swordGroup(pal);
  groups.armNearHand.add(sword);

  return { root, groups, robe, sword };
}

function headGroup(side, pal) {
  const g = new THREE.Group();
  const skull = new THREE.SphereGeometry(5.6, 14, 10);
  skull.scale(0.94, 1.08, 1);
  skull.translate(0, 4.6, 0);
  g.add(inkedMesh(skull, pal.skin, pal.ink, 2.2));

  if (side === 'ally') {
    const knot = new THREE.SphereGeometry(2.9, 10, 8);
    knot.translate(-1.4, 10.6, 0);
    g.add(inkedMesh(knot, pal.ink, pal.ink, 1.6));
    const band = new THREE.CylinderGeometry(5.8, 5.8, 2.2, 14, 1, true);
    band.translate(0, 6.6, 0);
    g.add(inkedMesh(band, pal.trim, pal.ink, 1.6));
  } else {
    const hood = new THREE.SphereGeometry(6.4, 14, 10, 0, Math.PI * 2, 0, Math.PI * 0.62);
    hood.scale(1, 1.12, 1.02);
    hood.translate(0, 4.4, 0);
    g.add(inkedMesh(hood, pal.ink, pal.ink, 2.2));
  }
  return g;
}

/** Wide sleeve over the forearm — the genre's clearest silhouette cue. */
function sleeveGroup(pal) {
  const g = new THREE.Group();
  g.add(inkedMesh(limbGeometry(2.6, 3.4, 14), pal.robe, pal.ink, 2));
  const cuff = new THREE.CylinderGeometry(7.6, 4.6, 9.5, 10, 1, true);
  cuff.translate(0, 4.4, 0);
  const sleeve = new THREE.Group();
  sleeve.add(new THREE.Mesh(cuff, celMaterial(pal.robeLit)));
  sleeve.children[0].material.side = THREE.DoubleSide;
  sleeve.add(new THREE.Mesh(cuff, outlineMaterial(pal.ink, 2.2)));
  g.add(sleeve);
  return g;
}

function handGroup(pal, near) {
  const g = new THREE.Group();
  const hand = new THREE.SphereGeometry(2.5, 8, 6);
  hand.translate(0, 2, 0);
  g.add(inkedMesh(hand, pal.skin, pal.ink, 1.8));
  return g;
}

function swordGroup(pal) {
  const g = new THREE.Group();
  const blade = new THREE.BoxGeometry(1.5, 46, 4.6);
  blade.translate(0, 27, 0);
  g.add(inkedMesh(blade, pal.blade, pal.ink, 2));
  const guard = new THREE.BoxGeometry(2.2, 1.8, 11);
  guard.translate(0, 4.2, 0);
  g.add(inkedMesh(guard, pal.ink, pal.ink, 1.6));
  const grip = new THREE.CylinderGeometry(1.5, 1.5, 8, 8);
  grip.translate(0, -1.6, 0);
  g.add(inkedMesh(grip, pal.ink, pal.ink, 1.6));
  return g;
}

/**
 * Write a solved pose into the hierarchy.
 *
 * `rig.js` measures a bone angle from +Y toward +X; a Three rotation about Z
 * by `-angle` does exactly that, so no other conversion is needed. The 2D
 * renderer and this one therefore consume the identical numbers.
 */
export function applyPose3D(character, pose, signals) {
  for (const bone of BONES) {
    const g = character.groups[bone.name];
    const local = bone.bind + pose.a[BONE_INDEX[bone.name]];
    g.rotation.z = -local * RAD;
  }
  character.groups.hips.position.set(pose.hipX, HIP_HEIGHT + pose.hipY, 0);
  character.groups.hips.rotation.z = -pose.roll * RAD;
  character.root.rotation.y = signals.facing >= 0 ? 0 : Math.PI;

  // The robe leans away from the hips' motion rather than simulating cloth —
  // a cone cannot carry the verlet hem the 2D renderer uses, and pretending
  // otherwise would overstate what this style gives for free.
  character.robe.rotation.z = (-pose.hipX * 0.6 - pose.roll * 0.4) * RAD;
}

/** Solve, then apply. Kept together so callers cannot apply a stale pose. */
export function poseCharacter3D(character, pose, signals) {
  solveSkeleton(pose);
  applyPose3D(character, pose, signals);
}

/**
 * A scene with two characters, a ground disc, and a fixed three-quarter
 * camera. Fixed because the spectator board never moves its viewpoint — which
 * is exactly the property that makes a 3D pipeline affordable here.
 */
export function createScene3D(width, height) {
  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(32, width / height, 1, 2000);
  camera.position.set(142, 104, 252);
  camera.lookAt(0, 40, 0);

  const ground = new THREE.Mesh(
    new THREE.CircleGeometry(150, 40),
    new THREE.MeshBasicMaterial({ color: 0x2b3542, transparent: true, opacity: 0.45 }),
  );
  ground.rotation.x = -Math.PI / 2;
  scene.add(ground);

  return { scene, camera };
}

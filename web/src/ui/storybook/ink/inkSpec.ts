export type InkElementKind =
  | 'ridge' | 'roofline' | 'gate' | 'pine' | 'bamboo' | 'road'
  | 'wall' | 'stairs' | 'desk' | 'lantern' | 'campfire' | 'banner'
  | 'scroll' | 'rain' | 'moon';

export interface InkElement {
  kind: InkElementKind;
  x: number;
  scale?: number;
  flip?: boolean;
}

export type InkPose =
  | 'stand' | 'walk' | 'confront' | 'kneel' | 'fallen' | 'bow'
  | 'duel-left' | 'duel-right' | 'sit' | 'reach';

export interface InkFigure {
  pose: InkPose;
  x: number;
  scale?: number;
  weapon?: 'sword' | 'club' | 'staff' | 'none';
}

export interface InkSceneSpec {
  horizon: number;
  mist: 0 | 1 | 2 | 3;
  far?: InkElement[];
  mid?: InkElement[];
  near?: InkElement[];
  figures?: InkFigure[];
  seal?: string;
  accent?: 'none' | 'seal-red' | 'gold';
  night?: boolean;
}

export function fnv1a(text: string): number {
  let hash = 0x811c9dc5;
  for (let index = 0; index < text.length; index += 1) {
    hash ^= text.charCodeAt(index);
    hash = Math.imul(hash, 0x01000193);
  }
  return hash >>> 0;
}

export function jitter(seed: number, index: number, range: number): number {
  const value = Math.imul(seed ^ Math.imul(index + 1, 0x9e3779b1), 0x45d9f3b) >>> 0;
  return (((value & 0xffff) / 0xffff) * 2 - 1) * range;
}

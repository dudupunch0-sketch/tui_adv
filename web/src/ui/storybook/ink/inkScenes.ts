import type { InkSceneSpec } from './inkSpec';

const scene = (overrides: Partial<InkSceneSpec>): InkSceneSpec => ({ horizon: 0.68, mist: 1, ...overrides });

export const inkScenes: Record<string, InkSceneSpec> = {
  wuxia_commute_rift: scene({ mist: 3, far: [{ kind: 'ridge', x: 0.22 }, { kind: 'ridge', x: 0.76 }], mid: [{ kind: 'road', x: 0.5 }], figures: [{ pose: 'stand', x: 0.5 }], seal: '裂' }),
  wuxia_heuksa_bang_first_fight: scene({ mid: [{ kind: 'roofline', x: 0.22 }, { kind: 'banner', x: 0.78 }], near: [{ kind: 'road', x: 0.5 }], figures: [{ pose: 'confront', x: 0.35, weapon: 'club' }, { pose: 'stand', x: 0.65 }], seal: '難' }),
  wuxia_cheonggi_record_first_fragment: scene({ mid: [{ kind: 'desk', x: 0.42 }, { kind: 'scroll', x: 0.5 }, { kind: 'lantern', x: 0.7 }], figures: [{ pose: 'sit', x: 0.4 }], seal: '記' }),
  wuxia_seo_harin_rescue: scene({ mist: 2, mid: [{ kind: 'wall', x: 0.25 }, { kind: 'roofline', x: 0.7 }], figures: [{ pose: 'kneel', x: 0.4 }, { pose: 'fallen', x: 0.62 }], seal: '救' }),
  wuxia_cheongryu_apprentice_entry: scene({ mist: 2, far: [{ kind: 'ridge', x: 0.2 }, { kind: 'pine', x: 0.8 }], mid: [{ kind: 'gate', x: 0.5 }, { kind: 'stairs', x: 0.5 }], figures: [{ pose: 'bow', x: 0.5 }], seal: '門' }),
  wuxia_cheongryu_chore_sparring: scene({ mid: [{ kind: 'wall', x: 0.5 }], figures: [{ pose: 'duel-left', x: 0.35, weapon: 'staff' }, { pose: 'duel-right', x: 0.65, weapon: 'staff' }], seal: '修' }),
  wuxia_cheongryu_raid_route_split: scene({ mist: 3, far: [{ kind: 'banner', x: 0.75 }], near: [{ kind: 'road', x: 0.35 }, { kind: 'road', x: 0.65, flip: true }], figures: [{ pose: 'stand', x: 0.4 }, { pose: 'stand', x: 0.6 }], seal: '岐' }),
  wuxia_baekdo_medicine_debt: scene({ mid: [{ kind: 'roofline', x: 0.55 }, { kind: 'lantern', x: 0.74 }], figures: [{ pose: 'kneel', x: 0.4 }, { pose: 'stand', x: 0.62 }], seal: '藥' }),
  wuxia_black_heaven_escape_price: scene({ night: true, mist: 2, mid: [{ kind: 'wall', x: 0.45 }, { kind: 'moon', x: 0.77 }], figures: [{ pose: 'walk', x: 0.42 }], seal: '逃' }),
  wuxia_mumyeong_first_sighting: scene({ far: [{ kind: 'ridge', x: 0.3 }, { kind: 'bamboo', x: 0.72 }], figures: [{ pose: 'stand', x: 0.73, scale: 0.62 }], seal: '影' }),
  wuxia_mumyeong_first_confrontation: scene({ mist: 2, near: [{ kind: 'road', x: 0.5 }], figures: [{ pose: 'confront', x: 0.35, weapon: 'sword' }, { pose: 'stand', x: 0.65 }], seal: '問' }),
  wuxia_mumyeong_copy_style_reveal: scene({ figures: [{ pose: 'duel-left', x: 0.35, weapon: 'sword' }, { pose: 'duel-right', x: 0.65, weapon: 'sword' }], seal: '倣' }),
  wuxia_mumyeong_midgame_reunion: scene({ mid: [{ kind: 'road', x: 0.5 }, { kind: 'pine', x: 0.76 }], figures: [{ pose: 'stand', x: 0.4 }, { pose: 'stand', x: 0.6 }], seal: '逢' }),
  wuxia_cheonggi_record_writing_sense: scene({ mid: [{ kind: 'desk', x: 0.45 }, { kind: 'scroll', x: 0.52 }, { kind: 'lantern', x: 0.7 }], seal: '書' }),
  wuxia_boss_first_appearance: scene({ far: [{ kind: 'wall', x: 0.55 }, { kind: 'stairs', x: 0.55 }], near: [{ kind: 'banner', x: 0.25 }], figures: [{ pose: 'stand', x: 0.58, scale: 0.68 }], seal: '王', accent: 'seal-red' }),
  wuxia_mumyeong_awakening: scene({ mist: 3, figures: [{ pose: 'stand', x: 0.5 }], seal: '覺', accent: 'seal-red' }),
  wuxia_sado_final_battle: scene({ mist: 2, far: [{ kind: 'ridge', x: 0.3 }], figures: [{ pose: 'duel-left', x: 0.35, weapon: 'sword' }, { pose: 'duel-right', x: 0.65, weapon: 'sword' }], seal: '決' }),
  wuxia_sado_battle_loss_route_bridge: scene({ night: true, figures: [{ pose: 'fallen', x: 0.4 }, { pose: 'stand', x: 0.67 }], seal: '敗' }),
  wuxia_black_serpent_aftermath: scene({ mid: [{ kind: 'roofline', x: 0.5 }, { kind: 'banner', x: 0.72 }], seal: '痕' }),
};

export function sceneForVisual(id: string, mode: string): InkSceneSpec | undefined {
  const normalized = id.startsWith('ending:') ? id.slice('ending:'.length) : id;
  if (inkScenes[id] ?? inkScenes[normalized]) return inkScenes[id] ?? inkScenes[normalized];
  if (id.startsWith('location:')) return scene({ far: [{ kind: 'ridge', x: 0.25 }, { kind: 'bamboo', x: 0.76 }], near: [{ kind: 'road', x: 0.5 }], figures: [{ pose: 'walk', x: 0.5 }], seal: '行' });
  if (mode === 'ending') return scene({ mid: [{ kind: 'desk', x: 0.45 }, { kind: 'scroll', x: 0.56 }], figures: [{ pose: 'stand', x: 0.7 }], seal: '結', accent: 'gold' });
  if (mode === 'movement') return scene({ near: [{ kind: 'road', x: 0.5 }], figures: [{ pose: 'walk', x: 0.5 }], seal: '行' });
  return undefined;
}

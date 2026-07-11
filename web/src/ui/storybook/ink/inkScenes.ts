import { fnv1a, type InkSceneSpec } from './inkSpec';

const scene = (overrides: Partial<InkSceneSpec>): InkSceneSpec => ({ mist: 1, ...overrides });
const duel = (mist: 0 | 1 | 2 | 3, accent: InkSceneSpec['accent'] = 'none'): InkSceneSpec => scene({
  mist,
  far: [{ kind: 'ridge', x: 0.3 }],
  figures: [{ pose: 'duel-left', x: 0.35, weapon: 'sword' }, { pose: 'duel-right', x: 0.65, weapon: 'sword' }],
  seal: '決',
  accent,
});
const recordScene = (seal = '結'): InkSceneSpec => scene({
  mid: [{ kind: 'desk', x: 0.45 }, { kind: 'scroll', x: 0.56 }],
  figures: [{ pose: 'stand', x: 0.7 }],
  seal,
  accent: 'gold',
});

export const inkScenes: Record<string, InkSceneSpec> = {
  wuxia_commute_rift: scene({ mist: 3, far: [{ kind: 'ridge', x: 0.22 }, { kind: 'ridge', x: 0.76 }], mid: [{ kind: 'road', x: 0.5 }], figures: [{ pose: 'stand', x: 0.5 }], seal: '裂' }),
  wuxia_heuksa_bang_first_fight: scene({ mid: [{ kind: 'roofline', x: 0.22 }, { kind: 'banner', x: 0.78 }], near: [{ kind: 'road', x: 0.5 }], figures: [{ pose: 'confront', x: 0.35, weapon: 'club' }, { pose: 'stand', x: 0.65 }], seal: '難' }),
  wuxia_cheonggi_record_first_fragment: scene({ mid: [{ kind: 'desk', x: 0.42 }, { kind: 'scroll', x: 0.5 }, { kind: 'lantern', x: 0.7 }], figures: [{ pose: 'sit', x: 0.4 }], seal: '記' }),
  wuxia_seo_harin_rescue: scene({ mist: 2, mid: [{ kind: 'wall', x: 0.25 }, { kind: 'roofline', x: 0.7 }], figures: [{ pose: 'kneel', x: 0.4 }, { pose: 'fallen', x: 0.62 }], seal: '救' }),
  wuxia_cheongryu_apprentice_entry: scene({ mist: 2, far: [{ kind: 'ridge', x: 0.2 }, { kind: 'pine', x: 0.8 }], mid: [{ kind: 'gate', x: 0.5 }, { kind: 'stairs', x: 0.5 }], figures: [{ pose: 'bow', x: 0.5 }], seal: '門' }),
  wuxia_cheongryu_chore_sparring: scene({ mid: [{ kind: 'wall', x: 0.5 }], figures: [{ pose: 'duel-left', x: 0.35, weapon: 'staff' }, { pose: 'duel-right', x: 0.65, weapon: 'staff' }], seal: '修' }),
  wuxia_cheongryu_raid_route_split: scene({ mist: 3, far: [{ kind: 'banner', x: 0.75 }], near: [{ kind: 'road', x: 0.35 }, { kind: 'road', x: 0.65, flip: true }], figures: [{ pose: 'stand', x: 0.4 }, { pose: 'stand', x: 0.6 }], seal: '岐' }),
  wuxia_cheongryu_raid_wounded_fallback: scene({ night: true, mist: 2, near: [{ kind: 'wall', x: 0.5 }], figures: [{ pose: 'walk', x: 0.4 }, { pose: 'fallen', x: 0.62 }], seal: '退' }),
  wuxia_baekdo_medicine_debt: scene({ mid: [{ kind: 'roofline', x: 0.55 }, { kind: 'lantern', x: 0.74 }], figures: [{ pose: 'kneel', x: 0.4 }, { pose: 'stand', x: 0.62 }], seal: '藥' }),
  wuxia_black_heaven_escape_price: scene({ night: true, mist: 2, mid: [{ kind: 'wall', x: 0.45 }, { kind: 'moon', x: 0.77 }], figures: [{ pose: 'walk', x: 0.42 }], seal: '逃' }),
  wuxia_heavenly_archive_previous_outsiders: scene({ mid: [{ kind: 'wall', x: 0.4 }, { kind: 'scroll', x: 0.54 }, { kind: 'scroll', x: 0.67 }, { kind: 'lantern', x: 0.8 }], figures: [{ pose: 'reach', x: 0.42 }], seal: '藏' }),
  wuxia_wounded_shelter_dawn_offers: scene({ far: [{ kind: 'ridge', x: 0.3 }], near: [{ kind: 'campfire', x: 0.5 }], figures: [{ pose: 'sit', x: 0.35 }, { pose: 'sit', x: 0.65 }], seal: '曉' }),
  wuxia_mumyeong_first_sighting: scene({ far: [{ kind: 'ridge', x: 0.3 }, { kind: 'bamboo', x: 0.72 }], figures: [{ pose: 'stand', x: 0.73, scale: 0.62 }], seal: '影' }),
  wuxia_mumyeong_first_confrontation: scene({ mist: 2, near: [{ kind: 'road', x: 0.5 }], figures: [{ pose: 'confront', x: 0.35, weapon: 'sword' }, { pose: 'stand', x: 0.65 }], seal: '問' }),
  wuxia_mumyeong_copy_style_reveal: scene({ figures: [{ pose: 'duel-left', x: 0.35, weapon: 'sword' }, { pose: 'duel-right', x: 0.65, weapon: 'sword' }], seal: '倣' }),
  wuxia_cheonoe_pyeonrin_first_reward: scene({ mid: [{ kind: 'desk', x: 0.45 }, { kind: 'scroll', x: 0.55 }], figures: [{ pose: 'reach', x: 0.42 }], seal: '賞', accent: 'gold' }),
  wuxia_cheonoe_pyeonrin_second_reward: scene({ mid: [{ kind: 'desk', x: 0.45 }, { kind: 'scroll', x: 0.55 }], figures: [{ pose: 'reach', x: 0.42 }], seal: '賞', accent: 'gold' }),
  wuxia_mumyeong_reads_orthodox_style: scene({ mid: [{ kind: 'wall', x: 0.5 }], figures: [{ pose: 'sit', x: 0.34 }, { pose: 'duel-right', x: 0.66, weapon: 'staff' }], seal: '讀' }),
  wuxia_mumyeong_midgame_reunion: scene({ mid: [{ kind: 'road', x: 0.5 }, { kind: 'pine', x: 0.76 }], figures: [{ pose: 'stand', x: 0.4 }, { pose: 'stand', x: 0.6 }], seal: '逢' }),
  wuxia_cheonggi_record_writing_sense: scene({ mid: [{ kind: 'desk', x: 0.45 }, { kind: 'scroll', x: 0.52 }, { kind: 'lantern', x: 0.7 }], seal: '書' }),
  wuxia_boss_first_appearance: scene({ far: [{ kind: 'wall', x: 0.55 }, { kind: 'stairs', x: 0.55 }], near: [{ kind: 'banner', x: 0.25 }], figures: [{ pose: 'stand', x: 0.58, scale: 0.68 }], seal: '王', accent: 'seal-red' }),
  wuxia_mumyeong_request_for_aid: scene({ night: true, near: [{ kind: 'campfire', x: 0.5 }], figures: [{ pose: 'stand', x: 0.37 }, { pose: 'kneel', x: 0.63 }], seal: '請' }),
  wuxia_mumyeong_awakening: scene({ mist: 3, figures: [{ pose: 'stand', x: 0.5 }], seal: '覺', accent: 'seal-red' }),
  wuxia_qingliu_attack_after_war: scene({ mist: 3, mid: [{ kind: 'gate', x: 0.5 }], figures: [{ pose: 'confront', x: 0.36, weapon: 'sword' }, { pose: 'confront', x: 0.64, weapon: 'sword' }], seal: '襲' }),
  wuxia_mumyeong_destroys_orthodox_sect: scene({ night: true, mist: 3, mid: [{ kind: 'roofline', x: 0.5 }], figures: [{ pose: 'fallen', x: 0.35 }, { pose: 'fallen', x: 0.57 }, { pose: 'stand', x: 0.73 }], seal: '滅' }),
  wuxia_boss_recruits_mumyeong: scene({ mid: [{ kind: 'banner', x: 0.5 }], figures: [{ pose: 'stand', x: 0.38, scale: 0.72 }, { pose: 'stand', x: 0.64 }], seal: '招', accent: 'seal-red' }),
  wuxia_mumyeong_departure_truth_summary: scene({ mist: 2, far: [{ kind: 'road', x: 0.5 }], figures: [{ pose: 'walk', x: 0.5, scale: 0.7 }], seal: '去' }),
  wuxia_seoharin_empty_place: scene({ mist: 2, mid: [{ kind: 'roofline', x: 0.46 }, { kind: 'desk', x: 0.66 }, { kind: 'scroll', x: 0.68 }], seal: '空' }),
  wuxia_seoharin_left_meal: scene({ mid: [{ kind: 'desk', x: 0.5 }, { kind: 'moon', x: 0.43 }, { kind: 'moon', x: 0.57 }], seal: '溫' }),
  wuxia_sado_final_battle: duel(1),
  wuxia_sado_final_battle_phase_1: duel(1),
  wuxia_sado_final_battle_phase_2: duel(2),
  wuxia_sado_final_battle_phase_3: duel(3, 'seal-red'),
  wuxia_sado_final_phase_1_price_tag: duel(1),
  wuxia_sado_final_phase_2_weakpoint_control: duel(2),
  wuxia_sado_final_phase_3_outside_calculation: duel(3, 'seal-red'),
  wuxia_sado_battle_loss_route_bridge: scene({ night: true, figures: [{ pose: 'fallen', x: 0.4 }, { pose: 'stand', x: 0.67 }], seal: '敗' }),
  wuxia_boss_resolution: recordScene(),
  wuxia_mumyeong_resolution: recordScene(),
  wuxia_seoharin_qingliu_resolution: recordScene(),
  wuxia_cheongirok_resolution: recordScene(),
  wuxia_returned_commute: scene({ far: [{ kind: 'ridge', x: 0.25 }, { kind: 'wall', x: 0.72 }], figures: [{ pose: 'stand', x: 0.5 }], seal: '歸' }),
  wuxia_qingliu_settlement: scene({ mist: 2, mid: [{ kind: 'roofline', x: 0.42 }, { kind: 'pine', x: 0.76 }], figures: [{ pose: 'sit', x: 0.38 }, { pose: 'sit', x: 0.62 }], seal: '定' }),
  wuxia_black_serpent_aftermath: scene({ mid: [{ kind: 'roofline', x: 0.5 }, { kind: 'banner', x: 0.72 }], seal: '痕' }),
  wuxia_preview: scene({ mid: [{ kind: 'road', x: 0.5 }], near: [{ kind: 'scroll', x: 0.25 }], figures: [{ pose: 'stand', x: 0.5 }], seal: '存' }),
};

export function sceneForVisual(id: string, mode: string): InkSceneSpec | undefined {
  const normalized = id.startsWith('ending:') ? id.slice('ending:'.length) : id;
  const direct = inkScenes[id] ?? inkScenes[normalized];
  if (direct) return direct;
  if (normalized.startsWith('wuxia_return_modern_commute_scene')) return inkScenes.wuxia_returned_commute;
  if (normalized.startsWith('wuxia_settlement_stay_scene')) return inkScenes.wuxia_qingliu_settlement;
  if (normalized === 'wuxia_preview_grounded') return inkScenes.wuxia_preview;
  if (id.startsWith('location:')) return locationScene(id);
  if (id.startsWith('storypack:')) return scene({ mist: 1, mid: [{ kind: 'desk', x: 0.5 }], seal: undefined });
  if (mode === 'ending') return recordScene();
  if (mode === 'movement') return locationScene(id);
  return undefined;
}

function locationScene(id: string): InkSceneSpec {
  const options = ['ridge', 'roofline', 'gate', 'bamboo'] as const;
  const seed = fnv1a(id);
  const first = options[seed % options.length];
  const second = options[(seed >>> 5) % options.length];
  return scene({ far: [{ kind: first, x: 0.25 }, { kind: second, x: 0.76 }], near: [{ kind: 'road', x: 0.5 }], figures: [{ pose: 'walk', x: 0.5 }], seal: '行' });
}

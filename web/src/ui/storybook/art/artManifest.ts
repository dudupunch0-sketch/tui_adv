export const ART_BASE_PATH = 'assets/art/';
/** visual_id → 파일명(확장자 포함). 등록된 장면만 이미지, 나머지는 SVG 폴백. */
export const artManifest: Record<string, string> = {
  'title_hero': 'title_hero.webp',
  'wuxia_commute_rift': 'wuxia_commute_rift.webp',
  'location:jianghu_roadside': 'location_jianghu_roadside.webp',
  'location:jianghu_market_street': 'location_jianghu_market_street.webp',
  'wuxia_heuksa_bang_first_fight': 'wuxia_heuksa_bang_first_fight.webp',
  'wuxia_cheonggi_record_first_fragment': 'wuxia_cheonggi_record_first_fragment.webp',
  'wuxia_seo_harin_rescue': 'wuxia_seo_harin_rescue.webp',
  'wuxia_cheongryu_apprentice_entry': 'wuxia_cheongryu_apprentice_entry.webp',
  'wuxia_mumyeong_first_confrontation': 'wuxia_mumyeong_first_confrontation.webp',
  'wuxia_boss_first_appearance': 'wuxia_boss_first_appearance.webp',
  'wuxia_sado_final_battle': 'wuxia_sado_final_battle.webp',
  'wuxia_return_modern_commute_scene_resolved': 'wuxia_return_modern_commute_scene_resolved.webp',
  'wuxia_settlement_stay_scene_resolved': 'wuxia_settlement_stay_scene_resolved.webp',
};

export function artAssetFor(visualId: string): string | undefined {
  const normalized = visualId.startsWith('ending:') ? visualId.slice('ending:'.length) : visualId;
  const file = artManifest[visualId] ?? artManifest[normalized];
  return file ? `${import.meta.env.BASE_URL}${ART_BASE_PATH}${file}` : undefined;
}

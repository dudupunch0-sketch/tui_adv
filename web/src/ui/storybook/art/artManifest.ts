export const ART_BASE_PATH = 'assets/art/';
/** visual_id → 파일명(확장자 포함). 등록된 장면만 이미지, 나머지는 SVG 폴백. */
export const artManifest: Record<string, string> = {
  'title_hero': 'title_hero.webp',
  'wuxia_commute_rift': 'wuxia_commute_rift.webp',
  'location:jianghu_roadside': 'location_jianghu_roadside.webp',
  'location:jianghu_market_street': 'location_jianghu_market_street.webp',
};

export function artAssetFor(visualId: string): string | undefined {
  const normalized = visualId.startsWith('ending:') ? visualId.slice('ending:'.length) : visualId;
  const file = artManifest[visualId] ?? artManifest[normalized];
  return file ? `${import.meta.env.BASE_URL}${ART_BASE_PATH}${file}` : undefined;
}

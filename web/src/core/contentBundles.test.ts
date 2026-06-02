import { describe, expect, it } from 'vitest';

import {
  DEFAULT_CONTENT_BUNDLE_JSON,
  DEFAULT_STORYPACK_ID,
  DEFAULT_STORYPACK_LABEL,
  STORYPACK_PREVIEW_OPTIONS,
  storypackPreviewById,
} from './contentBundles';

describe('runtime content bundle registry', () => {
  it('uses 이구학지 as the default Web runtime storypack', () => {
    const bundle = JSON.parse(DEFAULT_CONTENT_BUNDLE_JSON) as {
      runtime?: { runtime_mode?: string; storypack_id?: string; default_location?: string };
      content: { encounters: Array<{ id: string }> };
    };
    const encounterIds = bundle.content.encounters.map((encounter) => encounter.id);

    expect(DEFAULT_STORYPACK_ID).toBe('wuxia_jianghu_pack');
    expect(DEFAULT_STORYPACK_LABEL).toBe('이구학지 — 천기록');
    expect(bundle.runtime).toEqual({
      runtime_mode: 'storypack_main',
      world_id: 'wuxia_jianghu',
      storypack_id: 'wuxia_jianghu_pack',
      default_location: 'wuxia_commute_rift',
    });
    expect(encounterIds).toEqual([
      'wuxia_commute_rift_arrival',
      'wuxia_heuksa_bang_first_fight',
      'wuxia_cheonggi_record_first_fragment',
      'wuxia_seo_harin_rescue',
      'wuxia_cheongryu_apprentice_entry',
      'wuxia_cheongryu_chore_sparring',
      'wuxia_cheongryu_raid_route_split',
      'wuxia_cheongryu_raid_wounded_fallback',
      'wuxia_baekdo_medicine_debt',
      'wuxia_black_heaven_escape_price',
      'wuxia_heavenly_archive_previous_outsiders',
      'wuxia_wounded_shelter_dawn_offers',
      'wuxia_mumyeong_first_sighting',
      'wuxia_mumyeong_first_confrontation',
      'wuxia_mumyeong_copy_style_reveal',
      'wuxia_mumyeong_reads_orthodox_style',
      'wuxia_mumyeong_midgame_reunion',
      'wuxia_boss_first_appearance',
      'wuxia_mumyeong_request_for_aid',
      'wuxia_mumyeong_awakening',
      'wuxia_qingliu_attack_after_war',
      'wuxia_mumyeong_destroys_orthodox_sect',
      'wuxia_boss_recruits_mumyeong',
      'wuxia_mumyeong_departure_truth_summary',
    ]);
    expect(encounterIds).not.toContain('ex_employee_messenger');
  });

  it('does not expose 이구학지 as an opt-in preview now that it is the default', () => {
    expect(STORYPACK_PREVIEW_OPTIONS).toHaveLength(0);
    expect(storypackPreviewById('wuxia_jianghu_pack')).toBeUndefined();
  });
});

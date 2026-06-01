import { describe, expect, it } from 'vitest';

import {
  DEFAULT_CONTENT_BUNDLE_JSON,
  STORYPACK_PREVIEW_OPTIONS,
  storypackPreviewById,
} from './contentBundles';

describe('runtime content bundle registry', () => {
  it('keeps the default office bundle separate from storypack previews', () => {
    const office = JSON.parse(DEFAULT_CONTENT_BUNDLE_JSON) as { content: { encounters: Array<{ id: string }> } };
    const officeEncounterIds = office.content.encounters.map((encounter) => encounter.id);

    expect(officeEncounterIds).toContain('ex_employee_messenger');
    expect(officeEncounterIds).not.toContain('wuxia_commute_rift_arrival');
    expect(officeEncounterIds).not.toContain('wuxia_heuksa_bang_first_fight');
    expect(officeEncounterIds).not.toContain('wuxia_cheonggi_record_first_fragment');
    expect(officeEncounterIds).not.toContain('wuxia_seo_harin_rescue');
    expect(officeEncounterIds).not.toContain('wuxia_cheongryu_apprentice_entry');
  });

  it('registers wuxia_jianghu_pack as an explicit Web storypack preview option', () => {
    const option = storypackPreviewById('wuxia_jianghu_pack');
    const bundle = JSON.parse(option?.contentBundleJson ?? '{}') as {
      runtime?: { runtime_mode?: string; storypack_id?: string; default_location?: string };
      content?: { encounters?: Array<{ id: string }> };
    };

    expect(STORYPACK_PREVIEW_OPTIONS).toHaveLength(1);
    expect(option?.label).toContain('이구학지');
    expect(bundle.runtime).toEqual({
      runtime_mode: 'storypack_preview',
      world_id: 'wuxia_jianghu',
      storypack_id: 'wuxia_jianghu_pack',
      default_location: 'wuxia_commute_rift',
    });
    expect(bundle.content?.encounters?.map((encounter) => encounter.id)).toEqual([
      'wuxia_commute_rift_arrival',
      'wuxia_heuksa_bang_first_fight',
      'wuxia_cheonggi_record_first_fragment',
      'wuxia_seo_harin_rescue',
      'wuxia_cheongryu_apprentice_entry',
    ]);
  });
});

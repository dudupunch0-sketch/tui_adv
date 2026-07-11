import { describe, expect, it } from 'vitest';

import type { ScenePage } from '../../core/types';
import { sceneForVisual } from './ink/inkScenes';
import { renderStorybookPage } from './render';

function samplePrinterPage(overrides: Partial<ScenePage> = {}): ScenePage {
  return {
    mode: 'encounter',
    title: '복합기가 혼자 출력한다',
    location: {
      id: 'printer_area',
      name: '복합기 구역',
      description: '종이가 없는 복합기들이 대기 중이다.',
    },
    chapter_label: '격리 3턴',
    status_summary: {
      turn: 3,
      danger: 1,
      resources: [
        { id: 'health', label: '신체 반응', band: 'normal', text: '정상 범위', value: 92 },
        { id: 'sanity', label: '집중도', band: 'warning', text: '불안정', value: 28 },
        { id: 'battery', label: '단말기 전원', band: 'warning', text: '18%', value: 18 },
      ],
      warnings: ['집중도가 흔들리고 있습니다. 일부 기록이 다르게 보일 수 있습니다.'],
    },
    body_blocks: [
      {
        kind: 'narration',
        text: '복합기가 아직 고르지 않은 선택지를 출력한다.',
        source_id: 'printer_prints_alone',
      },
    ],
    dialogue_entries: [
      {
        speaker: '시스템 복합기',
        text: '아직 하지 않은 선택이 출력되고 있습니다.',
        source_id: 'printer_prints_alone',
      },
    ],
    visual: {
      id: 'printer_anomaly',
      kind: 'anomaly_object',
      alt: '복합기 출력물에 비상계단이라는 단어가 고정되어 있다.',
      source_id: 'printer_prints_alone',
    },
    actions: [
      { id: 'choice:take_printout', label: '출력물을 챙긴다', kind: 'choice', cost_text: null },
      { id: 'move:hallway', label: '복도로 물러난다', kind: 'move', cost_text: '허기 +1 / 갈증 +1' },
    ],
    blocked_actions: [
      {
        id: 'choice:check_toner',
        label: '토너 카트리지를 확인한다',
        kind: 'choice',
        cost_text: null,
        reasons: ['필요: 집중도 40 이상'],
      },
    ],
    history_entries: [
      { kind: 'action', text: '따뜻한 출력물을 접어 주머니에 넣었다.', source_id: 'printer_prints_alone' },
    ],
    inventory_summary: { items: ['crumpled_printout'], overflow_count: 0 },
    achievement_summary: { unlocked: ['first_signal'], newly_unlocked: [] },
    pressure_cues: [
      {
        kind: 'low_sanity',
        severity: 'warning',
        message: '집중도가 흔들리고 있습니다. 일부 기록이 다르게 보일 수 있습니다.',
        resource_id: 'sanity',
      },
    ],
    effect_cues: [
      {
        kind: 'glyph_anomaly',
        source: 'copier_output',
        intensity: 0.72,
        stable_terms: ['비상계단', '토너', '접힌 방향'],
        distortion: 'reflow_then_stabilize',
        duration_hint_ms: 1800,
        fallback_text: "출력물의 깨진 글자 사이로 '비상계단'이 선명하게 남는다.",
      },
    ],
    ...overrides,
  };
}

describe('Web Storybook renderer', () => {
  it('renders a ScenePage as storybook regions instead of the legacy fake-TUI dashboard', () => {
    const html = renderStorybookPage(samplePrinterPage());

    expect(html).toContain('data-renderer="web-storybook"');
    expect(html).toContain('data-story-phase="result"');
    expect(html).toContain('class="storybook-hud"');
    expect(html).toContain('class="hud-drawer-toggle"');
    expect(html).toContain('data-player-action="toggle-storybook-drawer"');
    expect(html).not.toContain('hud-nameplate');
    expect(html).not.toContain('hud-menu');
    expect(html).not.toContain('hud-stat-grid');
    expect(html).toContain('class="story-progress-rail"');
    expect(html).toContain('class="storybook-dock"');
    expect(html).toContain('class="choice-row"');
    expect(html).toContain('class="choice-bullet"');
    expect(html).toContain('data-region="visual"');
    expect(html).toContain('data-region="body"');
    expect(html).toContain('data-region="choices"');
    expect(html).toContain('data-region="history"');
    expect(html).toContain('data-region="status"');
    expect(html).toContain('복합기가 혼자 출력한다');
    expect(html).toContain('복합기 구역');
    expect(html).toContain('시스템 복합기');
    expect(html).toContain('복합기가 아직 고르지 않은 선택지를 출력한다.');
    expect(html).toContain('data-action-id="choice:take_printout"');
    expect(html).toContain('data-action-kind="choice"');
    expect(html).toContain('data-action-id="move:hallway"');
    expect(html).toContain('필요: 집중도 40 이상');
    expect(html).toContain('따뜻한 출력물을 접어 주머니에 넣었다.');
    expect(html).toContain('aria-label="기록"');
    expect(html).toContain('aria-label="소지품"');
    expect(html).toContain('data-player-action="show-start"');
    expect(html).toContain('data-player-action="abandon-run"');
    expect(html).toContain('data-player-action="toggle-audio"');
    expect(html).not.toContain('단말기 전원');
    expect(html).not.toContain('encounter · printer_area');
    expect(html).not.toContain('격리 3턴');
    expect(html).not.toContain('storybook-topline');
    expect(html).not.toContain('CURRENT ENCOUNTER');
    expect(html).not.toContain('LOCAL STATUS');
    expect(html).not.toContain('class="fake-tui"');
  });

  it('renders inventory and achievements as labeled drawers without placeholder dock items', () => {
    const html = renderStorybookPage(
      samplePrinterPage({
        inventory_summary: { items: ['commuter_badge'], overflow_count: 1 },
        achievement_summary: {
          unlocked: ['wuxia_first_arrival'],
          newly_unlocked: ['wuxia_first_arrival'],
        },
      }),
    );

    expect(html).toContain('class="storybook-dock" id="storybook-info-drawer"');
    expect(html).toContain('aria-label="소지품" data-dock="inventory"');
    expect(html).toContain('aria-label="업적" data-dock="achievements"');
    expect(html).toContain('사원증');
    expect(html).toContain('강호 출근');
    expect(html).toContain('…외 1개');
    expect(html).toContain('새로 새김');
    expect(html).not.toContain('data-dock="clues"');
    expect(html).not.toContain('data-dock="actions"');
  });

  it('keeps the scene title in the body and omits a duplicate movement title', () => {
    const encounterHtml = renderStorybookPage(samplePrinterPage());
    const movementHtml = renderStorybookPage(
      samplePrinterPage({
        mode: 'movement',
        title: '복합기 구역',
        visual: {
          id: 'printer_area',
          kind: 'location',
          alt: '복합기 구역으로 이어지는 복도',
          source_id: 'printer_area',
        },
      }),
    );

    expect(encounterHtml).toContain('<h1>복합기가 혼자 출력한다</h1>');
    expect(movementHtml).not.toContain('<h1>복합기 구역</h1>');
  });

  it('renders combat intervention pages as an ink duel without fabricated battle state', () => {
    const html = renderStorybookPage(
      samplePrinterPage({
        title: '흑사방 첫 난투',
        visual: {
          id: 'wuxia_heuksa_bang_first_fight',
          kind: 'combat_intervention',
          alt: '흑사방 말단과 마주 선 첫 난투',
          source_id: 'wuxia_heuksa_bang_first_fight',
        },
        effect_cues: [],
      }),
    );

    expect(html).toContain('data-story-phase="combat"');
    expect(html).toContain('data-visual-kind="combat"');
    expect(html).toContain('ink-scene--combat');
    expect(html).toContain('흑사방 말단과 마주 선 첫 난투');
    expect(html).not.toContain('전투 발생');
    expect(html).not.toContain('상황 개입');
  });

  it('passes through core-owned final epilogue body blocks without route recomputation', () => {
    const html = renderStorybookPage(
      samplePrinterPage({
        mode: 'ending',
        title: '이구학지 결산',
        location: {
          id: 'black_serpent_ledger_vault',
          name: '흑사방 장부고',
          description: '먹줄이 식은 장부고.',
        },
        body_blocks: [
          {
            kind: 'system',
            text: '천기록은 마지막 장을 다시 펼친다.',
            source_id: 'wuxia_final_epilogue_renderer_contract',
          },
          {
            kind: 'epilogue_result',
            text: 'final_result_key: true_route_victory\nowned_by: Rust GameCore',
            source_id: 'wuxia_final_epilogue_renderer_contract',
          },
          {
            kind: 'epilogue_card',
            text: 'card_id: epilogue_boss_broken_black_serpent\nvariant: true_route_victory',
            source_id: 'epilogue_boss_broken_black_serpent',
          },
          {
            kind: 'epilogue_suppressed',
            text: 'card_id: epilogue_boss_black_serpent_banner\nsuppressed_by: true_route_victory',
            source_id: 'epilogue_boss_black_serpent_banner',
          },
        ],
        actions: [],
        blocked_actions: [],
        visual: {
          id: 'ending:wuxia_final_epilogue_renderer_contract',
          kind: 'ending',
          alt: '이구학지 결산',
          source_id: 'wuxia_final_epilogue_renderer_contract',
        },
        history_entries: [
          {
            kind: 'action',
            text: '후일담 출력기는 아직 열리지 않는다.',
            source_id: 'wuxia_black_serpent_aftermath',
          },
        ],
      }),
    );

    expect(html).toContain('data-mode="ending"');
    expect(html).toContain('class="epilogue-block epilogue-block--epilogue_result"');
    expect(html).toContain('class="epilogue-block epilogue-block--epilogue_card"');
    expect(html).toContain('class="epilogue-block epilogue-block--epilogue_suppressed"');
    expect(html).toContain('data-body-kind="epilogue_result"');
    expect(html).toContain('data-body-kind="epilogue_card"');
    expect(html).toContain('data-body-kind="epilogue_suppressed"');
    expect(html).toContain('계약 기록');
    expect(html).toContain('후일담 카드');
    expect(html).toContain('data-field-key="final_result_key"');
    expect(html).toContain('true_route_victory');
    expect(html).toContain('data-field-key="owned_by"');
    expect(html).toContain('Rust GameCore');
    expect(html).toContain('data-field-key="card_id"');
    expect(html).toContain('epilogue_boss_broken_black_serpent');
    expect(html).toContain('data-field-key="suppressed_by"');
    expect(html).toContain('기록의 이 장은 여기서 끝났다.');
    expect(html).toContain('data-player-action="show-start"');
    expect(html).not.toContain('<p class="storybook-summary">후일담 출력기는 아직 열리지 않는다.</p>');
    expect(html).not.toContain('final_epilogue_renderer_opened');
  });

  it('keeps GlyphFX stable terms and fallback text readable for reduced-motion rendering', () => {
    const html = renderStorybookPage(samplePrinterPage());

    expect(html).toContain('data-effect-kind="glyph_anomaly"');
    expect(html).toContain('비상계단');
    expect(html).toContain('토너');
    expect(html).toContain('접힌 방향');
    expect(html).toContain('출력물의 깨진 글자 사이로 &#39;비상계단&#39;이 선명하게 남는다.');
  });

  it('renders unknown visual ids as safe placeholders without dropping actions', () => {
    const html = renderStorybookPage(
      samplePrinterPage({
        visual: {
          id: 'unknown_visual',
          kind: 'unknown',
          alt: '아직 카탈로그에 없는 장면이다.',
          source_id: 'fixture',
        },
        effect_cues: [],
      }),
    );

    expect(html).toContain('data-visual-kind="placeholder"');
    expect(html).toContain('아직 카탈로그에 없는 장면이다.');
    expect(html).toContain('data-action-id="choice:take_printout"');
  });

  it('authors the planned wuxia scenes and deterministic location variants as ink specs', () => {
    const plannedVisuals = [
      'wuxia_commute_rift',
      'wuxia_cheongryu_raid_wounded_fallback',
      'wuxia_heavenly_archive_previous_outsiders',
      'wuxia_cheonoe_pyeonrin_second_reward',
      'wuxia_mumyeong_request_for_aid',
      'wuxia_qingliu_attack_after_war',
      'wuxia_sado_final_phase_1_price_tag',
      'wuxia_sado_final_phase_2_weakpoint_control',
      'wuxia_sado_final_phase_3_outside_calculation',
      'ending:wuxia_return_modern_commute_scene_resolved',
      'ending:wuxia_settlement_stay_scene_resolved',
      'ending:wuxia_preview_grounded',
    ];

    for (const visualId of plannedVisuals) {
      expect(sceneForVisual(visualId, 'encounter')).toBeDefined();
    }
    expect(sceneForVisual('location:cheongryu_gate', 'movement')).toEqual(sceneForVisual('location:cheongryu_gate', 'movement'));
  });
});

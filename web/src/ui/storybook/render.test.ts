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
    expect(html).not.toContain('쪽 · ');
    expect(html).toContain('class="game-topbar" data-region="topbar"');
    expect(html).toContain('class="game-viewport" data-region="viewport"');
    expect(html).toMatch(/<p class="hud-document"[^>]*>기록<span/);
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

  it('renders ordered story content with illustrations and choices at their authored positions', () => {
    const page = samplePrinterPage({
      content_stream: [
        { kind: 'illustration', stage_id: 'opening', text: null, speaker: null, visual_id: null, alt: '복합기 사건.png', placeholder: true, actions: [] },
        { kind: 'narration', stage_id: 'opening', text: '복합기가 먼저 깨어났다.', speaker: null, visual_id: null, alt: null, placeholder: false, actions: [] },
        { kind: 'choice', stage_id: 'first_choice', text: null, speaker: null, visual_id: null, alt: null, placeholder: false, actions: [{ id: 'choice:listen', label: '기계음을 듣는다', kind: 'choice', cost_text: null }] },
        { kind: 'result_summary', stage_id: 'first_result', text: '종이 안쪽에서 발소리가 들렸다.', speaker: null, visual_id: null, alt: null, placeholder: false, actions: [] },
        { kind: 'dialogue', stage_id: 'aftermath', text: '아직 끝나지 않았습니다.', speaker: '시스템 복합기', visual_id: null, alt: null, placeholder: false, actions: [] },
      ],
    });
    const html = renderStorybookPage(page);

    expect(html).toContain('class="story-flow story-flow--ordered"');
    expect(html).toContain('data-story-phase="result"');
    expect(html).toContain('data-placeholder="true"');
    expect(html).toContain('복합기 사건.png');
    expect(html).toContain('data-stage-id="first_choice"');
    expect(html).toContain('data-action-id="choice:listen"');
    expect(html.indexOf('복합기 사건.png')).toBeLessThan(html.indexOf('복합기가 먼저 깨어났다.'));
    expect(html.indexOf('기계음을 듣는다')).toBeLessThan(html.indexOf('종이 안쪽에서 발소리가 들렸다.'));
    expect(html).not.toContain('data-action-id="choice:take_printout"');
  });

  it('does not label ordinary narration as cheongirok', () => {
    const html = renderStorybookPage(
      samplePrinterPage({
        content_stream: [
          { kind: 'narration', stage_id: 'story', text: '평범한 장면 서술.', speaker: null, visual_id: null, alt: null, placeholder: false, actions: [] },
        ],
      }),
    );

    expect(html).toContain('data-content-kind="narration"');
    expect(html).toContain('data-story-phase="story"');
    expect(html).not.toContain('data-content-kind="cheongirok"');
  });

  it('renders combat intervention pages as an ink duel without fabricated battle state', () => {
    const html = renderStorybookPage(
      samplePrinterPage({
        title: '흑사방 첫 난투',
        visual: {
          id: 'wuxia_heuksa_bang_first_fight_mock',
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

  it('renders registered art manifest visual IDs as image assets with SVG fallbacks', () => {
    const html = renderStorybookPage(
      samplePrinterPage({
        visual: {
          id: 'wuxia_commute_rift',
          kind: 'anomaly_object',
          alt: '균열에서 떨어진 주인공',
          source_id: 'commute_rift',
        },
        effect_cues: [],
      }),
    );

    expect(html).toContain('data-visual-kind="art"');
    expect(html).toContain('class="ink-scene__art"');
    expect(html).toContain('src="/assets/art/wuxia_commute_rift.webp"');
    expect(html).toContain('<svg viewBox="0 0 280 168"');
  });

  it('renders character_summary and action check info when present', () => {
    const html = renderStorybookPage(
      samplePrinterPage({
        actions: [
          {
            id: 'choice:take_printout',
            label: '출력물을 챙긴다',
            kind: 'choice',
            cost_text: '허기 -10',
            check: {
              ability_id: 'logic',
              ability_label: '논리',
              success_percent: 58.3,
            },
          },
        ],
        character_summary: {
          name: '주인공',
          title_label: '검호',
          abilities: [
            { id: 'logic', label: '논리', value: 3 },
            { id: 'empathy', label: '공감', value: 2 },
          ],
        },
      }),
    );

    expect(html).toContain('class="character-summary-section"');
    expect(html).toContain('class="character-name-line" data-region="character"');
    expect(html).toContain('<span class="character-title-seal">검호</span>');
    expect(html).toContain('<span class="character-name">주인공</span>');
    expect(html).toContain('class="ability-row" data-ability-id="logic"');
    expect(html).toContain('<strong>논리</strong> <span class="ability-value">3</span>');
    expect(html).toContain('class="ability-row" data-ability-id="empathy"');
    expect(html).toContain('<strong>공감</strong> <span class="ability-value">2</span>');

    expect(html).toContain('class="choice-check" data-ability-id="logic" data-check-band="uncertain"');
    expect(html).toContain('논리 판정');
    expect(html).toContain('--odds: 58.3%');
    expect(html).toContain('성공 58.3%');
  });

  it('renders the progression gauge in the HUD and drawer when progression is present', () => {
    const html = renderStorybookPage(
      samplePrinterPage({
        progression: { experience: 32, target: 100, label: '천기' },
      }),
    );

    expect(html).toContain('class="hud-progression"');
    expect(html).toContain('class="drawer-progression"');
    expect(html).toContain('aria-label="천기 32 / 100"');
    expect(html).toContain('--fill: 32%');
    expect(html).toContain('32 / 100');
  });

  it('omits the progression gauge when the page has no progression data', () => {
    const html = renderStorybookPage(samplePrinterPage());

    expect(html).not.toContain('hud-progression');
    expect(html).not.toContain('drawer-progression');
  });

  it('renders inline result log lines with color coding classes', () => {
    const html = renderStorybookPage(
      samplePrinterPage({
        history_entries: [
          { kind: 'action', text: '기본 서사 로그.\n+ 체력 10\n- 정신력 5', source_id: 'printer_prints_alone' },
        ],
        inventory_summary: { items: [], overflow_count: 0 },
        achievement_summary: { unlocked: [], newly_unlocked: [] },
      }),
    );

    expect(html).toContain('<p class="storybook-summary">기본 서사 로그.</p>');
    expect(html).toContain('<p class="storybook-summary result-gain">+ 체력 10</p>');
    expect(html).toContain('<p class="storybook-summary result-loss">- 정신력 5</p>');
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

  it('prefers content_labels over dictionary labels in rendering', () => {
    const html = renderStorybookPage(
      samplePrinterPage({
        inventory_summary: { items: ['commuter_badge'], overflow_count: 0 },
        achievement_summary: { unlocked: ['first_signal'], newly_unlocked: [] },
        content_labels: {
          items: [{ id: 'commuter_badge', label: '특별한 패스' }],
          achievements: [{ id: 'first_signal', label: '특별한 업적' }]
        }
      })
    );

    expect(html).toContain('특별한 패스');
    expect(html).not.toContain('사원증');
    expect(html).toContain('특별한 업적');
    expect(html).not.toContain('첫 신호 확인');
  });

  it('renders check resolution banner when check_result is present', () => {
    const html = renderStorybookPage(
      samplePrinterPage({
        check_result: {
          ability_id: 'logic',
          ability_label: '논리',
          dice: [4, 2],
          ability_value: 2,
          difficulty: 7,
          total: 8,
          success: true,
        },
      }),
    );

    expect(html).toContain('class="check-resolution"');
    expect(html).toContain('data-region="check-result"');
    expect(html).toContain('data-check-outcome="success"');
    expect(html).toContain('data-ability-id="logic"');
    expect(html).toContain('aria-label="판정 결과: 성공"');
    expect(html).toContain('<i class="check-die">⚃</i>');
    expect(html).toContain('<i class="check-die">⚁</i>');
    expect(html).toContain('2d6 4+2 +논리 2 = 8 / 목표 7');
    expect(html).toContain('class="check-resolution__seal" aria-hidden="true">成</span>');
    expect(html).toContain('성공</span>');
  });

  it('omits check resolution banner when check_result is absent', () => {
    const html = renderStorybookPage(samplePrinterPage());
    expect(html).not.toContain('class="check-resolution"');
  });

  it('forces data-story-phase="collapse" when page visual is collapse_gate', () => {
    const html = renderStorybookPage(
      samplePrinterPage({
        visual: {
          id: 'wuxia_collapse_gate',
          kind: 'collapse_gate',
          alt: '붕괴 게이트',
          source_id: 'collapse',
        },
      }),
    );

    expect(html).toContain('data-story-phase="collapse"');
  });

  it('renders bundle label with no 미번역 note when id is present in content_labels', () => {
    const html = renderStorybookPage(
      samplePrinterPage({
        achievement_summary: { unlocked: ['first_signal'], newly_unlocked: [] },
        content_labels: {
          items: [],
          achievements: [{ id: 'first_signal', label: '특별한 업적' }],
        },
      }),
    );

    expect(html).toContain('특별한 업적');
    expect(html).not.toContain('storybook-translation-note');
    expect(html).not.toContain('(미번역)');
  });

  it('renders humanized id plus exactly one 미번역 note when id is absent from content_labels and dictionary', () => {
    const html = renderStorybookPage(
      samplePrinterPage({
        achievement_summary: { unlocked: ['totally_unknown_achievement'], newly_unlocked: [] },
      }),
    );

    const resultLogMatch = html.match(/<section class="story-result-log"[\s\S]*?<\/section>/);
    expect(resultLogMatch).toBeTruthy();
    const resultLogHtml = resultLogMatch![0];
    expect(resultLogHtml).toContain('totally unknown achievement');
    expect(resultLogHtml).not.toContain('(미번역)');
    const noteCount = (resultLogHtml.match(/storybook-translation-note/g) ?? []).length;
    expect(noteCount).toBe(1);
  });
});

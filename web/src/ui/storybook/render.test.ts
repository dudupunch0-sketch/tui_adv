import { describe, expect, it } from 'vitest';

import type { ScenePage } from '../../core/types';
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
    expect(html).toContain('data-region="visual"');
    expect(html).toContain('data-region="body"');
    expect(html).toContain('data-region="choices"');
    expect(html).toContain('data-region="history"');
    expect(html).toContain('data-region="status"');
    expect(html).toContain('복합기가 혼자 출력한다');
    expect(html).toContain('복합기 구역');
    expect(html).toContain('격리 3턴');
    expect(html).toContain('시스템 복합기');
    expect(html).toContain('복합기가 아직 고르지 않은 선택지를 출력한다.');
    expect(html).toContain('data-action-id="choice:take_printout"');
    expect(html).toContain('data-action-id="move:hallway"');
    expect(html).toContain('필요: 집중도 40 이상');
    expect(html).toContain('따뜻한 출력물을 접어 주머니에 넣었다.');
    expect(html).not.toContain('CURRENT ENCOUNTER');
    expect(html).not.toContain('LOCAL STATUS');
    expect(html).not.toContain('class="fake-tui"');
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
});

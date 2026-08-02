import { describe, expect, it } from 'vitest';

import type {
  CombatConclusionReport,
  CombatSpectatorCue,
  CombatSpectatorFrame,
  CombatSpectatorLogEntry,
  CombatSpectatorPage,
  CombatSpectatorPiece,
  CombatSpectatorView,
} from '../../../core/types';
import { renderCombatBoard, renderCombatLog, renderCombatReport, renderCombatStage } from './renderCombatStage';

function piece(overrides: Partial<CombatSpectatorPiece> = {}): CombatSpectatorPiece {
  return {
    id: 'ally_1',
    side: 'ally',
    position: { x: 0, y: 0 },
    facing: { x: 1, y: 0 },
    active: true,
    cues: [],
    ...overrides,
  };
}

function frame(tick: number, pieces: CombatSpectatorPiece[]): CombatSpectatorFrame {
  return { tick, pieces };
}

function view(overrides: Partial<CombatSpectatorView> = {}): CombatSpectatorView {
  return {
    simulation_version: 'v-test',
    resolution_fingerprint: 'res-fp',
    tick_millis: 100,
    frames: [],
    core_log: [],
    full_log: [],
    fingerprint: 'view-fp',
    ...overrides,
  };
}

describe('renderCombatBoard', () => {
  it('renders only the last frame — earlier frame coordinates never appear', () => {
    const html = renderCombatBoard(
      view({
        frames: [
          frame(0, [piece({ id: 'ally_1', position: { x: 99, y: 99 } })]),
          frame(3, [piece({ id: 'ally_1', position: { x: 5, y: 5 } })]),
        ],
      }),
    );
    // Only frame 3's piece is a single piece so span === 0 -> 50% center;
    // the point is the earlier frame's distinguishing coordinate (99) never
    // leaks into the projection math anywhere in the output.
    expect(html).not.toContain('99');
    expect(html).toContain('data-piece-id="ally_1"');
  });

  it('keeps the extreme pieces off the board edge so translate(-50%) cannot clip them', () => {
    const html = renderCombatBoard(
      view({
        frames: [
          frame(1, [
            piece({ id: 'ally_1', position: { x: 0, y: 0 } }),
            piece({ id: 'enemy_1', position: { x: 5, y: 4 }, side: 'enemy' }),
          ]),
        ],
      }),
    );
    // 전투원 2명이면 두 말이 항상 min/max에 놓인다. 0%/100%로 투영하면
    // 둘 다 보드 경계에서 절반 잘린다.
    expect(html).not.toContain('--piece-x: 0%');
    expect(html).not.toContain('--piece-y: 0%');
    expect(html).not.toContain('--piece-x: 100%');
    expect(html).not.toContain('--piece-y: 100%');
    expect(html).toContain('--piece-x: 14%');
    expect(html).toContain('--piece-x: 86%');
  });

  it('centers pieces at 50% when the coordinate span is zero, with no NaN/Infinity', () => {
    const html = renderCombatBoard(
      view({
        frames: [
          frame(1, [
            piece({ id: 'ally_1', position: { x: 7, y: 7 } }),
            piece({ id: 'ally_2', position: { x: 7, y: 7 }, side: 'ally' }),
          ]),
        ],
      }),
    );
    expect(html).toContain('--piece-x: 50%');
    expect(html).toContain('--piece-y: 50%');
    expect(html).not.toContain('NaN');
    expect(html).not.toContain('Infinity');
  });

  it('shows all 5 cue symbols with matching data-cue attributes', () => {
    const cues: CombatSpectatorCue[] = ['attack', 'hit', 'evade', 'balance_broken', 'incapacitated'];
    const html = renderCombatBoard(
      view({ frames: [frame(1, [piece({ cues })])] }),
    );
    for (const cue of cues) {
      expect(html).toContain(`data-cue-${cue}="true"`);
      expect(html).toContain(`data-cue="${cue}"`);
    }
    // 여러 cue가 같은 자리에 절대 배치되어 겹치지 않도록 한 컨테이너에 담는다.
    expect(html).toContain('combat-board__cues');
    // The 5 glyphs from I11's cue table.
    expect(html).toContain('攻');
    expect(html).toContain('打');
    expect(html).toContain('避');
    expect(html).toContain('傾');
    expect(html).toContain('倒');
  });

  it('handles an empty frames array without throwing, and says so', () => {
    expect(() => renderCombatBoard(view({ frames: [] }))).not.toThrow();
    const html = renderCombatBoard(view({ frames: [] }));
    expect(html).toContain('표시할 프레임이 없다');
  });

  it('handles a frame with zero pieces without throwing, and says so', () => {
    const html = renderCombatBoard(view({ frames: [frame(1, [])] }));
    expect(html).toContain('표시할 말이 없다');
  });

  it('includes every piece id, side, and coordinate in the semantic alternative table', () => {
    const html = renderCombatBoard(
      view({
        frames: [
          frame(2, [
            piece({ id: 'ally_1', side: 'ally', position: { x: 0, y: 3 } }),
            piece({ id: 'enemy_1', side: 'enemy', position: { x: 10, y: 3 } }),
          ]),
        ],
      }),
    );
    expect(html).toContain('<table');
    expect(html).toContain('ally_1');
    expect(html).toContain('enemy_1');
    expect(html).toContain('아군');
    expect(html).toContain('적군');
    expect(html).toContain('0');
    expect(html).toContain('10');
  });

  it('never calls an active piece "생존" — active is participation, not liveness', () => {
    // 실측: 체력이 0이 된 뒤에도 core는 `active: true`를 유지하고 전투불능은
    // `Incapacitated` cue로만 나타난다. 생존/전투불능은 보고서의
    // survivor_ids/defeated_ids가 소유한다.
    const html = renderCombatBoard(
      view({
        frames: [
          frame(8, [piece({ id: 'ally_1', active: true, cues: ['incapacitated'] })]),
        ],
      }),
    );
    expect(html).not.toContain('생존');
    expect(html).toContain('참전');
    expect(html).toContain('전투불능');
  });

  it('escapes a piece id containing markup', () => {
    const html = renderCombatBoard(
      view({ frames: [frame(1, [piece({ id: '<script>alert(1)</script>' })])] }),
    );
    expect(html).not.toContain('<script>alert(1)</script>');
    expect(html).toContain('&lt;script&gt;');
  });
});

function logEntry(overrides: Partial<CombatSpectatorLogEntry> = {}): CombatSpectatorLogEntry {
  return {
    tick: 1,
    sequence: 0,
    template_id: 'combat.log.move_intent',
    importance: 'important',
    actor_id: 'ally_1',
    target_id: null,
    value_hundredths: null,
    effect_id: null,
    ...overrides,
  };
}

describe('renderCombatLog', () => {
  it('sentences only core_log entries; full_log-only entries never get a sentence', () => {
    const html = renderCombatLog(
      view({
        core_log: [logEntry({ actor_id: 'ally_1' })],
        full_log: [logEntry({ actor_id: 'ally_1' }), logEntry({ actor_id: 'ghost_only_in_full_log' })],
      }),
    );
    expect(html).toContain('ally_1 이동 의도');
    expect(html).not.toContain('ghost_only_in_full_log');
  });

  it('shows the full_log count', () => {
    const html = renderCombatLog(
      view({ core_log: [], full_log: [logEntry(), logEntry(), logEntry()] }),
    );
    expect(html).toContain('전체 로그 3건');
  });

  it('truncates core_log at 40 rows and states the omitted count explicitly', () => {
    const coreLog = Array.from({ length: 41 }, (_, i) => logEntry({ actor_id: `ally_${i}` }));
    const html = renderCombatLog(view({ core_log: coreLog, full_log: [] }));
    const rows = html.match(/class="combat-log__row"/g) ?? [];
    expect(rows.length).toBe(40);
    expect(html).toContain('…(생략 1줄)');
  });

  it('marks the damage_applied row with data-cue="hit"; the other 5 template ids carry no data-cue', () => {
    const knownIds = [
      'combat.log.move_intent',
      'combat.log.target_selection',
      'combat.log.collision',
      'combat.log.effect_applied',
      'combat.log.effect_applied_hidden',
    ];
    const html = renderCombatLog(
      view({
        core_log: [
          logEntry({ template_id: 'combat.log.damage_applied', value_hundredths: 100 }),
          ...knownIds.map((template_id) => logEntry({ template_id })),
        ],
      }),
    );
    expect(html).toContain('data-template-id="combat.log.damage_applied" data-cue="hit"');
    for (const id of knownIds) {
      const rowMatch = new RegExp(`data-template-id="${id.replace(/\./g, '\\.')}"[^>]*>`).exec(html);
      expect(rowMatch).not.toBeNull();
      expect(rowMatch![0]).not.toContain('data-cue');
    }
  });

  it('surfaces an unknown template_id instead of dropping it, with a visible marker', () => {
    const html = renderCombatLog(
      view({ core_log: [logEntry({ template_id: 'combat.log.made_up_event' })] }),
    );
    expect(html).toContain('data-log-unknown="true"');
    expect(html).toContain('combat.log.made_up_event');
  });
});

function baseReport(overrides: Partial<CombatConclusionReport> = {}): CombatConclusionReport {
  return {
    resolution_fingerprint: 'res-fp',
    outcome: 'ally_victory',
    reason: 'all_enemies_defeated',
    decisive_tick: 3,
    active_allies: 1,
    active_enemies: 0,
    survivor_ids: ['ally_1'],
    defeated_ids: ['enemy_1'],
    removed_combat_effect_ids: [],
    retained_effect_ids: [],
    duration_millis: 300,
    combatants: [
      {
        id: 'ally_1',
        damage_dealt_hundredths: 1050,
        damage_taken_hundredths: 200,
        kills: 1,
        incapacitated: false,
      },
    ],
    top_damage_dealt_id: 'ally_1',
    top_damage_taken_id: 'enemy_1',
    fingerprint: 'report-fp',
    ...overrides,
  };
}

describe('renderCombatReport', () => {
  it('hides top_damage_dealt_id / top_damage_taken_id lines entirely when null (no "없음" substitute)', () => {
    const html = renderCombatReport(
      view(),
      baseReport({ top_damage_dealt_id: null, top_damage_taken_id: null }),
    );
    expect(html).not.toContain('최대 피해를 가한');
    expect(html).not.toContain('최대 피해를 받은');
  });

  it('shows the highlight lines when the ids are present', () => {
    const html = renderCombatReport(view(), baseReport());
    expect(html).toContain('ally_1');
    expect(html).toContain('enemy_1');
  });

  it('hides the decisive_tick line when null, per I8', () => {
    const withTick = renderCombatReport(view(), baseReport({ decisive_tick: 3 }));
    const withoutTick = renderCombatReport(view(), baseReport({ decisive_tick: null }));
    expect(withTick).toContain('3');
    expect(withoutTick).not.toMatch(/tick[^"]*:\s*null/);
    // No numeral leaks in from a null decisive_tick.
    const tickLineRegex = /결착[^<]*tick[^<]*/;
    expect(withoutTick).not.toMatch(tickLineRegex);
  });

  it('puts the fingerprint and simulation_version in the same element (정본 03 비교 계약)', () => {
    const html = renderCombatReport(
      view({ simulation_version: 'v-42' }),
      baseReport({ fingerprint: 'fp-abc' }),
    );
    const match = /<p[^>]*>[^<]*fp-abc[^<]*<\/p>/.exec(html);
    expect(match).not.toBeNull();
    expect(match![0]).toContain('v-42');
  });

  it('never contains forbidden strategic-analysis phrases', () => {
    const html = renderCombatReport(view(), baseReport());
    for (const forbidden of ['MVP', '전략', '전환점', '조언', '원인']) {
      expect(html).not.toContain(forbidden);
    }
  });
});

describe('renderCombatStage', () => {
  it('returns an empty string when combat is undefined', () => {
    expect(renderCombatStage(undefined)).toBe('');
  });

  it('omits the combat-report section entirely when report is absent (combat in progress)', () => {
    const page: CombatSpectatorPage = {
      view: view({ frames: [frame(1, [piece()])] }),
    };
    const html = renderCombatStage(page);
    expect(html).toContain('data-region="combat"');
    expect(html).not.toContain('data-region="combat-report"');
  });

  it('includes the combat-report section when report is present', () => {
    const page: CombatSpectatorPage = {
      view: view({ frames: [frame(1, [piece()])] }),
      report: baseReport(),
    };
    const html = renderCombatStage(page);
    expect(html).toContain('data-region="combat-report"');
  });
});

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
import {
  renderCombatBoard,
  renderCombatFullLog,
  renderCombatLog,
  renderCombatReport,
  renderCombatStage,
} from './renderCombatStage';

function piece(overrides: Partial<CombatSpectatorPiece> = {}): CombatSpectatorPiece {
  return {
    id: 'ally_1',
    side: 'ally',
    position: { q: 0, r: 0 },
    facing: { q: 1, r: 0 },
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
          frame(0, [piece({ id: 'ally_1', position: { q: 99, r: 99 } })]),
          frame(3, [piece({ id: 'ally_1', position: { q: 5, r: 5 } })]),
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
            piece({ id: 'ally_1', position: { q: 0, r: 0 } }),
            piece({ id: 'enemy_1', position: { q: 5, r: 4 }, side: 'enemy' }),
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
            piece({ id: 'ally_1', position: { q: 7, r: 7 } }),
            piece({ id: 'ally_2', position: { q: 7, r: 7 }, side: 'ally' }),
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
            piece({ id: 'ally_1', side: 'ally', position: { q: 0, r: 3 } }),
            piece({ id: 'enemy_1', side: 'enemy', position: { q: 10, r: 3 } }),
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

  it('accessibility_table_labels_coordinates_as_q_and_r: matches the terminal renderer\'s "q=…, r=…" notation, never "(x, y)"', () => {
    // §4-4: the two renderers' notation must agree — terminal's format is
    // `"@ (q={}, r={})"` (`crates/escape-terminal/src/snapshot.rs`).
    const html = renderCombatBoard(
      view({
        frames: [frame(2, [piece({ id: 'ally_1', side: 'ally', position: { q: 3, r: -2 } })])],
      }),
    );
    expect(html).toContain('(q=3, r=-2)');
    expect(html).not.toMatch(/\(3, -2\)/);
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

// ---------------------------------------------------------------------------
// §4-2 of fable_combat_hex_t1b2_step1_2608072024.md — the axial-to-screen
// conversion goes in *ahead of* projectAxis's per-axis normalization, and
// that normalization's min/max must be computed from the *converted*
// px/py, never the raw q/r. Before this fix, `q`/`r` were normalized
// directly as if they were already orthogonal screen axes (they are 60°
// apart), shearing the board.
// ---------------------------------------------------------------------------
describe('renderCombatBoard — §4-2 axial-to-screen projection', () => {
  it('pieces_at_equal_hex_distance_are_equally_far_apart_on_screen: two hex-adjacent pairs in different directions land the same percent-distance apart', () => {
    // Flat-top axial -> screen: px = 1.5q, py = sqrt(3)*(r + q/2). Any two of
    // `HexCoord::NEIGHBOR_DIRECTIONS` are hex distance 1 from the origin, and
    // a correct conversion places every one of them at the same Euclidean
    // distance from the origin (they form a regular hexagon around it) — a
    // property that is specific to distance-1 pairs sharing an endpoint
    // (translation-invariance of the linear conversion extends it to any
    // adjacent pair anywhere on the grid), not a general "same hex distance
    // implies same screen distance" claim for arbitrary pairs.
    //
    // `renderCombatBoard`'s subsequent step (`projectAxis`) still normalizes
    // x and y *independently*, which is kept as-is per plan §9 (full
    // aspect-correct hex-tile projection is T9's job, out of scope here) —
    // independent per-axis scaling would rescale the two pairs' converted
    // deltas by different factors unless the combined bounding box is
    // square. `anchor` is a scaffolding piece (not real game data,
    // deliberately non-integer q/r) chosen so xspan === yspan, isolating the
    // §4-2 property this test pins from that separate, accepted limitation.
    const html = renderCombatBoard(
      view({
        frames: [
          frame(1, [
            piece({ id: 'origin', position: { q: 0, r: 0 } }),
            piece({ id: 'dir_a', position: { q: 1, r: 0 } }),
            piece({ id: 'dir_b', position: { q: 0, r: 1 } }),
            piece({ id: 'anchor', position: { q: 2 / Math.sqrt(3), r: -1 / Math.sqrt(3) } }),
          ]),
        ],
      }),
    );
    const percentOf = (id: string) => {
      const match = new RegExp(
        `data-piece-id="${id}"[^>]*--piece-x: (-?[\\d.]+)%; --piece-y: (-?[\\d.]+)%`,
      ).exec(html);
      expect(match).not.toBeNull();
      return { x: Number(match![1]), y: Number(match![2]) };
    };
    const origin = percentOf('origin');
    const distanceFromOrigin = (id: string) => {
      const p = percentOf(id);
      return Math.hypot(p.x - origin.x, p.y - origin.y);
    };
    expect(distanceFromOrigin('dir_a')).toBeCloseTo(distanceFromOrigin('dir_b'), 1);
    // With the square bounding box, both land exactly 72 points apart (the
    // full inset-adjusted 14%–86% range) — either one is both the axis
    // minimum on one screen axis and the axis maximum on the other.
    expect(distanceFromOrigin('dir_a')).toBeCloseTo(72, 1);
  });

  it('projection_range_uses_converted_coordinates_not_raw_axial: the axis range comes from converted px/py, not raw q/r', () => {
    // If min/max were taken from raw axial q/r (the bug this pins), `mid`
    // would land at --piece-y: 14% — the same as `origin`, since both have
    // raw r = 0 — instead of the correct 50% its actual converted y
    // (sqrt(3)*2, out of a converted span of sqrt(3)*4) works out to.
    const html = renderCombatBoard(
      view({
        frames: [
          frame(1, [
            piece({ id: 'origin', position: { q: 0, r: 0 } }),
            piece({ id: 'mid', position: { q: 4, r: 0 } }),
            piece({ id: 'far_r', position: { q: 0, r: 4 } }),
          ]),
        ],
      }),
    );
    expect(html).toMatch(/data-piece-id="mid"[^>]*--piece-y: 50%/);
    expect(html).not.toMatch(/data-piece-id="mid"[^>]*--piece-y: 14%/);
  });
});

// ---------------------------------------------------------------------------
// Wave 3 Step 1d-3 — WP2: playback wiring (projection range expansion +
// generated <style> block). The underlying keyframe-string math (I1/I5/I9)
// is pinned by combatMotion.test.ts; these tests only pin how
// renderCombatStage.ts feeds that module (I4: which frames/coordinates,
// nothing recomputed).
// ---------------------------------------------------------------------------
describe('renderCombatBoard — Step 1d-3 playback wiring', () => {
  it('expands the projection range across every frame, not just the last one, so a mid-motion piece never clips the board edge', () => {
    // ally_1 visits (q=10, r=0) at tick 0 but rests at the origin at the
    // final tick. If the projection only looked at the final frame (1d-2
    // behaviour), a single piece there would have span 0 and sit dead-center
    // (50%). This test pins the *offset* asserted inside the generated
    // `@keyframes`, which is the only place tick 0's coordinate can still
    // show up now that the projection spans all frames.
    //
    // Converted screen points: origin -> (0, 0); (q=10, r=0) -> (15,
    // 5*sqrt(3)) ≈ (15, 8.660254). The origin is the min on both axes and
    // (10, 0) the max on both, so both axes' spans differ from the old
    // cartesian fixture's numbers but the two axes' *own* offsets come out
    // equal by construction (both go from min to max over their own span) —
    // 72 points on both x and y, not just x.
    const html = renderCombatBoard(
      view({
        frames: [
          frame(0, [piece({ id: 'ally_1', position: { q: 10, r: 0 } })]),
          frame(1, [piece({ id: 'ally_1', position: { q: 0, r: 0 } })]),
        ],
      }),
    );
    expect(html).toContain('<style>');
    expect(html).toMatch(/0% \{ translate: calc\(-50% \+ 72cqw\) calc\(-50% \+ 72cqh\)/);
  });

  it('emits no <style> block when there is only one frame (nothing to animate, matches 1d-2 byte-for-byte apart from this)', () => {
    const html = renderCombatBoard(view({ frames: [frame(1, [piece()])] }));
    expect(html).not.toContain('<style>');
  });

  it('emits a <style> block whose animation duration is exactly (frames.length - 1) * tick_millis', () => {
    const html = renderCombatBoard(
      view({
        tick_millis: 150,
        frames: [
          frame(0, [piece({ id: 'ally_1', position: { q: 0, r: 0 } })]),
          frame(1, [piece({ id: 'ally_1', position: { q: 2, r: 0 } })]),
          frame(2, [piece({ id: 'ally_1', position: { q: 4, r: 0 } })]),
        ],
      }),
    );
    expect(html).toContain('300ms linear');
  });

  it('wraps the generated <style> content in prefers-reduced-motion: no-preference', () => {
    const html = renderCombatBoard(
      view({
        frames: [
          frame(0, [piece({ id: 'ally_1', position: { q: 0, r: 0 } })]),
          frame(1, [piece({ id: 'ally_1', position: { q: 2, r: 0 } })]),
        ],
      }),
    );
    expect(html).toContain('<style>@media (prefers-reduced-motion: no-preference)');
  });

  it('does not animate a piece that is missing from an earlier frame instead of inventing its position', () => {
    const html = renderCombatBoard(
      view({
        frames: [
          frame(0, [piece({ id: 'ally_1', position: { q: 0, r: 0 } })]),
          frame(1, [
            piece({ id: 'ally_1', position: { q: 2, r: 0 } }),
            piece({ id: 'enemy_1', side: 'enemy', position: { q: 8, r: 0 } }),
          ]),
        ],
      }),
    );
    // enemy_1 only exists at tick 1 (the last frame) — no track, no
    // @keyframes reference for it, but ally_1 (present at every frame)
    // still gets one.
    expect(html).not.toContain('data-piece-id="enemy_1"] { animation');
    expect(html).toMatch(/data-piece-id="ally_1"\] \{ animation/);
  });

  it('carries a piece\'s per-tick cues/facing through to the generated cue grammar (WP3 end-to-end)', () => {
    const html = renderCombatBoard(
      view({
        frames: [
          frame(0, [
            piece({ id: 'ally_1', position: { q: 5, r: 0 }, facing: { q: 1, r: 0 }, cues: ['attack'] }),
          ]),
          frame(1, [piece({ id: 'ally_1', position: { q: 5, r: 0 } })]),
        ],
      }),
    );
    // Identical position at both ticks -> span 0 on both axes -> the base
    // (pre-cue) offset is 0 at every stop, so the lunge stop's only
    // non-zero component is the attack contribution itself: hex facing
    // (q=1, r=0) converted to its flat-top screen vector (1.5, sqrt(3)/2),
    // normalized to (0.8660254, 0.5), and scaled by the fixed lunge
    // magnitude (4) -> (3.4641, 2). This is a changed expected value from
    // the pre-hex `facing: { x: 1, y: 0 }` fixture (which used to add +4 on
    // x only, since a cartesian unit vector along x needs no axis
    // conversion) — the hex direction (1, 0) is not itself a screen unit
    // vector, so both components of the lunge now carry contribution.
    expect(html).toMatch(/50% \{ translate: calc\(-50% \+ 3\.4641cqw\) calc\(-50% \+ 2cqh\)/);
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

  // -------------------------------------------------------------------------
  // Wave 3 Step 1d-3 — WP4: each row's reveal timing is entry.tick *
  // tick_millis (I6). No aria-live, no reordering, DOM never pruned.
  // -------------------------------------------------------------------------
  it('sets animation-delay to entry.tick * tick_millis for each row', () => {
    const html = renderCombatLog(
      view({
        tick_millis: 250,
        core_log: [logEntry({ tick: 0 }), logEntry({ tick: 3 }), logEntry({ tick: 7 })],
      }),
    );
    expect(html).toContain('style="animation-delay: 0ms"');
    expect(html).toContain('style="animation-delay: 750ms"');
    expect(html).toContain('style="animation-delay: 1750ms"');
  });

  it('anchors log reveal to the first frame tick so the board and the log share one origin', () => {
    // 실측 데이터의 첫 프레임 tick은 0이 아니라 1이다. 보드는 프레임 인덱스
    // k를 k × tick_millis에 놓으므로 tick 1이 0ms다. 로그를 entry.tick ×
    // tick_millis로 놓으면 같은 사건이 보드보다 한 tick 늦게 나타난다.
    const html = renderCombatLog(
      view({
        tick_millis: 100,
        frames: [frame(1, [piece()]), frame(2, [piece()]), frame(3, [piece()])],
        core_log: [logEntry({ tick: 1 }), logEntry({ tick: 3 })],
      }),
    );
    expect(html).toContain('style="animation-delay: 0ms"');
    expect(html).toContain('style="animation-delay: 200ms"');
    expect(html).not.toContain('style="animation-delay: 100ms"');
    expect(html).not.toContain('style="animation-delay: 300ms"');
  });

  it('never produces a negative reveal delay when a log tick precedes the first frame', () => {
    const html = renderCombatLog(
      view({
        tick_millis: 100,
        frames: [frame(5, [piece()]), frame(6, [piece()])],
        core_log: [logEntry({ tick: 2 })],
      }),
    );
    expect(html).toContain('style="animation-delay: 0ms"');
    expect(html).not.toMatch(/animation-delay: -/);
  });

  it('keeps core_log array order (sequence order) even when reveal delays are computed per row', () => {
    const html = renderCombatLog(
      view({
        tick_millis: 100,
        core_log: [
          logEntry({ tick: 2, sequence: 0, actor_id: 'first_in_sequence' }),
          logEntry({ tick: 2, sequence: 1, actor_id: 'second_in_sequence' }),
        ],
      }),
    );
    expect(html.indexOf('first_in_sequence')).toBeLessThan(html.indexOf('second_in_sequence'));
  });

  it('never adds aria-live anywhere in the log region', () => {
    const html = renderCombatLog(
      view({ core_log: [logEntry(), logEntry({ tick: 5 })] }),
    );
    expect(html).not.toContain('aria-live');
  });

  // -------------------------------------------------------------------------
  // WP2 — meta line wording. I2: the log-region meta line must not claim the
  // full log is readable while the fight is still in progress (no `report`
  // yet); once `report` is present, it should point at the new viewer below.
  // -------------------------------------------------------------------------
  it('says the full log is readable once the fight has ended', () => {
    const html = renderCombatLog(view({ full_log: [logEntry()] }), true);
    expect(html).toContain('전체 로그 1건');
    expect(html).toContain('열람');
    expect(html).not.toContain('이 화면은 개수만 표시');
  });

  it('does not claim the full log is readable while the fight is still in progress', () => {
    const html = renderCombatLog(view({ full_log: [logEntry()] }), false);
    expect(html).toContain('전체 로그 1건');
    expect(html).not.toContain('이 화면은 개수만 표시');
    // No claim that a viewer exists right now.
    expect(html).not.toContain('아래');
  });

  it('defaults to the not-yet-readable wording when the reportPresent flag is omitted', () => {
    const html = renderCombatLog(view({ full_log: [logEntry()] }));
    expect(html).not.toContain('아래');
  });
});

// ---------------------------------------------------------------------------
// WP1 — 전체 로그(`full_log`) 열람 섹션. 정본 07/13: 전투 종료 뒤에만
// 열람 가능(I2). `view.full_log`만 읽는다(I1) — core_log/resolution/
// execution 레벨에는 접근하지 않는다. 상한 없음(I4). importance는 데이터
// 그대로 쓴다(I5). core_log와의 중복을 `data-in-core-log`로 드러낸다(I6).
// 문장은 combatLogTemplateLine 재사용(I3).
// ---------------------------------------------------------------------------
describe('renderCombatFullLog', () => {
  it('shows every full_log row with no cap, even past the 40-row core-log limit', () => {
    const fullLog = Array.from({ length: 64 }, (_, i) =>
      logEntry({ actor_id: `ally_${i}`, sequence: i }),
    );
    const html = renderCombatFullLog(view({ full_log: fullLog }));
    const rows = html.match(/class="combat-full-log__row"/g) ?? [];
    expect(rows.length).toBe(64);
    expect(html).not.toContain('생략');
  });

  it('includes the full_log count in the <summary>', () => {
    const html = renderCombatFullLog(view({ full_log: [logEntry(), logEntry(), logEntry()] }));
    expect(html).toContain('<summary>');
    expect(html).toContain('전체 로그 3건');
  });

  it('shows all three importance levels with data-importance and their canon Korean labels', () => {
    const html = renderCombatFullLog(
      view({
        full_log: [
          logEntry({ importance: 'routine', actor_id: 'a1' }),
          logEntry({ importance: 'important', actor_id: 'a2' }),
          logEntry({ importance: 'decisive', actor_id: 'a3' }),
        ],
      }),
    );
    expect(html).toContain('data-importance="routine"');
    expect(html).toContain('data-importance="important"');
    expect(html).toContain('data-importance="decisive"');
    expect(html).toContain('일반');
    expect(html).toContain('중요');
    expect(html).toContain('결정적');
  });

  it('pairs each row\'s Korean label with that row\'s own importance', () => {
    // 위 테스트는 세 라벨이 html 어딘가에 있는지만 본다 — legend 문구가
    // "중요·결정적…"을 담고 있어서, 모든 줄이 `일반`으로 잘못 찍혀도 통과한다.
    // 라벨은 그 줄 **안에서** 확인해야 한다.
    const html = renderCombatFullLog(
      view({
        full_log: [
          logEntry({ importance: 'routine', actor_id: 'a1' }),
          logEntry({ importance: 'important', actor_id: 'a2' }),
          logEntry({ importance: 'decisive', actor_id: 'a3' }),
        ],
      }),
    );
    const rowFor = (importance: string) =>
      new RegExp(`<li[^>]*data-importance="${importance}"[^>]*>.*?</li>`).exec(html)?.[0] ?? '';
    expect(rowFor('routine')).toContain('일반');
    expect(rowFor('routine')).not.toContain('결정적');
    expect(rowFor('important')).toContain('중요');
    expect(rowFor('important')).not.toContain('일반');
    expect(rowFor('decisive')).toContain('결정적');
    expect(rowFor('decisive')).not.toContain('일반');
  });

  it('marks important/decisive rows as also present in the core log, but never routine rows', () => {
    const html = renderCombatFullLog(
      view({
        full_log: [
          logEntry({ importance: 'routine', actor_id: 'a1' }),
          logEntry({ importance: 'important', actor_id: 'a2' }),
          logEntry({ importance: 'decisive', actor_id: 'a3' }),
        ],
      }),
    );
    const routineRow = /<li[^>]*data-importance="routine"[^>]*>/.exec(html);
    const importantRow = /<li[^>]*data-importance="important"[^>]*>/.exec(html);
    const decisiveRow = /<li[^>]*data-importance="decisive"[^>]*>/.exec(html);
    expect(routineRow).not.toBeNull();
    expect(importantRow).not.toBeNull();
    expect(decisiveRow).not.toBeNull();
    expect(routineRow![0]).not.toContain('data-in-core-log');
    expect(importantRow![0]).toContain('data-in-core-log="true"');
    expect(decisiveRow![0]).toContain('data-in-core-log="true"');
  });

  it('states the core-log correspondence once, not on every row', () => {
    // 64줄 중 절반이 `중요`/`결정적`이다. 줄마다 "핵심 로그에도 있음"을 붙이면
    // 그 절반이 두 줄로 늘어나 목록을 훑을 수 없게 되고, 중요도 칩이 이미
    // 말하는 것을 반복하는 것뿐이다 (core_log는 정확히 importance >= 중요).
    const fullLog = Array.from({ length: 8 }, (_, i) =>
      logEntry({ sequence: i, importance: i % 2 === 0 ? 'routine' : 'important' }),
    );
    const html = renderCombatFullLog(view({ full_log: fullLog }));
    const legendCount = (html.match(/combat-full-log__legend/g) ?? []).length;
    expect(legendCount).toBe(1);
    expect(html).toContain('핵심 로그에도 나온 줄');
    // 줄 단위 배지가 다시 들어오면 잡는다.
    expect(html).not.toContain('핵심 로그에도 있음');
    expect(html).not.toContain('combat-full-log__core-flag');
  });

  it('shows tick and sequence for each row', () => {
    const html = renderCombatFullLog(view({ full_log: [logEntry({ tick: 8, sequence: 2 })] }));
    expect(html).toContain('t8·2');
  });

  it('sentences reuse combatLogTemplateLine — identical wording to the core log', () => {
    const html = renderCombatFullLog(
      view({ full_log: [logEntry({ template_id: 'combat.log.move_intent', actor_id: 'ally_9' })] }),
    );
    expect(html).toContain('ally_9 이동 의도');
  });

  it('surfaces an unknown template_id row instead of dropping it, with a visible marker', () => {
    const html = renderCombatFullLog(
      view({ full_log: [logEntry({ template_id: 'combat.log.made_up_event' })] }),
    );
    expect(html).toContain('data-log-unknown="true"');
    expect(html).toContain('combat.log.made_up_event');
  });

  it('escapes actor ids containing markup', () => {
    const html = renderCombatFullLog(
      view({ full_log: [logEntry({ actor_id: '<script>alert(1)</script>' })] }),
    );
    expect(html).not.toContain('<script>alert(1)</script>');
    expect(html).toContain('&lt;script&gt;');
  });

  it('uses <details>/<summary> — no custom toggle markup', () => {
    const html = renderCombatFullLog(view({ full_log: [logEntry()] }));
    expect(html).toContain('<details');
    expect(html).toContain('data-region="combat-full-log"');
    expect(html).toContain('<ol class="combat-full-log__list">');
  });
});

describe('renderCombatStage — full log viewer entry point (I2)', () => {
  it('omits the combat-full-log section entirely when report is absent (fight in progress)', () => {
    const page: CombatSpectatorPage = {
      view: view({ full_log: [logEntry()] }),
    };
    const html = renderCombatStage(page);
    expect(html).not.toContain('data-region="combat-full-log"');
  });

  it('includes the combat-full-log section once the fight has ended (report present)', () => {
    const page: CombatSpectatorPage = {
      view: view({ full_log: [logEntry()] }),
      report: baseReport(),
    };
    const html = renderCombatStage(page);
    expect(html).toContain('data-region="combat-full-log"');
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

  it('the log meta line only says the log is readable once report is present (WP2, I2)', () => {
    const inProgress = renderCombatStage({
      view: view({ frames: [frame(1, [piece()])], full_log: [logEntry()] }),
    });
    const ended = renderCombatStage({
      view: view({ frames: [frame(1, [piece()])], full_log: [logEntry()] }),
      report: baseReport(),
    });
    expect(inProgress).not.toContain('이 화면은 개수만 표시');
    expect(ended).toContain('열람');
  });
});

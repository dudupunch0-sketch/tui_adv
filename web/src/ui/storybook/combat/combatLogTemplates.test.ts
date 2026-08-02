import { describe, expect, it } from 'vitest';

import type { CombatSpectatorLogEntry } from '../../../core/types';
import { combatLogTemplateLine, roundHundredthsToInt } from './combatLogTemplates';

function entry(overrides: Partial<CombatSpectatorLogEntry> = {}): CombatSpectatorLogEntry {
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

describe('combatLogTemplateLine', () => {
  // -- 6 template ids, character-for-character match with
  // crates/escape-terminal/src/snapshot.rs::combat_log_template_line --

  it('move_intent: with and without target', () => {
    expect(combatLogTemplateLine(entry({ template_id: 'combat.log.move_intent' }))).toBe(
      'ally_1 이동 의도',
    );
    expect(
      combatLogTemplateLine(
        entry({ template_id: 'combat.log.move_intent', target_id: 'enemy_1' }),
      ),
    ).toBe('ally_1 이동 의도 (목표 enemy_1)');
  });

  it('target_selection: with and without target', () => {
    expect(
      combatLogTemplateLine(entry({ template_id: 'combat.log.target_selection' })),
    ).toBe('ally_1 목표 지정 (대상 없음)');
    expect(
      combatLogTemplateLine(
        entry({ template_id: 'combat.log.target_selection', target_id: 'enemy_1' }),
      ),
    ).toBe('ally_1 → 목표 지정: enemy_1');
  });

  it('collision: with and without target', () => {
    expect(combatLogTemplateLine(entry({ template_id: 'combat.log.collision' }))).toBe(
      'ally_1 충돌 (대상 없음)',
    );
    expect(
      combatLogTemplateLine(entry({ template_id: 'combat.log.collision', target_id: 'enemy_1' })),
    ).toBe('ally_1 × enemy_1 충돌');
  });

  it('damage_applied: with and without target, rounds value_hundredths', () => {
    expect(
      combatLogTemplateLine(
        entry({ template_id: 'combat.log.damage_applied', value_hundredths: 1050 }),
      ),
    ).toBe('ally_1 피해 11 (대상 없음)');
    expect(
      combatLogTemplateLine(
        entry({
          template_id: 'combat.log.damage_applied',
          target_id: 'enemy_1',
          value_hundredths: 1050,
        }),
      ),
    ).toBe('ally_1 → enemy_1 피해 11');
  });

  it('damage_applied: value_hundredths 1333 rounds to 13', () => {
    expect(
      combatLogTemplateLine(
        entry({
          template_id: 'combat.log.damage_applied',
          target_id: 'enemy_1',
          value_hundredths: 1333,
        }),
      ),
    ).toBe('ally_1 → enemy_1 피해 13');
  });

  it('damage_applied: value_hundredths null shows an explicit "no value" marker, never 0', () => {
    const line = combatLogTemplateLine(
      entry({
        template_id: 'combat.log.damage_applied',
        target_id: 'enemy_1',
        value_hundredths: null,
      }),
    );
    expect(line).toBe('ally_1 → enemy_1 피해 (수치 없음)');
    expect(line).not.toContain('피해 0');
  });

  it('effect_applied: shows effect id, and (효과 id 없음) when effect_id is null', () => {
    expect(
      combatLogTemplateLine(
        entry({
          template_id: 'combat.log.effect_applied',
          target_id: 'enemy_1',
          effect_id: 'burn',
        }),
      ),
    ).toBe('ally_1 → enemy_1 효과 적용 [burn]');
    expect(
      combatLogTemplateLine(
        entry({ template_id: 'combat.log.effect_applied', target_id: 'enemy_1', effect_id: null }),
      ),
    ).toBe('ally_1 → enemy_1 효과 적용 [(효과 id 없음)]');
    expect(
      combatLogTemplateLine(entry({ template_id: 'combat.log.effect_applied', effect_id: null })),
    ).toBe('ally_1 효과 적용 [(효과 id 없음)] (대상 없음)');
  });

  it('effect_applied_hidden: masks the effect id regardless of input', () => {
    const line = combatLogTemplateLine(
      entry({
        template_id: 'combat.log.effect_applied_hidden',
        target_id: 'enemy_1',
        effect_id: 'burn',
      }),
    );
    expect(line).toBe('ally_1 → enemy_1 효과 적용 [정체불명]');
    expect(line).not.toContain('burn');
    expect(
      combatLogTemplateLine(entry({ template_id: 'combat.log.effect_applied_hidden' })),
    ).toBe('ally_1 효과 적용 [정체불명] (대상 없음)');
  });

  it('unknown template_id falls back to a line that exposes the id, never dropped', () => {
    const line = combatLogTemplateLine(
      entry({ template_id: 'combat.log.made_up_event', target_id: 'enemy_1' }),
    );
    expect(line).toContain('combat.log.made_up_event');
    expect(line).toContain('알 수 없는 사건');
  });

  it('unknown template_id without a target still names the missing target', () => {
    const line = combatLogTemplateLine(entry({ template_id: 'combat.log.made_up_event' }));
    expect(line).toContain('(대상 없음)');
    expect(line).toContain('combat.log.made_up_event');
  });
});

describe('roundHundredthsToInt', () => {
  it('rounds half up for both signs, matching Rust round_hundredths_to_int', () => {
    expect(roundHundredthsToInt(1050)).toBe(11);
    expect(roundHundredthsToInt(1049)).toBe(10);
    expect(roundHundredthsToInt(1000)).toBe(10);
    expect(roundHundredthsToInt(-1050)).toBe(-11);
  });
});

import { describe, expect, it } from 'vitest';

import { buildTurn, executeAction } from './actions';
import { evaluateEnding } from './endings';
import { newGame } from './state';
import type { GameState, PlayerState } from './types';
import { renderGameShell } from '../ui/render';

function play(initial: GameState, actions: string[]): GameState {
  return actions.reduce((state, actionId) => executeAction(state, actionId).state, initial);
}

function withPlayer(state: GameState, player: Partial<PlayerState>): GameState {
  return {
    ...state,
    player: {
      ...state.player,
      ...player,
      abilities: {
        ...state.player.abilities,
        ...(player.abilities ?? {}),
      },
    },
  };
}

describe('browser terminal parity routes', () => {
  const routes: Array<{ name: string; initial: GameState; actions: string[]; endingId: string; kind: string }> = [
    {
      name: 'broadcast conquest',
      initial: newGame({ seed: 123 }),
      actions: [
        'choice:check_message',
        'move:dev_office',
        'move:hallway',
        'move:server_room_front',
        'choice:tune_internal_channel',
      ],
      endingId: 'conquest_broadcast_channel',
      kind: 'conquest',
    },
    {
      name: 'emergency stairs escape',
      initial: newGame({ seed: 123 }),
      actions: [
        'choice:check_message',
        'move:dev_office',
        'move:hallway',
        'move:emergency_stairs',
        'choice:align_breathing_floor',
        'choice:solve_distorted_floor',
      ],
      endingId: 'escape_commute',
      kind: 'escape',
    },
    {
      name: 'spatial collapse failure',
      initial: newGame({ seed: 123, locationId: 'emergency_stairs' }),
      actions: ['choice:align_breathing_floor', 'choice:walk_down_wrong_stairs'],
      endingId: 'game_over_spatial_collapse',
      kind: 'failure',
    },
    {
      name: 'truth protocol',
      initial: newGame({ seed: 123 }),
      actions: [
        'choice:search_ex_employee',
        'move:dev_office',
        'move:meeting_room',
        'choice:save_impossible_minutes',
        'move:dev_office',
        'move:hallway',
        'move:security_room',
        'choice:replay_delayed_cctv',
      ],
      endingId: 'truth_isolation_protocol',
      kind: 'truth',
    },
    {
      name: 'rooftop signal escape',
      initial: newGame({ seed: 123 }),
      actions: [
        'choice:check_message',
        'move:dev_office',
        'move:hallway',
        'move:elevator_hall',
        'choice:press_rooftop_button',
        'choice:send_limited_signal',
      ],
      endingId: 'escape_rooftop_signal',
      kind: 'escape',
    },
    {
      name: 'network admin conquest',
      initial: newGame({ seed: 123 }),
      actions: [
        'choice:check_message',
        'move:dev_office',
        'move:hallway',
        'move:server_room_front',
        'choice:follow_cold_air',
        'choice:assume_admin_console',
      ],
      endingId: 'conquest_network_admin',
      kind: 'conquest',
    },
    {
      name: 'security lockdown conquest',
      initial: newGame({ seed: 123, locationId: 'elevator_hall' }),
      actions: [
        'choice:force_elevator_doors',
        'choice:extract_security_override',
        'move:hallway',
        'move:server_room_front',
        'choice:enter_with_security_override',
        'choice:lock_isolation_with_security_override',
      ],
      endingId: 'conquest_security_lockdown',
      kind: 'conquest',
    },
    {
      name: 'parking lot escape',
      initial: newGame({ seed: 123 }),
      actions: [
        'choice:check_message',
        'move:dev_office',
        'move:hallway',
        'move:parking_lot',
        'choice:follow_idling_engine',
        'choice:open_exit_ramp',
      ],
      endingId: 'escape_parking_lot',
      kind: 'escape',
    },
    {
      name: 'lobby revolving door escape',
      initial: newGame({ seed: 123 }),
      actions: [
        'choice:check_message',
        'move:dev_office',
        'move:hallway',
        'move:lobby',
        'choice:print_visitor_badge',
        'choice:scan_visitor_badge',
      ],
      endingId: 'escape_lobby_revolving_door',
      kind: 'escape',
    },
    {
      name: 'executive approval conquest',
      initial: newGame({ seed: 123 }),
      actions: [
        'choice:check_message',
        'move:dev_office',
        'move:hallway',
        'move:lobby',
        'choice:press_executive_call',
        'choice:claim_executive_approval',
      ],
      endingId: 'conquest_executive_approval',
      kind: 'conquest',
    },
    {
      name: 'first reality hint',
      initial: newGame({ seed: 123, locationId: 'printer_area' }),
      actions: ['choice:take_printout', 'move:pantry', 'choice:look_behind_machine'],
      endingId: 'hidden_reality_hint_001',
      kind: 'hidden',
    },
    {
      name: 'second reality hint',
      initial: newGame({ seed: 123, locationId: 'printer_area' }),
      actions: ['choice:check_toner', 'move:pantry', 'choice:trace_toner_symbol'],
      endingId: 'hidden_reality_hint_002',
      kind: 'hidden',
    },
    {
      name: 'third reality hint',
      initial: newGame({ seed: 123, locationId: 'printer_area' }),
      actions: ['choice:read_printout', 'move:dev_office', 'move:meeting_room', 'choice:decode_whiteboard_marker'],
      endingId: 'hidden_reality_hint_003',
      kind: 'hidden',
    },
  ];

  it.each(routes)('reaches $name route ending', ({ initial, actions, endingId, kind }) => {
    const finalState = play(initial, actions);
    const ending = evaluateEnding(finalState);

    expect(ending?.id).toBe(endingId);
    expect(ending?.kind).toBe(kind);
  });
});

describe('browser terminal parity mechanics', () => {
  it('applies turn pressure, unlocks achievements, and exposes usable inventory actions', () => {
    let state = play(newGame({ seed: 123 }), [
      'choice:check_message',
      'move:dev_office',
      'move:supply_closet',
      'choice:take_power_bank',
    ]);

    expect(state.unlockedAchievements).toContain('first_signal_received');
    expect(state.inventory).toContain('power_bank');
    expect(state.player.battery).toBeLessThan(100);

    const actionIds = buildTurn(state).actions.map((action) => action.id);
    expect(actionIds).toContain('use:power_bank');

    state = executeAction(state, 'use:power_bank').state;

    expect(state.inventory).not.toContain('power_bank');
    expect(state.player.battery).toBe(100);
    expect(state.log.at(-1)).toContain('보조배터리');
  });

  it('selects the thirst hallucination encounter and applies terminal resource pressure', () => {
    const thirsty = withPlayer(newGame({ seed: 123, locationId: 'pantry' }), { thirst: 70 });
    const turn = buildTurn(thirsty);

    expect(turn.encounter?.id).toBe('strange_water_dispenser');

    const afterDrink = executeAction(thirsty, 'choice:drink_false_water').state;

    expect(afterDrink.player.sanity).toBe(92);
    expect(afterDrink.player.thirst).toBe(47);
    expect(afterDrink.flags).toContain('thirst_hallucination_seen');
  });

  it('renders low-sanity pressure warnings and distorted fake-TUI choices without changing actions', () => {
    const unstable = withPlayer(newGame({ seed: 123 }), { sanity: 30 });
    const turn = buildTurn(unstable);
    const html = renderGameShell(turn);

    expect(turn.actions[0].id).toBe('choice:check_message');
    expect(html).toContain('집중도가 흔들려 선택지가 부분적으로 왜곡된다');
    expect(html).toContain('메시▒를 확▒한다');
  });

  it('supports ability-check branches from the terminal encounter data', () => {
    const interfaceBuild = withPlayer(newGame({ seed: 123 }), { abilities: { interface: 6 } });
    const turn = buildTurn(interfaceBuild);

    expect(turn.actions.map((action) => action.id)).toContain('choice:trace_packet_delay');

    const traced = executeAction(interfaceBuild, 'choice:trace_packet_delay').state;

    expect(traced.player.battery).toBe(98);
    expect(traced.clues).toContain('delayed_packet_route');
    expect(traced.flags).toContain('network_truth_hint');
    expect(traced.log).toContain('지연 시간 사이에서 숨은 라우팅을 찾았다.');
  });
});

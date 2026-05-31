# 야근몽 encounter situation cards

Status: candidate

이 문서는 `docs/content/storypacks/yageunmong_pack.md`의 후보 인카운터를 runtime YAML 승격 전 상황 카드로 정리한다.

공통 원칙:

- 모든 카드는 `world_id: office_dream`, `storypack_id: yageunmong_pack`에 속한다.
- 현재 단계에서는 runtime encounter가 아니다.
- `isolation_pack` 기본 runtime을 대체하지 않는다.
- 선택지는 세부 수치보다 역할과 결과 hook을 먼저 정의한다.
- 최소 하나의 안전한 관찰/후퇴/fallback 선택지를 둔다.
- 실제 회사명, 실제 직원, 실제 위치, private reality hint는 쓰지 않는다.
- 2026-05-31 live Notion Markdown을 다시 확인했고, 관련 idea_box entry는 이 후보 문서/DB 승격 근거로 done 처리했다.

## 1. `yageunmong_late_night_desk_awake`

```yaml
id: yageunmong_late_night_desk_awake
world_id: office_dream
storypack_id: yageunmong_pack
status: candidate
phase: [late_night_sleep, lucid_dream_awareness]
priority_class: main_forced
location_tags: [desk, late_night, dream_entry]
surface: [office_object, messenger]
anomaly_type: [delayed_time, space_loop]
pressure_type: [sanity, battery]
npc_slots: []
candidate_characters: []
summary: 야근 중 책상에 엎드린 뒤, 멈춘 시간과 퇴근 미승인 알림으로 회사 악몽에 들어왔음을 보여준다.
setup_text: 잠깐 눈을 붙였을 뿐인데 모니터에는 아직 같은 업무 창이 열려 있다. 시계는 같은 분을 반복하고, 메신저에는 `아직 퇴근 승인되지 않았습니다`라는 시스템 알림이 떠 있다.
choice_shapes:
  - id: check_clock_and_body
    role: safe_observe
    expected_costs: []
    expected_gains: [lucid_dream_hint]
  - id: answer_manager_message
    role: social_probe
    expected_costs: [sanity_small]
    expected_gains: [nightmare_office_rule_hint]
  - id: stand_up_from_desk
    role: safe_reposition
    expected_costs: [danger_small]
    expected_gains: [office_loop_started]
outcome_hooks:
  possible_flags: [yageunmong_started, lucid_dream_hint_seen, office_loop_started]
  possible_clues: [clock_repeats_same_minute, clockout_not_approved_message]
  possible_items: [cold_coffee_cup]
main_spine_link: 야근몽의 출발점과 목표가 업무 완료가 아니라 꿈에서 깨어나기임을 세운다.
randomization_notes: main_forced opening beat. 반복 등장 금지.
promotion_notes: 첫 preview 후보. 기존 encounter schema로 title/body/choices/outcome만 사용해 구현 가능하다.
```

## 2. `yageunmong_unapproved_meeting_room_loop`

```yaml
id: yageunmong_unapproved_meeting_room_loop
world_id: office_dream
storypack_id: yageunmong_pack
status: candidate
phase: nightmare_office_loop
priority_class: random_pack
location_tags: [meeting_room, loop, review]
surface: [reservation_panel, meeting_minutes]
anomaly_type: [space_loop, future_record]
pressure_type: [sanity, relation]
npc_slots: [pm_worldline_mediator]
candidate_characters: []
summary: 불이 꺼지지 않는 회의실과 자동 회의록이 같은 검토 루프를 반복한다.
setup_text: 회의실 문을 닫고 나왔는데 다시 같은 회의실 앞이다. 회의록에는 아직 말하지 않은 발언과 이미 끝난 결론이 동시에 적혀 있고, 결론은 늘 `처음부터 다시 검토`다.
choice_shapes:
  - id: leave_room_without_answering
    role: safe_leave
    expected_costs: []
    expected_gains: [loop_distance_small]
  - id: read_repeated_action_items
    role: information_probe
    expected_costs: [sanity_small]
    expected_gains: [meeting_loop_clue]
  - id: add_note_that_meeting_is_dream
    role: high_risk_route_test
    expected_costs: [relation_risk]
    expected_gains: [lucidity_flag]
outcome_hooks:
  possible_flags: [meeting_loop_seen, lucid_note_added]
  possible_clues: [repeated_action_items, future_minutes_line]
  possible_items: [minutes_fragment]
main_spine_link: 업무를 끝내려 할수록 꿈의 루프가 강화된다는 규칙을 보여준다.
randomization_notes: opening 이후 1회성 또는 low weight. 기존 meeting_room route와 충돌하지 않게 flag gating 필요.
promotion_notes: 회의실 runtime encounter와 겹치지 않도록 자각몽/loop clue 중심으로 차별화한다.
```

## 3. `yageunmong_manager_approval_trap`

```yaml
id: yageunmong_manager_approval_trap
world_id: office_dream
storypack_id: yageunmong_pack
status: candidate
phase: [nightmare_office_loop, clockout_gate_refusal]
priority_class: route_key
location_tags: [approval, manager_desk, authority]
surface: [approval_system, organization_chart]
anomaly_type: [permission_denied_as_existence, identity_erasure]
pressure_type: [sanity, relation, danger]
npc_slots: [hr_identity_keeper]
candidate_characters: []
summary: 승인하지 않는 팀장이 퇴근과 존재 판정을 결재/권한 잠금으로 만든다.
setup_text: 결재 화면은 `퇴근 요청: 반려`를 반복한다. 팀장은 일단 진행하라고 말하지만 책임은 주인공에게 남긴다. 조직도에서 주인공의 이름은 담당자 칸에 계속 복제된다.
choice_shapes:
  - id: define_scope_of_responsibility
    role: safe_boundary
    expected_costs: []
    expected_gains: [responsibility_boundary_clue]
  - id: request_approval_again
    role: dark_bargain
    expected_costs: [sanity_medium]
    expected_gains: [approval_trap_confirmed]
  - id: close_approval_window
    role: safe_leave
    expected_costs: [relation_risk]
    expected_gains: [permission_rule_resisted]
outcome_hooks:
  possible_flags: [approval_trap_seen, permission_rule_resisted]
  possible_clues: [clockout_is_not_approved_by_manager, responsibility_scope_boundary]
  possible_items: [rejected_clockout_request]
main_spine_link: 퇴근은 승인받는 것이 아니라 꿈의 권한 규칙을 거부해야 열린다는 핵심을 세운다.
randomization_notes: route_key. 너무 초반에 나오면 결론이 빨라지므로 lucid clue 이후 권장.
promotion_notes: 새 approval UI를 만들지 않고 서술형 encounter로 먼저 구현 가능하다.
```

## 4. `yageunmong_reality_anchor_pantry`

```yaml
id: yageunmong_reality_anchor_pantry
world_id: office_dream
storypack_id: yageunmong_pack
status: candidate
phase: reality_anchor_collection
priority_class: npc_relation
location_tags: [pantry, anchor, sensory]
surface: [office_object, document_viewer]
anomaly_type: [document_contamination, delayed_time]
pressure_type: [sanity, thirst]
npc_slots: [facility_pathfinder]
candidate_characters: []
summary: 식은 커피 냄새와 현실 알람음 같은 public-safe 현실 앵커를 발견한다.
setup_text: 탕비실의 커피는 식었지만 향은 방금 전 같다. 프린터 위 문서에는 업무 지시 대신 `집에 가야 한다`는 문장이 한 줄만 찍혀 있고, 멀리서 현실 알람음 같은 소리가 난다.
choice_shapes:
  - id: breathe_and_name_the_anchor
    role: safe_observe
    expected_costs: []
    expected_gains: [reality_anchor_clue]
  - id: pocket_printed_sentence
    role: information_probe
    expected_costs: [sanity_small]
    expected_gains: [anchor_fragment_item]
  - id: follow_alarm_sound
    role: high_risk_route_test
    expected_costs: [danger_small]
    expected_gains: [clockout_gate_hint]
outcome_hooks:
  possible_flags: [reality_anchor_found, alarm_sound_heard]
  possible_clues: [cold_coffee_anchor, homeward_intent_line]
  possible_items: [anchor_fragment]
main_spine_link: 깨어나는 route가 업무 완료가 아니라 현실 앵커 회복과 연결됨을 보여준다.
randomization_notes: reality_anchor_collection phase에서 1회성. public-safe 감각 단서만 사용한다.
promotion_notes: 기존 pantry resource event와 충돌하지 않도록 dream preview 전용으로 둔다.
```

## 5. `yageunmong_awakening_fragment_choice`

```yaml
id: yageunmong_awakening_fragment_choice
world_id: office_dream
storypack_id: yageunmong_pack
status: candidate
phase: reality_anchor_collection
priority_class: route_key
location_tags: [notebook, growth, lucid_choice]
surface: [document_viewer, approval_system]
anomaly_type: [document_contamination, future_record]
pressure_type: [sanity, relation]
npc_slots: [newcomer_mirror]
candidate_characters: []
summary: 각성편린 3택 후보가 나타나고, 하나의 현실 감각만 붙잡을 수 있다.
setup_text: 문서 편집창의 빈 줄에 세 문장이 떠오른다. `이건 꿈이다`, `이 일은 내 일이 아니다`, `문을 닫고 나간다`. 하나를 선택하면 나머지 두 줄은 삭제 기록 없이 사라진다.
choice_shapes:
  - id: delay_fragment_choice
    role: safe_delay
    expected_costs: [opportunity_may_fade]
    expected_gains: [fragment_caution]
  - id: choose_this_is_a_dream
    role: lucidity_growth_choice
    expected_costs: [fragment_lockout_two_options]
    expected_gains: [lucidity_thread]
  - id: choose_not_my_work
    role: boundary_growth_choice
    expected_costs: [fragment_lockout_two_options]
    expected_gains: [responsibility_boundary_thread]
  - id: choose_close_the_door
    role: escape_growth_choice
    expected_costs: [fragment_lockout_two_options]
    expected_gains: [clockout_gate_thread]
outcome_hooks:
  possible_flags: [awakening_fragment_seen, first_awakened_fragment_chosen]
  possible_clues: [fragment_is_growth_direction, not_all_work_can_be_done]
  possible_growth_threads: [lucidity_thread, responsibility_boundary_thread, clockout_gate_thread]
main_spine_link: 회사팩의 각성편린이 무협팩의 천외편린과 같은 3택 성장 문법을 공유함을 보여준다.
randomization_notes: 너무 자주 뜨면 안 된다. 현실 앵커 획득 후나 큰 모순 발견 후에만 사용한다.
promotion_notes: 첫 구현에서는 새 reward/ability schema를 열지 말고 flag/clue/log/presentation text로만 처리한다.
```

## 6. `yageunmong_clockout_gate_self`

```yaml
id: yageunmong_clockout_gate_self
world_id: office_dream
storypack_id: yageunmong_pack
status: candidate
phase: [clockout_gate_refusal, wake_resolution]
priority_class: route_key
location_tags: [exit_gate, self_confrontation, ending]
surface: [security_gate, approval_system]
anomaly_type: [permission_denied_as_existence, identity_erasure]
pressure_type: [sanity, danger, relation]
npc_slots: [newcomer_mirror]
candidate_characters: []
summary: 최종 적인 퇴근을 잊은 나 자신과 마주하고, 승인을 기다리지 않는 퇴근 선언으로 깨어난다.
setup_text: 출입 게이트 앞에는 주인공과 같은 얼굴을 한 사람이 서 있다. 그는 말한다. “조금만 더 하면 돼.” 게이트 화면에는 `퇴근 승인 대기`가 깜빡이고, 뒤쪽 회의실에서는 아직도 누군가 이름을 부른다.
choice_shapes:
  - id: breathe_before_answering_self
    role: safe_observe
    expected_costs: []
    expected_gains: [final_self_recognition]
  - id: promise_to_finish_one_more_task
    role: dark_bargain
    expected_costs: [endless_overtime_risk]
    expected_gains: [loop_failure_hint]
  - id: declare_clockout_without_approval
    role: route_commit_escape
    expected_costs: [relation_or_guilt_cost]
    expected_gains: [wake_route_flag]
  - id: call_coworker_to_gate
    role: npc_relation
    expected_costs: [route_delay]
    expected_gains: [coworker_rescue_thread]
outcome_hooks:
  possible_flags: [clockout_gate_seen, wake_route_flag, self_confronted]
  possible_route_flags: [wake_up_route_started, coworker_rescue_route_started]
  possible_clues: [permission_is_the_trap, work_never_completes]
main_spine_link: 야근몽의 결말 목표가 업무 완료가 아니라 꿈의 핵심 규칙을 거부하는 것임을 닫는다.
randomization_notes: final route card. 충분한 앵커/각성편린 후에만 사용한다.
promotion_notes: 첫 runtime prototype 후보는 아니다. route/ending preview가 생긴 뒤 검토한다.
```

# 야근몽 encounter situation cards

Status: candidate

이 문서는 `docs/content/storypacks/yageunmong_pack.md`의 후보 인카운터를 runtime YAML 승격 전 상황 카드로 정리한다. 2026-06-02 확장에서는 이구학지 — 천기록의 storypack/encounter 문서 구조를 참조해 야근몽에도 route-pressure, 동료 구출 bridge, 후일담 후보, preview handoff를 추가했다.

공통 원칙:

- 모든 카드는 `world_id: office_dream`, `storypack_id: yageunmong_pack`에 속한다.
- 현재 단계에서는 runtime encounter가 아니다.
- `isolation_pack` 기본 runtime을 대체하지 않는다.
- 선택지는 세부 수치보다 역할과 결과 hook을 먼저 정의한다.
- 최소 하나의 안전한 관찰/후퇴/fallback 선택지를 둔다.
- 실제 회사명, 실제 직원, 실제 위치, private reality hint는 쓰지 않는다.
- 2026-06-02 live Notion Markdown을 다시 확인했고, 원문은 상위 컨셉 reference로 유지한다. 아직 야근몽 전용 Notion 사건 카드 DB는 확인되지 않았다.
- 이구학지처럼 runtime 승격은 별도 preview mode를 우선한다. 기본 `isolation_pack` office runtime, `escape-office` save/localStorage key, 새 reward/epilogue schema는 건드리지 않는다.
- 각성편린, 동료 구출, route split, 후일담은 매력적인 확장점이지만, 첫 구현에서는 flags/clues/log/presentation text로만 표현한다.

## 2026-06-02 Notion source mapping

| Notion source | repo use | status |
|---|---|---|
| `회사 스토리팩: 야근몽` / `36f37e69-695e-8113-9dd5-ddc56a633226` | 핵심 컨셉, 시작 장면, 자각몽 목표, 주요 적/장소/각성편린/엔딩 방향 | live checked 2026-06-02 |
| 야근몽 전용 사건 카드 DB | none yet | future source if created |
| 야근몽 전용 후일담 카드 DB | none yet | future source if created |

Repo 확장 기준:

- `yageunmong_late_night_desk_awake`, `yageunmong_unapproved_meeting_room_loop`, `yageunmong_awakening_fragment_choice`, `yageunmong_clockout_gate_self`는 Notion 원문에 직접 대응한다.
- `yageunmong_coworker_meeting_room_rescue`, `yageunmong_clockout_gate_route_split`, `yageunmong_wake_desk_aftermath`는 Notion의 동료 구출/퇴근 선언/현실 회복 방향을 이구학지식 bridge/route/afterword 카드로 확장한 repo 후보이다.
- `yageunmong_unread_mail_wall`, `yageunmong_dead_project_server_log`, `yageunmong_elevator_unapproved_floor`는 Notion의 읽지 않는 메일, 과거 프로젝트 서버룸, 아무도 없는 엘리베이터 장소를 상황 카드로 분리한 repo 후보이다.

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

## 7. `yageunmong_unread_mail_wall`

```yaml
id: yageunmong_unread_mail_wall
world_id: office_dream
storypack_id: yageunmong_pack
status: candidate
phase: nightmare_office_loop
priority_class: random_pack
location_tags: [corridor, mail, notification]
surface: [messenger, document_viewer]
anomaly_type: [document_contamination, space_loop]
pressure_type: [sanity, battery]
npc_slots: [pm_worldline_mediator]
candidate_characters: []
summary: 읽지 않은 메일과 알림이 복도 벽처럼 쌓이고, 모두 처리하려 할수록 길이 막힌다.
purpose: Notion 원문의 `읽지 않는 메일의 유령`을 하나의 압박 카드로 분리한다. 핵심은 모든 알림에 반응하는 것이 해결이 아니라는 사실을 보여주는 것이다.
setup_text: 복도 끝에 메일 제목들이 종이처럼 겹쳐 벽을 만든다. 긴급, 재송부, 확인 요청, 방금 보낸 메일이 같은 시간으로 찍혀 있고, 열지 않은 알림은 주인공의 이름을 점점 크게 부른다.
choice_shapes:
  - id: turn_off_noncritical_notifications
    role: safe_boundary
    fallback_choice: true
    expected_costs: []
    expected_gains: [notification_boundary_clue]
  - id: open_every_unread_mail
    role: dark_bargain
    expected_costs: [sanity_medium, battery_medium]
    expected_gains: [mail_wall_rule_confirmed]
  - id: search_for_one_real_message
    role: information_probe
    expected_costs: [sanity_small]
    expected_gains: [reality_anchor_hint]
outcome_hooks:
  possible_flags: [mail_wall_seen, notification_boundary_set, mail_wall_rule_confirmed]
  possible_clues: [not_every_message_matters, noise_hides_anchor_message]
  possible_items: [unread_mail_header_fragment]
main_spine_link: 모든 요청에 답하는 것이 아니라 소음과 중요한 신호를 구분해야 깨어나는 route가 열린다는 규칙을 보여준다.
randomization_notes: messenger/document route에서 1회성. opening 직후보다 lucid clue 이후가 적절하다.
promotion_notes: 새 notification UI 없이 encounter body/choice/outcome으로 구현 가능하다.
```

## 8. `yageunmong_dead_project_server_log`

```yaml
id: yageunmong_dead_project_server_log
world_id: office_dream
storypack_id: yageunmong_pack
status: candidate
phase: [nightmare_office_loop, reality_anchor_collection]
priority_class: npc_relation
location_tags: [server_room, dead_project, guilt]
surface: [build_log, document_viewer]
anomaly_type: [future_record, document_contamination]
pressure_type: [sanity, relation]
npc_slots: [pm_worldline_mediator]
candidate_characters: []
summary: 끝난 프로젝트의 서버 로그가 계속 재생되고, 실패 기억과 현실 앵커가 같은 파일에 섞인다.
purpose: 과거 프로젝트 서버룸을 죄책감 압박과 성장 hook으로 분리한다. 실제 회사/장애/고객 정보처럼 보이는 세부사항은 쓰지 않는다.
setup_text: 서버룸 콘솔에는 이미 닫힌 프로젝트의 빌드 로그가 반복된다. 실패 줄마다 주인공 이름이 붙지만, 중간에 한 줄만 낯설다. `저장하고 닫아도 된다`.
choice_shapes:
  - id: copy_only_the_anchor_line
    role: safe_observe
    fallback_choice: true
    expected_costs: []
    expected_gains: [reality_anchor_clue]
  - id: rerun_the_dead_build
    role: dark_bargain
    expected_costs: [sanity_medium]
    expected_gains: [guilt_loop_confirmed]
  - id: mark_failure_as_record_not_sentence
    role: reflection_growth_choice
    expected_costs: [sanity_small]
    expected_gains: [failure_record_thread]
outcome_hooks:
  possible_flags: [dead_project_log_seen, failure_record_thread, guilt_loop_confirmed]
  possible_clues: [failure_is_record_not_sentence, save_and_close_anchor_line]
  possible_items: [server_log_anchor_fragment]
main_spine_link: 실패를 영원히 수습해야 한다는 악몽 규칙을, 기록하고 닫을 수 있는 것으로 바꾼다.
randomization_notes: server/build_log 계열에서 1회성. 현실 프로젝트를 연상시키는 고유명사 사용 금지.
promotion_notes: 이구학지의 `실패 기록` 편린과 대응되는 회사팩 회복 카드 후보. 새 growth schema 없이 flag/clue만 사용한다.
```

## 9. `yageunmong_coworker_meeting_room_rescue`

```yaml
id: yageunmong_coworker_meeting_room_rescue
world_id: office_dream
storypack_id: yageunmong_pack
status: candidate
phase: [reality_anchor_collection, clockout_gate_refusal]
priority_class: npc_relation
location_tags: [meeting_room, coworker, rescue]
surface: [meeting_minutes, office_object]
anomaly_type: [space_loop, delayed_time]
pressure_type: [sanity, relation]
npc_slots: [facility_pathfinder]
candidate_characters: []
summary: 회의실에 갇힌 동료에게 업무 완료가 아니라 꿈 인식과 현실 앵커를 공유한다.
purpose: 이구학지의 `wuxia_seo_harin_rescue`가 구조/관계 bridge라면, 이 카드는 야근몽의 동료 구출 bridge다. 동료 시스템 schema를 열지 않고 rescue thread만 남긴다.
setup_text: 불이 꺼지지 않는 회의실 안에서 한 사람이 같은 문장을 반복한다. “이거 끝나면 나갈 수 있겠지?” 회의록은 그의 이름을 참석자, 액션 아이템, 미완료 사유에 동시에 적고 있다.
choice_shapes:
  - id: tell_coworker_this_is_a_dream
    role: safe_honesty
    fallback_choice: true
    expected_costs: [sanity_small]
    expected_gains: [coworker_lucidity_thread]
  - id: finish_coworkers_action_items
    role: dark_bargain
    expected_costs: [sanity_medium]
    expected_gains: [rescue_trap_confirmed]
  - id: share_reality_anchor
    role: npc_relation
    expected_costs: [anchor_attention_risk]
    expected_gains: [anchor_shared_with_coworker]
  - id: leave_door_open_without_solving_minutes
    role: safe_leave
    expected_costs: [relation_risk]
    expected_gains: [meeting_room_rescue_route]
outcome_hooks:
  possible_flags: [coworker_meeting_room_seen, coworker_lucidity_thread, anchor_shared_with_coworker, meeting_room_rescue_route]
  possible_clues: [coworker_cannot_be_saved_by_work, open_door_matters_more_than_minutes]
  possible_items: [shared_anchor_note]
  possible_relations: [coworker_rescue_thread]
main_spine_link: 야근몽의 동료 구출 엔딩 후보를 열고, 구출이 업무 대행이 아니라 꿈 규칙을 함께 거부하는 것임을 보여준다.
randomization_notes: 회의실 루프 카드 이후 1회성 bridge로 적합하다. 동료를 실제 직원처럼 특정하지 않는다.
promotion_notes: 첫 야근몽 preview chain의 중간 후보. companion/relation schema 없이 flags/clues/log로만 구현한다.
```

## 10. `yageunmong_elevator_unapproved_floor`

```yaml
id: yageunmong_elevator_unapproved_floor
world_id: office_dream
storypack_id: yageunmong_pack
status: candidate
phase: clockout_gate_refusal
priority_class: route_key
location_tags: [elevator, exit_hint, unapproved_floor]
surface: [security_gate, office_object]
anomaly_type: [space_loop, permission_denied_as_existence]
pressure_type: [sanity, danger]
npc_slots: []
candidate_characters: []
summary: 엘리베이터가 미승인층과 퇴근층을 번갈아 표시하며, 출구가 결재가 아니라 규칙 거부에 묶여 있음을 드러낸다.
purpose: Notion 원문의 아무도 없는 엘리베이터를 퇴근 게이트 전조 카드로 구체화한다.
setup_text: 엘리베이터에는 B1, 13F, 0F, 미승인층, 퇴근층이 번갈아 뜬다. 닫힘 버튼은 작동하지 않고, 열림 버튼 옆에는 `상위 승인 필요`라는 라벨이 붙어 있다.
choice_shapes:
  - id: step_out_before_floor_is_approved
    role: safe_leave
    fallback_choice: true
    expected_costs: [danger_small]
    expected_gains: [permission_rule_resisted]
  - id: wait_for_approval_floor
    role: dark_bargain
    expected_costs: [sanity_medium]
    expected_gains: [approval_wait_loop_confirmed]
  - id: press_clockout_floor_without_badge
    role: high_risk_route_test
    expected_costs: [danger_medium]
    expected_gains: [clockout_gate_hint]
outcome_hooks:
  possible_flags: [unapproved_floor_seen, approval_wait_loop_confirmed, clockout_floor_glimpsed]
  possible_clues: [elevator_permission_is_trap, clockout_floor_exists]
  possible_items: []
main_spine_link: 최종 게이트가 물리적 문이 아니라 승인 규칙을 거부할 때 보이는 출구임을 예고한다.
randomization_notes: 충분한 현실 앵커/각성편린 이후에만 등장. 너무 이르면 최종 규칙을 빨리 노출한다.
promotion_notes: route_key 후보지만 첫 preview에서는 후순위. 새 floor/elevator schema 없이 encounter로 처리한다.
```

## 11. `yageunmong_clockout_gate_route_split`

```yaml
id: yageunmong_clockout_gate_route_split
world_id: office_dream
storypack_id: yageunmong_pack
status: candidate
phase: [clockout_gate_refusal, wake_resolution]
priority_class: route_key
location_tags: [exit_gate, route_split, coworker_rescue]
surface: [security_gate, approval_system]
anomaly_type: [permission_denied_as_existence, identity_erasure]
pressure_type: [sanity, danger, relation]
npc_slots: [newcomer_mirror, facility_pathfinder]
candidate_characters: []
summary: 퇴근 게이트 앞에서 혼자 깨어남, 동료 구출, 무한 야근 루프의 route pressure가 처음 갈라진다.
purpose: 이구학지의 route split 문법을 야근몽에 맞춘다. multi-ending schema를 열지 않고 route flags만 남긴다.
setup_text: 퇴근 게이트는 열린 듯 보이지만, 뒤쪽 회의실에서는 아직 누군가 이름을 부른다. 게이트 화면에는 세 줄이 동시에 떠 있다. `혼자 퇴근`, `동료 확인`, `업무 계속`.
choice_shapes:
  - id: wake_alone_now
    role: route_commit_escape
    expected_costs: [relation_or_guilt_cost]
    expected_gains: [wake_up_route_started]
  - id: return_for_coworker_anchor
    role: npc_relation
    expected_costs: [route_delay, danger_small]
    expected_gains: [coworker_rescue_route_started]
  - id: continue_work_until_clear
    role: dark_bargain
    expected_costs: [endless_overtime_risk]
    expected_gains: [loop_failure_hint]
  - id: breathe_and_check_gate_rule
    role: safe_observe
    fallback_choice: true
    expected_costs: []
    expected_gains: [permission_is_the_trap]
outcome_hooks:
  possible_flags: [clockout_gate_route_split_seen, wake_up_route_started, coworker_rescue_route_started, endless_overtime_risk_seen]
  possible_route_flags: [wake_up_route_started, coworker_rescue_route_started, loop_failure_route_marked]
  possible_clues: [permission_is_the_trap, coworker_voice_behind_gate, work_never_completes]
main_spine_link: 야근몽의 결말 후보를 한 번에 구현하지 않고 route hook으로 분리한다.
randomization_notes: `clockout_gate_seen` 또는 충분한 anchor/thread 이후에만 사용. final encounter와 중복되지 않도록 resolved flag 필요.
promotion_notes: route graph/ending schema 없이 flags/clues/log로만 preview 가능하다. 후일담 후보는 separate 문서/카드에서 정산한다.
```

## 12. `yageunmong_wake_desk_aftermath`

```yaml
id: yageunmong_wake_desk_aftermath
world_id: office_dream
storypack_id: yageunmong_pack
status: candidate
phase: wake_resolution
priority_class: ambient
location_tags: [desk, aftermath, recovery]
surface: [office_object, messenger]
anomaly_type: [delayed_time, document_contamination]
pressure_type: [sanity, relation]
npc_slots: []
candidate_characters: []
summary: 깨어난 뒤 아직 남은 업무와 닫은 노트북, 기억하지 못하는 동료의 잔상을 정산한다.
purpose: 야근몽의 회복 서사를 후일담 카드 후보로 남긴다. 이 카드는 엔딩 renderer/schema가 생기기 전까지 docs-only afterword source다.
setup_text: 눈을 뜨자 현실에서는 얼마 지나지 않았다. 모니터에는 아직 처리하지 않은 업무가 남아 있다. 메신저 알림은 켜져 있지만, 이번에는 노트북을 닫을 수 있다. 다음 날 동료는 꿈을 기억하지 못하는 듯하다가 회의실 문 이야기를 조용히 꺼낸다.
choice_shapes:
  - id: close_laptop_and_leave
    role: safe_leave
    fallback_choice: true
    expected_costs: []
    expected_gains: [clockout_declaration_afterword]
  - id: write_one_boundary_note
    role: reflection_growth_choice
    expected_costs: [sanity_small]
    expected_gains: [boundary_afterword]
  - id: ask_coworker_about_meeting_room_dream
    role: npc_relation
    expected_costs: [awkwardness_small]
    expected_gains: [coworker_trace_afterword]
outcome_hooks:
  possible_flags: [wake_aftermath_seen, laptop_closed_after_wake, boundary_afterword, coworker_trace_afterword]
  possible_clues: [work_remains_but_can_be_left, dream_trace_in_coworker_memory]
  possible_items: [boundary_note]
main_spine_link: 성공이 업무 제거가 아니라 업무가 남아 있어도 나갈 수 있는 회복 선언임을 정산한다.
randomization_notes: ending/afterword 후보. 일반 random encounter로 쓰지 않는다.
promotion_notes: epilogue renderer/schema가 생기기 전까지 docs-only. 첫 runtime에서는 ending text나 `ScenePage.mode: ending` body로만 고려한다.
```

# 차원격리팩 인카운터 상황 카드

Status: candidate

이 문서는 `isolation_pack`의 첫 상황 카드 6개를 정리한다.

이 카드는 아직 runtime encounter가 아니다. 각 카드는 나중에 `src/tui_adv/data/encounters.yaml` 또는 별도 storypack DB로 승격할 수 있는 후보 설계다.

## 요약

| id | phase | surface | anomaly | NPC slot | 승격 우선순위 |
|---|---|---|---|---|---|
| `isolation_channel_mismatched_floor` | `opening_absence` | messenger | mismatched_floor | infra_interpreter | 높음 |
| `org_chart_missing_employee` | `normal_operations` | organization_chart | identity_erasure | hr_identity_keeper | 높음 |
| `delayed_cctv_next_action` | `isolation_rules` | cctv | delayed_time / future_record | security_witness | 높음 |
| `server_log_other_branch` | `isolation_rules` | build_log | worldline_branch | infra_interpreter | 중간 |
| `automatic_minutes_no_attendees` | `normal_operations` | meeting_minutes | absent_people / future_record | hr_identity_keeper | 중간 |
| `pantry_survivor_trace` | `opening_absence` | office_object | absent_people | facility_pathfinder 또는 none | 낮음 |

---

## `isolation_channel_mismatched_floor`

```yaml
id: isolation_channel_mismatched_floor
status: candidate
storypack_id: isolation_pack
phase: opening_absence
priority_class: random_pack
location_tags: [office, messenger, intranet]
surface: messenger
anomaly_type: mismatched_floor
pressure_type: [sanity, battery]
npc_slots: [infra_interpreter]
candidate_characters: [park_doyoon]
summary: 격리자 임시대응방에서 같은 회사에 있다는 사람들이 서로 다른 층수와 날씨를 보고한다.
setup_text: 사내 메신저에 `#격리자-임시대응방` 초대가 뜬다. 참가자들은 모두 같은 시간에 접속했지만, 누군가는 7층 비상등을 말하고 누군가는 옥상 비를 말한다. 박도윤은 채널 라우팅이 회사 내부망을 한 번 더 돌아온다고 적는다.
choice_shapes:
  - id: read_silently
    role: safe_observe
    expected_costs: []
    expected_gains: [minor_clue]
  - id: ask_floor_number
    role: social_probe
    expected_costs: [sanity_small]
    expected_gains: [isolation_rule_hint]
  - id: trace_channel_route
    role: system_probe
    required_player_ability_hint: interface
    expected_costs: [battery_medium]
    expected_gains: [route_flag, technical_clue]
outcome_hooks:
  possible_flags: [isolation_channel_joined, mismatched_floor_reported, channel_route_traced]
  possible_clues: [channel_floor_mismatch, delayed_channel_route]
  possible_items: []
  relation_notes:
    park_doyoon: tracing the route can increase trust, but failing may increase suspicion.
main_spine_link: 플레이어가 완전히 혼자가 아니라는 사실과, 같은 회사가 여러 격리 층으로 갈라졌다는 첫 단서를 준다.
randomization_notes: opening_absence 이후 1회성. `isolation_channel_joined` 이후 반복 금지.
promotion_notes: 초반 메신저 hook으로 적합하다. runtime 승격 시 기존 `ex_employee_messenger`와 같은 시작 위치에서 충돌하지 않도록 required/forbidden flag gating 필요.
```

### Runtime 승격 초안

- 후보 위치: `dev_desk` 또는 `dev_office`
- repeat policy: once-only
- recommended forbidden flag: `isolation_channel_joined`
- possible presentation:
  - `visual_id: isolation_channel`
  - `speaker: 사내 메신저`
  - `effect_cues: messenger_delay`, `sender_name_reflow`

---

## `org_chart_missing_employee`

```yaml
id: org_chart_missing_employee
status: candidate
storypack_id: isolation_pack
phase: normal_operations
priority_class: npc_relation
location_tags: [office, intranet, hr]
surface: organization_chart
anomaly_type: identity_erasure
pressure_type: [sanity, relation]
npc_slots: [hr_identity_keeper]
candidate_characters: [yoon_seoyeon]
summary: 윤서연이 분명히 기억하는 직원이 조직도와 사원 조회 시스템에서는 존재하지 않는다.
setup_text: 조직도 검색창에 이름을 입력하면 `해당 직원은 조회되지 않습니다`라는 안내만 반복된다. 윤서연은 그 사람이 방금 전까지 같은 대응방에 있었다고 말하지만, 화면은 입사 이력도 퇴사 이력도 없다고 주장한다.
choice_shapes:
  - id: close_org_chart
    role: safe_leave
    expected_costs: []
    expected_gains: [sanity_relief]
  - id: ask_yoon_memory
    role: social_probe
    expected_costs: [sanity_small]
    expected_gains: [missing_person_clue, relation_trust]
  - id: export_hr_record
    role: document_probe
    required_player_ability_hint: logic
    expected_costs: [battery_medium]
    expected_gains: [identity_erasure_flag]
  - id: accept_system_record
    role: dark_bargain
    expected_costs: [relation_loss]
    expected_gains: [danger_reduction]
outcome_hooks:
  possible_flags: [missing_employee_reported, hr_record_exported, system_record_accepted]
  possible_clues: [missing_employee_name_hash, organization_chart_erasure]
  possible_items: [hr_record_fragment]
  relation_notes:
    yoon_seoyeon: asking her memory can increase trust; accepting the system record can damage trust.
main_spine_link: 회사 시스템이 사람의 존재를 기록으로 판정한다는 핵심 공포를 보여준다.
randomization_notes: `isolation_channel_joined` 이후 등장하면 의미가 강하다. `missing_employee_reported` 이후 반복 금지.
promotion_notes: HR/identity route hook으로 좋다. runtime 승격 시 `organization_chart` surface가 아직 직접 UI로 없으면 서술형 encounter로 먼저 구현 가능.
```

### Runtime 승격 초안

- 후보 위치: `dev_office`, `meeting_room`, 또는 future HR-facing virtual surface
- repeat policy: once-only
- recommended required flag: `isolation_channel_joined` 또는 `truth_route_started`
- recommended forbidden flag: `missing_employee_reported`

---

## `delayed_cctv_next_action`

```yaml
id: delayed_cctv_next_action
status: candidate
storypack_id: isolation_pack
phase: isolation_rules
priority_class: npc_relation
location_tags: [security, surveillance]
surface: cctv
anomaly_type: delayed_time
pressure_type: [sanity, danger, relation]
npc_slots: [security_witness]
candidate_characters: [choi_minjae]
summary: 최민재가 CCTV에서 플레이어가 아직 하지 않은 행동을 이미 봤다고 말한다.
setup_text: 보안실 모니터에는 플레이어가 복도 모퉁이를 도는 장면이 보인다. 문제는 플레이어가 아직 그 복도에 가지 않았다는 것이다. 최민재는 화면에서 눈을 떼지 않고 “지금 가지 마십시오. 이미 한 번 갔습니다”라고 말한다.
choice_shapes:
  - id: trust_security_warning
    role: safe_observe
    expected_costs: []
    expected_gains: [danger_avoidance, relation_trust]
  - id: replay_cctv_frame
    role: information_probe
    expected_costs: [battery_small, sanity_small]
    expected_gains: [future_record_clue]
  - id: force_same_route
    role: high_risk_route_test
    expected_costs: [health_or_sanity_medium]
    expected_gains: [space_rule_flag]
  - id: ask_who_else_seen
    role: social_probe
    expected_costs: [sanity_small]
    expected_gains: [other_survivor_hint]
outcome_hooks:
  possible_flags: [future_cctv_seen, cctv_warning_trusted, repeated_route_tested]
  possible_clues: [cctv_next_action_frame, unsafe_corridor_pattern]
  possible_items: []
  relation_notes:
    choi_minjae: trusting the warning increases trust; forcing the route may increase suspicion or debt.
main_spine_link: 격리 공간의 시간이 선형이 아니며, 사내 시스템이 미래 행동을 기록할 수 있다는 규칙을 드러낸다.
randomization_notes: security_room 계열에서 1회성. 기존 `security_room_delayed_cctv`와 중복되지 않게 required clue/flag로 구분해야 한다.
promotion_notes: 기존 보안실/CCTV route와 연결하기 좋지만, 이미 `security_room_delayed_cctv`가 있으므로 “다른 격리자/NPC 관계” 중심으로 차별화해야 한다.
```

### Runtime 승격 초안

- 후보 위치: `security_room`
- repeat policy: once-only
- recommended required flag: `isolation_channel_joined` 또는 `impossible_meeting_saved`
- recommended forbidden flag: `future_cctv_seen`
- possible presentation:
  - `visual_id: delayed_cctv`
  - `effect_cues: frame_lag`, `ghost_afterimage`

---

## `server_log_other_branch`

```yaml
id: server_log_other_branch
status: candidate
storypack_id: isolation_pack
phase: isolation_rules
priority_class: route_key
location_tags: [server, intranet, build_log]
surface: build_log
anomaly_type: worldline_branch
pressure_type: [battery, sanity]
npc_slots: [infra_interpreter]
candidate_characters: [park_doyoon]
summary: 박도윤이 같은 사건의 성공/실패 로그가 서로 다른 브랜치에 남아 있는 것을 발견한다.
setup_text: 서버 로그에는 같은 시각, 같은 작업, 같은 사용자가 남긴 기록이 두 줄 있다. 하나는 `current_branch`에서 실패했고, 하나는 `success_branch`에서 이미 병합되었다. 박도윤은 둘 중 하나만 실제로 남길 수 있을 것 같다고 말한다.
choice_shapes:
  - id: copy_log_names_only
    role: safe_observe
    expected_costs: [battery_small]
    expected_gains: [minor_clue]
  - id: compare_branch_diff
    role: system_probe
    required_player_ability_hint: interface
    expected_costs: [battery_medium, sanity_small]
    expected_gains: [worldline_branch_clue]
  - id: mark_current_branch
    role: route_commit_current
    expected_costs: [sanity_medium]
    expected_gains: [current_branch_flag]
  - id: mark_success_branch
    role: dark_bargain
    expected_costs: [relation_or_memory_cost]
    expected_gains: [success_branch_flag]
outcome_hooks:
  possible_flags: [worldline_branch_seen, current_branch_marked, success_branch_marked]
  possible_clues: [branch_diff_log, merge_conflict_timestamp]
  possible_items: [log_export_fragment]
  relation_notes:
    park_doyoon: technical trust increases if branch diff is compared; moral conflict increases if success branch is marked.
main_spine_link: 격리 사건을 개발/업무 시스템의 branch 은유로 이해하게 하며, 후반 결말 후보와 연결된다.
randomization_notes: server/intranet route에 묶는다. 초반 랜덤으로 나오면 설명 과부하가 크므로 isolation_rules 이후 권장.
promotion_notes: SW 특화 소재이므로 일반 대기업 base에서는 과하게 기술적으로 쓰지 않는다. 윤리적 선택이 핵심이어야 한다.
```

### Runtime 승격 초안

- 후보 위치: `server_room_front`, `server_room`, future `monitoring_room`
- repeat policy: once-only
- recommended required clues: `delayed_channel_route` 또는 `server_log_fragment`
- recommended route use: truth/conquest 중간 hook

---

## `automatic_minutes_no_attendees`

```yaml
id: automatic_minutes_no_attendees
status: candidate
storypack_id: isolation_pack
phase: normal_operations
priority_class: random_pack
location_tags: [meeting_room, intranet]
surface: meeting_minutes
anomaly_type: absent_people
pressure_type: [sanity, battery]
npc_slots: [hr_identity_keeper]
candidate_characters: [yoon_seoyeon]
summary: 참석자가 아무도 없는 회의의 자동 회의록이 이미 결론을 작성했다.
setup_text: 회의실 예약 패널에는 방금 종료된 회의록이 공유되어 있다. 참석자는 0명인데 결론은 `전원 현 위치 유지`로 적혀 있다. 윤서연은 참석자 명단이 비어 있을 때 결론이 먼저 쓰이는 경우는 없다고 말한다.
choice_shapes:
  - id: leave_minutes_closed
    role: safe_leave
    expected_costs: []
    expected_gains: [sanity_relief]
  - id: read_decision_line
    role: information_probe
    expected_costs: [sanity_small]
    expected_gains: [meeting_rule_clue]
  - id: download_minutes
    role: document_probe
    expected_costs: [battery_medium]
    expected_gains: [minutes_fragment_item]
  - id: add_self_as_attendee
    role: high_risk_route_test
    expected_costs: [sanity_medium]
    expected_gains: [meeting_route_flag]
outcome_hooks:
  possible_flags: [no_attendee_minutes_seen, self_added_to_minutes]
  possible_clues: [empty_attendee_decision, meeting_rule_fragment]
  possible_items: [auto_minutes_fragment]
  relation_notes:
    yoon_seoyeon: downloading the minutes may help her verify identity records; adding self as attendee alarms her.
main_spine_link: 회사가 사람 없이도 의사결정을 계속한다는 핵심 정체성을 강화한다.
randomization_notes: meeting_room에서 기존 `meeting_room_all_hands`와 충돌 가능. required/forbidden flags로 별도 단계에 배치한다.
promotion_notes: 기존 회의실 truth route와 비슷하므로 runtime 승격 시 “격리자 rescue/identity” 쪽 hook으로 차별화한다.
```

### Runtime 승격 초안

- 후보 위치: `meeting_room`
- repeat policy: once-only
- recommended forbidden flags: `no_attendee_minutes_seen`, `impossible_meeting_saved`
- possible item: `auto_minutes_fragment`

---

## `pantry_survivor_trace`

```yaml
id: pantry_survivor_trace
status: candidate
storypack_id: isolation_pack
phase: opening_absence
priority_class: generic_pressure
location_tags: [pantry, office_object, survival]
surface: office_object
anomaly_type: absent_people
pressure_type: [thirst, sanity]
npc_slots: []
candidate_characters: []
summary: 탕비실에는 방금 누군가 물을 마신 흔적이 있지만, 컵과 생수 수량이 맞지 않는다.
setup_text: 탕비실 테이블 위에는 아직 물방울이 마르지 않은 종이컵이 있다. 냉장고에는 밀봉된 생수가 하나 줄어든 것처럼 보이지만, 재고표에는 오히려 한 병이 늘었다. 정수기 물소리는 아무도 없는 방향으로 이어진다.
choice_shapes:
  - id: ignore_wet_cup
    role: safe_leave
    expected_costs: []
    expected_gains: [sanity_relief]
  - id: inspect_cup_lip
    role: sensory_probe
    required_player_ability_hint: empathy_or_logic
    expected_costs: [sanity_small]
    expected_gains: [survivor_trace_clue]
  - id: take_bottled_water_anyway
    role: resource_gain
    expected_costs: [minor_risk]
    expected_gains: [water_item]
  - id: follow_water_sound
    role: high_risk_route_test
    expected_costs: [thirst_or_sanity_medium]
    expected_gains: [location_hint]
outcome_hooks:
  possible_flags: [pantry_survivor_trace_seen, false_inventory_count_seen]
  possible_clues: [wet_cup_recent_presence, water_sound_direction]
  possible_items: [bottled_water]
  relation_notes: {}
main_spine_link: 사람들이 완전히 사라진 것이 아니라 방금 전까지 있었거나 다른 격리 구역에서 흔적만 넘어온다는 감각을 만든다.
randomization_notes: pantry의 기존 물/커피/갈증 이벤트와 충돌 가능. resource pressure deck로 두고 반복 금지.
promotion_notes: 낮은 우선순위. 생존 압박과 분위기 보강용으로 좋지만 메인 루트 hook은 약하다.
```

### Runtime 승격 초안

- 후보 위치: `pantry`
- repeat policy: once-only
- recommended forbidden flags: `pantry_survivor_trace_seen`
- caution: 기존 `pantry_coffee_machine`, `strange_water_dispenser`와 경쟁하므로 weight를 낮게 두거나 required phase/flag를 둔다.

---

## Runtime 승격 우선순위 제안

1. `isolation_channel_mismatched_floor`
   - 초반 storypack hook.
   - 메신저 surface가 강하다.
   - 박도윤 introduction과 잘 맞다.
2. `org_chart_missing_employee`
   - identity erasure를 명확하게 보여준다.
   - 윤서연 introduction과 잘 맞다.
3. `delayed_cctv_next_action`
   - existing security/CCTV route와 자연스럽게 연결된다.
   - 최민재 introduction과 잘 맞다.

보류:

- `server_log_other_branch`: 좋지만 SW/developer metaphor가 강해서 톤 조절이 필요하다.
- `automatic_minutes_no_attendees`: 기존 회의실 truth route와 중복될 수 있다.
- `pantry_survivor_trace`: 분위기/자원 보강용으로 좋지만 route hook이 약하다.

## 검토 체크리스트

- [ ] 모든 카드가 `isolation_pack`에 연결되어 있다.
- [ ] 모든 카드가 phase를 가진다.
- [ ] 모든 카드가 surface와 anomaly_type을 가진다.
- [ ] 모든 카드가 fallback 선택지를 가진다.
- [ ] 모든 카드가 outcome hook을 가진다.
- [ ] 모든 카드가 main story spine과 연결된다.
- [ ] 모든 카드는 public-safe다.
- [ ] runtime 승격 후보는 기존 인카운터와 충돌하지 않게 gating 계획을 가진다.

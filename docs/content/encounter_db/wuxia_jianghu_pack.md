# 무협 강호팩 encounter situation cards

Status: candidate

이 문서는 `docs/content/storypacks/wuxia_jianghu_pack.md`의 후보 인카운터를 runtime YAML 승격 전 상황 카드로 정리한다.

공통 원칙:

- 모든 카드는 `world_id: wuxia_jianghu`, `storypack_id: wuxia_jianghu_pack`에 속한다.
- 현재 단계에서는 runtime encounter가 아니다.
- 플레이어 전제는 “회사에 다니던 직장인이 눈떠보니 무협 세계에 떨어졌다”이다.
- 선택지는 세부 수치보다 역할과 결과 hook을 먼저 정의한다.
- 최소 하나의 안전한 관찰/후퇴/fallback 선택지를 둔다.
- 첫 runtime 승격 전에는 office storypack과 섞이지 않도록 gating 전략을 별도 결정한다.
- 소림/무당/아미산은 자료 기반 분위기 앵커로만 쓰고, 실제 단체·장소의 세부 역사나 종교 설명을 게임 사실처럼 단정하지 않는다.

## 1. `wuxia_office_worker_arrival`

```yaml
id: wuxia_office_worker_arrival
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: candidate
phase: wuxia_arrival
priority_class: main_forced
location_tags: [inn, starting_room]
surface: inn_room
anomaly_type: world_displacement
pressure_type: [sanity, danger]
npc_slots: [innkeeper_guide]
candidate_characters: [old_innkeeper]
summary: 회사원이 객잔에서 깨어나고 현대 물건이 통하지 않는다.
setup_text: 퇴근 직전 엘리베이터 버튼을 누른 기억이 마지막이다. 눈을 뜨자 천장은 낯선 목조 서까래이고, 휴대폰은 켜지지 않으며, 사원증 목걸이는 거친 나무 패로 바뀌어 있다.
choice_shapes:
  - id: check_phone_and_badge
    role: safe_observe
    expected_costs: []
    expected_gains: [displacement_clue]
  - id: ask_where_office_is
    role: social_probe
    expected_costs: [embarrassment_or_relation_risk]
    expected_gains: [innkeeper_orientation]
  - id: leave_room_immediately
    role: risky_exit
    expected_costs: [danger_small]
    expected_gains: [jianghu_notice_board_hook]
outcome_hooks:
  possible_flags: [wuxia_arrival_confirmed, office_items_failed]
  possible_clues: [dead_phone, badge_changed_to_token]
  possible_items: [wooden_token]
main_spine_link: 사용자가 정한 office worker isekai 전제를 첫 장면에서 고정한다.
randomization_notes: main_forced opening beat. 반복 등장 금지.
promotion_notes: 첫 runtime wuxia prototype의 최우선 후보.
```

## 2. `wuxia_notice_foreigner_without_sect`

```yaml
id: wuxia_notice_foreigner_without_sect
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: candidate
phase: jianghu_orientation
priority_class: random_pack
location_tags: [inn, notice_board]
surface: jianghu_notice_board
anomaly_type: foreigner_without_sect
pressure_type: [sanity, relation]
npc_slots: [innkeeper_guide]
candidate_characters: [old_innkeeper]
summary: 강호 게시판에 “출신불명 외지인” 경고가 붙어 있다.
setup_text: 객잔 일층의 게시판에는 새 공고가 붙어 있다. “소속 문파 불명, 말투 괴이, 철제 명패 소지자 주의.” 플레이어는 그것이 자신을 말한다는 사실을 조금 늦게 깨닫는다.
choice_shapes:
  - id: read_without_touching
    role: safe_observe
    expected_costs: []
    expected_gains: [minor_clue]
  - id: ask_what_sect_means
    role: social_probe
    expected_costs: [relation_risk]
    expected_gains: [sect_orientation_clue]
  - id: remove_notice
    role: high_risk_identity_denial
    expected_costs: [danger_medium]
    expected_gains: [identity_rebellion_flag]
outcome_hooks:
  possible_flags: [foreigner_notice_seen, notice_disturbed]
  possible_clues: [sect_affiliation_required, strange_badge_report]
  possible_items: []
main_spine_link: 현대 회사원의 소속 개념과 강호의 문파 소속 개념을 충돌시킨다.
randomization_notes: jianghu_orientation 1회성. hub에 항상 eligible하게 두지 않는다.
promotion_notes: `wuxia_office_worker_arrival` 이후 두 번째 runtime 후보.
```

## 3. `wuxia_manual_as_onboarding`

```yaml
id: wuxia_manual_as_onboarding
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: candidate
phase: jianghu_orientation
priority_class: random_pack
location_tags: [inn, manual]
surface: martial_manual
anomaly_type: manual_as_onboarding
pressure_type: [sanity, health]
npc_slots: [innkeeper_guide, shaolin_anchor_monk]
candidate_characters: [old_innkeeper, traveling_monk]
summary: 비급을 회사 온보딩 문서처럼 읽다가 몸이 먼저 반응한다.
setup_text: 객잔 주인이 “초행이면 이것부터 보라”며 얇은 책자를 건넨다. 표지는 낡은 비급인데, 플레이어의 눈에는 이상하게도 신규 입사자 안내서처럼 목차가 정리되어 보인다.
choice_shapes:
  - id: skim_table_of_contents
    role: safe_observe
    expected_costs: []
    expected_gains: [manual_structure_clue]
  - id: follow_first_breathing_note
    role: tutorial_probe
    expected_costs: [sanity_small]
    expected_gains: [qi_sensation_clue]
  - id: memorize_like_work_procedure
    role: high_risk_overfocus
    expected_costs: [health_or_sanity_medium]
    expected_gains: [manual_overfit_flag]
outcome_hooks:
  possible_flags: [manual_read_as_onboarding, first_qi_reaction]
  possible_clues: [manual_translates_to_office_language, breathing_note]
  possible_items: [thin_manual]
main_spine_link: office UI/문서 독해 습관을 무협 surface로 번역한다.
randomization_notes: opening 이후 1회성 튜토리얼 후보.
promotion_notes: ability/resource display alias 후속 slice와 연결 가능.
```

## 4. `wuxia_shaolin_wudang_emei_rumor`

```yaml
id: wuxia_shaolin_wudang_emei_rumor
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: candidate
phase: sect_contact
priority_class: npc_relation
location_tags: [inn, tavern, rumor]
surface: tavern_rumor
anomaly_type: workplace_memory_mismatch
pressure_type: [relation, sanity]
npc_slots: [innkeeper_guide, shaolin_anchor_monk, wudang_anchor_taoist, emei_anchor_swordswoman]
candidate_characters: [old_innkeeper, traveling_monk, wandering_taoist, emei_swordswoman]
summary: 객잔 소문으로 소림/무당/아미산 계열 선택지의 성격을 짧게 배운다.
setup_text: 아래층 술상에서는 소림 쪽 행각승, 무당산에서 내려왔다는 도사, 아미산 쪽 검수 이야기가 뒤섞인다. 플레이어는 이것을 회사 조직도처럼 정리하려다가 금세 포기한다.
choice_shapes:
  - id: listen_and_map_names
    role: safe_observe
    expected_costs: [time_small]
    expected_gains: [sect_anchor_clue]
  - id: ask_for_reliable_contact
    role: social_probe
    expected_costs: [relation_risk]
    expected_gains: [first_contact_hint]
  - id: pretend_to_have_affiliation
    role: high_risk_deception
    expected_costs: [danger_or_relation_medium]
    expected_gains: [false_affiliation_flag]
outcome_hooks:
  possible_flags: [sect_anchor_rumors_heard, false_affiliation_attempted]
  possible_clues: [shaolin_wudang_emei_anchor, sects_are_not_departments]
  possible_items: []
main_spine_link: 공신력 있는 공개 자료로 확인 가능한 무협 앵커를 낮은 해상도로 소개한다.
randomization_notes: 반복 가능하지만 동일 clue는 1회만 지급한다.
promotion_notes: 자료 기반 앵커 설명이 충분할 때 runtime 후보로 검토한다.
```

## 5. `wuxia_badge_mistaken_for_token`

```yaml
id: wuxia_badge_mistaken_for_token
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: candidate
phase: sect_contact
priority_class: route_key
location_tags: [gate, sect_token, road]
surface: sect_token
anomaly_type: workplace_memory_mismatch
pressure_type: [sanity, relation, danger]
npc_slots: [courier_broker, wudang_anchor_taoist]
candidate_characters: [courier_broker, wandering_taoist]
summary: 사원증이 문파 패로 오해받아 통과 권한과 의심을 동시에 만든다.
setup_text: 길목의 검문꾼이 플레이어의 목걸이를 보고 멈칫한다. 플라스틱 사원증이어야 할 물건은 나무 패처럼 보이고, 검문꾼은 그것이 어느 문파의 추천서인지 묻는다.
choice_shapes:
  - id: admit_not_understanding_token
    role: safe_honesty
    expected_costs: [relation_risk]
    expected_gains: [token_rule_clue]
  - id: present_token_like_badge
    role: office_habit_probe
    expected_costs: [sanity_small]
    expected_gains: [temporary_passage_hint]
  - id: claim_lost_department_name
    role: high_risk_deception
    expected_costs: [danger_medium]
    expected_gains: [false_sect_route_flag]
outcome_hooks:
  possible_flags: [badge_token_mismatch_seen, false_sect_claimed]
  possible_clues: [token_controls_passage, office_badge_memory]
  possible_items: [wooden_token_marked]
main_spine_link: office pack의 사원증/보안권한을 무협의 문파 패/통행 권한으로 대응시킨다.
randomization_notes: route_key. gate/road location gating 필요.
promotion_notes: world-specific item display가 필요하면 별도 slice로 분리한다.
```

## 6. `wuxia_duel_bridge_intervention`

```yaml
id: wuxia_duel_bridge_intervention
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: candidate
phase: route_commitment
priority_class: route_key
location_tags: [bridge, duel, narrow_space]
surface: duel_record
anomaly_type: oath_binding
pressure_type: [health, danger, relation]
npc_slots: [emei_anchor_swordswoman, courier_broker]
candidate_characters: [emei_swordswoman, courier_broker]
summary: 다리 위 결투에서 자동 난투가 벌어지고 1회 개입 기회가 생긴다.
setup_text: 좁은 다리 위에서 검과 소매가 엉킨다. 플레이어는 싸움을 말리는 법도, 가세하는 법도 모른다. 다만 회사에서 배운 회의 중재 습관이 이상하게 한 박자 빠른 선택지를 만든다.
choice_shapes:
  - id: step_back_and_read_flow
    role: safe_reposition
    expected_costs: [danger_small]
    expected_gains: [survive_exchange]
  - id: interrupt_like_meeting_conflict
    role: combat_intervention
    expected_costs: [health_risk]
    expected_gains: [duel_deescalated_flag]
  - id: cut_bridge_rope
    role: high_risk_route_commitment
    expected_costs: [major_danger]
    expected_gains: [oath_break_route_flag]
outcome_hooks:
  possible_flags: [bridge_duel_intervened, duel_deescalated, oath_break_attempted]
  possible_clues: [office_conflict_skill_translates_badly]
  possible_items: []
main_spine_link: `Combat_System_Auto_Brawl.md`의 자동 난투 + 1회 상황 개입을 office-worker isekai world에서 검증한다.
randomization_notes: 보스/정예급 장면. 전투당 개입 0~3회 원칙을 지킨다.
promotion_notes: schema-less combat encounter prototype의 첫 무협 후보.
```

# 이구학지 — 천기록 encounter situation cards

Status: candidate

이 문서는 `docs/content/storypacks/wuxia_jianghu_pack.md`의 후보 인카운터를 runtime YAML 승격 전 상황 카드로 정리한다. `wuxia_commute_rift_arrival`와 `wuxia_heuksa_bang_first_fight`는 이 카드에서 separate storypack preview runtime으로 승격된 첫 두 slice이며, 나머지 카드는 아직 후보 상태다.

공통 원칙:

- 모든 카드는 `world_id: wuxia_jianghu`, `storypack_id: wuxia_jianghu_pack`에 속한다.
- 현재 단계에서는 대부분 runtime encounter가 아니다. 단, `wuxia_commute_rift_arrival`와 `wuxia_heuksa_bang_first_fight`는 `src/tui_adv/storypack-previews/wuxia_jianghu_pack/`의 preview source와 별도 generated preview bundle에 반영됐다.
- 최신 canonical 무협 설정은 **이구학지 — 천기록**이다. 이전의 generic 객잔/소림/무당/아미 placeholder는 superseded로 본다.
- 플레이어 전제는 “현대 회사원이 본인 몸과 출근복장 그대로 무협 세계의 시장 한복판에 전이됐다”이다.
- 선택지는 세부 수치보다 역할과 결과 hook을 먼저 정의한다.
- 최소 하나의 안전한 관찰/후퇴/fallback 선택지를 둔다.
- preview runtime 승격은 office storypack과 섞이지 않도록 separate preview mode를 유지한다.
- 실제 회사명, 실제 통근 경로, 실제 사원증 정보, 현실 종교/정치/민족 소재처럼 보이는 세부사항은 쓰지 않는다.

## 1. `wuxia_commute_rift_arrival`

```yaml
id: wuxia_commute_rift_arrival
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: candidate
phase: [commute_rift, market_arrival]
priority_class: main_forced
location_tags: [commute_boundary, market, starting_scene]
surface: [commute_rift, market_street, office_items]
anomaly_type: world_displacement
pressure_type: [sanity, danger]
npc_slots: []
candidate_characters: []
summary: 출근 중 균열에 휘말려 본인 몸과 출근복장 그대로 무협 세계 시장 한복판에 떨어진다.
setup_text: 엘리베이터 문이 열리거나 지하철 문이 닫히는 순간 눈앞이 하얗게 번진다. 다시 눈을 뜨자 형광등 대신 종이등, 아스팔트 대신 흙길, 자동차 소리 대신 말발굽 소리가 있다. 주인공은 정장 차림, 목에는 사원증, 손에는 커피나 가방을 든 채 시장 한복판에 서 있다. 스마트폰은 사라졌고, 가방에는 볼펜과 업무수첩만 남아 있다.
choice_shapes:
  - id: inventory_office_items
    role: safe_observe
    expected_costs: []
    expected_gains: [modern_items_confirmed, smartphone_missing_clue]
  - id: ask_if_this_is_a_set
    role: social_probe
    expected_costs: [embarrassment_or_suspicion]
    expected_gains: [world_displacement_confirmed]
  - id: move_out_of_crowd
    role: safe_reposition
    expected_costs: [danger_small]
    expected_gains: [market_exit_or_brawl_hook]
outcome_hooks:
  possible_flags: [wuxia_arrival_confirmed, smartphone_missing, office_items_remain]
  possible_clues: [commute_rift_memory, market_is_not_film_set]
  possible_items: [employee_badge, work_notebook, ballpoint_pen]
main_spine_link: updated wuxia story의 출발점을 고정하고, office 출신 플레이어가 office surface 없이도 같은 survival loop에 들어갈 수 있음을 보여준다.
randomization_notes: main_forced opening beat. 반복 등장 금지.
promotion_notes: 첫 schema-less wuxia runtime prototype의 최우선 후보. 새 world schema 없이 title/body/choices/outcome으로 구현 가능하다.
```

## 2. `wuxia_heuksa_bang_first_fight`

```yaml
id: wuxia_heuksa_bang_first_fight
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: candidate
phase: first_brawl
priority_class: main_forced
location_tags: [market, alley, brawl]
surface: [market_street, office_items]
anomaly_type: first_brawl_defeat
pressure_type: [health, danger, relation]
npc_slots: [early_rescuer]
candidate_characters: [seo_harin]
summary: 흑사방 말단 불량배에게 시비가 걸리고, 대부분 패배하는 튜토리얼성 첫 전투를 겪는다.
purpose: 이 세계의 폭력이 실제이며, 현대 회사원의 출근복/구두/가방/사원증이 전투에서 장점이 아니라 변수와 약점으로 작동한다는 사실을 보여준다. 승리 판정보다 부상, 평판, 도주로, 서하린 구조 hook을 남기는 것이 핵심이다.
start_conditions:
  runtime_mode: storypack_preview
  after_encounter: wuxia_commute_rift_arrival
  phase: first_brawl
  recommended_location: jianghu_market_street
  fallback_location_if_minimal_preview: jianghu_roadside
  required_flags: [wuxia_arrival_hidden]
  forbidden_flags: [heuksa_bang_first_fight_resolved]
  routing_note: 현재 preview arrival는 `wuxia_arrival_hidden`과 `wuxia_arrival_grounded`로 갈라진다. 첫 slice를 숨기 route smoke로 제한하면 `wuxia_arrival_hidden`만 써도 되고, 두 선택지를 모두 first fight로 잇는다면 새 any-of schema 대신 두 outcome에 공통 `wuxia_arrival_resolved` flag를 추가한다.
setup_text: 흑사방 말단들이 길을 막는다. “길을 막았으면 통행세를 내야지.” “그 목에 건 패, 꽤 값나 보이는군.” 주인공은 이 상황을 촬영장 장난처럼 넘기려 하지만, 몽둥이가 팔을 스치는 순간 이 세계의 폭력이 진짜임을 깨닫는다. 구두는 미끄럽고 정장은 움직임을 막으며, 주변 사람들은 쉽게 끼어들지 않는다.
choice_shapes:
  - id: run_toward_open_street
    role: safe_retreat_attempt
    fallback_choice: true
    label_direction: 큰길 쪽으로 비틀거리며 물러난다
    expected_costs: [health_small_or_danger_small]
    outcome_hook:
      resources: {health: -3}
      add_flags: [first_brawl_started, heuksa_bang_first_fight_resolved, first_brawl_survived]
      add_clues: [violence_is_real, open_street_escape_route]
      log_direction: 도망은 영웅적이지 않지만 큰 부상을 줄이고, 흑사방이 길목을 어떻게 막는지 본다.
  - id: deescalate_with_words
    role: social_probe
    label_direction: 말로 시간을 벌며 사원증을 감춘다
    expected_costs: [relation_risk, suspicion_small]
    outcome_hook:
      resources: {sanity: -2}
      danger: 1
      add_flags: [first_brawl_started, heuksa_bang_first_fight_resolved, seo_harin_rescue_interest]
      add_clues: [badge_misread_as_sect_token, heuksa_bang_uses_toll_excuse]
      log_direction: 말은 통하지만 세계관이 다르다. 설득은 완전 성공이 아니라 구조자가 끼어들 시간을 번다.
  - id: swing_commute_bag
    role: improvised_item_use
    label_direction: 출근 가방을 방패처럼 휘두른다
    expected_costs: [health_risk, item_damage]
    outcome_hook:
      resources: {health: -6}
      add_flags: [first_brawl_started, heuksa_bang_first_fight_resolved, commute_bag_damaged, brief_opening_created]
      add_clues: [office_items_can_block_once]
      log_direction: 가방은 무기가 아니라 한 번의 완충재다. 틈은 만들지만 물건은 망가지고 주인공은 다친다.
  - id: loosen_tie_and_drop_shoes
    role: combat_reposition
    label_direction: 넥타이를 풀고 구두를 벗어 움직임을 회복한다
    expected_costs: [dignity_loss_or_suspicion]
    outcome_hook:
      resources: {sanity: -1}
      add_flags: [first_brawl_started, heuksa_bang_first_fight_resolved, mobility_recovered]
      add_clues: [shoes_and_suit_are_liability]
      log_direction: 체면은 무너졌지만 발이 땅을 잡는다. 무공이 아니라 몸을 살리는 준비가 먼저다.
  - id: crash_in_with_body
    role: high_risk_body_check
    label_direction: 어깨로 들이받고 넘어지듯 버틴다
    expected_costs: [health_medium]
    outcome_hook:
      resources: {health: -10}
      danger: 1
      add_flags: [first_brawl_started, heuksa_bang_first_fight_resolved, dirty_holdout_flag, heuksa_bang_attention]
      add_clues: [violence_is_real, crowd_now_watches_you]
      log_direction: 잠깐 버티거나 더럽게 이길 수 있지만 무쌍은 아니다. 손은 떨리고 셔츠는 찢어지며 더 큰 시선이 붙는다.
outcome_hooks:
  possible_flags: [first_brawl_started, heuksa_bang_first_fight_resolved, first_real_injury, heuksa_bang_attention, seo_harin_rescue_interest]
  possible_clues: [violence_is_real, shoes_and_suit_are_liability, badge_misread_as_sect_token, open_street_escape_route]
  possible_items: [torn_shirt, damaged_bag]
  possible_relations: [seo_harin_rescue_interest]
  possible_log_tone:
    - 맞으면 진짜 아프고, 리셋되지 않는다는 감각
    - 출근복과 구두가 기동성을 방해한다는 감각
    - 사원증이 문파패/신물처럼 오해받는 감각
    - 서하린 구조/조사 장면으로 이어지는 hook
schema_boundary:
  allowed_existing_schema: [conditions.locations, required_flags, forbidden_flags, choices.cost, outcome.resources, outcome.danger, outcome.add_flags, outcome.add_clues, outcome.add_items, outcome.remove_items, outcome.destination_id, outcome.log, presentation]
  forbidden_new_schema: [CombatState, combat_hp_track, combat_resolver, skill_cooldown, reward_schema, ability_schema, fragment_choice_reward]
main_spine_link: 이 세계가 위험하며 현대 회사원의 기본 습관이 그대로 통하지 않는다는 점을 전투/도주/설득/소지품 사용으로 보여준다. 이후 `wuxia_seo_harin_rescue`와 청류문 수습생 편입으로 이어진다.
randomization_notes: opening 직후 1회성. 승패보다 부상, 평판, 구조 hook이 핵심이다. hub에 항상 eligible하게 두지 말고 arrival-resolved flag와 resolved forbidden flag로 반복을 막는다.
promotion_notes: 다음 runtime slice로 확정한다. 같은 storypack preview bundle에만 추가하고, 기본 office bundle/`escape-office` save key/천외편린 3택 성장 schema를 건드리지 않는다. preview launcher/UI wiring은 follow-up 후보일 뿐 선행 조건은 아니다.
```

## 3. `wuxia_seo_harin_rescue`

```yaml
id: wuxia_seo_harin_rescue
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: candidate
phase: rescue_and_investigation
priority_class: main_forced
location_tags: [market, aftermath, rescue]
surface: [market_street, faction_negotiation]
anomaly_type: outsider_without_sect
pressure_type: [health, relation, sanity]
npc_slots: [early_rescuer]
candidate_characters: [seo_harin]
summary: 청류문 외문 제자 서하린이 개입해 주인공을 구조하지만, 그를 정체불명의 외지인으로 의심한다.
setup_text: 전투가 길어지거나 주인공이 쓰러질 즈음, 청류문 외문 제자 서하린이 끼어든다. 흑사방 말단 정도는 제압할 수 있지만 그녀도 주인공을 믿지는 않는다. “무공도 모르는 자가 흑사방 말단을 건드렸다고?” “그 옷차림은 뭐지?” “목에 건 패는 어느 세력의 표식이냐?”
choice_shapes:
  - id: tell_plain_truth
    role: safe_honesty
    expected_costs: [suspicion_small]
    expected_gains: [truthful_outsider_flag]
  - id: explain_company_and_commute
    role: workplace_memory_probe
    expected_costs: [sanity_small, misunderstanding]
    expected_gains: [company_words_fail_clue]
  - id: ask_for_medical_help_first
    role: survival_priority
    expected_costs: [debt_small]
    expected_gains: [injury_stabilized]
  - id: hide_employee_badge
    role: high_risk_concealment
    expected_costs: [suspicion_medium]
    expected_gains: [badge_secret_flag]
outcome_hooks:
  possible_flags: [seo_harin_intervened, taken_under_watch, outsider_claim_recorded]
  possible_clues: [cheongryu_name_heard, sect_identity_matters]
  possible_items: []
  possible_relations: [seo_harin_suspicion, seo_harin_responsibility]
main_spine_link: 구조자/멘토 후보를 세우고, 주인공을 청류문 수습생 구간으로 이동시킨다.
randomization_notes: first_brawl 이후 forced aftermath. 별도 hub random으로 반복하지 않는다.
promotion_notes: runtime 승격 시 healing/debt/relation은 새 schema 없이 log/flag/clue로만 표현한다.
```

## 4. `wuxia_cheongryu_apprentice_entry`

```yaml
id: wuxia_cheongryu_apprentice_entry
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: candidate
phase: cheongryu_apprenticeship
priority_class: route_key
location_tags: [cheongryu_sect, courtyard, apprenticeship]
surface: [sect_courtyard, training_chore]
anomaly_type: sect_debt
pressure_type: [relation, hunger, health]
npc_slots: [sect_master_guardian, early_rescuer, archive_keeper]
candidate_characters: [seo_harin, cheongryu_sect_master, old_archive_keeper]
summary: 청류문이 주인공을 보호하지만, 신분은 정식 제자가 아니라 수습생/객식/잡역/임시 보호 대상이다.
setup_text: 청류문 장문인은 주인공을 위아래로 훑어본다. “무공도 없고, 신분도 없고, 은자도 없고, 말은 반쯤 미쳤구나.” 보호는 공짜가 아니다. 목숨값, 치료비, 숙식비를 갚아야 한다. 처음 맡겨진 일은 무공 수련이 아니라 장작 패기, 물 긷기, 연무장 청소, 약초 말리기, 서고 정리다.
choice_shapes:
  - id: accept_three_month_trial
    role: safe_acceptance
    expected_costs: [debt_or_time]
    expected_gains: [cheongryu_apprentice_status]
  - id: request_martial_training_immediately
    role: impatience_probe
    expected_costs: [relation_risk]
    expected_gains: [training_rule_clue]
  - id: organize_chores_like_workflow
    role: workplace_skill_translation
    expected_costs: [sanity_small]
    expected_gains: [efficiency_reputation_small]
  - id: inspect_archive_during_chore
    role: risky_curiosity
    expected_costs: [suspicion_or_fatigue]
    expected_gains: [cheonggi_record_foreshadow]
outcome_hooks:
  possible_flags: [cheongryu_trial_started, sect_debt_accepted, chore_training_open]
  possible_clues: [training_starts_with_labor, old_archive_locked]
  possible_items: [work_chore_token]
  possible_relations: [seo_harin_mentor_thread, sect_master_watch]
main_spine_link: 소속/채무/거점/훈련을 열어 공용 RPG 시스템이 office 밖에서도 성립하는지 검증한다.
randomization_notes: route_key hub intro. 이후 반복 잡일 카드는 별도 deck으로 분리할 수 있다.
promotion_notes: 첫 runtime에서는 location/state schema를 넓히지 않고 narrative outcome과 flags로만 표현한다.
```

## 5. `wuxia_cheonggi_record_first_fragment`

```yaml
id: wuxia_cheonggi_record_first_fragment
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: candidate
phase: cheonggi_record_awakening
priority_class: route_key
location_tags: [cheongryu_sect, archive, notebook, training]
surface: [cheonggi_record, fragment_choice]
anomaly_type: notebook_oracle
pressure_type: [sanity, relation]
npc_slots: [archive_keeper, cheonggi_record_keeper]
candidate_characters: [old_archive_keeper, yeon_soha]
summary: 업무수첩이 천기록과 연결되고, 현대 지식 후보 세 개 중 하나만 남기는 첫 천외편린 발현이 일어난다.
setup_text: 수련과 잡일에 지쳐 쓰러질 듯한 밤, 업무수첩의 빈 장에 먹물이 번지듯 글자가 떠오른다. 검색창도 질문란도 아니다. 세 개의 문장이 차례로 선명해진다. ‘호신 자세의 기본’, ‘발을 멈추지 않는 법’, ‘실패 기록’. 하나의 편린만 기록할 수 있고, 나머지는 흐려진다.
choice_shapes:
  - id: choose_guard_basics
    role: defensive_growth_choice
    expected_costs: [fragment_lockout_two_options]
    expected_gains: [defense_training_thread]
  - id: choose_keep_feet_moving
    role: mobility_growth_choice
    expected_costs: [fragment_lockout_two_options]
    expected_gains: [evasion_training_thread]
  - id: choose_failure_log
    role: reflection_growth_choice
    expected_costs: [fragment_lockout_two_options]
    expected_gains: [defeat_review_thread]
  - id: close_notebook_without_choice
    role: safe_delay
    expected_costs: [opportunity_may_fade]
    expected_gains: [cheonggi_record_caution]
outcome_hooks:
  possible_flags: [cheonggi_record_awakened, first_fragment_seen]
  possible_clues: [notebook_is_not_search, fragments_are_training_directions]
  possible_items: [cheonggi_record_notebook]
  possible_growth_threads: [defense_training_thread, evasion_training_thread, defeat_review_thread]
main_spine_link: updated wuxia story의 핵심 성장 구조인 천기록/천외편린을 연다.
randomization_notes: 너무 자주 뜨면 안 된다. 큰 전투 후, 심각한 패배 후, 수련 한계, 중요한 선택 직전 같은 특별한 순간에만 사용한다.
promotion_notes: 첫 구현에서는 새 ability/reward schema를 열지 말고 flag/clue/log/presentation text로만 처리한다. 3택 보상 시스템은 별도 설계 후 승격한다.
```

## 6. `wuxia_cheongryu_raid_route_split`

```yaml
id: wuxia_cheongryu_raid_route_split
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: candidate
phase: [cheongryu_raid, route_commitment]
priority_class: route_key
location_tags: [cheongryu_sect, raid, faction_choice]
surface: [sect_raid, faction_negotiation]
anomaly_type: faction_pressure
pressure_type: [danger, relation, sanity]
npc_slots: [righteous_ally, sapa_ally, cheonggi_record_keeper, blood_moon_antagonist]
candidate_characters: [namgung_seoyun, dowol, yeon_soha, yu_harin]
summary: 청류문 습격 사건 후 백도맹, 흑천련, 천기각이 서로 다른 명분과 대가를 제시하며 route commitment를 압박한다.
setup_text: 혈월교의 습격으로 청류문이 무너진다. 백도맹은 늦게 도착하고, 흑천련은 거래를 제안하며, 천기각은 주인공에게 도망치라고 한다. 천기록은 조용히 떨리고, 서하린은 피 묻은 소매로 연무장 문을 막아 선다. 어느 편도 완전히 선하거나 안전하지 않다.
choice_shapes:
  - id: defend_cheongryu_with_white_path
    role: righteous_route_commitment
    expected_costs: [political_debt, danger_medium]
    expected_gains: [righteous_route_flag, cheongryu_rebuild_thread]
  - id: trade_with_black_heaven
    role: sapa_route_commitment
    expected_costs: [trust_loss, debt_medium]
    expected_gains: [sapa_route_flag, survival_resources]
  - id: follow_heavenly_archive
    role: return_truth_route_commitment
    expected_costs: [cheongryu_relation_risk]
    expected_gains: [cheonggi_return_route_flag, world_rift_clue]
  - id: evacuate_the_wounded_first
    role: safe_human_priority
    expected_costs: [route_delay]
    expected_gains: [relation_gain, wounded_saved_flag]
outcome_hooks:
  possible_flags: [cheongryu_raid_survived, route_commitment_pressure]
  possible_route_flags: [righteous_route_started, sapa_route_started, cheonggi_return_route_started]
  possible_clues: [martial_knowledge_conflict, cheonggi_record_targeted]
  possible_relations: [seo_harin_loyalty_test, faction_debt]
main_spine_link: 중반의 큰 분기점으로 정파/사파/천기·귀환 루트를 연다.
randomization_notes: 보스/대형 사건급 route_key. 충분한 공통 루트와 천기록 각성 후에만 사용한다.
promotion_notes: 첫 runtime prototype 후보는 아니다. route system과 storypack gating이 생긴 뒤 중반 slice로 검토한다.
```

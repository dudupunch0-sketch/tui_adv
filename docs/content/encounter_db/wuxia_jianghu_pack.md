# 이구학지 — 천기록 encounter situation cards

Status: candidate + `wuxia_seoharin_left_meal` preview runtime implemented

이 문서는 `docs/content/storypacks/wuxia_jianghu_pack.md`의 후보 인카운터를 runtime YAML 승격 전/후 상황 카드로 정리한다. `wuxia_commute_rift_arrival`부터 `wuxia_seoharin_left_meal`까지는 separate storypack preview runtime으로 승격되었다.

공통 원칙:

- 모든 카드는 `world_id: wuxia_jianghu`, `storypack_id: wuxia_jianghu_pack`에 속한다.
- 현재 단계에서는 이 문서의 JSON/YAML형 카드가 runtime source of truth는 아니다. `wuxia_commute_rift_arrival`부터 `wuxia_seoharin_left_meal`까지는 `src/tui_adv/storypack-previews/wuxia_jianghu_pack/`의 preview source와 별도 generated preview bundle에 반영됐다. `wuxia_seoharin_left_meal_followup`은 `docs/design/Wuxia_Final_State_Routing.md`로 final state routing contract를 먼저 고정하기로 완료됐고, 다음 runtime 후보는 `wuxia_sado_final_phase_1_price_tag`다.
- 최신 canonical 무협 설정은 **이구학지 — 천기록**이다. 이전의 generic 객잔/소림/무당/아미 placeholder는 superseded로 본다.
- 플레이어 전제는 “현대 회사원이 본인 몸과 출근복장 그대로 무협 세계의 시장 한복판에 전이됐다”이다.
- 선택지는 세부 수치보다 역할과 결과 hook을 먼저 정의한다.
- 최소 하나의 안전한 관찰/후퇴/fallback 선택지를 둔다.
- preview runtime 승격은 office storypack과 섞이지 않도록 separate preview mode를 유지한다.
- 실제 회사명, 실제 통근 경로, 실제 사원증 정보, 현실 종교/정치/민족 소재처럼 보이는 세부사항은 쓰지 않는다.

2026-06-01 Notion source precedence:

- `무협 스토리팩: 이구학지 — 천기록` 상위 문서는 synopsis/초기 기획으로 보고, 최신 사건 운영 기준은 하위 문서와 `09. 이구학지 사건 카드 DB`를 우선한다.
- Notion 사건 카드 DB는 26개 row를 가진 authoritative design source지만, repo runtime이나 machine-readable mirror가 곧바로 26개 구현 완료라는 뜻은 아니다.
- Repo의 현재 구현/후보 체계는 이 문서와 `docs/dev/Notion_Design_Coverage.md`에 매핑한 뒤 별도 runtime slice에서만 승격한다.
- 후일담 카드 DB 17개는 future design source이며, 이 문서에서는 runtime encounter 구현 범위로 다루지 않는다.

## Notion 사건 카드 DB mapping

2026-06-01 live check 기준. `repo mapping`이 `none yet`인 row는 아직 repo encounter 후보로 구체화하지 않은 future design source다.

| Notion event id | Notion name | repo mapping | status |
|---|---|---|---|
| `wuxia_seoharin_unsaid_stay` | 가지 말라는 말 | none yet | future 서하린 late/return event |
| `wuxia_seoharin_left_meal` | 남겨둔 밥 | `wuxia_seoharin_left_meal` | preview runtime implemented; relationship/belonging bridge, not final return choice |
| `wuxia_seoharin_empty_place` | 비워둔 자리 | `wuxia_seoharin_empty_place` | preview runtime implemented; late empty-place bridge, not truth delivery |
| `wuxia_mumyeong_departure_truth_summary` | 무명 이탈의 진실 정리 | `wuxia_mumyeong_departure_truth_summary` | preview runtime implemented; sealed summary, not truth delivery |
| `wuxia_black_serpent_pressures_qingliu` | 흑사방의 청류문 압박 | partial: `wuxia_cheongryu_raid_route_split` background | future pressure/side event |
| `wuxia_mumyeong_copy_style_reveal` | 무명의 카피 무공 공개 | `wuxia_mumyeong_copy_style_reveal` | preview runtime implemented |
| `wuxia_mumyeong_resolution` | 무명 결산 | none yet | future final route event |
| `wuxia_mumyeong_midgame_reunion` | 무명 중반 재회 | `wuxia_mumyeong_midgame_reunion` | preview runtime implemented |
| `wuxia_boss_resolution` | 보스 결산 | none yet | future boss result event |
| `wuxia_mumyeong_first_sighting` | 무명 첫 목격 | `wuxia_mumyeong_first_sighting` | preview runtime implemented |
| `wuxia_mumyeong_first_confrontation` | 무명 첫 대치 | `wuxia_mumyeong_first_confrontation` | preview runtime implemented |
| `wuxia_boss_first_appearance` | 보스 첫 등장 | `wuxia_boss_first_appearance` | preview runtime implemented |
| `wuxia_mumyeong_destroys_orthodox_sect` | 정파 문파 멸문 | `wuxia_mumyeong_destroys_orthodox_sect` | preview runtime implemented |
| `wuxia_mumyeong_awakening` | 무명의 각성 | `wuxia_mumyeong_awakening` | preview runtime implemented |
| `wuxia_boss_recruits_mumyeong` | 흑사방 보스의 스카웃 | `wuxia_boss_recruits_mumyeong` | preview runtime implemented |
| `wuxia_mumyeong_reads_orthodox_style` | 무명의 정파 무공 간파 | `wuxia_mumyeong_reads_orthodox_style` | preview runtime implemented |
| `wuxia_qingliu_attack_after_war` | 무너져가는 청류문 습격 | `wuxia_qingliu_attack_after_war` | preview runtime implemented |
| `wuxia_mumyeong_request_for_aid` | 무명의 도움 요청 | `wuxia_mumyeong_request_for_aid` | preview runtime implemented |
| `wuxia_tianjilu_first_fragment` | 천기록 첫 천외편린 | `wuxia_cheonggi_record_first_fragment` | preview implemented as schema-less foreshadow; full reward schema future |
| `wuxia_seoharin_intervention` | 서하린의 개입 | `wuxia_seo_harin_rescue` | preview implemented as schema-less rescue/protection bridge |
| `wuxia_prologue_commute_rift` | 출근길의 균열 | `wuxia_commute_rift_arrival` | preview implemented |
| `wuxia_qingliu_apprentice_entry` | 청류문 임시 수습생 등록 | `wuxia_cheongryu_apprentice_entry` | preview runtime implemented as apprentice/chore bridge |
| `wuxia_qingliu_first_arrival` | 청류문 첫 도착 | partial: `wuxia_cheongryu_apprentice_entry` / `cheongryu_outer_courtyard` | future arrival/location beat folded into apprentice handoff for now |
| `wuxia_black_serpent_first_trouble` | 흑사방 첫 시비 | `wuxia_heuksa_bang_first_fight` | preview implemented |
| `wuxia_arrival_market_confusion` | 낯선 장터에 떨어지다 | `wuxia_commute_rift_arrival` | preview implemented |
| `wuxia_main_qingliu_eye_001` | 청류안 첫 발현 | none yet | future 청류안/천외편린 growth event |

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
notion_event_mapping:
  notion_event_id: wuxia_seoharin_intervention
  notion_event_name: 서하린의 개입
  mapping_status: preview_runtime_implemented
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
purpose: 첫 난투와 천기록 첫 편린이 남긴 부상/시선/수첩 hook을 관계와 거점으로 연결한다. 목표는 NPC 동료 시스템이 아니라 서하린의 구조, 외지인 조사, 청류문 보호/감시, 다음 `wuxia_cheongryu_apprentice_entry` bridge를 여는 것이다.
start_conditions:
  runtime_mode: storypack_preview
  after_encounters: [wuxia_heuksa_bang_first_fight, wuxia_cheonggi_record_first_fragment]
  phase: rescue_and_investigation
  location: jianghu_market_street
  required_flags: [heuksa_bang_first_fight_resolved, cheonggi_record_first_fragment_resolved]
  forbidden_flags: [seo_harin_rescue_resolved]
  destination_candidate: cheongryu_outer_courtyard
  routing_note: 현재 preview 순서는 first fight 뒤에 천기록 첫 편린 foreshadow를 보여준다. 따라서 rescue는 `cheonggi_record_first_fragment_resolved` 뒤에 붙여 기존 fragment smoke를 가로막지 않는다. full story에서 구조 순간을 먼저 보이도록 순서를 재편하려면 별도 sequence 재배치 slice로 다룬다.
setup_text: 시장 담벼락 아래에서 숨을 고르자 흑사방 말단들이 다시 다가온다. 그때 청류문 외문 제자 서하린이 군중을 가르며 끼어든다. 흑사방 정도는 물릴 수 있지만, 그녀도 주인공을 믿지는 않는다. “무공도 모르는 자가 흑사방 말단을 건드렸다고?” “그 옷차림은 뭐지?” “목에 건 패와 그 수첩은 어느 세력의 표식이냐?”
choice_shapes:
  - id: tell_plain_truth
    role: safe_honesty
    fallback_choice: true
    label_direction: 있는 그대로 길을 잃은 외지인이라고 말한다
    expected_costs: [suspicion_small, sanity_small]
    outcome_hook:
      resources: {sanity: -2}
      add_flags: [seo_harin_rescue_resolved, seo_harin_intervened, taken_under_watch, outsider_claim_recorded, truthful_outsider_claim]
      add_clues: [cheongryu_name_heard, sect_identity_matters]
      destination_id: cheongryu_outer_courtyard
      log_direction: 진실은 설득력이 낮지만 일관성은 있다. 서하린은 믿기보다 감시하기로 결정한다.
  - id: ask_for_medical_help_first
    role: survival_priority
    label_direction: 설명보다 치료와 안전한 곳을 먼저 부탁한다
    expected_costs: [debt_small]
    outcome_hook:
      resources: {health: 4, sanity: -1}
      add_flags: [seo_harin_rescue_resolved, seo_harin_intervened, injury_stabilized, rescue_debt_recorded, taken_under_watch]
      add_clues: [cheongryu_medicine_smells_of_herbs, sect_protection_has_price]
      destination_id: cheongryu_outer_courtyard
      log_direction: 상처는 임시로 묶이지만 목숨값과 치료비라는 단어가 따라붙는다.
  - id: explain_company_and_commute
    role: workplace_memory_probe
    label_direction: 회사와 출근길을 최대한 논리적으로 설명한다
    expected_costs: [sanity_small, misunderstanding]
    outcome_hook:
      resources: {sanity: -3}
      add_flags: [seo_harin_rescue_resolved, seo_harin_intervened, company_words_misunderstood, outsider_claim_recorded]
      add_clues: [company_words_fail_clue, commute_rift_story_sounds_like_madness]
      destination_id: cheongryu_outer_courtyard
      log_direction: 회사, 출근, 엘리베이터라는 말은 강호의 문법으로 번역되지 않는다. 기록할 말과 숨길 말이 갈라진다.
  - id: show_cheonggi_record_page
    role: risky_record_disclosure
    label_direction: 방금 떠오른 천기록의 글자를 조심스럽게 보여준다
    expected_costs: [danger_small, suspicion_medium]
    outcome_hook:
      resources: {sanity: -2}
      danger: 1
      add_flags: [seo_harin_rescue_resolved, seo_harin_intervened, seo_harin_noticed_cheonggi_record, cheonggi_record_must_be_hidden]
      add_clues: [notebook_draws_sect_attention, cheonggi_record_must_be_hidden]
      destination_id: cheongryu_outer_courtyard
      log_direction: 수첩은 도움을 부를 수도 있지만, 이름 붙는 순간 표적이 될 수도 있다.
  - id: hide_employee_badge
    role: high_risk_concealment
    label_direction: 사원증과 수첩을 품 안으로 숨긴다
    expected_costs: [suspicion_medium]
    outcome_hook:
      resources: {sanity: -1}
      danger: 1
      add_flags: [seo_harin_rescue_resolved, seo_harin_intervened, badge_secret_flag, seo_harin_suspicion_raised, taken_under_watch]
      add_clues: [badge_misread_as_sect_token, hiding_marks_you_as_suspicious]
      destination_id: cheongryu_outer_courtyard
      log_direction: 숨긴 물건은 지켜지지만, 숨기는 동작 자체가 의심을 산다.
outcome_hooks:
  possible_flags: [seo_harin_rescue_resolved, seo_harin_intervened, taken_under_watch, outsider_claim_recorded, truthful_outsider_claim, injury_stabilized, rescue_debt_recorded, company_words_misunderstood, seo_harin_noticed_cheonggi_record, cheonggi_record_must_be_hidden, badge_secret_flag, seo_harin_suspicion_raised]
  possible_clues: [cheongryu_name_heard, sect_identity_matters, cheongryu_medicine_smells_of_herbs, sect_protection_has_price, company_words_fail_clue, commute_rift_story_sounds_like_madness, notebook_draws_sect_attention, cheonggi_record_must_be_hidden, badge_misread_as_sect_token, hiding_marks_you_as_suspicious]
  possible_items: []
  possible_relations: [seo_harin_suspicion, seo_harin_responsibility, seo_harin_cautious_trust]
  possible_destinations: [cheongryu_outer_courtyard]
  possible_log_tone:
    - 구조는 구원이 아니라 보호와 감시의 시작이라는 감각
    - 현대어/회사어가 강호에서 오해되는 감각
    - 사원증과 천기록이 도움과 위험을 동시에 부르는 감각
    - 청류문 수습생/채무/잡역 bridge로 넘어가는 압박
schema_boundary:
  allowed_existing_schema: [conditions.locations, required_flags, forbidden_flags, choices.cost, outcome.resources, outcome.danger, outcome.add_flags, outcome.add_clues, outcome.add_items, outcome.remove_items, outcome.destination_id, outcome.log, presentation]
  forbidden_new_schema: [RelationScore, DebtLedger, FactionStanding, healing_schema, companion_schema, reward_schema, ability_schema, CombatState, fragment_choice_reward]
main_spine_link: 구조자/멘토 후보를 세우고, 주인공을 청류문 수습생 구간으로 이동시킨다. `wuxia_cheongryu_apprentice_entry`는 이 encounter의 `seo_harin_rescue_resolved`/`taken_under_watch`/`rescue_debt_recorded` 계열 hook을 받아 진행한다.
randomization_notes: first_brawl/first_fragment 이후 1회성 forced aftermath. 별도 hub random으로 반복하지 않고, `seo_harin_rescue_resolved`로 차단한다.
promotion_notes: preview runtime으로 구현 완료. healing/debt/relation은 새 schema 없이 health/sanity/danger, flags, clues, destination, log로만 표현했고, 기본 office bundle/`escape-office` save key/천외편린 3택 성장 schema는 건드리지 않았다. 이후 apprentice bridge, `wuxia_cheongryu_raid_route_split`, `wuxia_cheongryu_raid_wounded_fallback`도 구현 완료했으며, 다음은 route opener 선택/설계 handoff다.
```

## 4. `wuxia_cheongryu_apprentice_entry`

```yaml
id: wuxia_cheongryu_apprentice_entry
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: candidate
runtime_preview_design_status: implemented_in_storypack_preview
phase: cheongryu_apprenticeship
priority_class: route_key
location_tags: [cheongryu_sect, courtyard, apprenticeship]
surface: [sect_courtyard, training_chore]
anomaly_type: sect_debt
pressure_type: [relation, hunger, health]
npc_slots: [sect_master_guardian, early_rescuer, archive_keeper]
candidate_characters: [seo_harin, cheongryu_sect_master, old_archive_keeper]
summary: 청류문이 주인공을 보호하지만, 신분은 정식 제자가 아니라 수습생/객식/잡역/임시 보호 대상이다.
purpose: 서하린 구조 이후 주인공을 청류문 질서 안에 임시로 넣고, 보호의 대가·잡일·수련 허가·서고 curiosity hook을 연다. 관계/채무/훈련 XP schema가 아니라 기존 flags/clues/log만 사용한다.
setup_text: 청류문 장문인은 주인공을 위아래로 훑어본다. “무공도 없고, 신분도 없고, 은자도 없고, 말은 반쯤 미쳤구나.” 보호는 공짜가 아니다. 목숨값, 치료비, 숙식비를 갚아야 한다. 처음 맡겨진 일은 무공 수련이 아니라 장작 패기, 물 긷기, 연무장 청소, 약초 말리기, 서고 정리다.
runtime_preview_start_conditions:
  runtime_mode: storypack_preview
  prereq: `wuxia_seo_harin_rescue` runtime slice가 먼저 landing되어야 한다.
  location: cheongryu_outer_courtyard
  required_flags: [seo_harin_rescue_resolved, taken_under_watch]
  forbidden_flags: [cheongryu_apprentice_entry_resolved]
  note: `rescue_debt_recorded`는 rescue branch별 optional hook일 수 있으므로 필수 조건으로 요구하지 않는다.
choice_shapes:
  - id: accept_three_month_trial
    role: safe_acceptance
    fallback_choice: true
    label_direction: 석 달 동안 잡일과 수습 조건을 받아들인다
    expected_costs: [debt_or_time, hunger_small]
    expected_gains: [cheongryu_apprentice_status, chore_training_open]
    outcome_hook:
      add_flags: [cheongryu_apprentice_entry_resolved, cheongryu_trial_started, sect_debt_accepted, chore_training_open]
      add_clues: [training_starts_with_labor, protection_is_not_membership, sect_rules_written_in_chores]
      add_items: [work_chore_token]
      log_direction: 보호는 공짜가 아니며, 잡일은 벌이 아니라 수련의 입구라는 압박을 남긴다.
  - id: request_martial_training_immediately
    role: impatience_probe
    label_direction: 지금 당장 무공을 가르쳐 달라고 요구한다
    expected_costs: [relation_risk, danger_small]
    expected_gains: [training_rule_clue, sect_master_watch]
    outcome_hook:
      add_flags: [cheongryu_apprentice_entry_resolved, cheongryu_trial_started, training_request_denied, sect_rules_explained, sect_master_watch]
      add_clues: [training_requires_chore_credit, protection_is_not_membership]
      log_direction: 무공은 동정으로 주는 것이 아니라 문파 규칙과 책임 안에서 허가된다는 톤을 남긴다.
  - id: organize_chores_like_workflow
    role: workplace_skill_translation
    label_direction: 회사식 업무 분해로 잡일 동선을 정리한다
    expected_costs: [sanity_small, fatigue_small]
    expected_gains: [efficiency_reputation_small, workflow_translation_clue]
    outcome_hook:
      add_flags: [cheongryu_apprentice_entry_resolved, cheongryu_trial_started, modern_workflow_noticed, chore_roster_rewritten]
      add_clues: [workflow_thinking_translates_to_training, sect_rules_written_in_chores]
      log_direction: 현대 회사원의 분해·기록 습관이 무공 치트가 아니라 잡일 효율과 관찰력으로 먼저 번역된다.
  - id: inspect_archive_during_chore
    role: risky_curiosity
    label_direction: 서고 정리 중 잠긴 낡은 장부를 살핀다
    expected_costs: [suspicion_or_fatigue, sanity_small]
    expected_gains: [cheonggi_record_foreshadow, archive_hook]
    outcome_hook:
      add_flags: [cheongryu_apprentice_entry_resolved, cheongryu_trial_started, old_archive_locked_seen, archive_curiosity_marked]
      add_clues: [old_archive_locked, cheonggi_record_resonates_near_archive]
      log_direction: 서고는 천기록/천기각 future hook을 암시하지만, 이번 slice에서 3택 성장 UI를 직접 열지 않는다.
outcome_hooks:
  possible_flags: [cheongryu_apprentice_entry_resolved, cheongryu_trial_started, sect_debt_accepted, chore_training_open, training_request_denied, sect_rules_explained, modern_workflow_noticed, chore_roster_rewritten, old_archive_locked_seen, archive_curiosity_marked, seo_harin_mentor_thread, sect_master_watch]
  possible_clues: [training_starts_with_labor, protection_is_not_membership, training_requires_chore_credit, workflow_thinking_translates_to_training, old_archive_locked, cheonggi_record_resonates_near_archive, sect_rules_written_in_chores]
  possible_items: [work_chore_token]
  possible_destinations: [cheongryu_outer_courtyard]
  possible_relations: [seo_harin_mentor_thread, sect_master_watch]
  possible_log_tone:
    - 보호는 공짜가 아니고 채무와 규칙을 만든다는 감각
    - 잡일은 벌이 아니라 청류문식 수련의 입구라는 감각
    - 현대 회사원의 효율화/기록 습관이 무협 surface에 번역되는 감각
    - 서고와 천기록은 암시만 남기고 reward UI는 열지 않는 감각
schema_boundary:
  allowed_existing_schema: [conditions.locations, required_flags, forbidden_flags, choices.cost, outcome.resources, outcome.danger, outcome.add_flags, outcome.add_clues, outcome.add_items, outcome.remove_items, outcome.destination_id, outcome.log, presentation]
  forbidden_new_schema: [RelationScore, DebtLedger, FactionStanding, TrainingXP, ChoreScheduler, companion_schema, reward_schema, ability_schema, CombatState, fragment_choice_reward]
main_spine_link: 소속/채무/거점/훈련을 열어 공용 RPG 시스템이 office 밖에서도 성립하는지 검증한다. 이 card는 `wuxia_seo_harin_rescue`가 남긴 `seo_harin_rescue_resolved`/`taken_under_watch` hook을 받은 뒤 진행한다.
randomization_notes: route_key hub intro. rescue 직후 forced bridge로 1회만 사용하고, 이후 반복 잡일/서고 카드는 별도 deck으로 분리할 수 있다.
promotion_notes: preview runtime으로 구현 완료. 첫 runtime은 location/state schema를 넓히지 않고 narrative outcome, `work_chore_token`, flags/clues/log/presentation으로만 표현했다. 이후 `wuxia_cheongryu_raid_route_split`와 `wuxia_cheongryu_raid_wounded_fallback`도 구현 완료했으며, 다음은 route opener 선택/설계 handoff다.
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
purpose: 첫 난투의 실패를 천기록/천외편린 future hook으로 연결한다. runtime preview에서는 완전한 reward/ability 성장 UI가 아니라 thread flag, clue, log, presentation만 남긴다.
setup_text: 수련과 잡일에 지쳐 쓰러질 듯한 밤, 업무수첩의 빈 장에 먹물이 번지듯 글자가 떠오른다. 검색창도 질문란도 아니다. 세 개의 문장이 차례로 선명해진다. ‘호신 자세의 기본’, ‘발을 멈추지 않는 법’, ‘실패 기록’. 하나의 편린만 기록할 수 있고, 나머지는 흐려진다.
runtime_preview_start_conditions:
  runtime_mode: storypack_preview
  implemented_entrypoint: 첫 난투 직후 foreshadow version
  location: jianghu_market_street
  required_flags: [heuksa_bang_first_fight_resolved]
  forbidden_flags: [cheonggi_record_first_fragment_resolved]
  note: full story상 정식 발현은 청류문 수습생/서고 구간에서 회수한다. 현재 preview는 bridge 전 foreshadow slice다.
choice_shapes:
  - id: choose_guard_basics
    role: defensive_growth_choice
    label_direction: "'호신 자세의 기본' 문장을 붙든다"
    expected_costs: [sanity_small, fragment_lockout_two_options]
    outcome_hook:
      add_items: [cheonggi_record_notebook]
      add_flags: [cheonggi_record_awakened, first_fragment_seen, cheonggi_record_first_fragment_resolved, cheonggi_fragment_guard_basics_thread]
      add_clues: [notebook_is_not_search, fragments_are_training_directions]
  - id: choose_keep_feet_moving
    role: mobility_growth_choice
    label_direction: "'발을 멈추지 않는 법'을 남긴다"
    expected_costs: [sanity_small, fragment_lockout_two_options]
    outcome_hook:
      add_items: [cheonggi_record_notebook]
      add_flags: [cheonggi_record_awakened, first_fragment_seen, cheonggi_record_first_fragment_resolved, cheonggi_fragment_footwork_thread]
      add_clues: [fragments_are_training_directions, motion_matters_more_than_pose]
  - id: choose_failure_log
    role: reflection_growth_choice
    label_direction: "'실패 기록'을 받아들인다"
    expected_costs: [sanity_medium, fragment_lockout_two_options]
    outcome_hook:
      add_items: [cheonggi_record_notebook]
      add_flags: [cheonggi_record_awakened, first_fragment_seen, cheonggi_record_first_fragment_resolved, cheonggi_fragment_failure_log_thread]
      add_clues: [notebook_is_not_search, failures_can_be_training_material]
  - id: close_notebook_without_choice
    role: safe_delay
    fallback_choice: true
    label_direction: 수첩을 덮고 호흡부터 고른다
    expected_costs: [sanity_small]
    outcome_hook:
      add_items: [cheonggi_record_notebook]
      add_flags: [cheonggi_record_awakened, first_fragment_seen, cheonggi_record_first_fragment_resolved, cheonggi_record_caution]
      add_clues: [notebook_is_not_search]
outcome_hooks:
  possible_flags: [cheonggi_record_awakened, first_fragment_seen, cheonggi_record_first_fragment_resolved, cheonggi_fragment_guard_basics_thread, cheonggi_fragment_footwork_thread, cheonggi_fragment_failure_log_thread, cheonggi_record_caution]
  possible_clues: [notebook_is_not_search, fragments_are_training_directions, motion_matters_more_than_pose, failures_can_be_training_material]
  possible_items: [cheonggi_record_notebook]
  possible_growth_threads: [defense_training_thread, evasion_training_thread, defeat_review_thread]
schema_boundary:
  allowed_existing_schema: [conditions.locations, required_flags, forbidden_flags, choices.cost, outcome.resources, outcome.add_items, outcome.add_flags, outcome.add_clues, outcome.log, presentation]
  forbidden_new_schema: [reward_schema, ability_schema, fragment_choice_reward, fragment_lockout_state, full_fragment_choice_ui, CombatState]
main_spine_link: updated wuxia story의 핵심 성장 구조인 천기록/천외편린을 연다.
randomization_notes: 너무 자주 뜨면 안 된다. 큰 전투 후, 심각한 패배 후, 수련 한계, 중요한 선택 직전 같은 특별한 순간에만 사용한다.
promotion_notes: preview runtime으로 구현했다. 첫 구현은 새 ability/reward schema를 열지 않고 flag/clue/log/presentation text로만 처리한다. 3택 보상 시스템은 별도 설계 후 승격한다.
```

## 6. `wuxia_cheongryu_chore_sparring`

```yaml
id: wuxia_cheongryu_chore_sparring
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: candidate
runtime_preview_design_status: implemented_in_storypack_preview
phase: [cheongryu_apprenticeship, basic_combat_training]
priority_class: npc_relation
location_tags: [cheongryu_sect, training_ground, chore_yard]
surface: [sect_courtyard, training_chore, office_items]
anomaly_type: [first_brawl_defeat, qi_deviation, notebook_oracle]
pressure_type: [health, sanity, hunger, relation]
npc_slots: [early_rescuer, sect_master_guardian]
candidate_characters: [seo_harin, cheongryu_outer_disciple, cheongryu_sect_master]
summary: 청류문 잡일 중 장작 마당에서 첫 몸싸움식 겨루기를 겪고, 흑사방 난투/office item 경험을 균형·호흡·발디딤 수련으로 번역한다.
purpose: 청류문 수습생 구간을 따뜻한 언더독 homebase의 반복 수련 루프로 확장한다. 무공 치트나 내부 적대가 아니라 잡일, 균형, 호흡, 서하린 멘토링을 통해 성장 hook을 남긴다.
setup_text: 장작을 옮기던 중 외문 제자가 장난처럼 어깨를 밀어 온다. 정식 대련도 무공 수업도 아니지만, 장작 더미와 흙먼지 사이에서 몸이 밀리는 순간 첫 난투와 물품창고에서 배운 거리와 균형이 같은 문법으로 겹친다.
runtime_preview_start_conditions:
  runtime_mode: storypack_preview
  location: cheongryu_outer_courtyard
  required_flags: [cheongryu_apprentice_entry_resolved, cheongryu_trial_started, cheonggi_record_awakened, first_fragment_seen]
  forbidden_flags: [cheongryu_chore_sparring_resolved]
  note: 자기 resolved flag를 required와 forbidden에 동시에 넣지 않는다.
choice_shapes:
  - id: step_back_with_firewood
    role: safe_reposition
    fallback_choice: true
    label_direction: 장작을 떨어뜨리지 않고 반걸음 물러난다
    expected_costs: [hunger_small]
    expected_gains: [balance_training_noticed, office_combat_model_reused]
  - id: let_shoulder_turn_with_push
    role: flow_response
    label_direction: 밀리는 힘을 거스르지 않고 어깨를 돌린다
    expected_costs: [sanity_small]
    expected_gains: [cheongryu_flow_reading_started, pressure_can_be_turned]
  - id: plant_bare_foot_in_dust
    role: grounded_body_check
    label_direction: 흙먼지에 발을 박아 미끄러짐을 멈춘다
    expected_costs: [health_small]
    expected_gains: [footwork_training_grounded, shoes_and_suit_lessons_reused]
  - id: ask_harin_what_changed
    role: mentor_question
    label_direction: 방금 왜 덜 밀렸는지 서하린에게 묻는다
    expected_costs: [sanity_small]
    expected_gains: [seo_harin_mentor_thread, harin_names_balance_and_breath]
outcome_hooks:
  possible_flags: [cheongryu_chore_sparring_resolved, chore_sparring_completed, balance_training_noticed, office_combat_model_reused, cheongryu_flow_reading_started, pressure_can_be_turned, footwork_training_grounded, shoes_and_suit_lessons_reused, seo_harin_mentor_thread, harin_names_balance_and_breath]
  possible_clues: [balance_matters_more_than_force, office_items_can_translate_to_training, breath_changes_balance, posture_starts_from_ground, mentor_explains_balance_breath, flow_axes_are_trainable]
  possible_destinations: [cheongryu_outer_courtyard]
schema_boundary:
  allowed_existing_schema: [conditions.locations, required_flags, forbidden_flags, choices.cost, outcome.resources, outcome.add_flags, outcome.add_clues, outcome.destination_id, outcome.log, presentation]
  forbidden_new_schema: [CombatState, combat_resolver, skill_tree, reward_schema, ability_schema, relation_score]
main_spine_link: 청류문 수습생 bridge와 raid route split 사이의 1회성 training beat. 현대인의 관찰/몸싸움 경험이 청류문식 균형·호흡 수련으로 번역될 수 있음을 보여준다.
randomization_notes: route split 전 1회성 forced bridge로만 사용한다. 반복 대련 deck이나 전투 시스템으로 확장하지 않는다.
promotion_notes: preview runtime으로 구현 완료. 기본 office bundle과 storypack preview boundary를 유지했고, 새 combat/reward/ability schema는 열지 않는다.
```

## 7. `wuxia_cheongryu_raid_route_split`

```yaml
id: wuxia_cheongryu_raid_route_split
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: candidate
runtime_preview_design_status: implemented_in_storypack_preview
phase: [cheongryu_raid, route_commitment]
priority_class: route_key
location_tags: [cheongryu_sect, raid, faction_choice]
surface: [sect_raid, faction_negotiation]
anomaly_type: faction_pressure
pressure_type: [danger, relation, sanity]
npc_slots: [righteous_ally, sapa_ally, cheonggi_record_keeper, blood_moon_antagonist]
candidate_characters: [namgung_seoyun, dowol, yeon_soha, yu_harin]
summary: 청류문 습격 사건 후 백도맹, 흑천련, 천기각이 서로 다른 명분과 대가를 제시하며 route commitment를 압박한다.
purpose: 청류문 공통 루트가 충분히 쌓인 뒤 혈월교 습격을 통해 정파/사파/천기·귀환 축의 route pressure를 처음 노출한다. 완전한 faction reputation, route graph, ending system이 아니라 route flag/clue/log hook만 남긴다.
setup_text: 혈월교의 습격으로 청류문이 무너진다. 백도맹은 늦게 도착하고, 흑천련은 거래를 제안하며, 천기각은 주인공에게 도망치라고 한다. 천기록은 조용히 떨리고, 서하린은 피 묻은 소매로 연무장 문을 막아 선다. 어느 편도 완전히 선하거나 안전하지 않다.
runtime_preview_start_conditions:
  runtime_mode: storypack_preview
  prereq: rescue/apprentice runtime slice가 먼저 landing되어야 한다.
  location: cheongryu_outer_courtyard
  required_flags: [cheongryu_apprentice_entry_resolved, cheongryu_trial_started, cheonggi_record_awakened, first_fragment_seen]
  forbidden_flags: [cheongryu_raid_route_split_resolved]
  note: fragment branch별 thread flag는 flavor로만 쓰고 eligibility에는 공통 `first_fragment_seen`을 사용한다.
choice_shapes:
  - id: evacuate_the_wounded_first
    role: safe_human_priority
    fallback_choice: true
    label_direction: 부상자를 먼저 빼내고 선택을 미룬다
    expected_costs: [route_delay, danger_small]
    expected_gains: [relation_gain, wounded_saved_flag, seo_harin_survived_raid]
    outcome_hook:
      add_flags: [cheongryu_raid_route_split_resolved, cheongryu_raid_survived, route_commitment_pressure, route_commitment_deferred, wounded_saved_flag, seo_harin_survived_raid]
      add_clues: [saving_people_delays_route_choice, blood_moon_targets_cheonggi_record]
      log_direction: 선택을 미뤘지만 아무것도 선택하지 않은 것은 아니며, 사람을 구한 대가와 지연을 남긴다.
  - id: defend_cheongryu_with_white_path
    role: righteous_route_commitment
    label_direction: 백도맹 지원을 받아 청류문을 방어한다
    expected_costs: [political_debt, danger_medium]
    expected_gains: [righteous_route_flag, cheongryu_rebuild_thread]
    outcome_hook:
      add_flags: [cheongryu_raid_route_split_resolved, cheongryu_raid_survived, route_commitment_pressure, righteous_route_started, baekdo_alliance_debt, cheongryu_rebuild_thread]
      add_clues: [white_path_help_has_price, martial_knowledge_conflict]
      log_direction: 정파의 도움은 질서와 명분을 주지만, 청류문과 천기록을 정치적 빚 안에 묶는다.
  - id: trade_with_black_heaven
    role: sapa_route_commitment
    label_direction: 흑천련 도월과 거래해 탈출로를 산다
    expected_costs: [trust_loss, debt_medium]
    expected_gains: [sapa_route_flag, survival_resources]
    outcome_hook:
      add_flags: [cheongryu_raid_route_split_resolved, cheongryu_raid_survived, route_commitment_pressure, sapa_route_started, black_heaven_deal_marked, dowol_debt]
      add_clues: [black_heaven_bargain_has_teeth, martial_knowledge_conflict]
      log_direction: 사파의 거래는 빠른 생존을 주지만, 이후 갚아야 할 이름과 빚을 남긴다.
  - id: follow_heavenly_archive
    role: return_truth_route_commitment
    label_direction: 천기각 기록관을 따라 천기록의 출처를 쫓는다
    expected_costs: [cheongryu_relation_risk, sanity_small]
    expected_gains: [cheonggi_return_route_flag, world_rift_clue]
    outcome_hook:
      add_flags: [cheongryu_raid_route_split_resolved, cheongryu_raid_survived, route_commitment_pressure, cheonggi_return_route_started, heavenly_archive_contact, cheonggi_record_targeted]
      add_clues: [heavenly_archive_knows_previous_outsiders, blood_moon_targets_cheonggi_record]
      log_direction: 귀환과 진실의 단서는 가까워지지만, 청류문을 떠나는 죄책감과 감시를 남긴다.
outcome_hooks:
  possible_flags: [cheongryu_raid_route_split_resolved, cheongryu_raid_survived, route_commitment_pressure, route_commitment_deferred, wounded_saved_flag, seo_harin_survived_raid, righteous_route_started, baekdo_alliance_debt, cheongryu_rebuild_thread, sapa_route_started, black_heaven_deal_marked, dowol_debt, cheonggi_return_route_started, heavenly_archive_contact, cheonggi_record_targeted]
  possible_route_flags: [righteous_route_started, sapa_route_started, cheonggi_return_route_started, route_commitment_deferred]
  possible_clues: [martial_knowledge_conflict, blood_moon_targets_cheonggi_record, white_path_help_has_price, black_heaven_bargain_has_teeth, heavenly_archive_knows_previous_outsiders, saving_people_delays_route_choice]
  possible_relations: [seo_harin_loyalty_test, faction_debt, namgung_seoyun_notice, dowol_debt, heavenly_archive_contact]
  possible_destinations: [cheongryu_outer_courtyard, cheongryu_raid_courtyard, raid_aftermath_shelter]
  possible_log_tone:
    - 어느 편도 완전히 선하거나 안전하지 않다는 감각
    - 선택하지 않는 것도 대가와 지연을 만든다는 감각
    - 천기록이 구조물이 아니라 세력들이 노리는 물건으로 바뀌는 감각
    - route flag는 남기되 ending/route graph 구현은 열지 않는 감각
schema_boundary:
  allowed_existing_schema: [conditions.locations, required_flags, forbidden_flags, choices.cost, outcome.resources, outcome.danger, outcome.add_flags, outcome.add_clues, outcome.add_items, outcome.remove_items, outcome.destination_id, outcome.log, presentation]
  forbidden_new_schema: [FactionStanding, RouteGraph, BranchLock, CompanionDeath, MassCombat, boss_combat_resolver, reward_schema, ability_schema, fragment_choice_reward, multi_ending_implementation]
main_spine_link: 중반의 큰 분기점으로 정파/사파/천기·귀환 루트 압박을 연다. 이 card는 rescue/apprentice와 first-fragment 공통 hook이 runtime에 들어간 뒤에만 사용한다.
randomization_notes: 보스/대형 사건급 route_key. 충분한 공통 루트, 청류문 수습생 hook, 천기록 각성 hook 뒤에 1회성 forced route pressure로 사용한다.
promotion_notes: preview runtime으로 구현 완료. 첫 raid runtime은 route flag/clue/log/presentation만 남겼고, route graph, faction reputation, boss combat, ending implementation은 별도 slice로 둔다. fallback `evacuate_the_wounded_first`는 `route_commitment_deferred`/`wounded_saved_flag`를 남겨 구현된 `wuxia_cheongryu_raid_wounded_fallback`으로 이어진다.
```

## 8. `wuxia_cheongryu_raid_wounded_fallback`

```yaml
id: wuxia_cheongryu_raid_wounded_fallback
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: candidate
runtime_preview_design_status: implemented_in_storypack_preview
phase: [cheongryu_raid, route_commitment]
priority_class: route_key
location_tags: [cheongryu_sect, raid, wounded_shelter, faction_choice]
surface: [sect_raid, faction_negotiation, sect_courtyard]
anomaly_type: [faction_pressure, sect_debt]
pressure_type: [health, danger, relation, sanity]
npc_slots: [early_rescuer, righteous_ally, sapa_ally, cheonggi_record_keeper]
candidate_characters: [seo_harin, namgung_seoyun, dowol, yeon_soha]
summary: raid split에서 부상자를 먼저 대피시킨 뒤, route 선택을 미룬 대가와 신뢰를 기록하고 정파/사파/천기 route pressure를 다시 연다.
purpose: `evacuate_the_wounded_first` fallback을 dead-end가 아니라 공통 재합류 branch로 만든다. 사람을 구한 선택의 보상과 지연 비용을 flags/clues/log로 남기되, route graph/faction reputation schema는 열지 않는다.
setup_text: 날이 밝기 전, 임시 피난처에는 숨을 고르는 사람들의 이름이 하나씩 불린다. 서하린은 피 묻은 소매를 감추지 못하고, 백도맹의 약상자는 문장 깃발 아래 놓여 있으며, 흑천련의 붕대는 값표가 붙은 듯하다. 천기록의 빈 장에는 부상자 동선이 지도처럼 번진다. 선택을 미뤘지만, 세계는 그 결정을 잊지 않았다.
runtime_preview_start_conditions:
  runtime_mode: storypack_preview
  prereq: rescue/apprentice/raid split runtime slice가 먼저 landing되었다.
  location: cheongryu_outer_courtyard
  fallback_location_if_no_new_location: null
  required_flags: [cheongryu_raid_route_split_resolved, route_commitment_deferred, wounded_saved_flag, cheongryu_raid_survived]
  forbidden_flags: [cheongryu_raid_wounded_fallback_resolved]
  note: seo_harin_survived_raid는 flavor hook으로 우선 사용하고 eligibility 필수 조건으로 만들지 않는다.
choice_shapes:
  - id: stabilize_wounded_until_dawn
    role: safe_deferred_recovery
    fallback_choice: true
    label_direction: 새벽까지 부상자를 안정시키고 명단을 맞춘다
    expected_costs: [route_delay, fatigue_small]
    expected_gains: [survivor_roll_call_complete, trust_from_wounded]
    outcome_hook:
      add_flags: [cheongryu_raid_wounded_fallback_resolved, deferred_route_reopened, route_commitment_deferred, wounded_shelter_stabilized, survivor_roll_call_complete, route_delay_cost_recorded]
      add_clues: [saving_people_changed_witnesses, deferred_choice_is_still_choice]
      log_direction: 사람을 구한 선택은 신뢰를 만들지만, route pressure는 사라지지 않고 더 구체적인 대가로 돌아온다.
  - id: ask_baekdo_for_medicine_not_command
    role: delayed_righteous_commitment
    label_direction: 백도맹에 명령이 아니라 약과 호위를 요청한다
    expected_costs: [political_debt, autonomy_risk]
    expected_gains: [righteous_route_flag, medicine_support]
    outcome_hook:
      add_flags: [cheongryu_raid_wounded_fallback_resolved, deferred_route_reopened, righteous_route_started, baekdo_medicine_debt, cheongryu_rebuild_thread]
      add_clues: [medicine_has_banner, white_path_help_has_price]
      log_direction: 정파의 약상자는 사람을 살리지만, 깃발 아래 놓인 도움은 정치적 빚을 남긴다.
  - id: trade_black_heaven_bandages_for_exit
    role: delayed_sapa_bargain
    label_direction: 흑천련의 붕대와 탈출로를 거래한다
    expected_costs: [debt_medium, trust_loss]
    expected_gains: [sapa_route_flag, exit_route_hint]
    outcome_hook:
      add_flags: [cheongryu_raid_wounded_fallback_resolved, deferred_route_reopened, sapa_route_started, black_heaven_escape_marker, dowol_debt]
      add_clues: [black_heaven_help_marks_debt, black_heaven_bargain_has_teeth]
      log_direction: 사파의 도움은 빠르고 실용적이지만, 붕대 매듭마다 갚아야 할 이름이 묶인다.
  - id: follow_archive_triage_map
    role: delayed_return_truth_thread
    label_direction: 천기각 기록관의 부상자 동선 지도를 따른다
    expected_costs: [sanity_small, cheongryu_relation_risk]
    expected_gains: [cheonggi_return_route_flag, previous_outsider_trace]
    outcome_hook:
      add_flags: [cheongryu_raid_wounded_fallback_resolved, deferred_route_reopened, cheonggi_return_route_started, heavenly_archive_triage_map_seen, cheonggi_record_targeted]
      add_clues: [archive_records_count_the_living, heavenly_archive_knows_previous_outsiders]
      log_direction: 천기각의 지도는 산 사람의 동선을 세지만, 그 선은 주인공이 돌아갈 수 있는 균열과도 이어진다.
outcome_hooks:
  possible_flags: [cheongryu_raid_wounded_fallback_resolved, deferred_route_reopened, route_commitment_deferred, wounded_shelter_stabilized, survivor_roll_call_complete, route_delay_cost_recorded, righteous_route_started, baekdo_medicine_debt, cheongryu_rebuild_thread, sapa_route_started, black_heaven_escape_marker, dowol_debt, cheonggi_return_route_started, heavenly_archive_triage_map_seen, cheonggi_record_targeted]
  possible_route_flags: [righteous_route_started, sapa_route_started, cheonggi_return_route_started, route_commitment_deferred]
  possible_clues: [saving_people_changed_witnesses, deferred_choice_is_still_choice, medicine_has_banner, white_path_help_has_price, black_heaven_help_marks_debt, black_heaven_bargain_has_teeth, archive_records_count_the_living, heavenly_archive_knows_previous_outsiders]
  possible_relations: [trust_from_wounded, seo_harin_respect_thread, baekdo_medicine_debt, dowol_debt, heavenly_archive_contact]
  possible_destinations: [raid_aftermath_shelter, cheongryu_outer_courtyard]
  possible_log_tone:
    - 사람을 먼저 구한 선택이 route pressure를 없애지 않고 늦춘다는 감각
    - fallback도 보상과 대가가 있는 authored branch라는 감각
    - 이후 route opener가 direct branch와 deferred branch를 같은 route starter flag로 받을 수 있다는 감각
schema_boundary:
  allowed_existing_schema: [conditions.locations, required_flags, forbidden_flags, choices.cost, outcome.resources, outcome.danger, outcome.add_flags, outcome.add_clues, outcome.add_items, outcome.remove_items, outcome.destination_id, outcome.log, presentation]
  forbidden_new_schema: [RouteGraph, FactionStanding, BranchLock, TriageSystem, CompanionDeath, MassCombat, boss_combat_resolver, reward_schema, ability_schema, fragment_choice_reward, multi_ending_implementation]
main_spine_link: raid split fallback을 정파/사파/천기 route opener 전 공통 재합류 branch로 만든다. direct route choice와 deferred route choice가 같은 route starter flags를 남기게 해 future opener에서 새 any-of condition schema를 요구하지 않게 한다.
randomization_notes: route split fallback 직후 1회성 forced bridge로만 사용한다. 반복 피난처 deck이나 부상자 관리 시스템으로 확장하지 않는다.
promotion_notes: preview runtime으로 구현 완료. 첫 runtime은 `cheongryu_outer_courtyard`를 재사용했고, flags/clues/log/presentation만 남기며 route graph, faction reputation, triage system, companion death, boss combat, ending implementation은 별도 slice로 둔다. route opener docs-only handoff에서 다음 runtime 후보는 `wuxia_baekdo_medicine_debt`로 결정했다.
```

## 9. `wuxia_baekdo_medicine_debt`

```yaml
id: wuxia_baekdo_medicine_debt
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: candidate
runtime_preview_design_status: implemented
phase: [route_commitment]
priority_class: route_key
location_tags: [cheongryu_sect, faction_choice, righteous_route]
surface: [faction_negotiation, sect_courtyard]
anomaly_type: [faction_pressure, sect_debt]
pressure_type: [relation, danger, sanity]
npc_slots: [righteous_ally, early_rescuer, sect_master_guardian]
candidate_characters: [namgung_seoyun, seo_harin, cheongryu_sect_master]
summary: direct raid branch와 deferred wounded fallback branch가 남긴 정파 route starter를 받아, 백도맹의 약상자와 청류문 재건 지원이 사람을 살리면서 정치적 채무를 남기는 첫 정파 opener다.
purpose: 정파 루트를 “깨끗한 선”이 아니라 사람을 살리는 질서와 그 질서의 대가로 보여준다. 청류문 내부 악인 없이 외부 결핍/정파 정치/약재 부족을 route pressure로 사용한다.
setup_text: 백도맹 깃발 아래 약상자가 청류문 마당에 놓인다. 부상자들은 살아났지만, 약재 목록 옆에는 어느 문파의 이름으로 빚을 적을지 묻는 붓이 놓여 있다. 남궁서윤은 도와주겠다고 말하지만, 그 도움은 아무 기록도 남기지 않는 자비가 아니다.
runtime_preview_start_conditions:
  runtime_mode: storypack_preview
  prereq: raid split direct righteous branch or wounded fallback delayed righteous branch has landed
  location: cheongryu_outer_courtyard
  required_flags: [righteous_route_started, cheongryu_rebuild_thread]
  forbidden_flags: [baekdo_medicine_debt_resolved]
  flavor_flags_only: [baekdo_alliance_debt, baekdo_medicine_debt]
  note: direct/deferred branch 차이는 flavor hook으로만 읽는다. any-of condition schema를 열지 않는다.
choice_shapes:
  - id: accept_medicine_with_written_debt
    role: safe_acceptance
    fallback_choice: true
    label_direction: 채무 문서를 남기고 약상자와 호위를 받는다
    expected_costs: [political_debt, autonomy_risk]
    expected_gains: [medicine_support, cheongryu_rebuild_supplies]
    outcome_hook:
      add_flags: [baekdo_medicine_debt_resolved, righteous_route_opened, white_path_debt_recorded, cheongryu_rebuild_supplies_secured, namgung_seoyun_notice]
      add_clues: [medicine_has_banner, white_path_help_has_price, qingliu_survival_needs_outside_help]
      log_direction: 정파의 도움은 사람을 살리지만, 문서에 남은 이름은 이후 선택의 대가가 된다.
  - id: ask_terms_before_opening_gate
    role: negotiation_probe
    label_direction: 산문을 열기 전에 백도맹의 조건을 먼저 묻는다
    expected_costs: [sanity_small, suspicion_small]
    expected_gains: [terms_revealed, righteous_politics_clue]
    outcome_hook:
      add_flags: [baekdo_medicine_debt_resolved, righteous_route_opened, baekdo_terms_questioned, namgung_seoyun_notice]
      add_clues: [order_can_save_and_bind, white_path_help_has_price]
      log_direction: 조건을 묻는 순간, 도움과 종속의 경계가 얇아진다.
  - id: send_supplies_to_wounded_first
    role: homebase_alignment
    label_direction: 약과 식량을 장문 명부보다 부상자에게 먼저 돌린다
    expected_costs: [political_protocol_risk, fatigue_small]
    expected_gains: [trust_from_wounded, seo_harin_respect_thread]
    outcome_hook:
      add_flags: [baekdo_medicine_debt_resolved, righteous_route_opened, cheongryu_people_first, seo_harin_respect_thread, cheongryu_rebuild_supplies_secured]
      add_clues: [qingliu_survival_needs_outside_help, order_can_save_and_bind]
      log_direction: 청류문은 약하지만, 사람을 먼저 살리는 순서가 문파의 이름을 지킨다.
  - id: compare_banner_to_record_margin
    role: cheonggi_observation
    label_direction: 백도맹 깃발 문장과 천기록 여백의 채무 기록을 비교한다
    expected_costs: [sanity_small]
    expected_gains: [recorded_debt_clue, cheonggi_observation]
    outcome_hook:
      add_flags: [baekdo_medicine_debt_resolved, righteous_route_opened, cheonggi_record_notes_baekdo_debt]
      add_clues: [record_counts_debt_not_justice, white_path_help_has_price]
      log_direction: 천기록은 정체를 말하지 않는다. 다만 정의로 적힌 문장 옆에도 채무의 획이 남는다는 것만 보인다.
outcome_hooks:
  possible_flags: [baekdo_medicine_debt_resolved, righteous_route_opened, white_path_debt_recorded, cheongryu_rebuild_supplies_secured, baekdo_terms_questioned, namgung_seoyun_notice, cheongryu_people_first, seo_harin_respect_thread, cheonggi_record_notes_baekdo_debt]
  possible_route_flags: [righteous_route_opened]
  possible_clues: [medicine_has_banner, white_path_help_has_price, order_can_save_and_bind, qingliu_survival_needs_outside_help, record_counts_debt_not_justice]
  possible_relations: [namgung_seoyun_notice, seo_harin_respect_thread, trust_from_wounded]
  possible_destinations: [cheongryu_outer_courtyard]
  possible_log_tone:
    - 정파의 도움은 실제로 사람을 살린다는 감각
    - 그 도움은 청류문의 결핍과 외부 정치 채무를 동시에 드러낸다는 감각
    - 청류문 내부 악인이 아니라 부족한 약재와 외부 질서가 갈등 원천이라는 감각
schema_boundary:
  allowed_existing_schema: [conditions.locations, required_flags, forbidden_flags, choices.cost, outcome.resources, outcome.danger, outcome.add_flags, outcome.add_clues, outcome.add_items, outcome.remove_items, outcome.destination_id, outcome.log, presentation]
  forbidden_new_schema: [RouteGraph, FactionStanding, DebtLedger, RelationScore, BranchLock, reward_schema, ability_schema, fragment_choice_reward, multi_ending_implementation]
main_spine_link: route commitment의 첫 정파 opener. direct raid branch와 deferred wounded branch를 같은 `righteous_route_started`/`cheongryu_rebuild_thread` 조건으로 받아 any-of schema 없이 정파 루트를 연다.
randomization_notes: 1회성 route opener. hub random deck으로 반복하지 않는다. `stabilize_wounded_until_dawn`처럼 정파 flag가 없는 deferred branch는 별도 deferred-offer card 전까지 이 opener로 자동 진입하지 않는다.
promotion_notes: preview runtime으로 구현 완료. 첫 정파 route opener는 `cheongryu_outer_courtyard`에서 `righteous_route_started` + `cheongryu_rebuild_thread`를 받아 열리며, 백도맹 약상자/청류문 재건 채무를 flags/clues/log/presentation으로만 남긴다. 기본 office bundle, legacy `escape-office` key, faction route graph/reputation, debt/relation schema는 열지 않았다.
```

## 10. `wuxia_black_heaven_escape_price`

```yaml
id: wuxia_black_heaven_escape_price
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: implemented_in_storypack_preview
runtime_preview_design_status: implemented
phase: [route_commitment]
priority_class: route_key
location_tags: [cheongryu_sect, faction_choice, sapa_route]
surface: [faction_negotiation, sect_courtyard, market_street]
anomaly_type: [faction_pressure, sect_debt]
pressure_type: [danger, sanity, relation]
npc_slots: [sapa_ally, early_rescuer]
candidate_characters: [dowol, seo_harin]
summary: direct raid branch와 deferred wounded fallback branch가 남긴 사파 route starter를 받아, 흑천련의 빠른 탈출로와 도월의 표식이 이름과 값을 남기는 첫 사파 opener다.
purpose: 사파 루트를 “악의 길”이 아니라 밑바닥의 생존 거래로 보여준다. 청류문 내부 갈등 없이 외부 압박, 시장 장부, 도월의 실리적 도움을 route pressure로 사용한다.
setup_text: 청류문 바깥 담장 너머 시장 골목에서 도월이 낡은 표식을 굴린다. “흑천련 길은 빠르다. 공짜가 아닐 뿐이지.” 누가 값을 받을지, 누구의 이름이 장부에 남을지는 아직 정해지지 않았다.
runtime_preview_start_conditions:
  runtime_mode: storypack_preview
  prereq: raid split direct sapa branch or wounded fallback delayed sapa branch has landed
  location: cheongryu_outer_courtyard
  required_flags: [sapa_route_started, dowol_debt]
  forbidden_flags: [black_heaven_escape_price_resolved]
  flavor_flags_only: [black_heaven_deal_marked, black_heaven_escape_marker]
  note: direct/deferred branch 차이는 flavor hook으로만 읽는다. any-of condition schema를 열지 않는다.
choice_shapes:
  - id: accept_dowol_marker_for_safehouse
    role: safe_acceptance
    fallback_choice: true
    label_direction: 도월의 표식을 받고 흑천련 임시 은신처와 탈출로를 얻는다
    expected_costs: [market_debt, reputation_risk]
    expected_gains: [safehouse_access, exit_route]
    outcome_hook:
      add_flags: [black_heaven_escape_price_resolved, sapa_route_opened, black_heaven_safehouse_marked, market_route_debt_recorded]
      add_clues: [black_heaven_help_marks_debt, survival_bargain_is_not_loyalty]
      log_direction: 흑천련의 길은 빠르지만, 표식 하나가 이후 받을 값의 증거가 된다.
  - id: ask_who_collects_the_price
    role: negotiation_probe
    label_direction: 누가, 언제, 어떤 방식으로 값을 받는지 먼저 묻는다
    expected_costs: [sanity_small, suspicion_small]
    expected_gains: [terms_revealed, sapa_politics_clue]
    outcome_hook:
      add_flags: [black_heaven_escape_price_resolved, sapa_route_opened, dowol_terms_questioned]
      add_clues: [black_heaven_bargain_has_teeth, survival_bargain_is_not_loyalty]
      log_direction: 조건을 묻자 도월은 웃는다. 사파의 자비는 계약서보다 먼저 칼집을 보여준다.
  - id: keep_cheongryu_names_off_ledger
    role: homebase_alignment
    label_direction: 청류문 사람들의 이름은 흑천련 장부에 남기지 않는다고 못박는다
    expected_costs: [danger_small, negotiation_cost]
    expected_gains: [cheongryu_names_protected, dowol_attention]
    outcome_hook:
      add_flags: [black_heaven_escape_price_resolved, sapa_route_opened, cheongryu_names_kept_off_ledger, market_route_debt_recorded]
      add_clues: [ledger_can_be_bent_not_broken, sapa_can_save_without_mercy]
      log_direction: 청류문의 이름을 지우는 대신, 당신의 이름이 더 굵은 획으로 남는다.
  - id: map_exit_before_following_dowol
    role: survival_observation
    label_direction: 따라가기 전에 탈출로와 추적선을 먼저 기록한다
    expected_costs: [time_pressure, fatigue_small]
    expected_gains: [exit_route_clue, pursuit_pattern_seen]
    outcome_hook:
      add_flags: [black_heaven_escape_price_resolved, sapa_route_opened, sapa_survival_principle_seen]
      add_clues: [sapa_can_save_without_mercy, black_heaven_bargain_has_teeth]
      log_direction: 흑천련의 길은 사람을 살릴 수 있다. 다만 살아남은 사람이 어디로 빚을 갚으러 갈지도 함께 보여준다.
outcome_hooks:
  possible_flags: [black_heaven_escape_price_resolved, sapa_route_opened, black_heaven_safehouse_marked, dowol_terms_questioned, cheongryu_names_kept_off_ledger, market_route_debt_recorded, sapa_survival_principle_seen]
  possible_route_flags: [sapa_route_opened]
  possible_clues: [black_heaven_help_marks_debt, black_heaven_bargain_has_teeth, survival_bargain_is_not_loyalty, sapa_can_save_without_mercy, ledger_can_be_bent_not_broken]
  possible_relations: [dowol_debt, dowol_attention, seo_harin_respect_thread]
  possible_destinations: [cheongryu_outer_courtyard]
  possible_log_tone:
    - 사파의 도움은 빠르고 실제적이라는 감각
    - 그 도움은 이름, 표식, 장부를 통해 대가를 남긴다는 감각
    - 청류문 내부 악인이 아니라 외부 생존 거래가 갈등 원천이라는 감각
schema_boundary:
  allowed_existing_schema: [conditions.locations, required_flags, forbidden_flags, choices.cost, outcome.resources, outcome.danger, outcome.add_flags, outcome.add_clues, outcome.add_items, outcome.remove_items, outcome.destination_id, outcome.log, presentation]
  forbidden_new_schema: [RouteGraph, FactionStanding, DebtLedger, RelationScore, BranchLock, reward_schema, ability_schema, fragment_choice_reward, epilogue_schema, multi_ending_implementation]
main_spine_link: route commitment의 첫 사파 opener. direct raid branch와 deferred wounded branch를 같은 `sapa_route_started`/`dowol_debt` 조건으로 받아 any-of schema 없이 사파 루트를 연다.
randomization_notes: 1회성 route opener. hub random deck으로 반복하지 않는다. `black_heaven_deal_marked`와 `black_heaven_escape_marker`는 direct/deferred flavor만 바꾸고 eligibility를 가르지 않는다.
promotion_notes: preview runtime으로 구현 완료. `cheongryu_outer_courtyard`에서 `sapa_route_started` + `dowol_debt`를 받아 열리며, 흑천련 탈출로/도월 표식/시장 장부의 값을 flags/clues/log/presentation으로만 남긴다. 기본 office bundle, legacy `escape-office` key, faction route graph/reputation, debt/relation schema는 열지 않았다. 다음 handoff는 천기·귀환 opener `wuxia_heavenly_archive_previous_outsiders`로 결정됐다.
```

## 11. `wuxia_heavenly_archive_previous_outsiders`

```yaml
id: wuxia_heavenly_archive_previous_outsiders
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: implemented_in_storypack_preview
runtime_preview_design_status: implemented
phase: [route_commitment, cheonggi_return]
priority_class: route_key
location_tags: [cheongryu_sect, faction_choice, cheonggi_route]
surface: [cheonggi_record, faction_negotiation, sect_courtyard]
anomaly_type: [notebook_oracle, worldline_branch]
pressure_type: [sanity, relation, danger]
npc_slots: [cheonggi_record_keeper, archive_keeper]
candidate_characters: [yeon_soha, old_archive_keeper]
summary: direct raid branch와 deferred wounded fallback branch가 남긴 천기·귀환 route starter를 받아, 천기각 서고의 이전 이방인 기록과 세계 균열 흔적을 첫 천기 opener로 고정한다.
purpose: 천기·귀환 루트를 정답 찾기가 아니라 기록, 여백, 균열 감각으로 시작한다. 귀환법을 주지 않고, 이전에도 본인 몸 그대로 흘러든 사람이 있었을 가능성과 기록자의 시선만 남긴다.
setup_text: 청류문 마당의 먼지가 가라앉자 연소하가 낡은 서책 한 권을 펼친다. 표지에는 천기각의 인장이 있고, 여백에는 낯선 옷차림과 끊긴 길을 묘사한 문장이 남아 있다. 답은 없지만, 당신만 처음 온 사람은 아니라는 사실은 분명하다.
runtime_preview_start_conditions:
  runtime_mode: storypack_preview
  prereq: raid split direct heavenly archive branch or wounded fallback delayed heavenly archive branch has landed
  location: cheongryu_outer_courtyard
  required_flags: [cheonggi_return_route_started, cheonggi_record_targeted]
  forbidden_flags: [heavenly_archive_previous_outsiders_resolved]
  flavor_flags_only: [heavenly_archive_contact, heavenly_archive_triage_map_seen]
  note: direct/deferred branch 차이는 flavor hook으로만 읽는다. any-of condition schema를 열지 않는다.
choice_shapes:
  - id: read_previous_outsider_margins
    role: safe_reading
    fallback_choice: true
    label_direction: 이전 이방인의 여백 기록을 조용히 읽는다
    expected_costs: [sanity_small, time_pressure]
    expected_gains: [previous_outsider_clue, archive_access]
    outcome_hook:
      add_flags: [heavenly_archive_previous_outsiders_resolved, cheonggi_return_route_opened, previous_outsiders_record_seen]
      add_clues: [archive_has_other_outsiders, return_clue_is_not_return_method]
      log_direction: 서고의 여백에는 돌아간 사람보다 사라진 길을 붙잡은 사람의 흔적이 더 많다.
  - id: ask_yeon_soha_what_not_to_read
    role: boundary_probe
    label_direction: 연소하에게 무엇을 읽으면 안 되는지 먼저 묻는다
    expected_costs: [relation_risk, suspicion_small]
    expected_gains: [yeon_soha_boundary_clue, archive_warning]
    outcome_hook:
      add_flags: [heavenly_archive_previous_outsiders_resolved, cheonggi_return_route_opened, yeon_soha_warning_heard]
      add_clues: [cheonggi_record_refuses_identity_answer, record_gaze_without_name]
      log_direction: 연소하는 답을 숨기지 않는다. 다만 답으로 변하는 질문을 먼저 막는다.
  - id: mark_current_worldline_without_answer
    role: no_answer_acceptance
    label_direction: 정답을 요구하지 않고 현재 세계선의 흔적만 표시한다
    expected_costs: [sanity_small]
    expected_gains: [worldline_anchor, record_respect]
    outcome_hook:
      add_flags: [heavenly_archive_previous_outsiders_resolved, cheonggi_return_route_opened, worldline_margin_marked]
      add_clues: [worldline_gaps_have_patterns, cheonggi_record_refuses_identity_answer]
      log_direction: 천기록은 세계의 이름을 쓰지 않는다. 대신 이 세계선의 접힌 자국 하나를 남긴다.
  - id: compare_rift_terms_to_commute_memory
    role: return_clue_comparison
    label_direction: 서고의 균열 용어를 출근길 기억과 비교한다
    expected_costs: [sanity_small, fatigue_small]
    expected_gains: [commute_rift_pattern, return_thread]
    outcome_hook:
      add_flags: [heavenly_archive_previous_outsiders_resolved, cheonggi_return_route_opened, commute_rift_terms_compared]
      add_clues: [worldline_gaps_have_patterns, return_clue_is_not_return_method, record_gaze_without_name]
      log_direction: 출근길의 문틈과 서고의 균열이라는 말이 같은 모양으로 접힌다. 하지만 접힌 모양은 문이 아니다.
outcome_hooks:
  possible_flags: [heavenly_archive_previous_outsiders_resolved, cheonggi_return_route_opened, previous_outsiders_record_seen, yeon_soha_warning_heard, worldline_margin_marked, commute_rift_terms_compared]
  possible_route_flags: [cheonggi_return_route_opened]
  possible_clues: [archive_has_other_outsiders, cheonggi_record_refuses_identity_answer, return_clue_is_not_return_method, worldline_gaps_have_patterns, record_gaze_without_name]
  possible_relations: [yeon_soha_attention, archive_keeper_notice]
  possible_destinations: [cheongryu_outer_courtyard]
  possible_log_tone:
    - 이전 이방인이 있었다는 감각
    - 귀환 단서와 귀환 방법은 다르다는 감각
    - 천기록은 정체를 말하지 않고 여백과 시선만 남긴다는 감각
schema_boundary:
  allowed_existing_schema: [conditions.locations, required_flags, forbidden_flags, choices.cost, outcome.resources, outcome.danger, outcome.add_flags, outcome.add_clues, outcome.add_items, outcome.remove_items, outcome.destination_id, outcome.log, presentation]
  forbidden_new_schema: [RouteGraph, FactionStanding, DebtLedger, RelationScore, BranchLock, return_system, reward_schema, ability_schema, fragment_choice_reward, epilogue_schema, multi_ending_implementation]
main_spine_link: route commitment의 첫 천기·귀환 opener. direct raid branch와 deferred wounded branch를 같은 `cheonggi_return_route_started`/`cheonggi_record_targeted` 조건으로 받아 any-of schema 없이 천기·귀환 루트를 연다.
randomization_notes: 1회성 route opener. hub random deck으로 반복하지 않는다. `heavenly_archive_contact`와 `heavenly_archive_triage_map_seen`는 direct/deferred flavor만 바꾸고 eligibility를 가르지 않는다.
promotion_notes: preview runtime으로 구현 완료. `cheongryu_outer_courtyard`에서 `cheonggi_return_route_started` + `cheonggi_record_targeted`를 받아 열리며, 천기각 이전 이방인 기록/균열 단서를 flags/clues/log/presentation으로만 남긴다. 기본 office bundle, legacy `escape-office` key, 천기록 정체 reveal, return system, reward/ability schema는 열지 않았다. 다음 handoff에서 deferred-offer card `wuxia_wounded_shelter_dawn_offers`가 선택됐다.
```

## 12. `wuxia_wounded_shelter_dawn_offers`

```yaml
id: wuxia_wounded_shelter_dawn_offers
storypack_id: wuxia_jianghu_pack
world_id: wuxia_jianghu
status: candidate
runtime_preview_design_status: implemented
phase: [route_commitment]
priority_class: route_key
location_tags: [cheongryu_sect, wounded_shelter, deferred_route]
surface: [sect_courtyard, faction_negotiation, cheonggi_record]
anomaly_type: [sect_debt, faction_pressure, worldline_branch]
pressure_type: [health, danger, relation]
npc_slots: [early_rescuer, righteous_ally, sapa_ally, cheonggi_record_keeper]
candidate_characters: [seo_harin, namgung_seoyun, dowol, yeon_soha]
summary: `stabilize_wounded_until_dawn` branch가 남긴 deferred route state를 받아, 부상자 피난처의 새벽 제안으로 route pressure를 다시 연다.
purpose: 사람을 살리느라 루트 결정을 미룬 선택을 실패나 막다른 길로 처리하지 않는다. 살아남은 사람들의 명단, 약상자, 탈출로, 천기각 지도 조각이 같은 마당에 모이며, 플레이어는 다시 정파/사파/천기 또는 한 번 더 돌봄을 고른다.
setup_text: 새벽이 오자 임시 피난처의 숨소리가 하나씩 안정된다. 서하린은 젖은 천을 갈아 묶으며 부상자 명단을 접는다. 문밖에는 남궁서윤의 약상자, 도월이 남긴 짧은 전서, 연소하의 접힌 지도 조각이 서로 다른 그림자처럼 도착해 있다.
runtime_preview_start_conditions:
  runtime_mode: storypack_preview
  prereq: `wuxia_cheongryu_raid_wounded_fallback`에서 `stabilize_wounded_until_dawn` branch가 landed
  location: cheongryu_outer_courtyard
  required_flags: [cheongryu_raid_wounded_fallback_resolved, route_commitment_deferred, deferred_route_reopened, wounded_shelter_stabilized]
  forbidden_flags: [wounded_shelter_dawn_offers_resolved]
  flavor_flags_only: [survivor_roll_call_complete, route_delay_cost_recorded]
  note: direct route opener의 opened flags를 any-of로 묶지 않는다. deferred branch가 이미 가진 flags만 eligibility로 쓴다.
presentation:
  visual_id: wuxia_wounded_shelter_dawn_offers
  speaker: 서하린
  layout: deferred_route_offer
  effect_cues:
    - stable_terms: [새벽, 부상자, 제안]
choice_shapes:
  - id: keep_wounded_shelter_until_noon
    role: safe_care
    fallback_choice: true
    label_direction: 정오까지 피난처를 더 지킨다
    expected_costs: [time_pressure, danger_small]
    expected_gains: [wounded_shelter_stability, survivor_names]
    outcome_hook:
      add_flags: [wounded_shelter_dawn_offers_resolved, route_commitment_reopened, wounded_shelter_until_noon, deferred_offer_debt_recorded]
      add_clues: [saving_people_changed_witnesses, care_is_not_route_escape, dawn_shelter_keeps_names]
      log_direction: route를 고르지 않는 대신 살아남은 이름들이 더 또렷해진다.
  - id: accept_baekdo_medicine_after_roll_call
    role: delayed_righteous_offer
    label_direction: 생존자 점호 뒤 백도맹 약상자를 받는다
    expected_costs: [debt_mark, reputation_risk]
    expected_gains: [medicine_supply, righteous_route_reentry]
    outcome_hook:
      add_flags: [wounded_shelter_dawn_offers_resolved, route_commitment_reopened, righteous_route_started, cheongryu_rebuild_thread, baekdo_medicine_debt]
      add_clues: [offers_arrive_because_people_lived, delayed_choice_has_callers]
      log_direction: 약상자는 명령서가 아니라 명단 위에 놓인다. 빚은 생겼지만 문은 다시 열린다.
  - id: send_word_to_dowol_for_quiet_exit
    role: delayed_sapa_offer
    label_direction: 도월에게 조용한 퇴로를 부탁한다
    expected_costs: [debt_mark, suspicion_small]
    expected_gains: [safehouse_thread, exit_route]
    outcome_hook:
      add_flags: [wounded_shelter_dawn_offers_resolved, route_commitment_reopened, sapa_route_started, dowol_debt, black_heaven_escape_marker]
      add_clues: [delayed_choice_has_callers, care_is_not_route_escape]
      log_direction: 도월의 답장은 짧다. 사람을 살린 일에도 값은 붙지만, 값이 붙는다고 모두 거래가 되는 것은 아니다.
  - id: show_archive_map_to_yeon_soha
    role: delayed_cheonggi_offer
    label_direction: 연소하에게 피난처 지도의 접힌 부분을 보인다
    expected_costs: [sanity_small, time_pressure]
    expected_gains: [archive_route_reentry, worldline_clue]
    outcome_hook:
      add_flags: [wounded_shelter_dawn_offers_resolved, route_commitment_reopened, cheonggi_return_route_started, cheonggi_record_targeted, heavenly_archive_triage_map_seen]
      add_clues: [dawn_shelter_keeps_names, delayed_choice_has_callers, offers_arrive_because_people_lived]
      log_direction: 지도는 길보다 먼저 이름을 기억한다. 연소하는 그 순서가 중요하다고 말한다.
outcome_hooks:
  possible_flags: [wounded_shelter_dawn_offers_resolved, route_commitment_reopened, wounded_shelter_until_noon, deferred_offer_debt_recorded, righteous_route_started, cheongryu_rebuild_thread, baekdo_medicine_debt, sapa_route_started, dowol_debt, black_heaven_escape_marker, cheonggi_return_route_started, cheonggi_record_targeted, heavenly_archive_triage_map_seen]
  possible_route_flags: [route_commitment_reopened, righteous_route_started, sapa_route_started, cheonggi_return_route_started]
  possible_clues: [saving_people_changed_witnesses, delayed_choice_has_callers, care_is_not_route_escape, offers_arrive_because_people_lived, dawn_shelter_keeps_names]
  possible_relations: [seo_harin_trust, namgung_seoyun_attention, dowol_interest, yeon_soha_attention]
  possible_destinations: [cheongryu_outer_courtyard]
  possible_log_tone:
    - 사람을 살린 결과로 제안이 도착하는 감각
    - route 선택을 미룬 대가가 있지만 메인은 막히지 않는 감각
    - 정파/사파/천기 제안이 같은 마당에 놓인 긴장감
schema_boundary:
  allowed_existing_schema: [conditions.locations, required_flags, forbidden_flags, choices.cost, outcome.resources, outcome.danger, outcome.add_flags, outcome.add_clues, outcome.add_items, outcome.remove_items, outcome.destination_id, outcome.log, presentation]
  forbidden_new_schema: [TriageSystem, CompanionDeath, MassCombat, RouteGraph, FactionStanding, DebtLedger, RelationScore, BranchLock, return_system, reward_schema, ability_schema, fragment_choice_reward, epilogue_schema, multi_ending_implementation]
main_spine_link: route commitment을 미룬 wounded fallback branch를 다시 메인 route pressure에 붙인다. post-opener any-of condition이나 route graph 없이 기존 deferred flags만 사용한다.
randomization_notes: 1회성 deferred-offer card. hub random deck으로 반복하지 않는다. `survivor_roll_call_complete`와 `route_delay_cost_recorded`는 flavor만 바꾸고 eligibility를 가르지 않는다.
promotion_notes: preview runtime으로 구현 완료. `cheongryu_outer_courtyard`에서 `cheongryu_raid_wounded_fallback_resolved` + `route_commitment_deferred` + `deferred_route_reopened` + `wounded_shelter_stabilized`를 받아 열리며, 부상자 피난처 새벽 제안을 flags/clues/log/presentation으로만 남긴다. 기본 office bundle, legacy `escape-office` key, triage/companion death/mass combat, route graph/faction reputation/debt/relation schema, reward/ability/epilogue schema, return system, 천기록 정체 reveal은 열지 않았다. 다음 bridge `wuxia_mumyeong_first_sighting`도 preview runtime으로 구현됐다.
runtime_preview_implementation_notes:
  implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml
  generated_artifacts:
    - crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json
    - web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json
  default_bundle_changed: false
  new_schema_opened: false
  next_handoff: wuxia_mumyeong_first_sighting
```

## 13. `wuxia_mumyeong_first_sighting`

```yaml
id: wuxia_mumyeong_first_sighting
notion_event_mapping:
  notion_event_id: wuxia_mumyeong_first_sighting
  notion_event_name: 무명 첫 목격
  mapping_status: preview_runtime_implemented
storypack_id: wuxia_jianghu_pack
world_id: wuxia_jianghu
status: candidate
runtime_preview_design_status: implemented
phase: [midgame_rival]
priority_class: route_key
location_tags: [cheongryu_sect, black_serpent_pressure, rival_shadow]
surface: [sect_courtyard, market_street, training_chore]
anomaly_type: [faction_pressure, worldline_branch, sect_debt]
pressure_type: [sanity, danger, relation]
npc_slots: [early_rescuer]
candidate_characters: [seo_harin, mumyeong, black_serpent_runner]
summary: route opener 이후 흑사방 쪽에서 청류문식 흐름을 훔쳐 쓰는 그림자를 처음 목격하고, 무명/서하린/카피 무공 thread를 연다.
purpose: 첫 post-opener midgame continuity를 route graph 없이 연다. 무명은 아직 정식 대치하지 않고, 청류문식과 닮았지만 중심이 비어 있는 카피 무공, 서하린의 침묵, 흑사방의 외부 압박만 남긴다.
setup_text: 청류문 외원 담장 너머로 흑사방 심부름꾼 하나가 지나간다. 걸음은 거칠지만 팔꿈치가 꺾이는 순간, 장작 마당에서 배운 청류문식 흐름과 너무 닮은 선이 스친다. 서하린은 이름을 부르려다 멈추고, 그자는 뒤돌아보지 않은 채 어둠 속으로 사라진다.
runtime_preview_start_conditions:
  runtime_mode: storypack_preview
  location: cheongryu_outer_courtyard
  required_flags: [route_opener_resolved, cheongryu_raid_survived, cheongryu_trial_started, first_fragment_seen]
  forbidden_flags: [mumyeong_first_sighting_resolved]
  implementation_prerequisite: 세 route opener `wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`의 모든 choice outcome에 `route_opener_resolved`를 추가한다.
  flavor_flags_only: [righteous_route_opened, sapa_route_opened, cheonggi_return_route_opened, white_path_debt_recorded, market_route_debt_recorded, previous_outsiders_record_seen]
  note: route-specific opened flags는 eligibility가 아니라 branch flavor hook으로만 사용한다. 새 any-of condition schema를 열지 않는다.
presentation:
  visual_id: wuxia_mumyeong_first_sighting
  speaker: 서하린
  layout: midgame_rival_sighting
  effect_cues:
    - stable_terms: [무명, 청류문, 흑사방]
choice_shapes:
  - id: watch_the_stolen_qingliu_flow
    role: safe_observe
    fallback_choice: true
    label_direction: 훔쳐 쓴 청류문식 흐름을 끝까지 관찰한다
    expected_costs: [sanity_small]
    expected_gains: [copied_flow_clue, rival_shadow]
    outcome_hook:
      add_flags: [mumyeong_first_sighting_resolved, midgame_continuity_started, mumyeong_shadow_seen, copied_qingliu_flow_noted]
      add_clues: [mumyeong_exists, copied_flow_is_not_qingliu]
      destination_id: cheongryu_outer_courtyard
      log_direction: 닮았지만 중심이 다르다. 청류문식의 모양은 있으나 흐름을 이해한 흔적은 없다.
  - id: check_seo_harin_silence
    role: companion_observation
    label_direction: 서하린이 이름을 삼키는 순간을 본다
    expected_costs: [relation_risk, silence_weight]
    expected_gains: [seo_harin_wound_thread, rival_name_pressure]
    outcome_hook:
      add_flags: [mumyeong_first_sighting_resolved, midgame_continuity_started, seo_harin_recognized_mumyeong, mumyeong_wound_thread_opened]
      add_clues: [seo_harin_does_not_call_him_traitor, mumyeong_exists]
      destination_id: cheongryu_outer_courtyard
      log_direction: 서하린은 배신자라는 말을 쓰지 않는다. 그 침묵이 이름보다 먼저 상처를 드러낸다.
  - id: follow_black_serpent_runner
    role: risky_pursuit
    label_direction: 흑사방 심부름꾼의 뒤를 짧게 쫓는다
    expected_costs: [danger_small, health_risk]
    expected_gains: [black_serpent_trail, pressure_map]
    outcome_hook:
      danger: 1
      add_flags: [mumyeong_first_sighting_resolved, midgame_continuity_started, black_serpent_trail_marked, mumyeong_pursuit_risk]
      add_clues: [black_serpent_uses_borrowed_flows, copied_flow_is_not_qingliu]
      destination_id: cheongryu_outer_courtyard
      log_direction: 뒤를 밟는 데는 성공하지만, 흑사방은 일부러 발자국을 남기는 법도 안다.
  - id: pretend_not_to_see_the_form
    role: escalation_delay
    label_direction: 못 본 척하고 외원 순찰을 계속한다
    expected_costs: [unresolved_debt, sanity_small]
    expected_gains: [delay_pressure, survivor_focus]
    outcome_hook:
      add_flags: [mumyeong_first_sighting_resolved, midgame_continuity_started, mumyeong_clue_deferred, unresolved_rival_debt]
      add_clues: [not_seeing_is_also_a_choice, black_serpent_uses_borrowed_flows]
      destination_id: cheongryu_outer_courtyard
      log_direction: 모른 척한 일도 사라지지 않는다. 다음에 같은 흐름을 보면 더 늦게 알아볼 뿐이다.
outcome_hooks:
  possible_flags: [mumyeong_first_sighting_resolved, midgame_continuity_started, mumyeong_shadow_seen, copied_qingliu_flow_noted, seo_harin_recognized_mumyeong, mumyeong_wound_thread_opened, black_serpent_trail_marked, mumyeong_pursuit_risk, mumyeong_clue_deferred, unresolved_rival_debt]
  possible_clues: [mumyeong_exists, copied_flow_is_not_qingliu, seo_harin_does_not_call_him_traitor, black_serpent_uses_borrowed_flows, not_seeing_is_also_a_choice]
  possible_relations: [seo_harin_trust_risk, black_serpent_attention]
  possible_destinations: [cheongryu_outer_courtyard]
  possible_log_tone:
    - 무명 존재를 확정 대치 없이 감지하는 감각
    - 청류문식과 카피 무공의 차이가 보이는 감각
    - 서하린의 침묵이 관계 thread로 남는 감각
schema_boundary:
  allowed_existing_schema: [conditions.locations, required_flags, forbidden_flags, choices.cost, outcome.resources, outcome.danger, outcome.add_flags, outcome.add_clues, outcome.add_items, outcome.remove_items, outcome.destination_id, outcome.log, presentation]
  forbidden_new_schema: [AnyOfCondition, RouteGraph, FactionStanding, DebtLedger, RelationScore, CompanionSystem, CombatState, boss_combat_resolver, reward_schema, ability_schema, fragment_choice_reward, epilogue_schema, return_system, multi_ending_implementation]
main_spine_link: 세 route opener가 모두 지나간 뒤 route fan-out을 더 벌리지 않고, 무명/흑사방/서하린 상처를 첫 midgame continuity로 묶는다.
randomization_notes: 1회성 midgame bridge. `route_opener_resolved`를 통해 세 direct opener를 fan-in하고, deferred wounded shelter만 탄 branch는 route opener 전까지 바로 eligible하지 않는다.
promotion_notes: preview runtime으로 구현 완료. `route_midgame_continuity_after_wounded_shelter` docs-only 선택에서 route별 3개 card, deferred-offer 후속 bridge, 첫 대치, boss first appearance보다 작고 안전한 common midgame bridge로 선택했고, 세 route opener outcome에 `route_opener_resolved`를 추가해 fan-in했다. 기본 office bundle, legacy `escape-office` key, any-of condition schema, route graph/faction reputation/debt/relation schema, combat schema, reward/ability/epilogue/return system, 천기록 정체 reveal은 열지 않았다. 다음 handoff는 `wuxia_mumyeong_first_confrontation_after_sighting`다.
runtime_preview_implementation_notes:
  implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml
  generated_artifacts:
    - crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json
    - web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json
  common_flag_to_add: route_opener_resolved
  route_openers_to_patch: [wuxia_baekdo_medicine_debt, wuxia_black_heaven_escape_price, wuxia_heavenly_archive_previous_outsiders]
  selected_over: [route_specific_midgame_fanout, deferred_offer_only_bridge, wuxia_mumyeong_first_confrontation, boss_first_appearance]
  default_bundle_changed: false
  new_schema_opened: false
  next_handoff: wuxia_mumyeong_first_confrontation_after_sighting
```

## 14. `wuxia_mumyeong_first_confrontation`

```yaml
id: wuxia_mumyeong_first_confrontation
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
source_refs:
  notion_event_id: wuxia_mumyeong_first_confrontation
  notion_name: 무명 첫 대치
  source_page: 09. 이구학지 사건 카드 DB
  checked_against:
    - 04. 메인 루트 구조
    - 05. 사건 카드 운영 규칙
    - 06. 사이드 퀘스트와 미해결 부채
    - 07. 천기록 / 천외편린 보상
    - 99. 통합 체크포인트
mapping_status: preview_runtime_implemented
status: candidate
phase: [midgame_rival, rival_confrontation]
priority_class: route_key
location_tags: [cheongryu_outer_courtyard, training_yard, black_serpent_pressure]
surface: [sect_courtyard, training_chore, faction_negotiation]
anomaly_type: [faction_pressure, sect_debt, qi_deviation]
pressure_type: [health, sanity, danger, relation]
npc_slots: [early_rescuer]
candidate_characters: [seo_harin, mumyeong]
summary: 첫 목격 이후 무명을 라이벌로 확정하는 첫 대치. 이기는 전투가 아니라 버티기, 카피 무공 관찰, 서하린과 무명 사이의 침묵 확인이 핵심이다.
setup_text: 외원 담장 밑에서 다시 그 흐름이 나타난다. 무명은 청류문이 또 외부자를 주워 왔느냐고 묻지만, 말끝에는 조롱보다 피로가 먼저 묻어난다. 서하린은 칼집에 손을 얹고도 이름을 부르지 않는다.
runtime_preview_design_status: implemented
runtime_preview_start_conditions:
  runtime_mode: storypack_preview
  location: cheongryu_outer_courtyard
  required_flags: [mumyeong_first_sighting_resolved, midgame_continuity_started, cheongryu_raid_survived, first_fragment_seen]
  forbidden_flags: [mumyeong_first_confrontation_resolved]
  flavor_flags_only: [mumyeong_shadow_seen, copied_qingliu_flow_noted, seo_harin_recognized_mumyeong, mumyeong_wound_thread_opened, black_serpent_trail_marked, mumyeong_clue_deferred, righteous_route_opened, sapa_route_opened, cheonggi_return_route_opened]
presentation:
  visual_id: wuxia_mumyeong_first_confrontation
  speaker: 무명
  layout: rival_first_confrontation
  effect_cues:
    - stable_terms: [무명, 서하린, 청류문]
choice_shapes:
  - id: meet_mumyeong_head_on
    role: high_risk_confront
    label_direction: 무명과 정면으로 맞선다
    expected_costs: [danger_medium, health_risk]
    expected_gains: [rival_pressure, courage_thread]
    outcome_hook:
      add_flags: [mumyeong_first_confrontation_resolved, mumyeong_rival_thread_opened, rival_endured_not_defeated]
      add_clues: [mumyeong_is_not_boss_wall, winning_is_not_required]
      destination_id: cheongryu_outer_courtyard
      log_direction: 정면으로 부딪혔지만 이긴 것은 아니다. 다만 물러서지 않았다는 사실만은 남는다.
  - id: endure_until_copy_flow_breaks
    role: safe_endure
    fallback_choice: true
    label_direction: 버티며 카피한 흐름이 끊기는 순간을 기다린다
    expected_costs: [sanity_small, health_risk]
    expected_gains: [copied_flow_weakness, rival_pattern]
    outcome_hook:
      add_flags: [mumyeong_first_confrontation_resolved, mumyeong_rival_thread_opened, copied_flow_weakness_noted]
      add_clues: [copy_style_has_gap, copied_flow_is_not_qingliu]
      destination_id: cheongryu_outer_courtyard
      log_direction: 훔친 흐름은 빠르지만 오래 가지 않는다. 중심이 돌아오는 순간, 빈틈이 생긴다.
  - id: watch_seo_harin_hold_back
    role: companion_observation
    label_direction: 서하린이 왜 끼어들지 않는지 살핀다
    expected_costs: [relation_risk, silence_weight]
    expected_gains: [seo_harin_wound_thread, mumyeong_relation_clue]
    outcome_hook:
      add_flags: [mumyeong_first_confrontation_resolved, mumyeong_rival_thread_opened, seo_harin_mumyeong_silence_confirmed]
      add_clues: [seo_harin_does_not_call_him_traitor, mumyeong_was_not_only_enemy]
      destination_id: cheongryu_outer_courtyard
      log_direction: 서하린은 끼어들 수 있는데도 한 박자 늦춘다. 그 침묵은 망설임이 아니라 기억에 가깝다.
  - id: read_mumyeongs_copied_form
    role: information_probe
    label_direction: 무명의 초식이 어디서 어긋나는지 읽는다
    expected_costs: [sanity_medium, danger_small]
    expected_gains: [copy_defect_clue, cheonggi_contrast]
    outcome_hook:
      add_flags: [mumyeong_first_confrontation_resolved, mumyeong_rival_thread_opened, copied_flow_weakness_noted, cheonggi_copy_contrast_noted]
      add_clues: [copy_style_has_gap, understanding_is_not_copying]
      destination_id: cheongryu_outer_courtyard
      log_direction: 같은 선을 그어도 같은 뜻이 되지는 않는다. 훔친 초식과 이해한 흐름은 손끝에서 갈라진다.
  - id: do_not_provoke_mumyeong
    role: safe_deescalate
    label_direction: 도발하지 않고 물러설 거리를 만든다
    expected_costs: [unresolved_debt, sanity_small]
    expected_gains: [survival_space, future_confrontation_thread]
    outcome_hook:
      add_flags: [mumyeong_first_confrontation_resolved, mumyeong_rival_thread_opened, rivalry_deferred_not_avoided]
      add_clues: [winning_is_not_required, not_provoking_still_leaves_debt]
      destination_id: cheongryu_outer_courtyard
      log_direction: 싸움을 키우지 않는다고 대치가 사라지지는 않는다. 오늘 피한 말은 다음에 더 날카롭게 돌아온다.
outcome_hooks:
  possible_flags: [mumyeong_first_confrontation_resolved, mumyeong_rival_thread_opened, rival_endured_not_defeated, copied_flow_weakness_noted, seo_harin_mumyeong_silence_confirmed, cheonggi_copy_contrast_noted, rivalry_deferred_not_avoided]
  possible_clues: [mumyeong_is_not_boss_wall, winning_is_not_required, copy_style_has_gap, copied_flow_is_not_qingliu, seo_harin_does_not_call_him_traitor, mumyeong_was_not_only_enemy, understanding_is_not_copying, not_provoking_still_leaves_debt]
  possible_destinations: [cheongryu_outer_courtyard]
main_spine_link: 무명을 단순 적이 아니라 주인공보다 살짝 앞선 라이벌로 확정하고, 보스가 담당할 최종 벽과 무명의 라이벌 결산을 분리한다.
randomization_notes: 1회성 midgame confrontation. 첫 목격이 남긴 common hook으로만 열린다. 첫 목격 branch flags와 route opener flags는 문장 flavor에만 사용한다.
promotion_notes: preview runtime으로 구현 완료. `wuxia_mumyeong_first_confrontation_after_sighting`에서 첫 대치, 중반 재회, boss first appearance를 비교했고, 첫 대치를 다음 runtime 후보로 골랐다. 전투 사건이지만 승리/패배 resolver가 아니라 버티기/분석 encounter로 구현했다. 기본 office bundle, legacy `escape-office` key, combat resolver/schema, HP 숫자전, route graph/faction reputation/debt/relation schema, reward/ability/epilogue/return system, boss first appearance, 천기록 정체 reveal은 열지 않았다. 다음 handoff는 `wuxia_mumyeong_followup_after_first_confrontation`다.
runtime_preview_implementation_notes:
  implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml
  insert_after: wuxia_mumyeong_first_sighting
  generated_artifacts:
    - crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json
    - web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json
  selected_over: [wuxia_mumyeong_midgame_reunion, wuxia_boss_first_appearance, route_specific_clue_bridge]
  default_bundle_changed: false
  new_schema_opened: false
  next_handoff: wuxia_mumyeong_followup_after_first_confrontation
```

## 15. `wuxia_mumyeong_copy_style_reveal`

```yaml
id: wuxia_mumyeong_copy_style_reveal
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
source_refs:
  notion_event_id: wuxia_mumyeong_copy_style_reveal
  notion_name: 무명의 카피 무공 공개
  source_page: 09. 이구학지 사건 카드 DB
  checked_against:
    - 04. 메인 루트 구조
    - 05. 사건 카드 운영 규칙
    - 06. 사이드 퀘스트와 미해결 부채
    - 07. 천기록 / 천외편린 보상
    - 99. 통합 체크포인트
mapping_status: preview_runtime_implemented
status: candidate
phase: [midgame_rival, copy_style_analysis]
priority_class: route_key
location_tags: [cheongryu_outer_courtyard, training_yard, rival_shadow]
surface: [sect_courtyard, cheonggi_record, training_chore]
anomaly_type: [faction_pressure, qi_deviation, notebook_oracle]
pressure_type: [sanity, danger, relation]
npc_slots: [early_rescuer]
candidate_characters: [seo_harin, mumyeong]
summary: 첫 대치 이후 무명이 이번 회차에 덧씌운 카피 무공 계열의 윤곽과 결함을 드러낸다.
purpose: 무명의 카피가 완전한 복사가 아니라 겉흐름만 빠르게 훔치는 방식임을 보여주고, 주인공의 청류안/천기록식 이해와 대비한다. 다음 구현은 seed 기반 random table 없이 기존 flags/clues/log/presentation만으로 copy-style hint를 남긴다.
setup_text: 첫 대치가 끝난 뒤에도 무명의 움직임은 눈꺼풀 안쪽에 남아 있다. 같은 청류문식처럼 보였던 선은 어느 순간 검로처럼 꺾이고, 어느 순간 보법처럼 미끄러지며, 호흡이 어긋나는 짧은 틈마다 몸에 맞지 않는 초식의 반동을 드러낸다.
runtime_preview_design_status: implemented
runtime_preview_start_conditions:
  runtime_mode: storypack_preview
  location: cheongryu_outer_courtyard
  required_flags: [mumyeong_first_confrontation_resolved, mumyeong_rival_thread_opened, midgame_continuity_started]
  forbidden_flags: [mumyeong_copy_style_reveal_resolved]
  flavor_flags_only: [copied_flow_weakness_noted, cheonggi_copy_contrast_noted, seo_harin_mumyeong_silence_confirmed, rival_endured_not_defeated, rivalry_deferred_not_avoided, righteous_route_opened, sapa_route_opened, cheonggi_return_route_opened]
presentation:
  visual_id: wuxia_mumyeong_copy_style_reveal
  speaker: 서하린
  layout: copy_style_analysis
  effect_cues:
    - stable_terms: [무명, 청류안, 천기록]
choice_shapes:
  - id: read_the_stolen_blade_path
    role: information_probe
    label_direction: 훔쳐 쓴 검로가 어디서 꺾이는지 읽는다
    expected_costs: [sanity_small, danger_small]
    expected_gains: [copy_family_hint, blade_path_clue]
    outcome_hook:
      add_flags: [mumyeong_copy_style_reveal_resolved, copy_style_hint_recorded, copied_blade_path_noted]
      add_clues: [copied_form_family_seen, copy_is_surface_not_root]
      destination_id: cheongryu_outer_courtyard
      log_direction: 검로처럼 보이지만 칼끝의 뜻이 이어지지 않는다. 훔친 것은 선이지 중심이 아니다.
  - id: watch_mumyeongs_footwork
    role: movement_observation
    label_direction: 무명의 보법이 땅을 밀어내는 방식을 본다
    expected_costs: [sanity_small]
    expected_gains: [footwork_copy_clue, route_pressure_hint]
    outcome_hook:
      add_flags: [mumyeong_copy_style_reveal_resolved, copy_style_hint_recorded, copied_footwork_noted]
      add_clues: [copied_form_family_seen, footwork_without_root_wobbles]
      destination_id: cheongryu_outer_courtyard
      log_direction: 발은 빠르지만 땅을 믿지 않는다. 남의 보법을 얹은 몸은 한 박자 늦게 흔들린다.
  - id: listen_for_breath_mismatch
    role: safe_observe
    fallback_choice: true
    label_direction: 거리를 두고 호흡이 어긋나는 박자를 듣는다
    expected_costs: [sanity_small]
    expected_gains: [breath_mismatch_clue, safe_distance]
    outcome_hook:
      add_flags: [mumyeong_copy_style_reveal_resolved, copy_style_hint_recorded, copied_breath_mismatch_noted]
      add_clues: [breath_mismatch_marks_copy, understanding_is_not_copying]
      destination_id: cheongryu_outer_courtyard
      log_direction: 초식은 따라왔지만 숨은 따라오지 못한다. 그 박자가 청류안에 먼저 걸린다.
  - id: wait_for_body_to_shudder
    role: risk_timed_observation
    label_direction: 몸에 맞지 않는 초식이 반동을 내는 순간까지 기다린다
    expected_costs: [danger_small, health_risk]
    expected_gains: [copy_side_effect_clue, fragment_foreshadow]
    outcome_hook:
      add_flags: [mumyeong_copy_style_reveal_resolved, copy_style_hint_recorded, copy_side_effect_seen]
      add_clues: [copy_is_surface_not_root, fragment_candidate_variation_foreshadowed]
      destination_id: cheongryu_outer_courtyard
      log_direction: 무명의 어깨가 아주 짧게 떨린다. 천기록의 빈 줄은 아직 보상을 주지 않지만, 다음 편린의 모양을 기억한다.
outcome_hooks:
  possible_flags: [mumyeong_copy_style_reveal_resolved, copy_style_hint_recorded, copied_blade_path_noted, copied_footwork_noted, copied_breath_mismatch_noted, copy_side_effect_seen]
  possible_clues: [copied_form_family_seen, copy_is_surface_not_root, footwork_without_root_wobbles, breath_mismatch_marks_copy, understanding_is_not_copying, fragment_candidate_variation_foreshadowed]
  possible_destinations: [cheongryu_outer_courtyard]
main_spine_link: 무명의 카피 무공을 청류안/천기록의 이해와 대비시키고, 중반 재회와 무명 구원/비구원 변주로 가기 전 필요한 copy-style clue를 쌓는다.
randomization_notes: Notion상 매 회차 seed 기반 랜덤 카피 무공 후보를 암시하지만, 이 handoff에서는 random copy-style table을 열지 않는다. 첫 implementation은 copy-style family hint를 flags/clues로만 기록한다.
promotion_notes: preview runtime으로 구현 완료. `wuxia_mumyeong_followup_after_first_confrontation`에서 카피 무공 공개, 무명 중반 재회, boss first appearance를 비교했고, 카피 무공 공개를 runtime으로 landing했다. legacy office bundle, legacy `escape-office` key, seed 기반 random copy-style system/table, combat resolver/schema, HP 숫자전, route graph/faction reputation/debt/relation schema, reward/ability/epilogue/return system, boss first appearance, 무명 중반 재회, 천기록 정체 reveal은 열지 않는다.
runtime_preview_implementation_notes:
  implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml
  insert_after: wuxia_mumyeong_first_confrontation
  generated_artifacts:
    - crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json
    - web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json
  selected_over: [wuxia_mumyeong_midgame_reunion, wuxia_boss_first_appearance, route_specific_clue_bridge]
  default_bundle_changed: false
  new_schema_opened: false
  next_handoff: wuxia_mumyeong_followup_after_copy_style_reveal
```

## 16. `wuxia_mumyeong_reads_orthodox_style`

```yaml
id: wuxia_mumyeong_reads_orthodox_style
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
source_refs:
  - notion_event:wuxia_mumyeong_reads_orthodox_style
  - docs/dev/Notion_Design_Coverage.md
notion_event_mapping:
  notion_event_id: wuxia_mumyeong_reads_orthodox_style
  notion_event_name: 무명의 정파 무공 간파
  mapping_status: preview_runtime_implemented
notion_sources_checked:
  events:
    - wuxia_mumyeong_midgame_reunion
    - wuxia_boss_first_appearance
    - wuxia_mumyeong_departure_truth_summary
    - wuxia_mumyeong_reads_orthodox_style
  operating_docs:
    - 04. 메인 루트 구조
    - 05. 사건 카드 운영 규칙
    - 07. 천기록 / 천외편린 보상
    - 99. 통합 체크포인트
mapping_status: preview_runtime_implemented
status: candidate
phase: [midgame_rival, orthodox_style_trace]
priority_class: route_key
location_tags: [cheongryu_outer_courtyard, training_yard, old_wound_trace]
surface: [sect_courtyard, cheonggi_record, training_chore]
anomaly_type: [faction_pressure, qi_deviation, notebook_oracle]
pressure_type: [sanity, danger, relation]
npc_slots: [early_rescuer]
candidate_characters: [mumyeong, seo_harin]
summary: 카피 무공 공개 뒤 무명이 과거에 읽어낸 정파식 제압술 흔적을 현악문/복호금쇄수 단서로 연결한다.
purpose: 무명 중반 재회로 가기 전, 카피 결함 clue를 무명의 특별한 눈과 정파식 통제 무공의 흔적으로 바꿔 축적한다. 무명 이탈의 진실 전체와 보스 첫 등장은 아직 열지 않는다.
setup_text: 무명의 카피는 완전한 복사가 아니었다. 겉흐름 사이로 손목을 잠그는 각도, 기혈을 누르는 순서, 물러나는 보폭이 삐져나온다. 천기록의 빈 줄은 그 흔적을 현악문과 복호금쇄수라는 이름 옆에 잠시 멈춰 세운다.
runtime_preview_design_status: implemented
runtime_preview_start_conditions:
  runtime_mode: storypack_preview
  location: cheongryu_outer_courtyard
  required_flags: [mumyeong_copy_style_reveal_resolved, copy_style_hint_recorded, midgame_continuity_started, first_fragment_seen]
  forbidden_flags: [mumyeong_reads_orthodox_style_resolved]
  flavor_flags_only: [copied_form_family_seen, copy_is_surface_not_root, breath_mismatch_marks_copy, understanding_is_not_copying, fragment_candidate_variation_foreshadowed, righteous_route_opened, sapa_route_opened, cheonggi_return_route_opened]
presentation:
  visual_id: wuxia_mumyeong_reads_orthodox_style
  speaker: 천기록
  layout: orthodox_style_trace
  effect_cues:
    - stable_terms: [현악문, 복호금쇄수, 무명]
choice_shapes:
  - id: compare_copied_form_to_old_wound
    role: information_probe
    label_direction: 카피된 초식과 오래된 상처의 각도를 맞춰 본다
    expected_costs: [sanity_small]
    expected_gains: [orthodox_style_trace, past_mumyeong_clue]
    outcome_hook:
      add_flags: [mumyeong_reads_orthodox_style_resolved, orthodox_style_trace_recorded, old_wound_angle_compared]
      add_clues: [hyeonakmun_trace_suspected, orthodox_control_is_violence]
      destination_id: cheongryu_outer_courtyard
      log_direction: 손목을 잠그는 각도는 살수의 버릇이 아니라 정파식 통제의 흔적에 가깝다.
  - id: trace_qingliu_eye_variation
    role: perception_trace
    label_direction: 청류안 계열의 시선이 어디서 비틀렸는지 추적한다
    expected_costs: [sanity_small]
    expected_gains: [qingliu_eye_variation, copy_contrast_clue]
    outcome_hook:
      add_flags: [mumyeong_reads_orthodox_style_resolved, orthodox_style_trace_recorded, qingliu_eye_variation_traced]
      add_clues: [mumyeong_eye_variation_noted, copy_is_surface_not_root]
      destination_id: cheongryu_outer_courtyard
      log_direction: 무명은 흐름을 본 뒤 훔쳤고, 너는 흐름을 본 뒤 맞춰 보려 한다. 같은 눈에서 다른 결론이 갈라진다.
  - id: reconstruct_mumyeongs_sightline
    role: risky_reconstruction
    label_direction: 무명이 그날 보았을 시선을 따라 재구성한다
    expected_costs: [sanity_small, danger_small]
    expected_gains: [confirmed_name_hint, departure_truth_foreshadow]
    outcome_hook:
      add_flags: [mumyeong_reads_orthodox_style_resolved, orthodox_style_trace_recorded, mumyeong_sightline_reconstructed]
      add_clues: [bokho_geumsaesu_name_recorded, departure_truth_still_incomplete]
      destination_id: cheongryu_outer_courtyard
      log_direction: 복호금쇄수라는 이름은 떠오르지만, 그 이름이 왜 무명을 청류문 밖으로 밀어냈는지는 아직 한 줄이 비어 있다.
  - id: stop_before_truth_becomes_accusation
    role: safe_observe
    fallback_choice: true
    label_direction: 진실이 추궁이 되기 전에 기록을 덮는다
    expected_costs: [sanity_small]
    expected_gains: [safe_distance, incomplete_truth_marker]
    outcome_hook:
      add_flags: [mumyeong_reads_orthodox_style_resolved, orthodox_style_trace_recorded, truth_accusation_avoided]
      add_clues: [departure_truth_still_incomplete, understanding_is_not_copying]
      destination_id: cheongryu_outer_courtyard
      log_direction: 지금 필요한 것은 판결이 아니라 흔적이다. 천기록은 닫혔지만 이름 몇 개는 손끝에 남았다.
outcome_hooks:
  possible_flags: [mumyeong_reads_orthodox_style_resolved, orthodox_style_trace_recorded, old_wound_angle_compared, qingliu_eye_variation_traced, mumyeong_sightline_reconstructed, truth_accusation_avoided]
  possible_clues: [hyeonakmun_trace_suspected, bokho_geumsaesu_name_recorded, mumyeong_eye_variation_noted, orthodox_control_is_violence, departure_truth_still_incomplete, copy_is_surface_not_root, understanding_is_not_copying]
  possible_destinations: [cheongryu_outer_courtyard]
main_spine_link: 무명의 카피와 주인공의 이해를 대비한 뒤, 중반 재회와 무명 이탈 진실 정리로 가기 전 필요한 정파 무공 흔적을 쌓는다.
randomization_notes: 현악문/복호금쇄수는 최신 Notion 확정명으로 사용한다. 하지만 이 카드에서는 random copy-style table이나 full flashback을 열지 않고, 정파식 통제 무공 clue만 기록한다.
promotion_notes: preview runtime으로 구현 완료. `wuxia_mumyeong_followup_after_copy_style_reveal` docs-only handoff에서 다음 runtime 후보로 선택했고, 현악문/복호금쇄수 단서를 기존 flags/clues/log/presentation으로 landing했다. `wuxia_mumyeong_midgame_reunion`은 과거 단서가 더 필요해 보류했고, `wuxia_boss_first_appearance`는 boss-wall pressure 때문에 보류했으며, `wuxia_mumyeong_departure_truth_summary`는 후반 truth reveal이라 보류했다. legacy office bundle, legacy `escape-office` key, random copy-style system/table, combat resolver/schema, route graph/faction reputation/debt/relation schema, reward/ability/epilogue/return system, 천기록 정체 reveal은 열지 않는다.
runtime_preview_implementation_notes:
  implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml
  insert_after: wuxia_mumyeong_copy_style_reveal
  generated_artifacts:
    - crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json
    - web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json
  selected_over: [wuxia_mumyeong_midgame_reunion, wuxia_boss_first_appearance, wuxia_mumyeong_departure_truth_summary]
  default_bundle_changed: false
  new_schema_opened: false
  next_handoff: wuxia_mumyeong_followup_after_orthodox_style_trace
```

## 17. `wuxia_mumyeong_midgame_reunion`

```yaml
id: wuxia_mumyeong_midgame_reunion
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
source_refs:
  - notion_event:wuxia_mumyeong_midgame_reunion
  - docs/dev/Notion_Design_Coverage.md
notion_event_mapping:
  notion_event_id: wuxia_mumyeong_midgame_reunion
  notion_event_name: 무명 중반 재회
  mapping_status: preview_runtime_implemented
notion_sources_checked:
  events:
    - wuxia_mumyeong_midgame_reunion
    - wuxia_mumyeong_departure_truth_summary
    - wuxia_boss_first_appearance
    - wuxia_qingliu_attack_after_war
  operating_docs:
    - 04. 메인 루트 구조
    - 05. 사건 카드 운영 규칙
    - 07. 천기록 / 천외편린 보상
    - 99. 통합 체크포인트
mapping_status: preview_runtime_implemented
status: candidate
phase: [midgame_rival, rival_reunion]
priority_class: route_key
location_tags: [cheongryu_outer_courtyard, old_wound_trace, rival_duel]
surface: [sect_courtyard, cheonggi_record, training_chore]
anomaly_type: [faction_pressure, qi_deviation, notebook_oracle]
pressure_type: [sanity, danger, relation]
npc_slots: [early_rescuer]
candidate_characters: [mumyeong, seo_harin]
summary: 첫 대치, 카피 무공 공개, 정파식 제압술 흔적 이후 무명과 다시 마주쳐 라이벌/거울 관계와 서하린의 침묵을 깊게 만든다.
purpose: `wuxia_mumyeong_reads_orthodox_style`가 남긴 현악문/복호금쇄수/무명 시야 변주 단서를 무명의 개인 서사와 연결한다. 진실을 판결하거나 구원을 확정하지 않고, 무명과 주인공이 서로의 상처와 이해 방식을 비추는 중반 재회로 제한한다.
setup_text: 청류문 마당의 해가 기울자 무명이 다시 나타난다. 그는 싸움을 걸 듯 서 있지만, 시선은 서하린이 서 있던 빈 자리와 천기록의 접힌 모서리를 번갈아 스친다. 현악문과 복호금쇄수라는 이름은 아직 답이 아니라, 서로 모른 척할 수 없게 만든 흠집처럼 남아 있다.
runtime_preview_design_status: implemented
runtime_preview_start_conditions:
  runtime_mode: storypack_preview
  location: cheongryu_outer_courtyard
  required_flags: [mumyeong_reads_orthodox_style_resolved, orthodox_style_trace_recorded, mumyeong_first_confrontation_resolved, mumyeong_rival_thread_opened]
  forbidden_flags: [mumyeong_midgame_reunion_resolved]
  flavor_flags_only: [hyeonakmun_trace_suspected, bokho_geumsaesu_name_recorded, departure_truth_still_incomplete, seo_harin_mumyeong_silence_confirmed, copied_flow_weakness_noted, copy_style_hint_recorded]
presentation:
  visual_id: wuxia_mumyeong_midgame_reunion
  speaker: 무명
  layout: rival_reunion_trace
  effect_cues:
    - stable_terms: [무명, 서하린, 현악문]
choice_shapes:
  - id: ask_why_seoharin_never_called_him_traitor
    role: information_probe
    label_direction: 서하린이 왜 그를 배신자라 부르지 않았는지 묻는다
    expected_costs: [relation_risk, sanity_small]
    expected_gains: [seoharin_mumyeong_relation_clue, rival_wound_hint]
    outcome_hook:
      add_flags: [mumyeong_midgame_reunion_resolved, mumyeong_mirror_thread_deepened, seoharin_traitor_question_asked]
      add_clues: [seoharin_does_not_call_mumyeong_traitor, mumyeong_truth_still_incomplete]
      destination_id: cheongryu_outer_courtyard
      log_direction: 무명은 대답보다 먼저 침묵한다. 그 침묵은 서하린의 침묵과 같은 방향을 보고 있다.
  - id: show_the_hyeonakmun_trace_without_accusing
    role: orthodox_trace_probe
    label_direction: 현악문 흔적을 추궁이 아니라 기록으로 보여 준다
    expected_costs: [sanity_small, danger_small]
    expected_gains: [orthodox_trace_response, boss_wound_clue]
    outcome_hook:
      add_flags: [mumyeong_midgame_reunion_resolved, mumyeong_mirror_thread_deepened, hyeonakmun_trace_shared_carefully]
      add_clues: [hyeonakmun_trace_shared_without_accusation, boss_used_mumyeongs_wound]
      destination_id: cheongryu_outer_courtyard
      log_direction: 이름을 칼처럼 들이밀지 않자, 무명의 눈이 아주 짧게 흔들린다.
  - id: point_out_the_copied_form_gap
    role: rival_analysis
    label_direction: 훔친 초식과 이해한 흐름이 갈라지는 틈을 짚는다
    expected_costs: [danger_small, health_risk]
    expected_gains: [rival_mirror_clue, copy_gap_confirmed]
    outcome_hook:
      add_flags: [mumyeong_midgame_reunion_resolved, mumyeong_mirror_thread_deepened, copied_form_gap_named]
      add_clues: [rival_mirror_relationship_deepened, copy_is_surface_not_root]
      destination_id: cheongryu_outer_courtyard
      log_direction: 무명은 초식을 훔쳤고, 당신은 틈을 이해했다. 두 방식은 같은 상처 앞에서 서로를 비춘다.
  - id: keep_blades_low_and_watch_his_answer
    role: safe_observe
    fallback_choice: true
    label_direction: 칼끝을 낮추고 대답 대신 반응을 본다
    expected_costs: [unresolved_debt, sanity_small]
    expected_gains: [safe_distance, future_truth_marker]
    outcome_hook:
      add_flags: [mumyeong_midgame_reunion_resolved, mumyeong_mirror_thread_deepened, reunion_truth_deferred]
      add_clues: [mumyeong_truth_still_incomplete, rival_mirror_relationship_deepened]
      destination_id: cheongryu_outer_courtyard
      log_direction: 오늘은 답을 뽑아내지 않는다. 하지만 무명도 당신도 서로를 모른 척할 수 없게 됐다.
outcome_hooks:
  possible_flags: [mumyeong_midgame_reunion_resolved, mumyeong_mirror_thread_deepened, seoharin_traitor_question_asked, hyeonakmun_trace_shared_carefully, copied_form_gap_named, reunion_truth_deferred]
  possible_clues: [seoharin_does_not_call_mumyeong_traitor, boss_used_mumyeongs_wound, mumyeong_truth_still_incomplete, rival_mirror_relationship_deepened, hyeonakmun_trace_shared_without_accusation, copy_is_surface_not_root]
  possible_destinations: [cheongryu_outer_courtyard]
main_spine_link: 정파 무공 흔적을 무명/서하린 감정선과 라이벌 거울 관계로 연결해, 보스 첫 등장이나 무명 이탈 진실 정리 전에 중반 rival arc를 충분히 깊게 만든다.
randomization_notes: 1회성 midgame reunion. orthodox style trace 이후에만 열린다. 현악문/복호금쇄수 단서는 flavor와 clue로 쓰지만 full flashback이나 진실 summary로 확장하지 않는다.
promotion_notes: preview runtime으로 구현 완료. `wuxia_mumyeong_followup_after_orthodox_style_trace`에서 `wuxia_mumyeong_departure_truth_summary`, `wuxia_boss_first_appearance`, `wuxia_qingliu_attack_after_war`를 비교했고, 후반 truth reveal/boss wall/full flashback을 보류하기 위해 중반 재회를 먼저 골라 구현했다. legacy office bundle, legacy `escape-office` key, random copy-style system/table, combat resolver/schema, route graph/faction reputation/debt/relation schema, reward/ability/epilogue/return system, 천기록 정체 reveal은 열지 않는다.
runtime_preview_implementation_notes:
  implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml
  insert_after: wuxia_mumyeong_reads_orthodox_style
  selected_over: [wuxia_mumyeong_departure_truth_summary, wuxia_boss_first_appearance, wuxia_qingliu_attack_after_war]
  generated_artifacts:
    - crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json
    - web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json
  default_bundle_changed: false
  new_schema_opened: false
  next_handoff: wuxia_mumyeong_followup_after_midgame_reunion
```

## 18. `wuxia_boss_first_appearance`

```yaml
id: wuxia_boss_first_appearance
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
source_refs:
  - notion_event:wuxia_boss_first_appearance
  - docs/dev/Notion_Design_Coverage.md
notion_event_mapping:
  notion_event_id: wuxia_boss_first_appearance
  notion_event_name: 보스 첫 등장
  mapping_status: preview_runtime_implemented
status: implemented_in_storypack_preview
mapping_status: preview_runtime_implemented
phase: [midgame_boss, boss_wall_pressure]
priority_class: route_key
location_tags: [cheongryu_outer_courtyard, black_serpent_pressure, boss_wall]
surface: [sect_courtyard, faction_negotiation, training_chore]
anomaly_type: [faction_pressure, qi_deviation]
pressure_type: [danger, sanity, relation]
npc_slots: [early_rescuer]
candidate_characters: [black_serpent_boss, mumyeong, seo_harin]
summary: 무명 중반 재회 뒤 흑사방 보스가 처음 직접 압박으로 등장하고, 청류안으로 흐름을 읽어도 몸이 따라가지 못하는 벽을 각인한다.
setup_text: 청류문 마당의 소란이 가라앉기도 전에 검은 깃발의 그림자가 문턱에 걸린다. 흑사방주가 한 걸음 들어서자, 천기록의 빈 줄은 이상하게도 아무 해법도 쓰지 않는다. 움직임은 보인다. 사람의 약점을 재는 순서도 보인다. 하지만 몸이 먼저 굳는다.
runtime_preview_design_status: implemented
runtime_preview_start_conditions:
  runtime_mode: storypack_preview
  location: cheongryu_outer_courtyard
  required_flags: [mumyeong_midgame_reunion_resolved, mumyeong_mirror_thread_deepened, cheongryu_raid_survived, midgame_continuity_started]
  forbidden_flags: [boss_first_appearance_resolved]
  flavor_flags_only: [boss_used_mumyeongs_wound, hyeonakmun_trace_shared_without_accusation, seoharin_does_not_call_mumyeong_traitor, rival_mirror_relationship_deepened, reunion_truth_deferred]
presentation:
  visual_id: wuxia_boss_first_appearance
  speaker: 흑사방주
  layout: boss_wall_pressure
  effect_cues:
    - stable_terms: [흑사방주, 무명, 청류문]
choice_shapes:
  - id: read_the_boss_flow_and_fail_to_move
    role: overwhelming_observe
    label_direction: 보스의 흐름을 읽지만 몸이 늦는 것을 인정한다
    expected_costs: [sanity_medium, danger_small]
    expected_gains: [boss_wall_clue, body_limit_confirmed]
    outcome_hook:
      add_flags: [boss_first_appearance_resolved, boss_wall_thread_opened, black_serpent_core_pressure_opened, boss_flow_read_but_body_lagged]
      add_clues: [boss_reads_people_not_forms, boss_is_final_logic_wall]
      destination_id: cheongryu_outer_courtyard
      log_direction: 흐름은 보인다. 하지만 흑사방주는 초식보다 먼저 사람의 멈칫함을 베어 낸다.
  - id: pull_seo_harin_behind_broken_gate
    role: protective_reposition
    label_direction: 서하린을 부서진 산문 뒤로 물린다
    expected_costs: [danger_small, relation_risk]
    expected_gains: [seoharin_protected, qingliu_limit_clue]
    outcome_hook:
      add_flags: [boss_first_appearance_resolved, boss_wall_thread_opened, black_serpent_core_pressure_opened, seoharin_protected_from_boss_line]
      add_clues: [qingliu_cannot_outmuscle_boss_yet, boss_reads_people_not_forms]
      destination_id: cheongryu_outer_courtyard
      log_direction: 서하린은 물러서지만 눈을 피하지 않는다. 보스는 그 작은 움직임까지 이미 계산한 듯 웃는다.
  - id: watch_mumyeong_answer_the_boss
    role: rival_relation_probe
    label_direction: 무명이 보스의 말에 어떻게 반응하는지 본다
    expected_costs: [sanity_small, unresolved_debt]
    expected_gains: [mumyeong_boss_relation_clue, wound_exploitation_clue]
    outcome_hook:
      add_flags: [boss_first_appearance_resolved, boss_wall_thread_opened, black_serpent_core_pressure_opened, mumyeong_reacts_to_boss_voice]
      add_clues: [mumyeong_follows_power_that_saw_his_wound, boss_reads_people_not_forms]
      destination_id: cheongryu_outer_courtyard
      log_direction: 무명은 명령보다 먼저 침묵한다. 그 침묵은 복종이면서, 아직 끊지 못한 상처처럼 보인다.
  - id: retreat_before_the_second_step
    role: safe_reposition
    fallback_choice: true
    label_direction: 두 번째 걸음 전에 물러나 살아남는다
    expected_costs: [lost_momentum, sanity_small]
    expected_gains: [safe_distance, future_boss_marker]
    outcome_hook:
      add_flags: [boss_first_appearance_resolved, boss_wall_thread_opened, black_serpent_core_pressure_opened, boss_retreat_survival_chosen]
      add_clues: [boss_is_final_logic_wall, qingliu_cannot_outmuscle_boss_yet]
      destination_id: cheongryu_outer_courtyard
      log_direction: 오늘 넘을 벽이 아니라는 사실을 인정하는 것도 선택이다. 천기록은 그 후퇴를 비겁함이라 적지 않는다.
outcome_hooks:
  possible_flags: [boss_first_appearance_resolved, boss_wall_thread_opened, black_serpent_core_pressure_opened, boss_flow_read_but_body_lagged, seoharin_protected_from_boss_line, mumyeong_reacts_to_boss_voice, boss_retreat_survival_chosen]
  possible_clues: [boss_reads_people_not_forms, boss_is_final_logic_wall, mumyeong_follows_power_that_saw_his_wound, qingliu_cannot_outmuscle_boss_yet]
  possible_destinations: [cheongryu_outer_courtyard]
main_spine_link: 무명 중반 재회에서 열린 보스가 무명의 상처를 이용했다는 단서를 흑사방 보스의 첫 직접 압박으로 연결하되, 전투 결산이 아니라 최종 논리 벽의 첫 인상으로 남긴다.
randomization_notes: 1회성 boss-wall pressure. midgame reunion common hook 뒤에만 열고 `boss_first_appearance_resolved`로 반복을 막는다. `boss_used_mumyeongs_wound`는 flavor clue로만 사용해 특정 선택지에 갇히지 않게 한다.
promotion_notes: preview runtime으로 구현 완료. `wuxia_mumyeong_followup_after_midgame_reunion` docs-only handoff에서 다음 runtime 후보로 선택했고, 압도감/조직력/약점 읽기/무명이 따르는 이유를 기존 flags/clues/log/presentation으로 landing했다. `wuxia_mumyeong_departure_truth_summary`는 후반 truth reveal/서하린 진실 전달/구원 조건 확정 때문에 보류했고, `wuxia_qingliu_attack_after_war`는 full flashback/backstory reveal 때문에 보류했다. `wuxia_mumyeong_request_for_aid`는 Notion search에서 확인했지만 repo 현 후보 밖 future bridge로 남긴다. boss combat/final boss resolution, legacy office bundle, legacy `escape-office` key, random copy-style system/table, combat resolver/schema, route graph/faction reputation/debt/relation schema, reward/ability/epilogue/return system, 천기록 정체 reveal은 열지 않는다.
runtime_preview_implementation_notes:
  implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml
  insert_after: wuxia_mumyeong_midgame_reunion
  selected_over: [wuxia_mumyeong_departure_truth_summary, wuxia_qingliu_attack_after_war]
  deferred_bridge_candidates: [wuxia_mumyeong_request_for_aid]
  generated_artifacts:
    - crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json
    - web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json
  default_bundle_changed: false
  new_schema_opened: false
  next_handoff: wuxia_boss_followup_after_first_appearance
```

## 19. `wuxia_mumyeong_request_for_aid`

```yaml
id: wuxia_mumyeong_request_for_aid
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
source_refs:
  - notion_event:wuxia_mumyeong_request_for_aid
  - docs/dev/Notion_Design_Coverage.md
notion_event_mapping:
  notion_event_id: wuxia_mumyeong_request_for_aid
  notion_event_name: 무명의 도움 요청
  mapping_status: preview_runtime_implemented
status: implemented_in_storypack_preview
mapping_status: preview_runtime_implemented
phase: [midgame_rival, failed_aid_records]
priority_class: route_key
location_tags: [cheongryu_outer_courtyard, orthodox_refusal, mumyeong_past]
surface: [sect_courtyard, faction_negotiation, cheonggi_record]
anomaly_type: [faction_pressure]
pressure_type: [sanity, relation, danger]
npc_slots: [early_rescuer]
candidate_characters: [mumyeong, seo_harin, black_serpent_boss]
summary: 보스 첫 등장 뒤 무명이 청류문을 살리려 정파 문파들에 도움을 청했지만 거절당했다는 기록/소문을 추적한다.
setup_text: 청류문 마당에 남은 검은 깃발의 먼지가 가라앉자, 천기록의 빈 줄 옆에 오래된 객잔 이름과 반쯤 찢긴 서찰 봉인이 겹쳐 보인다. 무명은 배신자가 되기 전에 살릴 방법을 찾았고, 정파의 문턱은 그를 여러 번 돌려보낸 듯하다.
runtime_preview_design_status: implemented
runtime_preview_start_conditions:
  runtime_mode: storypack_preview
  conditions:
    locations: [cheongryu_outer_courtyard]
  required_flags: [boss_first_appearance_resolved, boss_wall_thread_opened, black_serpent_core_pressure_opened, mumyeong_mirror_thread_deepened, orthodox_style_trace_recorded, midgame_continuity_started]
  forbidden_flags: [mumyeong_request_for_aid_resolved]
  flavor_flags_only: [boss_used_mumyeongs_wound, hyeonakmun_trace_shared_without_accusation, mumyeong_follows_power_that_saw_his_wound, boss_reads_people_not_forms, boss_is_final_logic_wall, qingliu_cannot_outmuscle_boss_yet, seoharin_does_not_call_mumyeong_traitor]
presentation:
  visual_id: wuxia_mumyeong_request_for_aid
  speaker: 천기록
  layout: failed_aid_records
  effect_cues:
    - stable_terms: [무명, 청류문, 정파]
choice_shapes:
  - id: search_the_rejected_aid_letters
    role: safe_reading
    label_direction: 거절당한 도움 요청 서찰을 찾아 읽는다
    expected_costs: []
    expected_gains: [failed_aid_record, orthodox_refusal_clue]
    outcome_hook:
      add_flags: [mumyeong_request_for_aid_resolved, mumyeong_failed_aid_thread_opened, orthodox_hypocrisy_thread_opened, rejected_aid_letters_read]
      add_clues: [mumyeong_tried_to_save_qingliu, orthodox_refusal_broke_mumyeong]
      destination_id: cheongryu_outer_courtyard
      log_direction: 서찰에는 도움을 청한 말보다 거절의 예법이 더 길게 남아 있다.
  - id: follow_old_inn_rumors_about_mumyeong
    role: rumor_probe
    label_direction: 객잔에 남은 무명 소문을 좇는다
    expected_costs: [sanity_small, danger_small]
    expected_gains: [rumor_thread, boss_logic_clue]
    outcome_hook:
      add_flags: [mumyeong_request_for_aid_resolved, mumyeong_failed_aid_thread_opened, orthodox_hypocrisy_thread_opened, inn_rumor_thread_followed]
      add_clues: [boss_logic_found_mumyeongs_wound, orthodox_refusal_broke_mumyeong]
      destination_id: cheongryu_outer_courtyard
      log_direction: 사람들은 그가 떠난 날보다 문전박대당한 밤을 더 작게 말한다.
  - id: ask_seo_harin_what_help_never_came
    role: relation_probe
    label_direction: 서하린에게 오지 않았던 도움을 묻는다
    expected_costs: [relation_risk, sanity_small]
    expected_gains: [seoharin_silence_context, failed_aid_context]
    outcome_hook:
      add_flags: [mumyeong_request_for_aid_resolved, mumyeong_failed_aid_thread_opened, orthodox_hypocrisy_thread_opened, seoharin_failed_aid_question_asked]
      add_clues: [seoharin_does_not_know_failed_aid, aid_refusal_precedes_departure_truth]
      destination_id: cheongryu_outer_courtyard
      log_direction: 서하린은 그 서찰을 본 적이 없다고 말한다. 그 무지가 오히려 오래된 상처의 모양을 만든다.
  - id: keep_the_failed_aid_record_unshown
    role: safe_defer
    fallback_choice: true
    label_direction: 실패한 도움 요청 기록을 아직 보여주지 않는다
    expected_costs: [unresolved_debt]
    expected_gains: [safe_distance, future_truth_marker]
    outcome_hook:
      add_flags: [mumyeong_request_for_aid_resolved, mumyeong_failed_aid_thread_opened, orthodox_hypocrisy_thread_opened, failed_aid_record_kept_unshown]
      add_clues: [aid_refusal_precedes_departure_truth, mumyeong_tried_to_save_qingliu]
      destination_id: cheongryu_outer_courtyard
      log_direction: 기록은 접어 둔다. 오늘 밝히지 않는 진실도, 언젠가 누군가를 살릴 수 있다.
outcome_hooks:
  possible_flags: [mumyeong_request_for_aid_resolved, mumyeong_failed_aid_thread_opened, orthodox_hypocrisy_thread_opened, rejected_aid_letters_read, inn_rumor_thread_followed, seoharin_failed_aid_question_asked, failed_aid_record_kept_unshown]
  possible_clues: [mumyeong_tried_to_save_qingliu, orthodox_refusal_broke_mumyeong, boss_logic_found_mumyeongs_wound, aid_refusal_precedes_departure_truth, seoharin_does_not_know_failed_aid]
  possible_items: [rejected_aid_letter_fragment]
  possible_destinations: [cheongryu_outer_courtyard]
main_spine_link: 보스 첫 등장으로 열린 힘의 논리와 무명의 상처를, 정파가 청류문을 외면한 기록/소문으로 이어서 무명을 단순 배신자가 아닌 실패한 구조자 후보로 만든다.
randomization_notes: 1회성 failed-aid records bridge. boss first appearance 뒤에만 열고 `mumyeong_request_for_aid_resolved`로 반복을 막는다. 서하린에게 진실을 전달하거나 무명 구원을 확정하지 않는다.
promotion_notes: preview runtime으로 구현 완료. `wuxia_boss_followup_after_first_appearance` docs-only handoff에서 다음 runtime 후보로 선택했고, 보스 첫 등장 뒤 무명의 도움 요청 실패 기록/소문을 기존 flags/clues/log/presentation hook으로 landing했다. `wuxia_mumyeong_departure_truth_summary`는 후반 truth reveal/서하린 진실 전달/구원 조건 확정 때문에 보류했고, `wuxia_qingliu_attack_after_war`는 full flashback/backstory reveal 때문에 보류했으며, `wuxia_boss_resolution`은 final boss/faction/epilogue 결산 때문에 보류했다. legacy office bundle, legacy `escape-office` key, random copy-style system/table, combat resolver/schema, route graph/faction reputation/debt/relation schema, reward/ability/epilogue/return system, 천기록 정체 reveal은 열지 않는다.
runtime_preview_implementation_notes:
  implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml
  insert_after: wuxia_boss_first_appearance
  selected_over: [wuxia_mumyeong_departure_truth_summary, wuxia_qingliu_attack_after_war, wuxia_boss_resolution]
  generated_artifacts:
    - crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json
    - web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json
  default_bundle_changed: false
  new_schema_opened: false
  next_handoff: wuxia_mumyeong_followup_after_failed_aid
  next_selected_candidate: wuxia_mumyeong_awakening
```

## 20. `wuxia_mumyeong_awakening`

```yaml
id: wuxia_mumyeong_awakening
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
source_refs:
  - notion_event:wuxia_mumyeong_awakening
  - docs/dev/Notion_Design_Coverage.md
notion_event_mapping:
  notion_event_id: wuxia_mumyeong_awakening
  notion_event_name: 무명의 각성
  mapping_status: preview_runtime_implemented
status: implemented_in_storypack_preview
mapping_status: preview_runtime_implemented
phase: [midgame_rival, anger_copy_bloom]
priority_class: route_key
location_tags: [cheongryu_outer_courtyard, orthodox_refusal, mumyeong_past]
surface: [sect_courtyard, cheonggi_record, faction_negotiation]
anomaly_type: [faction_pressure, qi_deviation]
pressure_type: [sanity, relation, danger]
npc_slots: [early_rescuer]
candidate_characters: [mumyeong, seo_harin]
summary: 도움 요청 실패와 정파 무공 흔적 이후, 무명이 분노 속에서 카피 무공을 극단적으로 개화한 순간을 추적한다.
setup_text: 청류문 마당에 접어 둔 거절 서찰과 복호금쇄수의 흔적이 겹쳐 보인다. 천기록은 무명의 재능을 칭찬하지 않고, 분노가 남의 초식을 훔쳐 덧씌운 순간의 결을 적는다.
runtime_preview_design_status: implemented
runtime_preview_start_conditions:
  runtime_mode: storypack_preview
  conditions:
    locations: [cheongryu_outer_courtyard]
  required_flags: [mumyeong_request_for_aid_resolved, mumyeong_failed_aid_thread_opened, orthodox_hypocrisy_thread_opened, mumyeong_reads_orthodox_style_resolved, orthodox_style_trace_recorded, mumyeong_copy_style_reveal_resolved, copy_style_hint_recorded, midgame_continuity_started]
  forbidden_flags: [mumyeong_awakening_resolved]
  flavor_flags_only: [mumyeong_tried_to_save_qingliu, orthodox_refusal_broke_mumyeong, boss_logic_found_mumyeongs_wound, aid_refusal_precedes_departure_truth, hyeonakmun_trace_suspected, bokho_geumsaesu_name_recorded, departure_truth_still_incomplete, seoharin_does_not_know_failed_aid]
presentation:
  visual_id: wuxia_mumyeong_awakening
  speaker: 천기록
  layout: anger_copy_bloom
  effect_cues:
    - stable_terms: [무명, 카피, 분노]
choice_shapes:
  - id: compare_anger_to_copied_flow
    role: safe_observe
    fallback_choice: true
    label_direction: 분노가 베껴 낸 흐름과 복사한 초식을 비교한다
    expected_costs: []
    expected_gains: [copy_wound_clue, safe_distance]
    outcome_hook:
      add_flags: [mumyeong_awakening_resolved, mumyeong_awakening_thread_opened, copy_corruption_thread_opened]
      add_clues: [mumyeong_copy_bloomed_from_anger, copy_is_wound_not_growth]
      destination_id: cheongryu_outer_courtyard
      log_direction: 천기록은 무명의 초식이 자란 것이 아니라 상처 위에 덧씌워졌다고 적는다.
  - id: trace_awakening_from_failed_aid
    role: clue_trace
    label_direction: 도움 요청 실패가 각성으로 이어진 흔적을 좇는다
    expected_costs: [sanity_small]
    expected_gains: [failed_aid_to_anger_bridge, hyeonakmun_context]
    outcome_hook:
      add_flags: [mumyeong_awakening_resolved, mumyeong_awakening_thread_opened, copy_corruption_thread_opened]
      add_clues: [awakening_points_to_hyeonakmun_without_full_truth, mumyeong_copy_bloomed_from_anger]
      destination_id: cheongryu_outer_courtyard
      log_direction: 거절당한 서찰과 정파식 손자국 사이에서 무명의 분노가 처음 형태를 얻는다.
  - id: ask_what_the_copy_cost_him
    role: relation_probe
    label_direction: 그 카피가 무명에게 무엇을 빼앗았는지 묻는다
    expected_costs: [relation_risk, sanity_small]
    expected_gains: [copy_cost_context, mumyeong_mirror_context]
    outcome_hook:
      add_flags: [mumyeong_awakening_resolved, mumyeong_awakening_thread_opened, copy_corruption_thread_opened]
      add_clues: [copy_is_wound_not_growth, protagonist_understands_where_mumyeong_overlays]
      destination_id: cheongryu_outer_courtyard
      log_direction: 묻는 말은 닿지만 답은 오지 않는다. 침묵이 오히려 카피의 대가를 설명한다.
  - id: stop_before_calling_it_salvation
    role: safe_defer
    label_direction: 이것을 아직 구원이라고 부르지 않는다
    expected_costs: [unresolved_debt]
    expected_gains: [truth_boundary, future_salvation_marker]
    outcome_hook:
      add_flags: [mumyeong_awakening_resolved, mumyeong_awakening_thread_opened, copy_corruption_thread_opened]
      add_clues: [salvation_truth_still_unready, awakening_points_to_hyeonakmun_without_full_truth]
      destination_id: cheongryu_outer_courtyard
      log_direction: 구원은 아직 이름 붙일 수 없다. 오늘은 상처가 어떻게 힘의 모양을 흉내 냈는지만 기록한다.
outcome_hooks:
  possible_flags: [mumyeong_awakening_resolved, mumyeong_awakening_thread_opened, copy_corruption_thread_opened]
  possible_clues: [mumyeong_copy_bloomed_from_anger, copy_is_wound_not_growth, protagonist_understands_where_mumyeong_overlays, awakening_points_to_hyeonakmun_without_full_truth, salvation_truth_still_unready]
  possible_destinations: [cheongryu_outer_courtyard]
main_spine_link: 도움 요청 실패 기록과 정파 무공 흔적을 무명의 분노/카피 무공 변질로 잇되, 무명 이탈의 진실 전체나 서하린에게 전달할 구원 조건은 아직 열지 않는다.
randomization_notes: 1회성 anger/copy bloom bridge. request-for-aid와 orthodox-style trace 뒤에만 열고 `mumyeong_awakening_resolved`로 반복을 막는다.
promotion_notes: preview runtime으로 구현 완료. `wuxia_mumyeong_followup_after_failed_aid` docs-only handoff에서 다음 runtime 후보로 선택했고, 도움 요청 실패 기록과 정파 무공 흔적을 무명의 분노/카피 무공 변질로 landing했다. `wuxia_mumyeong_departure_truth_summary`는 후반 truth/서하린 진실 전달/구원 조건 범위라 보류했고, `wuxia_qingliu_attack_after_war`는 full flashback source라 보류했으며, `wuxia_boss_resolution`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_destroys_orthodox_sect`는 후반/final consequence 범위라 보류했다. legacy office bundle, legacy `escape-office` key, random copy-style system/table, combat resolver/schema, route graph/faction reputation/debt/relation schema, reward/ability/epilogue/return system, 천기록 정체 reveal은 열지 않는다.
runtime_preview_implementation_notes:
  implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml
  insert_after: wuxia_mumyeong_request_for_aid
  selected_over: [wuxia_mumyeong_departure_truth_summary, wuxia_qingliu_attack_after_war, wuxia_boss_resolution, wuxia_boss_recruits_mumyeong, wuxia_mumyeong_destroys_orthodox_sect]
  generated_artifacts:
    - crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json
    - web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json
  default_bundle_changed: false
  new_schema_opened: false
  next_handoff: wuxia_mumyeong_followup_after_awakening
```

## 21. `wuxia_qingliu_attack_after_war`

```yaml
id: wuxia_qingliu_attack_after_war
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
source_refs:
  - notion_event:wuxia_qingliu_attack_after_war
  - docs/dev/Notion_Design_Coverage.md
  - docs/dev/Development_Plan.md#044-2026-06-02-docs-only-awakening-follow-up-handoff-wuxia_qingliu_attack_after_war
notion_event_mapping:
  notion_event_id: wuxia_qingliu_attack_after_war
  notion_event_name: 무너져가는 청류문 습격
  mapping_status: preview_runtime_implemented
status: implemented_in_storypack_preview
mapping_status: preview_runtime_implemented
phase: [midgame_backstory, attack_trace_investigation]
priority_class: route_key
location_tags: [cheongryu_outer_courtyard, hyeonakmun_trace, qingliu_aftermath]
surface: [sect_courtyard, cheonggi_record, faction_negotiation]
anomaly_type: [faction_pressure, qi_deviation]
pressure_type: [sanity, danger, relation]
npc_slots: [early_rescuer]
candidate_characters: [seo_harin, mumyeong]
summary: 무명 각성 이후 청류문 폐허에 남은 현악문/복호금쇄수 흔적을 조사하되, 청류문 습격 전체 회상이나 무명 이탈 진실은 아직 열지 않는다.
setup_text: 청류문 외원에는 전쟁 뒤의 습격 흔적이 아직 벽과 자물쇠에 남아 있다. 천기록은 장면을 재생하지 않고, 현악문이라는 이름과 복호금쇄수의 손자국이 청류문 상처 위에 어떻게 겹쳤는지만 적는다.
runtime_preview_design_status: implemented
runtime_preview_start_conditions:
  runtime_mode: storypack_preview
  conditions:
    locations: [cheongryu_outer_courtyard]
  required_flags: [mumyeong_awakening_resolved, mumyeong_awakening_thread_opened, copy_corruption_thread_opened, mumyeong_request_for_aid_resolved, mumyeong_failed_aid_thread_opened, orthodox_hypocrisy_thread_opened, mumyeong_reads_orthodox_style_resolved, orthodox_style_trace_recorded, midgame_continuity_started]
  forbidden_flags: [qingliu_attack_after_war_resolved]
  flavor_flags_only: [hyeonakmun_trace_suspected, bokho_geumsaesu_name_recorded, mumyeong_tried_to_save_qingliu, orthodox_refusal_broke_mumyeong, seoharin_does_not_know_failed_aid, salvation_truth_still_unready, awakening_points_to_hyeonakmun_without_full_truth]
presentation:
  visual_id: wuxia_qingliu_attack_after_war
  speaker: 천기록
  layout: attack_trace_investigation
  effect_cues:
    - stable_terms: [청류문, 현악문, 복호금쇄수]
choice_shapes:
  - id: inspect_bokho_lock_scars
    role: safe_observe
    fallback_choice: true
    label_direction: 자물쇠와 문틀에 남은 복호금쇄수 자국을 살핀다
    expected_costs: []
    expected_gains: [bokho_trace_clue, safe_distance]
    outcome_hook:
      add_flags: [qingliu_attack_after_war_resolved, qingliu_attack_trace_confirmed, hyeonakmun_attack_thread_opened]
      add_clues: [bokho_geumsaesu_used_on_qingliu, full_flashback_still_unopened]
      destination_id: cheongryu_outer_courtyard
      log_direction: 천기록은 자물쇠의 상처가 무너진 힘이 아니라 잠그고 부러뜨린 손길이라고 적는다.
  - id: compare_hyeonakmun_trace_to_qingliu_wounds
    role: clue_compare
    label_direction: 현악문 흔적과 청류문 상처의 결을 대조한다
    expected_costs: [sanity_small]
    expected_gains: [hyeonakmun_trace_context, main_sect_boundary]
    outcome_hook:
      add_flags: [qingliu_attack_after_war_resolved, qingliu_attack_trace_confirmed, hyeonakmun_attack_thread_opened]
      add_clues: [qingliu_attack_trace_points_to_hyeonakmun, main_sect_not_directly_accused]
      destination_id: cheongryu_outer_courtyard
      log_direction: 이름은 현악문 쪽을 향하지만, 천기록은 아직 한 문파 전체를 죄인으로 쓰지 않는다.
  - id: ask_seo_harin_what_she_saw_afterward
    role: relation_probe
    label_direction: 서하린이 습격 뒤에 무엇을 보았는지 조심스레 묻는다
    expected_costs: [sanity_small, relation_risk]
    expected_gains: [seoharin_aftermath_context, partial_truth_boundary]
    outcome_hook:
      add_flags: [qingliu_attack_after_war_resolved, qingliu_attack_trace_confirmed, hyeonakmun_attack_thread_opened]
      add_clues: [seoharin_saw_aftermath_not_full_truth, qingliu_attack_trace_points_to_hyeonakmun]
      destination_id: cheongryu_outer_courtyard
      log_direction: 서하린의 기억은 피와 빈방에 멈춰 있다. 그녀도 습격의 중심을 보지는 못했다.
  - id: stop_before_replaying_the_attack
    role: safe_defer
    label_direction: 습격을 다시 재생하기 전에 기록을 덮는다
    expected_costs: [unresolved_debt]
    expected_gains: [flashback_boundary, future_truth_marker]
    outcome_hook:
      add_flags: [qingliu_attack_after_war_resolved, qingliu_attack_trace_confirmed, hyeonakmun_attack_thread_opened]
      add_clues: [full_flashback_still_unopened, seoharin_saw_aftermath_not_full_truth]
      destination_id: cheongryu_outer_courtyard
      log_direction: 오늘은 회상이 아니라 흔적만 읽는다. 공격의 밤은 아직 장면으로 열리지 않는다.
outcome_hooks:
  possible_flags: [qingliu_attack_after_war_resolved, qingliu_attack_trace_confirmed, hyeonakmun_attack_thread_opened]
  possible_clues: [qingliu_attack_trace_points_to_hyeonakmun, bokho_geumsaesu_used_on_qingliu, seoharin_saw_aftermath_not_full_truth, main_sect_not_directly_accused, full_flashback_still_unopened]
  possible_destinations: [cheongryu_outer_courtyard]
main_spine_link: 무명 각성 이후 정파 무공 흔적을 청류문 습격 상처로 좁혀, 현악문/복호금쇄수 thread를 강화한다. 다만 무명 이탈의 진실 전체나 서하린에게 전달할 구원 조건은 아직 열지 않는다.
randomization_notes: 1회성 attack-trace investigation. 각성, failed-aid, orthodox-style trace 뒤에만 열고 `qingliu_attack_after_war_resolved`로 반복을 막는다.
promotion_notes: preview runtime으로 구현 완료. `wuxia_mumyeong_followup_after_awakening` handoff에서 무명 각성 이후 후보들을 비교했고, `wuxia_qingliu_attack_after_war`를 full flashback이 아니라 현악문/복호금쇄수 흔적 조사로 제한해 선택했다. `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`은 후반/final/truth/salvation 범위라 보류한다. legacy office bundle, legacy `escape-office` key, random copy-style system/table, combat resolver/schema, route graph/faction reputation/debt/relation schema, reward/ability/epilogue/return system, 천기록 정체 reveal은 열지 않는다.
runtime_preview_implementation_notes:
  implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml
  insert_after: wuxia_mumyeong_awakening
  selected_over: [wuxia_mumyeong_destroys_orthodox_sect, wuxia_boss_recruits_mumyeong, wuxia_mumyeong_departure_truth_summary, wuxia_mumyeong_resolution, wuxia_boss_resolution]
  generated_artifacts:
    - crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json
    - web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json
  default_bundle_changed: false
  new_schema_opened: false
  next_handoff: wuxia_qingliu_attack_after_war_followup
```

## 22. `wuxia_mumyeong_destroys_orthodox_sect`

```yaml
id: wuxia_mumyeong_destroys_orthodox_sect
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
source_refs:
  - notion_event:wuxia_mumyeong_destroys_orthodox_sect
  - docs/dev/Notion_Design_Coverage.md
  - docs/dev/Development_Plan.md#046-2026-06-02-docs-only-post-qingliu-trace-handoff-wuxia_mumyeong_destroys_orthodox_sect
notion_event_mapping:
  notion_event_id: wuxia_mumyeong_destroys_orthodox_sect
  notion_event_name: 정파 문파 멸문
  mapping_status: preview_runtime_implemented
status: implemented_in_storypack_preview
mapping_status: preview_runtime_implemented
phase: [midgame_backstory, hyeonakmun_consequence_trace]
priority_class: route_key
location_tags: [cheongryu_outer_courtyard, hyeonakmun_trace, destroyed_orthodox_sect]
surface: [sect_courtyard, cheonggi_record, faction_negotiation]
anomaly_type: [faction_pressure, qi_deviation]
pressure_type: [sanity, danger, relation]
npc_slots: [early_rescuer]
candidate_characters: [seo_harin, mumyeong, black_serpent_boss]
summary: 청류문 흔적 조사 이후 빈 현악문 산문과 기록/풍문으로 무명이 현악문을 멸문시켰다는 consequence를 확인하되, playable 멸문 전투나 full flashback은 열지 않는다.
setup_text: 현악문이라는 이름은 청류문 외원의 흉터에서 끝나지 않는다. 천기록은 멸문 장면을 재생하지 않고, 비어 버린 산문과 부러진 현판, 복호금쇄수의 이름이 사라진 장부 가장자리만 적는다.
runtime_preview_design_status: implemented
runtime_preview_start_conditions:
  runtime_mode: storypack_preview
  conditions:
    locations: [cheongryu_outer_courtyard]
  required_flags: [qingliu_attack_after_war_resolved, qingliu_attack_trace_confirmed, hyeonakmun_attack_thread_opened, mumyeong_awakening_resolved, midgame_continuity_started]
  forbidden_flags: [mumyeong_destroys_orthodox_sect_resolved]
  flavor_flags_only: [qingliu_attack_trace_points_to_hyeonakmun, bokho_geumsaesu_used_on_qingliu, main_sect_not_directly_accused, full_flashback_still_unopened, mumyeong_tried_to_save_qingliu, orthodox_refusal_broke_mumyeong, salvation_truth_still_unready]
presentation:
  visual_id: wuxia_mumyeong_destroys_orthodox_sect
  speaker: 천기록
  layout: hyeonakmun_empty_gate_record
  effect_cues:
    - stable_terms: [현악문, 복호금쇄수, 무명]
choice_shapes:
  - id: read_hyeonakmun_empty_gate_record
    role: safe_reading
    fallback_choice: true
    label_direction: 빈 현악문 산문에 남은 기록을 읽는다
    expected_costs: []
    expected_gains: [hyeonakmun_empty_gate_context, safe_distance]
    outcome_hook:
      add_flags: [mumyeong_destroys_orthodox_sect_resolved, hyeonakmun_destruction_thread_opened, departure_truth_thread_deepened]
      add_clues: [hyeonakmun_was_destroyed_after_qingliu_attack, destruction_is_consequence_not_salvation]
      destination_id: cheongryu_outer_courtyard
      log_direction: 천기록은 멸문을 장면으로 보지 않는다. 비어 버린 산문과 끊긴 명패만이 결과를 말한다.
  - id: trace_bokho_lock_to_mumyeong
    role: clue_compare
    label_direction: 복호금쇄수 흔적이 무명의 분노로 되돌아간 길을 대조한다
    expected_costs: [sanity_small]
    expected_gains: [mumyeong_consequence_context, departure_truth_bridge]
    outcome_hook:
      add_flags: [mumyeong_destroys_orthodox_sect_resolved, hyeonakmun_destruction_thread_opened, departure_truth_thread_deepened]
      add_clues: [mumyeong_destroyed_hyeonakmun_alone, hyeonakmun_was_destroyed_after_qingliu_attack]
      destination_id: cheongryu_outer_courtyard
      log_direction: 복호금쇄수의 이름은 무명의 분노를 지나 현악문의 빈 문턱으로 돌아온다.
  - id: ask_why_seoharin_never_heard_full_story
    role: relation_probe
    label_direction: 왜 서하린이 전체 이야기를 듣지 못했는지 조심스럽게 묻는다
    expected_costs: [sanity_small, relation_risk]
    expected_gains: [seoharin_truth_boundary, future_delivery_marker]
    outcome_hook:
      add_flags: [mumyeong_destroys_orthodox_sect_resolved, hyeonakmun_destruction_thread_opened, departure_truth_thread_deepened]
      add_clues: [seoharin_truth_delivery_still_unopened, destruction_is_consequence_not_salvation]
      destination_id: cheongryu_outer_courtyard
      log_direction: 서하린에게 닿지 못한 진실은 아직 말이 되지 않는다. 오늘은 왜 침묵이 남았는지만 적는다.
  - id: stop_before_counting_the_dead
    role: safe_defer
    label_direction: 죽은 사람의 수를 세기 전에 기록을 덮는다
    expected_costs: [unresolved_debt]
    expected_gains: [full_flashback_boundary, boss_recruitment_marker]
    outcome_hook:
      add_flags: [mumyeong_destroys_orthodox_sect_resolved, hyeonakmun_destruction_thread_opened, departure_truth_thread_deepened]
      add_clues: [boss_recruitment_still_unopened, seoharin_truth_delivery_still_unopened]
      destination_id: cheongryu_outer_courtyard
      log_direction: 죽은 사람의 수를 세기 시작하면 장면이 열린다. 천기록은 아직 그 문을 열지 않는다.
outcome_hooks:
  possible_flags: [mumyeong_destroys_orthodox_sect_resolved, hyeonakmun_destruction_thread_opened, departure_truth_thread_deepened]
  possible_clues: [hyeonakmun_was_destroyed_after_qingliu_attack, mumyeong_destroyed_hyeonakmun_alone, destruction_is_consequence_not_salvation, seoharin_truth_delivery_still_unopened, boss_recruitment_still_unopened]
  possible_destinations: [cheongryu_outer_courtyard]
main_spine_link: 청류문 습격 흔적 조사가 남긴 현악문 thread를 무명의 후속 consequence로 좁힌다. 다만 무명 이탈의 진실 전체, 서하린에게 진실 전달, 보스 스카웃은 아직 다음 사건으로 넘긴다.
randomization_notes: 1회성 Hyeonakmun consequence trace. Qingliu attack trace 뒤에만 열고 `mumyeong_destroys_orthodox_sect_resolved`로 반복을 막는다.
promotion_notes: preview runtime으로 구현 완료. `wuxia_qingliu_attack_after_war_followup` docs-only handoff에서 다음 runtime 후보로 선택했고, Notion 사건 카드 DB의 정파 문파 멸문을 전투/멸문 flashback으로 직접 재생하지 않고 빈 현악문 산문과 기록/풍문을 확인하는 limited consequence trace로 landing했다. `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, `wuxia_seoharin_empty_place`는 다음 handoff에서 다시 비교한다. legacy office bundle, legacy `escape-office` key, random copy-style system/table, combat resolver/schema, route graph/faction reputation/debt/relation schema, reward/ability/epilogue/return system, 천기록 정체 reveal은 열지 않는다.
runtime_preview_implementation_notes:
  implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml
  insert_after: wuxia_qingliu_attack_after_war
  selected_over: [wuxia_boss_recruits_mumyeong, wuxia_mumyeong_departure_truth_summary, wuxia_mumyeong_resolution, wuxia_boss_resolution, wuxia_seoharin_empty_place]
  generated_artifacts:
    - crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json
    - web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json
  default_bundle_changed: false
  new_schema_opened: false
  next_handoff: wuxia_mumyeong_destroys_orthodox_sect_followup
```

## 23. `wuxia_boss_recruits_mumyeong`

```yaml
id: wuxia_boss_recruits_mumyeong
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: implemented_in_storypack_preview
source_refs:
  - notion_event:wuxia_boss_recruits_mumyeong
  - docs/dev/Notion_Design_Coverage.md
  - docs/dev/Development_Plan.md#048-2026-06-02-docs-only-hyeonakmun-consequence-follow-up-handoff-wuxia_boss_recruits_mumyeong
  - docs/dev/Development_Plan.md#049-2026-06-02-무협-wuxia_boss_recruits_mumyeong-preview-runtime-slice
notion_mapping:
  notion_event_id: wuxia_boss_recruits_mumyeong
  notion_event_name: 흑사방 보스의 스카웃
  mapping_status: preview_runtime_implemented
runtime_preview_design_status: implemented
phase: [midgame_backstory]
priority_class: main_branch
location_tags: [sect_courtyard, ruined_gate, recruitment_trace]
surface: [sect_courtyard, cheonggi_record, faction_negotiation]
anomaly_type: [boss_recruitment_trace]
pressure_type: [danger, sanity]
npc_slots: [rival, blood_moon_antagonist]
candidate_characters: [무명, 흑사방주]
summary: 현악문 consequence 이후 흑사방 보스가 무명의 상처와 힘을 보고 스카웃했다는 흔적을 확인한다.
setup_text: 천기록은 빈 현악문 산문 뒤에 이어진 장면을 곧장 열지 않는다. 대신 검은 비늘 문양과 짧은 문장만 남긴다. 흑사방주는 무명의 이름보다 먼저 상처를 보았고, 그 상처를 힘으로 바꿀 자리를 제안했다.
presentation:
  visual_id: wuxia_boss_recruits_mumyeong
  speaker: 천기록
  layout: boss_recruitment_trace
  stable_terms: [흑사방주, 무명, 현악문]
conditions:
  locations: [cheongryu_outer_courtyard]
  required_flags: [mumyeong_destroys_orthodox_sect_resolved, hyeonakmun_destruction_thread_opened, departure_truth_thread_deepened, boss_first_appearance_resolved, boss_wall_thread_opened, black_serpent_core_pressure_opened, midgame_continuity_started]
  forbidden_flags: [boss_recruits_mumyeong_resolved]
choice_shapes:
  - id: trace_boss_offer_after_hyeonakmun
    role: safe_trace
    expected_costs: [sanity_small, danger_small]
    expected_gains: [boss_recruited_mumyeong_after_hyeonakmun, recruitment_was_not_salvation]
    outcome_hook:
      add_flags: [boss_recruits_mumyeong_resolved, boss_recruitment_thread_opened, boss_saw_mumyeongs_wound]
      add_clues: [boss_recruited_mumyeong_after_hyeonakmun, recruitment_was_not_salvation]
      destination_id: cheongryu_outer_courtyard
  - id: read_mumyeong_choice_without_excusing_it
    role: moral_read
    expected_costs: [sanity_small]
    expected_gains: [mumyeong_had_nowhere_to_stand_after_destruction, boss_turned_wound_into_power]
    outcome_hook:
      add_flags: [boss_recruits_mumyeong_resolved, boss_recruitment_thread_opened, mumyeong_choice_thread_deepened]
      add_clues: [mumyeong_had_nowhere_to_stand_after_destruction, boss_turned_wound_into_power]
      destination_id: cheongryu_outer_courtyard
  - id: search_black_serpent_recruitment_record
    role: risky_investigation
    expected_costs: [sanity_small, danger_small]
    expected_gains: [black_serpent_recruits_wounds_not_names, boss_reads_people_not_forms]
    outcome_hook:
      add_flags: [boss_recruits_mumyeong_resolved, boss_recruitment_thread_opened, black_serpent_recruitment_record_seen]
      add_clues: [black_serpent_recruits_wounds_not_names, boss_reads_people_not_forms]
      destination_id: cheongryu_outer_courtyard
  - id: stop_before_following_him_into_black_serpent
    role: safe_stop
    expected_costs: [sanity_small]
    expected_gains: [departure_truth_not_ready_for_seoharin, boss_resolution_still_unopened]
    outcome_hook:
      add_flags: [boss_recruits_mumyeong_resolved, boss_recruitment_thread_opened, final_boss_resolution_still_unopened]
      add_clues: [departure_truth_not_ready_for_seoharin, boss_resolution_still_unopened]
      destination_id: cheongryu_outer_courtyard
outcome_hooks:
  possible_flags: [boss_recruits_mumyeong_resolved, boss_recruitment_thread_opened, boss_saw_mumyeongs_wound, mumyeong_choice_thread_deepened, black_serpent_recruitment_record_seen, final_boss_resolution_still_unopened]
  possible_clues: [boss_recruited_mumyeong_after_hyeonakmun, recruitment_was_not_salvation, mumyeong_had_nowhere_to_stand_after_destruction, boss_turned_wound_into_power, black_serpent_recruits_wounds_not_names, boss_reads_people_not_forms, departure_truth_not_ready_for_seoharin, boss_resolution_still_unopened]
  possible_destinations: [cheongryu_outer_courtyard]
main_spine_link: 현악문 consequence trace가 남긴 무명의 선택과 보스 first appearance의 weakness-reading hook을 연결하되, 무명 이탈 진실 전달과 보스 최종 결산은 다음 사건으로 넘긴다.
randomization_notes: 1회성 boss recruitment trace. Hyeonakmun consequence 뒤에만 열고 `boss_recruits_mumyeong_resolved`로 반복을 막는다.
promotion_notes: preview runtime으로 구현 완료. `wuxia_mumyeong_destroys_orthodox_sect_followup` docs-only handoff에서 다음 runtime 후보로 선택했고, 보스 스카웃을 구원이나 최종 결산이 아니라 recruitment trace로 제한했다. 무명 이탈 진실 전달, 서하린에게 진실 전달, 무명/보스 최종 결산, epilogue/return, combat resolver/schema, route graph/faction reputation/debt/relation schema, reward/ability schema, 천기록 identity reveal은 열지 않는다.
runtime_preview_implementation_notes:
  implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml
  insert_after: wuxia_mumyeong_destroys_orthodox_sect
  selected_over: [wuxia_mumyeong_departure_truth_summary, wuxia_mumyeong_resolution, wuxia_boss_resolution, wuxia_seoharin_empty_place]
  generated_artifacts:
    - crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json
    - web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json
  default_bundle_changed: false
  new_schema_opened: false
  next_handoff: wuxia_boss_recruits_mumyeong_followup
```

## 24. `wuxia_mumyeong_departure_truth_summary` — preview runtime 구현 완료

```yaml
id: wuxia_mumyeong_departure_truth_summary
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: implemented_in_storypack_preview
source_refs:
  - notion_event:wuxia_mumyeong_departure_truth_summary
  - docs/dev/Notion_Design_Coverage.md
  - docs/dev/Development_Plan.md#050-2026-06-02-docs-only-boss-recruitment-follow-up-handoff-wuxia_mumyeong_departure_truth_summary
  - docs/dev/Development_Plan.md#051-2026-06-02-무협-wuxia_mumyeong_departure_truth_summary-preview-runtime-slice
notion_mapping:
  notion_event_id: wuxia_mumyeong_departure_truth_summary
  notion_event_name: 무명 이탈의 진실 정리
  mapping_status: preview_runtime_implemented
runtime_preview_design_status: implemented
phase: [midgame_backstory]
priority_class: main_branch
location_tags: [sect_courtyard, sealed_truth, mumyeong_past]
surface: [sect_courtyard, cheonggi_record, faction_negotiation]
anomaly_type: [future_record, faction_pressure]
pressure_type: [sanity, relation, danger]
npc_slots: [rival, early_rescuer, blood_moon_antagonist]
candidate_characters: [무명, 서하린, 흑사방주]
summary: 보스 recruitment trace 이후 무명 이탈의 진실을 정리하되, 서하린에게 전달하거나 구원 조건을 만족시키지 않는 sealed summary를 준비한다.
setup_text: 천기록은 무명의 선택을 변명하지 않는다. 다만 빈 현악문, 거절된 도움 요청, 보스가 본 상처, 서하린이 아직 듣지 못한 문장을 한 장의 봉인된 기록으로 묶는다.
presentation:
  visual_id: wuxia_mumyeong_departure_truth_summary
  speaker: 천기록
  layout: sealed_departure_truth_summary
  stable_terms: [무명, 서하린, 현악문, 흑사방주]
conditions:
  locations: [cheongryu_outer_courtyard]
  required_flags: [boss_recruits_mumyeong_resolved, boss_recruitment_thread_opened, mumyeong_destroys_orthodox_sect_resolved, hyeonakmun_destruction_thread_opened, departure_truth_thread_deepened, mumyeong_request_for_aid_resolved, mumyeong_failed_aid_thread_opened, orthodox_hypocrisy_thread_opened, mumyeong_awakening_resolved, midgame_continuity_started]
  forbidden_flags: [mumyeong_departure_truth_summary_resolved]
choice_shapes:
  - id: assemble_departure_truth_without_delivering
    role: safe_trace
    expected_costs: [sanity_small]
    expected_gains: [departure_truth_can_be_understood_but_not_spoken_yet]
    outcome_hook:
      add_flags: [mumyeong_departure_truth_summary_resolved, sealed_departure_truth_summary_prepared, truth_delivery_still_unopened]
      add_clues: [departure_truth_can_be_understood_but_not_spoken_yet]
      destination_id: cheongryu_outer_courtyard
  - id: compare_failed_aid_to_recruitment_offer
    role: moral_read
    expected_costs: [sanity_small, danger_small]
    expected_gains: [boss_used_mumyeongs_wound_after_orthodox_refusal]
    outcome_hook:
      add_flags: [mumyeong_departure_truth_summary_resolved, sealed_departure_truth_summary_prepared, truth_delivery_still_unopened]
      add_clues: [boss_used_mumyeongs_wound_after_orthodox_refusal]
      destination_id: cheongryu_outer_courtyard
  - id: ask_seoharin_what_she_is_ready_to_hear
    role: relation_probe
    expected_costs: [sanity_small]
    expected_gains: [seoharin_truth_delivery_requires_later_consent]
    outcome_hook:
      add_flags: [mumyeong_departure_truth_summary_resolved, sealed_departure_truth_summary_prepared, truth_delivery_still_unopened]
      add_clues: [seoharin_truth_delivery_requires_later_consent]
      destination_id: cheongryu_outer_courtyard
  - id: seal_truth_until_mumyeong_faces_it
    role: safe_stop
    expected_costs: [sanity_small]
    expected_gains: [salvation_condition_seen_but_unmet]
    outcome_hook:
      add_flags: [mumyeong_departure_truth_summary_resolved, sealed_departure_truth_summary_prepared, truth_delivery_still_unopened]
      add_clues: [salvation_condition_seen_but_unmet]
      destination_id: cheongryu_outer_courtyard
outcome_hooks:
  possible_flags: [mumyeong_departure_truth_summary_resolved, sealed_departure_truth_summary_prepared, truth_delivery_still_unopened]
  possible_clues: [departure_truth_can_be_understood_but_not_spoken_yet, seoharin_truth_delivery_requires_later_consent, boss_used_mumyeongs_wound_after_orthodox_refusal, salvation_condition_seen_but_unmet]
  possible_destinations: [cheongryu_outer_courtyard]
main_spine_link: failed-aid records, Hyeonakmun consequence, Mumyeong awakening, and boss recruitment trace를 하나의 sealed truth summary로 묶되, Seo Harin truth delivery와 salvation/final resolution은 다음 단계로 넘긴다.
randomization_notes: 1회성 sealed summary trace. Boss recruitment trace 뒤에만 열고 `mumyeong_departure_truth_summary_resolved`로 반복을 막는다.
promotion_notes: preview runtime으로 구현 완료. `wuxia_boss_recruits_mumyeong_followup` docs-only handoff에서 다음 runtime 후보로 선택했고, 무명 이탈 진실을 서하린에게 전달하거나 구원 조건을 만족시키지 않는 sealed summary로 제한했다. told_seoharin_truth, 무명/보스 결산, final battle, epilogue/return, route/faction/relation/debt/reward schema, 천기록 identity reveal은 열지 않는다.
runtime_preview_implementation_notes:
  implementation_status: implemented
  implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml
  insert_after: wuxia_boss_recruits_mumyeong
  selected_over: [wuxia_mumyeong_resolution, wuxia_boss_resolution, wuxia_seoharin_empty_place]
  generated_artifacts:
    - crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json
    - web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json
  default_bundle_changed: false
  new_schema_opened: false
  next_handoff: wuxia_mumyeong_departure_truth_summary_followup
```

## 25. `wuxia_seoharin_empty_place` — preview runtime 구현 완료

```yaml
id: wuxia_seoharin_empty_place
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: implemented_in_storypack_preview
source_refs:
  - notion_event:wuxia_seoharin_empty_place
  - notion_page:사도 최종전 2페이즈: 약점 장악
  - notion_page:사도 최종전 3페이즈: 계산식 밖
  - notion_page:최종장 결산 라우팅 마스터
  - docs/dev/Development_Plan.md#052-2026-06-02-docs-only-departure-truth-summary-follow-up-handoff-wuxia_seoharin_empty_place
notion_mapping:
  notion_event_id: wuxia_seoharin_empty_place
  notion_event_name: 비워둔 자리
  mapping_status: preview_runtime_implemented
runtime_preview_design_status: implemented
implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml
phase: [midgame_backstory, seoharin_empty_place_bridge]
priority_class: npc_relation
location_tags: [cheongryu_outer_courtyard, empty_place, seoharin_axis]
surface: [sect_courtyard, training_chore, cheonggi_record]
anomaly_type: [future_record, oath_binding]
pressure_type: [sanity, relation]
npc_slots: [early_rescuer]
candidate_characters: [서하린, 무명]
summary: sealed departure truth summary 이후 청류문 수련장 한쪽의 비워둔 자리를 다시 보고, 서하린의 기다림을 소유가 아니라 돌아올 자리로 이해하는 late emotional bridge.
setup_text: 청류문 마당 한쪽의 먼지 없는 빈 자리에는 목검 하나가 가지런히 놓여 있다. 천기록은 그 자리가 아직 이름을 잃지 않았다고 적지만, 서하린은 여전히 누구를 붙잡으려 하지 않는다.
presentation:
  visual_id: wuxia_seoharin_empty_place
  speaker: 서하린
  layout: empty_place_memory
  stable_terms: [서하린, 무명, 청류문, 목검]
conditions:
  locations: [cheongryu_outer_courtyard]
  required_flags: [mumyeong_departure_truth_summary_resolved, sealed_departure_truth_summary_prepared, truth_delivery_still_unopened, midgame_continuity_started]
  forbidden_flags: [seoharin_empty_place_resolved]
choice_shapes:
  - id: ask_who_kept_the_empty_place
    role: relation_probe
    expected_costs: [sanity_small]
    expected_gains: [seoharin_remembers_without_possessing]
    outcome_hook:
      add_flags: [seoharin_empty_place_resolved, seoharin_axis_opened, empty_place_remembered, truth_delivery_still_unopened]
      add_clues: [seoharin_remembers_without_possessing]
      destination_id: cheongryu_outer_courtyard
  - id: leave_the_place_unclaimed
    role: safe_observe
    expected_costs: []
    expected_gains: [empty_place_is_return_not_claim]
    outcome_hook:
      add_flags: [seoharin_empty_place_resolved, seoharin_axis_opened, empty_place_remembered, truth_delivery_still_unopened]
      add_clues: [empty_place_is_return_not_claim]
      destination_id: cheongryu_outer_courtyard
  - id: set_down_the_work_notebook_briefly
    role: cheonggi_record_probe
    expected_costs: [sanity_small]
    expected_gains: [unpriced_wooden_sword_condition_seeded]
    outcome_hook:
      add_flags: [seoharin_empty_place_resolved, seoharin_axis_opened, empty_place_remembered, truth_delivery_still_unopened]
      add_clues: [unpriced_wooden_sword_condition_seeded]
      destination_id: cheongryu_outer_courtyard
  - id: step_back_without_naming_mumyeong
    role: safe_stop
    expected_costs: [sanity_small]
    expected_gains: [mumyeong_place_still_unclaimed]
    outcome_hook:
      add_flags: [seoharin_empty_place_resolved, seoharin_axis_opened, empty_place_remembered, truth_delivery_still_unopened]
      add_clues: [mumyeong_place_still_unclaimed]
      destination_id: cheongryu_outer_courtyard
outcome_hooks:
  possible_flags: [seoharin_empty_place_resolved, seoharin_axis_opened, empty_place_remembered, truth_delivery_still_unopened]
  possible_clues: [seoharin_remembers_without_possessing, empty_place_is_return_not_claim, mumyeong_place_still_unclaimed, unpriced_wooden_sword_condition_seeded, truth_delivery_still_requires_consent]
  possible_items: []
  possible_destinations: [cheongryu_outer_courtyard]
main_spine_link: sealed departure truth summary가 남긴 무명/서하린 진실 경계를 서하린의 소속감 축으로 넘긴다. 최종전 2/3페이즈의 remembered_empty_place와 seoharin_axis 조건을 준비하되, truth delivery나 final routing은 열지 않는다.
randomization_notes: 1회성 relation bridge. sealed summary 뒤에만 열고 `seoharin_empty_place_resolved`로 반복을 막는다.
promotion_notes: preview runtime으로 구현 완료. `wuxia_mumyeong_departure_truth_summary_followup` docs-only handoff에서 다음 runtime 후보로 선택했다. Notion 원본은 초반 companion card지만, 최신 사도 최종전 2/3페이즈와 최종장 결산 라우팅 마스터가 비워둔 자리 확인을 seoharin_axis/high 및 remembered_empty_place 조건으로 재사용하므로 sealed truth summary 이후 late bridge로 재해석한다. told_seoharin_truth, 서하린 truth delivery, 무명 구원 확정, 무명/보스 결산, 사도 최종전, item_unpriced_wooden_sword 지급, final/epilogue/return schema, route/faction/relation/debt/reward schema, combat resolver/schema, 천기록 identity reveal은 열지 않는다.
runtime_preview_handoff:
  handoff_status: implemented
  insert_after: wuxia_mumyeong_departure_truth_summary
  selected_over: [wuxia_mumyeong_resolution, wuxia_boss_resolution, wuxia_sado_final_battle, wuxia_sado_final_phase_2_weakpoint_control, wuxia_sado_final_phase_3_outside_calculation]
  next_runtime_scope: empty_place_memory_bridge
  default_bundle_changed: false
  new_schema_opened: false
runtime_preview_implementation:
  implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml
  insert_after: wuxia_mumyeong_departure_truth_summary
  generated_artifacts:
    - crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json
    - web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json
  next_handoff: wuxia_seoharin_empty_place_followup
```

## 26. `wuxia_seoharin_left_meal` — preview runtime 구현 완료

```yaml
id: wuxia_seoharin_left_meal
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: implemented_in_storypack_preview
source_refs:
  - notion_event:wuxia_seoharin_left_meal
  - notion_page:남겨둔 밥
  - notion_page:가지 말라는 말
  - notion_page:무명 결산
  - notion_page:사도 최종전 3페이즈: 계산식 밖
  - docs/dev/Development_Plan.md#054-2026-06-02-docs-only-seoharin-empty-place-follow-up-handoff-wuxia_seoharin_left_meal
notion_mapping:
  notion_event_id: wuxia_seoharin_left_meal
  notion_event_name: 남겨둔 밥
  mapping_status: preview_runtime_implemented
runtime_preview_design_status: implemented
implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml
phase: [midgame_backstory, seoharin_belonging_bridge]
priority_class: npc_relation
location_tags: [cheongryu_outer_courtyard, kitchen, seoharin_axis]
surface: [sect_courtyard, food, daily_care]
anomaly_type: [future_record, oath_binding]
pressure_type: [sanity, relation]
npc_slots: [early_rescuer]
candidate_characters: [서하린]
summary: 비워둔 자리 이후, 늦게 돌아온 주인공에게 남겨진 식은 밥 한 그릇을 통해 서하린의 소속감 표현을 말이 아니라 생활로 확인하는 relation bridge.
setup_text: 식사는 끝났지만 청류문 부엌 근처에는 덮어 둔 밥 한 그릇이 남아 있다. 서하린은 붙잡는 말 대신 "네 몫도 있다. 먹어."라고만 말한다.
presentation:
  visual_id: wuxia_seoharin_left_meal
  speaker: 서하린
  layout: left_meal_memory
  stable_terms: [서하린, 밥그릇, 청류문, 귀환]
conditions:
  locations: [cheongryu_outer_courtyard]
  required_flags: [seoharin_empty_place_resolved, seoharin_axis_opened, empty_place_remembered, truth_delivery_still_unopened, midgame_continuity_started]
  forbidden_flags: [seoharin_left_meal_resolved]
choice_shapes:
  - id: eat_the_left_meal_quietly
    role: relation_accept
    expected_costs: []
    expected_gains: [left_meal_was_kept_for_return, belonging_is_daily_care]
  - id: thank_seoharin_for_the_bowl
    role: relation_probe
    expected_costs: [sanity_small]
    expected_gains: [seoharin_care_named_without_claim]
  - id: joke_about_who_ordered_extra_rice
    role: soft_deflect
    expected_costs: [sanity_small]
    expected_gains: [seoharin_deflects_care_with_plain_words]
  - id: pass_without_eating_the_meal
    role: safe_stop
    expected_costs: [sanity_small]
    expected_gains: [last_bowl_epilogue_seeded, belonging_can_be_refused]
outcome_hooks:
  possible_flags: [seoharin_left_meal_resolved, seoharin_axis_deepened, qingliu_belonging_warmed, seoharin_axis_still_open, left_meal_left_untouched, truth_delivery_still_unopened]
  possible_clues: [left_meal_was_kept_for_return, belonging_is_daily_care, seoharin_care_named_without_claim, seoharin_deflects_care_with_plain_words, last_bowl_epilogue_seeded, belonging_can_be_refused]
  possible_items: []
  possible_destinations: [cheongryu_outer_courtyard]
main_spine_link: empty-place bridge가 연 서하린 관계축을 청류문 소속감으로 낮게 이어 붙인다. 귀환/정착/침식 최종 분기나 relation schema는 열지 않고, `가지 말라는 말`과 후일담 카드의 조건 씨앗만 남긴다.
randomization_notes: 1회성 relation bridge. `seoharin_axis_opened` 이후에만 열고 `seoharin_left_meal_resolved`로 반복을 막는다.
promotion_notes: preview runtime으로 구현 완료. `wuxia_seoharin_empty_place_followup` handoff에서 `wuxia_seoharin_left_meal`을 다음 runtime 후보로 선택했다. Notion 원문은 "서하린 관계축 최소 개방"과 "청류문 소속감 상승"을 선행/보상으로 두므로, `seoharin_empty_place_resolved` 뒤에 schema-less daily-care bridge로 landing한다. `wuxia_seoharin_unsaid_stay`는 귀환/정착/침식 최종 관계 분기라 보류하고, `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, `wuxia_sado_final_battle` 계열은 final/epilogue/combat/reward schema를 요구하므로 보류한다.
runtime_preview_handoff:
  handoff_status: implemented
  insert_after: wuxia_seoharin_empty_place
  selected_over: [wuxia_seoharin_unsaid_stay, wuxia_mumyeong_resolution, wuxia_boss_resolution, wuxia_sado_final_battle, wuxia_sado_final_phase_2_weakpoint_control, wuxia_sado_final_phase_3_outside_calculation]
  next_runtime_scope: seoharin_daily_care_bridge
  default_bundle_changed: false
  new_schema_opened: false
runtime_preview_implementation:
  implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml
  insert_after: wuxia_seoharin_empty_place
  generated_artifacts:
    - crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json
    - web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json
  next_handoff: wuxia_seoharin_left_meal_followup
```

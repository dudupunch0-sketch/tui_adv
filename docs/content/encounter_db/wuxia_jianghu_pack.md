# 이구학지 — 천기록 encounter situation cards

Status: candidate

이 문서는 `docs/content/storypacks/wuxia_jianghu_pack.md`의 후보 인카운터를 runtime YAML 승격 전 상황 카드로 정리한다. `wuxia_commute_rift_arrival`, `wuxia_heuksa_bang_first_fight`, `wuxia_cheonggi_record_first_fragment`는 이 카드에서 separate storypack preview runtime으로 승격된 첫 세 slice이며, 나머지 카드는 아직 후보 상태다.

공통 원칙:

- 모든 카드는 `world_id: wuxia_jianghu`, `storypack_id: wuxia_jianghu_pack`에 속한다.
- 현재 단계에서는 대부분 runtime encounter가 아니다. 단, `wuxia_commute_rift_arrival`, `wuxia_heuksa_bang_first_fight`, `wuxia_cheonggi_record_first_fragment`는 `src/tui_adv/storypack-previews/wuxia_jianghu_pack/`의 preview source와 별도 generated preview bundle에 반영됐다.
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
| `wuxia_seoharin_left_meal` | 남겨둔 밥 | none yet | future 서하린 companion event |
| `wuxia_seoharin_empty_place` | 비워둔 자리 | none yet | future 서하린/무명 clue event |
| `wuxia_mumyeong_departure_truth_summary` | 무명 이탈의 진실 정리 | none yet | future 무명 truth event |
| `wuxia_black_serpent_pressures_qingliu` | 흑사방의 청류문 압박 | partial: `wuxia_cheongryu_raid_route_split` background | future pressure/side event |
| `wuxia_mumyeong_copy_style_reveal` | 무명의 카피 무공 공개 | none yet | future rival/growth event |
| `wuxia_mumyeong_resolution` | 무명 결산 | none yet | future final route event |
| `wuxia_mumyeong_midgame_reunion` | 무명 중반 재회 | none yet | future rival event |
| `wuxia_boss_resolution` | 보스 결산 | none yet | future boss result event |
| `wuxia_mumyeong_first_sighting` | 무명 첫 목격 | none yet | future reveal event |
| `wuxia_mumyeong_first_confrontation` | 무명 첫 대치 | none yet | future rival confrontation |
| `wuxia_boss_first_appearance` | 보스 첫 등장 | none yet | future boss-wall event |
| `wuxia_mumyeong_destroys_orthodox_sect` | 정파 문파 멸문 | none yet | future consequence/backstory event |
| `wuxia_mumyeong_awakening` | 무명의 각성 | none yet | future rival corruption/growth event |
| `wuxia_boss_recruits_mumyeong` | 흑사방 보스의 스카웃 | none yet | future backstory event |
| `wuxia_mumyeong_reads_orthodox_style` | 무명의 정파 무공 간파 | none yet | future 청류안 contrast event |
| `wuxia_qingliu_attack_after_war` | 무너져가는 청류문 습격 | none yet | future backstory/pressure event |
| `wuxia_mumyeong_request_for_aid` | 무명의 도움 요청 | none yet | future backstory event |
| `wuxia_tianjilu_first_fragment` | 천기록 첫 천외편린 | `wuxia_cheonggi_record_first_fragment` | preview implemented as schema-less foreshadow; full reward schema future |
| `wuxia_seoharin_intervention` | 서하린의 개입 | `wuxia_seo_harin_rescue` | next runtime slice; docs/handoff ready, not implemented |
| `wuxia_prologue_commute_rift` | 출근길의 균열 | `wuxia_commute_rift_arrival` | preview implemented |
| `wuxia_qingliu_apprentice_entry` | 청류문 임시 수습생 등록 | `wuxia_cheongryu_apprentice_entry` | follow-up docs/handoff ready, not implemented |
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
  mapping_status: direct_next_runtime_slice
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
promotion_notes: 다음 runtime slice로 확정한다. healing/debt/relation은 새 schema 없이 health/sanity/danger, flags, clues, destination, log로만 표현하고, 기본 office bundle/`escape-office` save key/천외편린 3택 성장 schema는 건드리지 않는다.
```

## 4. `wuxia_cheongryu_apprentice_entry`

```yaml
id: wuxia_cheongryu_apprentice_entry
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: candidate
runtime_preview_design_status: designed_follow_up_not_implemented
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
promotion_notes: follow-up runtime candidate로 설계 완료. `wuxia_seo_harin_rescue` 구현 전에는 승격하지 않는다. 첫 runtime에서는 location/state schema를 넓히지 않고 narrative outcome과 flags/clues/log/presentation으로만 표현한다.
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

## 6. `wuxia_cheongryu_raid_route_split`

```yaml
id: wuxia_cheongryu_raid_route_split
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: candidate
runtime_preview_design_status: designed_later_not_implemented
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
promotion_notes: later runtime candidate로 설계 완료. `wuxia_seo_harin_rescue`와 `wuxia_cheongryu_apprentice_entry` 구현 전에는 승격하지 않는다. 첫 raid runtime은 route flag/clue/log/presentation만 남기고 route graph, faction reputation, boss combat, ending implementation은 별도 slice로 둔다.
```

## 7. `wuxia_cheongryu_raid_wounded_fallback`

```yaml
id: wuxia_cheongryu_raid_wounded_fallback
world_id: wuxia_jianghu
storypack_id: wuxia_jianghu_pack
status: candidate
runtime_preview_design_status: designed_later_not_implemented
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
  prereq: rescue/apprentice/raid split runtime slice가 먼저 landing되어야 한다.
  location: raid_aftermath_shelter
  fallback_location_if_no_new_location: cheongryu_outer_courtyard
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
promotion_notes: later runtime candidate로 설계 완료. `wuxia_cheongryu_raid_route_split` 구현 전에는 승격하지 않는다. 첫 runtime은 flags/clues/log/presentation만 남기고 route graph, faction reputation, triage system, companion death, boss combat, ending implementation은 별도 slice로 둔다.
```

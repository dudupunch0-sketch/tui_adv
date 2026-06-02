# 스토리팩 / 인카운터 DB 설계

Status: 설계 문서

이 문서는 `tui_adv`의 스토리팩 후보와 랜덤 인카운터 상황을 DB처럼 축적하고, 나중에 검증된 일부만 런타임 콘텐츠로 승격하기 위한 설계 기준이다. 현재 Web/terminal default storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**이며, `escape from the office` office isolation content는 legacy/parity storypack으로 유지한다. 이 DB는 office가 아닌 세계관도 같은 엔진 계약으로 다루기 위해 확장한다.

핵심 방향은 다음이다.

```text
굵은 메인 스토리 spine
  + storypack별 encounter deck
  + 6스탯 기반 NPC profile
  + public-safe 검토와 승격 규칙
```

2026-05-29 / 2026-05-31 갱신:

- Storypack은 이제 office 내부 변주만이 아니라 `world_id`를 가진 세계관 단위 후보를 포함한다.
- 기본 world/storypack은 `wuxia_jianghu` / `wuxia_jianghu_pack`이다. `office_apocalypse` / `isolation_pack` 계열은 legacy/parity runtime 후보로 유지한다.
- 추가 office-family 후보는 `office_dream` / `yageunmong_pack`이다. 이 후보는 회사 자각몽/악몽/각성편린/퇴근 게이트 premise를 보존하되, 기본 office runtime을 대체하지 않는다.
- 첫 비-office 기준팩은 `wuxia_jianghu` / `wuxia_jianghu_pack`이며, 최신 canonical story는 **이구학지 — 천기록**이다. 이전 generic 무협 placeholder는 superseded로 본다.
- 2026-06-01 Notion live check 이후 `이구학지` parent page는 synopsis/초기 기획이고, 하위 관리 문서와 `09. 이구학지 사건 카드 DB` / `10. 이구학지 후일담 카드 DB`가 최신 세부 운영 기준이다. Repo DB와 runtime 승격은 Notion DB row를 직접 가져오지 않고, `docs/dev/Notion_Design_Coverage.md`의 mapping과 canonical content docs를 먼저 통과한다.
- 새 기능은 가능하면 office isolation, office dream, wuxia surface에서 설명 가능한 engine-neutral 형태로 설계한다. 자세한 기준은 `docs/design/Storypack_World_Model.md`를 따른다.

이 문서는 게임 엔진 구현 계획이 아니다. `src/tui_adv/data/encounters.yaml`이나 Rust GameCore에 바로 새 규칙을 추가하기 전에, 어떤 스토리팩/상황/NPC를 만들고 어떻게 검토할지 정하는 콘텐츠 설계 문서다.

## 1. 목적

스토리팩과 인카운터 DB의 목적은 다음이다.

1. 기존 idea_box의 큰 스토리팩 아이디어를 작은 인카운터 상황 카드로 쪼갠다.
2. 랜덤 인카운터가 메인 스토리를 흐리지 않고 보강하게 한다.
3. 등장인물을 6스탯 기반으로 일관되게 만들고, 인카운터 기능과 연결한다.
4. 후보 콘텐츠와 런타임 확정 콘텐츠를 분리한다.
5. public-safe 검토를 통과한 카드만 나중에 YAML/content bundle로 승격한다.

## 2. 비목표

이 문서는 다음을 하지 않는다.

- Rust core, Python engine, Web Storybook, terminal renderer 구현 변경
- 런타임에서 LLM이 즉석으로 인카운터를 생성하는 시스템 설계
- 기존 플레이어 능력치와 NPC 6스탯의 즉시 통합
- 모든 스토리팩 후보를 한 번에 구현 대상으로 확정
- 실제 회사명, 실제 직원, 실제 내부망 주소, 실제 사무실 구조, 현실 힌트 최종 위치 기록

## 3. 기본 구조

### 3.1 메인 스토리 spine

게임은 완전 랜덤 이야기가 아니라 하나의 굵은 메인 스토리를 가진다. 랜덤 인카운터는 이 spine을 대체하지 않고, 플레이어가 같은 큰 사건을 다른 경로와 세부 상황으로 경험하게 만든다.

메인 spine phase 초안:

| phase | 의미 | 인카운터 역할 |
|---|---|---|
| `opening_absence` | 사람들이 사라지고 회사 시스템만 남음 | 부재 확인, 첫 연락, 첫 단서 |
| `normal_operations` | 회사는 비정상 상황을 정상 업무처럼 처리함 | 블랙코미디, 자동 메일, 결재, 회의록 |
| `isolation_rules` | 격리/반복/기록 오류 규칙이 드러남 | 패턴 발견, 위험 선택, 루트 힌트 |
| `route_commitment` | 탈출/진실/정복/히든 중 방향이 생김 | 루트별 핵심 플래그/단서 |
| `resolution_pressure` | 결말 직전 압박 | 선택의 대가, NPC 운명, 엔딩 조건 |

모든 storypack record와 encounter situation card는 최소 하나의 phase에 연결되어야 한다.
단, phase 이름은 world-specific 확장을 허용한다. legacy office storypack은 `opening_absence` 같은 기존 phase를 쓰고, 야근몽은 `late_night_sleep`, `lucid_dream_awareness`, `reality_anchor_collection`, `clockout_gate_refusal` 같은 office-dream phase를 쓸 수 있다. 이구학지 무협팩은 `commute_rift`, `market_arrival`, `first_brawl`, `cheongryu_apprenticeship`, `cheonggi_record_awakening`처럼 storypack-specific phase를 쓸 수 있다.

### 3.2 Deck 계층

런타임 승격 시에는 다음 우선순위를 따른다.

1. `main_forced`: 반드시 발생해야 하는 메인 spine beat
2. `route_key`: 현재 루트의 핵심 진행 beat
3. `npc_relation`: NPC 관계/동행/배신/구출 beat
4. `random_pack`: 활성 storypack의 랜덤 카드
5. `generic_pressure`: 생존 자원/압박/분위기 카드
6. `ambient`: 이동을 막지 않는 짧은 배경 변화 또는 무사건

DB 단계에서 이 우선순위를 기록하는 이유는, 나중에 인카운터-first turn loop에서 랜덤 카드가 이동이나 핵심 루트를 막지 않게 하기 위해서다.

## 4. Canonical taxonomy

새 카드는 가능하면 아래 분류를 사용한다. 새 라벨이 필요하면 짧은 정의를 추가한다.

### 4.1 `surface`

TUI/fake-terminal에서 사건이 드러나는 표면이다.

| surface | 의미 |
|---|---|
| `messenger` | 사내 메신저, 임시 채널, DM |
| `intranet` | 사내 포털, FAQ, 공지 검색 |
| `meeting_minutes` | 자동 회의록, 발언 로그, 참석자 기록 |
| `reservation_panel` | 회의실/시설 예약 화면 |
| `cctv` | 보안실 모니터, 지연 화면, 출입 기록 |
| `document_viewer` | PDF, 사양서, 문서관리 시스템 |
| `approval_system` | 결재선, 승인/반려, 문서번호 |
| `organization_chart` | 조직도, 인사기록, 사원 조회 |
| `build_log` | CI, 빌드팜, 테스트 대시보드 |
| `payroll_sheet` | 급여명세서, 연봉 테이블, 보상 모델 |
| `security_gate` | 출입 게이트, 사원증, 방문증 |
| `office_object` | 복합기, 정수기, 커피머신, 화이트보드 등 사물 |
| `commute_rift` | 출근길의 엘리베이터/지하철/횡단보도 같은 경계에서 발생하는 world transition |
| `market_street` | 무협 세계 첫 도착 시장, 군중 시선, 흑사방 시비, 거리 난투 |
| `office_items` | 정장, 구두, 사원증, 지갑, 볼펜, 업무수첩 같은 현대 소지품 |
| `sect_courtyard` | 청류문 수습생 구간의 거점, 보호/채무/수련 허가 surface |
| `training_chore` | 장작 패기, 물 긷기, 연무장 청소, 서고 정리 같은 잡일-수련 surface |
| `cheonggi_record` | 업무수첩과 연결된 천기록, 검색창이 아닌 기록/기연 surface |
| `fragment_choice` | 천외편린 3택 보상/수련 방향 선택 surface |
| `sect_raid` | 청류문 습격 같은 route commitment 대형 사건 surface |
| `faction_negotiation` | 백도맹/흑천련/천기각의 명분과 대가가 충돌하는 협상 surface |

### 4.2 `anomaly_type`

| anomaly_type | 의미 |
|---|---|
| `mismatched_floor` | 층수, 위치, 날씨, 동선이 서로 맞지 않음 |
| `delayed_time` | 화면/기록/소리가 현실보다 한 박자 늦거나 빠름 |
| `future_record` | 아직 하지 않은 선택이나 미래 결과가 기록됨 |
| `absent_people` | 사람은 없지만 업무 흔적과 시스템 처리가 남음 |
| `identity_erasure` | 인물의 이름, 소속, 권한, 사원번호가 지워짐 |
| `document_contamination` | 문서를 읽거나 수정하면 현실 인식이 바뀜 |
| `space_loop` | 계단, 복도, 회의실, 엘리베이터가 반복됨 |
| `permission_denied_as_existence` | 권한 오류가 존재 판정처럼 작동함 |
| `worldline_branch` | 같은 사건의 다른 결과/브랜치가 동시에 존재함 |
| `world_displacement` | 기존 세계에서 다른 세계관으로 이동함 |
| `workplace_memory_mismatch` | 회사원 기억과 새 세계의 규칙이 충돌함 |
| `outsider_without_sect` | 소속 문파가 없는 외지인으로 판정됨 |
| `first_brawl_defeat` | 첫 무림 난투가 대부분 패배/부상/구조 hook으로 귀결됨 |
| `notebook_oracle` | 업무수첩이 천기록과 연결되어 임의의 현대지식 조각을 보여줌 |
| `fragment_choice` | 세 후보 중 하나만 남기는 천외편린 선택 압박 |
| `sect_debt` | 보호, 치료비, 숙식비, 수련 허가가 채무/의무로 묶임 |
| `faction_pressure` | 정파/사파/기록 세력의 명분과 대가가 route commitment를 압박함 |
| `qi_deviation` | 내공/기혈/심마 압박이 상태 이상으로 드러남 |
| `oath_binding` | 맹세, 문파 규칙, 결투 약속이 강제력처럼 작동함 |

### 4.3 `pressure_type`

| pressure_type | 의미 |
|---|---|
| `health` | 물리적 위험, 부상, 추격, 이동 부담 |
| `sanity` | 공포, 기억/기록 오염, 선택지 신뢰성 |
| `battery` | 사내망, 녹음/촬영, 로그 추적, 조명 비용 |
| `hunger` | 장기 생존 압박, 회복 저하 |
| `thirst` | 탈수, 환각, 정수기/탕비실 이벤트 |
| `danger` | 회사/구역 위험도 상승 |
| `relation` | NPC 신뢰, 의심, 빚, 배신, 동행 여부 |

### 4.4 `npc_slot`

| npc_slot | 기능 |
|---|---|
| `infra_interpreter` | 사내망, 서버 로그, 권한, CI/빌드 시스템을 해석함 |
| `hr_identity_keeper` | 조직도, 근태, 인사기록, 존재 말소를 다룸 |
| `security_witness` | CCTV, 출입통제, 봉쇄, 물리 안전을 다룸 |
| `facility_pathfinder` | 회의실, 창고, 비상계단, 비공식 동선을 앎 |
| `finance_record_reader` | 비용, 급여, 보상, 예산 기록을 읽음 |
| `pm_worldline_mediator` | 실패한 프로젝트, 일정, 의사결정 세계선을 연결함 |
| `cleaning_unofficial_witness` | 야간 사무실, 뒷문, 직원들이 모르는 공간을 봄 |
| `newcomer_mirror` | 플레이어의 혼란을 반사하고 기본 설명을 유도함 |
| `early_rescuer` | 흑사방 첫 전투 후 개입하는 구조자/멘토 후보 |
| `sect_master_guardian` | 보호, 채무, 수습생 조건, 수련 허가를 관리함 |
| `archive_keeper` | 폐서고, 몰락한 문파의 기록, 선택의 대가를 암시함 |
| `righteous_ally` | 백도맹/정파 루트의 명분과 정치 압박을 제공함 |
| `sapa_ally` | 흑천련/사파 루트의 생존, 거래, 암투 감각을 제공함 |
| `cheonggi_record_keeper` | 천기각/천기록/귀환 단서를 해석함 |
| `blood_moon_antagonist` | 혈월교의 급진 명분과 메인 갈등을 대표함 |

## 5. Storypack DB schema

스토리팩 record는 큰 후보 세계/에피소드 묶음을 정의한다.

```yaml
id: isolation_pack
world_id: office_apocalypse
status: candidate
source_refs:
  - idea_box/done/2026-05-22-general-corporate-storypacks.md
  - idea_box/done/2026-05-22-semiconductor-sw-storypacks.md
name: 차원격리팩
one_line: 퇴근 직전 회사 일부 공간이 격리되고, 사내 시스템으로 다른 격리자들과 연결된다.
main_surfaces: [messenger, intranet, cctv, meeting_minutes, organization_chart]
anomaly_types: [mismatched_floor, delayed_time, absent_people, worldline_branch]
main_phases: [opening_absence, normal_operations, isolation_rules, route_commitment]
sensitive_topics: [real_company_similarity]
reusable_npc_slots: [infra_interpreter, hr_identity_keeper, security_witness]
ending_candidates: [escape_alone, rescue_channel_members, close_failed_worldlines, merge_isolation_zones]
main_spine_support: 사라진 사람과 남은 시스템이라는 핵심 미스터리를 확장한다.
notes: 공개 콘텐츠에서는 실제 회사명/부서명/개인명을 쓰지 않는다.
```

### 5.1 status

| status | 의미 |
|---|---|
| `raw` | 아이디어에서 막 추출한 원재료 |
| `candidate` | DB 형식으로 정리된 후보 |
| `curated` | 톤/안전/구조 검토 통과 |
| `promoted` | 런타임 YAML/content bundle로 승격됨 |
| `merged` | 다른 카드나 팩에 흡수됨 |
| `rejected` | 톤/안전/중복/범위 문제로 폐기됨 |

## 6. Encounter Situation DB schema

Encounter Situation DB는 runtime encounter가 아니라 “상황 카드” DB다. 이 단계에서는 아직 정확한 `encounters.yaml` 문장이나 수치가 확정되지 않아도 된다.

```yaml
id: isolation_channel_mismatched_floor
world_id: office_apocalypse
status: candidate
storypack_id: isolation_pack
phase: opening_absence
priority_class: random_pack
location_tags: [office, messenger]
surface: messenger
anomaly_type: mismatched_floor
pressure_type: [sanity, battery]
npc_slots: [infra_interpreter]
candidate_characters: [park_doyoon]
summary: 격리자 임시대응방에서 같은 회사에 있다는 사람들이 서로 다른 층수와 날씨를 보고한다.
setup_text: 사내 메신저에 임시 대응방 초대가 뜬다. 참가자들은 모두 같은 시간에 접속했지만 층수 표시가 서로 다르다.
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
    expected_gains: [route_flag]
outcome_hooks:
  possible_flags: [isolation_channel_joined, mismatched_floor_reported]
  possible_clues: [channel_floor_mismatch, delayed_channel_route]
  possible_items: []
main_spine_link: 사람들이 사라졌지만 사내망에는 다른 격리자들이 남아 있다는 사실을 보여준다.
randomization_notes: opening_absence 이후 1회성. 반복 등장 금지.
promotion_notes: runtime 승격 시 messenger UI presentation metadata를 붙인다.
```

필수 기준:

- 하나의 카드는 하나의 명확한 상황만 다룬다.
- 선택지는 세부 문장보다 역할을 먼저 정의한다.
- 최소 하나의 `safe_observe`, `leave`, `ignore`, `wait` 같은 fallback 선택지가 있어야 한다.
- 최소 하나의 game-state hook이 있어야 한다: flag, clue, item, relation, resource, route hint 중 하나.
- `main_spine_link`가 비어 있으면 랜덤 잡음으로 간주하고 승격하지 않는다.
- hub 위치에서 항상 eligible하게 만들 가능성이 있으면 `randomization_notes`에 차단 위험을 적는다.

### 6.1 Machine-readable 검증 DB

사람용 후보 문서는 설명/톤/해설을 보존하고, 참조 무결성 검사는 별도 JSON mirror에서 수행한다.

- `docs/content/storypack_db/storypacks.json`: `StorypackRecord` 후보 목록. 현재 `isolation_pack`, `yageunmong_pack`, `wuxia_jianghu_pack`를 포함한다.
- `docs/content/storypack_db/encounter_situations.json`: `EncounterSituationCard` 후보 목록. 현재 `isolation_pack` 6개, `yageunmong_pack` 12개, `wuxia_jianghu_pack` 28개, 총 46개 repo 후보 카드를 포함한다. Notion 사건 카드 DB 26개 row는 upstream design source이며, 이 mirror에 자동으로 전부 추가하지 않는다.
- `src/tui_adv/game/storypack_db.py`: `load_storypack_db(root)`와 `validate_storypack_db(root)`를 제공한다.
- `tests/test_storypack_db.py`: office isolation / office dream / wuxia 후보 카드가 같은 DB에서 로드되고, `storypack_id`/`world_id`/taxonomy/fallback/outcome hook 계약을 검증한다.

현재 JSON DB는 런타임 콘텐츠가 아니다. `src/tui_adv/data/encounters.yaml`, Rust content bundle, Web generated data에 자동으로 섞지 않는다. 목적은 다음 runtime slice 전에 office isolation / office dream / wuxia 후보 카드가 최소한의 구조 계약을 만족하는지 확인하는 것이다.

2026-05-31 후속 결정:

- `wuxia_commute_rift_arrival`, `wuxia_heuksa_bang_first_fight`, `wuxia_cheonggi_record_first_fragment`, `wuxia_seo_harin_rescue`, `wuxia_cheongryu_apprentice_entry`, `wuxia_cheongryu_chore_sparring`, `wuxia_cheongryu_raid_route_split`, `wuxia_cheongryu_raid_wounded_fallback`, `wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price` preview는 완료했다.
- `wuxia_heavenly_archive_previous_outsiders`, `wuxia_wounded_shelter_dawn_offers`, `wuxia_mumyeong_first_sighting`, `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_reads_orthodox_style`, `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance` preview도 완료했다.
- 이 preview runtime content는 `storypack_preview` bundle에만 들어가며, 기본 office runtime과 `src/tui_adv/data/*.yaml`에는 직접 섞지 않는다.
- `preview launcher/UI wiring`은 explicit opt-in entrypoint로 구현했다. 후속 content slice에서 다시 열지 않는다.
- 첫 정파 route opener `wuxia_baekdo_medicine_debt`는 `righteous_route_started` + `cheongryu_rebuild_thread`만 eligibility로 요구하고, direct `baekdo_alliance_debt`와 deferred `baekdo_medicine_debt`는 flavor hook으로만 사용해 구현 완료했다. 첫 사파 route opener `wuxia_black_heaven_escape_price`도 `sapa_route_started` + `dowol_debt`만 eligibility로 요구하고, direct `black_heaven_deal_marked`와 deferred `black_heaven_escape_marker`는 flavor hook으로만 사용해 구현 완료했다. 첫 천기·귀환 route opener `wuxia_heavenly_archive_previous_outsiders`도 `cheonggi_return_route_started` + `cheonggi_record_targeted`만 eligibility로 요구하고, direct `heavenly_archive_contact`와 deferred `heavenly_archive_triage_map_seen`는 flavor hook으로만 사용해 구현 완료했다. Deferred-offer card `wuxia_wounded_shelter_dawn_offers`도 `cheongryu_raid_wounded_fallback_resolved` + `route_commitment_deferred` + `deferred_route_reopened` + `wounded_shelter_stabilized`만 eligibility로 요구하고, `survivor_roll_call_complete`와 `route_delay_cost_recorded`는 flavor hook으로만 사용해 구현 완료했다. Common midgame bridge `wuxia_mumyeong_first_sighting`는 `route_midgame_continuity_after_wounded_shelter` handoff 결과로 세 route opener outcome의 `route_opener_resolved`를 fan-in flag로 사용해 구현 완료했다. Rival confrontation `wuxia_mumyeong_first_confrontation`는 `wuxia_mumyeong_first_confrontation_after_sighting` handoff 결과로 구현 완료했다.
- `wuxia_mumyeong_followup_after_first_confrontation` handoff는 다음 runtime 후보를 `wuxia_mumyeong_copy_style_reveal`로 결정했고, 카피 무공 공개는 기존 encounter schema로 구현 완료했다.
- `wuxia_mumyeong_followup_after_copy_style_reveal` handoff는 다음 runtime 후보를 `wuxia_mumyeong_reads_orthodox_style`로 결정했고, 현악문/복호금쇄수 clue는 기존 encounter schema로 구현 완료했다.
- `wuxia_mumyeong_followup_after_orthodox_style_trace` handoff는 다음 runtime 후보를 `wuxia_mumyeong_midgame_reunion`으로 결정했고, 중반 재회는 기존 encounter schema로 구현 완료했다. `wuxia_mumyeong_followup_after_midgame_reunion` handoff는 다음 runtime 후보를 `wuxia_boss_first_appearance`로 결정했고, 보스 첫 등장은 기존 encounter schema로 구현 완료했다. `wuxia_boss_followup_after_first_appearance` handoff는 다음 runtime 후보를 `wuxia_mumyeong_request_for_aid`로 결정했고, 무명의 도움 요청 실패 기록도 기존 encounter schema로 구현 완료했다. `wuxia_mumyeong_followup_after_failed_aid` handoff는 다음 runtime 후보를 `wuxia_mumyeong_awakening`으로 결정했고, 무명의 각성도 기존 encounter schema로 구현 완료했다. `wuxia_mumyeong_followup_after_awakening` handoff는 다음 runtime 후보를 `wuxia_qingliu_attack_after_war`로 결정했고, 청류문 흔적 조사도 기존 encounter schema로 구현 완료했다. `wuxia_qingliu_attack_after_war_followup` handoff는 다음 runtime 후보를 `wuxia_mumyeong_destroys_orthodox_sect`로 결정했고, 현악문 consequence trace도 기존 encounter schema로 구현 완료했다. `wuxia_mumyeong_destroys_orthodox_sect_followup` handoff는 다음 runtime 후보를 `wuxia_boss_recruits_mumyeong`로 결정했고, 보스 recruitment trace도 기존 encounter schema로 구현 완료했다. `wuxia_boss_recruits_mumyeong_followup` handoff는 다음 runtime 후보를 `wuxia_mumyeong_departure_truth_summary`로 결정했고, sealed departure truth summary도 기존 encounter schema로 구현 완료했다. `wuxia_mumyeong_departure_truth_summary_followup` handoff는 다음 runtime 후보를 `wuxia_seoharin_empty_place`로 결정했고, late empty-place memory bridge도 기존 encounter schema로 구현 완료했다. `wuxia_seoharin_empty_place_followup` handoff는 다음 runtime 후보를 `wuxia_seoharin_left_meal`로 결정했고, left-meal daily-care bridge도 기존 encounter schema로 구현 완료했다. `wuxia_seoharin_left_meal_followup`은 `docs/design/Wuxia_Final_State_Routing.md`로 final state routing contract를 먼저 고정했고, `wuxia_sado_final_phase_1_price_tag`도 기존 encounter schema로 구현 완료했다. `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`, `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, `wuxia_seoharin_qingliu_resolution`, `wuxia_cheongirok_resolution`, `wuxia_black_serpent_aftermath`도 기존 encounter schema로 구현 완료했으며, `wuxia_final_epilogue_renderer_contract`는 preview ending + Rust GameCore-owned structured body block consumer로 구현 완료했다. full boss combat, return/settlement schema, post-opener any-of condition, route graph, faction reputation, debt/relation/combat/random copy-style/reward schema, 서하린 truth delivery, Cheonggi Record identity reveal, `told_seoharin_truth`, `item_unpriced_wooden_sword` 지급은 바로 열지 않는다.
- 필요한 신규 설계는 encounter/choice/outcome 수준으로 제한한다. 새 combat/reward/ability schema, 천외편린 3택 reward schema, faction route graph schema는 별도 slice 전까지 열지 않는다.

2026-06-01 Notion sync 결정:

- Notion 사건 카드 DB 26개는 `docs/dev/Notion_Design_Coverage.md`와 `docs/content/encounter_db/wuxia_jianghu_pack.md`에서 repo 후보와 future source로 매핑한다.
- Notion 후일담 카드 DB 17개는 future design source다. 아직 runtime epilogue schema/renderer나 machine-readable encounter mirror에 넣지 않는다.
- Notion `wuxia_seoharin_intervention` / `서하린의 개입`은 repo `wuxia_seo_harin_rescue`에 직접 대응하며 preview runtime으로 구현됐다.
- Notion `wuxia_tianjilu_first_fragment`, `wuxia_black_serpent_first_trouble`, `wuxia_prologue_commute_rift`, `wuxia_arrival_market_confusion`, `wuxia_seoharin_intervention`과 repo raid route-pressure mapping은 이미 구현된 preview beat와 매핑된다. 이 매핑은 runtime completeness 범위를 해당 preview encounter들로만 제한한다.

`validate_storypack_db()`가 검사하는 기준:

1. card의 `storypack_id`가 존재한다.
2. card의 `world_id`가 storypack record의 `world_id`와 일치한다.
3. `status`, `priority_class`, `surface`, `anomaly_type`, `pressure_type`, `npc_slots`가 canonical taxonomy 안에 있다.
4. 카드마다 최소 하나의 `safe_*` 또는 fallback 선택지가 있다.
5. 카드마다 최소 하나의 outcome hook이 있다.
6. `main_spine_link`가 비어 있지 않다.
7. 공개 DB에 `final_hint`, `actual_ip_address`, `office_location`, `treasure_location` 같은 private-only field를 넣지 않는다.

## 7. Character DB / 6스탯 schema

NPC는 6스탯과 인카운터 기능을 함께 가진다. stat은 성격 묘사보다 “이 인물이 어떤 장면에서 무엇을 가능하게 하는가”를 설명해야 한다.

```yaml
id: park_doyoon
status: candidate
name: 박도윤
department: IT 인프라팀
story_function: [infra_interpreter, unreliable_helper]
one_line: 냉소적이지만 서버 로그를 끝까지 확인하는 IT 인프라 담당자.
core_stats:
  sense: 11
  social: 7
  reason: 16
  self: 10
  impulse: 6
  body: 10
stat_total: 60
encounter_uses:
  enables: [server_log_interpretation, channel_route_trace_hint]
  complicates: [social_reassurance, urgent_physical_escape]
```

### 7.1 6스탯 의미

| stat | 의미 | 인카운터에서의 기능 |
|---|---|---|
| `sense` / 감각 | 미세한 변화, 표정, 소리, 공간 위화감 감지 | 단서 발견, 거짓말 징후, CCTV/공간 이상 감지 |
| `social` / 사회 | 회사 인간관계와 말의 생태계에서 살아남는 능력 | 설득, 소문, 거짓말, 분위기 완충/조작 |
| `reason` / 이성 | 문서, 숫자, 규칙, 시스템, 절차 이해 | 로그 분석, 회의록 해석, 사내 시스템 퍼즐 |
| `self` / 자아 | 자기 자신을 유지하는 힘 | 오염/공포/존재 말소 저항 |
| `impulse` / 충동 | 순간 행동력, 도주, 기습, 감정적 추진력 | 빠른 구조, 돌발 배신, 무모한 선택 |
| `body` / 신체 | 물리적 능력과 존재감 | 추격/도주, 문 열기/막기, 부상 저항 |

규칙:

- 초기 NPC 총합은 60.
- 초기 스탯은 0~20.
- 특별 이유가 없으면 최소 3 이상.
- 관계값은 core stat이 아니다. `trust`, `suspicion`, `debt`, `contamination`, `alive`, `companion` 같은 별도 상태로 다룬다.
- 플레이어 ability 0~6 체계와 NPC 0~20 체계는 당장 통합하지 않는다.
- 런타임에서 연결할 때는 보조 효과로 시작한다.
  - 높은 `reason`: 문서/로그 분석 선택지 힌트 제공
  - 높은 `body`: 위험 이동 실패 피해 감소 후보
  - 높은 `social`: 설득/협상 대체 경로 후보
  - 낮은 `self`: 문서/기록 오염 리스크 후보

## 8. Promotion workflow

```text
raw idea
  -> storypack/card extraction
  -> candidate DB card
  -> tone/safety/schema review
  -> curated card
  -> runtime promotion target 선택
  -> 별도 구현 작업에서 runtime YAML/content bundle 작성
  -> docs/content 목록 갱신
  -> idea_box source 파일 done/merged/rejected 기록
```

승격 전 체크리스트:

- 메인 story spine을 보강하는가?
- 의미 있는 선택지가 있는가, 단순 분위기 문단이 아닌가?
- 안전한 fallback 선택지가 있는가?
- 인카운터-first loop에서 기존 이동/루트를 막지 않는가?
- public-safe인가?
- TUI/fake-terminal surface와 잘 맞는가?
- flags/clues/items/relation hooks가 추적 가능한가?
- once-only, repeatable, cooldown, route-gated 중 어느 정책인지 적혀 있는가?
- 기존 runtime encounter와 너무 중복되지 않는가?

## 9. Validation model

문서 DB 단계 검증:

- storypack id 고유성
- character id 고유성
- encounter situation id 고유성
- 모든 encounter situation의 `storypack_id`가 존재
- `candidate_characters`가 있으면 character record가 존재
- NPC `stat_total`이 60
- NPC core stats가 0~20
- 모든 encounter situation에 `main_spine_link` 존재
- 모든 encounter situation에 fallback 선택지 존재
- public-safe 검토 통과

나중에 machine-readable YAML로 전환하면 다음 테스트를 추가할 수 있다.

- `tests/test_storypack_db.py`
  - storypack/character/situation DB 로드
  - 참조 무결성 검증
  - stat 총합 검증
  - public-safe 금지 필드/문구 검증

런타임 콘텐츠로 승격하는 별도 작업에서는 기존 content validation도 함께 실행한다.

- `python -m pytest tests/test_content_data.py -q`
- `python scripts/export_web_data.py --check`
- `cargo test -p escape-core`

## 10. 첫 vertical slice: 차원격리팩

첫 DB slice는 `isolation_pack`으로 시작했다. 첫 비-office 검증 slice는 **이구학지 — 천기록** 설정의 `wuxia_jianghu_pack`으로 한다.

이유:

- 메인 스토리의 “부재, 사내망, 회의록, CCTV, 격리” 축과 가장 가깝다.
- 기존 runtime 인카운터인 `ex_employee_messenger`, `meeting_room_all_hands`, `security_room_delayed_cctv`와 이어 붙이기 쉽다.
- 일반 대기업 배경과 반도체/SW 개발센터 배경을 모두 흡수할 수 있다.
- TUI/fake-terminal 표면이 강하다.

첫 office slice 산출물:

- `docs/content/storypacks/isolation_pack.md`
- `docs/content/characters/recurrent_npcs.md`
- `docs/content/encounter_db/isolation_pack.md`

추가 office-dream 후보 산출물:

- `docs/content/storypacks/yageunmong_pack.md`
- `docs/content/encounter_db/yageunmong_pack.md`

첫 비-office slice 산출물:

- `docs/design/Storypack_World_Model.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`

첫 NPC 후보:

- 박도윤: IT 인프라팀, 사내망/로그/권한 해석
- 윤서연: 인사팀, 조직도/근태/존재 기록
- 최민재: 보안팀/방재실, CCTV/출입통제/건물 봉쇄

첫 encounter situation 후보:

1. `isolation_channel_mismatched_floor`
2. `org_chart_missing_employee`
3. `delayed_cctv_next_action`
4. `server_log_other_branch`
5. `automatic_minutes_no_attendees`
6. `pantry_survivor_trace`

## 11. 열린 질문

1. docs-first DB가 충분한가, 아니면 다음 단계에서 machine-readable YAML DB가 필요한가?
   - 기본 추천: docs-first로 한 slice를 검토한 뒤 YAML화 여부를 결정한다.
2. 이름 있는 NPC 3명을 canonical로 둘 것인가, role slot을 canonical로 둘 것인가?
   - 기본 추천: role slot을 primary key로 두고 named NPC는 candidate로 둔다.
3. 일반 대기업 버전과 반도체/SW 버전 중 어느 쪽이 상위 기준인가?
   - 기본 추천: 일반 대기업 버전을 상위 기준으로 두고, SW 소재는 TUI/system surface가 강할 때만 흡수한다.
4. NPC 6스탯을 플레이어 ability와 통합할 것인가?
   - 기본 추천: 지금은 통합하지 않는다. 런타임 승격 때 companion/assist effect로 연결한다.
5. 카드 몇 개가 있어야 storypack이 유효한가?
   - 기본 추천: storypack당 최소 6개 후보 카드, 첫 runtime 승격은 2~3개면 충분하다.

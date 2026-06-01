# 스토리팩 세계관 모델

## 1. 문서 목적

이 문서는 `tui_adv`를 “회사 아포칼립스 전용 게임”이 아니라, 여러 세계관 storypack을 갈아끼울 수 있는 선택지/생존/인카운터 엔진으로 개발하기 위한 기준을 정한다.

현재 Web/terminal default storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**이다. `escape from the office` office isolation content는 legacy/parity 기준팩으로 유지한다. 앞으로의 개발 기준은 다음처럼 둔다.

```text
Generic narrative engine
  -> world/storypack content bundle
  -> Rust GameCore truth
  -> ScenePage / ActionResult
  -> Web Storybook + GlyphFX
  -> SuperLightTUI terminal renderer
```

즉, “회사”는 엔진의 정체성이 아니라 첫 번째 legacy storypack이다. 신규 기능은 office 전용 단어, office 전용 surface, office 전용 resource 해석에 바로 묶지 않고, 최소 두 세계관에서 설명 가능한 형태로 설계한다.

## 2. 현재 고정 결정

- 기본 storypack: `wuxia_jianghu_pack` / **이구학지 — 천기록**. 2026-06-01 전환 이후 Web player의 main/default run은 이구학지다.
- legacy/parity storypack: `office_isolation_pack` / `isolation_pack` 계열. 기존 office runtime content와 `escape-office` save/localStorage compatibility는 유지한다.
- 추가 office-family 후보 storypack: `yageunmong_pack`. 회사 자각몽/악몽/퇴근 게이트 후보이며, 기본 office runtime을 자동 대체하지 않는다.
- 첫 비-office 기준 storypack: `wuxia_jianghu_pack`.
- `wuxia_jianghu_pack`의 최신 canonical story는 **이구학지 — 천기록**이다.
- 2026-06-01 Notion live check 이후 `이구학지` upstream은 parent page를 synopsis로 보고, 하위 관리 문서와 사건/후일담 DB를 최신 세부 기준으로 우선한다. Repo implementation은 `docs/dev/Notion_Design_Coverage.md`, `idea_box/notion_sources.yml`, storypack/encounter canonical docs에 매핑된 내용만 따른다.
- 이전의 짧은 generic 무협 placeholder(객잔 도착, 소림/무당/아미 저해상도 앵커)는 superseded로 본다.
- 첫 목표는 런타임 다중 storypack 선택 UI가 아니라, 새 설계/콘텐츠가 office 없이도 성립하는지, 그리고 office 내부에서도 다른 story premise가 같은 공용 시스템을 공유할 수 있는지 검증하는 것이다.
- Rust GameCore / `ScenePage` / WASM JSON boundary는 계속 renderer-neutral truth를 소유한다.
- Web Storybook과 SuperLightTUI는 storypack-specific 문장과 semantic presentation hint를 표시하지만, storypack별 규칙을 renderer에서 재계산하지 않는다.

## 3. Storypack과 world의 관계

이 프로젝트에서 `world`와 `storypack`은 다음처럼 구분한다.

| 개념 | 의미 | 예 |
|---|---|---|
| `world_id` | 큰 장르/세계관 축 | `office_apocalypse`, `office_dream`, `wuxia_jianghu` |
| `storypack_id` | world 안에서 실제 사건/덱을 묶는 후보 | `isolation_pack`, `yageunmong_pack`, `wuxia_jianghu_pack` |
| `surface` | 사건이 드러나는 매체/표면 | 사내 메신저, 시장, 수련 잡일, 천기록, 문파 습격 |
| `pressure_type` | 플레이어를 압박하는 축 | 체력, 정신력, 위험도, 관계, 명성, 빚, 심마 등 |
| `route_hook` | 탈출/정복/진실/히든 같은 큰 목표와 연결되는 고리 | 격리 규칙, 귀환 단서, 문파 소속, 천기록의 정체 |

`world_id`는 나중에 machine-readable schema로 열 수 있지만, 지금은 docs-first 설계 필드로만 사용한다.

## 4. 엔진 중립 원칙

새 기능을 설계할 때 다음 질문을 통과해야 한다.

1. 이 기능은 회사가 아닌 무협 storypack에서도 의미가 있는가?
2. 특정 surface가 office 전용이라면, 무협에서 대응 surface는 무엇인가?
3. renderer가 이 기능을 표시만 하는가, 아니면 storypack 규칙을 재계산하고 있는가?
4. `ScenePage`에 renderer-specific 또는 world-specific presentation detail을 넣고 있지 않은가?
5. 런타임 save/state key가 영구적으로 `escape-office`에 묶여야 하는가, 아니면 compatibility layer로 남길 것인가?

## 5. 기존 office 요소의 일반화

| 현재 office 표현 | 일반화된 의미 | 이구학지 대응 예 |
|---|---|---|
| 사내 메신저 | 원격/지연 커뮤니케이션 surface | 표국 전언, 객잔 소문, 천기각 기록 통지 |
| 회의록 | 합의/기록/왜곡된 증언 surface | 문파 회의 기록, 청류문 장문 명령, 객잔 증언 |
| 조직도/근태 | 존재/소속/권한 판정 surface | 청류문 수습생 신분, 문파 명부, 백도맹/흑천련 소속 |
| CCTV/출입기록 | 감시/동선/증거 surface | 시장 목격담, 산문 출입 기록, 흑사방 추적 흔적 |
| 서버 로그/build log | 규칙/분기/진실 기록 surface | 천기록, 천기각 지하서고 기록, 수련 복기 노트 |
| 사원증/보안권한 | 통과/인증 item | 목에 건 사원증을 문파 패나 외지인 표식으로 오해함 |
| 사내 방송 | 광역 공지/압박 cue | 시장 소문, 문파 비상 종, 청류문 습격 경보 |
| 현실 연결 힌트 | 선택적 ARG/local-only layer | 기본 무협 pack에서는 비활성 또는 별도 local layer |

## 6. Resource와 ability 일반화

기존 core 자원은 바로 버리지 않는다.

| 현재 자원 | 엔진 의미 | office 표시 | 이구학지 표시 후보 |
|---|---|---|---|
| `health` | 물리적 생존력 | 신체 반응 | 기혈 / 내상 / 외상 |
| `sanity` | 현실/자아 안정성 | 집중도 / 정신 안정도 | 심마 / 이방인 정체성 압박 |
| `battery` | 정보/도구 사용 여력 | 단말기 전원 | 스마트폰 부재, 천기록 발현 기회, 전서/정보 비용 |
| `hunger` | 장기 생존 압박 | 허기 | 굶주림 / 수련 피로 / 숙식 채무 |
| `thirst` | 빠른 생존 압박 | 갈증 | 갈증 / 진기 소모 / 약초차 의존 |
| `danger` | 세계/구역 압박 | 격리 위험도 | 흑사방 추적도 / 혈월교 습격 압박 / 강호 소문 |

첫 단계에서는 내부 field명을 바꾸지 않는다. world별 display label과 content text로 해석을 분리한다. 필드명 rename은 save/schema migration이 필요한 별도 작업이다.

## 7. Storypack authoring contract

새 storypack 문서는 최소한 다음을 가진다.

```yaml
id: wuxia_jianghu_pack
world_id: wuxia_jianghu
status: candidate
name: 이구학지 — 천기록
one_line: 한 줄 컨셉
main_surfaces: []
anomaly_types: []
main_phases: []
reusable_npc_slots: []
ending_candidates: []
main_spine_support: 엔진의 큰 루프를 어떻게 검증하는지
runtime_promotion_notes: 첫 runtime slice 후보와 금지선
```

Encounter situation card는 다음 필드를 권장한다.

```yaml
id: wuxia_commute_rift_arrival
storypack_id: wuxia_jianghu_pack
world_id: wuxia_jianghu
phase: market_arrival
priority_class: main_forced
location_tags: [commute_boundary, market, starting_scene]
surface: [commute_rift, market_street, office_items]
anomaly_type: world_displacement
pressure_type: [sanity, danger]
choice_shapes:
  - role: safe_observe
  - role: social_probe
  - role: safe_reposition
main_spine_link: office 출신 플레이어가 다른 world로 이동해도 같은 engine loop를 검증하는 이유
```

## 8. 첫 비-office 기준팩: 이구학지 — 천기록

`docs/content/storypacks/wuxia_jianghu_pack.md`를 첫 기준팩으로 둔다. 이 팩의 플레이어 전제는 “현대 회사원이 본인 몸과 출근복장 그대로 무협 세계의 시장 한복판에 떨어지고, 흑사방 첫 전투와 청류문 수습생 구간을 거쳐 천기록/천외편린 성장 구조를 경험한다”이다.

최신 운영 기준:

- 청류문은 몰락한 정파 문파이자 따뜻한 언더독 홈베이스다. 내부 정치질/배신자/장로회의 고구마가 아니라 결핍과 외부 압박으로 갈등을 만든다.
- 청류문 성장 문법은 `청류심법`, `청류안`, `관류`, `수류`, `역류`, `환류`를 중심으로 둔다.
- 천기록은 검색창이 아니며 후보 3개 중 하나만 고르는 천외편린 보상 장치다. 인격처럼 보이는 기록으로 암시할 수 있지만 정체는 끝까지 밝히지 않는다.
- Notion 사건 카드 DB 26개와 후일담 카드 DB 17개는 future/source coverage로 추적하되, 현재 runtime/machine-readable mirror에 자동으로 모두 반영하지 않는다.

이 팩이 필요한 이유:

- office에서 출발한 플레이어가 office-specific surface 없이도 선택지/상태/인카운터/route hook을 이어갈 수 있는지 검증한다.
- 소속, 평판, 관계, 성장, 전투, 상태, 자원, 선택, 사건, 엔딩을 storypack 언어만 바꿔 재사용하는 기준이 된다.
- 최근 전투 시스템 문서의 “자동 난투 + 상황 개입” 설계를 흑사방 첫 전투로 자연스럽게 시험할 수 있다.
- 천기록/천외편린은 새 성장 UI 후보지만, 첫 slice에서는 flag/clue/log/presentation text로만 다루고 schema 확장은 보류할 수 있다.
- Web Storybook의 모바일 게임북 UI와 SuperLightTUI는 시장, 문파 수습생, 수련 잡일, 천기록 3택 같은 텍스트 surface를 표현하기 좋다.

## 8.5 office-family 후보: 야근몽

`docs/content/storypacks/yageunmong_pack.md`는 Notion-origin `회사 스토리팩: 야근몽` 아이디어를 repo의 storypack 후보로 옮긴 문서다. 이 후보는 `isolation_pack`과 같은 회사 surface를 사용하지만, premise와 목표가 다르다.

- `isolation_pack`: 사람 실종/공간 격리/사내 시스템 미스터리를 기본 office runtime에 가까운 후보로 다룬다.
- `yageunmong_pack`: 회사에서 잠든 주인공이 자각몽 상태의 회사 악몽에서 업무 완료가 아니라 깨어나기를 목표로 한다.

공유해야 하는 공용 시스템 축:

- 소속/권한: 부서·결재·승인 surface를 storypack별로 다르게 해석한다.
- 관계: 동료/NPC를 구출·협력 대상으로 둘지, 악몽화된 압박 형상으로 둘지 분리한다.
- 성장: `yageunmong_pack`의 각성편린은 `wuxia_jianghu_pack`의 천외편린과 같은 3택 성장 문법을 공유할 수 있지만, 첫 runtime slice에서는 새 reward schema를 열지 않는다.
- 엔딩: 깨어남/무한 야근/동료 구출/퇴근 선언은 runtime 구현 전 candidate ending으로만 둔다.

2026-05-31 live Notion Markdown 대조를 완료했고, 관련 idea entry는 이 문서와 후보 DB로 승격되어 done 처리했다.

## 9. 개발 순서 제안

### Step 1 — docs-first world/storypack foundation

현재 문서 변경의 범위다.

- `Storypack_World_Model.md` 추가/갱신.
- `wuxia_jianghu_pack.md`를 최신 **이구학지 — 천기록** 설정으로 갱신.
- `encounter_db/wuxia_jianghu_pack.md`를 최신 설정의 situation card로 갱신.
- README/Index/Development_Plan/Checklist/AGENTS를 office-only 표현에서 storypack-capable 표현으로 조정.

### Step 2 — machine-readable storypack DB 검토

현재 완료된 slice다.

- `docs/content/storypack_db/storypacks.json`에 office isolation / office dream / wuxia storypack record를 둔다.
- `docs/content/storypack_db/encounter_situations.json`에 `isolation_pack` 6개, `yageunmong_pack` 6개, `wuxia_jianghu_pack` 18개, 총 30개 repo 후보 카드를 둔다. Notion 사건 카드 DB의 26개 row는 `docs/dev/Notion_Design_Coverage.md`에서 매핑한 upstream source이며, machine-readable mirror에 전부 자동 import하지 않는다.
- `src/tui_adv/game/storypack_db.py`의 `validate_storypack_db()`로 `world_id`, `storypack_id`, `surface`, `phase`, `priority_class`, fallback 선택지, outcome hook을 검증한다.
- 아직 runtime game content로 바로 섞지 않는다.

### Step 3 — storypack runtime preview mode 결정

현재 완료된 결정이다.

- 첫 non-office runtime prototype은 **separate preview mode first**로 진행한다.
- 기본 office bundle과 `src/tui_adv/data/*.yaml`에는 무협 prototype을 직접 섞지 않는다.
- `wuxia_jianghu_pack`은 Web/terminal 기본 storypack이며, explicit preview bundle/preview flag도 같은 fixture를 가리키는 호환 entrypoint로 유지한다.
- save/localStorage의 `escape-office` key는 변경하지 않는다.
- 자세한 결정은 `docs/dev/Storypack_Runtime_Preview_Mode.md`를 따른다.

### Step 4 — first runtime wuxia route slices

진행 상태: `wuxia_commute_rift_arrival`, `wuxia_heuksa_bang_first_fight`, `wuxia_cheonggi_record_first_fragment`, `wuxia_seo_harin_rescue`, `wuxia_cheongryu_apprentice_entry`, `wuxia_cheongryu_chore_sparring`, `wuxia_cheongryu_raid_route_split`, `wuxia_cheongryu_raid_wounded_fallback`, `wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`, `wuxia_wounded_shelter_dawn_offers`, `wuxia_mumyeong_first_sighting`, `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_reads_orthodox_style`, `wuxia_mumyeong_midgame_reunion` preview는 완료했다. 다음 작업은 `wuxia_boss_first_appearance` runtime implementation이다.

- 열일곱 개 implemented preview encounter는 같은 `storypack_preview` bundle에만 둔다. `wuxia_boss_first_appearance`는 다음 runtime 후보로 설계 완료됐지만 아직 YAML/generated bundle에는 없다.
- route opener 후보도 먼저 existing encounter schema로 표현 가능한 start conditions와 outcome hooks를 문서화한다: `conditions`, `choices`, `cost`, `outcome.resources`, `danger`, `add_flags`, `add_clues`, `add_items`/`remove_items`, `destination_id`, `log`, optional `presentation`.
- `wuxia_heuksa_bang_first_fight`의 안정 choice id는 `run_toward_open_street`(fallback), `deescalate_with_words`, `swing_commute_bag`, `loosen_tie_and_drop_shoes`, `crash_in_with_body`다.
- `wuxia_cheonggi_record_first_fragment`의 안정 choice id는 `choose_guard_basics`, `choose_keep_feet_moving`, `choose_failure_log`, fallback `close_notebook_without_choice`다.
- `wuxia_seo_harin_rescue`의 안정 choice id는 `tell_plain_truth`(fallback), `ask_for_medical_help_first`, `explain_company_and_commute`, `show_cheonggi_record_page`, `hide_employee_badge`다.
- `wuxia_cheongryu_apprentice_entry`의 안정 choice id는 `accept_three_month_trial`(fallback), `request_martial_training_immediately`, `organize_chores_like_workflow`, `inspect_archive_during_chore`이며 preview runtime에 구현됐다.
- `wuxia_cheongryu_raid_route_split`의 안정 choice id는 `evacuate_the_wounded_first`(fallback), `defend_cheongryu_with_white_path`, `trade_with_black_heaven`, `follow_heavenly_archive`다.
- `wuxia_cheongryu_raid_wounded_fallback`의 안정 choice id는 `stabilize_wounded_until_dawn`(fallback), `ask_baekdo_for_medicine_not_command`, `trade_black_heaven_bandages_for_exit`, `follow_archive_triage_map`다.
- `wuxia_heavenly_archive_previous_outsiders`는 preview runtime 구현 완료했고, 안정 choice id는 `read_previous_outsider_margins`(fallback), `ask_yeon_soha_what_not_to_read`, `mark_current_worldline_without_answer`, `compare_rift_terms_to_commute_memory`다.
- `wuxia_mumyeong_reads_orthodox_style`는 preview runtime 구현 완료이며, 안정 choice id는 `compare_copied_form_to_old_wound`, `trace_qingliu_eye_variation`, `reconstruct_mumyeongs_sightline`, `stop_before_truth_becomes_accusation`다.
- `wuxia_mumyeong_midgame_reunion`은 preview runtime 구현 완료이며, 안정 choice id는 `ask_why_seoharin_never_called_him_traitor`, `show_the_hyeonakmun_trace_without_accusing`, `point_out_the_copied_form_gap`, `keep_blades_low_and_watch_his_answer`다.
- `ScenePage` schema 변경 없이 진행한다.
- CLI/Rust/Web generated content smoke로 office가 아닌 location/encounter도 표시되는지 검증한다.
- preview launcher/UI wiring은 opt-in entrypoint로 완료했다. 후속 content slice에서 다시 구현하지 않는다.
- 저장 키와 시작 화면의 `escape-office` 명칭은 compatibility 이슈로 별도 migration plan을 세운다.

## 10. 금지선

- 이번 단계에서 기존 office runtime 콘텐츠를 삭제하거나 약화하지 않는다.
- `escape from the office` 문서/아카이브의 역사적 맥락을 억지로 rename하지 않는다.
- save/localStorage key를 즉시 바꾸지 않는다.
- 이구학지의 청류문/백도맹/흑천련/천기각/혈월교는 fiction 세력이다. 현실 종교·정치·민족 소재처럼 보이는 세부 묘사를 무리하게 넣지 않는다.
- 실제 회사명, 실제 통근 경로, 실제 사원증 정보, 실제 위치를 쓰지 않는다.
- renderer에 world별 gameplay truth를 넣지 않는다.

## 11. 현재 상태

- 상태: 설계 문서화 + machine-readable storypack DB 검증 + preview mode 결정 + 무협 열일곱 개 preview runtime + `wuxia_boss_first_appearance` 다음 runtime 후보 선택 + Web/terminal default 이구학지 wiring 완료.
- legacy office 계열: `isolation_pack`은 기존 runtime/parity 후보, `yageunmong_pack`은 별도 office-dream 후보.
- 첫 비-office storypack: `wuxia_jianghu_pack` / **이구학지 — 천기록**.
- 이전 무협 placeholder: superseded.
- 런타임 구현: `wuxia_commute_rift_arrival`, `wuxia_heuksa_bang_first_fight`, `wuxia_cheonggi_record_first_fragment`, `wuxia_seo_harin_rescue`, `wuxia_cheongryu_apprentice_entry`, `wuxia_cheongryu_chore_sparring`, `wuxia_cheongryu_raid_route_split`, `wuxia_cheongryu_raid_wounded_fallback`, `wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`, `wuxia_wounded_shelter_dawn_offers`, `wuxia_mumyeong_first_sighting`, `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_reads_orthodox_style`, `wuxia_mumyeong_midgame_reunion` preview는 완료했다. `wuxia_mumyeong_first_sighting`는 `route_midgame_continuity_after_wounded_shelter` handoff 결과로 구현됐고, `wuxia_mumyeong_first_confrontation`는 `wuxia_mumyeong_first_confrontation_after_sighting` handoff 결과로 구현됐다. `wuxia_mumyeong_reads_orthodox_style`는 `wuxia_mumyeong_followup_after_copy_style_reveal` handoff 결과로 구현됐다. `wuxia_mumyeong_midgame_reunion`는 `wuxia_mumyeong_followup_after_orthodox_style_trace` handoff 결과로 구현됐다. `wuxia_boss_first_appearance`는 `wuxia_mumyeong_followup_after_midgame_reunion` handoff 결과로 다음 runtime 후보가 됐다. `yageunmong_pack` runtime은 미착수다.
- 다음 추천: `wuxia_boss_first_appearance` runtime implementation. legacy office bundle/legacy `escape-office` key/seed 기반 random copy-style system/천외편린 3택 성장/return system/faction route graph/debt ledger/combat schema/boss combat/final boss resolution은 변경하지 않는다.

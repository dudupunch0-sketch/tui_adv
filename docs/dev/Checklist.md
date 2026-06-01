# escape from the office 개발 체크리스트

이 문서는 전체 개발 진행 상황을 추적하기 위한 체크리스트다.
체크박스는 실제 작업 완료 후 갱신한다.

계획 문서 우선순위: 다음 작업 순서와 우선순위는 `docs/dev/Development_Plan.md`가 canonical main plan이다. 이 파일은 완료 여부 추적용이며, 독립적인 다음 계획을 두지 않는다.

## 상태 범례

- `[ ]` 아직 시작하지 않음
- `[x]` 완료
- `[~]` 진행 중으로 표시하고 싶으면 줄 끝에 `진행 중` 메모를 남긴다
- `[!]` 막힘이 있으면 줄 끝에 `BLOCKED:` 사유를 적는다

## Phase 0: 문서 기반 정렬

### 0.1 문서 계층

- [x] `docs/00_Index.md` 생성
- [x] `docs/dev/Development_Plan.md` 생성
- [x] `docs/dev/Checklist.md` 생성
- [x] `docs/01_Game_Overview.md` 생성
- [x] `docs/story/Story.md` 생성
- [x] `docs/story/Reality_Link.md` 생성
- [x] `docs/story/Dream_Ending_Branching.md` 생성
- [x] `docs/story/Real_Escape_Ending_Branching.md` 생성
- [x] `docs/design/Player_State.md` 생성
- [x] `docs/design/Game_Loop.md` 생성
- [x] `docs/design/Map.md` 생성
- [x] `docs/design/UI_Rules.md` 생성
- [x] `docs/dev/Architecture.md` 생성
- [x] `docs/dev/Data_Schema.md` 생성
- [x] `docs/dev/TUI_Layout.md` 생성

### 0.2 핵심 결정

- [x] 게임 제목 또는 임시 코드명 결정: `escape from the office`
- [x] 핵심 자원 5개 확정: 체력, 정신력, 배터리, 허기, 갈증
- [x] 허기/갈증 수치 방향 확정: 낮을수록 좋음, 시간이 지날수록 증가
- [x] 1차 수직 슬라이스 범위 확정: 제안 범위 그대로
- [x] 1차 재난 타입 확정: 불명 재난, 사람 실종, 공간/차원 격리, 사내망 간헐 연락
- [x] 1차 엔딩 범위 확정: 실패, 비상계단 탈출, 첫 히든 힌트
- [x] 현실 연결 안전 원칙 문서화
- [x] 실제 위치 정보 비공개 관리 방식 확정: 공개 문서에는 중간 힌트까지만, 최종 위치는 private/local

### 0.2b 2026-05-22 렌더러/런타임 방향 갱신

- [x] 활성 방향 확정: Rust GameCore 공통 + Web Storybook/GlyphFX primary UX + SuperLightTUI terminal renderer/fallback
- [x] `docs/dev/Rust_Core_Dual_Renderer_Architecture.md` 생성
- [x] 최신 계획 문서에 SuperLightTUI terminal renderer가 fallback/debug dump와 다르다는 기준 반영
- [x] `docs/dev/Data_Schema.md`에 renderer-neutral bundle, `ScenePage`, action id, `EffectCue`, WASM JSON boundary 설계 기록
- [x] `docs/design/UI_Rules.md`와 `docs/dev/TUI_Layout.md`에 Web Storybook/SuperLightTUI renderer UX contract 기록
- [x] `escape-core`에 renderer-safe `ScenePage` contract 추가
- [x] Web Storybook renderer skeleton 추가
- [x] `escape-wasm` JSON-string boundary 추가
- [x] `escape-terminal`을 SuperLightTUI renderer로 전환
- [x] Web/terminal 모두 같은 Rust core action id를 표시하는 parity smoke 추가

### 0.2c 2026-05-22 Rust GameCore route parity 확장

- [x] Movement pages를 Rust core + `ScenePage` 기준으로 확장하고 terminal/Web action id contract 유지
- [x] Item use를 Rust core truth로 이전: usable inventory action, resource effect, consume, turn advance
- [x] Ability checks를 Rust core에서 seeded 2d6 + ability로 처리하고 success/failure outcome 적용
- [x] Escape/failure/truth/conquest/hidden reality-link ending `ScenePage` smoke 추가
- [x] Achievement unlock과 `newly_unlocked_achievements`/`achievement_summary` JSON contract 추가
- [x] Low sanity/low battery/high hunger/high thirst pressure cues를 Rust `ScenePage` semantic cue로 노출
- [x] Reality-link public reward metadata만 ending body block으로 노출하고 private-only fields 차단 유지
- [x] Web Storybook runtime을 `escape-wasm` JSON boundary + generated content bundle에 연결하고 Rust state localStorage 저장 추가

### 0.2d 2026-05-22 Web WASM build/preview 표준화

- [x] Web WASM build/preview 절차 표준화
- [x] `web/package.json`에 `wasm:build`, `build:wasm`, `dev:wasm`, `preview:wasm` script 추가
- [x] `build:wasm`이 generated wasm package를 `web/dist/assets/wasm-pkg/`로 복사
- [x] `web/src/core/wasm-pkg/` generated package를 local-only artifact로 ignore
- [x] legacy Python/Textual/TypeScript mirror freeze 범위 결정

### 0.2e 2026-05-22 SuperLightTUI terminal polish

- [x] terminal visual card가 visual_id/layout/alt를 ASCII/Unicode card로 표시
- [x] GlyphFX fallback이 intensity meter, stable terms, fallback text를 보존
- [x] 직접 플레이 입력 안내가 현재 턴 번호 범위와 action id 사용법을 표시

### 0.2f 2026-05-22 Web 배포 표면 결정

- [x] Web/Tauri/Electron 패키징 검토와 Web-only 배포 표면 결정
- [x] `web/package.json`에 `build:player` / `preview:player` alias 추가
- [x] Tauri/Electron은 desktop wrapper 고유 가치가 생길 때까지 deferred로 문서화

### 0.2g 2026-05-22 SuperLightTUI app loop / GlyphFX baseline

- [x] `escape-terminal --app` full-screen SuperLightTUI app loop 추가
- [x] `--app-smoke --tick` headless app-frame smoke 추가
- [x] raw-draw GlyphFX layer가 tick 변화와 stable terms/fallback text를 함께 검증
- [x] inline image는 baseline 밖 optional future로 deferred 결정

### 0.2h 2026-05-23 Web Storybook 모바일 픽셀 board redesign

- [x] Web Storybook 모바일 픽셀 board contract 문서화
- [x] HUD/rail/dock/sentence-choice renderer contract 구현
- [x] reference-size browser visual QA

### 0.2i 2026-05-23 Web Storybook visual regression 자동화

- [x] `web/scripts/storybook-reference-qa.mjs` Playwright viewport runner 추가
- [x] package script로 visual QA command 노출
- [x] reference viewport DOM/layout/interaction contract 자동 검증
- [x] optional `--require-wasm` Rust/WASM-primary resource load smoke 추가
- [x] screenshots/JSON report를 scratch output에만 남기도록 문서화
- [x] visual QA contract/docs tests 추가

### 0.2j 2026-05-24 Web player 공개 배포 계획 문서화

- [x] `idea_box/web_play_like_pokerogue.md`를 읽고 `docs/dev/Web_Player_PokeRogue_Style_Plan.md`로 승격
- [x] Web player URL 즉시 플레이, WASM-required production policy, static deploy QA, start/save UX PR 순서를 문서화

### 0.2k 2026-05-24 Web player deployment readiness

- [x] `VITE_BASE_PATH` 기반 Vite base path 설정
- [x] WASM module path를 `import.meta.url` 기준으로 하드닝
- [x] `VITE_REQUIRE_WASM=true` production fatal policy 추가
- [x] GitHub Pages deploy workflow 추가
- [x] Web player deployment contract/docs tests 추가

### 0.2l 2026-05-26 Web player start/save UX first slice

- [x] Web player start screen 추가
- [x] 이어하기/새 게임/seed 표시/save timestamp UX 추가
- [x] 새 게임 전 저장 reset confirmation 추가
- [x] 저장 summary schema mismatch warning 추가
- [x] visual QA가 start screen을 통과해 Storybook page를 검증하도록 갱신
- [x] start/save UX contract tests 추가

### 0.2m 2026-05-26 Web Storybook transition/audio readiness

- [x] transition/audio readiness active main plan 승격
- [x] player settings localStorage contract 구현
- [x] start screen audio/motion toggle 연결
- [x] transition plan type과 reduced-motion no-op 구조 추가
- [x] audio muted default policy와 opt-in skeleton 추가
- [x] transition controller 적용
- [x] Web Audio API lazy/no-op engine 추가
- [x] muted no schedule + user-gesture opt-in one-shot cue API 구현
- [x] looping ambience API와 binary asset 없는 generated oscillator backend 구현
- [x] visual QA motion/audio 안정화 확인

### 0.2n 2026-05-29 전투 시스템 아이디어 문서화

- [x] `idea_box/combat_system.md`를 `docs/design/Combat_System_Auto_Brawl.md`로 승격
- [x] 자동 난투 + 상황 개입을 Rust GameCore/`ScenePage` 호환 설계 후보로 정리
- [x] `docs/design/Basic_Combat_Action_Model.md`에 이구학지 기준이지만 office에도 재사용하는 기본 전투 액션 taxonomy 정리

### 0.2o 2026-05-29 schema-less combat encounter prototype runtime

- [x] 기존 encounter/choice/outcome schema만 사용한 물품창고 자동 난투 구현
- [x] `supply_closet_cache`에서 `supply_closet_auto_brawl`로 이어지는 1회 상황 개입 경로 추가
- [x] Rust `ScenePage` / SuperLightTUI / Web generated data parity 검증 추가
- [x] 새 `CombatState`, combat schema, HP 숫자전, renderer gameplay 판정 없이 완료

### 0.2p 2026-05-29 storypack/world 일반화와 무협 기준팩

- [x] office-only 편향을 줄이는 `docs/design/Storypack_World_Model.md` 추가
- [x] 첫 비-office 기준팩 `docs/content/storypacks/wuxia_jianghu_pack.md` 추가
- [x] 무협 기준팩 encounter situation cards 추가
- [x] AGENTS/README/Index/Development_Plan을 storypack 기반 방향으로 동기화

### 0.2q 2026-05-31 Notion-first 아이디어-설계 운영 규칙

- [x] Notion을 원본 reference로 두는 아이디어-설계 파이프라인 문서화
- [x] 설계 아이디어 문서 변환 → main plan 격상 → 설계 후 Notion reference 대조 → done 처리 규칙 추가
- [x] `idea_box` 운영 문서와 LLM 설계 핸드오프에 Notion-origin entry 처리 기준 반영

### 0.2r 2026-05-29 이구학지 — 천기록 최신화

- [x] Notion 최신 무협안 `이구학지 — 천기록`을 `wuxia_jianghu_pack` canonical story로 반영
- [x] 이전 generic 무협 placeholder를 superseded로 명시
- [x] 흑사방 첫 전투, 청류문 수습생, 천기록/천외편린 encounter situation cards 갱신
- [x] README/AGENTS/Index/Development_Plan과 idea_box backlog 상태 동기화

### 0.2s 2026-05-29 machine-readable storypack DB 검증

- [x] `docs/content/storypack_db/storypacks.json`에 office/wuxia storypack record 추가
- [x] `docs/content/storypack_db/encounter_situations.json`에 후보 카드 12개 추가
- [x] `src/tui_adv/game/storypack_db.py`에 `load_storypack_db()` / `validate_storypack_db()` 추가
- [x] storypack/world/taxonomy/fallback/outcome hook 참조 무결성 테스트 추가

### 0.2t 2026-05-29 storypack runtime preview mode 결정

- [x] `docs/dev/Storypack_Runtime_Preview_Mode.md`에 separate preview mode first 결정 문서화
- [x] 기본 office bundle과 무협 preview bundle을 섞지 않는 no default bundle mixing 경계 확정
- [x] `escape-office` save/localStorage key 유지와 renderer-neutral `ScenePage` 표시 경계 확정
- [x] Data_Schema/World_Model/README/Index/docs contract 동기화

### 0.2u 2026-05-31 무협 storypack preview runtime prototype

- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/*.yaml`에 `wuxia_commute_rift_arrival` preview source 추가
- [x] `scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack` preview bundle write/check 경로 추가
- [x] Rust fixture와 Web generated preview bundle을 기본 office bundle과 분리해 생성
- [x] `escape-terminal`과 `escape-wasm::new_game_json()`이 preview `runtime.default_location`에서 시작하도록 연결
- [x] Python exporter, Rust content metadata, WASM JSON boundary, SuperLightTUI smoke 테스트 추가

### 0.2v 2026-05-31 야근몽 storypack 후보 문서화

- [x] `docs/content/storypacks/yageunmong_pack.md`에 야근몽 office-dream candidate record 추가
- [x] `docs/content/encounter_db/yageunmong_pack.md`에 runtime 승격 전 상황 카드 6개 추가
- [x] `docs/content/storypack_db/storypacks.json`와 `encounter_situations.json`에 야근몽 후보 record/card mirror 추가
- [x] `docs/design/Storypack_World_Model.md`, `docs/design/Storypack_Encounter_DB.md`, README/Index/Development_Plan 동기화
- [x] live Notion Markdown 대조 후 관련 idea_box entry 2개 done 처리

### 0.2w 2026-05-31 무협 후속 preview slice 설계 확정

- [x] `wuxia_heuksa_bang_first_fight`를 다음 무협 storypack preview 구현 slice로 확정
- [x] stable choice id, fallback choice, outcome hook, flags/clues/log 방향 문서화
- [x] 기본 office bundle, `escape-office` save/localStorage key, 천외편린/각성편린 reward schema non-goal 재확인
- [x] 다음 구현 세션의 예상 수정 파일, 테스트, 검증 명령, preview artifact 위치 handoff 작성

### 0.2x 2026-05-31 무협 `wuxia_heuksa_bang_first_fight` preview runtime slice

- [x] `jianghu_market_street` preview location과 `wuxia_arrival_hidden` route 연결 추가
- [x] 기존 encounter/choice/outcome schema만 사용한 흑사방 첫 난투 구현
- [x] Rust fixture와 Web generated preview bundle 재생성
- [x] Python exporter, Rust content fixture, WASM JSON boundary, SuperLightTUI first-fight parity 테스트 추가
- [x] 기본 office bundle, Web 기본 generated bundle, `src/tui_adv/data/*.yaml`, `escape-office` save/localStorage key 미변경 확인

### 0.2y 2026-05-31 무협 preview launcher/UI wiring

- [x] `escape-terminal --storypack-preview wuxia_jianghu_pack` opt-in flag 추가
- [x] Web start screen에 `wuxia_jianghu_pack` storypack preview launcher 추가
- [x] Web `contentBundles` registry로 default office bundle과 generated storypack preview bundle 분리
- [x] preview launcher가 기본 office save/localStorage key를 변경하지 않는 boundary 테스트 추가
- [x] default office bundle, Web 기본 generated bundle, `src/tui_adv/data/*.yaml` 미변경 유지

### 0.2z 2026-05-31 무협 `wuxia_cheonggi_record_first_fragment` preview runtime slice

- [x] 첫 난투 뒤 `heuksa_bang_first_fight_resolved`로 이어지는 천기록 첫 편린 encounter 추가
- [x] `cheonggi_record_notebook` item과 `wuxia_first_fragment_seen` achievement를 preview bundle에만 추가
- [x] `choose_guard_basics`, `choose_keep_feet_moving`, `choose_failure_log`, fallback `close_notebook_without_choice` stable choice id 고정
- [x] Rust fixture와 Web generated preview bundle 재생성
- [x] Python exporter, Rust content fixture, WASM JSON boundary, SuperLightTUI first-fragment parity 테스트 추가
- [x] 천외편린 3택 reward/ability schema, 기본 office bundle, `escape-office` save/localStorage key 미변경 유지

### 0.2aa 2026-05-31 무협 `wuxia_seo_harin_rescue` 후속 slice 설계

- [x] `wuxia_seo_harin_rescue`를 다음 무협 storypack preview 구현 slice로 확정
- [x] 시작 조건, location/phase, stable choice id, fallback choice, outcome hook 문서화
- [x] relation/debt/healing/companion/combat/reward/ability schema와 천외편린 3택 성장 non-goal 재확인
- [x] 다음 구현 세션의 예상 수정 파일, 테스트, 검증 명령, preview artifact 위치 handoff 작성
- [x] 기본 office bundle, `escape-office` save/localStorage key, yageunmong runtime 미변경 유지

### 0.2ab 2026-05-31 무협 `wuxia_cheongryu_apprentice_entry` 후속 후보 설계

- [x] `wuxia_cheongryu_apprentice_entry`를 `wuxia_seo_harin_rescue` 이후 follow-up 후보로 설계
- [x] rescue 선행 조건, `cheongryu_outer_courtyard` location, stable choice id, fallback choice, outcome hook 문서화
- [x] relation/debt/faction/training XP/chore scheduler/combat/reward/ability schema와 천외편린 3택 성장 non-goal 재확인
- [x] 다음 구현 세션의 예상 수정 파일, 테스트, 검증 명령, preview artifact 위치 handoff 작성
- [x] 기본 office bundle, `escape-office` save/localStorage key, yageunmong runtime 미변경 유지

### 0.2ac 2026-05-31 무협 `wuxia_cheongryu_raid_route_split` 다음 후보 설계

- [x] `wuxia_cheongryu_raid_route_split`를 rescue/apprentice/first-fragment hook 이후 다음 후보로 설계
- [x] raid 선행 조건, location/phase, stable choice id, fallback choice, route outcome hook 문서화
- [x] faction route graph/mass combat/boss combat/companion death/reward/ability schema와 천외편린 3택 성장 non-goal 재확인
- [x] 다음 구현 세션의 예상 수정 파일, 테스트, 검증 명령, preview artifact 위치 handoff 작성
- [x] 기본 office bundle, `escape-office` save/localStorage key, yageunmong runtime 미변경 유지

### 0.2ad 2026-05-31 무협 `wuxia_cheongryu_raid_wounded_fallback` deferred 후보 설계

- [x] raid split의 `evacuate_the_wounded_first` fallback 이후를 받는 deferred-route 후보 설계
- [x] wounded fallback 선행 조건, location/phase, stable choice id, fallback choice, route starter outcome hook 문서화
- [x] route graph/faction/triage/companion death/mass combat/boss combat/reward/ability schema와 천외편린 3택 성장 non-goal 재확인
- [x] later 구현 세션의 예상 수정 파일, 테스트, 검증 명령, preview artifact 위치 handoff 작성
- [x] 구현 backlog 순서, 기본 office bundle, `escape-office` save/localStorage key, yageunmong runtime 미변경 유지

### 0.2ae 2026-06-01 무협 `wuxia_seo_harin_rescue` preview runtime slice

- [x] `cheongryu_outer_courtyard` preview location 추가
- [x] `wuxia_seo_harin_rescue`를 `wuxia_cheonggi_record_first_fragment` 뒤에 schema-less 구조/조사 encounter로 추가
- [x] `tell_plain_truth`, `ask_for_medical_help_first`, `explain_company_and_commute`, `show_cheonggi_record_page`, `hide_employee_badge` stable choice id 고정
- [x] 모든 rescue outcome이 `seo_harin_rescue_resolved`, `seo_harin_intervened`, `taken_under_watch`, `outsider_claim_recorded`, `destination_id: cheongryu_outer_courtyard` bridge를 남기도록 구현
- [x] Rust fixture와 Web generated preview bundle 재생성
- [x] Python exporter, Rust content fixture, WASM JSON boundary, SuperLightTUI rescue parity 테스트 추가
- [x] relation/debt/faction/healing/companion/combat/reward/ability schema, 기본 office bundle, `escape-office` save/localStorage key 미변경 유지

### 0.2af 2026-06-01 무협 `wuxia_cheongryu_apprentice_entry` preview runtime slice

- [x] `work_chore_token` preview item 추가
- [x] `wuxia_cheongryu_apprentice_entry`를 `wuxia_seo_harin_rescue` 뒤에 schema-less 수습생/잡역/서고 bridge encounter로 추가
- [x] `accept_three_month_trial`, `request_martial_training_immediately`, `organize_chores_like_workflow`, `inspect_archive_during_chore` stable choice id 고정
- [x] 모든 apprentice outcome이 `cheongryu_apprentice_entry_resolved`, `cheongryu_trial_started`, `seo_harin_mentor_thread`, `destination_id: cheongryu_outer_courtyard` bridge를 남기도록 구현
- [x] Rust fixture와 Web generated preview bundle 재생성
- [x] Python exporter, Rust content fixture, WASM JSON boundary, SuperLightTUI apprentice parity 테스트 추가
- [x] relation/debt/faction/training XP/chore scheduler/combat/reward/ability schema, 기본 office bundle, `escape-office` save/localStorage key 미변경 유지
- [x] 다음 구현 slice를 `wuxia_cheongryu_raid_route_split`로 갱신

### 0.2ag-pre 2026-06-01 무협 `wuxia_cheongryu_chore_sparring` preview runtime slice

- [x] `wuxia_cheongryu_chore_sparring`를 `wuxia_cheongryu_apprentice_entry`와 first fragment 뒤 1회성 training bridge encounter로 추가
- [x] `step_back_with_firewood`, `let_shoulder_turn_with_push`, `plant_bare_foot_in_dust`, `ask_harin_what_changed` stable choice id 고정
- [x] 모든 chore sparring outcome이 `cheongryu_chore_sparring_resolved`, `chore_sparring_completed`, `destination_id: cheongryu_outer_courtyard` bridge를 남기도록 구현
- [x] 자기 resolved flag를 required/forbidden에 동시에 넣지 않고 `forbidden_flags`에만 둔다고 정리
- [x] Rust fixture와 Web generated preview bundle 재생성
- [x] Python exporter, Rust content fixture, WASM JSON boundary, SuperLightTUI chore sparring parity, Web content bundle tests 추가
- [x] CombatState/combat resolver/skill tree/reward/ability/relation score schema, 기본 office bundle, `escape-office` save/localStorage key 미변경 유지

### 0.2ag 2026-06-01 무협 `wuxia_cheongryu_raid_route_split` preview runtime slice

- [x] `wuxia_cheongryu_raid_route_split`를 `wuxia_cheongryu_apprentice_entry`/`wuxia_cheongryu_chore_sparring` 뒤에 schema-less route-pressure encounter로 추가
- [x] `evacuate_the_wounded_first`, `defend_cheongryu_with_white_path`, `trade_with_black_heaven`, `follow_heavenly_archive` stable choice id 고정
- [x] 모든 raid outcome이 `cheongryu_raid_route_split_resolved`, `cheongryu_raid_survived`, `route_commitment_pressure`, `destination_id: cheongryu_outer_courtyard` bridge를 남기도록 구현
- [x] fallback outcome이 `route_commitment_deferred` / `wounded_saved_flag`를 남겨 `wuxia_cheongryu_raid_wounded_fallback` 선행 조건을 충족하도록 구현
- [x] Rust fixture와 Web generated preview bundle 재생성
- [x] Python exporter, Rust content fixture, WASM JSON boundary, SuperLightTUI raid parity, Web content bundle tests 추가
- [x] faction route graph/reputation/companion death/mass combat/boss combat/reward/ability schema, 기본 office bundle, `escape-office` save/localStorage key 미변경 유지
- [x] 당시 다음 구현 slice를 `wuxia_cheongryu_raid_wounded_fallback`로 갱신했고, 후속 0.2ah에서 구현 완료

### 0.2ah 2026-06-01 무협 `wuxia_cheongryu_raid_wounded_fallback` preview runtime slice

- [x] `wuxia_cheongryu_raid_wounded_fallback`를 `evacuate_the_wounded_first` fallback 뒤 조건부 deferred bridge로 추가
- [x] `stabilize_wounded_until_dawn`, `ask_baekdo_for_medicine_not_command`, `trade_black_heaven_bandages_for_exit`, `follow_archive_triage_map` stable choice id 고정
- [x] 모든 wounded fallback outcome이 `cheongryu_raid_wounded_fallback_resolved`, `deferred_route_reopened`, `destination_id: cheongryu_outer_courtyard` bridge를 남기도록 구현
- [x] route starter flags는 `righteous_route_started`, `sapa_route_started`, `cheonggi_return_route_started`, `route_commitment_deferred`로만 남기고 route graph/faction schema는 열지 않음
- [x] Rust fixture와 Web generated preview bundle 재생성
- [x] Python exporter, Rust content fixture, WASM JSON boundary, SuperLightTUI wounded fallback parity, Web content bundle tests 추가
- [x] faction route graph/reputation/triage/companion death/mass combat/boss combat/reward/ability schema, 기본 office bundle, `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 route opener 선택/설계 docs-only handoff로 갱신

### 0.2ai 2026-06-01 무협 route opener docs-only handoff

- [x] direct raid branch와 deferred wounded fallback branch가 남기는 route starter flags 확인
- [x] 첫 route opener 후보를 정파/백도맹 약상자 채무 축 `wuxia_baekdo_medicine_debt`로 결정
- [x] start conditions를 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [righteous_route_started, cheongryu_rebuild_thread]`, `forbidden_flags: [baekdo_medicine_debt_resolved]`로 문서화
- [x] `baekdo_alliance_debt`와 `baekdo_medicine_debt`는 any-of eligibility가 아니라 branch flavor hook으로만 사용한다고 명시
- [x] stable choice id 후보 `accept_medicine_with_written_debt`, `ask_terms_before_opening_gate`, `send_supplies_to_wounded_first`, `compare_banner_to_record_margin` 고정
- [x] route graph/faction reputation/debt ledger/relation/reward/ability schema는 열지 않는다고 명시
- [x] storypack DB JSON mirror와 docs contract를 다음 runtime handoff 기준으로 갱신
- [x] runtime YAML/Rust/Web/generated bundle, 기본 office bundle, `escape-office` save/localStorage key 미변경 유지

### 0.2aj 2026-06-01 무협 `wuxia_baekdo_medicine_debt` preview runtime slice

- [x] `wuxia_baekdo_medicine_debt`를 첫 정파 route opener로 같은 storypack preview source에 추가
- [x] start conditions를 `righteous_route_started` + `cheongryu_rebuild_thread`로 고정하고 `baekdo_alliance_debt`/`baekdo_medicine_debt`는 flavor hook으로만 유지
- [x] stable choice id `accept_medicine_with_written_debt`, `ask_terms_before_opening_gate`, `send_supplies_to_wounded_first`, `compare_banner_to_record_margin` 구현
- [x] 모든 outcome이 `baekdo_medicine_debt_resolved`, `righteous_route_opened`, `destination_id: cheongryu_outer_courtyard` bridge를 남기도록 구현
- [x] presentation을 `visual_id: wuxia_baekdo_medicine_debt`, `speaker: 남궁서윤`, `layout: righteous_route_opener`, stable terms `약상자 / 백도맹 / 채무`로 고정
- [x] Rust fixture와 Web generated preview bundle 재생성
- [x] Python exporter, Rust content fixture, WASM JSON boundary, SuperLightTUI route opener parity, Web content bundle tests 추가
- [x] route graph/faction reputation/debt ledger/relation/reward/ability schema, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `route_opener_followup_after_baekdo` docs-only handoff로 갱신

### 0.2ak 2026-06-01 무협 route opener follow-up docs-only handoff

- [x] 사파/흑천련 opener, 천기·귀환 opener, deferred-offer card를 Notion reference와 repo hooks 기준으로 비교
- [x] 다음 runtime 후보를 사파/흑천련 거래 opener `wuxia_black_heaven_escape_price`로 결정
- [x] start conditions를 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [sapa_route_started, dowol_debt]`, `forbidden_flags: [black_heaven_escape_price_resolved]`로 문서화
- [x] `black_heaven_deal_marked`와 `black_heaven_escape_marker`는 any-of eligibility가 아니라 branch flavor hook으로만 사용한다고 명시
- [x] stable choice id 후보 `accept_dowol_marker_for_safehouse`, `ask_who_collects_the_price`, `keep_cheongryu_names_off_ledger`, `map_exit_before_following_dowol` 고정
- [x] route graph/faction reputation/debt ledger/relation/reward/ability/epilogue schema는 열지 않는다고 명시
- [x] storypack DB JSON mirror와 docs contract를 다음 runtime handoff 기준으로 갱신
- [x] runtime YAML/Rust/Web/generated bundle, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지

### 0.2al 2026-06-02 무협 `wuxia_black_heaven_escape_price` preview runtime slice

- [x] `wuxia_black_heaven_escape_price`를 첫 사파 route opener로 같은 storypack preview source에 추가
- [x] start conditions를 `sapa_route_started` + `dowol_debt`로 고정하고 `black_heaven_deal_marked`/`black_heaven_escape_marker`는 flavor hook으로만 유지
- [x] stable choice id `accept_dowol_marker_for_safehouse`, `ask_who_collects_the_price`, `keep_cheongryu_names_off_ledger`, `map_exit_before_following_dowol` 구현
- [x] 모든 outcome이 `black_heaven_escape_price_resolved`, `sapa_route_opened`, `destination_id: cheongryu_outer_courtyard` bridge를 남기도록 구현
- [x] presentation을 `visual_id: wuxia_black_heaven_escape_price`, `speaker: 도월`, `layout: sapa_route_opener`, stable terms `탈출로 / 흑천련 / 값`으로 고정
- [x] Rust fixture와 Web generated preview bundle 재생성
- [x] Python exporter, Rust content fixture, WASM JSON boundary, SuperLightTUI route opener parity, Web content bundle tests 추가
- [x] route graph/faction reputation/debt ledger/relation/reward/ability/epilogue schema, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `route_opener_followup_after_black_heaven` docs-only handoff로 갱신

### 0.2am 2026-06-02 무협 route opener follow-up after black heaven docs-only handoff

- [x] 천기·귀환 opener와 `stabilize_wounded_until_dawn` deferred-offer card를 Notion reference와 repo hooks 기준으로 비교
- [x] 다음 runtime 후보를 천기각 이전 이방인 기록 opener `wuxia_heavenly_archive_previous_outsiders`로 결정
- [x] start conditions를 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [cheonggi_return_route_started, cheonggi_record_targeted]`, `forbidden_flags: [heavenly_archive_previous_outsiders_resolved]`로 문서화
- [x] `heavenly_archive_contact`와 `heavenly_archive_triage_map_seen`는 any-of eligibility가 아니라 branch flavor hook으로만 사용한다고 명시
- [x] stable choice id 후보 `read_previous_outsider_margins`, `ask_yeon_soha_what_not_to_read`, `mark_current_worldline_without_answer`, `compare_rift_terms_to_commute_memory` 고정
- [x] 천기록 정체 reveal, return system, route graph/faction reputation/debt ledger/relation/reward/ability/epilogue schema는 열지 않는다고 명시
- [x] storypack DB JSON mirror와 docs contract를 다음 runtime handoff 기준으로 갱신
- [x] runtime YAML/Rust/Web/generated bundle, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지

### 0.2an 2026-06-02 무협 `wuxia_heavenly_archive_previous_outsiders` preview runtime slice

- [x] `wuxia_heavenly_archive_previous_outsiders`를 첫 천기·귀환 route opener로 같은 storypack preview source에 추가
- [x] start conditions를 `cheonggi_return_route_started` + `cheonggi_record_targeted`로 고정하고 `heavenly_archive_contact`/`heavenly_archive_triage_map_seen`는 flavor hook으로만 유지
- [x] stable choice id `read_previous_outsider_margins`, `ask_yeon_soha_what_not_to_read`, `mark_current_worldline_without_answer`, `compare_rift_terms_to_commute_memory` 구현
- [x] 모든 outcome이 `heavenly_archive_previous_outsiders_resolved`, `cheonggi_return_route_opened`, `destination_id: cheongryu_outer_courtyard` bridge를 남기도록 구현
- [x] presentation을 `visual_id: wuxia_heavenly_archive_previous_outsiders`, `speaker: 연소하`, `layout: cheonggi_return_opener`, stable terms `천기각 / 이방인 / 균열`로 고정
- [x] Rust fixture와 Web generated preview bundle 재생성
- [x] Python exporter, Rust content fixture, WASM JSON boundary, SuperLightTUI route opener parity, Web content bundle tests 추가
- [x] 천기록 정체 reveal, return system, route graph/faction reputation/debt ledger/relation/reward/ability/epilogue schema, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `route_opener_followup_after_heavenly_archive` docs-only handoff로 갱신

### 0.2ao 2026-06-02 무협 route opener follow-up after heavenly archive docs-only handoff

- [x] `stabilize_wounded_until_dawn` deferred-offer card와 세 route opener 이후 midgame continuity card를 Notion reference와 repo hooks 기준으로 비교
- [x] 다음 runtime 후보를 부상자 피난처 새벽 제안 `wuxia_wounded_shelter_dawn_offers`로 결정
- [x] start conditions를 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [cheongryu_raid_wounded_fallback_resolved, route_commitment_deferred, deferred_route_reopened, wounded_shelter_stabilized]`, `forbidden_flags: [wounded_shelter_dawn_offers_resolved]`로 문서화
- [x] `survivor_roll_call_complete`와 `route_delay_cost_recorded`는 eligibility가 아니라 branch flavor hook으로만 사용한다고 명시
- [x] stable choice id 후보 `keep_wounded_shelter_until_noon`, `accept_baekdo_medicine_after_roll_call`, `send_word_to_dowol_for_quiet_exit`, `show_archive_map_to_yeon_soha` 고정
- [x] triage/companion death/mass combat/route graph/faction reputation/debt ledger/relation/reward/ability/epilogue schema, return system, 천기록 정체 reveal은 열지 않는다고 명시
- [x] storypack DB JSON mirror와 docs contract를 다음 runtime handoff 기준으로 갱신
- [x] runtime YAML/Rust/Web/generated bundle, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지

### 0.3 완료 기준

- [x] README 또는 인덱스만 보고 프로젝트 방향을 이해할 수 있다.
- [x] 개발자가 어떤 문서에 무엇을 써야 하는지 알 수 있다.
- [x] 공개 문서와 비공개 현실 위치 정보의 경계가 명확하다.

### 0.4 1차 콘텐츠 초안 문서

- [x] `docs/content/Location_List.md` 생성
- [x] `docs/content/Item_List.md` 생성
- [x] `docs/content/Encounter_List.md` 생성
- [x] `docs/content/Ending_List.md` 생성
- [x] `docs/content/Secret_List.md` 생성
- [x] `docs/content/Horror_Ideas.md` 생성
- [x] `docs/archive/idea_0515.md` 원본 아이디어 노트 보존

## Phase 1: 프로젝트 스캐폴딩과 기술 확정

### 1.1 기술 선택

- [x] Python 버전 결정: Python 3.x
- [x] TUI 라이브러리 결정: Textual
- [x] 데이터 포맷 결정: YAML, 저장 파일은 JSON
- [x] 테스트 프레임워크 결정: pytest
- [x] 패키지/의존성 관리 방식 결정: pyproject.toml 기준

### 1.2 파일 구조

- [x] `pyproject.toml` 생성
- [x] `src/tui_adv/__init__.py` 생성
- [x] `src/tui_adv/main.py` 생성
- [x] `src/tui_adv/game/` 패키지 생성
- [x] `src/tui_adv/tui/` 패키지 생성
- [x] `src/tui_adv/data/` 디렉터리 생성
- [x] `tests/` 디렉터리 생성
- [x] `.gitignore` 생성 또는 갱신
- [x] `private/` 또는 `.local` 비밀 파일 ignore 규칙 추가

### 1.3 실행 검증

- [x] 기본 실행 명령 정의: `PYTHONPATH=src python -m tui_adv --new --seed 123`
- [x] 앱이 초기 상태 smoke 출력을 하며 실행된다.
- [x] `pytest`가 실행된다.
- [x] README에 초기 실행 방법을 기록한다.

## Phase 2: 도메인 모델과 상태 시스템

### 2.1 GameState

- [x] `GameState` 모델 생성
- [x] 현재 위치 필드 추가
- [x] 턴 수 또는 시간 필드 추가
- [x] 위험도 필드 추가
- [x] 인벤토리 필드 추가
- [x] 단서 목록 필드 추가
- [x] 플래그 집합 필드 추가
- [x] 최근 로그 필드 추가

### 2.2 PlayerState 또는 Resources

- [x] 체력 필드 추가
- [x] 정신력 필드 추가
- [x] 배터리 필드 추가
- [x] 허기 필드 추가
- [x] 갈증 필드 추가
- [x] 능력치 필드 추가
- [x] 0-100 범위 clamp 구현
- [x] 0-6 능력치 clamp 구현
- [x] 상태 변경 함수 구현
- [x] 턴 경과 함수 구현

### 2.3 임계치 효과

- [x] 체력 0 실패 상태 정의
- [x] 정신력 0 실패 상태 정의
- [x] 허기 한계 실패 또는 지속 피해 정의
- [x] 갈증 한계 실패 또는 지속 피해 정의
- [x] 배터리 0일 때 제한되는 행동 정의
- [x] 정신력 낮음 선택지 왜곡 규칙 초안 구현 또는 문서화
- [x] 갈증 높음 환각 이벤트 규칙 초안 구현 또는 문서화

### 2.4 테스트

- [x] 상태 기본값 테스트
- [x] 상태 clamp 테스트
- [x] 턴 경과 시 허기/갈증 변화 테스트
- [x] 임계치 계산 테스트
- [x] 실패 상태 판정 테스트

## Phase 3: 위치, 이동, 기본 게임 루프

### 3.1 위치 모델

- [x] Location 모델 정의
- [x] 위치 id/name/description 필드 정의
- [x] 인접 위치 목록 정의
- [x] 위험도 보정 또는 태그 필드 정의
- [x] 위치별 가능한 인카운터 태그 정의

### 3.2 1차 위치

- [x] 내 자리
- [x] 개발팀 사무실
- [x] 복도
- [x] 탕비실
- [x] 회의실
- [x] 복합기 구역
- [x] 서버실 앞
- [x] 비상계단

### 3.3 이동과 턴

- [x] 현재 위치에서 가능한 이동 선택지 생성
- [x] 이동 선택 적용
- [x] 이동 시 턴 증가
- [x] 이동 시 허기/갈증 변화
- [x] 이동 로그 기록
- [x] 위험도 변화 규칙 적용

### 3.4 테스트

- [x] 시작 위치 테스트
- [x] 인접 위치 이동 테스트
- [x] 연결되지 않은 위치 이동 불가 테스트
- [x] 이동 시 턴 증가 테스트
- [x] 이동 로그 테스트

## Phase 4: 인카운터와 선택지 엔진

### 4.1 모델

- [x] Encounter 모델 정의
- [x] Choice 모델 정의
- [x] Condition 모델 또는 조건 검사 함수 정의
- [x] Effect 모델 또는 결과 적용 함수 정의
- [x] Outcome 모델 정의
- [x] AbilityCheck 모델 정의
- [x] 현재 상태 기반 인카운터 선택 함수 정의

### 4.2 조건과 비용

- [x] 자원 최소/최대 조건 검사
- [x] 아이템 보유 조건 검사
- [x] 플래그 조건 검사
- [x] 위치 조건 검사
- [x] 재난 타입 조건 검사
- [x] 선택 비용 적용
- [x] 비용 부족 시 선택 불가 처리
- [x] 능력치 기반 선택지 조건 검사

### 4.3 결과 적용

- [x] 체력 변화
- [x] 정신력 변화
- [x] 배터리 변화
- [x] 허기 변화
- [x] 갈증 변화
- [x] 아이템 추가/제거
- [x] 단서 추가
- [x] 플래그 추가/제거
- [x] 위치 이동
- [x] 위험도 변화
- [x] 로그 추가

### 4.4 샘플 인카운터

- [x] 퇴사자의 메신저
- [x] 복합기가 혼자 출력한다
- [x] 탕비실 커피머신

### 4.5 테스트

- [x] 조건 만족 선택지만 표시되는지 테스트
- [x] 비용 적용 테스트
- [x] 아이템/플래그 결과 테스트
- [x] 성공/실패 분기 테스트
- [x] 현재 상태 기반 인카운터 선택 테스트
- [x] 능력치 기반 선택지 노출 테스트
- [x] 인카운터 후 로그 테스트
- [x] CLI 한 턴 인카운터 표시 테스트
- [x] CLI 선택지 실행 테스트
- [x] 선택지 판정 결과 포맷터 테스트

## Phase 5: 데이터 파일 분리

### 5.1 스키마 문서

- [x] `docs/dev/Data_Schema.md` 작성
- [x] Location 스키마 정의
- [x] Item 스키마 정의
- [x] Encounter 스키마 정의
- [x] Choice 스키마 정의
- [x] Ending 스키마 정의
- [x] Secret 힌트 스키마 정의
- [x] Achievement 스키마 정의

### 5.2 데이터 파일

- [x] `src/tui_adv/data/locations.yaml` 생성
- [x] `src/tui_adv/data/items.yaml` 생성
- [x] `src/tui_adv/data/encounters.yaml` 생성
- [x] `src/tui_adv/data/endings.yaml` 생성
- [x] `src/tui_adv/data/achievements.yaml` 생성
- [x] `src/tui_adv/data/secrets.example.yaml` 생성
- [x] 실제 위치용 로컬 비공개 secret 파일 경로 결정: `private/secrets.local.yaml`

### 5.3 데이터 로더

- [x] 데이터 로더 구현: `src/tui_adv/game/content.py`
- [x] 필수 필드 로드 경로 구현
- [x] id 중복 방지: dict 변환과 테스트로 보장
- [x] 참조 무결성 검증: `validate_public_content()`
- [x] 오류 메시지 개선: 위치 연결, 저장 파일, secret 안전 검사

### 5.4 테스트

- [x] 정상 데이터 로드 테스트
- [x] 필수 필드/스키마 오류 테스트
- [x] 잘못된 id 참조 테스트
- [x] 중복 id에 준하는 런타임 dict wire 테스트
- [x] 샘플 데이터 기반 한 턴 실행 테스트

## Phase 6: TUI 수직 슬라이스

### 6.1 레이아웃

- [x] Textual Header/Footer와 단일 스크롤 게임 패널 구현
- [x] 위치 섹션 구현
- [x] LOCAL STATUS 섹션 구현
- [x] 현재 인카운터/선택지 섹션 구현
- [x] 현재 이동/아이템 행동 섹션 구현
- [x] 소지품/단서 요약 섹션 구현
- [x] 하단 최근 로그 섹션 구현
- [x] 저장 파일 목록과 시작 화면 섹션 구현
- [x] 압박 경고 패널 구현
- [x] 분리된 Textual 위젯/그리드 패널 스타일링

### 6.2 입력

- [x] 숫자 키 선택 지원
- [x] 이동 단축키 지원
- [x] 도움말 키 구현
- [x] 소지품/단서 상세 키 구현
- [x] 전체 로그 상세 키 구현
- [x] 저장 키 구현
- [x] 종료 키 구현
- [x] 시작 화면 새 게임/저장 슬롯 선택/삭제 입력 구현

### 6.3 표시 규칙

- [x] 체력/정신력/배터리는 높을수록 좋은 상태 라벨로 표시
- [x] 허기/갈증은 높을수록 위험한 상태 라벨로 표시
- [x] 선택 불가 선택지 이유 표시
- [x] 정신력 낮음 상태의 텍스트 왜곡 표시
- [x] 압박 경고 패널 표시
- [x] 긴 로그 스크롤 또는 최근 N개만 표시 결정: 기본 최근 5개, `l` 상세 로그 패널
- [x] Textual CSS 색상 테마 위젯 연결

### 6.4 스모크 테스트

- [x] TUI snapshot 시작 가능: `--tui-smoke`
- [x] 시작 화면 표시
- [x] 선택지 입력 가능
- [x] 한 턴 진행 가능
- [x] 다중 턴 scripted smoke 가능
- [x] 저장/불러오기/삭제 smoke 가능
- [x] 실제 Textual 화면 수동 QA

## Phase 7: 공개 콘텐츠 작성

### 7.1 위치 16개

- [x] 런타임 위치 YAML 16개 작성
- [x] `docs/content/Location_List.md`를 YAML 기준으로 갱신
- [x] `docs/implementation-map/data/content.js` 위치 목록 갱신

### 7.2 아이템 13개

- [x] 생수
- [x] 커피
- [x] 과자
- [x] 컵라면
- [x] 구급상자
- [x] 보조배터리
- [x] 손전등
- [x] 사원증
- [x] 보안실 우회권한
- [x] 구겨진 출력물
- [x] 퇴사자의 메모
- [x] 지하주차장 키태그
- [x] 임시 방문증

### 7.3 인카운터 21개

- [x] 퇴사자의 메신저
- [x] 복합기가 혼자 출력한다
- [x] 탕비실 커피머신
- [x] 정수기의 이상한 물
- [x] 물품창고 비상 보급함
- [x] 물품창고 자동 난투
- [x] 회의실 화이트보드 모서리
- [x] 존재하지 않는 부서의 전체회의
- [x] 지연된 CCTV 화면
- [x] 어긋난 층수의 보안 콘솔
- [x] 비상계단 문틈 표식
- [x] 비상계단 공간 왜곡
- [x] 서버실 문 앞 무전기
- [x] 관리자 콘솔
- [x] 존재하지 않는 층의 엘리베이터
- [x] 옥상의 제한된 외부 신호
- [x] 지하주차장의 시동음
- [x] 지하주차장 차단기
- [x] 무인 로비 안내 키오스크
- [x] 로비 출구 게이트
- [x] 대표실 결재 콘솔

### 7.4 엔딩 13개 + 즉시 실패 2개

- [x] 코드 내 실패 엔딩: 체력 0
- [x] 코드 내 실패 엔딩: 정신력 0
- [x] YAML 실패 엔딩: 비상계단 공간 붕괴
- [x] 탈출 엔딩: 비상계단 퇴근
- [x] 탈출 엔딩: 옥상 외부 신호
- [x] 탈출 엔딩: 지하주차장
- [x] 탈출 엔딩: 로비 회전문
- [x] 히든 현실 연결 엔딩 3개
- [x] 진실 엔딩: 격리 프로토콜
- [x] 정복 엔딩: 사내망 관리자 권한
- [x] 정복 엔딩: 보안 격리 권한 장악
- [x] 정복 엔딩: 사내 방송 장악
- [x] 정복 엔딩: 대표 승인권 장악

### 7.5 검증

- [x] 새 게임에서 모든 핵심 자원을 볼 수 있다.
- [x] 최소 3개 위치를 오갈 수 있다.
- [x] 음식/물 아이템으로 허기/갈증을 조절할 수 있다.
- [x] 배터리를 쓰는 선택지가 있다.
- [x] 정신력에 영향을 주는 선택지가 있다.
- [x] 하나 이상의 엔딩에 도달할 수 있다.
- [x] 탈출/정복/진실/현실 연결 대표 루트 smoke가 있다.

## Phase 8: 저장/불러오기와 랜덤 시드

### 8.1 저장 데이터

- [x] 저장 파일 버전 필드 추가
- [x] 현재 위치 저장
- [x] 턴/시간 저장
- [x] 위험도 저장
- [x] 플레이어 상태 저장
- [x] 아이템 저장
- [x] 단서 저장
- [x] 플래그 저장
- [x] 랜덤 시드 저장
- [x] 본 인카운터와 해금 업적 저장
- [x] 로그 저장

### 8.2 기능

- [x] 저장하기 구현
- [x] 불러오기 구현
- [x] 저장 파일 목록 보기 구현
- [x] 저장 파일 삭제 구현
- [x] 저장 파일 손상 시 오류 처리
- [x] 버전 불일치 처리 방식 결정

### 8.3 테스트

- [x] 저장 파일 생성 테스트
- [x] 로드 후 상태 동일성 테스트
- [x] 랜덤 시드 재현 테스트
- [x] 잘못된 저장 파일 오류 테스트
- [x] 저장 슬롯 목록/선택/삭제 테스트

## Phase 9: 엔딩 루트 확장

### 9.1 탈출 루트

- [x] 비상계단 탈출 루트 강화
- [x] 옥상 구조 루트 추가
- [x] 지하주차장 차량 루트 추가
- [x] 로비 회전문/출구 게이트 루트 추가

### 9.2 정복 루트

- [x] 보안실 접근 조건 설계
- [x] 서버실 권한 획득 설계
- [x] 사내 방송 장악 설계
- [x] 대표실 결재 콘솔 정복 분기 추가
- [x] 생존자 또는 시스템 제압/설득 설계
- [x] 정복 엔딩 작성

### 9.3 진실 루트

- [x] 퇴사자의 로그 체인 설계
- [x] 회의실 반복 이벤트 설계
- [x] 서버 로그 조각 설계
- [x] 재난 원인 문서 설계
- [x] 진실 엔딩 작성

### 9.4 현실 연결 루트

- [x] `Reality_Link.md` 원칙 반영
- [x] 공개 힌트와 비공개 최종 위치 분리
- [x] 첫 번째 현실 메모 루트 구현
- [x] 두 번째 현실 메모 후보 구현
- [x] 세 번째 현실 메모 후보 구현
- [x] 비밀 정보 릴리즈 검사 추가
- [x] 로컬 비공개 secret 템플릿과 안전 점검 문서 추가

### 9.5 현실 탈출 후일담

- [x] 첫 runtime slice 범위 확정: `escape_commute` 단일 엔딩, text-backed `[POST-ESCAPE REPORT]`, 새 schema/kind 없음
- [x] 설계 계약 문서화: `Development_Plan.md`, `Real_Escape_Ending_Branching.md`, `Data_Schema.md`, `Ending_List.md`
- [x] RED 테스트 추가: Python content, Rust `ScenePage`, SuperLightTUI snapshot, Web generated parity
- [x] `escape_commute.text`에 공개-safe 후일담 블록 추가
- [x] Rust/Web generated content bundle 갱신
- [x] targeted GREEN 및 전체 validation matrix 통과

## Phase 10: 밸런싱, QA, 패키징

### 10.1 밸런싱

- [x] 턴당 허기 증가량 조정
- [x] 턴당 갈증 증가량 조정
- [x] 배터리 사용량 조정
- [x] 체력 피해량 조정
- [x] 정신력 피해량 조정
- [x] 음식/물 회복량 조정
- [x] 인카운터 발생률 조정
- [x] 엔딩 도달 난이도 조정

### 10.2 QA

- [x] 새 게임 10회 수동 플레이 기록
- [x] 탈출 엔딩 도달 테스트
- [x] 실패 엔딩 도달 테스트
- [x] 히든 힌트 도달 테스트
- [x] 저장/로드 반복 테스트
- [x] 터미널 크기별 화면 확인
- [x] 키 입력 오류 처리 확인

### 10.3 패키징과 문서

- [x] README 업데이트
- [x] 설치 방법 작성
- [x] 실행 방법 작성
- [x] 조작법 작성
- [x] 게임 컨셉 소개 작성
- [x] 현실 연결 안전 고지 작성
- [x] 릴리즈 전 비밀 정보 검사

## 릴리즈 전 최종 체크

- [x] 공개 저장소에 실제 사무실 최종 위치가 없다.
- [x] 공개 저장소에 개인 이름, 회사 기밀, 고객 정보가 없다.
- [x] `private/` 또는 `.local` 파일이 커밋되지 않았다.
- [x] 모든 테스트가 통과한다.
- [x] README만 보고 실행할 수 있다.
- [x] 최소 하나의 정상 엔딩에 도달할 수 있다.
- [x] 최소 하나의 실패 엔딩에 도달할 수 있다.
- [x] 히든 힌트 루트가 안전한 위치만 안내한다.

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

### 0.2ap 2026-06-02 무협 `wuxia_wounded_shelter_dawn_offers` preview runtime slice

- [x] `wuxia_wounded_shelter_dawn_offers`를 `stabilize_wounded_until_dawn` deferred branch 뒤에 schema-less 제안 encounter로 추가
- [x] start conditions를 `cheongryu_raid_wounded_fallback_resolved` + `route_commitment_deferred` + `deferred_route_reopened` + `wounded_shelter_stabilized`로 고정하고 `survivor_roll_call_complete`/`route_delay_cost_recorded`는 flavor hook으로만 유지
- [x] stable choice id `keep_wounded_shelter_until_noon`, `accept_baekdo_medicine_after_roll_call`, `send_word_to_dowol_for_quiet_exit`, `show_archive_map_to_yeon_soha` 구현
- [x] 모든 outcome이 `wounded_shelter_dawn_offers_resolved`, `route_commitment_reopened`, `destination_id: cheongryu_outer_courtyard` bridge를 남기도록 구현
- [x] route 선택 outcome이 정파/사파/천기 opener가 읽는 starter flags를 남기도록 구현
- [x] presentation을 `visual_id: wuxia_wounded_shelter_dawn_offers`, `speaker: 서하린`, `layout: deferred_route_offer`, stable terms `새벽 / 부상자 / 제안`으로 고정
- [x] Rust fixture와 Web generated preview bundle 재생성
- [x] Python exporter, Rust content fixture, WASM JSON boundary, SuperLightTUI deferred-offer parity, Web content bundle tests 추가
- [x] triage/companion death/mass combat/route graph/faction reputation/debt ledger/relation/reward/ability/epilogue schema, return system, 천기록 정체 reveal, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `route_midgame_continuity_after_wounded_shelter` docs-only handoff로 갱신

### 0.2aq 2026-06-02 무협 post-opener midgame continuity docs-only handoff

- [x] route별 midgame card 3개, 공통 midgame bridge, deferred-offer 후속 bridge, 무명 첫 대치, boss first appearance를 Notion reference와 repo hooks 기준으로 비교
- [x] 다음 runtime 후보를 common midgame bridge `wuxia_mumyeong_first_sighting`로 결정
- [x] Notion 사건 카드 DB `wuxia_mumyeong_first_sighting` / `무명 첫 목격`을 직접 대조하고, 첫 대치/중반 재회는 후속으로 보류
- [x] 새 any-of condition schema 대신 세 route opener outcome에 공통 `route_opener_resolved` flag를 추가하는 구현 방향 문서화
- [x] start conditions를 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [route_opener_resolved, cheongryu_raid_survived, cheongryu_trial_started, first_fragment_seen]`, `forbidden_flags: [mumyeong_first_sighting_resolved]`로 문서화
- [x] route-specific opened flags는 eligibility가 아니라 branch flavor hook으로만 사용한다고 명시
- [x] stable choice id 후보 `watch_the_stolen_qingliu_flow`, `check_seo_harin_silence`, `follow_black_serpent_runner`, `pretend_not_to_see_the_form` 고정
- [x] boss combat/combat schema/route graph/faction reputation/relation/debt/reward/ability/epilogue/return schema와 천기록 정체 reveal은 열지 않는다고 명시
- [x] storypack DB JSON mirror와 docs contract를 다음 runtime handoff 기준으로 갱신
- [x] runtime YAML/Rust/Web/generated bundle, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지

### 0.2ar 2026-06-02 무협 `wuxia_mumyeong_first_sighting` preview runtime slice

- [x] 세 route opener `wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`의 모든 choice outcome에 공통 `route_opener_resolved` flag 추가
- [x] `wuxia_mumyeong_first_sighting` encounter를 storypack preview source에 추가
- [x] start conditions를 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [route_opener_resolved, cheongryu_raid_survived, cheongryu_trial_started, first_fragment_seen]`, `forbidden_flags: [mumyeong_first_sighting_resolved]`로 구현
- [x] stable choice id `watch_the_stolen_qingliu_flow`, `check_seo_harin_silence`, `follow_black_serpent_runner`, `pretend_not_to_see_the_form` 구현
- [x] common hook `mumyeong_first_sighting_resolved`, `midgame_continuity_started`, `destination_id: cheongryu_outer_courtyard` 구현
- [x] 무명 존재/카피 무공/서하린 침묵 clue와 `midgame_rival_sighting` presentation hook 구현
- [x] Rust/Web storypack preview bundle만 재생성하고 기본 office bundle/generated content는 미변경 유지
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web content bundle registry 테스트 갱신
- [x] any-of condition, route graph/faction reputation/debt/relation/combat/reward/ability/epilogue/return schema, boss first appearance, 무명 첫 대치, 천기록 정체 reveal 미변경 유지
- [x] 다음 작업을 `wuxia_mumyeong_first_confrontation_after_sighting` docs-only handoff로 갱신

### 0.2as 2026-06-02 무협 rival confrontation docs-only handoff

- [x] Notion 사건 카드 DB `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`를 직접 대조
- [x] Notion 운영 문서 `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `06. 사이드 퀘스트와 미해결 부채`, `07. 천기록 / 천외편린 보상`, `99. 통합 체크포인트`와 repo hook을 대조
- [x] 다음 runtime 후보를 `wuxia_mumyeong_first_confrontation`로 결정
- [x] `wuxia_mumyeong_midgame_reunion`은 첫 대치와 과거 단서 일부 이후로 보류
- [x] `wuxia_boss_first_appearance`는 boss-wall/final logic 압박이 커서 보류
- [x] start conditions를 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [mumyeong_first_sighting_resolved, midgame_continuity_started, cheongryu_raid_survived, first_fragment_seen]`, `forbidden_flags: [mumyeong_first_confrontation_resolved]`로 문서화
- [x] stable choice id 후보 `meet_mumyeong_head_on`, `endure_until_copy_flow_breaks`, `watch_seo_harin_hold_back`, `read_mumyeongs_copied_form`, `do_not_provoke_mumyeong` 고정
- [x] 첫 대치는 승리 판정이 아니라 버티기/관찰/분석 encounter로 구현한다고 명시
- [x] combat resolver/schema, HP 숫자전, route graph/faction reputation/debt/relation/reward/ability/epilogue/return schema, boss first appearance, 천기록 정체 reveal은 열지 않는다고 명시
- [x] storypack DB JSON mirror와 docs contract를 다음 runtime handoff 기준으로 갱신
- [x] runtime YAML/Rust/Web/generated bundle, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_mumyeong_first_confrontation` runtime implementation으로 갱신

### 0.2at 2026-06-02 무협 `wuxia_mumyeong_first_confrontation` preview runtime slice

- [x] `wuxia_mumyeong_first_confrontation` encounter를 `wuxia_mumyeong_first_sighting` 뒤에 storypack preview source로 추가
- [x] start conditions를 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [mumyeong_first_sighting_resolved, midgame_continuity_started, cheongryu_raid_survived, first_fragment_seen]`, `forbidden_flags: [mumyeong_first_confrontation_resolved]`로 구현
- [x] stable choice id `meet_mumyeong_head_on`, `endure_until_copy_flow_breaks`, `watch_seo_harin_hold_back`, `read_mumyeongs_copied_form`, `do_not_provoke_mumyeong` 구현
- [x] common hook `mumyeong_first_confrontation_resolved`, `mumyeong_rival_thread_opened`, `destination_id: cheongryu_outer_courtyard` 구현
- [x] 카피 무공 결함/승리 불필요/서하린 침묵/청류안 대비 clue와 `rival_first_confrontation` presentation hook 구현
- [x] Rust/Web storypack preview bundle만 재생성하고 기본 office bundle/generated content는 미변경 유지
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web content bundle registry 테스트 갱신
- [x] combat resolver/schema, HP 숫자전, boss first appearance, 무명 중반 재회, route graph/faction reputation/debt/relation/reward/ability/epilogue/return schema, 천기록 정체 reveal 미변경 유지
- [x] 다음 작업을 `wuxia_mumyeong_followup_after_first_confrontation` docs-only handoff로 갱신

### 0.2au 2026-06-02 무협 post-confrontation follow-up docs-only handoff

- [x] Notion 사건 카드 DB `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance` 재확인
- [x] `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `06. 사이드 퀘스트와 미해결 부채`, `07. 천기록 / 천외편린 보상`, `99. 통합 체크포인트`와 비교
- [x] 다음 runtime 후보를 `wuxia_mumyeong_copy_style_reveal`로 결정
- [x] `wuxia_mumyeong_midgame_reunion`은 무명 과거 단서 일부가 더 필요해 보류
- [x] `wuxia_boss_first_appearance`는 boss-wall/final logic 압박이 커서 보류
- [x] start conditions를 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [mumyeong_first_confrontation_resolved, mumyeong_rival_thread_opened, midgame_continuity_started]`, `forbidden_flags: [mumyeong_copy_style_reveal_resolved]`로 문서화
- [x] stable choice id 후보 `read_the_stolen_blade_path`, `watch_mumyeongs_footwork`, `listen_for_breath_mismatch`, `wait_for_body_to_shudder` 고정
- [x] seed 기반 random copy-style system/table 없이 flags/clues/log/presentation으로 카피 계열 윤곽만 먼저 구현한다고 명시
- [x] 천외편린 3택 reward/ability schema, combat resolver/schema, boss first appearance, 무명 중반 재회, route graph/faction reputation/debt/relation/epilogue/return schema, 천기록 정체 reveal은 열지 않는다고 명시
- [x] storypack DB JSON mirror와 docs contract를 다음 runtime handoff 기준으로 갱신
- [x] runtime YAML/Rust/Web/generated bundle, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_mumyeong_copy_style_reveal` runtime implementation으로 갱신

### 0.2av 2026-06-02 무협 `wuxia_mumyeong_copy_style_reveal` preview runtime slice

- [x] `wuxia_mumyeong_copy_style_reveal` encounter를 `wuxia_mumyeong_first_confrontation` 뒤에 storypack preview source로 추가
- [x] start conditions를 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [mumyeong_first_confrontation_resolved, mumyeong_rival_thread_opened, midgame_continuity_started]`, `forbidden_flags: [mumyeong_copy_style_reveal_resolved]`로 구현
- [x] stable choice id `read_the_stolen_blade_path`, `watch_mumyeongs_footwork`, `listen_for_breath_mismatch`, `wait_for_body_to_shudder` 구현
- [x] common hook `mumyeong_copy_style_reveal_resolved`, `copy_style_hint_recorded`, `destination_id: cheongryu_outer_courtyard` 구현
- [x] 카피 계열 윤곽/호흡 불일치/겉흐름 복사/천외편린 후보 변형 clue와 `copy_style_analysis` presentation hook 구현
- [x] Rust/Web storypack preview bundle만 재생성하고 legacy office bundle/generated content는 미변경 유지
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web content bundle registry 테스트 갱신
- [x] seed 기반 random copy-style system/table, combat resolver/schema, HP 숫자전, boss first appearance, 무명 중반 재회, 무명 과거 진실 reveal, route graph/faction reputation/debt/relation/reward/ability/epilogue/return schema, 천기록 정체 reveal 미변경 유지
- [x] 다음 작업을 `wuxia_mumyeong_followup_after_copy_style_reveal` docs-only handoff로 갱신

### 0.2aw 2026-06-02 무협 post-copy-style follow-up docs-only handoff

- [x] Notion 사건 카드 DB `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_mumyeong_reads_orthodox_style` 재확인
- [x] `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `07. 천기록 / 천외편린 보상`, `99. 통합 체크포인트`와 비교
- [x] 다음 runtime 후보를 `wuxia_mumyeong_reads_orthodox_style`로 결정
- [x] `wuxia_mumyeong_midgame_reunion`은 무명 과거 단서 일부가 아직 부족해 보류
- [x] `wuxia_boss_first_appearance`는 boss-wall/final logic 압박이 커서 보류
- [x] `wuxia_mumyeong_departure_truth_summary`는 후반 truth reveal/구원 조건 확정 범위가 커서 보류
- [x] start conditions를 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [mumyeong_copy_style_reveal_resolved, copy_style_hint_recorded, midgame_continuity_started, first_fragment_seen]`, `forbidden_flags: [mumyeong_reads_orthodox_style_resolved]`로 문서화
- [x] stable choice id 후보 `compare_copied_form_to_old_wound`, `trace_qingliu_eye_variation`, `reconstruct_mumyeongs_sightline`, `stop_before_truth_becomes_accusation` 고정
- [x] 현악문/복호금쇄수는 최신 확정명으로 사용하되 무명 이탈 진실 전체 reveal은 열지 않는다고 명시
- [x] 천외편린 3택 reward/ability schema, combat resolver/schema, boss first appearance, 무명 중반 재회, route graph/faction reputation/debt/relation/epilogue/return schema, 천기록 정체 reveal은 열지 않는다고 명시
- [x] storypack DB JSON mirror와 docs contract를 다음 runtime handoff 기준으로 갱신
- [x] runtime YAML/Rust/Web/generated bundle, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_mumyeong_reads_orthodox_style` runtime implementation으로 갱신

### 0.2ax 2026-06-02 무협 `wuxia_mumyeong_reads_orthodox_style` preview runtime slice

- [x] `wuxia_mumyeong_reads_orthodox_style` encounter를 `wuxia_mumyeong_copy_style_reveal` 뒤에 storypack preview source로 추가
- [x] start conditions를 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [mumyeong_copy_style_reveal_resolved, copy_style_hint_recorded, midgame_continuity_started, first_fragment_seen]`, `forbidden_flags: [mumyeong_reads_orthodox_style_resolved]`로 구현
- [x] stable choice id `compare_copied_form_to_old_wound`, `trace_qingliu_eye_variation`, `reconstruct_mumyeongs_sightline`, `stop_before_truth_becomes_accusation` 구현
- [x] common hook `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `destination_id: cheongryu_outer_courtyard` 구현
- [x] 현악문/복호금쇄수/무명 시야 변주/정파식 통제 무공 clue와 `orthodox_style_trace` presentation hook 구현
- [x] Rust/Web storypack preview bundle만 재생성하고 legacy office bundle/generated content는 미변경 유지
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web content bundle registry 테스트 갱신
- [x] 무명 중반 재회, 보스 첫 등장, 무명 이탈 진실 정리, 청류문 습격 full flashback, seed 기반 random copy-style system/table, combat resolver/schema, HP 숫자전, route graph/faction reputation/debt/relation/reward/ability/epilogue/return schema, 천기록 정체 reveal 미변경 유지
- [x] 다음 작업을 `wuxia_mumyeong_followup_after_orthodox_style_trace` docs-only handoff로 갱신

### 0.2ay 2026-06-02 terminal default storypack 전환

- [x] `escape-terminal --scene content`가 bundle 인자 없이 `wuxia_jianghu_pack` / **이구학지 — 천기록** built-in fixture를 기본 선택하도록 변경
- [x] `--storypack-preview wuxia_jianghu_pack`는 명시적 동일 경로로 유지
- [x] legacy office fixture는 `--content-bundle` 명시 경로로 유지
- [x] terminal default smoke 테스트와 README/plan/runtime preview 문서 동기화

### 0.2az 2026-06-02 무협 orthodox-style follow-up docs-only handoff

- [x] Notion 사건 카드 DB `wuxia_mumyeong_midgame_reunion`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_boss_first_appearance`, `wuxia_qingliu_attack_after_war` 재확인
- [x] `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `07. 천기록 / 천외편린 보상`, `99. 통합 체크포인트`와 비교
- [x] 다음 runtime 후보를 `wuxia_mumyeong_midgame_reunion`으로 결정
- [x] `wuxia_mumyeong_departure_truth_summary`는 후반 truth reveal/서하린 진실 전달/무명 구원 조건 확정 범위가 커서 보류
- [x] `wuxia_boss_first_appearance`는 boss-wall/final logic 압박과 boss combat 기대가 커져 보류
- [x] `wuxia_qingliu_attack_after_war`는 full flashback/backstory reveal 범위가 커서 보류
- [x] start conditions를 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [mumyeong_reads_orthodox_style_resolved, orthodox_style_trace_recorded, mumyeong_first_confrontation_resolved, mumyeong_rival_thread_opened]`, `forbidden_flags: [mumyeong_midgame_reunion_resolved]`로 문서화
- [x] stable choice id 후보 `ask_why_seoharin_never_called_him_traitor`, `show_the_hyeonakmun_trace_without_accusing`, `point_out_the_copied_form_gap`, `keep_blades_low_and_watch_his_answer` 고정
- [x] common hook `mumyeong_midgame_reunion_resolved`, `mumyeong_mirror_thread_deepened`, `destination_id: cheongryu_outer_courtyard` 문서화
- [x] 무명 중반 재회는 라이벌/거울 관계와 부분 단서만 다루고, 무명 이탈 진실 전체 reveal/구원 확정/서하린에게 진실 전달은 열지 않는다고 명시
- [x] 천외편린 3택 reward/ability schema, combat resolver/schema, boss first appearance, full flashback, route graph/faction reputation/debt/relation/epilogue/return schema, 천기록 정체 reveal은 열지 않는다고 명시
- [x] storypack DB JSON mirror와 docs contract를 다음 runtime 후보 기준으로 갱신
- [x] runtime YAML/Rust/Web generated bundle, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_mumyeong_midgame_reunion` runtime implementation으로 갱신

### 0.2ba 2026-06-02 무협 `wuxia_mumyeong_midgame_reunion` preview runtime slice

- [x] `wuxia_mumyeong_midgame_reunion` encounter를 `wuxia_mumyeong_reads_orthodox_style` 뒤에 storypack preview source로 추가
- [x] start conditions를 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [mumyeong_reads_orthodox_style_resolved, orthodox_style_trace_recorded, mumyeong_first_confrontation_resolved, mumyeong_rival_thread_opened]`, `forbidden_flags: [mumyeong_midgame_reunion_resolved]`로 구현
- [x] stable choice id `ask_why_seoharin_never_called_him_traitor`, `show_the_hyeonakmun_trace_without_accusing`, `point_out_the_copied_form_gap`, `keep_blades_low_and_watch_his_answer` 구현
- [x] common hook `mumyeong_midgame_reunion_resolved`, `mumyeong_mirror_thread_deepened`, `destination_id: cheongryu_outer_courtyard` 구현
- [x] 서하린 침묵, 현악문 흔적 공유, 보스가 무명의 상처를 이용했다는 clue와 `rival_reunion_trace` presentation hook 구현
- [x] Rust/Web storypack preview bundle만 재생성하고 legacy office bundle/generated content는 미변경 유지
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web default content bundle registry 테스트 갱신
- [x] 무명 이탈 진실 전체 reveal, 보스 첫 등장/전투, 청류문 습격 full flashback, 서하린에게 진실 전달, 구원 확정, seed 기반 random copy-style system/table, combat resolver/schema, route graph/faction reputation/debt/relation/reward/ability/epilogue/return schema, 천기록 정체 reveal 미변경 유지
- [x] 다음 작업을 `wuxia_mumyeong_followup_after_midgame_reunion` docs-only handoff로 갱신

### 0.2bb 2026-06-02 무협 midgame-reunion follow-up docs-only handoff

- [x] Notion 사건 카드 DB `wuxia_mumyeong_departure_truth_summary`, `wuxia_boss_first_appearance`, `wuxia_qingliu_attack_after_war` 재확인
- [x] Notion search에서 `wuxia_mumyeong_request_for_aid` bridge 후보를 확인했지만, repo 현 handoff 후보 밖 future bridge로 보류
- [x] `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `07. 천기록 / 천외편린 보상`, `99. 통합 체크포인트`와 비교
- [x] 다음 runtime 후보를 `wuxia_boss_first_appearance`로 결정
- [x] `wuxia_mumyeong_departure_truth_summary`는 후반 truth reveal/서하린 진실 전달/무명 구원 조건 확정 범위가 커서 보류
- [x] `wuxia_qingliu_attack_after_war`는 full flashback/backstory reveal 범위가 커서 보류
- [x] start conditions를 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [mumyeong_midgame_reunion_resolved, mumyeong_mirror_thread_deepened, cheongryu_raid_survived, midgame_continuity_started]`, `forbidden_flags: [boss_first_appearance_resolved]`로 문서화
- [x] stable choice id 후보 `read_the_boss_flow_and_fail_to_move`, `pull_seo_harin_behind_broken_gate`, `watch_mumyeong_answer_the_boss`, `retreat_before_the_second_step` 고정
- [x] 보스 첫 등장은 combat/final resolution이 아니라 압도감, 조직력, 약점 읽기, 무명이 따르는 이유를 flags/clues/log/presentation으로만 표현한다고 명시
- [x] boss combat/final boss resolution, 무명 이탈 진실 전체 reveal, 청류문 습격 full flashback, 서하린에게 진실 전달, seed 기반 random copy-style system/table, combat resolver/schema, route graph/faction reputation/debt/relation/reward/ability/epilogue/return schema, 천기록 정체 reveal은 열지 않는다고 명시
- [x] storypack DB JSON mirror와 docs contract를 다음 runtime 후보 기준으로 갱신
- [x] runtime YAML/Rust/Web generated bundle, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_boss_first_appearance` runtime implementation으로 갱신

### 0.2bc 2026-06-02 무협 `wuxia_boss_first_appearance` preview runtime slice

- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_mumyeong_midgame_reunion` 뒤에 `wuxia_boss_first_appearance` 추가
- [x] start conditions를 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [mumyeong_midgame_reunion_resolved, mumyeong_mirror_thread_deepened, cheongryu_raid_survived, midgame_continuity_started]`, `forbidden_flags: [boss_first_appearance_resolved]`로 구현
- [x] stable choice id `read_the_boss_flow_and_fail_to_move`, `pull_seo_harin_behind_broken_gate`, `watch_mumyeong_answer_the_boss`, `retreat_before_the_second_step` 구현
- [x] 모든 선택지가 `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `destination_id: cheongryu_outer_courtyard` bridge를 남기도록 구현
- [x] `wuxia_boss_first_appearance` presentation을 `speaker: 흑사방주`, `layout: boss_wall_pressure`, stable terms `[흑사방주, 무명, 청류문]`로 구현
- [x] Rust/Web storypack preview generated bundle 재생성
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web default content bundle registry 테스트 갱신
- [x] 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key 미변경 유지
- [x] boss combat/final boss resolution, 무명 이탈 진실 전체 reveal, 청류문 습격 full flashback, 서하린에게 진실 전달, 구원 확정, seed 기반 random copy-style system/table, combat resolver/schema, route graph/faction reputation/debt/relation/reward/ability/epilogue/return schema, 천기록 정체 reveal 미변경 유지
- [x] 다음 작업을 `wuxia_boss_followup_after_first_appearance` docs-only handoff로 갱신

### 0.2bd 2026-06-02 무협 boss follow-up docs-only handoff

- [x] Notion 사건 카드 DB `wuxia_mumyeong_request_for_aid`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_qingliu_attack_after_war`, `wuxia_boss_resolution` 재확인
- [x] `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `06. 사이드 퀘스트와 미해결 부채`, `07. 천기록 / 천외편린 보상`, `99. 통합 체크포인트`와 비교
- [x] 다음 runtime 후보를 `wuxia_mumyeong_request_for_aid`로 결정
- [x] 보스 첫 등장 뒤 “왜 보스의 힘 논리가 무명에게 먹혔는가”를 failed-aid records bridge로 준비한다고 문서화
- [x] `wuxia_mumyeong_departure_truth_summary`는 후반 truth reveal/서하린 진실 전달/무명 구원 조건 확정 범위가 커서 보류
- [x] `wuxia_qingliu_attack_after_war`는 full flashback/backstory reveal 범위가 커서 보류
- [x] `wuxia_boss_resolution`은 final boss resolution/epilogue 범위가 커서 보류
- [x] start conditions를 `conditions.locations: [cheongryu_outer_courtyard]`, `required_flags: [boss_first_appearance_resolved, boss_wall_thread_opened, black_serpent_core_pressure_opened, mumyeong_mirror_thread_deepened, orthodox_style_trace_recorded, midgame_continuity_started]`, `forbidden_flags: [mumyeong_request_for_aid_resolved]`로 문서화
- [x] stable choice id 후보 `search_the_rejected_aid_letters`, `follow_old_inn_rumors_about_mumyeong`, `ask_seo_harin_what_help_never_came`, `keep_the_failed_aid_record_unshown` 고정
- [x] common hook `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `destination_id: cheongryu_outer_courtyard` 문서화
- [x] storypack DB JSON mirror와 docs contract를 다음 runtime 후보 기준으로 갱신
- [x] runtime YAML/Rust/Web generated bundle, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_mumyeong_request_for_aid` runtime implementation으로 갱신

### 0.2be 2026-06-02 무협 `wuxia_mumyeong_request_for_aid` preview runtime slice

- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_boss_first_appearance` 뒤에 `wuxia_mumyeong_request_for_aid` 추가
- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/items.yaml`에 `rejected_aid_letter_fragment` clue item 추가
- [x] start conditions를 `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `mumyeong_mirror_thread_deepened`, `orthodox_style_trace_recorded`, `midgame_continuity_started`로 구현
- [x] stable choice id `search_the_rejected_aid_letters`, `follow_old_inn_rumors_about_mumyeong`, `ask_seo_harin_what_help_never_came`, `keep_the_failed_aid_record_unshown` 구현
- [x] 모든 선택지가 `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `destination_id: cheongryu_outer_courtyard` bridge를 남기도록 구현
- [x] `wuxia_mumyeong_request_for_aid` presentation을 `speaker: 천기록`, `layout: failed_aid_records`, stable terms `[무명, 청류문, 정파]`로 구현
- [x] Rust/Web storypack preview generated bundle 재생성
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web default content bundle registry 테스트 갱신
- [x] 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key 미변경 유지
- [x] truth reveal, full Qingliu flashback, boss combat/final boss resolution, 무명 구원 확정, 서하린에게 진실 전달, seed 기반 random copy-style system/table, combat resolver/schema, route graph/faction reputation/debt/relation/reward/ability/epilogue/return schema, 천기록 정체 reveal 미변경 유지
- [x] 다음 작업을 `wuxia_mumyeong_followup_after_failed_aid` docs-only handoff로 갱신

### 0.2bf 2026-06-02 무협 failed-aid follow-up docs-only handoff

- [x] Notion 사건 카드 DB `wuxia_mumyeong_awakening`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_qingliu_attack_after_war`, `wuxia_boss_resolution`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_destroys_orthodox_sect` 재확인
- [x] `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `07. 천기록 / 천외편린 보상`, `99. 통합 체크포인트`와 비교
- [x] 다음 runtime 후보를 `wuxia_mumyeong_awakening`으로 결정
- [x] `wuxia_mumyeong_awakening`은 failed-aid 기록과 정파 무공 흔적을 무명의 분노/카피 무공 변질로 잇는 중반 bridge라고 문서화
- [x] `wuxia_mumyeong_departure_truth_summary`는 후반 truth reveal/서하린 진실 전달/무명 구원 조건 확정 범위가 커서 보류
- [x] `wuxia_qingliu_attack_after_war`는 full flashback/backstory reveal 범위가 커서 보류
- [x] `wuxia_boss_resolution`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_destroys_orthodox_sect`는 후반/final consequence 범위가 커서 보류
- [x] start conditions를 `required_flags: [mumyeong_request_for_aid_resolved, mumyeong_failed_aid_thread_opened, orthodox_hypocrisy_thread_opened, mumyeong_reads_orthodox_style_resolved, orthodox_style_trace_recorded, mumyeong_copy_style_reveal_resolved, copy_style_hint_recorded, midgame_continuity_started]`, `forbidden_flags: [mumyeong_awakening_resolved]`로 문서화
- [x] stable choice id 후보 `compare_anger_to_copied_flow`, `trace_awakening_from_failed_aid`, `ask_what_the_copy_cost_him`, `stop_before_calling_it_salvation` 고정
- [x] storypack DB JSON mirror와 docs contract를 다음 runtime 후보 기준으로 갱신
- [x] runtime YAML/Rust/Web generated bundle, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_mumyeong_awakening` runtime implementation으로 갱신

### 0.2bg 2026-06-02 무협 `wuxia_mumyeong_awakening` preview runtime slice

- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_mumyeong_request_for_aid` 뒤에 `wuxia_mumyeong_awakening` 추가
- [x] start conditions를 `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `mumyeong_copy_style_reveal_resolved`, `copy_style_hint_recorded`, `midgame_continuity_started`로 구현
- [x] stable choice id `compare_anger_to_copied_flow`, `trace_awakening_from_failed_aid`, `ask_what_the_copy_cost_him`, `stop_before_calling_it_salvation` 구현
- [x] 모든 선택지가 `mumyeong_awakening_resolved`, `mumyeong_awakening_thread_opened`, `copy_corruption_thread_opened`, `destination_id: cheongryu_outer_courtyard` bridge를 남기도록 구현
- [x] `wuxia_mumyeong_awakening` presentation을 `speaker: 천기록`, `layout: anger_copy_bloom`, stable terms `[무명, 카피, 분노]`로 구현
- [x] Rust/Web storypack preview generated bundle 재생성
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web default content bundle registry 테스트 갱신
- [x] 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key 미변경 유지
- [x] truth reveal, full Qingliu flashback, 정파 문파 멸문, boss recruit/final boss resolution, 무명 구원 확정, 서하린에게 진실 전달, seed 기반 random copy-style system/table, combat resolver/schema, route graph/faction reputation/debt/relation/reward/ability/epilogue/return schema, 천기록 정체 reveal 미변경 유지
- [x] 다음 작업을 `wuxia_mumyeong_followup_after_awakening` docs-only handoff로 갱신

### 0.2bh 2026-06-02 무협 awakening follow-up docs-only handoff

- [x] Notion 사건 카드 DB `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_qingliu_attack_after_war`, `wuxia_mumyeong_resolution` 재확인
- [x] future guardrail source `wuxia_boss_resolution`, `wuxia_seoharin_empty_place`, `wuxia_mumyeong_request_for_aid` 확인
- [x] `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `07. 천기록 / 천외편린 보상`, `99. 통합 체크포인트`와 비교
- [x] 다음 runtime 후보를 `wuxia_qingliu_attack_after_war`로 결정
- [x] `wuxia_qingliu_attack_after_war`는 full flashback이 아니라 현악문/복호금쇄수 흔적 조사로 제한한다고 문서화
- [x] `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`는 후반/final/truth/salvation 범위가 커서 보류
- [x] start conditions를 `required_flags: [mumyeong_awakening_resolved, mumyeong_awakening_thread_opened, copy_corruption_thread_opened, mumyeong_request_for_aid_resolved, mumyeong_failed_aid_thread_opened, orthodox_hypocrisy_thread_opened, mumyeong_reads_orthodox_style_resolved, orthodox_style_trace_recorded, midgame_continuity_started]`, `forbidden_flags: [qingliu_attack_after_war_resolved]`로 문서화
- [x] stable choice id 후보 `inspect_bokho_lock_scars`, `compare_hyeonakmun_trace_to_qingliu_wounds`, `ask_seo_harin_what_she_saw_afterward`, `stop_before_replaying_the_attack` 고정
- [x] storypack DB JSON mirror와 docs contract를 다음 runtime 후보 기준으로 갱신
- [x] runtime YAML/Rust/Web generated bundle, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_qingliu_attack_after_war` runtime implementation으로 갱신

### 0.2bi 2026-06-02 무협 `wuxia_qingliu_attack_after_war` preview runtime slice

- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_mumyeong_awakening` 뒤에 `wuxia_qingliu_attack_after_war` 추가
- [x] start conditions를 `mumyeong_awakening_resolved`, `mumyeong_awakening_thread_opened`, `copy_corruption_thread_opened`, `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `midgame_continuity_started`로 구현
- [x] stable choice id `inspect_bokho_lock_scars`, `compare_hyeonakmun_trace_to_qingliu_wounds`, `ask_seo_harin_what_she_saw_afterward`, `stop_before_replaying_the_attack` 구현
- [x] 모든 선택지가 `qingliu_attack_after_war_resolved`, `qingliu_attack_trace_confirmed`, `hyeonakmun_attack_thread_opened`, `destination_id: cheongryu_outer_courtyard` bridge를 남기도록 구현
- [x] `wuxia_qingliu_attack_after_war` presentation을 `speaker: 천기록`, `layout: attack_trace_investigation`, stable terms `[청류문, 현악문, 복호금쇄수]`로 구현
- [x] Rust/Web storypack preview generated bundle 재생성
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web default content bundle registry 테스트 갱신
- [x] 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key 미변경 유지
- [x] full flashback, 무명 이탈 진실 전체 reveal, 정파 문파 멸문, boss recruit/final boss resolution, 무명 구원 확정, 서하린에게 진실 전달, seed 기반 random copy-style system/table, combat resolver/schema, route graph/faction reputation/debt/relation/reward/ability/epilogue/return schema, 천기록 정체 reveal 미변경 유지
- [x] 다음 작업을 `wuxia_qingliu_attack_after_war_followup` docs-only handoff로 갱신

### 0.2bj 2026-06-02 무협 post-Qingliu trace docs-only handoff

- [x] Notion 사건 카드 DB `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, `wuxia_seoharin_empty_place` 재확인
- [x] parent `무협 스토리팩: 이구학지 — 천기록`, `04. 메인 루트 구조`, `99. 통합 체크포인트`와 비교
- [x] 다음 runtime 후보를 `wuxia_mumyeong_destroys_orthodox_sect`로 결정
- [x] 구현 범위를 현악문 멸문 전투가 아니라 빈 현악문 산문/기록/풍문을 확인하는 consequence trace로 제한한다고 문서화
- [x] `wuxia_boss_recruits_mumyeong`은 정파 문파 멸문 이후 후반 스카웃 사건이라 보류
- [x] `wuxia_mumyeong_departure_truth_summary`는 후반 truth reveal/서하린 진실 전달/무명 구원 조건 확정 범위가 커서 보류
- [x] `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`은 최종장/후일담 routing 범위라 보류
- [x] `wuxia_seoharin_empty_place`는 초반 서하린 side beat라 현재 post-Qingliu trace 위치에서는 보류
- [x] start conditions를 `required_flags: [qingliu_attack_after_war_resolved, qingliu_attack_trace_confirmed, hyeonakmun_attack_thread_opened, mumyeong_awakening_resolved, midgame_continuity_started]`, `forbidden_flags: [mumyeong_destroys_orthodox_sect_resolved]`로 문서화
- [x] stable choice id 후보 `read_hyeonakmun_empty_gate_record`, `trace_bokho_lock_to_mumyeong`, `ask_why_seoharin_never_heard_full_story`, `stop_before_counting_the_dead` 고정
- [x] storypack DB JSON mirror와 docs contract를 다음 runtime 후보 기준으로 갱신
- [x] runtime YAML/Rust/Web generated bundle, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_mumyeong_destroys_orthodox_sect` runtime implementation으로 갱신

### 0.2bk 2026-06-02 무협 `wuxia_mumyeong_destroys_orthodox_sect` preview runtime slice

- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_qingliu_attack_after_war` 뒤에 `wuxia_mumyeong_destroys_orthodox_sect` 추가
- [x] start conditions를 `qingliu_attack_after_war_resolved`, `qingliu_attack_trace_confirmed`, `hyeonakmun_attack_thread_opened`, `mumyeong_awakening_resolved`, `midgame_continuity_started`로 구현
- [x] stable choice id `read_hyeonakmun_empty_gate_record`, `trace_bokho_lock_to_mumyeong`, `ask_why_seoharin_never_heard_full_story`, `stop_before_counting_the_dead` 구현
- [x] 모든 선택지가 `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `destination_id: cheongryu_outer_courtyard` bridge를 남기도록 구현
- [x] `wuxia_mumyeong_destroys_orthodox_sect` presentation을 `speaker: 천기록`, `layout: hyeonakmun_empty_gate_record`, stable terms `[현악문, 복호금쇄수, 무명]`로 구현
- [x] Rust/Web storypack preview generated bundle 재생성
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web default content bundle registry 테스트 갱신
- [x] 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key 미변경 유지
- [x] playable 멸문 전투, full flashback, 무명 이탈 진실 전체 reveal, 보스 스카웃/final boss resolution, 무명 구원 확정, 서하린에게 진실 전달, seed 기반 random copy-style system/table, combat resolver/schema, route graph/faction reputation/debt/relation/reward/ability/epilogue/return schema, 천기록 정체 reveal 미변경 유지
- [x] 다음 작업을 `wuxia_mumyeong_destroys_orthodox_sect_followup` docs-only handoff로 갱신

### 0.2bl 2026-06-02 무협 Hyeonakmun consequence follow-up docs-only handoff

- [x] Notion 사건 카드 DB `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, `wuxia_seoharin_empty_place` 재확인
- [x] 운영 문서 `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `07. 천기록 / 천외편린 보상`, `99. 통합 체크포인트`와 비교
- [x] 다음 runtime 후보를 `wuxia_boss_recruits_mumyeong`로 결정
- [x] 구현 범위를 보스가 무명의 상처/힘을 흑사방으로 끌어들이는 recruitment trace로 제한한다고 문서화
- [x] `wuxia_mumyeong_departure_truth_summary`는 후반 truth/서하린 진실 전달/무명 구원 조건 범위가 커서 보류
- [x] `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`은 최종장/후일담 routing 범위라 보류
- [x] `wuxia_seoharin_empty_place`는 초반 companion/epilogue beat라 현재 Hyeonakmun consequence 위치에서는 보류
- [x] start conditions를 `required_flags: [mumyeong_destroys_orthodox_sect_resolved, hyeonakmun_destruction_thread_opened, departure_truth_thread_deepened, boss_first_appearance_resolved, boss_wall_thread_opened, black_serpent_core_pressure_opened, midgame_continuity_started]`, `forbidden_flags: [boss_recruits_mumyeong_resolved]`로 문서화
- [x] stable choice id 후보 `trace_boss_offer_after_hyeonakmun`, `read_mumyeong_choice_without_excusing_it`, `search_black_serpent_recruitment_record`, `stop_before_following_him_into_black_serpent` 고정
- [x] storypack DB JSON mirror와 docs contract를 다음 runtime 후보 기준으로 갱신
- [x] runtime YAML/Rust/Web generated bundle, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_boss_recruits_mumyeong` runtime implementation으로 갱신

### 0.2bm 2026-06-02 무협 `wuxia_boss_recruits_mumyeong` preview runtime slice

- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_mumyeong_destroys_orthodox_sect` 뒤에 `wuxia_boss_recruits_mumyeong` 추가
- [x] start conditions를 `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `midgame_continuity_started`로 구현
- [x] stable choice id `trace_boss_offer_after_hyeonakmun`, `read_mumyeong_choice_without_excusing_it`, `search_black_serpent_recruitment_record`, `stop_before_following_him_into_black_serpent` 구현
- [x] 모든 선택지가 `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `destination_id: cheongryu_outer_courtyard` bridge를 남기도록 구현
- [x] `wuxia_boss_recruits_mumyeong` presentation을 `speaker: 천기록`, `layout: boss_recruitment_trace`, stable terms `[흑사방주, 무명, 현악문]`로 구현
- [x] Rust/Web storypack preview generated bundle 재생성
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web default content bundle registry 테스트 갱신
- [x] 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 무명 이탈 진실 전체 reveal, 서하린에게 진실 전달, 무명 구원 확정, 보스 전투/final boss resolution, 무명/보스 결산, seed 기반 random copy-style system/table, combat resolver/schema, route graph/faction reputation/debt/relation/reward/ability/epilogue/return schema, 천기록 정체 reveal 미변경 유지
- [x] 다음 작업을 `wuxia_boss_recruits_mumyeong_followup` docs-only handoff로 갱신

### 0.2bn 2026-06-02 무협 boss recruitment follow-up docs-only handoff

- [x] Notion 사건 카드 DB `wuxia_mumyeong_departure_truth_summary`, `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, `wuxia_seoharin_empty_place` 및 기존 late/final 후보 재확인
- [x] 운영 문서 `04. 메인 루트 구조`, `05. 사건 카드 운영 규칙`, `07. 천기록 / 천외편린 보상`, `99. 통합 체크포인트`와 비교
- [x] 다음 runtime 후보를 `wuxia_mumyeong_departure_truth_summary`로 결정
- [x] 구현 범위를 `sealed_departure_truth_summary` trace로 제한한다고 문서화
- [x] `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`은 최종장/후일담 routing 범위라 보류
- [x] `wuxia_seoharin_empty_place`는 초반 companion/무명 clue beat라 현재 boss recruitment 직후 위치에서는 보류
- [x] start conditions를 `required_flags: [boss_recruits_mumyeong_resolved, boss_recruitment_thread_opened, mumyeong_destroys_orthodox_sect_resolved, hyeonakmun_destruction_thread_opened, departure_truth_thread_deepened, mumyeong_request_for_aid_resolved, mumyeong_failed_aid_thread_opened, orthodox_hypocrisy_thread_opened, mumyeong_awakening_resolved, midgame_continuity_started]`, `forbidden_flags: [mumyeong_departure_truth_summary_resolved]`로 문서화
- [x] stable choice id 후보 `assemble_departure_truth_without_delivering`, `compare_failed_aid_to_recruitment_offer`, `ask_seoharin_what_she_is_ready_to_hear`, `seal_truth_until_mumyeong_faces_it` 고정
- [x] common hook `mumyeong_departure_truth_summary_resolved`, `sealed_departure_truth_summary_prepared`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard` 고정
- [x] 서하린에게 진실 전달, `told_seoharin_truth`, 무명 구원 확정, 무명/보스 결산, 최종전, epilogue/return, route/faction/relation/debt/reward schema, 천기록 정체 reveal 미변경 유지
- [x] runtime YAML/Rust/Web generated bundle, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_mumyeong_departure_truth_summary` runtime implementation으로 갱신

### 0.2bo 2026-06-02 무협 `wuxia_mumyeong_departure_truth_summary` preview runtime slice

- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_boss_recruits_mumyeong` 뒤에 `wuxia_mumyeong_departure_truth_summary` 추가
- [x] start conditions를 `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `mumyeong_awakening_resolved`, `midgame_continuity_started`로 구현
- [x] stable choice id `assemble_departure_truth_without_delivering`, `compare_failed_aid_to_recruitment_offer`, `ask_seoharin_what_she_is_ready_to_hear`, `seal_truth_until_mumyeong_faces_it` 구현
- [x] 모든 선택지가 `mumyeong_departure_truth_summary_resolved`, `sealed_departure_truth_summary_prepared`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard` bridge를 남기도록 구현
- [x] `wuxia_mumyeong_departure_truth_summary` presentation을 `speaker: 천기록`, `layout: sealed_departure_truth_summary`, stable terms `[무명, 서하린, 현악문, 흑사방주]`로 구현
- [x] Rust/Web storypack preview generated bundle 재생성
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web default content bundle registry 테스트 갱신
- [x] 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 서하린에게 진실 전달, `told_seoharin_truth`, 무명 구원 확정, 보스 전투/final boss resolution, 무명/보스 결산, seed 기반 random copy-style system/table, combat resolver/schema, route graph/faction reputation/debt/relation/reward/ability/epilogue/return schema, 천기록 정체 reveal 미변경 유지
- [x] 다음 작업을 `wuxia_mumyeong_departure_truth_summary_followup` docs-only handoff로 갱신

### 0.2bp 2026-06-02 무협 departure truth summary follow-up docs-only handoff

- [x] Notion 사건 카드 DB `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, `wuxia_seoharin_empty_place`, `wuxia_mumyeong_departure_truth_summary` 재확인
- [x] 최신 `사도 최종전`, `사도 최종전 2페이즈: 약점 장악`, `사도 최종전 3페이즈: 계산식 밖`, `최종장 결산 라우팅 마스터`, `무명 결산`, `보스 결산` 문서와 비교
- [x] 다음 runtime 후보를 `wuxia_seoharin_empty_place`로 결정
- [x] 구현 범위를 sealed departure truth summary 뒤의 late empty-place memory bridge로 제한한다고 문서화
- [x] `wuxia_mumyeong_resolution`, `wuxia_boss_resolution`, 사도 최종전, final/epilogue/return routing은 최종장 schema 범위라 보류
- [x] 서하린 truth delivery, `told_seoharin_truth`, 무명 구원 확정, `item_unpriced_wooden_sword` 지급은 미변경 유지
- [x] start conditions를 `required_flags: [mumyeong_departure_truth_summary_resolved, sealed_departure_truth_summary_prepared, truth_delivery_still_unopened, midgame_continuity_started]`, `forbidden_flags: [seoharin_empty_place_resolved]`로 문서화
- [x] stable choice id 후보 `ask_who_kept_the_empty_place`, `leave_the_place_unclaimed`, `set_down_the_work_notebook_briefly`, `step_back_without_naming_mumyeong` 고정
- [x] common hook `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard` 고정
- [x] presentation `visual_id: wuxia_seoharin_empty_place`, `speaker: 서하린`, `layout: empty_place_memory`, stable terms `[서하린, 무명, 청류문, 목검]` 고정
- [x] runtime YAML/Rust/Web generated bundle, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_seoharin_empty_place` runtime implementation으로 갱신

### 0.2bq 2026-06-02 무협 `wuxia_seoharin_empty_place` preview runtime slice

- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_mumyeong_departure_truth_summary` 뒤에 `wuxia_seoharin_empty_place` 추가
- [x] start conditions를 `mumyeong_departure_truth_summary_resolved`, `sealed_departure_truth_summary_prepared`, `truth_delivery_still_unopened`, `midgame_continuity_started`로 구현
- [x] stable choice id `ask_who_kept_the_empty_place`, `leave_the_place_unclaimed`, `set_down_the_work_notebook_briefly`, `step_back_without_naming_mumyeong` 구현
- [x] 모든 선택지가 `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard` bridge를 남기도록 구현
- [x] `wuxia_seoharin_empty_place` presentation을 `speaker: 서하린`, `layout: empty_place_memory`, stable terms `[서하린, 무명, 청류문, 목검]`로 구현
- [x] `unpriced_wooden_sword_condition_seeded`는 clue seed로만 남기고 `item_unpriced_wooden_sword`는 지급하지 않음
- [x] Rust/Web storypack preview generated bundle 재생성
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web default content bundle registry 테스트 갱신
- [x] 서하린에게 진실 전달, `told_seoharin_truth`, 무명 구원 확정, 보스 전투/final boss resolution, 무명/보스 결산, seed 기반 random copy-style system/table, combat resolver/schema, route graph/faction reputation/debt/relation/reward/ability/epilogue/return schema, 천기록 정체 reveal 미변경 유지
- [x] 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_seoharin_empty_place_followup` docs-only handoff로 갱신

### 0.2br 2026-06-02 무협 Seo Harin empty-place follow-up docs-only handoff

- [x] Notion 사건 카드 DB `wuxia_seoharin_left_meal`, `wuxia_seoharin_unsaid_stay`, `wuxia_mumyeong_resolution`, `wuxia_boss_resolution` 재확인
- [x] Notion `남겨둔 밥`, `가지 말라는 말`, `무명 결산`, `사도 최종전 3페이즈: 계산식 밖` 원문 fetch로 대조
- [x] 다음 runtime 후보를 `wuxia_seoharin_left_meal`로 결정
- [x] 구현 범위를 `seoharin_axis_opened` 뒤 daily-care/belonging bridge로 제한한다고 문서화
- [x] `wuxia_seoharin_unsaid_stay`는 final return/settlement/corruption relationship branch라 보류
- [x] 무명/보스 결산과 사도 최종전은 final/epilogue/combat/reward/item-log/relation-state schema 범위라 보류
- [x] start conditions를 `required_flags: [seoharin_empty_place_resolved, seoharin_axis_opened, empty_place_remembered, truth_delivery_still_unopened, midgame_continuity_started]`, `forbidden_flags: [seoharin_left_meal_resolved]`로 문서화
- [x] stable choice id 후보 `eat_the_left_meal_quietly`, `thank_seoharin_for_the_bowl`, `joke_about_who_ordered_extra_rice`, `pass_without_eating_the_meal` 고정
- [x] runtime YAML/Rust/Web generated bundle, 기본 office bundle, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_seoharin_left_meal` runtime implementation으로 갱신

### 0.2bs 2026-06-02 무협 `wuxia_seoharin_left_meal` preview runtime slice

- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_seoharin_empty_place` 뒤에 `wuxia_seoharin_left_meal` 추가
- [x] start conditions를 `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened`, `midgame_continuity_started`로 구현
- [x] stable choice id `eat_the_left_meal_quietly`, `thank_seoharin_for_the_bowl`, `joke_about_who_ordered_extra_rice`, `pass_without_eating_the_meal` 구현
- [x] 긍정 선택은 `seoharin_left_meal_resolved`, `seoharin_axis_deepened`, `qingliu_belonging_warmed`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard` bridge를 남기도록 구현
- [x] 거절 선택은 `seoharin_left_meal_resolved`, `seoharin_axis_still_open`, `left_meal_left_untouched`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard` bridge를 남기도록 구현
- [x] `wuxia_seoharin_left_meal` presentation을 `speaker: 서하린`, `layout: left_meal_memory`, stable terms `[서하린, 밥그릇, 청류문, 귀환]`로 구현
- [x] Rust/Web storypack preview generated bundle 재생성
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web default content bundle registry 테스트 갱신
- [x] 서하린에게 진실 전달, `told_seoharin_truth`, final return/settlement choice, 무명/보스 결산, 사도 최종전, combat/reward/relation/epilogue/return schema, 천기록 정체 reveal 미변경 유지
- [x] 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_seoharin_left_meal_followup` docs-only handoff로 갱신

### 0.2bt 2026-06-02 무협 Seo Harin left-meal follow-up docs-only handoff

- [x] Notion `가지 말라는 말`, `무명 결산`, `보스 결산`, `사도 최종전`, `최종장 결산 라우팅 마스터`, `사도 최종전 상태값 사전` 재확인
- [x] `wuxia_seoharin_unsaid_stay`는 final return/settlement/corruption relationship branch라 보류
- [x] `wuxia_mumyeong_resolution`과 `wuxia_boss_resolution`은 final epilogue matrix와 salvation/result routing을 요구하므로 보류
- [x] `wuxia_sado_final_battle` 및 phase 2/3은 final battle/result schema가 필요하므로 보류
- [x] 직접 final runtime 구현 전에 final-route state dictionary를 repo design contract로 먼저 고정하기로 결정
- [x] 다음 runtime 후보를 `wuxia_sado_final_phase_1_price_tag`로 선택하되 기존 encounter schema handoff로 제한
- [x] 서하린 truth delivery, `told_seoharin_truth`, 무명 구원 확정, final epilogue/return schema, combat resolver/HP 숫자전, relation/debt/faction/reward schema, `item_unpriced_wooden_sword` payout 미변경 유지

### 0.2bu 2026-06-02 무협 final state routing contract docs slice

- [x] `docs/design/Wuxia_Final_State_Routing.md` 추가
- [x] `canonical_final_inputs`와 `combat_result`, `boss_resolution_route`, `evidence_state`, `network_handling`, `pressure_state`, `seoharin_axis`, `mumyeong_salvation`, `item_logs` contract 문서화
- [x] `final_result_priority`와 `final_epilogue_master_matrix` handoff boundary 문서화
- [x] `state_alias_and_deprecation_policy`, `item_log_state` local helper only, deprecated aliases 문서화
- [x] 다음 runtime 후보 `wuxia_sado_final_phase_1_price_tag`와 no combat resolver/no HP boundary 문서화
- [x] README, docs index, main plan, Notion coverage, storypack docs, next_goal handoff 동기화

### 0.2bv 2026-06-02 무협 `wuxia_sado_final_phase_1_price_tag` preview runtime slice

- [x] Notion `사도 최종전 1페이즈: 가격표`, `사도 최종전`, phase 2/3, 보스/무명 결산, 서하린 late branch 재확인
- [x] `wuxia_sado_final_phase_1_price_tag`를 기존 encounter schema로 구현 가능한 final-entry slice로 결정
- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/locations.yaml`에 `black_serpent_ledger_vault` 추가
- [x] `cheongryu_outer_courtyard`와 `black_serpent_ledger_vault`를 왕복 연결
- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_seoharin_left_meal` 뒤에 `wuxia_sado_final_phase_1_price_tag` 추가
- [x] start conditions를 `seoharin_left_meal_resolved`, `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened`, `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `boss_first_appearance_resolved`, `black_serpent_core_pressure_opened`, `sealed_departure_truth_summary_prepared`, `midgame_continuity_started`로 구현
- [x] stable choice id `approach_sado_before_the_ledger`, `burn_the_blackscale_ledger`, `secure_the_blackscale_ledger`, `ease_hostage_pressure_first` 구현
- [x] 모든 선택지가 `sado_final_phase_1_price_tag_resolved`, `final_state_routing_seeded`, `destination_id: black_serpent_ledger_vault` bridge를 남기도록 구현
- [x] direct/ledger burn/ledger secure/pressure relief별 `network_handling`, `evidence_state`, `pressure_state`, `item_logs` seed를 flags/clues/log로만 구현
- [x] `secure_the_blackscale_ledger`는 `item_blackscale_ledger_logged` clue와 `final_item_logs_blackscale_ledger_seeded` flag만 남기고 새 item 지급은 하지 않음
- [x] Rust/Web storypack preview generated bundle 재생성
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web default content bundle registry 테스트 갱신
- [x] combat resolver, HP 숫자전, final epilogue/return schema, 서하린 truth delivery, `told_seoharin_truth`, 무명 구원 확정, relation/reward schema, `item_unpriced_wooden_sword` payout 미변경 유지
- [x] 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_sado_final_phase_2_weakpoint_control` handoff 검토로 갱신

### 0.2bw 2026-06-02 무협 `wuxia_sado_final_phase_2_weakpoint_control` preview runtime slice

- [x] Notion `사도 최종전 2페이즈: 약점 장악`, `사도 최종전 3페이즈: 계산식 밖`, `사도 최종전`, `보스 결산` 재확인
- [x] `wuxia_sado_final_phase_2_weakpoint_control`을 기존 encounter schema로 구현 가능한 final-entry slice로 결정
- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_sado_final_phase_1_price_tag` 뒤에 `wuxia_sado_final_phase_2_weakpoint_control` 추가
- [x] start conditions를 `sado_final_phase_1_price_tag_resolved`, `final_state_routing_seeded`로 구현
- [x] stable choice id `respond_to_seoharin_pressure`, `return_flow_to_mumyeong`, `read_dangerous_cheongirok_sentence`, `focus_on_sado` 구현
- [x] 모든 선택지가 `sado_final_phase_2_weakpoint_control_resolved`, `final_phase_2_weakpoint_control_resolved`, `destination_id: black_serpent_ledger_vault` bridge를 남기도록 구현
- [x] 서하린/무명/천기록/사도 집중별 `seoharin_axis`, `qingliu_rebuild`, `mumyeong_salvation`, `successor_route`, `own_flow_choice`, `cheongirok_state`, `player_method` seed를 flags/clues/log로만 구현
- [x] Rust/Web storypack preview generated bundle 재생성
- [x] Python exporter/docs/storypack DB, Rust content bundle, Web default content bundle registry 테스트 갱신
- [x] combat resolver, HP 숫자전, final epilogue/return schema, 서하린 truth delivery, `told_seoharin_truth`, 무명 구원 확정, relation/reward schema, `item_unpriced_wooden_sword` payout 미변경 유지
- [x] 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_sado_final_phase_3_outside_calculation` handoff 검토로 갱신

### 0.2bx 2026-06-02 무협 `wuxia_sado_final_phase_3_outside_calculation` preview runtime slice

- [x] Notion `사도 최종전 3페이즈: 계산식 밖`, `사도 최종전`, `보스 결산`, `최종장 결산 라우팅 마스터`, `사도 최종전 상태값 사전` 재확인
- [x] `wuxia_sado_final_phase_3_outside_calculation`을 기존 encounter schema로 구현 가능한 final-entry slice로 결정
- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_sado_final_phase_2_weakpoint_control` 뒤에 `wuxia_sado_final_phase_3_outside_calculation` 추가
- [x] start conditions를 `sado_final_phase_2_weakpoint_control_resolved`, `final_phase_2_weakpoint_control_resolved`, `final_state_routing_seeded`로 구현
- [x] stable choice id `remember_the_empty_place`, `let_mumyeong_choose_own_flow`, `endure_with_qingliu_will`, `point_to_blank_in_ledger`, `answer_with_sado_calculation` 구현
- [x] 모든 선택지가 `sado_final_phase_3_outside_calculation_resolved`, `final_phase_3_outside_calculation_resolved`, `final_combat_result_battle_victory_seeded`, `destination_id: black_serpent_ledger_vault` bridge를 남기도록 구현
- [x] true/meaningful/corrupted boss resolution candidate와 서하린/무명/청류문/장부/천기록 final-state seed를 flags/clues/log로만 구현
- [x] Rust/Web storypack preview generated bundle 재생성
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web default content bundle registry 테스트 갱신
- [x] combat resolver, HP 숫자전, 보스 결산 출력기, final epilogue/return schema, 서하린 truth delivery, `told_seoharin_truth`, 무명 구원 확정, relation/reward schema, `item_unpriced_wooden_sword` payout 미변경 유지
- [x] 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_boss_resolution` handoff 검토로 갱신

### 0.2by 2026-06-02 무협 `wuxia_boss_resolution` preview runtime slice

- [x] Notion `보스 결산`, `최종장 결산 라우팅 마스터`, `사도 최종전 상태값 사전`, `사도 최종전` 재확인
- [x] `wuxia_boss_resolution`을 기존 encounter schema로 구현 가능한 boss-resolution route seed bridge로 결정
- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_sado_final_phase_3_outside_calculation` 뒤에 `wuxia_boss_resolution` 추가
- [x] start conditions를 `sado_final_phase_3_outside_calculation_resolved`, `final_phase_3_outside_calculation_resolved`, `final_combat_result_battle_victory_seeded`, `final_state_routing_seeded`로 구현
- [x] stable choice id `confirm_true_route_outside_calculation`, `confirm_meaningful_victory_with_evidence`, `confirm_incomplete_victory_residue`, `confirm_mumyeong_unsaved_successor_risk`, `confirm_corrupted_victory` 구현
- [x] 모든 선택지가 `boss_resolution_resolved`, `final_result_priority_applied_seeded`, `destination_id: black_serpent_ledger_vault` bridge를 남기도록 구현
- [x] true/meaningful/incomplete/mumyeong-unsaved/corrupted route seed와 후속 epilogue candidate seed를 flags/clues/log로만 구현
- [x] Rust/Web storypack preview generated bundle 재생성
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web default content bundle registry 테스트 갱신
- [x] final epilogue renderer, return/settlement schema, combat resolver, HP 숫자전, 서하린 truth delivery, `told_seoharin_truth`, 무명 구원 확정, relation/reward schema, `item_unpriced_wooden_sword` payout 미변경 유지
- [x] 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_mumyeong_resolution` handoff 검토로 갱신

### 0.2bz 2026-06-02 무협 `wuxia_mumyeong_resolution` preview runtime slice

- [x] Notion `무명 결산`, `최종장 결산 라우팅 마스터`, `사도 최종전 상태값 사전` 재확인
- [x] `wuxia_mumyeong_resolution`을 기존 encounter schema로 구현 가능한 Mumyeong-resolution route seed bridge로 결정
- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_boss_resolution` 뒤에 `wuxia_mumyeong_resolution` 추가
- [x] start conditions를 `boss_resolution_resolved`, `final_result_priority_applied_seeded`, `final_combat_result_battle_victory_seeded`, `final_state_routing_seeded`로 구현
- [x] stable choice id `ask_mumyeong_for_own_flow`, `reveal_boss_used_mumyeongs_wound`, `leave_room_for_unsent_apology`, `let_stolen_forms_end`, `confirm_black_serpent_successor_risk`, `judge_with_sado_style_calculation` 구현
- [x] 모든 선택지가 `mumyeong_resolution_resolved`, `destination_id: black_serpent_ledger_vault` bridge를 남기도록 구현
- [x] own-flow/relational/incomplete/end-of-stolen-forms/black-serpent-successor/corrupted-unsaved route seed와 후속 epilogue candidate seed를 flags/clues/log로만 구현
- [x] Rust/Web storypack preview generated bundle 재생성
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web default content bundle registry 테스트 갱신
- [x] final epilogue renderer, return/settlement schema, combat resolver, HP 숫자전, 서하린 truth delivery, `told_seoharin_truth`, relation/reward schema, `item_unpriced_wooden_sword` payout 미변경 유지
- [x] 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_seoharin_qingliu_resolution` handoff 검토로 갱신

### 0.2ca 2026-06-02 무협 `wuxia_seoharin_qingliu_resolution` preview runtime slice

- [x] Notion `가지 말라는 말`, `서하린의 후일`, `청류문의 후일`, `닫히지 않은 산문`, `닫힌 산문`, `최종장 결산 라우팅 마스터`, `사도 최종전 상태값 사전` 재확인
- [x] `wuxia_seoharin_qingliu_resolution`을 기존 encounter schema로 구현 가능한 Seo Harin/Qingliu resolution route seed bridge로 결정
- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_mumyeong_resolution` 뒤에 `wuxia_seoharin_qingliu_resolution` 추가
- [x] start conditions를 `mumyeong_resolution_resolved`, `boss_resolution_resolved`, `final_result_priority_applied_seeded`, `final_combat_result_battle_victory_seeded`, `final_state_routing_seeded`로 구현
- [x] stable choice id `leave_the_gate_unclosed`, `record_qingliu_rebuild_without_glory`, `keep_empty_place_for_return_or_absence`, `mark_qingliu_pressure_still_unresolved`, `close_the_gate_with_sado_logic` 구현
- [x] 모든 선택지가 `seoharin_qingliu_resolution_resolved`, `destination_id: black_serpent_ledger_vault` bridge를 남기도록 구현
- [x] open-gate/empty-place/Qingliu future/weakened-pressure/closed-gate epilogue candidate seed를 flags/clues/log로만 구현
- [x] Rust/Web storypack preview generated bundle 재생성
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web default content bundle registry 테스트 갱신
- [x] final epilogue renderer, return/settlement schema, combat resolver, HP 숫자전, 서하린 truth delivery, `told_seoharin_truth`, relation/reward schema, `item_unpriced_wooden_sword` payout 미변경 유지
- [x] 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_cheongirok_resolution` handoff 검토로 갱신

### 0.2cb 2026-06-02 무협 `wuxia_cheongirok_resolution` preview runtime slice

- [x] Notion `천기록의 마지막 장`, `07. 천기록 / 천외편린 보상`, `08. 엔딩과 후일담 연결`, `최종장 결산 라우팅 마스터`, `사도 최종전 상태값 사전` 재확인
- [x] `wuxia_cheongirok_resolution`을 기존 encounter schema로 구현 가능한 Cheonggi Record last-page route seed bridge로 결정
- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_seoharin_qingliu_resolution` 뒤에 `wuxia_cheongirok_resolution` 추가
- [x] start conditions를 `seoharin_qingliu_resolution_resolved`, `boss_resolution_resolved`, `mumyeong_resolution_resolved`, `final_result_priority_applied_seeded`, `final_combat_result_battle_victory_seeded`, `final_state_routing_seeded`로 구현
- [x] stable choice id `turn_the_last_page_without_question`, `leave_blank_as_unpriced_place`, `read_the_lines_that_align_like_ledger`, `close_record_before_it_becomes_answer`, `let_record_reflect_the_method` 구현
- [x] 모든 선택지가 `cheongirok_resolution_resolved`, `destination_id: black_serpent_ledger_vault` bridge를 남기도록 구현
- [x] safe last-page/blank true-route/corruption/low-use silence/player-method epilogue candidate seed를 flags/clues/log로만 구현
- [x] Rust/Web storypack preview generated bundle 재생성
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web default content bundle registry 테스트 갱신
- [x] final epilogue renderer, return/settlement schema, combat resolver, HP 숫자전, 서하린 truth delivery, `told_seoharin_truth`, relation/reward schema, `item_unpriced_wooden_sword` payout, 천기록 기록자 정체 reveal 미변경 유지
- [x] 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_black_serpent_aftermath` handoff 검토로 갱신

### 0.2cc 2026-06-02 무협 `wuxia_black_serpent_aftermath` preview runtime slice

- [x] Notion `08. 엔딩과 후일담 연결`, `최종장 결산 라우팅 마스터`, `사도 최종전 상태값 사전` 재확인
- [x] `wuxia_black_serpent_aftermath`를 기존 encounter schema로 구현 가능한 Black Serpent aftermath route seed bridge로 결정
- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에서 `wuxia_cheongirok_resolution` 뒤에 `wuxia_black_serpent_aftermath` 추가
- [x] start conditions를 `cheongirok_resolution_resolved`, `seoharin_qingliu_resolution_resolved`, `mumyeong_resolution_resolved`, `boss_resolution_resolved`, `final_result_priority_applied_seeded`, `final_combat_result_battle_victory_seeded`, `final_state_routing_seeded`로 구현
- [x] stable choice id `mark_broken_serpent_without_erasing_scars`, `fold_the_banner_without_calling_it_gone`, `send_ledger_to_alliance_and_watch_silence`, `listen_for_southern_market_debt_rumor`, `let_true_route_suppress_the_banner` 구현
- [x] 모든 선택지가 `black_serpent_aftermath_resolved`, `destination_id: black_serpent_ledger_vault` bridge를 남기도록 구현
- [x] broken serpent/banner residue/alliance silence/southern market rumor/true-route suppression epilogue candidate seed를 flags/clues/log로만 구현
- [x] Rust/Web storypack preview generated bundle 재생성
- [x] Python exporter/docs/storypack DB, Rust content bundle, WASM JSON boundary, terminal smoke, Web default content bundle registry 테스트 갱신
- [x] final epilogue renderer, return/settlement schema, combat resolver, HP 숫자전, 서하린 truth delivery, `told_seoharin_truth`, relation/reward schema, `item_unpriced_wooden_sword` payout, 천기록 기록자 정체 reveal 미변경 유지
- [x] 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_final_epilogue_renderer_contract` handoff 검토로 갱신

### 0.2cd 2026-06-02 무협 final epilogue renderer contract docs-only handoff

- [x] Notion `최종장 결산 라우팅 마스터`, `08. 엔딩과 후일담 연결`, `사도 최종전 상태값 사전` 재확인
- [x] `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, `wuxia_seoharin_qingliu_resolution`, `wuxia_cheongirok_resolution`, `wuxia_black_serpent_aftermath` seed bridge 대조
- [x] 추가 seed bridge 없이 `wuxia_final_epilogue_renderer_contract` implementation slice를 열 수 있다고 결정
- [x] Rust GameCore가 seed consumption, `final_result_priority`, suppress, card ordering을 소유하는 boundary 문서화
- [x] Web Storybook/SuperLightTUI는 core 결과 표시만 하며 후일담 카드 enable/suppress를 재계산하지 않는 boundary 문서화
- [x] `docs/design/Wuxia_Final_State_Routing.md`에 candidate group, preconditions, suppress examples, open implementation questions 추가
- [x] runtime YAML/Rust/Web/generated artifact 미변경 유지
- [x] final epilogue output schema, return/settlement schema, combat resolver, HP 숫자전, relation/debt/faction ledger, reward/ability schema, `item_unpriced_wooden_sword` payout 미변경 유지
- [x] 서하린 truth delivery, `told_seoharin_truth`, 천기록 기록자 정체 reveal 미변경 유지
- [x] 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 `wuxia_final_epilogue_renderer_contract` implementation slice 검토로 갱신

### 0.2ce 2026-06-02 무협 `wuxia_final_epilogue_renderer_contract` runtime implementation slice

- [x] `wuxia_final_epilogue_renderer_contract`를 preview ending으로 추가
- [x] Rust GameCore-owned final epilogue seed consumer 구현
- [x] final result priority, seed consumption, suppress, card ordering을 renderer가 아니라 core에서 처리
- [x] structured `ScenePage.body_blocks` convention으로 `epilogue_result`, `epilogue_card`, `epilogue_suppressed` 출력
- [x] Web Storybook과 SuperLightTUI는 core 제공 body block 표시만 하도록 검증
- [x] Rust/Web storypack preview generated bundle 재생성
- [x] Rust core direct-state route parity, WASM JSON boundary, terminal smoke, Web Storybook pass-through 테스트 추가
- [x] combat resolver, HP 숫자전, return/settlement schema, relation/debt/faction ledger, reward/ability schema, `item_unpriced_wooden_sword` payout 미변경 유지
- [x] 서하린 truth delivery, `told_seoharin_truth`, 천기록 기록자 정체 reveal 미변경 유지
- [x] 기본 office bundle, Web legacy generated `content.bundle.json`, `src/tui_adv/data/*.yaml`, legacy `escape-office` save/localStorage key 미변경 유지
- [x] 다음 작업을 final epilogue UX/playtest 및 contract follow-up 검증으로 갱신

### 0.2cf 2026-06-02 무협 final epilogue UX/playtest follow-up

- [x] Web Storybook이 `epilogue_result`, `epilogue_card`, `epilogue_suppressed`, `epilogue_contract_error` body block을 양피지형 후일담 section으로 표시
- [x] 후일담 prose는 본문에 즉시 노출하고, contract metadata는 접힌 `details` 기록으로 보관
- [x] `data-body-kind`, `data-source-id`, metadata field key data attribute를 유지해 renderer-neutral contract test point 보존
- [x] final epilogue 화면에서 직전 result history row가 summary 영역에 중복 노출되지 않도록 처리
- [x] SuperLightTUI가 epilogue heading, compact metadata/prose, whitespace wrap으로 40-line snapshot 안에 핵심 카드 내용을 유지
- [x] local ignored `web/src/core/wasm-pkg/` stale 상태에서는 Web playtest가 final epilogue block을 못 받는 문제 확인 및 local WASM package 재빌드
- [x] fresh Web run에서 scripted final route가 빈 action choice와 10개 core-owned epilogue block으로 끝나는지 DOM playtest
- [x] combat resolver, HP 숫자전, return/settlement schema, relation/debt/faction ledger, reward/ability schema, `item_unpriced_wooden_sword` payout 미변경 유지
- [x] 서하린 truth delivery, `told_seoharin_truth`, 천기록 기록자 정체 reveal 미변경 유지
- [x] 다음 작업을 return/settlement, final battle loss path, reward/ability, relation/debt/faction ledger 중 하나의 contract handoff 선택으로 갱신

### 0.2cg 2026-06-02 무협 return/settlement contract docs-only handoff

- [x] Notion `가지 말라는 말`, `08. 엔딩과 후일담 연결`, `11. True Ending 단일 루트`, `사도 최종전`, `최종장 결산 라우팅 마스터`, `사도 최종전 상태값 사전` 재확인
- [x] Notion `06. 사이드 퀘스트와 미해결 부채`, `07. 천기록 / 천외편린 보상`을 next contract 후보 비교용으로 재확인
- [x] 다음 contract surface를 return/settlement로 결정
- [x] 다음 runtime 후보를 `wuxia_seoharin_unsaid_stay` / `가지 말라는 말`로 결정
- [x] 첫 구현 범위를 full return ending이 아니라 서하린 late relationship trigger + return/settlement/corruption 후보 seed bridge로 제한
- [x] stable choice id `say_return_home_honestly`, `say_you_will_stay_with_qingliu`, `share_uncertainty_without_running`, `turn_away_from_the_empty_place` 확정
- [x] `docs/design/Wuxia_Final_State_Routing.md`에 required/common/candidate seed와 guardrail 기록
- [x] final battle loss path, reward/ability schema, relation/debt/faction ledger는 다음 contract로 선택하지 않은 이유 기록
- [x] full modern return ending, return/settlement save/archive schema, relation/debt/faction ledger, reward/ability schema 미변경 유지
- [x] combat resolver, HP 숫자전, 서하린 truth delivery, `told_seoharin_truth`, 천기록 기록자 정체 reveal 미변경 유지
- [x] 다음 작업을 `wuxia_seoharin_unsaid_stay` runtime implementation slice로 갱신

### 0.2ch 2026-06-02 무협 `wuxia_seoharin_unsaid_stay` preview runtime slice

- [x] Notion `가지 말라는 말` 원문과 `Return/Settlement Contract Handoff` 대조
- [x] `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`에 `wuxia_seoharin_unsaid_stay` 추가
- [x] `wuxia_seoharin_qingliu_resolution` 뒤, `wuxia_cheongirok_resolution` 앞에 삽입
- [x] `say_return_home_honestly`, `say_you_will_stay_with_qingliu`, `share_uncertainty_without_running`, `turn_away_from_the_empty_place` stable choice id 적용
- [x] `seoharin_unsaid_stay_resolved`, `final_return_settlement_contract_seeded` common hook 적용
- [x] return/settlement/uncertainty/evasion 후보 seed를 flags/clues/log/presentation hook으로만 남김
- [x] `wuxia_cheongirok_resolution` required flags에 `seoharin_unsaid_stay_resolved` 추가
- [x] Rust/Web generated preview bundle 갱신
- [x] Python storypack DB/export, Rust content bundle, WASM JSON boundary, terminal smoke, Web default bundle registry 테스트 갱신
- [x] full modern return ending, return/settlement save/archive schema, relation/debt/faction ledger 미변경 유지
- [x] reward/ability schema, combat resolver, HP 숫자전, 서하린 truth delivery, `told_seoharin_truth`, 천기록 기록자 정체 reveal 미변경 유지
- [x] 다음 작업을 `return_settlement_followup_handoff` docs-only contract slice로 갱신

### 0.2ci 2026-06-02 무협 `wuxia_return_settlement_epilogue_contract` runtime slice

- [x] Notion `가지 말라는 말`, `08. 엔딩과 후일담 연결`, `01. 메인 엔딩 구조`, `09. 예시 엔딩`, `10. 이구학지 후일담 카드 DB`, `06. 사이드 퀘스트와 미해결 부채`, `07. 천기록 / 천외편린 보상` 대조
- [x] 다음 runtime을 새 main ending enum/schema가 아니라 existing final epilogue body block branch 확장으로 결정
- [x] `crates/escape-core/src/final_epilogue.rs`에 return/settlement branch card group 추가
- [x] `epilogue_wuxia_returned_commute`, `epilogue_wuxia_qingliu_settlement`, `epilogue_wuxia_empty_place_kept_open`, `epilogue_wuxia_closed_gate_risk` card id 적용
- [x] `final_return_intent_honest_seeded`, `final_settlement_intent_honest_seeded`, `final_return_settlement_uncertain_shared_seeded`, `final_return_settlement_evasion_seeded` 후보 seed 소비
- [x] contradictory direct-state seed에서 `epilogue_wuxia_closed_gate_risk`가 optimistic branch cards를 `return_settlement_evasion`으로 suppress
- [x] Rust core route parity, WASM JSON boundary, SuperLightTUI smoke assertions 갱신
- [x] full modern return scene, return/settlement save/archive schema, relation/debt/faction ledger 미변경 유지
- [x] reward/ability schema, combat resolver, HP 숫자전, 서하린 truth delivery, `told_seoharin_truth`, 천기록 기록자 정체 reveal 미변경 유지
- [x] 다음 작업을 `return_settlement_epilogue_followup_handoff` docs-only contract slice로 갱신

### 0.2cj 2026-06-02 무협 return/settlement epilogue follow-up docs-only handoff

- [x] Notion `최종장 결산 라우팅 마스터`, `사도 최종전`, `사도 최종전 상태값 사전`, `08. 엔딩과 후일담 연결` 재대조
- [x] Notion 후일담 DB row `흑사방의 깃발`, `검은 뱀의 새 비늘`, `닫힌 산문`, `천기록의 마지막 장` 재대조
- [x] `06. 사이드 퀘스트와 미해결 부채`, `07. 천기록 / 천외편린 보상`, `01. 메인 엔딩 구조` 범위 재대조
- [x] battle-loss branch, broader corruption/closed-gate branch, reward/ability schema, relation/debt/faction ledger, main ending archive/save surface 비교
- [x] 다음 runtime 후보를 `wuxia_battle_loss_epilogue_contract`로 결정
- [x] `docs/design/Wuxia_Final_State_Routing.md`에 battle-loss bundle, suppress rule, still-closed 범위 기록
- [x] full final battle container, combat resolver, HP 숫자전, archive/save schema, relation/debt/faction ledger, reward/ability schema 미변경 유지

### 0.2ck 2026-06-02 무협 `wuxia_battle_loss_epilogue_contract` runtime slice

- [x] Existing final epilogue body block consumer가 `final_combat_result_battle_loss_seeded`를 소비
- [x] `epilogue_boss_black_serpent_banner`, `epilogue_wuxia_southern_market_rumor`, `epilogue_mumyeong_black_serpent_new_scale`, `epilogue_seoharin_closed_gate`, `epilogue_tianjilu_last_page` loss bundle 출력
- [x] optimistic victory cards `epilogue_boss_broken_black_serpent`, `epilogue_seoharin_open_gate`, `epilogue_mumyeong_stolen_forms_stopped`를 `battle_loss`로 suppress
- [x] final epilogue ending YAML gate를 victory-only required flag에서 Rust consumer-owned victory/loss precondition으로 전환
- [x] Rust fixture와 Web generated storypack-preview bundle 재생성
- [x] Rust route parity test와 WASM JSON boundary test 추가
- [x] full final battle container, combat resolver, HP 숫자전, playable defeat route, archive/save schema, relation/debt/faction ledger, reward/ability schema 미변경 유지
- [x] 다음 작업을 `wuxia_battle_loss_epilogue_followup_handoff` docs-only contract slice로 갱신

### 0.2cl 2026-06-02 무협 `wuxia_battle_loss_epilogue_followup_handoff` docs-only handoff

- [x] Notion `최종장 결산 라우팅 마스터`, `사도 최종전`, `사도 최종전 상태값 사전`, `08. 엔딩과 후일담 연결` 재대조
- [x] Notion `06. 사이드 퀘스트와 미해결 부채`, `07. 천기록 / 천외편린 보상`, `03. 세력과 외부 압박`, `엔딩 시스템`, `01. 메인 엔딩 구조`, `06. 엔딩 아카이브` 재대조
- [x] full final battle container, broader corruption/closed-gate branch, reward/ability schema, relation/debt/faction ledger, main ending archive/save surface, playable defeat-route bridge 비교
- [x] 다음 runtime 후보를 `wuxia_final_state_canonical_collapse_contract`로 결정
- [x] 기존 `final_*_seeded` local flags를 canonical final state labels로 접는 scope 기록
- [x] `ScenePage.body_blocks` 기반 `epilogue_state_audit` audit block 후보 기록
- [x] full final battle container, combat resolver, HP 숫자전, playable defeat route, archive/save schema, relation/debt/faction ledger, reward/ability schema 미변경 유지
- [x] 다음 작업을 `wuxia_final_state_canonical_collapse_contract_implementation`으로 갱신

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

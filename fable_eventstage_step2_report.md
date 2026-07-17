# Event Stage Wave 1 구현 보고서

## 범위와 기준

- 계획: `fable_eventstage_step1_2607171255.md`
- 작업 기준 baseline: `381480a` (`origin/main` fast-forward 확인 후 시작)
- 범위: 이구학지 preview 사건 10개를 ordered Event/Stage/ContentBlock 모델로 전환
- 비범위: office/default 콘텐츠, 신규 일러스트 자산, Web 기능 추가, 전투/성장 규칙 변경

## 구현 결과

### WP-D1 — 문서 계약

`docs/design/Event_Stage_Content_Model.md`와 `docs/dev/Data_Schema.md`에 Event, Stage, ContentBlock의 authoring/wire 계약을 정리했다. Story/Choice/ResultStage의 순서, 결과 블록의 역할, placeholder 일러스트 표기를 명시했다.

### WP-S1 — 코어와 타입 미러

- `ContentBlockDef.branch: Option<String>`을 additive 필드로 추가했다.
- branch 값은 `success`와 `failure`만 허용하고, 일반 Story/Choice 블록과 legacy bundle은 그대로 유지한다.
- ResultStage에서 `state.last_check`와 일치하는 branch만 노출하고 branch 없는 블록은 항상 노출한다.
- Choice의 `next_stage_id`가 자기 ResultStage를 직접 가리킬 수 있도록 cursor를 보정했다. ResultStage 완료 후에는 해당 결과의 후속 stage로 이동하며, 기존 post-result choice 참조의 legacy 동작은 보존한다.
- `SceneContentItem.branch`를 Web 타입에 미러링했다.

### WP-C1/C2 — Wave 1 콘텐츠

다음 10개 사건을 Story → Choice → choice별 ResultStage ordered stream으로 변환했다.

1. `wuxia_heuksa_bang_first_fight`
2. `wuxia_cheongryu_apprentice_entry`
3. `wuxia_cheongryu_chore_sparring`
4. `wuxia_cheongryu_raid_route_split`
5. `wuxia_cheongryu_raid_wounded_fallback`
6. `wuxia_baekdo_medicine_debt`
7. `wuxia_black_heaven_escape_price`
8. `wuxia_heavenly_archive_previous_outsiders`
9. `wuxia_wounded_shelter_dawn_offers`
10. `wuxia_mumyeong_first_sighting`

각 결과 stage는 기존 choice의 outcome log를 요약 블록과 verbatim log 블록으로 유지한다. 판정 choice에는 success/failure narration을 추가했고, 기존 visual ID를 재사용했다. manifest에 없는 일러스트는 `placeholder: true`로 표시했으며 새 이미지 파일은 만들지 않았다. action ID, flags, checks, 기존 content는 변경하지 않았다.

### WP-W1 — 렌더러 호환

Web은 authored `content_stream` 순서를 그대로 사용한다. 기존 terminal scripted smoke가 presentation-only story/result stage에서 멈추지 않도록 terminal snapshot에 legacy visual anchor를 유지하고, 비대화형 `tui_smoke`에서만 `event:continue`를 자동 적용한다. 이는 Web 기능이 아니며 interactive terminal 동작은 바꾸지 않는다.

## 검증

- `cargo test --workspace` 통과. 주요 suite 결과: content bundle 9, core contract 32, event stage 9, Wave 1 guard 3, route parity 23, terminal CLI smoke 61, WASM JSON contract 36 (합계 173).
- `.venv/bin/pytest -q tests/test_web_data_export.py tests/test_docs_contract.py` — 72 passed.
- `python3 scripts/export_web_data.py --check` — web data up to date.
- 두 wuxia preview bundle export/check — up to date.
- `git diff --check` — 통과.
- `cd web && npx vitest run && npx tsc --noEmit && npm run build` — 13 files / 72 tests 통과, TypeScript와 Vite build 통과.
- `wasm-pack build crates/escape-wasm --target web --out-dir web/src/core/wasm-pkg` — 통과.
- `npm run qa:storybook:visual -- --require-wasm` — 390×844, 414×896, 800×1440, 810×1644, 1440×1000 모두 통과. narrow viewport overlap/HUD/scroll-width 검증 포함.
- 자동 Wave 1 completeness/action-ID guard 및 10개 사건의 outcome-log/branch 보존 검증 통과.

## 수동 확인 체크리스트

- [x] 자동 테스트로 staged cursor, result routing, success/failure branch를 확인했다.
- [x] 5 viewport visual QA와 WASM 로딩 검증을 실행했다.
- [ ] 브라우저에서 390px typewriter motion과 check seal을 직접 관찰
- [ ] heuksa 성공/실패를 두 seed로 직접 플레이
- [ ] save/reload 후 staged cursor 복원 직접 확인
- [ ] badge ending route 직접 플레이

Playwright용 Chrome 설치가 현재 WSL 환경에서 완료되지 않아 motion/screenshot 기반 수동 점검은 이번 closeout에서 실행하지 못했다. 다음 QA에서 위 네 항목을 우선 수행한다.

## 후속 작업

- Wave 2 사건을 동일 계약으로 전환한다.
- Notion 설계 DB는 이번 Wave 1에서 신규 사건 ID/내용을 만들지 않았으므로 상태를 변경하지 않았다. 신규 카드 매핑 시 사건별 runtime 대조를 수행한다.
- office/default bundle과 legacy save key는 계속 변경하지 않는다.

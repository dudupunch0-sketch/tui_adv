# Event/Stage Wave 3 설계 — 최종장과 붕괴 게이트

상태: 구현 승인 대기 중인 실행 설계

원본 계획: `fable_eventstage_wave3_step1_2607171715.md`

설계 기준: `origin/main` `341aab0` (Wave 3 계획 커밋까지 포함한 최신 main)

선행 계약: `docs/design/Event_Stage_Content_Model.md`, `docs/dev/Data_Schema.md`

## 1. 설계 목표

무협 preview의 남은 14개 Encounter를 기존 Encounter 선택·조건·결과 의미를 보존한
ordered Event로 투영한다. Wave 1/2에서 사용한 `StoryStage → ChoiceStage →
ResultStage` 문법을 최종장과 붕괴 게이트에도 적용하여, Wave 3 종료 시점에
`wuxia_jianghu_pack`의 44개 Encounter가 모두 `event.stages`를 갖도록 한다.

이 설계는 새 게임 규칙이나 새 wire field를 열지 않는다. Story/Result의 표현 순서와
커서만 Event가 소유하고, eligibility·outcome·flag·clue·ending 판정은 기존 Rust
GameCore가 계속 소유한다.

## 2. 건설 범위

### 포함

- 최종전·결산·귀환/정착 후일담·붕괴 게이트 14개 Encounter의 staged authoring
- 각 Encounter의 기존 선택 ID, action ID, 조건, 비용, 자원 변화, 위험도, flags, clues,
  destination, outcome log, `presentation.visual_id` 보존
- Encounter당 1개의 `illustration` block. 현재 manifest에 없는 그림은
  `placeholder: true`와 한 줄짜리 완전한 한국어 `alt`로 유지
- `wuxia_cheongirok_resolution`에서 천기록이 직접 쓰거나 반응하는 문장만
  `document`/`cheongirok` block으로 표현
- 해당 장면에서도 일반 서술은 화자 없는 `narration`으로 두며, `speaker: 천기록`을
  narration에 붙이지 않는다.
- `wuxia_collapse_gate`의 붕괴 시각 표면과 기존 `wuxia_death_rest` ending 우선순위 보존
- core index/cursor/ending precedence를 고정하는 회귀 테스트, 44/44 coverage guard,
  generated bundle 및 문서 동기화

### 제외

- `web/public/assets/art/`, `artManifest.ts`와 새 일러스트 제작
- `epilogue_*` body block 계약, `crates/escape-core/src/final_epilogue.rs` 수정
- 전투 resolver, HP 전투 규칙, trait effect, usable-item content
- 새 action prefix, storage key, dependency, schema version, archive/save 형식
- office pack과 legacy `body`/`presentation`/`choices` 제거
- Wave 3 선택지의 임의 `check`/`branch` 추가

## 3. 공통 Stage 설계

모든 일반 Wave 3 Encounter는 아래의 동일한 최소 그래프를 사용한다.

```text
<encounter>_story
  └─ (narration[, dialogue][, document/cheongirok], illustration)
       ↓ event:continue
<encounter>_choice
  └─ choice ref → <choice_id>_result
       ↓ choice:<choice_id>
<choice_id>_result
  └─ result_summary, narration/dialogue/document (legacy outcome log 포함)
       ↓ event:continue
Event 종료 → 기존 Encounter selector / ending precedence
```

- StoryStage의 block 배열이 첫 화면의 서사 순서다. 기존 `body`를 문장 경계에서만
  나누고, 한국어 원문은 바꾸지 않는다.
- ChoiceStage의 `choices[*].id`는 legacy `choices[*].id`와 1:1 동일해야 한다.
  각 ref의 `next_stage_id`는 해당 `<choice_id>_result`를 가리킨다.
- ResultStage는 선택마다 하나씩 만든다. `result_summary`는 선택 label을 요약하고,
  뒤의 narration/dialogue는 해당 legacy `outcome.log`를 verbatim으로 담는다.
  결과 로그는 engine이 상태를 적용할 때 만드는 action log와 중복되지만, 다음 화면의
  작가 서사로서 필요하므로 삭제하지 않는다.
- 결과 뒤에 별도 StoryStage가 필요하지 않은 14개 장면은 ResultStage 종료 후 Event를
  끝낸다. 다음 Encounter는 기존 조건·가중치 선택으로 결정된다.
- `branch`는 이번 Wave 3에서 사용하지 않는다. checked choice가 0개라는 조사 결과를
  테스트로 고정하고, branch fixture를 콘텐츠 의미 없이 추가하지 않는다.
- 기존 `body`, `presentation`, `choices`는 terminal/office parity와 migration 기간을
  위해 그대로 남긴다. `event`가 runtime ordered stream의 우선 표현이 된다.

### Illustration 규칙

- 일반 13개는 StoryStage에 기존 `presentation.visual_id`를 그대로 한 번 넣는다.
- 그림이 아직 asset manifest에 없으면 `placeholder: true`, 빈 문자열이 아닌 완전한
  한국어 장면 설명 `alt`를 사용한다. `이벤트 이름.png`를 실제 파일 경로처럼 쓰지 않는다.
- Collapse gate도 illustration block을 StoryStage에 넣되, Encounter의
  `presentation.layout: collapse_gate`는 유지한다. Web renderer의
  `storyPhase()`는 `page.visual.kind`를 먼저 확인하므로 staged stream에서도 붕괴
  vignette가 유지된다.

## 4. 14개 Encounter authoring map

계획서의 WP 순서와 YAML 선언 순서를 혼동하지 않는다. 구현자는 기존 YAML 순서를
재배치하지 않고, 아래 목록의 범위 순서대로 확인한다. Encounter selector의 기존
조건/가중치 순서를 보존하는 것이 우선이다.

| 순서 | Encounter | 선택 수 | 설계 메모 |
|---:|---|---:|---|
| 1 | `wuxia_sado_final_phase_1_price_tag` | 4 | 가격표 진입 서사와 장부 그림을 Story에 배치 |
| 2 | `wuxia_cheonoe_analysis_thread_phase1_bridge` | 2 | 복기 루프를 Story로 열고 기존 분석 hook/log 유지 |
| 3 | `wuxia_sado_final_phase_2_weakpoint_control` | 4 | 서하린·무명·천기록·사도 약점 선택을 각각 Result로 분리 |
| 4 | `wuxia_sado_final_phase_3_outside_calculation` | 5 | 계산식 밖/사도식 계산 양쪽 결과를 모두 authored |
| 5 | `wuxia_sado_battle_loss_route_bridge` | 2 | 패배 seed를 engine outcome에 맡기고 authored beat 유지 |
| 6 | `wuxia_boss_resolution` | 5 | final result priority seed를 narrate하지 않고 outcome이 방출 |
| 7 | `wuxia_mumyeong_resolution` | 6 | 무명 resolution candidate를 outcome flags/clues로 보존 |
| 8 | `wuxia_seoharin_qingliu_resolution` | 5 | 산문/재건/압박 결과를 기존 route seed로 유지 |
| 9 | `wuxia_seoharin_unsaid_stay` | 4 | 귀환·정착·불확실·회피 intent와 기존 hook 보존 |
| 10 | `wuxia_cheongirok_resolution` | 5 | 천기록 직접 반응 문장만 document/cheongirok로 표현 |
| 11 | `wuxia_black_serpent_aftermath` | 5 | 후일담 seed를 출력하지 않고 기존 outcome만 authored |
| 12 | `wuxia_return_modern_commute_scene` | 3 | final ending consumer가 supersede할 수 있어도 Result authored |
| 13 | `wuxia_settlement_stay_scene` | 3 | 정착 후일담 consumer가 supersede할 수 있어도 Result authored |
| 14 | `wuxia_collapse_gate` | 2 | 아래의 전용 설계를 사용; choices 순서가 UI 계약 |

각 표의 선택 수는 현재 `origin/main`의 legacy choice count를 기준으로 한다. 구현 중
count가 달라지면 콘텐츠를 임의로 맞추지 말고, 원본 diff와 route-parity를 먼저 검토한다.

## 5. Collapse gate 전용 설계

### 5.1 Stage graph

```text
collapse_story (story)
  ├─ narration: 기존 붉게 번지는 획 본문 verbatim
  └─ illustration: visual_id=wuxia_collapse_gate, 기존 alt/placeholder 규칙
       ↓ event:continue
collapse_choice (choice)
  ├─ wuxia_collapse_revive → wuxia_collapse_revive_result
  └─ wuxia_collapse_rest   → wuxia_collapse_rest_result
       ↓ choice:<id>
<choice>_result (result)
  ├─ result_summary
  └─ 기존 outcome log verbatim
       ↓ event:continue (단, ending이 우선하면 다음 view에서 ending)
```

`collapse_choice`의 순서는 반드시 기사회생 first, 안식 last다. 기존 CSS의
`.storybook-choices li:first-child / li:last-child` styling contract가 이 순서를
의미 있게 사용한다.

### 5.2 Engine facts와 테스트 경계

엔진 변경을 전제로 하지 않는다. 현재 구현의 동작을 다음 테스트로 고정한다.

1. `current_content_encounter`는 `active_event_id`를 collapse pending보다 먼저
   반환한다. health가 0이 된 시점이 staged Event 중간이어도 active Event의 cursor를
   끝까지 진행하고, Event 종료 후에만 `wuxia_collapse_gate` StoryStage를 반환한다.
2. 기사회생은 기존 outcome으로 health를 40 올리고 `used_flag`를 설정한다. ResultStage가
   보인 뒤 Event가 끝나며 정상 play로 돌아가고, used flag 때문에 gate가 재발동하지
   않는다.
3. 안식은 기존 outcome으로 `used_flag`와 death flag를 설정한다. ResultStage를
   authoring하지만, 다음 `turn_view`에서는 `wuxia_death_rest` ending이 우선한다.
   결과가 안 보이는 것은 승인된 ending-supersede 동작이며, action rejection loop를
   만들기 위한 engine 보정은 하지 않는다.
4. `current_content_ending`의 collapse pending 차단은 그대로 둔다. health 0이고
   used flag가 아직 없을 때 death ending이 gate를 건너뛰지 않아야 한다.

## 6. Finale ending supersede 설계

최종 결산 Encounter의 outcome이 final epilogue 조건을 완성하면 다음 view가
ResultStage 대신 기존 `wuxia_final_epilogue_renderer_contract` ending이 되는 것이
정상이다. ResultStage는 문법·action beat·save cursor 계약 때문에 항상 authoring한다.

전용 core 회귀 테스트는 다음을 수행한다.

1. preview bundle을 index하고, 마지막 결산 Encounter인
   `wuxia_black_serpent_aftermath`의 choice stage로 cursor를 진입시킨다.
2. 상태에 기존 final precondition flags
   (`boss_resolution_resolved`, `mumyeong_resolution_resolved`,
   `seoharin_qingliu_resolution_resolved`, `black_serpent_aftermath_resolved`,
   `final_result_priority_applied_seeded`, `final_state_routing_seeded`)를
   설정하고 현재 Encounter가 방출하는 `*_resolution_resolved`만 선택 outcome으로
   추가한다.
3. 선택 직후 `turn_view_from_content`가 `ending_id ==
   wuxia_final_epilogue_renderer_contract`를 반환하는지 확인한다. 선택 action이
   unknown으로 거부되지 않고, 다음 view가 ending으로 안정적으로 수렴하는 것이
   pass 조건이다.

이 테스트는 `final_epilogue.rs`, ending YAML, renderer contract를 수정하지 않고
현재 priority를 검증한다. 구현 중 precondition이 실제 YAML과 불일치하면 테스트
fixture를 추측으로 늘리지 말고, `docs/design/Wuxia_Final_State_Routing.md`의
canonical inputs를 기준으로 고친다.

## 7. 검증 계획

### 자동 검증

1. `cargo fmt --all`
2. `cargo test --workspace`
3. focused core: `event_stage`, `event_stage_wave1`, `event_stage_wave2`, 새
   `event_stage_wave3`, `route_parity`, collapse contract/json contract
4. `.venv/bin/pytest -q tests/test_web_data_export.py tests/test_docs_contract.py`
5. `python3 scripts/export_web_data.py --check`와 두 wuxia preview bundle `--check`
6. `git diff --check`
7. `cd web && npm test && npx tsc --noEmit && npm run build`
8. `wasm-pack build crates/escape-wasm --target web --out-dir web/src/core/wasm-pkg`
9. `npm run qa:storybook:visual -- --require-wasm`의 390×844, 414×896, 800×1440,
   810×1644, 1440×1000

### Wave 3 guard test

새 `event_stage_wave3.rs`는 고정 목록 14개를 대상으로 다음을 검사한다.

- 첫 stage가 `story`, 둘째가 `choice`, 각 event가 illustration을 정확히 1개
  보유하고 모든 illustration alt가 비어 있지 않음
- staged choice action IDs와 legacy choice IDs의 집합이 동일함
- 14개 선택지에 `check`가 없고, 어떠한 block에도 `branch`가 없음
- legacy outcome log가 해당 선택 ResultStage의 authored text에 그대로 존재함
- `wuxia_cheongirok_resolution`의 document surface와 narration speaker 규칙
- collapse gate choice 순서, mid-event health-zero, revive/no-retrigger, rest/death ending
- Wave 3 완료 후 wuxia preview 전체 44개 Encounter에 `event`가 존재함

### 수동 acceptance

- 390px에서 collapse vignette가 staged Story 화면에도 활성화되고 두 카드가 시각적으로
  구분됨
- 기사회생 후 Result beat → 정상 play, 안식 후 death ending/restart button
- 한 finale resolution에서 choice → beat → ResultStage 또는 합법적인 ending supersede가
  한 번만 발생하고 stuck state가 없음
- ResultStage 내부에서 save/reload한 뒤 `active_event_id`, `event_stage_index`,
  `event_next_stage_id`가 같은 cursor를 복원함

WSL에서 Chrome 설치가 계속 불가능하면 다섯 viewport 자동 QA까지만 실행하고, 위 네
항목은 report에 `deferred: Chrome unavailable in WSL`로 명시하여 Fable review에서
수행한다.

## 8. 구현 순서와 산출물

구현은 다음 독립 단계로 나눈다. 각 단계 종료 후 bundle과 테스트가 유효해야 한다.

1. **WP-S1**: collapse 전용 core 회귀 테스트와 Wave 3 test harness 준비. 기존 engine
   ordering을 보정하지 않는다.
2. **WP-C1**: 표의 1–7번을 plan 순서로 staged authoring하고 core preview bundle을
   재생성한다.
3. **WP-C2**: 표의 8–13번을 staged authoring하고 ending-supersede guard를 확장한다.
4. **WP-C3**: 14번 collapse gate와 44/44 full-coverage guard를 추가한다.
5. **WP-W1**: generated Web bundle, WASM, five-viewport QA.
6. **WP-D1**: `fable_eventstage_wave3_step2_report.md`, `Development_Plan.md`,
   `Checklist.md`를 실제 검증 수치로 갱신한다.
7. **WP-D2**: Notion runtime ledger에 baseline, 44/44 coverage, collapse/ending
   precedence, 수동 QA 보류를 reverse-sync한다. Notion이 불가하면 report에 pending을
   남긴다.

주요 산출물은 `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`,
두 generated preview bundle, `crates/escape-core/tests/event_stage_wave3.rs`,
collapse 회귀 테스트, Wave 3 report 및 개발 문서다. art/epilogue renderer 파일은
변경 목록에 나타나면 범위 위반으로 되돌린다.

## 9. 위험과 rollback

- **Ending 조건 drift**: 실제 ending precondition이 설계 표와 다르면 canonical
  `Wuxia_Final_State_Routing.md`와 YAML을 다시 대조하고, content flag를 새로 만들지
  않는다.
- **Encounter ordering drift**: YAML 항목을 재정렬하지 않고 조건/forbidden flag와
  route parity를 비교한다.
- **ResultStage 미표시 오해**: ending supersede는 정상 precedence다. renderer나
  final epilogue contract를 우회해 결과를 강제로 보여 주지 않는다.
- **Asset drift**: 모든 미매핑 visual은 placeholder로 남긴다. 이 설계의 rollback은
  `event` block만 제거하고 legacy fields를 그대로 두면 되며, save/schema migration은
  필요하지 않다.

## 10. 승인 후 handoff

이 문서가 승인되면 `origin/main`에서 구현 branch를 만들고 WP-S1 → C1 → C2 → C3 →
W1 → D1 → D2 순서로 실행한다. 구현 완료 후 `/check`로 diff·테스트·generated
artifact·remote 상태를 검토하고, 사용자가 요청할 때만 commit/push/PR을 수행한다.

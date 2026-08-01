# Event/Stage/ContentBlock closeout — Step 2 구현 보고서

작성: 2026-08-01
구현: coding subagent (sonnet)
플랜: `fable_eventstage_closeout_step1_2608012336.md`

## 요약

WP-1 ~ WP-5를 플랜 순서대로 구현했다. 커밋/push/branch 전환은 하지 않았다 (working tree만 수정).
최종 `cargo test --workspace --no-fail-fast` = **249 passed / 0 failed** (baseline 246 + 신규 3, 목표치와 일치).
`crates/escape-terminal/tests/cli_smoke.rs`는 읽지도 수정하지도 않았고, WP-4 반영 후에도 61/61 그대로 통과했다 (이 파일에 표시된 uncommitted `M`은 플랜에 적힌 대로 다른 작업자 소유 변경이며 이 세션이 만든 것이 아니다).

## WP-1 — `EventStageDef::visible_blocks` 추가

파일: `crates/escape-core/src/content.rs` (수정)

플랜에 적힌 코드 그대로 `impl EventStageDef { pub fn visible_blocks(...) -> impl Iterator<Item = &ContentBlockDef> }`를 `EventChoiceRef` 정의 앞에 추가했다. 새 struct/enum/필드 없음.

검증:
- `cargo fmt --all -- --check` → 출력 없음 (통과)
- `cargo test -p escape-core` → 17개 test binary 전부 `ok`, 합계 **149 passed / 0 failed** (당시 workspace 기준 escape-core만)

## WP-2 — `scene_content_stream` 리팩터

파일: `crates/escape-core/src/scene_page.rs` (수정)

인라인 branch 필터(`.filter(|block| { ... })` 블록)를 삭제하고
```rust
stage.visible_blocks(state.last_check.as_ref().map(|check| check.success))
```
로 교체했다. `SceneContentItem` 매핑, `choice`/`continue` sentinel, flat encounter fallback은 그대로 두었다. 순수 리팩터.

검증:
- `cargo fmt --all -- --check` → 통과
- `cargo test -p escape-core --test event_stage --test event_stage_wave1` → `event_stage`: 11 passed, `event_stage_wave1`: 3 passed (전부 0 failed). `heuksa_checked_result_streams_keep_only_the_matching_branch` 계속 통과.
- `cargo test --workspace --no-fail-fast` → **246 passed / 0 failed** (baseline 그대로, 출력 무변화 확인)

## WP-3 — 회귀 테스트 작성 + red 확인 (필수)

파일: `crates/escape-core/tests/event_stage_branch_surfaces.rs` (신규)

테스트 3개 작성:
1. `result_stage_turn_view_body_keeps_only_matching_branch`
2. `result_stage_scene_page_body_blocks_and_dialogue_keep_only_matching_branch`
3. `result_stage_without_check_resolution_keeps_only_common_blocks` (office `content.bundle.json`에 `serde_json`으로 `event`를 주입, fixture 파일 자체는 수정하지 않음)

WP-4 수정 **전에** 실행해 3개 전부 FAIL하는 것을 확인했다.

### red 출력 원문 (WP-4 이전, `cargo test -p escape-core --test event_stage_branch_surfaces`)

```
running 3 tests
test result_stage_without_check_resolution_keeps_only_common_blocks ... FAILED
test result_stage_scene_page_body_blocks_and_dialogue_keep_only_matching_branch ... FAILED
test result_stage_turn_view_body_keeps_only_matching_branch ... FAILED

failures:

---- result_stage_without_check_resolution_keeps_only_common_blocks stdout ----

thread 'result_stage_without_check_resolution_keeps_only_common_blocks' panicked at crates/escape-core/tests/event_stage_branch_surfaces.rs:167:5:
assertion failed: !body.contains("성공 분기 결과")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- result_stage_scene_page_body_blocks_and_dialogue_keep_only_matching_branch stdout ----

thread 'result_stage_scene_page_body_blocks_and_dialogue_keep_only_matching_branch' panicked at crates/escape-core/tests/event_stage_branch_surfaces.rs:101:5:
assertion failed: !success_body_texts.iter().any(|t| t.contains(FAILURE_TEXT))

---- result_stage_turn_view_body_keeps_only_matching_branch stdout ----

thread 'result_stage_turn_view_body_keeps_only_matching_branch' panicked at crates/escape-core/tests/event_stage_branch_surfaces.rs:76:5:
assertion failed: !success_body.contains(FAILURE_TEXT)

failures:
    result_stage_scene_page_body_blocks_and_dialogue_keep_only_matching_branch
    result_stage_turn_view_body_keeps_only_matching_branch
    result_stage_without_check_resolution_keeps_only_common_blocks

test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

이 실패는 플랜에 적힌 결함(성공/실패 분기 텍스트가 `TurnView.body`/`ScenePage.body_blocks`/`dialogue_entries`에 동시 노출)을 정확히 재현한다.

## WP-4 — `event_stage_text`에 branch 해석 적용 (green)

파일: `crates/escape-core/src/turn.rs` (수정)

- `fn event_stage_text(stage: &crate::content::EventStageDef) -> String` → `fn event_stage_text(stage: &crate::content::EventStageDef, state: &GameState) -> String`로 변경, 내부에서 `stage.visible_blocks(state.last_check.as_ref().map(|check| check.success))`를 순회하도록 수정. `join("\n\n")` 유지.
- 호출부 `turn.rs:188-189`를 `stage.map(|stage| event_stage_text(stage, state))`로 변경. 다른 호출부 없음 (`event_stage_text`는 private 유지, 새 public API 없음).

### green 출력 (`cargo test -p escape-core --test event_stage_branch_surfaces`)

```
running 3 tests
test result_stage_without_check_resolution_keeps_only_common_blocks ... ok
test result_stage_scene_page_body_blocks_and_dialogue_keep_only_matching_branch ... ok
test result_stage_turn_view_body_keeps_only_matching_branch ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

추가 확인:
- `cargo fmt --all -- --check` → 통과 (출력 없음)
- `cargo test -p escape-core --test event_stage --test event_stage_wave1 --test event_stage_wave3` → 11 + 3 + 8 = 22 passed, 0 failed
- `cargo test --workspace --no-fail-fast` (전체) → 아래 test binary별 결과, 총 **249 passed / 0 failed**:

```
escape-core unittests            0 passed
combat_conclusion_wave2          4 passed
combat_contract_wave1            5 passed
combat_execution_wave2           6 passed
combat_opportunity_wave1        12 passed
combat_resolution_wave2         11 passed
combat_simulation_wave2         10 passed
combat_state_wave1               8 passed
content_bundle                   9 passed
core_contract                   32 passed
event_stage                     11 passed
event_stage_branch_surfaces      3 passed  (신규)
event_stage_wave1                3 passed
event_stage_wave2                3 passed
event_stage_wave3                8 passed
reward_pipeline_wave1             4 passed
route_parity                     23 passed
escape-terminal unittests         0 passed
cli_smoke                        61 passed  (수정 안 함, 그대로 통과)
escape_wasm unittests             0 passed
json_contract                    36 passed
doc-tests (2개 크레이트)           0 passed

합계: 249 passed / 0 failed
```

**주의사항 관련**: `cli_smoke.rs`의 기대 출력은 WP-4 반영 후에도 깨지지 않았다 (61/61 그대로 통과). 파일은 읽지도 수정하지도 않았다.

- `git diff --check` → exit 0 (whitespace 오류 없음)

## WP-5 — canonical 문서 truth-up

수정 파일 4개, 새 계획 발명 없이 플랜에 적힌 사실만 반영:

1. **`docs/dev/Data_Schema.md`** (87~91행 부근) — "core가 `content_stream`을 만들 때만 해석한다" → "core가 해당 stage의 텍스트 표면(`content_stream`, `TurnView.body`, `ScenePage.body_blocks`·`dialogue_entries`)을 만들 때 동일하게 해석한다"로 수정하고 해석 지점이 `EventStageDef::visible_blocks` 한 곳임을 명시. 나머지 규칙 문장은 그대로 유지.
2. **`docs/dev/Development_Plan.md`**:
   - 130행(combat 항목)에 Wave 2 Step 4 conclusion sidecar 구현/검증 완료 사실과 남은 미착수 항목(조기 결착/tick 중단, 패주·항복·증원·대형·결속·배경 전투, 종료 narrative/report consumer, Wave 3 Step 1/2, 밸런스)을 추가.
   - 192행(Event/Stage closeout 최우선 항목)을 재작성: 2~6번 완료 근거를 파일:라인+테스트명으로 압축, `event_stage.rs`/`event_stage_wave1.rs`/`event_stage_wave2.rs`/`event_stage_wave3.rs`(51/51)/`render.test.ts` 명시, 이번 slice가 닫은 branch 누출 결함 1~2문장 기록, legacy adapter 결정(별도 adapter 모듈 없음, `scene_page.rs:537-586` fallback, office pack staged 전환 전까지 신규 adapter 없음) 기록, 남은 closeout 항목 3개(수동 acceptance / Notion reverse sync / office·isolation pack staged 마이그레이션)로 정리.
   - 190행 뒤에 44/44 → 51/51 각주 추가 (190행 원문은 삭제하지 않음).
3. **`docs/design/Event_Stage_Content_Model.md`**:
   - 45행 "adapter로 해석할 수 있다" 뒤에 현재 구현이 별도 adapter 모듈이 아니라 core의 ordered stream fallback이며 Encounter selection 의미가 유지된다는 문장 추가.
   - 62행(branch 섹션) "renderer는 content_stream을 받은 뒤 branch를 다시 판정하지 않는다"는 유지하고 core의 모든 텍스트 표면이 같은 필터를 쓴다는 문장 추가.
4. **`docs/design/Combat_System_Implementation_Plan_Index.md`**:
   - `status: wave2-step4-complete`, 17행 Wave 2 Step 4 서술, 단계 순서 표의 `fable_combat_wave2_step4_2607261845.md` 행은 이미 PR #176/#177 반영분으로 존재했다 (재작성 불필요, 사실 확인만 함).
   - 유일하게 빠져 있던 "구현 위치 1줄"을 추가: `Wave 2 Step 4 구현 위치: crates/escape-core/src/combat_conclusion.rs, crates/escape-core/tests/combat_conclusion_wave2.rs.` (두 파일 존재 확인 완료)

### 문서 크기 확인 (`wc -c`, 100KB 이하)

```
32991 docs/dev/Data_Schema.md
46440 docs/dev/Development_Plan.md
 7654 docs/design/Event_Stage_Content_Model.md
 3895 docs/design/Combat_System_Implementation_Plan_Index.md
```

모두 100,000바이트 미만.

WP-5 후 최종 `cargo test --workspace --no-fail-fast` 재실행 → **249 passed / 0 failed** (WP-4와 동일, 문서만 변경했으므로 회귀 없음). `git diff --check` → exit 0.

## 스킵/이탈 항목

없음. WP-1~WP-5 전부 계획대로 완료했고, 위험 신호(레드 확인 없는 진행, `cli_smoke.rs` 깨짐, hard invariant 위반)는 발생하지 않았다.

## 명시적 범위 밖 (플랜대로 손대지 않음)

- office/isolation pack 21개 encounter의 staged Event 전환
- `EventStageDef.kind`/`ContentBlockDef.kind`의 enum 타입화
- 새 ContentBlock taxonomy, `cheongirok` block 의미 변경
- Notion reverse sync 실제 수행
- 수동 acceptance 플레이
- Web/Storybook DOM·CSS 변경, wasm 재빌드, 5뷰포트 QA
- `crates/escape-terminal/tests/cli_smoke.rs` 수정
- combat Wave 3 Step 1/2 구현
- 밸런스 수치 확정
- commit/push/PR 생성

## 최종 `git status --short -uall` / `git diff --stat`

```
 M crates/escape-core/src/content.rs
 M crates/escape-core/src/scene_page.rs
 M crates/escape-core/src/turn.rs
 M crates/escape-terminal/tests/cli_smoke.rs
 M docs/design/Combat_System_Implementation_Plan_Index.md
 M docs/design/Event_Stage_Content_Model.md
 M docs/dev/Data_Schema.md
 M docs/dev/Development_Plan.md
?? .claude/worktrees/caveman-repo-sync-8a6b94/
?? crates/escape-core/tests/event_stage_branch_surfaces.rs
?? fable_eventstage_closeout_step1_2608012336.md
---
 crates/escape-core/src/content.rs                                  | 25 ++++++++++++++++++++++
 crates/escape-core/src/scene_page.rs                               | 16 +-------------
 crates/escape-core/src/turn.rs                                     |  7 +++---
 crates/escape-terminal/tests/cli_smoke.rs                          | 12 -----------
 docs/design/Combat_System_Implementation_Plan_Index.md             |  2 ++
 docs/design/Event_Stage_Content_Model.md                           |  4 ++--
 docs/dev/Data_Schema.md                                            |  6 ++++--
 docs/dev/Development_Plan.md                                       |  6 ++++--
 8 files changed, 41 insertions(+), 37 deletions(-)
```

`crates/escape-terminal/tests/cli_smoke.rs`의 `M` 상태와 `.claude/worktrees/caveman-repo-sync-8a6b94/`는 이 세션이 만든 변경이 아니다 (플랜의 baseline 서술과 일치, 이 세션은 두 경로 모두 읽지도 쓰지도 않았다).

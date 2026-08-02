# Combat Wave 3 Step 1c — 구현 보고서

작성: 2026-08-02
구현 담당: coding subagent (sonnet)
플랜: `fable_combat_wave3_step1c_2608021109.md`
브랜치: `claude/combat-wave3-step1a-v2` (baseline `2a0e460`)

## 요약

WP-1~WP-4를 순서대로 구현·검증·커밋했다. 기존 테스트는 하나도 깨지지 않았고,
`ScenePage.combat`이 `None`일 때 기존 JSON은 바이트 단위로 동일함을 신규 테스트로
고정했다. `cargo test --workspace --no-fail-fast` 최종 결과: **294 passed / 0 failed**
(baseline 287 + WP-1 1개 + WP-3 6개 신규 테스트).

## WP-1 — `CombatSpectatorView.simulation_version`

변경 파일:
- `crates/escape-core/src/combat_spectator.rs` — `simulation_version: CombatSimulationVersion` 필드 추가, `spectate()`에서 기존 `tick_millis` provenance 읽기 지점(`request.resolution.execution.provenance`)에서 함께 파생하도록 수정. 두 번 읽지 않는다 — `provenance` 참조를 한 번만 얻고 `tick_millis`/`simulation_version` 둘 다 그 참조에서 꺼낸다.
- `crates/escape-core/tests/combat_spectator_wave3.rs` — `view_reports_the_simulation_version_from_provenance` 테스트 추가.

### Red 출력 (테스트를 먼저 작성한 뒤 확인)

```
$ cargo test -p escape-core --test combat_spectator_wave3
   Compiling escape-core v0.1.0 (/home/dudu/work/tui-adv/crates/escape-core)
error[E0609]: no field `simulation_version` on type `CombatSpectatorView`
   --> crates/escape-core/tests/combat_spectator_wave3.rs:479:14
    |
479 | ...ew.simul...rsion,
    |       ^^^^^...^^^^^ unknown field
    |
    = note: available fields are: `resolution_fingerprint`, `tick_millis`, `frames`, `core_log`, `full_log`, `fingerprint`

For more information about this error, try `rustc --explain E0609`.
error: could not compile `escape-core` (test "combat_spectator_wave3") due to 1 previous error
```

구현 후 green: `cargo test -p escape-core --test combat_spectator_wave3` → **20 passed; 0 failed**
(기존 19개 + 신규 1개). `cargo test --workspace --no-fail-fast` → **288 passed; 0 failed**
(baseline 287 + 1).

검증 명령 실행 결과: `cargo fmt --all -- --check` 통과(무출력), `git diff --check` 통과.

커밋: `67f90f8 feat(combat): expose simulation_version on CombatSpectatorView`

## WP-2 — `CombatSpectatorPage`와 `ScenePage.combat`

변경 파일:
- `crates/escape-core/src/combat_spectator.rs` — `CombatSpectatorPage { view, report: Option<CombatConclusionReport> }` 정의 (`#[serde(default, skip_serializing_if = "Option::is_none")]`를 `report`에 부착). `CombatConclusionReport` import 추가.
- `crates/escape-core/src/lib.rs` — `CombatSpectatorPage` re-export 추가.
- `crates/escape-core/src/scene_page.rs` — `ScenePage.combat: Option<CombatSpectatorPage>` 필드 추가(`#[serde(default, skip_serializing_if = "Option::is_none")]`), `scene_page_from_turn_view`(`ScenePage` 리터럴 생성 지점, 원래 `:383` 근방, 이번 slice 편집으로 위치가 살짝 밀렸다)에 `combat: None`만 추가. `scene_page_from_content`에는 combat 생산 로직을 넣지 않았다.

검증: `cargo fmt --all -- --check` 통과, `cargo test -p escape-core --test combat_spectator_wave3` → 20 passed(변화 없음, 컴파일만 확인), `cargo test --workspace --no-fail-fast` → **288 passed; 0 failed** (WP-2는 신규 테스트를 추가하지 않았으므로 수치 유지 — 기존 테스트가 하나도 깨지지 않았음을 확인). `git diff --check` 통과.

커밋: `1a16b60 feat(combat): add CombatSpectatorPage and ScenePage.combat boundary`

## WP-3 — JSON 경계 회귀 테스트

신규 파일 `crates/escape-core/tests/scene_page_combat_boundary.rs` (5개 테스트 함수, 플랜의 6개 assertion을 모두 포함):
- `content_backed_scene_page_has_no_combat_producer_and_no_combat_key_in_json` — assertion 1 & 2 (combat이 `None`, JSON에 `"combat"` 키 없음)
- `filled_combat_serializes_with_simulation_version_alongside_fingerprint` — assertion 3 (`combat` 채우면 키 등장, `view.simulation_version`·`view.fingerprint` 함께 존재)
- `filled_combat_scene_page_round_trips_through_serde` — assertion 4
- `scene_page_json_missing_combat_key_deserializes_to_none_without_error` — assertion 5
- `combat_spectator_page_with_no_report_omits_report_key` — assertion 6

`crates/escape-wasm/tests/json_contract.rs`에 신규 테스트 1개 추가 (기존 테스트 본문은 무수정):
- `json_boundary_scene_page_has_no_combat_key_before_combat_authoring_exists`

검증:
- `cargo fmt --all -- --check` → `scene_page_combat_boundary.rs`에 diff 있어 `rustfmt`로 그 파일만 정리 후 재확인, 통과.
- `cargo test -p escape-core --test scene_page_combat_boundary` → **5 passed; 0 failed**.
- `cargo test -p escape-wasm --test json_contract` → **37 passed; 0 failed** (기존 36 + 신규 1).
- `cargo test -p escape-core --test event_stage_branch_surfaces` → **3 passed; 0 failed** (무변경, 회귀 없음 확인).
- `cargo test --workspace --no-fail-fast` → **294 passed; 0 failed** (288 + 5 + 1).
- `git diff --check` 통과.

커밋: `42dcb2a test(combat): lock ScenePage.combat JSON boundary invariants`

### 기존 ScenePage JSON 무변경 근거

`content_backed_scene_page_has_no_combat_producer_and_no_combat_key_in_json`이
`scene_page_from_content`가 만든 실제 콘텐츠 기반 `ScenePage`를 `serde_json::to_value`로
직렬화한 뒤 `value.as_object().unwrap().get("combat").is_none()`을 assert한다 — 즉 이 slice가
추가되기 전 콘텐츠 번들로 만든 페이지의 JSON과 `"combat"` 키 유무를 제외하면 구조가 동일함을
고정한다. WASM 경계에서도 `json_boundary_scene_page_has_no_combat_key_before_combat_authoring_exists`가
`new_game_json`/`scene_page_json`이 만든 실제 JSON 문자열에 `"combat"` 키가 없음을 확인한다.
두 테스트 모두 `cargo test --workspace --no-fail-fast`의 294 passed에 포함되어 있고, 이 slice
이전에 존재하던 76개(escape-core 61 content_tui + escape-wasm 36 json_contract) 시나리오
테스트도 모두 그대로 통과했다(회귀 없음).

## WP-4 — 문서 갱신 + 단계 순서 조정

변경 파일과 요지:
- `docs/dev/Data_Schema.md` — `ScenePage` 필드표에 `combat` 행 추가, `### ScenePage.combat — 전투 관전 boundary (Wave 3 Step 1c)` 절 신설: optional/키 부재 규칙, producer 부재 사유, renderer 표시 전용(template_id, 로그 문장 아님), fingerprint·simulation_version 페어링 계약, `report`도 optional임을 기술하고 위 테스트 함수명을 인용.
- `docs/design/Combat_System_Implementation_Plan_Index.md` — `status: wave3-step1c-complete`로 갱신. "현재 코드와 정본의 경계" 문단에 Wave 3 Step 1c 확보분 문장 추가, "다음 계약은 아직 없다" 목록에 "전투를 시작하는 인카운터 authoring — `ScenePage.combat`의 producer가 없어 현재 항상 `None` → Wave 3 Step 2" 항목 추가(근거 포함). 단계 표에서 `(플랜 미작성) — Wave 3 Step 1c` 행을 `fable_combat_wave3_step1c_2608021109.md`로 교체하고, **`Step 2` 행을 `Step 1d` 앞으로 이동**(순서: `… → 1c → Step 2 → Step 1d`), 표 아래에 근거 한 줄 추가. 구현 위치 목록에 Step 1c 줄 추가, Step 5 줄의 테스트 수 각주를 20으로 갱신.
- `docs/dev/Combat_System_Operating_Guide.md` — Step 6 항목 뒤에 Step 1c 완료 항목(플랜 파일명, 구현/테스트 파일, 설명) 추가, 기존 형식(플랜 파일 → 소스 파일 → 테스트 파일 → 설명 문장) 그대로 따름.
- `docs/dev/Combat_System_Goal_Prompt.md` — baseline 목록에 Step 1c 완료 문장 추가, "아직 미구현" 문장에서 `ScenePage`/WASM 항목을 "전투를 여는 인카운터 authoring(producer)"으로 교체, "권장 다음 goal 문장"을 Step 1c 완료 반영 + Step 2를 Step 1d보다 먼저 진행하라는 순서로 갱신.

문서 크기 (`wc -c`, 100KB 이하 확인):
```
 35428 docs/dev/Data_Schema.md
 12520 docs/design/Combat_System_Implementation_Plan_Index.md
 17883 docs/dev/Combat_System_Operating_Guide.md
 10480 docs/dev/Combat_System_Goal_Prompt.md
```

검증: `cargo fmt --all -- --check` 통과, `git diff --check` 통과,
`cargo test --workspace --no-fail-fast` → **294 passed; 0 failed** (문서만 바꿨으므로 수치 유지).

커밋: `8af0d91 docs(combat): record Wave 3 Step 1c and reorder Step 2 before Step 1d`

## 최종 workspace 검증 (WP-4 이후, 최종 상태)

```
cargo fmt --all -- --check      -> 통과 (무출력)
git diff --check                -> 통과 (무출력)
cargo test --workspace --no-fail-fast -> 294 passed; 0 failed (모든 crate 합산)
```

주요 개별 스위트:
- `cargo test -p escape-core --test combat_spectator_wave3` → 20 passed; 0 failed
- `cargo test -p escape-core --test scene_page_combat_boundary` → 5 passed; 0 failed
- `cargo test -p escape-core --test event_stage_branch_surfaces` → 3 passed; 0 failed
- `cargo test -p escape-wasm --test json_contract` → 37 passed; 0 failed

## 스킵/이탈 항목

없음. 플랜의 WP-1~WP-4, 최종 체크리스트 전 항목을 구현했다. `crates/escape-terminal/`,
`web/src/`, `.claude/worktrees/`, `Cargo.toml`, fixture/generated JSON은 손대지 않았다.
`crates/escape-terminal/tests/cli_smoke.rs`의 다른 작업자 uncommitted 변경은 `git status`에서
계속 modified 상태로 보존됐다(읽기만 했고 add/commit하지 않음).

## 최종 git 상태

```
$ git status --short -uall
 M crates/escape-terminal/tests/cli_smoke.rs
?? .claude/worktrees/caveman-repo-sync-8a6b94/
?? fable_combat_wave3_step1c_2608021109.md

$ git diff --stat 2a0e460..HEAD
 crates/escape-core/src/combat_spectator.rs         |  30 +++--
 crates/escape-core/src/lib.rs                      |   3 +-
 crates/escape-core/src/scene_page.rs               |   8 ++
 crates/escape-core/tests/combat_spectator_wave3.rs |  10 ++
 .../tests/scene_page_combat_boundary.rs            | 139 +++++++++++++++++++++
 crates/escape-wasm/tests/json_contract.rs          |  18 +++
 .../Combat_System_Implementation_Plan_Index.md     |  16 +--
 docs/dev/Combat_System_Goal_Prompt.md              |   5 +-
 docs/dev/Combat_System_Operating_Guide.md          |   6 +
 docs/dev/Data_Schema.md                            |  41 ++++++
 10 files changed, 259 insertions(+), 17 deletions(-)

$ git log --oneline -6
8af0d91 docs(combat): record Wave 3 Step 1c and reorder Step 2 before Step 1d
42dcb2a test(combat): lock ScenePage.combat JSON boundary invariants
1a16b60 feat(combat): add CombatSpectatorPage and ScenePage.combat boundary
67f90f8 feat(combat): expose simulation_version on CombatSpectatorView
2a0e460 docs(combat): track Wave 2 Step 6 plan and implementation report
8387402 docs(combat): record Wave 2 Step 6 provenance slice and resolve fingerprint precondition
```

diff --stat의 파일 목록은 플랜의 "예상 변경 파일" 표와 정확히 일치한다(9개 예상 파일 + 신규
`scene_page_combat_boundary.rs`).

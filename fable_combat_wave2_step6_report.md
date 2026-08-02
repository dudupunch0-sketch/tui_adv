# Combat Wave 2 Step 6 — 구현 보고서

작성: 2026-08-02
구현: coding subagent (sonnet)
plan: `fable_combat_wave2_step6_2608020838.md`

## 요약

WP-1~WP-5를 순서대로 구현했다. `CombatExecutionResult.provenance`
(`CombatProvenance { simulation_version, tick_millis, manifest_fingerprint }`)를
추가하고, `CombatSpectatorRequest.tick_millis`/`CombatConclusionRequest.tick_millis`
중복 파라미터와 두 `InvalidTickMillis` 에러 변형을 제거해 `MissingProvenance`로
대체했다. 최종 `cargo test --workspace --no-fail-fast` = **287 passed / 0 failed**
(baseline 283 + WP-2에서 추가한 4개 신규 테스트).

## WP-1 — `CombatProvenance` 정의와 채우기

변경 파일:
- `crates/escape-core/src/combat_execution.rs` — `CombatProvenance` struct 정의,
  `CombatExecutionResult.provenance: Option<CombatProvenance>` (`#[serde(default)]`)
  추가, `execute()`가 `manifest.fingerprint()`를 호출해 항상 `Some(..)`으로 채움.
  실패 시 기존 `CombatExecutionError::InvalidInput`으로 매핑(새 변형 없음).
- `crates/escape-core/src/lib.rs` — `CombatProvenance` re-export 추가.
- `crates/escape-core/tests/combat_conclusion_wave2.rs` — **예상 밖 필요 조치**:
  `CombatExecutionResult` 리터럴을 직접 구성하는 두 fixture(`resolution()`,
  `multi_resolution()`)가 새 `provenance` 필드 없이는 컴파일이 안 되어(Rust
  struct 리터럴은 필드 생략을 허용하지 않음 — `#[serde(default)]`는 역직렬화에만
  적용된다), 두 곳에 `provenance: None,`을 기계적으로 추가했다. 논리 변경은
  없고(이 시점에는 아무도 provenance를 읽지 않음), WP-4에서 `Some(..)`으로
  다시 바뀐다.

`effective_seed`/`namespace` 결정 로직과 `stable_fingerprint(&(...))` 호출부의
인자 튜플은 변경하지 않았다(invariant 2·3 준수). `provenance`는 fingerprint
입력에 포함되지 않는다.

**WP-1 직후 `cargo test --workspace --no-fail-fast` 결과**: 283 passed / 0 failed
(baseline과 동일 — 각 크레이트별 수치도 baseline과 완전히 일치: 0,14,5,6,12,16,
10,19,8,9,32,11,3,3,3,8,4,23,0,61,0,36,0,0 = 283).

커밋: `67a923b feat(combat): add CombatProvenance to execution result`

## WP-2 — provenance 회귀 테스트

변경 파일: `crates/escape-core/tests/combat_execution_wave2.rs` (기존 6개 테스트
본문 무수정, 4개 신규 추가):
- `provenance_matches_input_manifest_version_tick_millis_and_fingerprint`
- `forecast_mode_reports_the_same_provenance_as_input`
- `deserializing_result_json_without_provenance_field_yields_none`
- `same_input_executed_twice_yields_identical_provenance`

`cargo test -p escape-core --test combat_execution_wave2` → 10 passed / 0 failed.
`cargo test --workspace --no-fail-fast` → **287 passed / 0 failed**.

커밋: `b5a6e2f test(combat): add provenance regression coverage for execute()`

## WP-3 — `spectate()`가 provenance를 쓰게 변경 (red 먼저 확인)

변경 파일:
- `crates/escape-core/tests/combat_spectator_wave3.rs` — 7곳의
  `CombatSpectatorRequest` 리터럴에서 `tick_millis: SIM_TICK_MILLIS,` 제거,
  `view_reports_the_requested_tick_millis` → `view_reports_the_tick_millis_from_provenance`
  로 이름·설명 갱신(assert 자체는 동일 값 검증), `zero_tick_millis_is_rejected`
  → `missing_provenance_is_rejected`로 교체(요청 필드 대신
  `resolution.execution.provenance = None`으로 유발).
- `crates/escape-core/src/combat_spectator.rs` — `CombatSpectatorRequest.tick_millis`
  필드 제거, `CombatSpectatorError::InvalidTickMillis(u32)` →
  `MissingProvenance`로 교체, `spectate()`가
  `resolution.execution.provenance.as_ref().map(|p| p.tick_millis).filter(|m| *m > 0)`
  로 tick_millis를 구하고 실패 시 `MissingProvenance`.

**red 확인 (WP-3 구현 전, 테스트만 먼저 수정한 상태에서 실행)**:
```
error[E0063]: missing field `tick_millis` in initializer of `escape_core::CombatSpectatorRequest`
   --> crates/escape-core/tests/combat_spectator_wave3.rs:140:5
error[E0063]: missing field `tick_millis` in initializer of `escape_core::CombatSpectatorRequest`
   --> crates/escape-core/tests/combat_spectator_wave3.rs:170:5
error[E0063]: missing field `tick_millis` in initializer of `escape_core::CombatSpectatorRequest`
   --> crates/escape-core/tests/combat_spectator_wave3.rs:242:5
error[E0063]: missing field `tick_millis` in initializer of `escape_core::CombatSpectatorRequest`
   --> crates/escape-core/tests/combat_spectator_wave3.rs:448:35
error[E0063]: missing field `tick_millis` in initializer of `escape_core::CombatSpectatorRequest`
   --> crates/escape-core/tests/combat_spectator_wave3.rs:454:35
error[E0599]: no variant, associated function, or constant named `MissingProvenance` found for enum `CombatSpectatorError` in the current scope
   --> crates/escape-core/tests/combat_spectator_wave3.rs:481:35
error[E0063]: missing field `tick_millis` in initializer of `escape_core::CombatSpectatorRequest`
   --> crates/escape-core/tests/combat_spectator_wave3.rs:544:5
error: could not compile `escape-core` (test "combat_spectator_wave3") due to 7 previous errors
```
(6x E0063 missing field, 1x E0599 unknown variant — 7 errors total.)

구현 후: `cargo test -p escape-core --test combat_spectator_wave3` →
19 passed / 0 failed (개수 유지). `cargo test --workspace --no-fail-fast` →
287 passed / 0 failed (변화 없음).

커밋: `f22e1e0 fix(combat): read spectate() tick_millis from provenance, drop duplicate param`

## WP-4 — `conclude()`가 provenance를 쓰게 변경 (red 먼저 확인)

변경 파일:
- `crates/escape-core/tests/combat_conclusion_wave2.rs` — 12곳의
  `CombatConclusionRequest` 리터럴에서 `tick_millis: 100/50,` 제거, 공유 fixture
  `resolution()`/`multi_resolution()`의 `provenance`를 `None`에서
  `Some(CombatProvenance { tick_millis: 100 또는 50, .. })`로 교체(제거된 요청
  필드가 갖던 값을 그대로 이전), `zero_tick_millis_is_rejected` →
  `missing_provenance_is_rejected`로 교체(`resolution.execution.provenance = None`).
- `crates/escape-core/src/combat_conclusion.rs` — `CombatConclusionRequest.tick_millis`
  필드 제거, `CombatConclusionError::InvalidTickMillis(u32)` →
  `MissingProvenance`로 교체, `conclude()`가 WP-3과 동일한 방식으로
  `resolution.execution.provenance`에서 `tick_millis`를 구해 `duration_millis`
  계산에 사용.

**red 확인 (WP-4 구현 전, 테스트만 먼저 수정한 상태에서 실행)**:
```
error[E0063]: missing field `tick_millis` in initializer of `escape_core::CombatConclusionRequest`
   --> crates/escape-core/tests/combat_conclusion_wave2.rs:138:21
error[E0063]: missing field `tick_millis` ... :178:17
error[E0063]: missing field `tick_millis` ... :192:15
error[E0063]: missing field `tick_millis` ... :211:17
error[E0063]: missing field `tick_millis` ... :251:17
error[E0063]: missing field `tick_millis` ... :272:23
error[E0063]: missing field `tick_millis` ... :291:17
error[E0599]: no variant, associated function, or constant named `MissingProvenance` found for enum `CombatConclusionError`
   --> crates/escape-core/tests/combat_conclusion_wave2.rs:305:36
error[E0063]: missing field `tick_millis` ... :359:19
error[E0063]: missing field `tick_millis` ... :435:19
error[E0063]: missing field `tick_millis` ... :472:19
error[E0063]: missing field `tick_millis` ... :522:19
error[E0063]: missing field `tick_millis` ... :584:5
error: could not compile `escape-core` (test "combat_conclusion_wave2") due to 13 previous errors
```
(12x E0063 missing field, 1x E0599 unknown variant — 13 errors total.)

구현 후: `cargo test -p escape-core --test combat_conclusion_wave2` →
14 passed / 0 failed (개수 유지). `cargo test --workspace --no-fail-fast` →
287 passed / 0 failed (변화 없음).

커밋: `91c2887 fix(combat): read conclude() tick_millis from provenance, drop duplicate param`

## WP-5 — 문서 갱신

변경 파일과 `wc -c`:
- `docs/design/Combat_System_Implementation_Plan_Index.md` (11794 bytes)
  - `status:` → `wave2-step6-complete`
  - 단계 표에 `fable_combat_wave2_step6_2608020838.md` 행 추가
  - "현재 코드와 정본의 경계" 문단에 Wave 2 Step 6 구현 내용과 테스트명 추가
  - **Step 1c 선결 과제(fingerprint 안정성)를 해소로 갱신**: 정본 03 인용
    ("같은 고정층 + 변경층 + seed + 개입 ID/tick + simulation version은 같은
    결과를 낸다. 이 결정성은 같은 version 내부에서만 보장한다 …")과 함께
    "fingerprint를 비교하는 consumer는 반드시 simulation_version도 함께
    비교해야 한다"를 계약으로 명시
  - Wave 2 Step 6 구현 위치 줄 추가(파일 목록, 테스트 수, 제거된 API 요약)
- `docs/dev/Combat_System_Operating_Guide.md` (16185 bytes) — "현재 구현 기준점"에
  Wave 2 Step 6 완료 기록 한 단락 추가
- `docs/dev/Combat_System_Goal_Prompt.md` (9297 bytes) — baseline 목록에 Wave 2
  Step 6 완료 기록 추가, `combat_execution_wave2.rs` 테스트 수 6→10 갱신(stale
  수치 정정), 권장 다음 goal 문장에서 fingerprint 안정성 선결 과제가 해소됐음을
  반영

모두 100KB 이하.

커밋: `8387402 docs(combat): record Wave 2 Step 6 provenance slice and resolve fingerprint precondition`

## 제거한 공개 API

- `CombatSpectatorRequest.tick_millis: u32` 필드
- `CombatConclusionRequest.tick_millis: u32` 필드
- `CombatSpectatorError::InvalidTickMillis(u32)` 변형 → `MissingProvenance`로 교체
- `CombatConclusionError::InvalidTickMillis(u32)` 변형 → `MissingProvenance`로 교체

## 최종 검증

```
cargo fmt --all -- --check          → 통과 (출력 없음)
cargo test -p escape-core --test combat_execution_wave2   → 10 passed / 0 failed
cargo test -p escape-core --test combat_resolution_wave2  → 16 passed / 0 failed
cargo test -p escape-core --test combat_conclusion_wave2  → 14 passed / 0 failed
cargo test -p escape-core --test combat_spectator_wave3   → 19 passed / 0 failed
cargo test --workspace --no-fail-fast → 287 passed / 0 failed
git diff --check                     → 통과 (exit 0)
```

크레이트별 최종 수치(순서대로): 0,14,5,10,12,16,10,19,8,9,32,11,3,3,3,8,4,23,0,
61,0,36,0,0 = **287 passed / 0 failed**.

기존 combat 테스트 90개(8파일)는 다음처럼 유지됐다: `combat_contract_wave1.rs`
5, `combat_state_wave1.rs` 8, `combat_opportunity_wave1.rs` 12,
`combat_simulation_wave2.rs` 10, `combat_execution_wave2.rs` 6→10(신규 4개
추가, 기존 6개 무수정), `combat_resolution_wave2.rs` 16, `combat_spectator_wave3.rs`
19(2개 이름·본문 갱신, 개수 유지), `combat_conclusion_wave2.rs` 14(1개 이름·본문
갱신, 개수 유지). 합계 90 → 94(신규 4개 추가분 포함), 개수가 준 파일은 없다.

## 스킵/이탈 항목

- 계획에 없던 조치: WP-1에서 `combat_conclusion_wave2.rs`의 두 `CombatExecutionResult`
  리터럴에 `provenance: None,`을 추가해야 했다(Rust 구조체 리터럴은 필드 생략을
  허용하지 않아, `#[serde(default)]`만으로는 컴파일이 되지 않음). 예상 변경 파일
  표 안에 있는 파일이고 논리 변경이 없는 기계적 조치라 WP-1 커밋에 포함했다.
  WP-4에서 같은 두 자리를 실제 값으로(`Some(..)`) 다시 바꿨다.
- 그 외 이탈 없음. `cli_smoke.rs`는 읽기만 했고 수정하지 않았다.
  `.claude/worktrees/`는 접근하지 않았다. fixture·generated JSON·`Cargo.toml`
  무변경.

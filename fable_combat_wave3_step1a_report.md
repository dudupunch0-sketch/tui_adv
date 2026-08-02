# Combat Wave 3 Step 1a — 구현 보고서

작성: 2026-08-02 (coding subagent)
플랜: `fable_combat_wave3_step1a_2608020020.md`
브랜치: `claude/combat-wave3-step1a`

## 요약

WP-1 ~ WP-8 전부 완료. 신규 모듈 `crates/escape-core/src/combat_spectator.rs` 1개와 신규 테스트 파일
`crates/escape-core/tests/combat_spectator_wave3.rs` (12개 테스트) 추가. `lib.rs`는 `mod`/`pub use` 추가만.
기존 combat 7모듈·기존 combat 테스트 7파일·`cli_smoke.rs`·`.claude/worktrees/`·fixture/generated JSON은
전부 무변경. `cargo test --workspace --no-fail-fast` 최종 **261 passed / 0 failed**
(baseline 249 + 신규 12, 기존 56개 combat 테스트 포함 baseline 193개는 파일 단위로 카운트까지 동일).

## WP별 변경 파일과 내용

### WP-1 — 타입 정의와 `lib.rs` 배선
- 변경: `crates/escape-core/src/combat_spectator.rs` (신규), `crates/escape-core/src/lib.rs` (+5줄, mod/pub use 추가)
- 내용: 플랜의 공개 API 계약 그대로 `CombatSpectatorCue/Piece/Frame/LogEntry/View/Request/Error` 타입 정의.
  `spectate()`는 `todo!()`가 아니라 컴파일되는 최소 구현(빈 `frames`/`core_log`/`full_log`, `resolution_fingerprint`만 채움)으로 둠.
- 커밋: `7c3d960`
- 검증: `cargo fmt --all -- --check` 통과, `cargo build -p escape-core` 경고 0건,
  `cargo test --workspace --no-fail-fast` → **249 passed / 0 failed** (baseline과 완전 동일, 신규 테스트 없음).

### WP-2 — 프레임 파생
- 변경: `combat_spectator.rs`, `combat_spectator_wave3.rs` (신규, 3 테스트)
- 내용: `resolution.execution.frames[i].positions`를 순회하며 `participants`에서 `side`/`facing`/`active`를 조회.
  조회 실패 시 `CombatSpectatorError::UnknownParticipant(id)`. `BTreeMap<String, CombatPosition>` 순회이므로
  id 오름차순이 자동 보장됨 (HashMap 미사용).
- 신규 테스트: `frame_positions_facing_side_and_active_match_input`, `unknown_participant_is_rejected`,
  `participant_input_order_does_not_affect_view`
- 커밋: `a74bf7b`
- 검증: `cargo test -p escape-core --test combat_spectator_wave3` → 3 passed.
  `cargo test --workspace --no-fail-fast` → **252 passed / 0 failed**.

### WP-3 — cue 파생
- 변경: `combat_spectator.rs` (`cues_for` 함수 추가), `combat_spectator_wave3.rs` (+2 테스트)
- 내용: 같은 tick의 `resolution.frames[i].outcomes`만 보고 Attack(actor_id 일치)/Hit(target && hit && damage>0)/
  Evade(target && in_range && !hit) 3규칙만 적용. `BTreeSet<CombatSpectatorCue>`로 모아 dedup + enum 선언순
  (Attack < Hit < Evade) 정렬을 자동 보장.
- 신규 테스트: `attack_hit_and_evade_cues_follow_the_three_rules_only` (2-way 공격으로 세 cue를
  한 프레임에서 동시 검증), `cues_are_sorted_attack_then_hit_then_evade_with_no_duplicates`
- 커밋: `2578824`
- 검증: 5 passed (spectator 파일). 전체 **254 passed / 0 failed**.

### WP-4 — 이중 로그와 템플릿 id
- 변경: `combat_spectator.rs` (`build_log` 함수, 템플릿 상수 추가), `combat_spectator_wave3.rs` (+3 테스트)
- 내용: `execution.full_log`(`CombatLogEvent`)와 `resolution.full_log`(`CombatResolutionLogEvent`)를
  `(tick, sequence, source_rank)` 튜플로 병합 정렬(`source_rank`: 실행로그=0, 판정로그=1 → 동률 시 실행로그 우선).
  태그 → `template_id` 표를 적용. **주의**: 이 WP 시점에는 아직 누설 차단(WP-5)이 없어 `AttackRoll`/
  `EffectSuppressed`도 그대로 통과시키고 `effect_id`도 마스킹 없이 통과시키는 순진한 구현이었음(의도적, TDD 준비 단계).
- 신규 테스트: `log_entries_use_registered_template_ids_not_free_sentences`,
  `full_log_is_ordered_by_tick_then_sequence`,
  `core_log_is_a_subset_of_full_log_filtered_by_importance_and_keeps_order`
- 커밋: `c1b3d55`
- 검증: 8 passed (spectator 파일). 전체 **257 passed / 0 failed**.

### WP-5 — 누설 차단 (테스트 먼저, red 확인 후 구현)
- 변경: `combat_spectator.rs` (`build_log` 수정: AttackRoll/EffectSuppressed 제외, EffectApplied visibility
  마스킹), `combat_spectator_wave3.rs` (+2 테스트 및 `leak_resolution_request`/`leak_spectator_request`/
  `effect_def` 픽스처)
- **RED 확인 절차**: 테스트 2개(`attack_roll_and_effect_suppressed_never_leak_into_any_log`,
  `hidden_conditional_and_unregistered_effect_ids_are_masked`)를 WP-4의 구현(위 "순진한" 버전) 그대로 둔
  채 먼저 추가하고 실행 → 아래 실패를 실제로 관찰함.

  ```text
  running 10 tests
  test core_log_is_a_subset_of_full_log_filtered_by_importance_and_keeps_order ... ok
  test attack_hit_and_evade_cues_follow_the_three_rules_only ... ok
  test cues_are_sorted_attack_then_hit_then_evade_with_no_duplicates ... ok
  test frame_positions_facing_side_and_active_match_input ... ok
  test full_log_is_ordered_by_tick_then_sequence ... ok
  test attack_roll_and_effect_suppressed_never_leak_into_any_log ... FAILED
  test hidden_conditional_and_unregistered_effect_ids_are_masked ... FAILED
  test log_entries_use_registered_template_ids_not_free_sentences ... ok
  test unknown_participant_is_rejected ... ok
  test participant_input_order_does_not_affect_view ... ok

  failures:

  ---- attack_roll_and_effect_suppressed_never_leak_into_any_log stdout ----

  thread 'attack_roll_and_effect_suppressed_never_leak_into_any_log' (31435) panicked at crates/escape-core/tests/combat_spectator_wave3.rs:387:9:
  assertion `left != right` failed
    left: "combat.log.attack_roll"
   right: "combat.log.attack_roll"
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

  ---- hidden_conditional_and_unregistered_effect_ids_are_masked stdout ----

  thread 'hidden_conditional_and_unregistered_effect_ids_are_masked' (31440) panicked at crates/escape-core/tests/combat_spectator_wave3.rs:416:5:
  assertion `left == right` failed: buff_hidden, buff_conditional, and buff_unregistered must all be masked
    left: 0
   right: 3

  failures:
      attack_roll_and_effect_suppressed_never_leak_into_any_log
      hidden_conditional_and_unregistered_effect_ids_are_masked

  test result: FAILED. 8 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
  ```

- 구현: `AttackRoll`/`EffectSuppressed` 이벤트를 병합/정렬 전에 `continue`로 제외. `EffectApplied`는
  `request.catalog`(spectator 자체 catalog, 판정에 쓰인 catalog와 별개 필드)에서 `effect_id`를 조회해
  `EffectVisibility::Public`이 아니거나(=Hidden/Conditional) catalog에 아예 없으면 `effect_id`를 `None`으로
  마스킹하고 `template_id`를 `combat.log.effect_applied_hidden`으로 교체.
- GREEN 확인:

  ```text
  running 10 tests
  test attack_hit_and_evade_cues_follow_the_three_rules_only ... ok
  test attack_roll_and_effect_suppressed_never_leak_into_any_log ... ok
  test core_log_is_a_subset_of_full_log_filtered_by_importance_and_keeps_order ... ok
  test cues_are_sorted_attack_then_hit_then_evade_with_no_duplicates ... ok
  test frame_positions_facing_side_and_active_match_input ... ok
  test full_log_is_ordered_by_tick_then_sequence ... ok
  test unknown_participant_is_rejected ... ok
  test log_entries_use_registered_template_ids_not_free_sentences ... ok
  test hidden_conditional_and_unregistered_effect_ids_are_masked ... ok
  test participant_input_order_does_not_affect_view ... ok

  test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
  ```

- 커밋: `9036aa6`
- 검증: `cargo test --workspace --no-fail-fast` → **259 passed / 0 failed**.

### WP-6 — fingerprint
- 변경: `combat_spectator_wave3.rs` (+2 테스트: `spectate_is_deterministic_for_identical_input`,
  `fingerprint_chains_the_resolution_fingerprint`). `combat_spectator.rs`의 `fingerprint()`/`fnv()`는
  WP-1부터 이미 기존 모듈과 같은 방식(canonical JSON → FNV-1a → `{:016x}`)으로 구현되어 있었고,
  `CombatSpectatorView.fingerprint`는 `resolution_fingerprint` 필드를 포함한 struct 전체를 직렬화해
  해시하므로 `resolution_fingerprint`가 이미 체인에 포함되어 있었음. 이 WP에서는 새 코드 없이 그 사실을
  테스트로 고정.
- 커밋: `d900a59`
- 검증: 12 passed (spectator 파일). 전체 **261 passed / 0 failed**.

### WP-7 — 회귀 테스트
`combat_spectator_wave3.rs`는 WP-2~WP-6에 걸쳐 누적 구축되어 최종 12개 테스트로 플랜의 9개 필수
시나리오(최소 8케이스)를 전부 포함한다. 별도 코드 변경이 필요 없어 새 커밋 없이 검증만 수행:

| # | 플랜 항목 | 테스트 함수 |
|---|---|---|
| 1 | 결정론(동일 입력 → 동일 fingerprint/view) | `spectate_is_deterministic_for_identical_input` |
| 2 | 프레임 좌표·facing·side·active 일치 | `frame_positions_facing_side_and_active_match_input` |
| 3 | 미등록 participant → `UnknownParticipant` | `unknown_participant_is_rejected` |
| 4 | Attack/Hit/Evade 규칙대로만 cue 생성 | `attack_hit_and_evade_cues_follow_the_three_rules_only` |
| 5 | cue 정렬·중복 제거 고정 | `cues_are_sorted_attack_then_hit_then_evade_with_no_duplicates` |
| 6 | `core_log ⊆ full_log`, importance>=Important, 순서 유지 | `core_log_is_a_subset_of_full_log_filtered_by_importance_and_keeps_order` |
| 7 | AttackRoll·EffectSuppressed 어느 로그에도 없음 | `attack_roll_and_effect_suppressed_never_leak_into_any_log` |
| 8 | Hidden/Conditional/미등록 효과 id 마스킹 | `hidden_conditional_and_unregistered_effect_ids_are_masked` |
| 9 | participants 입력 순서 무관 | `participant_input_order_does_not_affect_view` |

부가 테스트(플랜 범위 안, 세부 보강): `log_entries_use_registered_template_ids_not_free_sentences`,
`full_log_is_ordered_by_tick_then_sequence`, `fingerprint_chains_the_resolution_fingerprint`.

- 검증: `cargo test -p escape-core --test combat_spectator_wave3` → **12 passed / 0 failed**.

### WP-8 — 문서 갱신
- 변경:
  - `docs/design/Combat_System_Implementation_Plan_Index.md`: `status: wave3-step1a-complete`로 변경,
    Wave 3 Step 1 행을 1a(완료)/1b/1c/1d(플랜 미작성)로 분할, 존재하지 않는
    `fable_combat_wave3_step1_2607261845.md`/`fable_combat_wave3_step2_2607261845.md` 참조 제거(Step 2 행도
    `(플랜 미작성)`으로 교체), "현재 코드와 정본의 경계"에 관전 adapter 확보분과 미확보 항목(per-tick 상태
    스냅샷, 보고서 확장, ScenePage/WASM/renderer, 프리셋, 치유·명줄) 기록.
  - `docs/dev/Combat_System_Operating_Guide.md`: 섹션 3 "현재 구현 기준점"에 Step 1a 완료 항목과 구현/테스트
    파일, 12개 테스트 함수명 나열, "아직 열지 않음" 항목 갱신.
  - `docs/dev/Combat_System_Goal_Prompt.md`: "현재 baseline"에 Step 1a 완료 한 줄 추가, 권장 다음 goal 문장을
    Step 1b로 갱신(Step 1이 이미 1a로 분할됐으므로).
  - `docs/dev/Development_Plan.md`: 10번 항목에 Wave 3 Step 1a 완료·구현 파일·테스트 함수명 반영,
    잔여 항목(1b/1c/1d, Step 2, 밸런스)을 명시.
- 문서 크기 (`wc -c`, 100KB=102400 이하 확인):
  - `Combat_System_Implementation_Plan_Index.md`: 5810 bytes
  - `Combat_System_Operating_Guide.md`: 11143 bytes
  - `Combat_System_Goal_Prompt.md`: 6888 bytes
  - `Development_Plan.md`: 47297 bytes
- 커밋: `faa30e9`

## 최종 검증

```text
$ cargo fmt --all -- --check
(출력 없음, 통과)

$ cargo test -p escape-core --test combat_spectator_wave3
running 12 tests
... (전부 ok)
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo test -p escape-core --test combat_resolution_wave2
running 11 tests
... (전부 ok, 기존 테스트 무변경 확인)
test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

$ cargo test -p escape-core --test combat_conclusion_wave2
running 4 tests
... (전부 ok, 기존 테스트 무변경 확인)
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo test --workspace --no-fail-fast
... 261 passed; 0 failed (전체 24개 테스트 바이너리 합산)

$ git diff --check
(출력 없음, 통과)
```

### 249 → 261 증가분 근거

baseline과 최종 실행의 `test result:` 라인을 바이너리별로 비교하면, 새로 추가된 `combat_spectator_wave3`
바이너리(12 passed) 1개를 제외한 나머지 23개 바이너리는 **정확히 동일한 통과 개수**를 유지한다(0, 4, 5, 6,
12, 11, 10, 8, 9, 32, 11, 3, 3, 3, 8, 4, 23, 0, 61, 0, 36, 0, 0 — baseline과 100% 일치). 즉 249 + 12 = 261이며,
기존 combat 56개 테스트(`combat_contract_wave1` 5, `combat_state_wave1` 8, `combat_opportunity_wave1` 12,
`combat_simulation_wave2` 10, `combat_execution_wave2` 6, `combat_resolution_wave2` 11,
`combat_conclusion_wave2` 4 = 56)는 값 하나도 바뀌지 않았다.

## 기존 combat 7모듈·7테스트 파일 무변경 근거

```text
$ git diff --stat f9035d2..HEAD -- crates/escape-core/src/combat_contract.rs \
    crates/escape-core/src/combat_state.rs crates/escape-core/src/combat_opportunity.rs \
    crates/escape-core/src/combat_simulation.rs crates/escape-core/src/combat_execution.rs \
    crates/escape-core/src/combat_resolution.rs crates/escape-core/src/combat_conclusion.rs \
    'crates/escape-core/tests/combat_*.rs' crates/escape-core/fixtures web/src/data/generated \
    crates/escape-terminal/tests/cli_smoke.rs

 crates/escape-core/tests/combat_spectator_wave3.rs | 459 +++++++++++++++++++++
 1 file changed, 459 insertions(+)
```

위 diff에는 신규 추가 파일 `combat_spectator_wave3.rs` 한 줄만 나타난다. 기존 7모듈, 기존 combat 테스트
7파일, fixture/generated JSON, `cli_smoke.rs`는 `f9035d2`(baseline) 대비 바이트 하나 바뀌지 않았다.

`grep -n 'HashMap\|derive_seed\|roll(' crates/escape-core/src/combat_spectator.rs` → 매치 없음
(RNG 재호출·판정 재계산·HashMap 순회 전부 0회).

## BalanceBroken / Incapacitated cue 제외 사유 (플랜 근거 인용)

플랜의 "명시적 범위 밖" 섹션을 그대로 따랐다:

> `BalanceBroken` / `Incapacitated` cue — **의도적 제외.** `CombatResolutionFrame`에 tick별 전투원 상태
> 스냅샷이 없어(`CombatResolutionState`는 최종 상태 1개) tick 단위로 균형 0 / 생명력 0 시점을 알 수 없다.
> adapter에서 델타를 재누적하면 `combat_resolution.rs`의 clamp 로직과 갈라질 수 있으므로, **`CombatResolutionFrame`에
> per-tick 스냅샷을 추가하는 Wave 2 후속 slice**를 먼저 열고 그 뒤에 이 두 cue를 추가한다.

실제로 `combat_resolution.rs`를 확인한 결과 `CombatResolutionResult.state: CombatResolutionState`는 결착 후
최종 `current_health_hundredths`/`balance_hundredths` 1세트만 담고 있고, `CombatResolutionFrame`(tick별)에는
`outcomes`(공격 결과)만 있을 뿐 그 시점의 잔여 체력/균형 스냅샷이 없다. adapter 레벨에서 tick마다
`balance_delta_hundredths`를 누적해 재구성하는 것은 "판정 재계산 금지"(hard invariant #2)를 사실상
위반하는 것과 다름없고, `combat_resolution.rs`의 clamp(0, max) 로직과 독립적으로 계산되므로 결과가 어긋날 위험이
있다. 따라서 이번 slice에서는 구현하지 않았다.

## tick_millis 관련 플랜 공백 (이탈 항목, 사유 명시)

플랜의 공개 API 계약에는 `CombatSpectatorView.tick_millis: u32` 필드가 있지만, "파생 규칙" 섹션에는
이 필드의 파생 규칙이 명시돼 있지 않다. 실제 코드를 확인한 결과:

- `CombatSimulationConfig.tick_millis`는 `CombatSimulationInput`(즉 **요청** 쪽)에만 있다.
- `CombatExecutionResult`/`CombatResolutionResult`(즉 `CombatSpectatorRequest.resolution`이 담고 있는 **결과**
  쪽)에는 `tick_millis`를 보존하는 필드가 전혀 없다.

즉 `CombatSpectatorRequest { resolution, participants, catalog }` 3개 필드만으로는 `tick_millis`를 유도할
방법이 없다(재계산 금지 원칙 때문에 임의로 새 seed/설정을 다시 읽어올 수도 없음). 플랜의 API 시그니처를
그대로 유지하기 위해 `CombatSpectatorRequest`에 필드를 추가하는 이탈은 하지 않았고, 대신 `tick_millis: 0`
고정값을 두고 소스 코드에 그 사유를 주석으로 남겼다(`combat_spectator.rs` WP-1 커밋 참고). 렌더러가 이 값을
실제로 소비하는 시점은 Step 1c/1d(ScenePage/WASM/renderer, 이 slice의 범위 밖)이므로 지금 당장 기능적으로
막히는 부분은 없지만, Step 1c 착수 전에 `tick_millis`를 어디서 가져올지(요청에 필드 추가 vs
`CombatExecutionResult`에 필드 추가) 결정이 필요하다.

## 스킵/이탈 항목 정리

1. **`tick_millis` 파생 불가** — 위 항목 참고. 플랜 API 그대로 유지, 값은 0 고정, 사유는 소스 주석과 이 보고서에 기록.
2. **`BalanceBroken`/`Incapacitated` cue 미구현** — 플랜이 명시적으로 요구한 제외 사항. Wave 2 후속(per-tick
   상태 스냅샷) slice 선행 필요.
3. **WP-7 별도 커밋 없음** — 테스트 파일이 WP-2~WP-6에 걸쳐 점진적으로 구축되어 WP-7 시점에는 이미 9개
   필수 시나리오(12개 테스트)를 전부 만족했다. 별도 코드 변경이 없어 새 커밋을 만들지 않고 검증만 수행했다.
4. 위험해 보여 스킵한 WP는 없음. WP-1~WP-8 전부 수행 완료.

## 최종 `git` 상태

```text
$ git status --short -uall
 M crates/escape-terminal/tests/cli_smoke.rs
?? .claude/worktrees/caveman-repo-sync-8a6b94/
?? fable_combat_wave3_step1a_2608020020.md

$ git diff --stat HEAD~7..HEAD
 crates/escape-core/src/combat_spectator.rs         | 280 +++++++++++++
 crates/escape-core/src/lib.rs                      |   5 +
 crates/escape-core/tests/combat_spectator_wave3.rs | 459 +++++++++++++++++++++
 docs/design/Combat_System_Implementation_Plan_Index.md | 22 +-
 docs/dev/Combat_System_Goal_Prompt.md              |   5 +-
 docs/dev/Combat_System_Operating_Guide.md          |   7 +-
 docs/dev/Development_Plan.md                       |   2 +-
 7 files changed, 770 insertions(+), 10 deletions(-)

$ git log --oneline -7
faa30e9 docs(combat): record Wave 3 Step 1a completion and split Step 1 (WP-8)
d900a59 test(combat): confirm spectator fingerprint chaining (WP-6)
9036aa6 fix(combat): mask hidden judgement and effect leaks in spectator log (WP-5)
c1b3d55 feat(combat): merge dual logs into template-id entries (WP-4)
2578824 feat(combat): derive Attack/Hit/Evade cues from outcomes (WP-3)
a74bf7b feat(combat): derive spectator frames from resolution positions (WP-2)
7c3d960 feat(combat): add spectator view types and lib wiring (WP-1)
```

`cli_smoke.rs`의 `M` 표시는 다른 작업자가 이 세션 시작 이전부터 남겨 둔 uncommitted 변경이며, 이번 7개
커밋 중 어느 것에도 포함되지 않았다(`git diff --stat HEAD~7..HEAD`에는 나타나지 않고, 단일 인자
`git diff HEAD~7`에서만 working tree diff로 섞여 보인다는 점을 확인함). `.claude/worktrees/`는 읽지도
쓰지도 않았다.

---

## 오케스트레이터 리뷰 수정 (2026-08-02, Fable)

subagent 보고를 WSL에서 직접 재검증한 뒤 결함 1건을 고쳤다.

### 검증한 것 (전부 직접 실행)

- `cargo fmt --all -- --check` 통과
- `git diff --check` 통과
- `git diff --name-only HEAD~7..HEAD`에 기존 combat 7모듈·기존 테스트 7파일 **없음** 확인
- `grep -nE "derive_seed|rand|roll|Rng|thread_rng|HashMap" crates/escape-core/src/combat_spectator.rs` → 매치 0건. 판정 재계산·RNG 호출·해시 순회 의존이 없다.
- `crates/escape-terminal/tests/cli_smoke.rs` mtime `2026-07-26 20:08` — 이 세션 미변경
- **누설 차단 독립 red 재현**: `git checkout 9036aa6~1 -- crates/escape-core/src/combat_spectator.rs`로 WP-5 소스만 되돌린 뒤 실행 → `attack_roll_and_effect_suppressed_never_leak_into_any_log`와 `hidden_conditional_and_unregistered_effect_ids_are_masked` 2건 FAIL (`left: 0, right: 3`). 복원 후 green. 즉 테스트가 실제로 결함을 잡는다.
- 유령 플랜 파일 참조(`fable_combat_wave3_step1_2607261845.md`, `fable_combat_wave3_step2_2607261845.md`) 제거 확인

### 고친 결함: `tick_millis`가 공개 API에서 거짓값

subagent가 `CombatSpectatorView.tick_millis`를 `0` 고정으로 두고 "유도 불가"라고 보고했다. 유도 불가 진단 자체는 맞다 — `CombatResolutionResult`도 `CombatExecutionResult`도 입력 `CombatSimulationConfig`를 보관하지 않는다. 그러나 `0`을 내보내는 것은 거짓값이다. 테스트 입력은 `tick_millis: 100`이었는데 view는 `0`을 보고했고, **아무 테스트도 그 값을 검증하지 않았다.** 렌더러가 이 값을 믿으면 정본 13의 "시뮬레이션 시간과 화면 시간은 항상 일치한다"가 깨진다.

수정 내용:

- `CombatSpectatorRequest`에 `tick_millis: u32`를 추가했다. 호출자가 시뮬레이션에 쓴 값을 그대로 전달한다.
- `CombatSpectatorError::InvalidTickMillis(u32)`를 추가해 `0`을 거부한다.
- `spectate()`가 request 값을 view로 옮긴다. 고정 상수와 그 사유 주석을 제거했다.
- 테스트 2건 추가: `view_reports_the_requested_tick_millis`, `zero_tick_millis_is_rejected`.
- 테스트 파일의 `tick_millis: 100` 리터럴을 `SIM_TICK_MILLIS` 상수로 묶어, 시뮬레이션 config와 관전 request가 항상 같은 값을 쓰도록 고정했다.

### 수정 후 최종 검증

- `cargo fmt --all -- --check` 통과
- `cargo test -p escape-core --test combat_spectator_wave3` → **14 passed / 0 failed**
- `cargo test --workspace --no-fail-fast` → **263 passed / 0 failed**, exit 0 (249 baseline + 신규 14)
- `git diff --check` 통과

### 작업 노하우 기록

이 slice에서 셸 heredoc에 backtick이 들어간 마크다운을 넣다가 내용이 명령 치환으로 유실되는 사고가 두 번 났다 (PR #178 본문, 이 보고서 append). **backtick·마크다운이 포함된 파일 내용은 셸 heredoc으로 쓰지 말고 파일 쓰기 도구로 직접 쓰고, `gh pr edit --body-file` / `git commit -F <file>`로 넘긴다.**

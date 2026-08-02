# Combat Wave 3 Step 1b — 구현 보고서

작성: 2026-08-02
구현 담당: coding subagent (sonnet, effort medium)
플랜: `fable_combat_wave3_step1b_2608020437.md`

## 결과 요약

WP-1 ~ WP-6 전부 구현·검증·커밋 완료. 기존 combat 테스트 80개 개수 그대로 통과, 신규 10개 추가로 `combat_conclusion_wave2.rs`는 4 → 14개. `cargo test --workspace --no-fail-fast` 최종 **283 passed / 0 failed** (baseline 273 + 10 신규).

## WP별 변경 파일과 내용

### WP-1 — `tick_millis` 입력과 `duration_millis`

파일: `crates/escape-core/src/combat_conclusion.rs`, `crates/escape-core/tests/combat_conclusion_wave2.rs`

- `CombatConclusionRequest.tick_millis: u32` 추가. `0`이면 `CombatConclusionError::InvalidTickMillis(0)` 반환 (policy 검사보다 먼저 체크).
- `CombatConclusionReport.duration_millis: u64` 추가 (`#[serde(default)]`). `decisive_tick`이 `Some(t)`면 `(t+1) * tick_millis`, `None`이면 `frames.len() * tick_millis`.
- 기존 4개 테스트(`all_outcomes_and_mutual_precedence_are_stable`, `permutation_and_invalid_inputs_are_deterministic`, `cleanup_is_split_without_persistent_promotion`, `policy_and_active_side_validation_are_explicit`)의 `CombatConclusionRequest` 리터럴 6곳에 `tick_millis: 100` (또는 `eval` 헬퍼 경유) 필드만 추가, 본문 로직 무수정.
- 신규 테스트: `zero_tick_millis_is_rejected`, `duration_millis_uses_decisive_tick_plus_one_when_terminal`, `duration_millis_uses_frame_count_when_not_terminal`.

**red 원문** (`cargo test -p escape-core --test combat_conclusion_wave2`, 테스트 추가 직후·구현 전):

```
error[E0560]: struct `escape_core::CombatConclusionRequest` has no field named `tick_millis`
   --> crates/escape-core/tests/combat_conclusion_wave2.rs:192:9
    ...
error[E0599]: no variant, associated function, or constant named `InvalidTickMillis` found for enum `CombatConclusionError` in the current scope
   --> crates/escape-core/tests/combat_conclusion_wave2.rs:238:36
    ...
error[E0609]: no field `duration_millis` on type `CombatConclusionReport`
   --> crates/escape-core/tests/combat_conclusion_wave2.rs:247:23
    ...
error: could not compile `escape-core` (test "combat_conclusion_wave2") due to 10 previous errors
```

구현 후: `cargo test -p escape-core --test combat_conclusion_wave2` → **7 passed; 0 failed**.
`cargo fmt --all -- --check` → 최초 diff 있었음(함수 인자 줄바꿈), `cargo fmt --all` 적용 후 재확인 통과.
`cargo test --workspace --no-fail-fast` → **276 passed; 0 failed**.

커밋: `fd80f51 feat(combat): add tick_millis input and duration_millis to conclusion report`

### WP-2 — 캐릭터별 집계

파일: 동일 2개

- `CombatCombatantReport { id, damage_dealt_hundredths, damage_taken_hundredths, kills, incapacitated }` struct 추가.
- `CombatConclusionReport.combatants: Vec<CombatCombatantReport>` 추가 (`#[serde(default)]`, id 오름차순 — `participants` `BTreeMap`의 key 순회를 그대로 사용).
- `damage_dealt`/`damage_taken`은 `request.resolution.frames[].outcomes`에서 `hit && damage_hundredths > 0`인 것만 actor/target 기준 합산.
- `incapacitated`는 마지막 frame의 `combatants` 스냅샷(비어 있으면 `request.resolution.state.combatants`로 대체) 기준 `current_health_hundredths <= 0`.
- `kills`는 이 WP에서는 항상 `0` (WP-3에서 구현).
- 신규 테스트: `combatants_report_sums_damage_and_marks_incapacitated` (3인 시나리오, id 정렬·피해 합산·미전투불능 확인, `kills == 0` 확인 — 이 시나리오는 아무도 전투불능이 되지 않으므로 WP-3 구현 후에도 값이 그대로 유지되어 회귀 걱정 없음).

**red 원문**:

```
error[E0609]: no field `combatants` on type `CombatConclusionReport`
   --> crates/escape-core/tests/combat_conclusion_wave2.rs:371:14
error[E0609]: no field `combatants` on type `CombatConclusionReport`
   --> crates/escape-core/tests/combat_conclusion_wave2.rs:377:40
error: could not compile `escape-core` (test "combat_conclusion_wave2") due to 2 previous errors
```

구현 후: `cargo test -p escape-core --test combat_conclusion_wave2` → **8 passed; 0 failed**.
`cargo fmt --all -- --check` → diff 있었음(`let by_id` 줄바꿈), fmt 적용 후 통과.
`cargo test --workspace --no-fail-fast` → **277 passed; 0 failed**.

커밋: `7fcf86b feat(combat): add per-combatant damage and incapacitated aggregation`

### WP-3 — 처치 수

파일: 동일 2개

- 각 대상 id별로 `frames[i].combatants`에서 `current_health_hundredths <= 0`이 **처음** 관측되는 frame index를 찾고(`first_defeated_at`), 그 frame의 `outcomes`를 뒤에서부터 순회해 `target_id == 대상 && hit && damage_hundredths > 0`인 **첫 번째로 만나는(=원본 순서상 마지막)** outcome의 `actor_id`에게 1을 부여.
- 매칭되는 outcome이 없으면 아무에게도 부여하지 않음(빈 스냅샷·이미 전투불능 상태로 시작한 경우 포함).
- 신규 테스트: `kills_are_attributed_to_last_valid_lethal_outcome_in_the_ko_tick` — 4인 시나리오. "d"는 첫 frame부터 이미 0(그 frame에 d를 대상으로 한 유효 outcome 없음 → kill 없음), "e"는 tick 1에서 두 번 맞아 사망(마지막 outcome의 actor "a"만 kill을 받고, 같은 tick에 먼저 때린 "c"는 받지 않음을 검증).

**red 원문**:

```
running 1 test
test kills_are_attributed_to_last_valid_lethal_outcome_in_the_ko_tick ... FAILED

thread 'kills_are_attributed_to_last_valid_lethal_outcome_in_the_ko_tick' panicked at crates/escape-core/tests/combat_conclusion_wave2.rs:452:5:
assertion `left == right` failed
  left: 0
 right: 1
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s
```

구현 후: `cargo test -p escape-core --test combat_conclusion_wave2` → **9 passed; 0 failed**.
`cargo fmt --all -- --check` → 통과(diff 없음).
`cargo test --workspace --no-fail-fast` → **278 passed; 0 failed**.

커밋: `6297f68 feat(combat): attribute kills to the last lethal outcome per KO tick`

### WP-4 — 하이라이트 2개

파일: 동일 2개

- `top_id_by(combatants, value_fn)` 헬퍼: `combatants`가 이미 id 오름차순이므로 엄격히 `>` 비교로 최댓값을 찾으면 동점 시 자동으로 최소 id가 유지된다. 최댓값이 `0` 이하면 `None`.
- `CombatConclusionReport.top_damage_dealt_id`/`top_damage_taken_id: Option<String>` 추가 (`#[serde(default)]`).
- 신규 테스트: `top_damage_highlights_hidden_when_no_damage_occurs` (피해 0 → 둘 다 `None`), `top_damage_highlights_pick_max_with_lowest_id_tie_break` (`a`/`e` damage_dealt 동점 25 → `Some("a")`, damage_taken 고유 최댓값 `e` → `Some("e")`).

**red 원문**:

```
error[E0609]: no field `top_damage_dealt_id` on type `CombatConclusionReport`
   --> crates/escape-core/tests/combat_conclusion_wave2.rs:483:23
error[E0609]: no field `top_damage_taken_id` on type `CombatConclusionReport`
   --> crates/escape-core/tests/combat_conclusion_wave2.rs:484:23
error[E0609]: no field `top_damage_dealt_id` on type `CombatConclusionReport`
   --> crates/escape-core/tests/combat_conclusion_wave2.rs:536:23
error[E0609]: no field `top_damage_taken_id` on type `CombatConclusionReport`
   --> crates/escape-core/tests/combat_conclusion_wave2.rs:538:23
error: could not compile `escape-core` (test "combat_conclusion_wave2") due to 4 previous errors
```

구현 후: `cargo test -p escape-core --test combat_conclusion_wave2` → **11 passed; 0 failed**.
`cargo fmt --all -- --check` → 통과.
`cargo test --workspace --no-fail-fast` → **280 passed; 0 failed**.

커밋: `eb61870 feat(combat): add top damage dealt/taken highlights to conclusion report`

### WP-5 — 회귀 테스트

파일: `crates/escape-core/tests/combat_conclusion_wave2.rs`만.

플랜의 11개 회귀 케이스는 WP-1~4에서 이미 대부분 커버되었으므로, WP-5에서는 아직 없던 3개(결정론 재현, participants 순서 무관, 구버전 JSON 역직렬화)만 신규 추가했다. 이 WP는 새 프로덕션 로직이 없으므로 **red 확인 없이 즉시 green**(기존 구현이 이미 요구사항을 만족함을 확인하는 성격의 회귀 테스트).

- `same_input_conclude_twice_yields_identical_report_and_fingerprint` (항목 9)
- `shuffled_participant_order_yields_identical_report` (항목 10)
- `deserializing_report_json_without_new_fields_uses_defaults` (항목 11, 구버전 필드만 있는 JSON을 `serde_json::from_str`로 역직렬화 → 에러 없이 `duration_millis == 0`, `combatants` 비어 있음, 하이라이트 둘 다 `None`)

11개 항목 커버리지:

| # | 항목 | 테스트 |
|---|---|---|
| 1 | tick_millis 0 거부 | `zero_tick_millis_is_rejected` (WP-1) |
| 2 | decisive_tick 있을 때 duration | `duration_millis_uses_decisive_tick_plus_one_when_terminal` (WP-1) |
| 3 | decisive_tick 없을 때 duration | `duration_millis_uses_frame_count_when_not_terminal` (WP-1) |
| 4 | combatants가 모든 참가자 id 오름차순 포함 | `combatants_report_sums_damage_and_marks_incapacitated` (WP-2) |
| 5 | 피해 합산 일치 | 위와 동일 |
| 6 | 처치 수 귀속 | `kills_are_attributed_to_last_valid_lethal_outcome_in_the_ko_tick` (WP-3) |
| 7 | 피해 0 → 하이라이트 None | `top_damage_highlights_hidden_when_no_damage_occurs` (WP-4) |
| 8 | 동점 tie-break | `top_damage_highlights_pick_max_with_lowest_id_tie_break` (WP-4) |
| 9 | 동일 입력 → 동일 보고서/fingerprint | `same_input_conclude_twice_yields_identical_report_and_fingerprint` (WP-5) |
| 10 | participants 순서 무관 | `shuffled_participant_order_yields_identical_report` (WP-5) |
| 11 | 구버전 JSON 역직렬화 | `deserializing_report_json_without_new_fields_uses_defaults` (WP-5) |

`cargo test -p escape-core --test combat_conclusion_wave2` → **14 passed; 0 failed**.
`cargo fmt --all -- --check` → 통과(diff 없음).
`cargo test --workspace --no-fail-fast` → **283 passed; 0 failed**.

커밋: `71976e2 test(combat): add regression coverage for conclusion report aggregation`

### 부가 커밋 — `lib.rs` re-export

플랜의 "공개 API 변경" 절이 명시한 `lib.rs`의 `combat_conclusion` re-export 목록에 `CombatCombatantReport`를 추가하는 작업을 WP-2 때 누락했음을 뒤늦게 발견해 별도 소형 커밋으로 보완했다(WP 번호에 속하지 않는 순수 공개 API 완결 작업이라 새 WP를 만들지 않고 독립 커밋으로 분리).

커밋: `087424f feat(combat): re-export CombatCombatantReport from escape-core`

### WP-6 — 문서 갱신

파일: `docs/design/Combat_System_Implementation_Plan_Index.md`, `docs/dev/Combat_System_Operating_Guide.md`, `docs/dev/Combat_System_Goal_Prompt.md`

- Plan Index: `status` → `wave3-step1b-complete`. "현재 코드와 정본의 경계" 문단에 Wave 3 Step 1b 확보분을 테스트명과 함께 서술. 단계 표의 `(플랜 미작성) — Wave 3 Step 1b` 행을 `fable_combat_wave3_step1b_2608020437.md`로 교체. 치유량/명줄 두 줄과 "정본 13 금지 항목 의도적 미구현" 한 줄을 명시.
- Operating Guide: "완료" 목록에 Wave 3 Step 1b 항목과 10개 신규 테스트명 추가, "아직 열지 않음" 목록에서 "전투 종료 보고서 확장" 제거 후 치유량/명줄 두 줄로 대체.
- Goal Prompt: baseline 절 결착 테스트 수 4→14 갱신, Wave 3 Step 1b 완료 사실 추가, 미구현 목록에서 보고서 확장 제거하고 치유량/명줄 추가, 다음 goal 추천 문장을 Step 1c로 갱신.

문서 크기(`wc -c`, 100KB 이하 확인):

```
8796  docs/design/Combat_System_Implementation_Plan_Index.md
14458 docs/dev/Combat_System_Operating_Guide.md
8185  docs/dev/Combat_System_Goal_Prompt.md
```

커밋: `f0ede24 docs(combat): record Wave 3 Step 1b conclusion report completion`

## 최종 검증

```
cargo fmt --all -- --check                                   → 통과 (0 diff)
cargo test -p escape-core --test combat_conclusion_wave2     → 14 passed; 0 failed
cargo test -p escape-core --test combat_resolution_wave2     → 16 passed; 0 failed
cargo test -p escape-core --test combat_spectator_wave3      → 19 passed; 0 failed
cargo test --workspace --no-fail-fast                        → 283 passed; 0 failed
git diff --check                                              → 통과 (0 issue)
```

**기존 combat 80개 테스트 개수 그대로 통과 근거**: `combat_conclusion_wave2.rs`를 제외한 7개 combat 테스트 파일(`combat_contract_wave1.rs` 5, `combat_execution_wave2.rs` 6, `combat_opportunity_wave1.rs` 12, `combat_resolution_wave2.rs` 16, `combat_simulation_wave2.rs` 10, `combat_spectator_wave3.rs` 19, `combat_state_wave1.rs` 8 = 76개)는 이번 작업에서 전혀 건드리지 않았다. `combat_conclusion_wave2.rs`는 기존 4개 테스트 본문 로직을 그대로 두고(`tick_millis` 필드만 추가) 신규 10개를 추가해 4→14. 즉 baseline 80개(76+4) 전부가 그대로 통과하고, 신규 10개가 더해져 총 combat 테스트 90개, 전체 workspace 283개(baseline 273 + 신규 10).

## 치유량·명줄을 넣지 않은 이유 (플랜 인용)

플랜 원문(§"치유량을 넣지 않는 이유"):

> `combat_resolution.rs:293`의 체력 갱신은 감소 전용이다. `CombatAttackDefinition`에도 `CombatEffectDefinition`에도 회복을 표현하는 필드가 없고, `grep -niE "heal|치유|회복량"`가 combat 모듈에서 회복 로직을 하나도 찾지 못한다. 합산할 원천이 없으므로 `healing_hundredths: 0`을 보고하면 값이 있는 척하는 거짓값이 된다 ... 필드를 만들지 않는다.

플랜 원문(§"명줄을 넣지 않는 이유"):

> 명줄은 치명적 실패를 감당하는 런 단위 메타 자원이다 ... 소모 여부는 각 인카운터의 패배 결과 정의가 소유하고 ... 명줄 소모는 전투 tick resolver가 아니라 그 위 레이어의 판정이다. 코드에는 패배 결과 스키마도 명줄 상태도 없다 ... 전투 resolver가 발명하지 않는다.

두 필드 모두 코드에 추가하지 않았다(`CombatCombatantReport`, `CombatConclusionReport` 어디에도 healing/lifeline 관련 필드 없음).

## 스킵/이탈 항목과 사유

- 플랜에 정의된 모든 WP(1~6)를 스킵 없이 순서대로 완료했다.
- 유일한 이탈은 "lib.rs re-export"를 WP-2 시점에 놓쳐 별도 커밋(`087424f`)으로 보완한 것 — 플랜의 예상 변경 파일 표 안의 파일(`lib.rs`)이라 범위 위반은 아니며, WP 번호를 부여하지 않고 순수 보완 커밋으로 분리했다.
- 명시적 범위 밖 항목(플랜 §"명시적 범위 밖")은 전부 미착수: 치유량/명줄, `ScenePage`/WASM 노출, terminal/Web 화면, 프리셋/우선 목표 규칙, 밸런스 확정 수치.
- 정본 13이 금지한 항목(전략 평가·전환점·원인 분석·조언·MVP·이전 전투 비교)은 필드·함수 모두 미구현.

## 최종 체크리스트 대조

- [x] 치유량·명줄 필드를 만들지 않았다
- [x] 금지 항목(전략 평가·전환점·원인 분석·조언·MVP·이전 전투 비교) 필드/함수 없음
- [x] 하이라이트가 0건일 때 `None` (`Some(0)` 아님) — `top_id_by`가 `filter(|(_, v)| *v > 0)`로 강제
- [x] 동점 tie-break가 id 오름차순 최소로 고정 — `combatants`가 이미 id 오름차순이고 엄격 `>` 비교
- [x] RNG 추가 호출 0회, `HashMap` 순회 의존 없음 — `BTreeMap`/`Vec`만 사용
- [x] 기존 `combat_conclusion_wave2.rs` 4개 테스트 본문 로직 무수정 (request 리터럴에 `tick_millis` 추가만)
- [x] 각 WP에서 red 확인 후 구현, red 출력 기록 (WP-1~4; WP-5는 신규 로직 없어 red 불필요, 본 보고서에 명시)
- [x] `cargo fmt --all -- --check`, `git diff --check` 통과
- [x] `cargo test --workspace --no-fail-fast` 0 failed (283 passed)
- [x] WP-6 문서 3개 갱신
- [x] `cli_smoke.rs`·`.claude/worktrees/`·fixture/generated JSON 무변경
- [x] 보고서 `fable_combat_wave3_step1b_report.md` 작성
- [x] 보고서/커밋 메시지를 Write 툴로 UNC 경로에 작성하고 `git commit -F`로 커밋

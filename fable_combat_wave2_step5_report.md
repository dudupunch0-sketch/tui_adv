# Combat Wave 2 Step 5 — 구현 보고서

작성: coding subagent (sonnet, effort medium)
플랜: `fable_combat_wave2_step5_2608020117.md`
브랜치: `claude/combat-wave3-step1a-v2`
Baseline: `78923b5` (263 passed / 0 failed, WSL 실측)

## 요약

- WP-1~WP-5 전부 순서대로 구현, WP당 커밋 1개 (총 5커밋).
- `cargo test --workspace --no-fail-fast` 최종 **273 passed / 0 failed** (baseline 263 + 신규 10개: resolution_wave2 +5, spectator_wave3 +5).
- fingerprint 공식 두 호출부(`combat_resolution.rs`) 완전 동일, line number만 이동.
- 기존 combat 테스트 70개는 전부 그대로 통과 (신규 테스트만 추가, 기존 테스트 본문 무수정).
- 스킵/이탈 항목 없음.

## WP별 변경 파일과 내용

### WP-1 — 스냅샷 필드 추가 (commit `4b72e76`)

- `crates/escape-core/src/combat_resolution.rs`: `CombatResolutionFrame`에 `#[serde(default)] pub combatants: Vec<CombatResolutionCombatant>` 필드 추가 (fingerprint 미포함, 문서 주석 포함). `resolve()`는 아직 `Vec::new()`만 채움.
- `crates/escape-core/tests/combat_conclusion_wave2.rs`: 52행 근처 `CombatResolutionFrame` 리터럴에 `combatants: vec![],` **1줄만** 추가.

검증:
```bash
cargo fmt --all -- --check   # 통과 (출력 없음)
```
```bash
cargo test --workspace --no-fail-fast
```
결과: **263 passed / 0 failed** (baseline과 완전 동일) — fingerprint 공식이 새 필드의 영향을 받지 않았다는 직접 증거. 각 test binary별 수치(4,5,6,12,11,10,14,8,9,32,11,3,3,3,8,4,23,61,36 = 합계 263)도 baseline과 1:1 동일.

### WP-2 — 스냅샷 채우기 (commit `acf7201`)

- `crates/escape-core/src/combat_resolution.rs`: tick 루프 끝, `outcomes` 처리 직후에 `combatants.values().cloned().collect()`로 스냅샷을 만들어 `CombatResolutionFrame.combatants`에 채움. `into_values()`가 아니라 `.values().cloned()`를 사용해 맵을 다음 tick에도 계속 쓸 수 있게 함. 판정 재계산·RNG 추가 호출 없음 — `combatants` `BTreeMap`은 `resolve()`가 이미 유지하던 상태를 읽기만 한다.

### WP-3 — 스냅샷 회귀 테스트 (commit `8c6e17b`)

- `crates/escape-core/tests/combat_resolution_wave2.rs`에 5개 테스트 추가 (기존 11개 본문 무수정, 총 16개):
  - `frame_snapshot_is_id_sorted_and_covers_every_combatant`
  - `frame_snapshot_reflects_the_tick_damage_from_its_own_outcomes`
  - `last_frame_snapshot_matches_final_state_combatants`
  - `combatants_field_is_additive_optional_for_deserialization`
  - `frame_snapshots_are_deterministic_across_identical_runs`

검증:
```bash
cargo test -p escape-core --test combat_resolution_wave2
```
결과: `running 16 tests ... test result: ok. 16 passed; 0 failed`

```bash
cargo test -p escape-core --test combat_conclusion_wave2
```
결과: `running 4 tests ... test result: ok. 4 passed; 0 failed`

```bash
cargo test --workspace --no-fail-fast
```
결과: **268 passed / 0 failed** (263 + 5).

### WP-4 — cue 2개, red 먼저 확인 (commit `3b327ef`)

`crates/escape-core/tests/combat_spectator_wave3.rs`에 헬퍼(`incapacitated_only_request`, `balance_broken_only_request`, `all_cues_request`, `spectator_request_for`) + 5개 테스트를 **먼저** 추가 (기존 14개 본문 무수정, 총 19개):
- `incapacitated_cue_marks_a_combatant_whose_health_snapshot_hit_zero`
- `balance_broken_cue_marks_a_combatant_whose_balance_snapshot_hit_zero`
- `neither_state_cue_applies_when_health_and_balance_stay_above_zero`
- `cue_ordering_is_fixed_attack_hit_evade_balance_broken_incapacitated`
- `empty_combatant_snapshot_yields_no_state_cues_and_no_error`

**Red 확인 (구현 전, `CombatSpectatorCue::BalanceBroken`/`Incapacitated`가 없는 상태로 컴파일)**:
```bash
cargo test -p escape-core --test combat_spectator_wave3
```
red 출력 (10개 E0599, 발췌):
```
error[E0599]: no variant, associated function, or constant named `BalanceBroken` found for enum `CombatSpectatorCue` in the current scope
   --> crates/escape-core/tests/combat_spectator_wave3.rs:566:54
error[E0599]: no variant, associated function, or constant named `Incapacitated` found for enum `CombatSpectatorCue` in the current scope
   --> crates/escape-core/tests/combat_spectator_wave3.rs:578:55
error[E0599]: no variant, associated function, or constant named `Incapacitated` found for enum `CombatSpectatorCue` in the current scope
   --> crates/escape-core/tests/combat_spectator_wave3.rs:579:55
error[E0599]: no variant, associated function, or constant named `BalanceBroken` found for enum `CombatSpectatorCue` in the current scope
   --> crates/escape-core/tests/combat_spectator_wave3.rs:587:59
...
error: could not compile `escape-core` (test "combat_spectator_wave3") due to 10 previous errors
```
(`grep -c "^error\[E0599\]" 로그` = 10)

**구현 (`crates/escape-core/src/combat_spectator.rs`)**:
- `CombatSpectatorCue`에 `BalanceBroken`, `Incapacitated` 변형 추가 (`Evade` 뒤, 선언 순서 = 정렬 순서).
- `spectate()`에 `combatants_by_tick: BTreeMap<u32, &Vec<CombatResolutionCombatant>>` 추가, `cues_for()`에 스냅샷 인자로 전달.
- `cues_for()`: 스냅샷에서 `id`로 찾은 combatant의 `balance_hundredths <= 0` → `BalanceBroken`, `current_health_hundredths <= 0` → `Incapacitated`. 스냅샷 없음/빈 Vec/해당 id 없음이면 조용히 스킵 (에러 아님). 이전 tick과 비교하지 않음.

**Green 확인**:
```bash
cargo test -p escape-core --test combat_spectator_wave3
```
결과: `running 19 tests ... test result: ok. 19 passed; 0 failed` (신규 5개 포함 전부 통과).

```bash
cargo test --workspace --no-fail-fast
```
결과: **273 passed / 0 failed** (268 + 5).

### WP-5 — 문서 갱신 (commit `8c7a782`)

- `docs/design/Combat_System_Implementation_Plan_Index.md`
  - `status:` → `wave2-step5-complete`
  - 단계 표에 `fable_combat_wave2_step5_2608020117.md` 행 추가 (Wave 3 Step 1a 행 뒤, Step 1b 앞)
  - "현재 코드와 정본의 경계"에서 per-tick 스냅샷 부재 항목 제거, 정본 13 공용 연출 문법 5개 전부 확보됐음을 명시
  - Wave 2 Step 5 구현 위치 라인 추가 (신규 테스트 수 포함)
- `docs/dev/Combat_System_Operating_Guide.md`: "완료" 목록에 Wave 2 Step 5 bullet 추가 (신규 테스트 함수명 전부 명시), "아직 열지 않음"에서 스냅샷 부재 항목 제거
- `docs/dev/Combat_System_Goal_Prompt.md`: baseline 목록에 Wave 2 Step 5 완료 bullet 추가

각 문서 `wc -c`:
```
6636 docs/design/Combat_System_Implementation_Plan_Index.md
12687 docs/dev/Combat_System_Operating_Guide.md
7385 docs/dev/Combat_System_Goal_Prompt.md
```
전부 100KB 이하.

## 최종 검증

```bash
cargo fmt --all -- --check    # 통과 (출력 없음)
git diff --check              # 통과 (exit 0)
cargo test --workspace --no-fail-fast
```
최종 합계: **273 passed / 0 failed / 0 ignored** — 모든 test binary에서 실패 없음.

test binary별 수치: 0,4,5,6,12,16,10,19,8,9,32,11,3,3,3,8,4,23,0,61,0,36,0,0
(16 = combat_resolution_wave2, 19 = combat_spectator_wave3, 나머지는 baseline과 동일)

## fingerprint 무결성 증거

```
--- 현재 (HEAD) ---
384:        let fp = fingerprint(&(frame.tick, &outcomes));
406:    let fp = fingerprint(&(execution.fingerprint.clone(), &frames, &state, &full_log));
--- baseline (78923b5) ---
378:        let fp = fingerprint(&(frame.tick, &outcomes));
396:    let fp = fingerprint(&(execution.fingerprint.clone(), &frames, &state, &full_log));
```
두 호출부 텍스트가 완전히 동일하다 (line number만 앞선 코드 추가로 이동). WP-1 직후 263 유지가 이를 실측으로 뒷받침한다.

## 기존 combat 테스트 70개 개수 그대로 통과 근거

- baseline: `crates/escape-core/tests/combat_*.rs` 8파일 70 테스트.
- 이번 slice에서 손댄 파일은 `combat_conclusion_wave2.rs`(1줄만, 기존 4개 테스트 본문 무수정), `combat_resolution_wave2.rs`(+5, 기존 11개 무수정), `combat_spectator_wave3.rs`(+5, 기존 14개 무수정) 3개뿐.
- 나머지 5개 combat 테스트 파일(`combat_contract_wave1.rs` 5, `combat_state_wave1.rs` 8, `combat_opportunity_wave1.rs` 12, `combat_simulation_wave2.rs` 10, `combat_execution_wave2.rs` 6)은 전혀 건드리지 않음 — `git status --short`/`git diff --stat`로 미변경 확인.
- 최종 workspace 실행에서 위 8개 combat 파일의 test binary 수치는 5, 8, 12, 10, 6, 16(11+5), 4, 19(14+5) — 기존 70 + 신규 10 = 80.

## 스킵/이탈 항목

없음. 플랜의 WP-1~WP-5 전부 계획대로 구현했고, 위험해 보여 스킵한 WP도 없다.

## 최종 체크리스트 대조

- [x] fingerprint 공식 무변경 (위 diff 비교)
- [x] WP-1 직후 workspace 263 유지 확인
- [x] `combat_conclusion_wave2.rs` 변경 정확히 1줄 (`combatants: vec![],`)
- [x] 기존 `combat_resolution_wave2.rs` 11개·`combat_spectator_wave3.rs` 14개 테스트 본문 무수정
- [x] RNG 추가 호출 0회 (스냅샷은 `combatants.values().cloned()`만 사용), `HashMap` 순회 의존 없음 (`BTreeMap`만 사용)
- [x] WP-4 red 출력 기록 (위 10개 E0599)
- [x] `cargo fmt --all -- --check`, `git diff --check` 통과
- [x] `cargo test --workspace --no-fail-fast` 0 failed (273 passed)
- [x] WP-5 문서 3개 갱신, per-tick 스냅샷 부재 항목 제거
- [x] `cli_smoke.rs`·`.claude/worktrees/`·fixture/generated JSON 무변경 (전부 미커밋 상태로 그대로 둠)
- [x] 보고서에 red→green 출력, 실행 명령, 스킵 항목 없음 기록
- [x] 보고서 작성에 Write 툴 사용, backtick 포함 커밋 메시지는 파일로 작성 후 `git commit -F`로 전달

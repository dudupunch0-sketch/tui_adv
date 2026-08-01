# Combat Wave 2 Step 5 — resolution frame의 per-tick 전투원 스냅샷 + 보류 cue 2개 완성

작성: 2026-08-02
작성자: Fable (orchestrator plan)
구현 담당: coding subagent (sonnet, effort medium)

## Baseline

- 기준 커밋: `origin/main` = `3bb8ad5` (PR #178 머지 후) 위의 `claude/combat-wave3-step1a-v2` (`78923b5`)
- Baseline 검증 상태: `cargo test --workspace --no-fail-fast` → **263 passed / 0 failed** (2026-08-02 WSL 실측)
- combat 테스트 현황: `crates/escape-core/tests/combat_*.rs` 8파일 / 70 테스트 (Wave 1 25 + Wave 2 31 + Wave 3 1a 14)

## 왜 이 slice인가

Wave 3 Step 1a에서 정본 13의 공용 연출 문법 5개 중 **`BalanceBroken`(균형 붕괴=흔들림/기울어짐)과 `Incapacitated`(전투불능=흐려짐/표식)를 구현하지 못했다.** 사유는 `CombatResolutionFrame`에 tick별 전투원 상태가 없어서(`CombatResolutionState`는 전투 종료 후 최종 상태 1개뿐) "이 tick에 균형이 무너졌는가 / 전투불능이 되었는가"를 알 수 없다는 것이었다. adapter에서 `damage_hundredths`·`balance_delta_hundredths` 델타를 재누적하면 `combat_resolution.rs`의 clamp 로직과 갈라질 수 있어 의도적으로 보류했다.

이 slice가 그 구멍을 core에서 닫고, 보류했던 cue 2개를 완성한다. 스냅샷만 추가하고 소비자가 없으면 죽은 코드이므로 **두 작업을 한 slice로 묶는다.**

## 정본 근거

- [13. 감독형 관전·전략 피드백 시스템](https://app.notion.com/p/3a937e69695e81daa01df6f79823c4d6): "공용 문법은 공격=짧은 전진/복귀, 피격=밀림/진동, 회피=측면 이동, **균형 붕괴=흔들림/기울어짐, 전투불능=흐려짐/표식**이다."
- 같은 문서: "연출은 판정 뒤 표현만 담당하여 seed·판정·AI 순서에 영향하지 않는다." → 스냅샷은 판정 결과의 **기록**이며 판정을 바꾸지 않는다.

## Scope

- P1 (`combat_resolution.rs`): `CombatResolutionFrame`에 tick 종료 시점 전투원 스냅샷을 additive로 추가
- P2 (`combat_spectator.rs`): 스냅샷을 읽어 `BalanceBroken` / `Incapacitated` cue 파생
- P3: 회귀 테스트와 문서 갱신

## Hard invariants (위반 금지)

1. **fingerprint 공식을 바꾸지 않는다.** `combat_resolution.rs:378`의 `fingerprint(&(frame.tick, &outcomes))`와 `:396`의 `fingerprint(&(execution.fingerprint.clone(), &frames, &state, &full_log))`를 **그대로 둔다.**
   - 근거: 스냅샷은 이미 fingerprint에 들어간 입력(초기 state + outcomes)에서 결정론적으로 파생되는 값이라 감사 무결성에 추가 정보를 주지 않는다. 반면 공식을 바꾸면 기존 fingerprint 계약이 전부 깨진다.
   - `&frames`가 fingerprint 입력에 들어가지만 `frames`의 **Serialize 결과가 아니라 Hash/canonical 표현**을 쓰는지 반드시 확인하고, 새 필드가 값에 섞여 들어가면 그 시점에 멈추고 보고하라. 기존 combat 테스트의 fingerprint 비교가 깨지는지가 판정 기준이다.
2. **판정 재계산 금지**: 스냅샷은 `resolve()`가 이미 유지하는 `combatants: BTreeMap<String, CombatResolutionCombatant>`(`combat_resolution.rs:133`)를 tick 루프 끝에서 복사하는 것이다. 피해·명중·효과를 다시 계산하지 않는다. RNG를 추가 호출하지 않는다.
3. **additive-optional 직렬화**: 새 필드는 `#[serde(default)]`. `CONTENT_BUNDLE_SCHEMA_VERSION`·`SAVE_SCHEMA_VERSION`을 올리지 않는다. 기존 JSON이 계속 읽혀야 한다.
4. **결정론**: `BTreeMap`/정렬된 `Vec`만. `HashMap` 순회 의존 금지. 스냅샷 `Vec`은 id 오름차순 고정.
5. **밸런스 수치 금지**: cue 판정 임계값을 임의로 만들지 않는다. "0 이하"만 쓴다.
6. **신규 의존성 금지**: `Cargo.toml` 수정 금지.
7. **다른 작업자 변경 보존**: `crates/escape-terminal/tests/cli_smoke.rs`는 읽기만. `.claude/worktrees/`는 읽지도 쓰지도 않는다.
8. **콘텐츠 데이터 불변**: `crates/escape-core/fixtures/**`, `web/src/data/generated/**` 수정 금지.
9. **renderer 미접촉**: `ScenePage`·`escape-wasm`·`escape-terminal`·`web/src`를 건드리지 않는다 (Step 1c/1d 소관).

## 예상 변경 파일 (이 목록 밖은 손대지 말 것)

| 파일 | 성격 |
|---|---|
| `crates/escape-core/src/combat_resolution.rs` | 필드 추가 + tick 루프에서 스냅샷 채우기 |
| `crates/escape-core/src/combat_spectator.rs` | cue 2개 추가 |
| `crates/escape-core/tests/combat_conclusion_wave2.rs` | **1줄만** — `CombatResolutionFrame` 리터럴(52행)에 새 필드 추가 |
| `crates/escape-core/tests/combat_resolution_wave2.rs` | 스냅샷 회귀 테스트 추가 (기존 11개 수정 금지) |
| `crates/escape-core/tests/combat_spectator_wave3.rs` | cue 회귀 테스트 추가 (기존 14개 수정 금지) |
| `docs/design/Combat_System_Implementation_Plan_Index.md` | 단계 표·경계 갱신 |
| `docs/dev/Combat_System_Operating_Guide.md` | 단계 기록 |
| `docs/dev/Combat_System_Goal_Prompt.md` | 완료 목록 |

## 공개 API 변경

```rust
// combat_resolution.rs
pub struct CombatResolutionFrame {
    pub tick: u32,
    pub outcomes: Vec<CombatAttackOutcome>,
    /// 이 tick의 모든 outcome을 적용한 뒤의 전투원 상태. id 오름차순.
    /// `CombatResolutionState`(전투 종료 후 최종 상태)와 달리 tick 단위 기록이며,
    /// 관전 연출이 균형 붕괴·전투불능 시점을 알 수 있게 한다.
    /// `fingerprint`는 이 필드를 포함하지 않는다 — outcomes에서 결정론적으로 파생되기 때문이다.
    #[serde(default)]
    pub combatants: Vec<CombatResolutionCombatant>,
    pub fingerprint: String,
}
```

```rust
// combat_spectator.rs
pub enum CombatSpectatorCue {
    Attack,
    Hit,
    Evade,
    BalanceBroken,   // 신규
    Incapacitated,   // 신규
}
```

`CombatSpectatorCue`는 기존 `Attack < Hit < Evade` 정렬 규칙을 유지하고 그 뒤에 `BalanceBroken < Incapacitated`를 붙인다. 즉 선언 순서가 정렬 순서다.

## 파생 규칙

### 스냅샷 (`combat_resolution.rs`)
- 각 tick의 outcome 처리 루프가 끝난 직후, `combatants` 맵의 값들을 `into_values()`가 아니라 **복사**해 `Vec`으로 담는다 (맵은 다음 tick에도 계속 쓰인다).
- 순서는 `BTreeMap` 순회 = id 오름차순. 별도 정렬 불필요하지만 의도를 주석으로 남긴다.
- `CombatResolutionState`(최종 상태) 생성 로직은 그대로 둔다.

### cue (`combat_spectator.rs`)
같은 tick의 `resolution.frames[i].combatants`만 본다. 이전 tick과 비교하지 않는다 (상태형 cue이므로 "지금 그 상태인가"가 기준이다).
- `BalanceBroken`: 그 tick 스냅샷에서 `balance_hundredths <= 0`
- `Incapacitated`: 그 tick 스냅샷에서 `current_health_hundredths <= 0`
- 스냅샷에 해당 id가 없으면(구 JSON에서 `#[serde(default)]`로 빈 Vec인 경우 포함) 두 cue를 붙이지 않는다. 에러가 아니다.

## Work packages (순서 고정, WP당 커밋 1개)

### WP-1 — 스냅샷 필드 추가
`CombatResolutionFrame.combatants` 추가 + `combat_conclusion_wave2.rs:52` 리터럴에 `combatants: vec![]` 1줄 추가. `resolve()`는 아직 빈 Vec을 넣는다.
검증: `cargo fmt --all -- --check`, `cargo test --workspace --no-fail-fast` → **263 유지** (fingerprint가 안 바뀌었다는 증거다. 숫자가 줄거나 실패하면 invariant 1 위반이므로 멈추고 보고하라).

### WP-2 — 스냅샷 채우기
tick 루프 끝에서 실제 상태를 복사해 넣는다.
검증: 위 + WP-3 테스트.

### WP-3 — 스냅샷 회귀 테스트 (`combat_resolution_wave2.rs`에 추가)
1. 각 frame의 `combatants`가 id 오름차순이고 모든 전투원을 포함한다
2. tick 1개 시나리오에서 스냅샷의 `current_health_hundredths`가 `outcomes`의 `damage_hundredths` 적용 결과와 일치한다
3. 마지막 frame의 스냅샷이 `CombatResolutionState.combatants`와 일치한다
4. `combatants` 필드가 없는 JSON을 `CombatResolutionFrame`으로 역직렬화하면 빈 Vec이 되고 에러가 아니다 (additive-optional 증명)
5. 같은 입력 두 번 → 동일 스냅샷, 동일 fingerprint

### WP-4 — cue 2개 (테스트 red 먼저)
`BalanceBroken`/`Incapacitated` 테스트를 `combat_spectator_wave3.rs`에 먼저 쓰고 **red를 실제로 확인한 뒤** 구현한다. red 출력을 보고서에 남긴다.
테스트 최소 4케이스:
1. 체력이 0으로 내려간 전투원에게 `Incapacitated`가 붙는다
2. 균형이 0으로 내려간 전투원에게 `BalanceBroken`이 붙는다
3. 둘 다 아닌 전투원에게는 두 cue가 붙지 않는다
4. cue 정렬이 `Attack < Hit < Evade < BalanceBroken < Incapacitated`로 고정된다
5. 스냅샷이 빈 Vec인 frame에서는 두 cue가 붙지 않고 에러도 아니다

시나리오 만들기 힌트: 기존 `resolution_request()`의 `combatant()` 헬퍼는 `current_health: 100`, `balance: 100`이다. `power_hundredths`/`balance_power_hundredths`를 충분히 크게 잡으면 한 tick에 0으로 clamp된다. **밸런스 의미가 있는 수치가 아니라 테스트 픽스처이므로 임의값을 써도 된다** — 단 프로덕션 코드 상수로는 넣지 않는다.

### WP-5 — 문서 갱신 (생략 금지)
- `docs/design/Combat_System_Implementation_Plan_Index.md`
  - `status:` → `wave2-step5-complete`
  - 단계 표에 `fable_combat_wave2_step5_2608020117.md` 행 추가 (구현 단위: "resolution frame per-tick 전투원 스냅샷과 균형 붕괴·전투불능 cue", non-goal: "renderer 노출·보고서 확장·밸런스 확정값"). Wave 3 Step 1a 행 뒤, Step 1b 앞에 둔다.
  - "현재 코드와 정본의 경계"에서 **per-tick 스냅샷 부재 항목을 제거**하고, 공용 연출 문법 5개가 전부 확보됐음을 적는다.
- `docs/dev/Combat_System_Operating_Guide.md`, `docs/dev/Combat_System_Goal_Prompt.md`에 기존 형식대로 한 줄씩 추가한다.
- 문서에 수치를 적을 때는 **그 수치를 고정하는 테스트 함수명을 같이 적는다.**
- 각 문서 100KB 이하 유지.

## 명시적 범위 밖

- 전투 종료 보고서 확장 (전투 시간, 캐릭터별 피해/치유/처치 수) → Wave 3 Step 1b
- `ScenePage` 필드 추가, WASM 노출 → Wave 3 Step 1c
- terminal/Web 렌더러 → Wave 3 Step 1d
- 치유(healing)·명줄(life thread) — 파이프라인에 개념 자체가 없다. 정본 10번 확인 후 별도 slice
- `CombatSpectatorCue`에 6번째 종류 추가
- fingerprint 공식 변경
- 프리셋·우선 목표 규칙
- 밸런스 확정 수치

## 검증 명령

```bash
cargo fmt --all -- --check
cargo test -p escape-core --test combat_resolution_wave2
cargo test -p escape-core --test combat_conclusion_wave2
cargo test -p escape-core --test combat_spectator_wave3
cargo test --workspace --no-fail-fast
git diff --check
```

기준: 263 passed + 신규 증가분 / 0 failed. 기존 combat 테스트 70개가 **개수 그대로** 통과해야 한다.

## 최종 체크리스트

- [ ] fingerprint 공식 무변경 (`git diff`로 두 `fingerprint(&(...))` 호출부 확인)
- [ ] WP-1 직후 workspace 263 유지 확인 (fingerprint 안정성 증거)
- [ ] `combat_conclusion_wave2.rs` 변경이 정확히 1줄
- [ ] 기존 `combat_resolution_wave2.rs` 11개·`combat_spectator_wave3.rs` 14개 테스트 본문 무수정
- [ ] RNG 추가 호출 0회, `HashMap` 순회 의존 없음
- [ ] WP-4 red 출력 기록
- [ ] `cargo fmt --all -- --check`, `git diff --check` 통과
- [ ] `cargo test --workspace --no-fail-fast` 0 failed
- [ ] WP-5 문서 3개 갱신, per-tick 스냅샷 부재 항목 제거
- [ ] `cli_smoke.rs`·`.claude/worktrees/`·fixture/generated JSON 무변경
- [ ] 보고서 `fable_combat_wave2_step5_report.md`에 red→green 출력, 실행 명령, 스킵 항목과 사유 기록
- [ ] **보고서/PR 본문에 backtick 있는 마크다운을 셸 heredoc으로 넣지 말 것** — 파일 쓰기 도구로 쓰고 `-F`/`--body-file`로 넘긴다 (이 세션에서 2회 유실 사고 있었음)

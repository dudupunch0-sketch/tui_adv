# Combat Wave 2 Step 6 — 전투 기록에 provenance(simulation version·tick 길이·manifest fingerprint) 저장

작성: 2026-08-02
작성자: Fable (orchestrator plan)
구현 담당: coding subagent (sonnet, effort medium)

## Baseline

- 기준 브랜치: `claude/combat-wave3-step1a-v2` (`origin/main` = `3bb8ad5`보다 24 커밋 앞)
- Baseline 검증 상태: `cargo test --workspace --no-fail-fast` → **283 passed / 0 failed** (2026-08-02 WSL 실측)
- combat 테스트 현황: `crates/escape-core/tests/combat_*.rs` 8파일 / 90 테스트

## 왜 이 slice인가 (정본 요구 + 실측된 설계 결함)

정본 03 「핵심 상태 시스템」의 RNG·재시도·버전 절:

> 같은 고정층 + 변경층 + seed + 개입 ID/tick + simulation version은 같은 결과를 낸다. **이 결정성은 같은 version 내부에서만 보장한다.** … 활성 전투와 즉시 재시도는 같은 simulation version을 사용하고, **전투 기록에는 version을 저장한다.** 이후 갱신된 version이 과거 seed 결과를 재현할 필요는 없다.

이 문장이 두 가지를 확정한다.

1. **fingerprint 값을 schema 추가에 대해 고정할 필요가 없다.** 결정성은 같은 simulation version 안에서만 보장된다. (Wave 2 Step 5에서 frame 필드 추가로 `CombatResolutionResult.fingerprint` 값이 바뀐 것은 계약 위반이 아니다. Wave 3 Step 1c 선결 과제로 인덱스에 올려 둔 항목의 답이 이것이다.)
2. **대신 전투 기록이 simulation version을 저장해야 한다.** 현재 저장하지 않는다.

실측된 결함: `CombatExecutionResult`(`combat_execution.rs:52-62`)는 입력을 하나도 보관하지 않는다. `execute()`는 `request.input.manifest.simulation_version`과 `request.input.config.tick_millis`를 손에 들고 있는데도 결과에 남기지 않는다. 그 결과:

- `CombatSpectatorRequest.tick_millis`(Step 1a)와 `CombatConclusionRequest.tick_millis`(Step 1b)를 **호출자가 각각 따로 넘기게** 됐다. 같은 값이 두 곳에 있어 갈라질 수 있고, 실제로 Step 1a에서 이 값이 `0`으로 굳어 있던 결함을 리뷰에서 잡았다.
- Step 1c에서 `simulation_version`까지 세 번째로 손으로 넘기면 같은 결함이 세 배가 된다.

이 slice는 provenance를 실행 결과에 심고 **중복 파라미터를 제거**한다.

## Scope

- P1: `CombatProvenance`를 정의하고 `CombatExecutionResult`에 additive로 추가
- P2: `CombatSpectatorRequest.tick_millis`와 `CombatConclusionRequest.tick_millis` **제거** — provenance에서 읽는다
- P3: 회귀 테스트와 문서 갱신

## Hard invariants (위반 금지)

1. **판정 재계산 금지**: provenance는 `execute()`가 이미 가진 입력을 복사하는 것뿐이다. RNG 추가 호출 금지, seed 파생 로직 변경 금지.
2. **기존 판정·로그·frame 생성 로직을 바꾸지 않는다.** `effective_seed`/`namespace` 결정 로직(`combat_execution.rs:65-80`)도 그대로 둔다.
3. **fingerprint 공식(호출부)을 바꾸지 않는다.** 새 필드가 값에 섞이는 것은 정본 03 근거로 허용한다. 단 **기존 combat 테스트가 개수 그대로 통과**해야 한다. 깨지면 멈추고 보고하라.
4. **additive-optional 직렬화**: 새 필드는 `#[serde(default)]`. schema version을 올리지 않는다. provenance가 없는 기존 JSON이 계속 읽혀야 한다.
5. **결정론**: `BTreeMap`/정렬 `Vec`만. `HashMap` 순회 의존 금지.
6. **거짓값 금지**: provenance가 비어 있는(구 JSON 역직렬화) 결과로 `tick_millis`를 구할 수 없으면 **0을 쓰지 말고 에러를 낸다** (`MissingProvenance`). 이 slice의 존재 이유가 "값이 있는 척하지 않기"다.
7. **신규 의존성 금지**, `Cargo.toml` 수정 금지.
8. **renderer 미접촉**: `ScenePage`·`escape-wasm`·`escape-terminal`·`web/src` 건드리지 않는다 (Step 1c/1d 소관).
9. **다른 작업자 변경 보존**: `crates/escape-terminal/tests/cli_smoke.rs` 읽기만. `.claude/worktrees/` 읽지도 쓰지도 않는다.
10. **콘텐츠 데이터 불변**: `crates/escape-core/fixtures/**`, `web/src/data/generated/**` 수정 금지.

## 예상 변경 파일 (이 목록 밖은 손대지 말 것)

| 파일 | 성격 |
|---|---|
| `crates/escape-core/src/combat_execution.rs` | `CombatProvenance` 정의 + 결과에 채우기 |
| `crates/escape-core/src/combat_spectator.rs` | `tick_millis` 파라미터 제거, provenance에서 읽기 |
| `crates/escape-core/src/combat_conclusion.rs` | 같음 |
| `crates/escape-core/src/lib.rs` | `CombatProvenance` re-export 추가 |
| `crates/escape-core/tests/combat_execution_wave2.rs` | provenance 테스트 추가 (기존 6개 본문 무수정) |
| `crates/escape-core/tests/combat_spectator_wave3.rs` | `tick_millis` 관련 케이스 갱신 |
| `crates/escape-core/tests/combat_conclusion_wave2.rs` | 같음 |
| `docs/design/Combat_System_Implementation_Plan_Index.md` | 단계 표·경계·선결 과제 해소 |
| `docs/dev/Combat_System_Operating_Guide.md` | 단계 기록 |
| `docs/dev/Combat_System_Goal_Prompt.md` | 완료 목록 |

## 공개 API 변경

```rust
// combat_execution.rs

/// 전투 기록의 출처. 정본 03 "전투 기록에는 version을 저장한다"의 구현이다.
/// 결과에서 입력 맥락을 되찾을 수 있게 하여, 하위 단계가 같은 값을 호출자에게
/// 다시 받지 않도록 한다.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CombatProvenance {
    pub simulation_version: CombatSimulationVersion,
    /// tick 한 칸의 길이(ms). 입력 `CombatSimulationConfig.tick_millis`를 그대로 옮긴다.
    pub tick_millis: u32,
    pub manifest_fingerprint: String,
}

pub struct CombatExecutionResult {
    // ... 기존 필드 전부 유지 ...
    #[serde(default)]
    pub provenance: Option<CombatProvenance>,
    pub fingerprint: String,
}
```

`Option`인 이유: provenance가 없는 기존 JSON을 계속 읽어야 한다(invariant 4). `execute()`가 만드는 결과는 **항상 `Some`**이다.

`manifest_fingerprint`는 `request.input.manifest.fingerprint()`의 결과를 쓴다. 실패하면 기존 `CombatExecutionError::InvalidInput`으로 매핑한다 (새 에러 변형을 만들지 않는다).

### 제거되는 것

```rust
// combat_spectator.rs
pub struct CombatSpectatorRequest {
    pub resolution: CombatResolutionResult,
    pub participants: Vec<CombatSimulationParticipant>,
    pub catalog: CombatEffectCatalog,
    // pub tick_millis: u32,   <-- 제거
}

// combat_conclusion.rs
pub struct CombatConclusionRequest {
    pub resolution: CombatResolutionResult,
    pub participants: Vec<CombatSimulationParticipant>,
    pub policy: CombatTerminationPolicy,
    // pub tick_millis: u32,   <-- 제거
}
```

두 함수는 `request.resolution.execution.provenance`에서 `tick_millis`를 읽는다.

에러 변형 변경:
- `CombatSpectatorError::InvalidTickMillis(u32)` → **`MissingProvenance`로 교체**. provenance가 `None`이거나 `tick_millis == 0`이면 이 에러다.
- `CombatConclusionError::InvalidTickMillis(u32)` → 같음.

`InvalidTickMillis`를 남겨 두지 않는다. 호출자가 값을 넘기지 않으므로 그 에러는 도달 불가능해진다.

## 파생 규칙

- `provenance.simulation_version` = `request.input.manifest.simulation_version.clone()`
- `provenance.tick_millis` = `request.input.config.tick_millis`
- `provenance.manifest_fingerprint` = `request.input.manifest.fingerprint()?`
- `spectate()`/`conclude()`의 `tick_millis` = `resolution.execution.provenance.as_ref().map(|p| p.tick_millis).filter(|m| *m > 0).ok_or(MissingProvenance)?`
- `CombatSpectatorView.tick_millis`와 `CombatConclusionReport.duration_millis` 계산식은 **그대로** 유지한다. 값의 출처만 바뀐다.

## Work packages (순서 고정, WP당 커밋 1개)

### WP-1 — `CombatProvenance` 정의와 채우기
`combat_execution.rs`에 struct 정의, `execute()`가 `Some(...)`으로 채움, `lib.rs` re-export.
검증: `cargo fmt --all -- --check`, `cargo test --workspace --no-fail-fast` → **283 유지** (숫자가 줄면 멈추고 보고).

### WP-2 — provenance 회귀 테스트 (`combat_execution_wave2.rs`에 추가)
1. `execute()` 결과의 provenance가 입력 manifest의 `simulation_version`, config의 `tick_millis`, manifest fingerprint와 일치
2. `Forecast` 모드에서도 provenance가 동일하다 (seed만 파생되고 provenance는 입력 그대로)
3. provenance 필드가 없는 JSON을 `CombatExecutionResult`로 역직렬화하면 `None`이고 에러가 아니다
4. 같은 입력 두 번 → 동일 provenance

### WP-3 — `spectate()`가 provenance를 쓰게 변경 (테스트 red 먼저)
`CombatSpectatorRequest.tick_millis` 제거, `MissingProvenance` 도입, 테스트에서 `SIM_TICK_MILLIS`를 request가 아니라 시뮬레이션 config에서만 쓰도록 정리. `view_reports_the_requested_tick_millis`는 `view_reports_the_tick_millis_from_provenance`로 의미를 갱신하고, `zero_tick_millis_is_rejected`는 **provenance를 지운 resolution으로 `MissingProvenance`를 검증**하도록 바꾼다.
red를 먼저 확인하고 출력을 보고서에 남긴다.

### WP-4 — `conclude()`가 provenance를 쓰게 변경 (테스트 red 먼저)
WP-3과 같은 방식. `CombatConclusionRequest.tick_millis` 제거, `MissingProvenance` 도입, 기존 테스트 리터럴에서 `tick_millis` 줄 제거. `duration_millis` 검증 테스트는 유지하되 값의 출처가 provenance임을 반영한다.

### WP-5 — 문서 갱신 (생략 금지)
- `docs/design/Combat_System_Implementation_Plan_Index.md`
  - `status:` → `wave2-step6-complete`
  - 단계 표에 `fable_combat_wave2_step6_2608020838.md` 행 추가 (구현 단위: "전투 기록 provenance(simulation version·tick 길이·manifest fingerprint)와 중복 tick_millis 파라미터 제거", non-goal: "renderer 노출·밸런스 확정값").
  - **Step 1c의 "선결 과제(fingerprint 안정성)" 항목을 해소로 바꾼다.** 정본 03 인용과 함께 다음을 적는다: 결정성은 같은 simulation version 안에서만 보장되므로 schema 추가로 fingerprint 값이 바뀌는 것은 계약 위반이 아니다. 대신 기록이 `simulation_version`을 저장해야 하며 그것을 이 slice가 구현했다. **fingerprint를 비교하는 consumer는 반드시 `simulation_version`도 함께 비교해야 한다**를 계약으로 명시한다.
  - "현재 코드와 정본의 경계"에 provenance 확보분을 적는다.
- `docs/dev/Combat_System_Operating_Guide.md`, `docs/dev/Combat_System_Goal_Prompt.md`에 기존 형식대로 한 줄씩 추가한다.
- 문서에 수치를 적을 때는 **그 수치를 고정하는 테스트 함수명을 같이 적는다.** 이미 적힌 수치가 stale해졌으면 갱신한다.
- 각 문서 100KB 이하 유지.

## 명시적 범위 밖

- `ScenePage` 필드 추가, WASM 노출 → Wave 3 Step 1c
- terminal/Web 렌더러 → Wave 3 Step 1d
- 치유량·명줄·패배 결과 스키마
- 고급 다수전 AI·조기 결착·증원
- simulation version을 실제로 bump하는 정책 (콘텐츠/릴리스 결정 사항)
- 재시도 고정층/변경층 스키마 (정본 03의 재시도 manifest 2층 구조) — 별도 slice
- 밸런스 확정 수치

## 검증 명령

```bash
cargo fmt --all -- --check
cargo test -p escape-core --test combat_execution_wave2
cargo test -p escape-core --test combat_resolution_wave2
cargo test -p escape-core --test combat_conclusion_wave2
cargo test -p escape-core --test combat_spectator_wave3
cargo test --workspace --no-fail-fast
git diff --check
```

기준: 283 passed + 신규 증가분 / 0 failed. 기존 combat 테스트 90개가 개수 그대로 통과해야 한다 (`tick_millis` 관련 2개 테스트는 의미가 바뀌므로 이름·본문 갱신을 허용하며, 개수는 유지된다).

## 최종 체크리스트

- [ ] WP-1 직후 workspace 283 유지 확인
- [ ] `CombatSpectatorRequest`/`CombatConclusionRequest`에 `tick_millis`가 더 이상 없다
- [ ] `InvalidTickMillis` 변형이 두 에러 enum에서 사라지고 `MissingProvenance`로 대체됐다
- [ ] provenance 없는 결과에 0을 쓰지 않고 에러를 낸다
- [ ] `execute()`의 `effective_seed`/`namespace` 결정 로직 무변경
- [ ] fingerprint 계산 호출부 무변경
- [ ] RNG 추가 호출 0회, `HashMap` 순회 의존 없음
- [ ] WP-3·WP-4 red 출력 기록
- [ ] `cargo fmt --all -- --check`, `git diff --check` 통과
- [ ] `cargo test --workspace --no-fail-fast` 0 failed
- [ ] WP-5 문서 3개 갱신, Step 1c 선결 과제가 **해소**로 기록됨
- [ ] `cli_smoke.rs`·`.claude/worktrees/`·fixture/generated JSON 무변경
- [ ] 보고서 `fable_combat_wave2_step6_report.md` 작성
- [ ] **보고서/커밋 메시지에 backtick 있는 마크다운을 셸 heredoc으로 넣지 말 것** — Write 툴로 쓰고 `git commit -F <파일>`로 넘긴다

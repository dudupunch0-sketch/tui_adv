# Combat Wave 3 Step 1b — 전투 종료 보고서 수치 확장

작성: 2026-08-02
작성자: Fable (orchestrator plan)
구현 담당: coding subagent (sonnet, effort medium)

## Baseline

- 기준 브랜치: `claude/combat-wave3-step1a-v2` (`862afc4`), `origin/main` = `3bb8ad5`보다 16 커밋 앞
- Baseline 검증 상태: `cargo test --workspace --no-fail-fast` → **273 passed / 0 failed** (2026-08-02 WSL 실측)
- combat 테스트 현황: `crates/escape-core/tests/combat_*.rs` 8파일 / 80 테스트

## 정본 근거

- [13. 감독형 관전·전략 피드백 시스템](https://app.notion.com/p/3a937e69695e81daa01df6f79823c4d6) — 보고서 정본
- [07. UI·템포·리스크](https://app.notion.com/p/36f37e69695e81258c60fc669f4d6800) — 같은 항목을 재확인
- [10. 패배·명줄·재도전 시스템](https://app.notion.com/p/3a837e69695e81338475fe832ee43826) — 명줄의 소유 위치 판단 근거

정본 13이 요구하는 보고서:

> 기본 항목: 승패, 전투 시간, 생존/전투불능, 최대 피해 가한 캐릭터, 최대 피해 받은 캐릭터, 최대 치유량 캐릭터, 명줄 소모/패배 결과.
> 발생하지 않은 항목은 숨긴다.
> 캐릭터별 상세 기록: 입힌 생명력 피해, 받은 생명력 피해, 치유량, 처치 수, 전투불능 여부.

정본 13이 **금지**하는 것 (구현하면 위반):

> 전략 수행 평가, 핵심 전환점, 자동 원인 분석, 전략 조언, 종합 MVP와 이전 전투 결과 자동 비교는 제공하지 않는다. 원인은 체스판과 로그로 플레이어가 판단한다.

## Scope 판정 (2026-08-02 코드 실측)

| 정본 항목 | 현재 | 이 slice |
|---|---|---|
| 승패, 사유 | `CombatConclusionReport.outcome`/`reason` | 이미 있음 |
| 생존/전투불능 | `survivor_ids`/`defeated_ids` | 이미 있음 |
| 전투 시간 | 없음 | **추가** |
| 캐릭터별 입힌 피해 | 없음 | **추가** |
| 캐릭터별 받은 피해 | 없음 | **추가** |
| 캐릭터별 처치 수 | 없음 | **추가** |
| 캐릭터별 전투불능 여부 | `defeated_ids`로 간접 | **캐릭터별 행에 명시 추가** |
| 최대 피해 가한/받은 캐릭터 | 없음 | **추가** (발생 안 하면 `None`) |
| **치유량 / 최대 치유량 캐릭터** | **파이프라인에 healing 개념 자체가 없음** | **범위 밖** |
| **명줄 소모 / 패배 결과** | 개념 없음 | **범위 밖** |

### 치유량을 넣지 않는 이유

`combat_resolution.rs:293`의 체력 갱신은 **감소 전용**이다. `CombatAttackDefinition`에도 `CombatEffectDefinition`에도 회복을 표현하는 필드가 없고, `grep -niE "heal|치유|회복량"`가 combat 모듈에서 회복 로직을 하나도 찾지 못한다. 합산할 원천이 없으므로 `healing_hundredths: 0`을 보고하면 값이 있는 척하는 거짓값이 된다 (Wave 3 Step 1a의 `tick_millis: 0`과 같은 실패 유형). **필드를 만들지 않는다.** healing 파이프라인이 생기는 slice에서 같은 자리에 additive로 추가한다.

### 명줄을 넣지 않는 이유

정본 10: "명줄은 치명적 실패를 감당하는 **런 단위 메타 자원**이다. 두 번째 생명력 수치가 아니며 공격 대상·피해·AI·전투 전 예측에 관여하지 않는다." 소모 여부는 각 인카운터의 **패배 결과 정의**가 소유하고("모든 인카운터는 패배 결과를 정의해야 한다"), 주인공 전투불능이 기본 손실이지만 목표 실패·적 도주·호위 실패는 기본적으로 손실이 아니다. 즉 명줄 소모는 전투 tick resolver가 아니라 그 위 레이어의 판정이다. 코드에는 패배 결과 스키마도 명줄 상태도 없다(`grep -niE "life_thread|명줄|lifeline"` → 0건). **전투 resolver가 발명하지 않는다.** 별도 slice(패배 결과 + 명줄 상태)에서 다루고, 보고서는 그때 additive로 받는다.

## Hard invariants (위반 금지)

1. **판정 재계산 금지**: 보고서는 `request.resolution`에 이미 있는 `frames[].outcomes`와 `frames[].combatants` 스냅샷만 집계한다. RNG 호출 금지, 피해·명중 재계산 금지.
2. **금지 항목 미구현**: 전략 평가·핵심 전환점·자동 원인 분석·전략 조언·종합 MVP·이전 전투 비교 필드나 함수를 만들지 않는다. "최대 피해 가한 캐릭터"는 정본이 명시한 기본 항목이므로 MVP가 아니다.
3. **발생하지 않은 항목은 숨긴다**: 하이라이트는 `Option`으로 두고, 해당 사건이 0건이면 `None`이다. `Some`에 0을 담지 않는다.
4. **additive-optional 직렬화**: 새 필드는 `#[serde(default)]`. schema version을 올리지 않는다.
5. **결정론**: `BTreeMap`/정렬된 `Vec`만. `HashMap` 순회 의존 금지. 동점 tie-break는 **id 오름차순 최소값**으로 고정한다.
6. **`combat_conclusion.rs`의 기존 outcome/reason 판정 로직을 바꾸지 않는다.** 집계만 추가한다.
7. **fingerprint**: `CombatConclusionReport.fingerprint` 계산 방식(기존 호출부)을 바꾸지 않는다. 새 필드가 값에 섞이는 것은 허용한다 — Wave 2 Step 5에서 이미 확인된 성질이며, Step 1c 선결 과제로 인덱스에 기록돼 있다. 다만 **기존 combat 테스트 개수와 통과 여부가 유지되는지** 확인하고, 깨지면 멈추고 보고하라.
8. **신규 의존성 금지**, `Cargo.toml` 수정 금지.
9. **renderer 미접촉**: `ScenePage`·`escape-wasm`·`escape-terminal`·`web/src` 건드리지 않는다 (Step 1c/1d 소관).
10. **다른 작업자 변경 보존**: `crates/escape-terminal/tests/cli_smoke.rs` 읽기만. `.claude/worktrees/` 읽지도 쓰지도 않는다.
11. **콘텐츠 데이터 불변**: `crates/escape-core/fixtures/**`, `web/src/data/generated/**` 수정 금지.

## 예상 변경 파일 (이 목록 밖은 손대지 말 것)

| 파일 | 성격 |
|---|---|
| `crates/escape-core/src/combat_conclusion.rs` | 필드 추가 + 집계 |
| `crates/escape-core/src/lib.rs` | 새 struct re-export 추가만 |
| `crates/escape-core/tests/combat_conclusion_wave2.rs` | 기존 4개 테스트의 `CombatConclusionRequest` 리터럴에 `tick_millis` 추가 + 신규 테스트 |
| `docs/design/Combat_System_Implementation_Plan_Index.md` | 단계 표·경계 갱신 |
| `docs/dev/Combat_System_Operating_Guide.md` | 단계 기록 |
| `docs/dev/Combat_System_Goal_Prompt.md` | 완료 목록 |

## 공개 API 변경

```rust
// combat_conclusion.rs

pub struct CombatConclusionRequest {
    pub resolution: CombatResolutionResult,
    pub participants: Vec<CombatSimulationParticipant>,
    pub policy: CombatTerminationPolicy,
    /// tick 한 칸의 길이(ms). `CombatResolutionResult`가 입력 `CombatSimulationConfig`를
    /// 보관하지 않으므로 호출자가 전달한다. 0은 `InvalidTickMillis`로 거부한다.
    /// (Wave 3 Step 1a의 `CombatSpectatorRequest.tick_millis`와 같은 이유·같은 규칙)
    pub tick_millis: u32,
}

/// 캐릭터 한 명의 전투 기록. 정본 13의 "캐릭터별 상세 기록".
/// 치유량은 파이프라인에 회복 개념이 없어 아직 필드를 두지 않는다.
pub struct CombatCombatantReport {
    pub id: String,
    pub damage_dealt_hundredths: i64,
    pub damage_taken_hundredths: i64,
    pub kills: u32,
    pub incapacitated: bool,
}

pub struct CombatConclusionReport {
    // ... 기존 필드 전부 유지 ...
    /// 결착까지의 전투 시간. tick 수 × tick_millis.
    #[serde(default)]
    pub duration_millis: u64,
    /// id 오름차순.
    #[serde(default)]
    pub combatants: Vec<CombatCombatantReport>,
    /// 피해가 하나도 없으면 `None` (발생하지 않은 항목은 숨긴다).
    #[serde(default)]
    pub top_damage_dealt_id: Option<String>,
    #[serde(default)]
    pub top_damage_taken_id: Option<String>,
    pub fingerprint: String,
}

pub enum CombatConclusionError {
    // ... 기존 변형 유지 ...
    InvalidTickMillis(u32),
}
```

`lib.rs`의 `combat_conclusion` re-export 목록에 `CombatCombatantReport`를 추가한다.

## 집계 규칙 (결정론적)

입력은 `request.resolution.frames`다.

- `damage_dealt_hundredths[actor]` = 모든 tick의 모든 outcome에서 `hit && damage_hundredths > 0`인 것의 `damage_hundredths` 합 (actor 기준)
- `damage_taken_hundredths[target]` = 같은 outcome 집합을 target 기준으로 합
- `kills[actor]`: 어떤 전투원 T의 `frames[i].combatants`에서 `current_health_hundredths <= 0`이 **처음** 관측된 tick i를 찾고, 그 tick의 outcome 중 `target_id == T && hit && damage_hundredths > 0`인 **마지막** outcome의 `actor_id`에게 1을 준다. 그런 outcome이 없으면(예: 스냅샷이 비어 있거나 피해 없이 0이 된 경우) 아무에게도 주지 않는다.
- `incapacitated[id]` = 마지막 frame 스냅샷의 `current_health_hundredths <= 0`. 스냅샷이 비어 있으면 `request.resolution.state.combatants`로 대체한다.
- `combatants` 목록에는 `request.participants`의 모든 id를 넣는다 (피해가 0이어도 행은 존재한다 — 이건 "발생하지 않은 항목"이 아니라 캐릭터별 상세 표의 행이다). id 오름차순.
- `top_damage_dealt_id`: `damage_dealt_hundredths`가 최대인 id. 최대값이 0이면 `None`. 동점이면 id 오름차순 최소값.
- `top_damage_taken_id`: 같은 규칙.
- `duration_millis` = `decisive_tick`이 `Some(t)`면 `(t as u64 + 1) * tick_millis as u64`, `None`이면 `frames.len() as u64 * tick_millis as u64`. `checked_mul`을 쓰고 넘치면 기존 `CombatConclusionError`의 적절한 변형(없으면 `InvalidTickMillis`가 아니라 새 변형 없이 `u64` 범위로 충분함을 확인)으로 처리한다. `u32 * u32 → u64`는 넘치지 않으므로 단순 캐스팅으로 충분하다.

## Work packages (순서 고정, WP당 커밋 1개)

### WP-1 — `tick_millis` 입력과 `duration_millis`
`CombatConclusionRequest.tick_millis` 추가, `0` 거부, `duration_millis` 계산. 기존 테스트 4개의 request 리터럴에 `tick_millis` 추가 (본문 로직은 수정 금지).
검증: `cargo fmt --all -- --check`, `cargo test -p escape-core --test combat_conclusion_wave2`, `cargo test --workspace --no-fail-fast`.

### WP-2 — 캐릭터별 집계
`CombatCombatantReport`와 `combatants` 필드, 피해 합산·`incapacitated`. `kills`는 아직 0.

### WP-3 — 처치 수
위 `kills` 규칙 구현.

### WP-4 — 하이라이트 2개
`top_damage_dealt_id`/`top_damage_taken_id`, 0이면 `None`, 동점은 id 최소.

### WP-5 — 회귀 테스트 (`combat_conclusion_wave2.rs`에 추가)
**WP-1~4 각 WP의 테스트는 그 WP 전에 쓰고 red를 확인한 뒤 구현한다.** 최소 케이스:
1. `tick_millis: 0` → `InvalidTickMillis(0)`
2. `decisive_tick`이 있는 시나리오의 `duration_millis`가 `(t+1) * tick_millis`와 일치
3. `decisive_tick`이 없으면 `frames.len() * tick_millis`
4. `combatants`가 `participants`의 모든 id를 id 오름차순으로 포함
5. 입힌 피해·받은 피해 합이 outcome 합과 일치
6. 처치 수가 체력 0 도달 tick의 마지막 유효 타격 actor에게 귀속
7. 피해가 0건이면 두 하이라이트가 `None`
8. 동점이면 id 오름차순 최소가 선택된다
9. 같은 입력 두 번 → 동일 보고서·동일 fingerprint
10. `participants` 입력 순서를 섞어도 보고서가 동일
11. 새 필드가 없는 JSON을 `CombatConclusionReport`로 역직렬화하면 기본값이 되고 에러가 아니다

### WP-6 — 문서 갱신 (생략 금지)
- `docs/design/Combat_System_Implementation_Plan_Index.md`
  - `status:` → `wave3-step1b-complete`
  - 단계 표의 `(플랜 미작성) — Wave 3 Step 1b` 행을 `fable_combat_wave3_step1b_2608020437.md`로 교체한다.
  - "현재 코드와 정본의 경계"에서 보고서 확장 항목을 확보분으로 옮기고, **아직 없는 것으로 아래 두 줄을 남긴다**:
    - 치유량·최대 치유량 캐릭터 — combat 파이프라인에 회복 개념이 없어 보류 (healing slice 선행 필요)
    - 명줄 소모·패배 결과 — 정본 10 기준 런 단위 메타 자원이며 인카운터 패배 결과 정의가 소유. 패배 결과 스키마 slice 선행 필요
  - 정본 13이 금지한 항목(전략 평가·전환점·원인 분석·조언·MVP·이전 전투 비교)을 **의도적 미구현**으로 한 줄 명시한다.
- `docs/dev/Combat_System_Operating_Guide.md`, `docs/dev/Combat_System_Goal_Prompt.md`에 기존 형식대로 한 줄씩 추가한다.
- 문서에 수치를 적을 때는 **그 수치를 고정하는 테스트 함수명을 같이 적는다.**
- 각 문서 100KB 이하 유지.

## 명시적 범위 밖

- 치유량, 최대 치유량 캐릭터 (위 사유)
- 명줄 소모, 패배 결과, 즉시 재도전 UX (위 사유 — 정본 10의 패배 결과 스키마가 별도 slice)
- 전략 평가·핵심 전환점·자동 원인 분석·전략 조언·종합 MVP·이전 전투 비교 (정본이 금지)
- `ScenePage`/WASM 노출 → Step 1c
- terminal/Web 보고서 화면 → Step 1d
- 프리셋·우선 목표 규칙
- 밸런스 확정 수치

## 검증 명령

```bash
cargo fmt --all -- --check
cargo test -p escape-core --test combat_conclusion_wave2
cargo test -p escape-core --test combat_resolution_wave2
cargo test -p escape-core --test combat_spectator_wave3
cargo test --workspace --no-fail-fast
git diff --check
```

기준: 273 passed + 신규 증가분 / 0 failed. 기존 combat 테스트 80개가 **개수 그대로** 통과해야 한다.

## 최종 체크리스트

- [ ] 치유량·명줄 필드를 만들지 않았다
- [ ] 금지 항목(전략 평가·전환점·원인 분석·조언·MVP·이전 전투 비교) 필드/함수 없음
- [ ] 하이라이트가 0건일 때 `None` (`Some(0)` 아님)
- [ ] 동점 tie-break가 id 오름차순 최소로 고정
- [ ] RNG 추가 호출 0회, `HashMap` 순회 의존 없음
- [ ] 기존 `combat_conclusion_wave2.rs` 4개 테스트 **본문 로직** 무수정 (request 리터럴에 `tick_millis` 추가만)
- [ ] 각 WP에서 red 확인 후 구현, red 출력 기록
- [ ] `cargo fmt --all -- --check`, `git diff --check` 통과
- [ ] `cargo test --workspace --no-fail-fast` 0 failed
- [ ] WP-6 문서 3개 갱신
- [ ] `cli_smoke.rs`·`.claude/worktrees/`·fixture/generated JSON 무변경
- [ ] 보고서 `fable_combat_wave3_step1b_report.md` 작성
- [ ] **보고서/커밋 메시지에 backtick 있는 마크다운을 셸 heredoc으로 넣지 말 것** — Write 툴로 쓰고 `git commit -F <파일>`로 넘긴다 (이 세션에서 2회 유실 사고)

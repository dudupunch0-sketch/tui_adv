# T0 — simulation_version 강제 검증

plan: `fable_combat_hex_t0_step1_2608061847.md`
report: `fable_combat_hex_t0_step2_report.md`
baseline: `ab9fc7e`
상위 문서: [Combat_Hex_Rework_Development_Plan.md](docs/design/Combat_Hex_Rework_Development_Plan.md) §6 T0

## 1. 목적

정본 03은 **"결정성은 같은 simulation version 내부에서만 보장한다"**고 정했다.
그런데 코드가 이 계약을 강제하지 않는다. 저작이 선언한 `simulation_version` 문자열을
그대로 기록에 실어 보낼 뿐이라, **코드가 실제로 구현하지 않는 version을 적어도 아무도 잡지 못한다.**

이 슬라이스는 그 구멍을 막는다. T1이 육각 좌표계로 breaking change를 낼 때 안전장치가 되므로
T1보다 먼저 선다.

현재 저작에서 선언된 version은 `v2` 한 값뿐이다(3곳: preview YAML 1, 생성 번들 2 —
번들은 export 산출물이라 손으로 고치지 않는다).

## 2. 선행 조건

없다. baseline에서 바로 시작한다.

## 3. 소유 파일

수정 가능:

- `crates/escape-core/src/combat_contract.rs`
- `crates/escape-core/src/combat_simulation.rs` (검증 호출 추가만)
- `crates/escape-core/src/content.rs` (`validate_encounter_combat`에 규칙 추가만)
- `crates/escape-core/tests/combat_contract_wave1.rs` (테스트 추가)
- `crates/escape-core/tests/encounter_combat_wave3.rs` (테스트 추가)

수정 금지:

- `crates/escape-core/src/combat_resolution.rs`, `combat_conclusion.rs`, `combat_spectator.rs`,
  `combat_execution.rs`, `combat_state.rs`, `combat_opportunity.rs`, `scene_page.rs`
- `src/tui_adv/storypack-previews/**` (저작 값 `v2`는 그대로 유효하다 — 바꿀 이유가 없다)
- `crates/escape-core/fixtures/**`, `web/src/data/generated/**` (export 산출물)
- `crates/escape-terminal/**`, `web/**`
- 다른 작업자의 미추적 파일

## 4. 설계

### 4-1. 지원 version은 "목록"이 아니라 "현재 값 하나"다

한 바이너리가 두 가지 시뮬레이션 거동을 동시에 구현하는 것은 정본 03의 정신과 어긋나고
유지 비용이 크다. 따라서:

```rust
pub const CURRENT_SIMULATION_VERSION: &str = "v2";
```

이 값과 다른 version으로 **시뮬레이션을 돌리려는 시도**를 거부한다.

복수 version 동시 지원은 이 슬라이스의 명시적 범위 밖이다. 필요해지면 별도 슬라이스에서 연다.

### 4-2. 검증 지점은 "시뮬레이션 진입"이지 "역직렬화"가 아니다

**과거 기록은 계속 읽을 수 있어야 한다.** 옛 version이 박힌 저장 기록을 역직렬화하는 것과,
그 version으로 새로 시뮬레이션을 돌리는 것은 다른 일이다. 전자를 막으면 기존 저장이 깨진다.

따라서 검증은 두 곳에만 넣는다.

| 지점 | 무엇을 | 왜 |
|---|---|---|
| `validate_encounter_combat` (index-time) | 저작이 선언한 version이 `CURRENT_SIMULATION_VERSION`과 다르면 하드 오류 | 기존 11개 규칙과 같은 자리. 저작자가 즉시 피드백을 받는다 |
| `CombatSimulation::new()` (runtime) | 같은 조건으로 거부 | 저작을 거치지 않는 경로(테스트·직접 호출)까지 덮는 방어선 |

**넣지 않는 곳**: `CombatSimulationVersion::new()`, serde `Deserialize`.
`CombatSimulationVersion`은 tuple struct에 serde derive가 붙어 있어 역직렬화가 `new()`를 거치지 않는다.
이 성질을 **바꾸지 말고 그대로 둔다** — 바꾸면 과거 기록 읽기가 깨진다.

### 4-3. 오류 타입

- `CombatContractError::UnsupportedSimulationVersion(String)` 추가.
  `Display`는 받은 값과 기대 값을 모두 노출한다 (예: `unsupported simulation version 'v9' (this build implements 'v2')`).
- `CombatSimulationError`는 기존 `InvalidReference`로 뭉개지 **말고** 전용 변형을 추가한다.
  기존에 `InvalidReference`가 manifest 검증 실패를 통째로 삼키고 있어 원인 지목이 안 된다
  (같은 문제로 `ContentTurnError::CombatProducer`가 리뷰에서 전용 변형으로 교체된 선례가 있다).
- `validate_encounter_combat`의 오류 메시지에는 **인카운터 id를 포함**한다 (기존 규칙들과 동일).

## 5. Hard invariants (상속)

상위 문서 §3에서 상속한다. 이 슬라이스에서 특히 걸리는 것만 다시 적는다.

1. **결정론.** 이 슬라이스는 판정을 바꾸지 않는다. 어떤 fingerprint도 값이 변하면 안 된다.
2. **RNG 호출 0회.** 새 난수원을 도입하지 않는다.
3. **additive-optional 직렬화.** 필드를 추가·제거·개명하지 않는다. 기존 JSON 바이트가 그대로여야 한다.
4. **과거 기록 역직렬화 유지.** 임의 version 문자열이 박힌 JSON이 에러 없이 역직렬화된다.
5. **renderer 무변경.** `crates/escape-terminal`, `web/`을 건드리지 않는다.

## 6. WP 목록

순서 고정. WP당 커밋 1개. 각 WP는 검증 → 커밋 → 다음.

### WP1 — `CURRENT_SIMULATION_VERSION`과 오류 타입

`combat_contract.rs`에 상수와 `CombatContractError::UnsupportedSimulationVersion(String)`을 추가하고,
version이 상수와 일치하는지 확인하는 공개 helper 하나를 둔다. **호출부는 아직 붙이지 않는다.**

검증: `cargo test -p escape-core --test combat_contract_wave1`

### WP2 — runtime 검증 (`CombatSimulation::new`)

`CombatSimulation::new()`가 WP1의 helper로 `input.manifest.simulation_version`을 검사하고,
전용 `CombatSimulationError` 변형으로 거부한다. 기존 manifest/state 검증 호출은 건드리지 않는다.

검증:
```bash
cargo test -p escape-core --test combat_simulation_wave2
cargo test -p escape-core --test combat_execution_wave2
cargo test -p escape-core --test combat_resolution_wave2
```
기존 테스트가 전부 통과해야 한다 — 픽스처가 이미 `v2`이므로 통과가 정상이다.
**하나라도 깨지면 멈추고 보고한다** (픽스처가 다른 version을 쓰고 있다는 뜻이므로 계획을 다시 세워야 한다).

### WP3 — index-time 검증 (`validate_encounter_combat`)

기존 11개 하드 오류 규칙 옆에 12번째 규칙으로 추가한다. 오류 메시지에 인카운터 id를 포함한다.

검증: `cargo test -p escape-core --test encounter_combat_wave3`

### WP4 — 테스트

`combat_contract_wave1.rs`와 `encounter_combat_wave3.rs`에 추가한다.
**구현보다 테스트를 먼저 red로 만든 뒤 통과시키는 순서를 권장한다.**

필수 케이스:

| 테스트 | 고정하는 것 |
|---|---|
| `unsupported_simulation_version_is_rejected_at_simulation_entry` | 런타임 거부 |
| `unsupported_simulation_version_is_rejected_at_index_time` | 저작 거부. 오류 문자열에 인카운터 id 포함 |
| `current_simulation_version_is_accepted` | `v2`가 통과 |
| `error_message_names_both_the_received_and_the_expected_version` | Display 문자열에 두 값이 다 있음 |
| `unknown_version_json_still_deserializes_without_error` | **과거 기록 읽기가 안 깨짐** (invariant 4) |
| `version_enforcement_does_not_change_any_fingerprint` | 같은 입력의 manifest/simulation fingerprint가 baseline과 동일 |

검증: `cargo test --workspace --no-fail-fast`

## 7. 검증 명령 (WSL, `cd /home/dudu/work/tui-adv`)

```bash
cargo fmt --all -- --check
cargo test -p escape-core --test combat_contract_wave1
cargo test -p escape-core --test encounter_combat_wave3
cargo test --workspace --no-fail-fast
git diff --check
```

기대: baseline의 워크스페이스 테스트 수(346)에서 **감소가 없어야 한다.** 신규 테스트만큼 증가한다.

`web`은 이 슬라이스에서 변경이 없으므로 `npm test`는 필요 없다. 실행했다면 결과를 보고에 적는다.

## 8. 명시적 범위 밖

- version bump 자체 (`v2` → `v3`). **T1이 좌표계를 바꿀 때 한다.**
- 복수 version 동시 지원, version 간 마이그레이션 경로
- `CombatPresentationSpeed`의 재생 레이어 이동 — **T4로 이관했다** (실행/재생 분리는 T4가 어차피 수술하는 자리이며, 지금 건드리면 같은 곳을 두 번 연다)
- 저작 YAML·번들·픽스처의 version 값 변경
- 육각 좌표계 관련 일체
- renderer 변경

## 9. 보고 형식

`fable_combat_hex_t0_step2_report.md`에 적는다.

- WP별 커밋 해시와 한 줄 요약
- 실제로 실행한 검증 명령과 **출력 수치** (테스트 수, 실패 수)
- 스킵한 WP와 사유
- 계획과 다르게 구현한 부분과 사유 (**계획서의 서술이 실제 코드와 다르면 코드를 따르고 그 사실을 보고한다**)
- 발견했지만 이 슬라이스 범위 밖이라 손대지 않은 것

## 10. 최종 체크리스트

- [ ] `CURRENT_SIMULATION_VERSION`이 한 곳에만 정의되어 있다
- [ ] 런타임·index-time 두 지점에서 거부된다
- [ ] 오류가 `InvalidReference` 같은 뭉뚱그린 변형이 아니라 전용 변형이다
- [ ] 오류 문자열이 받은 값과 기대 값을 모두 노출한다
- [ ] index-time 오류에 인카운터 id가 들어 있다
- [ ] 임의 version JSON이 여전히 역직렬화된다
- [ ] fingerprint 값이 하나도 바뀌지 않았다
- [ ] `crates/escape-terminal`, `web/`, 저작 YAML, 번들, 픽스처 무변경
- [ ] `cargo fmt --all -- --check` 통과
- [ ] `cargo test --workspace --no-fail-fast` 0 failed, 테스트 수 감소 없음

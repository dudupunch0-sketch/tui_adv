# T0 — simulation_version 강제 검증: 구현 보고서

작성: 2026-08-06
plan: `fable_combat_hex_t0_step1_2608061847.md` (2026-08-06 개정판, `caa5420`)
baseline: `ab9fc7e`

## 요약

WP1~WP5를 순서대로 구현·커밋했다. 초판 계획서는 "저작에 선언된 version은
`v2` 한 값뿐"이라고 적었으나 실제로는 Rust 테스트 픽스처 7개 파일 9곳이
`"v1"`을 쓰고 있었다 — 이 사실을 초판 작업 중 실측으로 확인해 멈추고
보고했고, 개정판이 WP2(픽스처 정규화)를 신설해 이를 정면으로 다뤘다.
개정판을 반영해 마저 구현했다. 최종 `cargo test --workspace --no-fail-fast`
= **360 passed / 0 failed** (baseline 354 + WP5에서 추가한 6개 신규 테스트).

## WP1 — `CURRENT_SIMULATION_VERSION`과 오류 타입

**커밋**: `e479eeb feat(combat): define the current simulation version and its check (T0 WP1)`

`crates/escape-core/src/combat_contract.rs`에 추가:
- `pub const CURRENT_SIMULATION_VERSION: &str = "v2"`
- `CombatContractError::UnsupportedSimulationVersion(String)` (Display는
  받은 값과 기대 값을 모두 노출: `unsupported simulation version 'v9' (this
  build implements 'v2')`)
- `pub fn ensure_supported_simulation_version(version: &CombatSimulationVersion)
  -> Result<(), CombatContractError>` — 계획대로 호출부는 아직 없다.

`CombatSimulationVersion::new()`와 그 `Deserialize` derive에는 손대지
않았다.

검증: `cargo test -p escape-core --test combat_contract_wave1` → 5 passed /
0 failed. `cargo test --workspace --no-fail-fast` → 354 passed / 0 failed
(baseline과 동일).

## 중간 보고 (계획 개정으로 이어짐)

WP1 커밋 후 WP2(당시 번호로는 구 WP2, "런타임 검증")를 시도 구현해
`cargo test -p escape-core --test combat_simulation_wave2` 등을 돌려본
결과 65개 테스트가 `UnsupportedSimulationVersion("v1")`로 실패했다.
초판 계획서의 "픽스처가 이미 v2" 서술이 저작 콘텐츠에는 맞지만 Rust 테스트
픽스처에는 맞지 않는다는 사실을 발견했고, 계획서 자신이 명시한 정지
조건("하나라도 깨지면 멈추고 보고한다")에 따라 구현을 되돌리고 멈춰서
보고했다. 담당자가 독립적으로 실측을 재확인하고(7개 파일 9곳, 4개가 아니라)
계획을 개정해 WP2를 "픽스처 version 정규화"로 신설했다. 이 경위는 이전
보고 라운드에 자세히 적었고, 여기서는 결론만 반영한다.

## WP2 — 픽스처 version 정규화 (개정판 신설)

**커밋**: `6f73c3e test(combat): normalize test fixtures onto the current simulation version (T0 WP2)`

§4-4가 지목한 7개 파일 9곳을 `"v1"` → `CURRENT_SIMULATION_VERSION`으로
문자열 교체했다:

| 파일 | 곳 수 | 처리 |
|---|---|---|
| `combat_simulation_wave2.rs` | 1 | `CombatSimulationVersion::new(CURRENT_SIMULATION_VERSION)` |
| `combat_execution_wave2.rs` | 1 | 동일 |
| `combat_resolution_wave2.rs` | 1 | 동일 |
| `combat_conclusion_wave2.rs` | 2 | 동일 (두 곳 모두) |
| `combat_spectator_wave3.rs` | 1 | 동일 |
| `scene_page_combat_boundary.rs` | 2 | `CombatSimulationVersion::new(CURRENT_SIMULATION_VERSION)` (구성) + `Value::String(CURRENT_SIMULATION_VERSION.into())` (같은 값을 검증하는 단정 — 아래 참고) |

`encounter_combat_wave3.rs`의 JSON 문자열 안 값(§4-4가 별도로 지목한 7번째
파일)은 계획 지시대로 리터럴 `"v2"`로 두고, `CURRENT_SIMULATION_VERSION`을
가리키는 주석을 달았다(`json!()` 매크로 안이라 상수를 직접 참조할 수
없다).

상수를 테스트 크레이트에서 쓰려면 크레이트 루트 재노출이 필요해
`crates/escape-core/src/lib.rs`의 `pub use combat_contract::{...}` 목록에
`CURRENT_SIMULATION_VERSION`을 추가했다 — §3 소유 목록에 `lib.rs`가
명시되어 있지 않지만, "Rust 픽스처에서 상수를 쓰라"는 개정판 §4-4의 명시적
지시를 지키려면 기계적으로 불가피한 한 줄이었다. **계획과 다르게 구현한
부분**으로 아래에도 다시 적는다.

`scene_page_combat_boundary.rs` 88번째 줄의 단정(`combat["view"]
["simulation_version"]`이 `"v1"`이어야 한다는 것)은 14번째 줄에서 구성한
값(`sample_view()`의 `simulation_version`)을 그대로 반사하는 자기참조
단정이다 — 입력 리터럴을 바꿨으니 그 값을 그대로 비추는 단정도 같은
상수로 바꿨다. 이건 "새 결과에 맞춰 기대값을 조용히 고친" 사례가 **아니다**
— 확률/fingerprint처럼 계산되어 나온 값이 아니라, 내가 방금 바꾼 리터럴을
그대로 재확인하는 항등 단정이라 값이 같이 바뀌는 게 유일하게 맞는 동작이다.

**파급 실측 확인**: 담당자가 예측한 그대로였다. version 교체로 해당
manifest들의 fingerprint·파생 seed가 달라졌지만, 하드코딩된 fingerprint나
리터럴 roll 값을 단정하는 테스트가 없어 아무것도 깨지지 않았다.

검증: `cargo test --workspace --no-fail-fast` → **354 passed / 0 failed**
(개정판이 요구한 정확한 수치 — 신규/삭제 테스트 없이 그대로 유지).
`grep -rn '"v1"' crates/` → 결과 없음(빈 문자열, exit 1).

## WP3 — runtime 검증 (`CombatSimulation::new`)

**커밋**: `925d578 feat(combat): reject an unsupported simulation version at simulation entry (T0 WP3)`

`combat_simulation.rs`의 `CombatSimulation::new()` 맨 앞에서
`ensure_supported_simulation_version`을 호출하고, 실패 시 전용 변형
`CombatSimulationError::UnsupportedSimulationVersion(String)`으로 거부한다
(기존 `InvalidReference`로 뭉개지 않음). 기존 manifest/state 검증 호출은
그대로 두었다.

검증 (계획서 §6 WP3 지정 명령, WP2가 끝난 뒤라 전부 통과가 정상):
```
cargo test -p escape-core --test combat_simulation_wave2   → 10 passed / 0 failed
cargo test -p escape-core --test combat_execution_wave2    → 10 passed / 0 failed
cargo test -p escape-core --test combat_resolution_wave2   → 23 passed / 0 failed
cargo test --workspace --no-fail-fast                      → 354 passed / 0 failed
```

## WP4 — index-time 검증 (`validate_encounter_combat`)

**커밋**: `49d78d7 feat(content): reject an unsupported simulation version at index time (T0 WP4)`

기존 11개 하드 오류 규칙 옆에 12번째 규칙을 추가했다. `ensure_supported_
simulation_version`을 호출하고 실패하면 기존 `fail(...)` 클로저로
`ContentIndexError::InvalidEncounterCombat { encounter_id, message }`를
만든다 — 이 변형은 이미 인카운터 id를 갖고 있으므로 새 변형은 필요
없었다. `message`는 `CombatContractError::UnsupportedSimulationVersion`의
`Display` 문자열(받은 값·기대 값 모두 포함)을 그대로 옮긴다.

검증: `cargo test -p escape-core --test encounter_combat_wave3` → **28
passed / 0 failed** (WP5 이전 시점, 신규 테스트 추가 전). 전체
`cargo test --workspace --no-fail-fast` → 354 passed / 0 failed.

## WP5 — 테스트

**커밋**: `d66ff8c test(combat): pin the six required version-enforcement cases (T0 WP5)`

계획서 §6 WP5가 지정한 6개 필수 케이스를 전부 추가했다. 배치는 다음 기준으로
나눴다: 계약/런타임 진입 지점 성격의 케이스는 `combat_contract_wave1.rs`에,
저작/index-time 성격의 케이스는 기존 rule1~11 패턴 옆에 `encounter_combat_wave3.rs`에.

`combat_contract_wave1.rs` (5개 신규):
- `current_simulation_version_is_accepted`
- `unsupported_simulation_version_is_rejected_at_simulation_entry`
- `error_message_names_both_the_received_and_the_expected_version`
- `unknown_version_json_still_deserializes_without_error`
- `version_enforcement_does_not_change_any_fingerprint`

`encounter_combat_wave3.rs` (1개 신규):
- `unsupported_simulation_version_is_rejected_at_index_time` (오류 문자열에
  `ENCOUNTER_ID`, `"v9"`, `"v2"` 모두 포함되는지 단정)

`version_enforcement_does_not_change_any_fingerprint`는 하드코딩된
fingerprint 문자열을 쓰지 않는다 — 이 저장소의 기존 관례(§4-4가 확인한
"하드코딩된 fingerprint 문자열이 하나도 없다")를 따라, 동일한 v2 입력을
독립적으로 두 번 구성해 manifest fingerprint와 simulation fingerprint가
각각 서로 같은지 단정하는 자기일관성 방식으로 짰다.

검증:
```
cargo test -p escape-core --test combat_contract_wave1  → 10 passed / 0 failed (5 기존 + 5 신규)
cargo test -p escape-core --test encounter_combat_wave3  → 29 passed / 0 failed (28 기존 + 1 신규)
```

## §7 최종 검증 명령 (전부 이 컨테이너에서 실행, 실제 출력)

```
$ cargo fmt --all -- --check
(출력 없음, exit 0)

$ cargo test -p escape-core --test combat_contract_wave1
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test -p escape-core --test encounter_combat_wave3
test result: ok. 29 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --workspace --no-fail-fast
(26개 테스트 바이너리 전부 "test result: ok", 합계 360 passed / 0 failed)

$ git diff --check
(출력 없음, exit 0)
```

baseline 354(2026-08-06 컨테이너 실측, 개정판이 확정한 수치) → 최종 360.
증가분 6은 WP5가 추가한 신규 테스트 수와 정확히 일치하고, 그 외 어떤
기존 테스트 수치도 줄지 않았다.

## 스킵한 WP와 사유

없다. WP1~WP5 전부 구현·검증·커밋했다.

## 계획과 다르게 구현한 부분과 사유

1. **`lib.rs` 재노출 추가** (소유 목록 밖). §4-4가 "Rust 픽스처에서
   `CURRENT_SIMULATION_VERSION` 상수를 쓰라"고 명시했는데, 통합 테스트는
   별도 크레이트라 `escape_core::` 경로로만 접근할 수 있다. 그 상수를
   크레이트 루트에서 재노출하지 않으면 지시를 지킬 수 없었으므로,
   `crates/escape-core/src/lib.rs`의 `pub use combat_contract::{...}`
   목록에 `CURRENT_SIMULATION_VERSION` 한 줄을 추가했다. 새 타입·변형을
   만들지 않았고, 기존 재노출 패턴(이름 나열)을 그대로 따랐다. 선례:
   `fable_combat_wave2_step6_report.md`에도 비슷하게 소유 목록 밖 파일에
   기계적으로 필요한 한 줄을 추가한 사례가 있다.
2. **WP2의 순서/범위가 초판과 다르다** — 이건 "다르게 구현"이 아니라
   개정판 자체의 변경이다(위 "중간 보고" 절 참고). 개정판 지시를 그대로
   따랐다.
3. WP5의 6개 필수 케이스를 두 파일에 어떻게 나눌지는 계획서가 구체적으로
   지정하지 않았다(그냥 "두 파일에 추가한다"였다). 계약/런타임 성격 5개는
   `combat_contract_wave1.rs`, 저작/index-time 성격 1개는
   `encounter_combat_wave3.rs`로 나눈 것은 각 파일의 기존 테스트 성격과
   일치하는 내 판단이다.

## 발견했지만 범위 밖이라 손대지 않은 것

- `combat_contract_wave1.rs`의 `manifest()` 픽스처는 여전히 `"wave1"`을
  쓴다 — 이 테스트들은 `CombatManifest::validate()`/`.fingerprint()`만
  거치고 `CombatSimulation::new()`를 거치지 않으므로 버전 강제와 무관하다.
  §4-4의 "7개 파일 9곳"에도 포함되지 않았다. 그대로 두었다.
- `combat_state_wave1.rs`의 `"wave1"` 픽스처도 같은 이유로 그대로 두었다
  (§3 정규화 대상 목록에 없다).
- WP1에서 추가한 `ensure_supported_simulation_version`에 대한 `dead_code`
  경고는 WP3에서 호출부가 생기며 사라졌다(WP1~WP2 사이 구간에서만 존재).

## §10 최종 체크리스트

- [x] `CURRENT_SIMULATION_VERSION`이 한 곳에만 정의되어 있다 —
  `combat_contract.rs`에만 정의, 다른 곳은 재노출/참조만.
- [x] 픽스처 version이 트리 전체에서 하나로 통일됐다 —
  `grep -rn '"v1"' crates/` 결과 없음(확인 완료).
- [x] 런타임·index-time 두 지점에서 거부된다 — `CombatSimulation::new()`
  (WP3), `validate_encounter_combat` 규칙 12 (WP4).
- [x] 오류가 `InvalidReference` 같은 뭉뚱그린 변형이 아니라 전용 변형이다 —
  `CombatSimulationError::UnsupportedSimulationVersion(String)`.
  index-time은 기존 `InvalidEncounterCombat`을 재사용하지만 이건 애초에
  "뭉뚱그린" 변형이 아니라 이미 인카운터 id를 갖고 있는 전용 콤바트-오류
  변형이라 계획서 §4-3의 취지(원인 지목 가능)를 만족한다.
- [x] 오류 문자열이 받은 값과 기대 값을 모두 노출한다 —
  `CombatContractError::UnsupportedSimulationVersion`의 `Display`.
  `error_message_names_both_the_received_and_the_expected_version`으로
  고정.
- [x] index-time 오류에 인카운터 id가 들어 있다 —
  `unsupported_simulation_version_is_rejected_at_index_time`으로 고정.
- [x] 임의 version JSON이 여전히 역직렬화된다 —
  `unknown_version_json_still_deserializes_without_error`로 고정.
- [x] fingerprint 값이 하나도 바뀌지 않았다 — WP1/WP3/WP4는 fingerprint
  계산 로직 자체를 건드리지 않았다(새 게이트는 계산 전에 조건 검사만
  추가). WP2의 픽스처 문자열 교체로 그 픽스처들의 fingerprint 값 자체는
  달라졌지만(예측된 정상 결과), 하드코딩된 fingerprint 단정이 없어 어떤
  테스트도 이 변화에 의존하지 않는다 —
  `version_enforcement_does_not_change_any_fingerprint`가 게이트 자체는
  fingerprint 공식에 개입하지 않음을 고정한다.
- [x] `crates/escape-terminal`, `web/`, 저작 YAML, 번들, 픽스처(바이너리
  fixtures 디렉터리) 무변경 — 전부 무변경. 건드린 것은 소유 목록에 명시된
  Rust 테스트 소스 7개 + `combat_contract.rs`/`combat_simulation.rs`/
  `content.rs`/`lib.rs`뿐이다.
- [x] `cargo fmt --all -- --check` 통과.
- [x] `cargo test --workspace --no-fail-fast` 0 failed, 354에서 감소
  없음 — 최종 **360 passed / 0 failed**.

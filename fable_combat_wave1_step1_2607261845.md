# 전투 시스템 Wave 1 / Step 1 — 결정론 계약 기반

status: implementation-verified
소유 agent: `coding_implementer` (5.6 luna, medium)
기준: Notion 전투 시스템 00, 01, 03, 07, 13

## 목적

현재 게임의 encounter/action truth를 깨지 않으면서, 이후 전투 resolver가 공유할 최소한의 결정론 계약을 `escape-core`에 추가한다.

이번 단계의 산출물은 **전투를 실행하지 않는 계약 계층**이다. 같은 manifest 입력, seed, simulation version을 stable serialization/fingerprint로 식별할 수 있어야 하며, 예측 RNG와 실제 전투 RNG가 이름 수준에서 분리되어야 한다.

## 소유 파일

필수:

- `crates/escape-core/src/combat_contract.rs` (신규)
- `crates/escape-core/src/lib.rs` (public export만)
- `crates/escape-core/tests/combat_contract_wave1.rs` (신규)

필요할 때만:

- `docs/dev/Data_Schema.md`의 새 public JSON contract 설명
- `docs/dev/Development_Plan.md`의 전투 트랙 링크 한 줄

이 단계에서 `state.rs`, `turn.rs`, `scene_page.rs`, WASM, terminal, Web, YAML bundle은 수정하지 않는다.

## 구현 계약

### 1. Simulation version

- `CombatSimulationVersion(String)` 또는 동등한 검증 가능한 newtype를 둔다.
- 빈 문자열과 공백만인 값은 거부한다.
- 버전 비교/증가 정책은 이번 단계에서 만들지 않는다. 결정성 보장은 동일 version 내부에서만 유효하다.

### 2. RNG namespace

Notion 03의 다섯 namespace를 enum으로 고정한다.

- `StoryResolution`
- `EncounterComposition`
- `ActualCombat`
- `ForecastEnsemble`
- `CosmeticPresentation`

각 namespace는 stable 문자열 표현과 분리된 derived seed 입력을 제공해야 한다. `ActualCombat`과 `ForecastEnsemble`은 같은 base seed를 받아도 namespace가 다르면 같은 derived seed가 나오지 않아야 한다. 전역 RNG 상태나 wall-clock을 사용하지 않는다.

### 3. Manifest 최소 구조

`CombatManifest`는 다음 정보를 명시적으로 보존한다.

- `simulation_version`
- 실제 전투 `actual_seed`
- 원본 월드 상태의 opaque/stable snapshot 또는 fingerprint
- 적용된 효과와 제외(suppressed)된 효과의 stable ID·사유
- 전투원/배치/환경/조건부 규칙의 stable ID 목록
- 플레이어에게 공개된 정보 목록

세부 전투원 상태, effect catalog 필드, 위치 좌표, 기술 수치의 최종 schema는 후속 단계가 소유한다. Step 1에서는 확장 가능한 구조(`serde` + `BTreeMap`/순서 보장 컬렉션)를 사용하고, 임의의 피해·회복·쿨타임 상수를 넣지 않는다.

### 4. Fingerprint

- manifest를 canonical JSON 또는 동등한 안정 serialization으로 직렬화한다.
- `BTreeMap`/정렬된 stable ID를 사용해 입력 순서만 바뀌어도 fingerprint가 변하지 않게 한다.
- 같은 manifest·namespace·seed·version은 같은 fingerprint/derived seed를 내야 한다.
- hash 알고리즘은 프로젝트 의존성을 새로 추가하지 않는 범위에서 구현한다. 기존 deterministic hash helper가 있으면 재사용하고, 없으면 테스트 가능한 작은 pure function으로 둔다.

## 검증 기준

필수 테스트:

1. 동일 manifest와 동일 `simulation_version`이 동일 fingerprint를 만든다.
2. manifest의 입력 순서가 canonical 정렬 후 동일하면 fingerprint가 동일하다.
3. namespace가 다르면 derived seed가 분리된다.
4. 빈 simulation version, 빈 stable ID, 잘못된 필수 필드가 명시적으로 실패한다.
5. serde JSON round-trip 후 manifest와 fingerprint가 보존된다.
6. 기존 `cargo test --workspace`에 회귀가 없다.

권장 명령:

```bash
cargo test -p escape-core --test combat_contract_wave1
cargo test --workspace --no-fail-fast
```

## 금지 사항

- 실제 tick, 캐릭터 이동, AI 행동, 전투 판정, 승패 resolver를 추가하지 않는다.
- `GameState` save schema를 이번 단계에서 변경하지 않는다.
- Web/terminal에 전투 UI를 추가하거나 renderer-local 판정을 넣지 않는다.
- 기술 비용, 호흡 회복률, 피해량, 방어 계수, 승률, 개입 상한을 코드 상수로 임의 확정하지 않는다.
- Notion에 정의되지 않은 `CombatState` 세부 필드를 이번 단계의 이름으로 추정해 확정하지 않는다.

## 보고 형식

subagent는 다음을 보고한다.

- 변경 파일 목록
- public type/API 요약
- 각 테스트 명령과 실제 결과
- Notion 계약과 다르게 해석한 부분이 있다면 그 이유
- 다음 Step 2에 필요한 미결정 사항

# Fable Combat I2 prerequisite — canonical response payload and registry

## 목적

최신 `origin/main`에는 `combat_runtime.rs`의 pause/checkpoint 기반은 존재하지만, WP-I2b가 소비해야 하는 canonical composite response payload와 selector/formula registry가 없다. 이 슬라이스는 runtime 판정이나 `GameState` mutation을 만들지 않고 그 선행 계약만 추가한다.

## 정본

1. `AGENTS.md`
2. `docs/dev/Implementation_Slice_Discipline.md`
3. `docs/content/design_source/contracts/intervention.yml`
4. `docs/content/design_source/schema/combat_intervention_response.schema.json`
5. `docs/content/design_source/handoffs/combat_contract_handoff.md`의 WP-I2b

## 소유 파일

- `crates/escape-core/src/combat_intervention.rs` (신규)
- `crates/escape-core/src/lib.rs` (module/public export만)
- `crates/escape-core/tests/combat_intervention_contract.rs` (신규)

그 밖의 Rust/Web/content/save 파일은 수정하지 않는다.

## 구현

- `CombatInterventionPayload`: optional `strategy_modifier` + optional `special_effect`; 둘 중 하나 이상 필수.
- strategy scope/duration/operation과 special effect success/failure branch, typed outcome action DTO를 정본 이름 그대로 정의한다.
- 모든 authoring DTO에 serde unknown-field 거부를 적용한다.
- executor 2개, target 6개, fixed-chance formula 1개, strategy targeting rule registry를 canonical namespaced ID로 고정한다.
- runtime에서 legacy alias(`self`, `target`, `observer`, `opponent`, `any`)와 unknown ID를 거부한다.
- `fixed_chance`는 `chance_percent` 정수 하나만 허용하고 `0..=100`을 검증한다.
- effect/action ID 중복, 빈 ID, 빈 operation, invalid claim policy를 거부한다.
- 이 단계에서 RNG, selector resolution, action plan, overlay mutation, runtime wiring을 구현하지 않는다.

## Acceptance

- effect-only / strategy-only / composite payload가 통과한다.
- empty payload와 unknown fields가 실패한다.
- canonical selector/formula IDs가 통과하고 alias/unknown ID가 실패한다.
- fixed chance missing/extra/wrong type/out-of-range가 실패한다.
- success/failure branch와 outcome action typed validation이 고정된다.
- `cargo fmt --all -- --check`
- `cargo test -p escape-core --test combat_intervention_contract --no-fail-fast`
- `cargo test -p escape-core --test combat_opportunity_wave1 --no-fail-fast`
- `cargo check -p escape-core`
- `git diff --check`

## 중단 조건

- 정본 schema와 Rust 필드명이 충돌한다.
- 기존 public `CombatResponseDefinition`을 이 슬라이스에서 제거해야 한다.
- save schema 또는 simulation fingerprint 변경이 필요하다.
- 소유 파일 밖 gameplay/runtime 변경이 필요하다.

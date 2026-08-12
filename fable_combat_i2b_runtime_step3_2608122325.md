# Fable Combat I2b runtime S3 — target compatibility and source provenance

## 선행 조건

I2b runtime S2 review 완료.

## 소유 파일

- `crates/escape-core/src/combat_intervention.rs`
- `crates/escape-core/src/combat_runtime.rs`
- focused runtime tests

## 구현

- executor-self, selected-target, nearest-active-enemy, lowest-health-active-ally, surrounded-active-ally, all-active-allies 여섯 selector를 pause snapshot에서 resolve한다.
- tie-break와 output ID는 stable ascending이다. lowest health는 max_hp invariant와 integer cross multiplication을 사용한다.
- surrounded는 anchor 6-neighborhood distinct occupied hex를 세고 후보 전체 footprint를 ally count에서 제외한다.
- success/failure 양 branch의 모든 authored effect ID를 RNG 전에 `CombatEffectCatalog`에서 조회한다.
- effect definition의 canonical target selector가 response selector와 exact match해야 한다.
- effect-major, target stable ascending 적용 순서와 source registry/selector/bound ID provenance를 fully-resolved plan에 보존한다.
- mismatch/unknown/zero-target는 preflight rollback이며 RNG/history/state/cost 0이다.

## Acceptance

- 여섯 selector의 stable 결과와 insertion-order 독립성.
- multi-tile surrounded 및 lowest-health ratio edge cases.
- success branch mismatch와 failure branch mismatch 모두 RNG 전 실패.
- unknown effect ID, noncanonical source selector, zero target 실패.
- success/failure resolved plan의 exact source provenance와 deterministic fingerprint.

## 중단 조건

- renderer/ScenePage/save/GameState 변경이 필요하다.
- effect catalog schema가 canonical selector를 표현하지 못한다.
- I7a transaction 적용을 이 슬라이스에 섞어야 한다.

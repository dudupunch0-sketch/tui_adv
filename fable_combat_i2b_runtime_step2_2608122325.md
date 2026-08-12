# Fable Combat I2b runtime S2 — formula tuple and deterministic receipt

## 선행 조건

I2b runtime S1 review 완료.

## 소유 파일

- `crates/escape-core/src/combat_intervention.rs`
- `crates/escape-core/src/combat_runtime.rs`
- 두 module의 focused tests

## 구현

- fixed chance canonical tuple을 정본 순서 그대로 compact JSON으로 직렬화한다.
- tuple 선두 semantic/domain은 `combat.formula.v1.fixed_chance`, `actual_combat`이다.
- BTreeMap parameter key와 target stable ID order를 사용한다.
- FNV-1a64 lowercase 16-hex input fingerprint와 동일 u64 sub-seed를 만든다.
- chance 0/100은 sub-seed/fingerprint는 만들되 RNG draw/roll은 null, 1..99는 draw index 0 한 번과 `sub_seed % 100`, 성공은 `roll < chance`다.
- strategy-only에는 formula receipt가 없고, special effect에는 receipt가 필수다.
- preflight 오류는 pause/history/segment seed/state를 변경하지 않는다.

## Acceptance

- parameter insertion order와 target input order가 결과에 영향 없음.
- canonical tuple byte/fingerprint golden.
- 0/100/1..99의 RNG 0/0/1회 규칙.
- 동일 tuple replay/restore parity, actual/forecast namespace 분리.
- preflight 실패 전후 checkpoint byte equality.

## 중단 조건

- 새로운 RNG source가 필요하다.
- simulation version bump 없이 tuple/hash를 바꿔야 한다.
- receipt를 save/GameState durable schema에 넣어야 한다.

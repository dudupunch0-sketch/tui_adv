# T4 S3c — full-frame checkpoint size measurement

status: ready-for-implementation
date: 2026-08-08
baseline_commit: `7d78d63`
baseline_test: `cargo test --workspace --no-fail-fast --quiet` = 0 failures
workspace: `/home/dudu/work/tui-adv` (WSL)

## 1. 읽기·운영 규칙

1. `docs/design/Combat_Hex_Rework_Handoff.md`
2. `docs/dev/Implementation_Slice_Discipline.md`
3. `fable_combat_hex_t4_slice_plan_2608081232.md`
4. `fable_combat_hex_t4_step8_2608081545.md`

구현자는 `/caveman lite`를 적용한다. 이 slice는 S3b full-frame public checkpoint의 실제
크기를 측정하고, delta 설계에 필요한 baseline만 남긴다. delta/schema/압축 알고리즘은 측정
결과 검토 후 별도 slice다.

## 2. 목표

12 participant·1,200 tick 규모의 deterministic runtime checkpoint를 만들고 다음을 실측한다.

- checkpoint JSON bytes
- SaveEnvelope(JSON) bytes
- execution/resolution frame 수와 평균 frame bytes
- checkpoint 생성 및 serde round-trip 시간(가능한 범위에서 WSL 명령으로 측정)

임의 상한, 압축률 목표, delta 표현을 코드에 고정하지 않는다. 테스트는 결과가 0보다 크고
round-trip fingerprint가 같은지만 보장한다.

## 3. 소유·구현 경계

- 소유: `crates/escape-core/src/combat_runtime.rs`의 `#[cfg(test)]`와 해당 테스트 helper만.
- public schema/save/lib 변경 금지.
- 기존 2 participant fixture의 expected values/fingerprint 변경 금지.
- 측정 출력은 `--nocapture`에서 읽을 수 있게 stable label을 사용한다.

## 4. 구현 순서

### WP-0 — fixture inventory

기존 runtime fixture를 복제하지 말고, 12개 id·고유 위치·동일 role/defense를 deterministic하게
만드는 helper를 추가한다. participant/combatant/defense id 집합 불일치가 없음을 확인한다.

### WP-1 — measurement test

1. `max_ticks = ticks = 1200` request를 만든다.
2. runtime을 끝까지 진행하고 checkpoint를 만든다.
3. `serde_json::to_vec`로 checkpoint와 SaveEnvelope payload를 각각 측정한다.
4. frame count/평균 frame bytes를 계산한다.
5. checkpoint를 JSON round-trip하고 restore/finish fingerprint equality를 확인한다.

## 5. acceptance

- 12 participant·1,200 tick fixture가 panic 없이 끝난다.
- 측정값을 stable labels로 출력하고 구현 보고서에 실제 수치를 기록한다.
- round-trip 후 결과 fingerprint가 원본과 같다.
- `cargo fmt --all -- --check`, `git diff --check`, targeted/full workspace tests 통과.
- delta/압축/schema 변경은 없다.

## 6. 정지 조건

- 1,200 tick fixture가 기존 max tick/overflow 계약과 충돌하는 경우.
- participant 수를 늘리려면 combat rule/fingerprint 기대값을 바꿔야 하는 경우.
- 측정 결과 없이 delta 상한·압축률·새 schema를 추가하려는 경우.

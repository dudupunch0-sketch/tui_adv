# 전투 시스템 Wave 2 / Step 1 — 고정 tick·AI 역할·우선 목표·연속 위치

status: implementation-verified
phase: combat-wave2
date: 2026-07-26

## 목적

Wave 1에서 확정한 `CombatManifest`·`CombatState`·effect/opportunity 계약 위에, 전투가 텍스트 교환이 아니라 **같은 입력에서 같은 순서로 움직이는 고정 tick 시뮬레이션**임을 표현하는 renderer-neutral Rust 계약을 추가한다. 이번 slice는 실제 피해·기술·상태 effect를 적용하지 않고, 전투원의 연속 위치·방향성 기하·역할 가중치·우선 목표 선택·tick별 이동 intent까지만 소유한다.

## 정본 대조

- [전투 시스템 허브](https://app.notion.com/p/36f37e69695e81168360ef11cf3a4449)
- [00. 전투 시스템 개요](https://app.notion.com/p/36f37e69695e8128aa34d4ef1f4f866)
- [01. 전투 루프와 개입 예산](https://app.notion.com/p/36f37e69695e812c92efd2c11edabb66)
- [02. 자동 전투와 상황 트리거](https://app.notion.com/p/36f37e69695e8116a4fdcacbd24c55f8)
- [03. 핵심 상태 시스템](https://app.notion.com/p/36f37e69695e81a9a36fcbe1df5b527f)
- [04. 선택지 생성 규칙](https://app.notion.com/p/36f37e69695e81a090ebe5f63ab5932e)
- [05. 무기 시스템](https://app.notion.com/p/36f37e69695e8108b284e006bc0a3533)
- [06. 동료·환경·실수 시스템](https://app.notion.com/p/36f37e69695e81799020fdeccceb44a0)
- [09. 다수전 전투 시스템](https://app.notion.com/p/3a737e69695e81fc9ab1fe94e2dd98d7)
- [11. 능력치·숙련·전투 스킬 시스템](https://app.notion.com/p/3a837e69695e818eafbcfa309d08149)
- [13. 감독형 관전·전략 피드백 시스템](https://app.notion.com/p/3a937e69695e81daa01df6f79823c4d6)

이번 단계에 적용하는 정본 규칙:

1. 같은 `manifest·seed·simulation version·준비 설정`은 고정 tick·AI·기술 순서의 같은 결과를 내야 한다. 속도/표현 모드는 판정 입력을 바꾸지 않는다.
2. 활성 전투는 최대 아군 4명·적 8명이다. 좌표는 내부 연속 공간이며 방향, 공격/지원/충돌 범위는 데이터로 표현한다. UI 전술 구역 변환은 다음 단계 이후다.
3. 배치·캐릭터당 주 역할 1개·우선 목표·전체 전략은 초기 준비층이다. 역할은 별도 전투 규칙이 아니라 AI 행동 가중치 프리셋이다.
4. 우선 목표는 강제 고정이 아니다. 대상이 유효하지 않거나 접근 불가하면 역할 AI fallback을 사용하고, 동률은 가장 가까운 대상 뒤 stable ID로 결정한다.
5. 좌표/거리 계산은 부동소수점을 사용하지 않는다. 0.01 이상 정밀도가 필요한 후속 수치를 위해 고정소수점 정수 필드를 사용하며, 밸런스 수치는 코드에 임의로 고정하지 않는다.
6. 렌더러는 tick 결과를 표시만 하며, 이번 단계에서 판정·피해·effect·로그 문장을 재계산하지 않는다.

## 소유 파일

coding agent가 수정할 수 있는 파일:

- `crates/escape-core/src/combat_simulation.rs` (신규)
- `crates/escape-core/src/lib.rs` (모듈 선언 및 public re-export)
- `crates/escape-core/tests/combat_simulation_wave2.rs` (신규)

main이 후처리할 문서 파일:

- `docs/design/Combat_System_Implementation_Plan_Index.md`
- `docs/dev/Combat_System_Operating_Guide.md`
- `docs/dev/Combat_System_Goal_Prompt.md`
- `docs/dev/Development_Plan.md` (전투 active plan 링크/상태만)

다음 파일은 수정 금지:

- 기존 `combat_contract.rs`, `combat_state.rs`, `combat_opportunity.rs`의 public shape
- `content`, `turn`, `scene_page`, `escape-wasm`, `web`, office legacy bundle
- 실제 콘텐츠·기술 수치·피해/방어/회복 밸런스
- 미추적 `.claude/worktrees/` 및 다른 작업자의 변경

## 구현 계약

`combat_simulation.rs`는 다음 renderer-neutral 타입과 동작을 제공한다. 필드명은 이 plan을 따르되, 내부 helper 명칭은 구현자가 정할 수 있다.

### 1. 입력/기하

- `CombatSimulationConfig { tick_millis, max_ticks }`: 둘 다 0을 허용하지 않으며, 시간 배속이 아닌 시뮬레이션 입력이다.
- `CombatSide`: `ally` / `enemy`.
- `CombatPosition { x, y }`: 고정소수점 정수 좌표. 거리 제곱, 공격 범위, 지원 범위, 충돌 판정 helper를 제공한다.
- `CombatFacing { x, y }` 또는 동등한 정수 방향 표현. 동일 좌표/0 벡터는 검증 오류로 거부한다.
- `CombatSimulationParticipant`: stable ID, side, position/facing, `speed_per_tick`, `collision_radius`, `attack_range`, `support_range`, `role_id`, optional target policy ID, active 여부를 가진다. 속도/범위는 입력 데이터이고 양수 검증만 한다.

### 2. 역할/목표

- `CombatRoleWeights`: preferred distance, aggression, formation maintenance, pursuit range, protect priority, target priority, risk tolerance, ability priority를 정수 데이터로 보유한다. 이번 단계에서 기술 사용·피해 보정으로 해석하지 않는다.
- `CombatRolePreset { id, weights }`: stable ID 중복과 누락 참조를 검증한다.
- `CombatTargetPreference { target_id, priority }`와 `CombatTargetPolicy { id, preferences, fallback }`를 제공한다. fallback은 최소 `nearest`를 지원하고, 정본의 우선 목표 규칙을 확장할 수 있는 enum/데이터 형태로 둔다.
- 목표 선택은 (1) 살아 있고 active이며 상대 진영인 유효 preference 중 priority 내림차순, 거리 오름차순, stable ID 오름차순, (2) 유효 preference가 없으면 role AI fallback, (3) 그래도 없으면 `None` 순서로 결정한다. 입력 벡터 순서는 결과에 영향을 주지 않는다.

### 3. 고정 tick

- `CombatSimulation::new(input)`은 `CombatManifest`/`CombatState` 검증, manifest fingerprint 결합, active 상한(아군 4·적 8), 역할/목표 참조와 stable ID 중복을 검증한다.
- `advance_tick()`은 현재 snapshot에서 모든 참가자의 target과 move intent를 먼저 계산한 뒤, stable ID 정렬 순서로 **동시에** 좌표를 갱신한다. 한 참가자의 갱신이 같은 tick의 다른 참가자 target 선택에 영향을 주면 안 된다.
- 역할 가중치의 `preferred_distance`와 `aggression`, 참가자의 `speed_per_tick`은 이동 intent를 data-driven하게 만든다. 목표가 선호 거리보다 멀면 접근, 선호 거리보다 가깝고 aggression이 음수면 후퇴, 그 외에는 정지한다. 거리/속도/좌표 계산은 정수 산술로 한다.
- `CombatMoveIntent { actor_id, target_id, from, to, mode }`와 `CombatTickFrame { tick, moves, positions, fingerprint }`를 반환한다. `mode`는 최소 hold/advance/retreat를 표현한다.
- `run_ticks(count)` 또는 동등 API는 매 tick frame을 순서대로 반환하며 `max_ticks`를 넘는 실행을 거부한다. RNG roll, damage, ability, effect application은 호출하지 않는다. actual-combat namespace seed가 필요하면 manifest에서 파생해 fingerprint에만 바인딩하고 임의 난수를 소비하지 않는다.
- snapshot/canonical JSON/fingerprint는 participant/role/policy 입력 순서를 정규화해 같은 manifest·seed·준비 설정이면 동일해야 한다.

## 테스트 요구사항

`combat_simulation_wave2.rs`에 최소 다음 회귀 테스트를 둔다.

1. config/좌표/방향/범위의 0·음수·overflow 및 stable ID 오류가 거부된다.
2. 아군 5명 또는 적 9명의 active 입력이 거부되고, 비활성/배경 placeholder는 상한 계산에서 제외된다.
3. 명시 target preference가 priority → 거리 → ID 순서로 결정되고, 유효하지 않으면 nearest fallback으로 내려간다.
4. role preferred distance/aggression과 speed가 advance/hold/retreat intent 및 다음 좌표에 반영된다.
5. 한 tick에서 participant 입력 순서를 바꿔도 target, move frame, snapshot fingerprint가 동일하다(동시 갱신).
6. 두 simulation 인스턴스가 같은 manifest·seed·준비 설정으로 `run_ticks`를 실행하면 tick별 frame/fingerprint가 동일하고, simulation version 또는 seed 변경은 fingerprint를 바꾼다.
7. `attack_range`, `support_range`, `collision_radius`의 in-range/overlap helper가 경계값에서 결정론적으로 동작한다.
8. `max_ticks` 초과 실행, missing role/policy reference, duplicate participant/role/policy ID가 구조 검증 오류로 반환된다.

## 비범위

- 실제 HP/균형/호흡/공포/분노 변화, 기술 명중·효과 확률, d100, damage/defense/cooldown/recovery 수치
- effect catalog 적용, opportunity 평가/개입 pause, 상태이상 결착
- 배경 전투 집단 결과와 증원 입장, 실제 충돌 해결/공격 판정/AI 스킬 사용
- actual/forecast/retry/auto/fast 결과 parity, 이중 로그, 전투 종료 보고서
- ScenePage/WASM/Web/SuperLightTUI renderer, Canvas/GlyphFX, 콘텐츠 authoring

## 검증 명령

```bash
cargo fmt --all -- --check
cargo test -p escape-core --test combat_contract_wave1 --test combat_state_wave1 --test combat_opportunity_wave1 --test combat_simulation_wave2
cargo test --workspace --no-fail-fast
git diff --check
```

## 보고 형식

- 변경 파일과 public API 요약
- 8개 테스트 요구사항별 통과 여부
- main 재검증 명령과 실제 출력 요약
- 비범위/남은 위험: 실제 resolver·parity·renderer는 다음 단계로 남긴다.

## 구현 보고

- `crates/escape-core/src/combat_simulation.rs`와 public re-export를 추가했다.
- 고정 정수 좌표/범위 helper, active 아군 4·적 8 검증, role weights, target preference/fallback, snapshot 동시 tick, move intent/frame, canonical simulation fingerprint를 구현했다.
- `crates/escape-core/tests/combat_simulation_wave2.rs`에 10개 회귀 테스트를 추가했다. 입력 순서 불변성, inactive duplicate, state/seed/version identity, checked 이동과 structural validation을 포함한다.
- main WSL 검증: `cargo fmt --all -- --check`, Wave 1 + Wave 2 targeted tests(5+8+12+10), `cargo test --workspace --no-fail-fast`, `git diff --check` 모두 통과.
- 비범위 유지: 실제 공격/충돌·피해·기술/상태 effect·로그·parity·renderer·밸런스 수치는 다음 slice로 남긴다.

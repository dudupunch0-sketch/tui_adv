# T1-c — 점유 강제와 포위 판정: 구현 보고

plan: `fable_combat_hex_t1c_step1_2608072138.md`
baseline 확인 커밋: `585a752`

## 1. 시작 확인

워크트리를 `585a752`로 fast-forward한 뒤 확인:

- `cargo test --workspace --no-fail-fast` → **399 passed / 0 failed** (기대와 일치)
- `cd web && npm test` → **168 passed** (기대와 일치, `npm install` 필요했음 — 컨테이너에
  `node_modules`가 없었다)

## 2. WP1의 우여곡절 — 왜 커밋이 5개나 되는가

이 슬라이스는 중간에 한 번 멈추고 코디네이터에게 보고했다가, 소유 목록이 넓혀진 뒤
다시 진행했다. 그 경위를 순서대로 남긴다 — **이 발견 자체가 기억할 가치가 있다.**

1. `cbd01a9` — WP1(§4-2①: `CombatSimulation::new()`가 같은 타일에서 시작하는 두 활성
   전투원을 거부)을 계획대로 구현.
2. 이걸 켠 채로 전 스위트를 돌리자 **소유 목록 밖의 파일 두 곳에서 42개 테스트가
   깨졌다**: `crates/escape-core/tests/combat_resolution_wave2.rs`(21개)와
   `crates/escape-core/tests/combat_spectator_wave3.rs`(그 파일 전체, 21개). 두 파일
   모두 두 전투원 fixture 헬퍼가 기본값으로 **둘 다 같은 타일**(전자는
   `HexCoord{q:0,r:0}`, 후자는 둘 다 `q:0`)에 놓고 있었다. 이 두 테스트 스위트는
   처음부터 공격 판정과 관전 로그 포맷만 검증했지 이동이나 공간 유효성은 신경 쓴 적이
   없어서, 지금까지 좌표가 겹쳐도 아무 문제가 없었다.
3. 이 두 파일은 계획 §3의 "수정 가능" 목록에 없었다. 소유 목록 밖의 파일을 바꿔야
   하는 상황이라 판단해, **WP1을 되돌리고(`6dec84a`) 코디네이터에게 멈추고 보고했다**
   — 이것이 최초 보고였다. 되돌리면서 `occupancy_snapshot`이 전제하던 "두 전투원이
   결코 같은 타일에서 시작하지 않는다"는 보장도 함께 사라져, 그 함수의 `.expect()`가
   (원래도 좌표가 겹쳐 있던) 두 fixture에서 패닉했다 — 그때는 `try_occupy` 실패를
   조용히 무시하는 것으로 임시 봉합했다.
4. **코디네이터가 이 발견을 독립적으로 검증했다**: 두 fixture가 정말로 같은 타일에
   전투원을 놓고 있었고, 이는 "지금까지 아무도 체크하지 않았던, 처음부터 유효하지
   않았던 입력"이었다고 확인했다. `collision_radius: 1`(합산 임계값 2)과
   `attack_range: 2`를 근거로, 전투원 하나를 한 칸만 옮기면(거리 0 → 1) 두 임계값
   모두 그대로 만족되어 판정 결과가 바뀌지 않는다는 것도 미리 계산해서 알려줬다.
   **소유 목록을 이 두 파일까지 넓히고, WP1을 제대로 착지시키라고 지시했다.**
5. 지시대로 두 fixture에서 적 전투원을 한 칸 옮기고(`02393ea`), WP1을 다시
   구현하고(`fd7a183`, 이번엔 그대로 유지), `occupancy_snapshot`의 조용한 무시를
   진짜 에러 반환으로 바꾸고(같은 커밋), 신규 테스트를 되살렸다(`2ea655c`).

## 3. WP별 최종 커밋

| WP | 커밋 | 요약 |
|---|---|---|
| WP2 | `52dfc76` | 경로 차단(관통 금지) — advance/retreat 양쪽, tick-시작 스냅샷 기준 |
| WP3 | `121eb3d` | 목적지 경합 — 둘 다 제자리(우선순위 미발명) |
| WP4 | `34fb29e` | 포위 판정 순수 함수 `surrounding_enemy_neighbors` — 배선 0 |
| (최초 시도) | `cbd01a9` → `6dec84a`로 되돌림 | §2 참고 |
| 픽스처 수정 | `02393ea` | `combat_resolution_wave2.rs`/`combat_spectator_wave3.rs`의 두 전투원을 한 칸 분리 |
| WP1 | `fd7a183` | 초기 배치 중복 타일 거부 — 이번엔 최종 착지. `occupancy_snapshot`이 `Result`를 반환하도록 함께 수정 |
| WP5 | `432fcd5`, `2ea655c` | 신규 `combat_occupancy_t1c.rs` 9개(WP1 pin 포함) + `encounter_combat_wave3.rs` 확인용 단정 1개 |

## 4. 픽스처 수정의 근거 (§2-4, 코디네이터 검증)

`combat_resolution_wave2.rs`의 `p` 클로저와 `combat_spectator_wave3.rs`의
`participants()`는 둘 다 두 번째 전투원("e")을 첫 번째("a")와 같은 타일에 두고
있었다. 각각 적 전투원만 `q`를 1 옮겼다(다른 곳은 건드리지 않음):

- `combat_resolution_wave2.rs`: `e.position = HexCoord { q: 1, r: 0 }` (기존 둘 다
  `(0,0)`)
- `combat_spectator_wave3.rs`: `participant("e", CombatSide::Enemy, 1)` (기존 둘 다
  `q: 0`)

두 파일 모두 `collision_radius: 1`(합산 임계값 2), `attack_range: 2`를 쓰고, 공유
역할의 `preferred_distance: 1`에 `aggression: 1`(음수 아님)이라 애초에 어느 tick에도
움직이지 않는다(전진 조건 `d > preferred`도, 후퇴 조건 `d < preferred && aggression <
0`도 성립하지 않음 — 예전 거리 0에서도, 새 거리 1에서도 마찬가지). 즉 이 변경은
순수한 fixture 수정이고 실제 판정 결과에 영향이 없다. 두 파일 전부(24개, 21개) 및
`.position`을 명시적으로 덮어쓰는 개별 테스트(`attack_range_is_measured_in_hex_distance`,
`accuracy_range_penetration_and_overflow_are_explicit`의 `far` 케이스 등, 이들은
애초에 기본값을 쓰지 않으므로 무관)까지 전부 통과를 재확인했다.

## 5. 검증 명령과 실제 출력 (최종)

```
$ cargo fmt --all -- --check
(종료 코드 0, 출력 없음)

$ cargo test -p escape-core --test combat_occupancy_t1c
running 9 tests
test a_tile_vacated_this_tick_is_not_entered_this_tick ... ok
test a_unit_stops_before_an_occupied_tile_instead_of_passing_through ... ok
test retreat_is_blocked_by_occupancy_too ... ok
test surround_count_reports_enemy_occupied_neighbors_only ... ok
test two_participants_cannot_start_on_the_same_tile ... ok
test surround_detection_is_not_wired_into_movement_or_targeting ... ok
test two_units_targeting_the_same_tile_both_hold ... ok
test no_two_units_share_a_tile_at_any_tick ... ok
test shuffled_participant_order_yields_identical_frames ... ok
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test -p escape-core --test combat_resolution_wave2 --test combat_spectator_wave3
combat_resolution_wave2: test result: ok. 24 passed; 0 failed
combat_spectator_wave3:  test result: ok. 21 passed; 0 failed

$ cargo test -p escape-core --test encounter_combat_wave3
running 32 tests
... (전부 ok)
test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --workspace --no-fail-fast
(전 스위트, 실패 0건 -- 합계 아래)

$ git diff --check
(종료 코드 0, 출력 없음)

$ cd web && npm test
 Test Files  16 passed (16)
      Tests  168 passed (168)
```

**Rust 합계: 409 passed / 0 failed** (399 baseline + `combat_occupancy_t1c.rs` 9개
[WP1 pin 포함] + `encounter_combat_wave3.rs` 확인용 단정 1개 = 409). 감소 없음,
계획이 기대한 신규 개수와 일치(§9 최종 체크리스트 참고). **web: 168 그대로.**

## 6. §6의 두 확인 — WP1 착지 후 재확인, 반드시 둘 다

### ① 고정 테스트 값이 그대로인가 — **예, 그대로다.**

`authored_preview_bout_behaviour_is_unchanged_by_the_coordinate_swap` 통과, 값 변경 없음:

- `combat.view.frames.len()` == 8
- `report.decisive_tick` == `Some(8)`
- `report.outcome` == `MutualDefeat`
- `report.reason` == `BothSidesDefeated`
- 피해 로그 16건, 전부 `value_hundredths == Some(1333)`

WP1은 `CombatSimulation::new()`의 생성 시점 검증만 추가하고 이동·판정 로직을 전혀
건드리지 않으므로, 이 저작 전투(시작 좌표가 이미 서로 다른 `(0,0)`/`(5,0)`)는 WP1
착지 전후로 완전히 동일하게 동작한다 — 실측으로도 재확인했다.

### ② 프레임 좌표가 실제로 달라졌는가 — **예, 달라졌다 (WP2/3의 효과, WP1과 무관하게 동일).**

§7의 궤적 비교가 그 증거다. WP1 착지 전/후 양쪽에서 동일한 궤적을 다시 뽑아
확인했다 — WP1은 저작 콘텐츠의 시작 좌표가 이미 서로 다르므로 이 전투에 전혀
영향을 주지 않는다.

**만약 이 둘 중 하나라도 반대였다면(값이 움직였거나, 좌표가 그대로였다면) 여기서
멈추고 보고했을 것이다 — 실제로는 예측대로 "①은 그대로, ②는 변경됨"이었다.**

## 7. 저작 전투 좌표 궤적 — 변경 전 / 변경 후

`wuxia_combat_spectator_preview_bout` (ally는 `(0,0)`에서, challenger는 `(5,0)`에서
시작, `preferred_distance: 0`, `speed_per_tick: 1`, `attack_range: 10`).

**변경 전 (기록된 결함 — 서로 관통하며 좌우가 뒤바뀐다):**

| tick | ally | challenger |
|---|---|---|
| 1 | (1,0) | (4,0) |
| 2 | (2,0) | (3,0) |
| 3 | **(3,0)** | **(2,0)** ← 여기서 이미 서로를 지나쳤다 (ally가 challenger보다 오른쪽) |
| 4 | (2,0) | (3,0) |
| 5 | (3,0) | (2,0) |
| 6 | (2,0) | (3,0) |
| 7 | (3,0) | (2,0) |
| 8 | (2,0) | (3,0) |

**변경 후 (이 슬라이스 적용 후 — 인접에서 멈추고 좌우가 고정된다):**

| tick | ally | challenger |
|---|---|---|
| 1 | (1,0) | (4,0) |
| 2 | (2,0) | (3,0) |
| 3 | (2,0) | (3,0) |
| 4 | (2,0) | (3,0) |
| 5 | (2,0) | (3,0) |
| 6 | (2,0) | (3,0) |
| 7 | (2,0) | (3,0) |
| 8 | (2,0) | (3,0) |

tick 2부터 두 전투원은 서로의 타일(각각 목적지 경합·경로 차단에 걸림)을 두고 그
자리에 서고, ally는 항상 challenger보다 왼쪽(`q` 값이 작음)을 유지한다 — 정본 09의
좌우 계약이 재생 내내 지켜진다. 둘 다 `attack_range: 10` 이내이므로 명중은 매 tick
유지되고, 이것이 §6-①의 값이 그대로인 이유다.

이 궤적은 `authored_preview_bout_never_lets_the_two_combatants_swap_sides_or_share_a_tile`
(WP5, `encounter_combat_wave3.rs`)로 회귀 테스트에 고정했다.

## 8. 계획과 다르게 구현한 부분과 사유

1. **소유 목록 확장** — §2에서 상세히 다뤘다. 계획 §3의 원래 목록에 없던
   `combat_resolution_wave2.rs`/`combat_spectator_wave3.rs`를 코디네이터 지시로
   수정했다. 두 파일의 두 전투원 fixture는 처음부터 같은 타일에 있었고, 이는
   T1-c 이전까지 아무도 체크하지 않았던 무효 입력이었다 — 이번 슬라이스가 그
   결함을 처음으로 드러낸 것이다.
2. **`occupancy_snapshot`이 `Result`를 반환하도록 변경** — 계획은 이 함수 자체를
   명시하지 않았다. WP1이 자리를 잡으면서 "두 전투원이 tick 시작 시점에 결코 같은
   타일에 있지 않다"는 전제가 다시 성립하므로, 그 전제가 깨졌을 때(있어서는 안 될
   버그) 패닉이 아니라 `CombatSimulationError::OccupancyInvariantViolated`로
   보고하도록 했다 — 이 크레이트의 다른 어디도 잘못된 상태에서 패닉하지 않는다는
   원칙을 따른 것이다.
3. 그 외 WP2/WP3/WP4/WP5는 계획 그대로 구현했다.

## 9. §11 최종 체크리스트

- [x] 어떤 tick에서도 두 유닛이 같은 타일에 있지 않다 — `no_two_units_share_a_tile_at_any_tick`,
      그리고 이제 **애초에 같은 타일에서 시작하는 입력** 자체도
      `two_participants_cannot_start_on_the_same_tile`로 거부된다
- [x] 관통이 불가능하다 (전진·후퇴 양쪽) — `a_unit_stops_before_an_occupied_tile_instead_of_passing_through`,
      `retreat_is_blocked_by_occupancy_too`
- [x] 목적지 경합에서 우선순위를 발명하지 않았다 — `resolve_destination_contention` 및
      그 doc 주석, `two_units_targeting_the_same_tile_both_hold`
- [x] 점유를 tick 시작 스냅샷에서 읽는다 — `occupancy_snapshot`은 `advance_tick`이
      매 tick 한 번만 만든다
- [x] 입력 순서를 섞어도 프레임이 동일하다 — `shuffled_participant_order_yields_identical_frames`
- [x] "이번 tick에 비는 타일은 못 들어간다"가 의도임이 주석과 테스트에 남았다 —
      `first_free_tile_along` doc 주석, `a_tile_vacated_this_tick_is_not_entered_this_tick`
- [x] 포위 판정이 순수 함수이며 어디에도 배선되지 않았다 — `surrounding_enemy_neighbors`,
      `surround_detection_is_not_wired_into_movement_or_targeting`
- [x] 포위 임계값 상수를 만들지 않았다 — 반환은 `Vec<HexCoord>`, 개수는 `.len()`으로만
- [x] `combat_hex.rs`·`combat_resolution.rs`·terminal·web·YAML·번들 무변경 — `git diff --stat`
      확인, 이 네 범주에 변경 없음
- [x] `collision_radius`가 그대로 남아 있다 — 필드·검증 로직 무변경
- [x] version bump 없음 — `CombatSimulationParticipant`/`CombatTickFrame` 등 직렬화
      가능 타입에 필드 추가·삭제 없음, `simulation_version`은 v3 그대로
- [x] §6의 고정 테스트 값이 변하지 않았고, 좌표는 변했다 — §6·§7 참고
- [x] `cargo fmt --all -- --check`, `git diff --check` 통과
- [x] Rust 399에서 감소 없음, web 168 무변경 — **Rust 409 passed / 0 failed**
      (399 + 신규 10개), **web 168 passed**

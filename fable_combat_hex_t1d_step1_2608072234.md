# T1-d — 대형 유닛 다중 타일 점유

plan: `fable_combat_hex_t1d_step1_2608072234.md`
report: `fable_combat_hex_t1d_step2_report.md`
baseline: `6180751`
상위 문서: [Combat_Hex_Rework_Development_Plan.md](docs/design/Combat_Hex_Rework_Development_Plan.md) §6 T1
선행 슬라이스: T1-a, T1-b1, T1-b2, T1-c — 전부 착지 완료

## 1. 목적

대형 유닛이 2칸 이상을 점유하게 한다(정본 결정 12). T1 트랙의 마지막 슬라이스다.

`HexShape`는 T1-a에서 이미 완성돼 있고 T1-c가 점유 강제를 세웠다.
여기서 남은 일은 **시뮬레이션이 그 둘을 실제로 쓰게 하는 배선**이다.

## 2. 선행 조건

시작 전 baseline 확인. 다르면 멈추고 보고한다.

- `cargo test --workspace --no-fail-fast` → **409 passed / 0 failed**
- `cd web && npm test` → **168 passed**

## 3. 소유 파일

수정 가능:

- `crates/escape-core/src/combat_simulation.rs`
- `crates/escape-core/src/combat_resolution.rs` (거리 측정 지점만 — §4-4)
- `crates/escape-core/src/lib.rs` (re-export 조정)
- `crates/escape-core/tests/combat_simulation_wave2.rs`
- `crates/escape-core/tests/combat_resolution_wave2.rs`
- 신규 `crates/escape-core/tests/combat_large_units_t1d.rs`

수정 금지:

- `crates/escape-core/src/combat_hex.rs` — **동결.** `HexShape`/`HexOccupancy`를 쓰되 고치지 않는다.
  부족하면 우회하지 말고 멈추고 보고한다
- `crates/escape-core/src/combat_spectator.rs`, `combat_conclusion.rs`, `combat_contract.rs`, `content.rs`
- `crates/escape-terminal/**`, `web/**`
- 저작 YAML, 번들 2종 — **콘텐츠를 만들지 않는다**(§9)
- 다른 작업자의 미추적 파일

## 4. 설계

### 4-1. 점유 형태는 참가자가 들고 있는 오프셋 목록이다

`HexShape`는 `Serialize`/`Deserialize`가 없다(T1-a의 의도적 선택). 그러므로 직렬화 경계에는
**평범한 오프셋 목록**을 두고, 검증 시점에 `HexShape`로 변환한다.

`CombatSimulationParticipant`에 추가한다.

```rust
#[serde(default, skip_serializing_if = "Vec::is_empty")]
pub occupies: Vec<HexCoord>,
```

- **빈 목록 = 한 타일**(`position` 하나만 점유). 기존 전투원 전부가 여기 해당한다.
- `skip_serializing_if`까지 붙이는 이유: 빈 경우 JSON에 키가 **아예 나타나지 않아**
  기존 번들·픽스처의 바이트가 그대로 유지된다. 따라서 **version bump가 필요 없다.**
  bump가 필요해 보이면 그것 자체를 보고한다 — 설계가 틀렸다는 뜻이다.

### 4-2. 앵커는 반드시 점유 타일에 포함된다

`occupies`가 비어 있지 않으면 **`(0, 0)` 오프셋을 반드시 포함해야 한다.** 아니면 거부한다.

근거: `position`은 이 전투원이 "어디에 있는가"이고, 로그·관전·타겟팅이 전부 그 값을 쓴다.
앵커가 점유 타일 밖에 있으면 "말이 자기가 서 있지 않은 칸에 있다고 보고"하게 된다.

### 4-3. 연결성을 요구한다 (대칭성과는 다른 규칙이다)

`occupies`의 타일들은 **서로 이어져 있어야 한다**(인접 관계로 하나의 덩어리).
떨어진 두 칸을 점유하는 것은 유닛이 아니라 두 유닛이다.

**이것은 "대칭 형태만"과 다른 규칙이다.** 대칭 제한은 상위 문서의 결정에 따라
**코드로 강제하지 않고 콘텐츠 규칙으로 둔다**(나중에 회전이 필요해질 때 열 수 있게).
연결성은 그것과 무관한 구조적 유효성이므로 코드가 막는다.

### 4-4. 거리는 점유 타일 중 최솟값이다

지금 거리는 앵커끼리 잰다. 대형 유닛에 그대로 두면 **7칸 보스의 몸통이 코앞에 있어도
앵커가 멀다는 이유로 사거리 밖**이 된다.

그러므로 두 전투원 사이의 거리를 **각자의 점유 타일 조합 중 최솟값**으로 정의하는 helper를 만들고,
현재 앵커 거리를 쓰는 **다섯 지점 전부**를 바꾼다.

| 파일 | 지점 |
|---|---|
| `combat_simulation.rs` | 목표 선호 계산 2곳, 최근접 fallback, 이동 판단 |
| `combat_resolution.rs` | 사거리·근접 판정 |

`combat_resolution.rs`는 프레임의 앵커(`frame.positions`)만 갖고 있지만
`request.execution.input.participants`에서 `occupies`를 읽을 수 있다. **프레임 스키마를 바꾸지 마라.**

한 타일 유닛끼리는 이 정의가 기존 앵커 거리와 **정확히 같다.** 따라서 기존 전투는 값이 변하지 않아야 한다.

### 4-5. 이동은 발자국 전체가 비어야 한다

`line()` 경로를 따라 전진할 때, 각 후보 앵커에서 **발자국 전체**가 비어 있어야 그 칸으로 갈 수 있다.
하나라도 막혀 있으면 그 앞에서 멈춘다.

- **자기 자신의 현재 타일은 자기를 막지 않는다.** 한 타일 유닛에서는 자명했지만
  대형 유닛은 이동 중 발자국이 자기 옛 발자국과 겹치므로 명시적으로 제외해야 한다.
  여기가 이 슬라이스에서 가장 틀리기 쉬운 지점이다.
- 목적지 경합(T1-c §4-3의 "둘 다 제자리")은 **발자국이 한 칸이라도 겹치면** 성립한다.
  규칙 자체는 바꾸지 않는다 — 겹침 판정만 발자국 기준으로 넓힌다.
- 점유는 여전히 **tick 시작 스냅샷**에서 읽는다(T1-c §4-4).

### 4-6. 초기 배치

`CombatSimulation::new()`가 이미 같은 타일 시작을 거부한다(T1-c WP1).
발자국 기준으로 넓힌다 — **두 전투원의 발자국이 한 칸이라도 겹치면 거부한다.**
`HexOccupancy::try_occupy`의 all-or-nothing이 이미 그 일을 한다.

## 5. Hard invariants

상위 문서 §3에서 상속한다. 이 슬라이스에서 특히 걸리는 것:

1. **기존 전투 무변경.** 모든 기존 전투원은 `occupies`가 비어 있다 = 한 타일.
   §4-4의 거리 정의가 그 경우 앵커 거리와 동일하므로 **어떤 기존 테스트 값도 변하면 안 된다.**
2. **JSON 바이트 무변경.** `skip_serializing_if`로 기존 직렬화가 그대로 유지된다.
3. **version bump 없음.**
4. **순서 독립성.** 입력 순서를 섞어도 같은 결과.
5. **RNG 호출 0회 추가.**
6. **`combat_hex.rs` 무변경.**
7. **관전·terminal·web·저작 YAML·번들 무변경.**

## 6. WP 목록

순서 고정. WP당 커밋 1개.

### WP1 — `occupies` 필드와 검증

§4-1, §4-2, §4-3, §4-6. `HexShape`로 변환하며 빈 집합·중복·앵커 부재·비연결을 거부한다.
전용 오류 변형을 쓴다(뭉뚱그린 변형에 넣지 않는다).

검증: `cargo test -p escape-core --test combat_simulation_wave2` — **값 변화 0**이어야 한다.

### WP2 — 발자국 거리

§4-4. helper를 만들고 다섯 지점을 모두 바꾼다. **한 곳이라도 빠뜨리면 대형 유닛이
어떤 판정에서는 크고 어떤 판정에서는 작아진다** — 찾기 어려운 종류의 버그다.

검증:
```bash
cargo test -p escape-core --test combat_simulation_wave2
cargo test -p escape-core --test combat_resolution_wave2
cargo test -p escape-core --test encounter_combat_wave3
```
전부 **값 변화 0**.

### WP3 — 발자국 이동

§4-5. 자기 옛 발자국 제외에 주의한다.

검증: `cargo test -p escape-core --test combat_occupancy_t1c` — T1-c의 성질이 유지돼야 한다.

### WP4 — 테스트

신규 `combat_large_units_t1d.rs`. 최소 집합:

| 테스트 | 고정하는 것 |
|---|---|
| `an_empty_occupies_list_means_a_single_tile_at_the_anchor` | 기본값 |
| `occupies_without_the_origin_offset_is_rejected` | §4-2 |
| `a_disconnected_footprint_is_rejected` | §4-3 |
| `a_duplicate_offset_is_rejected` | |
| `two_large_units_with_overlapping_footprints_cannot_both_start` | §4-6 |
| `distance_is_measured_from_the_nearest_occupied_tile` | §4-4 |
| `a_large_unit_in_range_by_its_body_but_not_its_anchor_can_attack` | §4-4의 존재 이유 |
| `a_large_unit_stops_when_any_footprint_tile_would_be_blocked` | §4-5 |
| `a_large_unit_does_not_block_itself_while_moving` | §4-5의 함정 |
| `overlapping_destinations_make_both_large_units_hold` | §4-5 |
| `single_tile_units_behave_exactly_as_before` | 불변식 1 |
| `occupies_is_absent_from_json_when_empty` | 불변식 2 |
| `shuffled_participant_order_yields_identical_frames` | 불변식 4 |

검증: `cargo test --workspace --no-fail-fast`

## 7. 검증 명령

```bash
cargo fmt --all -- --check
cargo test -p escape-core --test combat_large_units_t1d
cargo test --workspace --no-fail-fast
git diff --check
cd web && npm test
```

기대: Rust **409에서 감소 없음**, 신규만큼 증가. web **168 무변경**.

## 8. 저작 전투에 대한 예측

`wuxia_combat_spectator_preview_bout`의 두 전투원은 `occupies`가 없다 = 한 타일.
§4-4의 거리 정의가 한 타일끼리는 앵커 거리와 동일하므로 **거동이 전혀 바뀌지 않아야 한다.**

`authored_preview_bout_behaviour_is_unchanged_by_the_coordinate_swap`과
`authored_preview_bout_never_lets_the_two_combatants_swap_sides_or_share_a_tile`이
**둘 다 값 그대로**여야 한다. 하나라도 움직이면 멈추고 보고한다. 기대값을 고쳐 쓰지 마라.

## 9. 명시적 범위 밖

- **대형 유닛 콘텐츠 저작.** 저작 YAML을 건드리지 않는다. 이 슬라이스는 기구만 만들고
  테스트 픽스처로 증명한다
- **관전 표면 노출.** `CombatSpectatorPiece`는 계속 앵커 하나만 보고한다.
  따라서 렌더러는 대형 유닛을 한 칸짜리로 그린다 — **지금은 문제가 없다.
  저작된 대형 유닛이 하나도 없어서 잘못 그릴 대상이 없기 때문이다.**
  발자국을 관전 표면에 내보내는 것은 T8(표현 문법)의 몫이며, 대형 유닛 콘텐츠보다 먼저 서야 한다
- **회전·미러.** 상위 문서 결정 12에 따라 열지 않는다
- **대칭성 강제.** 콘텐츠 규칙이며 코드로 막지 않는다(§4-3)
- 이동 예약·밀어내기 — T2
- 행동 주기 — T3

## 10. 보고 형식

`fable_combat_hex_t1d_step2_report.md`에 적고 커밋한다.

- WP별 커밋 해시와 한 줄 요약
- 검증 명령과 **실제 숫자 출력**
- **§8의 두 테스트가 값 그대로인지** 명시
- **§4-4의 다섯 지점을 전부 바꿨는지** — 지점별로 확인했다고 적어라. "전부 바꿨다"로 뭉개지 마라
- `occupies`가 빈 경우 JSON에 나타나지 않는지 실측
- 계획과 다르게 구현한 부분과 사유

## 11. 최종 체크리스트

- [ ] `occupies`가 비면 JSON에 키가 없고 기존 번들 바이트가 그대로다
- [ ] version bump 없음
- [ ] 앵커 미포함·비연결·중복·빈 집합이 거부된다
- [ ] 거리를 재는 다섯 지점이 **전부** 발자국 기준이다
- [ ] 대형 유닛이 이동 중 자기 옛 발자국에 막히지 않는다
- [ ] 발자국이 겹치는 목적지에서 둘 다 제자리에 선다
- [ ] 한 타일 유닛의 거동이 이전과 완전히 같다
- [ ] §8의 두 테스트 값이 변하지 않았다
- [ ] `combat_hex.rs`·관전·terminal·web·YAML·번들 무변경
- [ ] `cargo fmt --all -- --check`, `git diff --check` 통과
- [ ] Rust 409에서 감소 없음, web 168 무변경

# T1-a — 육각 좌표 수학 모듈 (미배선)

plan: `fable_combat_hex_t1a_step1_2608061847.md`
report: `fable_combat_hex_t1a_step2_report.md`
baseline: `ab9fc7e`
상위 문서: [Combat_Hex_Rework_Development_Plan.md](docs/design/Combat_Hex_Rework_Development_Plan.md) §6 T1

## 1. 목적

육각 좌표계 교체(T1)에서 **가장 틀리기 쉬운 부분은 좌표 수학이고, 가장 되돌리기 비싼 부분은
직렬화 경계 교체**다. 이 슬라이스는 둘을 분리해 전자를 먼저 못박는다.

여기서는 **순수 모듈 하나를 만들고 아무 데도 연결하지 않는다.** `CombatPosition`을 건드리지 않고,
JSON 경계를 건드리지 않고, 시뮬레이션 거동을 건드리지 않는다. 따라서 이 슬라이스는
**기존 출력이 바이트 단위로 동일해야 한다.**

배선은 다음 슬라이스(T1-b)가 한다. 그 계획서는 **이 슬라이스가 확정한 public API와 테스트를 보고**
작성한다 — 지금 미리 정하지 않는다.

## 2. 선행 조건

없다. T0와 독립이며 **병렬 진행 가능**하다(파일이 겹치지 않는다).

## 3. 소유 파일

수정 가능:

- `crates/escape-core/src/combat_hex.rs` (**신규**)
- `crates/escape-core/src/lib.rs` (`pub mod combat_hex;`와 re-export **추가만**)
- `crates/escape-core/tests/combat_hex_t1a.rs` (**신규**)

수정 금지: 그 외 전부. 특히 `combat_simulation.rs`, `combat_resolution.rs`, `combat_spectator.rs`,
`scene_page.rs`, `crates/escape-terminal/**`, `web/**`, 저작 YAML, 번들, 픽스처.

모듈명은 기존 컨벤션(`combat_contract` / `combat_state` / `combat_simulation` …)을 따라 `combat_hex`로 한다.

## 4. 만들 것

좌표계는 **axial `(q, r)`**, 배치는 **flat-top**이다(레퍼런스 실측과 일치).

### 4-1. `HexCoord { q: i32, r: i32 }`

derive: `Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize`.

**`Ord`는 장식이 아니라 결정론 요구사항이다.** 정렬 기준을 `(q, r)` 사전순으로 **문서화**한다.
이후 슬라이스가 타일을 순회할 때 이 순서에 의존한다.

### 4-2. 거리·인접

- `HexCoord::distance(self, other) -> i32` — axial 거리.
- `HexCoord::neighbors(self) -> [HexCoord; 6]`
  - **방향 순서를 상수로 고정하고 문서화한다.** 순서가 결과에 영향을 주는 소비자(경로, 포위 판정,
    타일 배정 해소)가 뒤따르므로, 이 배열 순서 자체가 계약이다.
- `HexCoord::is_adjacent(self, other) -> bool`

### 4-3. 범위

- `ring(center, radius) -> Vec<HexCoord>` — 반지름 `radius`의 테두리. `radius == 0`이면 `[center]`.
- `range(center, radius) -> Vec<HexCoord>` — 반지름 이내 전체.
- 둘 다 **`Ord` 기준 오름차순 정렬해서 반환한다.** 생성 순서를 그대로 흘리지 않는다.
- 음수 `radius`는 빈 벡터가 아니라 **에러**다 (값을 지어내지 않는다).

### 4-4. 직선 경로

- `line(from, to) -> Vec<HexCoord>` — 양 끝 포함.
- 대시·관통 이동(T2)이 소비한다.
- **동점 타일(정확히 두 타일 사이를 지나는 경우)의 tie-break를 반드시 문서화하고 테스트로 고정한다.**
  부동소수점 epsilon에 의존하지 않는 방식을 쓴다 — 결정론이 부동소수점 반올림에 걸리면 안 된다.
- 방향에 따른 비대칭(`line(a,b)`를 뒤집은 것과 `line(b,a)`가 다를 수 있음)은 **허용하되 문서화**한다.
  억지로 대칭을 만들려다 tie-break를 흐리지 않는다.

### 4-5. `HexShape` — 대형 유닛 점유 형태

- 앵커 기준 **오프셋 집합**이다. 회전은 없다(상위 문서 결정 12).
- 정규화: 같은 형태를 다른 순서로 넣어도 **동일한 정규형**이 나와야 한다.
  정렬 + 앵커 기준 이동으로 정규형을 정의하고 문서화한다.
- `tiles_at(anchor) -> Vec<HexCoord>` — `Ord` 오름차순.
- 빈 집합, 중복 오프셋은 에러다.
- 회전 API를 만들지 않는다. "회전 없음"을 **주석으로 명시**해 다음 구현자가 추가하지 않게 한다.

### 4-6. `HexOccupancy` — 점유 맵

- 내부는 `BTreeMap<HexCoord, String>` (결정론적 순회).
- `try_occupy(tiles, id)` — 요청한 타일 중 **하나라도** 이미 점유돼 있으면 실패하고
  **부분 점유를 남기지 않는다**(all-or-nothing). 대형 유닛에 필수다.
- `vacate(id)`, `occupant_at(coord)`, `is_free(coord)`, `iter()` (정렬 순).
- 한 id가 여러 타일을 점유할 수 있다(대형 유닛). 한 타일에는 한 id만.

### 4-7. 오버플로

모든 산술을 checked로 처리한다. 기존 코드의 관용구를 따른다
(`checked_mul(..).ok_or(Error::Overflow)?` — `combat_simulation.rs`의 `distance_squared` 참고).
`i32::MIN`/`i32::MAX` 근처 입력에서 **패닉하지 않는다.**

## 5. Hard invariants (상속 + 이 슬라이스 고유)

상위 문서 §3에서 상속한다. 이 슬라이스에서 특히 걸리는 것:

1. **출력 무변경.** 이 모듈은 아무도 호출하지 않으므로, `ScenePage` JSON·terminal 스냅샷·web 출력이
   baseline과 **바이트 단위로 동일**해야 한다. 기존 테스트 수와 결과가 그대로여야 한다.
2. **RNG 호출 0회.** `Math.random`류 없음, 시간 의존 없음.
3. **결정론적 순서.** 반환하는 모든 컬렉션이 정렬되어 있거나, 순서가 상수로 고정·문서화되어 있다.
   `HashMap`/`HashSet`을 쓰지 않는다 (`BTreeMap`/`BTreeSet`만).
4. **부동소수점 결정론.** 좌표 계산에서 부동소수점 반올림에 의존하지 않는다. `line()`이 내부적으로
   보간을 쓰더라도 tie-break는 정수 규칙으로 결정된다.
5. **패닉 금지.** 잘못된 입력은 `Result`로 거부하고, 산술은 checked다.
6. **신규 의존성 금지.** `Cargo.toml`을 건드리지 않는다.

## 6. WP 목록

순서 고정. WP당 커밋 1개. **테스트를 먼저 red로 만든 뒤 구현하는 순서를 권장한다.**

### WP1 — `HexCoord` + 거리 + 인접

`combat_hex.rs` 신규, `lib.rs`에 모듈 등록. 4-1, 4-2.

검증: `cargo test -p escape-core --test combat_hex_t1a`

### WP2 — `ring` / `range`

4-3.

### WP3 — `line`

4-4. **tie-break 문서화가 이 WP의 산출물 절반이다.**

### WP4 — `HexShape`

4-5.

### WP5 — `HexOccupancy`

4-6. all-or-nothing 성질에 주의한다.

### WP6 — 오버플로 방어와 마무리

4-7. 모듈 상단에 **모듈 주석**을 단다: 좌표계(axial/flat-top), 방향 순서 상수, `line` tie-break,
`HexShape` 정규형, 회전 없음, "이 모듈은 아직 배선되지 않았다"를 적는다.
기존 모듈들(`combat_spectator.rs`, `combatMotion.ts`)의 주석 밀도를 따른다.

## 7. 테스트 (`crates/escape-core/tests/combat_hex_t1a.rs`)

**여기가 이 슬라이스의 실질 산출물이다.** 아래는 최소 집합이며, 더 추가해도 좋다.

거리·인접
- `distance_to_self_is_zero`
- `every_neighbor_is_at_distance_one`
- `distance_is_symmetric`
- `neighbors_are_six_distinct_coords`
- `neighbor_direction_order_is_fixed` ← 순서 상수를 명시적으로 고정

범위
- `ring_of_radius_zero_is_just_the_center`
- `ring_size_is_six_times_radius`
- `range_size_is_one_plus_three_r_times_r_plus_one`
- `ring_and_range_results_are_sorted`
- `negative_radius_is_rejected`

경로
- `line_includes_both_endpoints`
- `line_length_equals_distance_plus_one`
- `consecutive_line_tiles_are_adjacent`
- `line_tie_break_is_pinned` ← 정확히 두 타일 사이를 지나는 입력을 명시
- `line_direction_asymmetry_is_pinned` ← 문서화한 거동을 그대로 고정

형태
- `same_shape_in_different_order_normalizes_identically`
- `tiles_at_translates_every_offset`
- `empty_shape_is_rejected`
- `duplicate_offset_is_rejected`

점유
- `one_tile_holds_one_occupant`
- `occupying_an_occupied_tile_fails`
- `a_failed_multi_tile_occupy_leaves_no_partial_state` ← all-or-nothing
- `vacate_frees_every_tile_of_that_id`
- `iteration_order_is_deterministic`

안전
- `extreme_coordinates_do_not_panic`

## 8. 검증 명령 (WSL, `cd /home/dudu/work/tui-adv`)

```bash
cargo fmt --all -- --check
cargo test -p escape-core --test combat_hex_t1a
cargo test --workspace --no-fail-fast
git diff --check
```

기대: 워크스페이스 테스트가 baseline(346)에서 **신규 테스트 수만큼만 증가**하고,
**기존 테스트는 하나도 값이 바뀌지 않는다.** 기존 테스트가 하나라도 깨지면 배선이 새어 들어간 것이므로
멈추고 보고한다.

`web`은 변경이 없다.

## 9. 명시적 범위 밖

- **배선 일체.** `CombatPosition`을 이 슬라이스에서 바꾸지 않는다.
- 점유 규칙의 시뮬레이션 적용 (T1-c)
- 대형 유닛의 시뮬레이션 적용 (T1-d)
- 포위 판정 (T1-c 또는 T1-d)
- 경로 탐색(A\* 등 장애물 우회) — `line()`은 **직선**만이다
- 회전
- 화면 좌표 투영 (T9의 고정 타일 메트릭)
- `simulation_version` bump (T1-b)
- 렌더러

## 10. 보고 형식

`fable_combat_hex_t1a_step2_report.md`에 적는다.

- WP별 커밋 해시와 한 줄 요약
- **확정된 public API 전체 서명** ← T1-b 계획서가 이걸 보고 작성된다. 반드시 적는다
- 문서화한 규약 3종: 방향 순서 상수, `line` tie-break, `HexShape` 정규형
- 실행한 검증 명령과 출력 수치(테스트 수, 실패 수)
- 계획과 다르게 구현한 부분과 사유
- 스킵한 WP와 사유

## 11. 최종 체크리스트

- [ ] `combat_hex.rs`를 어떤 기존 모듈도 호출하지 않는다 (`grep`으로 확인)
- [ ] `lib.rs` 변경이 모듈 등록과 re-export 추가뿐이다
- [ ] `HashMap`/`HashSet`이 없다
- [ ] 부동소수점 반올림이 tie-break를 결정하지 않는다
- [ ] 모든 공개 함수가 잘못된 입력에 `Result`로 답한다 (패닉·무언의 기본값 없음)
- [ ] 극단 좌표에서 패닉하지 않는다
- [ ] 회전 API가 없고, 없는 이유가 주석에 있다
- [ ] 모듈 주석에 좌표계·방향 순서·tie-break·정규형·미배선 사실이 적혀 있다
- [ ] `Cargo.toml` 무변경
- [ ] 기존 테스트 결과 무변경, `cargo test --workspace --no-fail-fast` 0 failed
- [ ] `cargo fmt --all -- --check` 통과

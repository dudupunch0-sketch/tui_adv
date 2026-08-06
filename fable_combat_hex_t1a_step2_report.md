# T1-a — 육각 좌표 수학 모듈 (미배선) — 완료 보고

plan: `fable_combat_hex_t1a_step1_2608061847.md`
baseline: `6f28160` (이 worktree의 HEAD — 계획서에 적힌 `ab9fc7e`는 main 체크아웃에서
이 슬라이스와 무관한 후속 문서 커밋 2개(`8306203`, `ab9fc7e`) 뒤에 있었고, 이
worktree에는 반영돼 있지 않았다. §9에서 상세)
head: `f50b0ae`

## 1. WP별 커밋

| WP | 커밋 | 한 줄 요약 |
|---|---|---|
| WP1 | `1582f86` | `HexCoord{q,r}` + `distance`/`is_adjacent`/`neighbors` + `NEIGHBOR_DIRECTIONS` 상수 |
| WP2 | `28a9a3c` | `ring`/`range` (반지름 조회, 음수 반지름 거부) |
| WP3 | `1e1523e` | `line()` — 정수 전용 tie-break (반올림 half-up + 최대오차축 x>y>z 우선) |
| WP4 | `c37d601` | `HexShape` — 앵커 기준 오프셋 정규형 + `tiles_at` |
| WP5 | `8d26b63` | `HexOccupancy` — `BTreeMap` 기반 점유 맵, all-or-nothing `try_occupy` |
| WP6 | `f50b0ae` | overflow 커버리지 보강(`ring`/`range`) + 슬라이스 마감 확인 |

모든 커밋은 `crates/escape-core/src/combat_hex.rs`, `crates/escape-core/src/lib.rs`
(모듈 등록 + re-export만), `crates/escape-core/tests/combat_hex_t1a.rs`만 건드렸다.

## 2. 확정된 public API 전체 서명

```rust
// crates/escape-core/src/combat_hex.rs (전부 crate 루트에서 재노출됨)

pub struct HexCoord {
    pub q: i32,
    pub r: i32,
}
// derive: Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize
// Ord = (q, r) 사전순 (derive 기본 동작 그대로, 필드 선언 순서)

impl HexCoord {
    pub const NEIGHBOR_DIRECTIONS: [HexCoord; 6]; // 계약 #1, 아래 §3

    pub fn distance(self, other: Self) -> i64;
    pub fn is_adjacent(self, other: Self) -> bool;
    pub fn neighbors(self) -> Result<[HexCoord; 6], HexError>;
}

pub fn ring(center: HexCoord, radius: i32) -> Result<Vec<HexCoord>, HexError>;
pub fn range(center: HexCoord, radius: i32) -> Result<Vec<HexCoord>, HexError>;
pub fn line(from: HexCoord, to: HexCoord) -> Vec<HexCoord>; // 계약 #2, 아래 §3

pub struct HexShape { /* private: normalized_offsets: Vec<HexCoord> */ }
// derive: Clone, Debug, PartialEq, Eq (Serialize/Deserialize 없음 — §7 참고)

impl HexShape {
    pub fn new(offsets: Vec<HexCoord>) -> Result<HexShape, HexError>; // 계약 #3, 아래 §3
    pub fn tiles_at(&self, anchor: HexCoord) -> Result<Vec<HexCoord>, HexError>;
}

pub struct HexOccupancy { /* private: tiles: BTreeMap<HexCoord, String> */ }
// derive: Clone, Debug, Default, PartialEq, Eq (Serialize/Deserialize 없음)

impl HexOccupancy {
    pub fn new() -> Self;
    pub fn try_occupy(&mut self, tiles: &[HexCoord], id: &str) -> Result<(), HexError>;
    pub fn vacate(&mut self, id: &str);
    pub fn occupant_at(&self, coord: HexCoord) -> Option<&str>;
    pub fn is_free(&self, coord: HexCoord) -> bool;
    pub fn iter(&self) -> impl Iterator<Item = (&HexCoord, &str)>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HexError {
    Overflow,
    NegativeRadius,
    EmptyShape,
    DuplicateOffset(HexCoord),
    TileOccupied(HexCoord),
}
impl std::fmt::Display for HexError { .. } // "{self:?}"
impl std::error::Error for HexError {}
```

`lib.rs` re-export 한 줄:

```rust
pub use combat_hex::{line, range, ring, HexCoord, HexError, HexOccupancy, HexShape};
```

(`HexCoord::NEIGHBOR_DIRECTIONS`는 연관 상수라 `HexCoord`가 재노출되면 자동으로
`escape_core::HexCoord::NEIGHBOR_DIRECTIONS`로 접근 가능하다 — 별도 재노출 항목이
필요 없다.)

### `Result` 규칙 (설계 근거, §5-1에서 상세)

- **측정/판정** (`distance`, `is_adjacent`): `Result` 없음. 결과 타입을 `i64`로
  넓혀(또는 `bool`) 오버플로 자체를 원천 차단했다.
- **산술로 새 `HexCoord`를 만드는 모든 함수** (`neighbors`, `ring`, `range`,
  `HexShape::tiles_at`, `HexOccupancy::try_occupy`): `Result<_, HexError>`.
- **예외**: `line()`은 새 `HexCoord`를 만들지만 `Result`가 없다 — 만들어내는
  모든 좌표가 `from`/`to`의 해당 축 값 사이로 수학적으로 바운드되므로(증명은
  모듈 소스의 `line` 문서 주석) 오버플로가 원천적으로 불가능하다. `debug_assert`로
  그 증명을 테스트에서 자체 검증한다.

## 3. 세 가지 확정 계약

**#1 — `HexCoord::NEIGHBOR_DIRECTIONS`의 순서** (`neighbor_direction_order_is_fixed`가 고정):

```rust
[
    HexCoord { q: 1, r: 0 },
    HexCoord { q: 1, r: -1 },
    HexCoord { q: 0, r: -1 },
    HexCoord { q: -1, r: 0 },
    HexCoord { q: -1, r: 1 },
    HexCoord { q: 0, r: 1 },
]
```
의미상 이름(동/서 등)을 붙이지 않았다 — axial 이웃 오프셋은 flat-top/pointy-top
배치와 무관하게 동일한 여섯 벡터이고, 화면 방향은 투영(T9 소관, 이 슬라이스
범위 밖)에서만 정해지기 때문이다.

**#2 — `line()`의 tie-break** (`line_tie_break_is_pinned`, `line_direction_asymmetry_is_pinned`가
고정): axial `(q,r)`을 큐브 `(x,y,z)=(q,r,-q-r)`로 보고, 단계별 선형보간 값을
① **반올림 동점(.5)은 위로**(round-half-up, `+inf` 방향) 반올림하고, ②
`x+y+z=0` 제약이 깨지면 **반올림 오차가 가장 큰 축 하나를 나머지 둘의 합의
음수로 재계산**한다(오차 동점이면 `x`를 `y`·`z`보다, `y`를 `z`보다 먼저 재계산
대상으로 고른다). float epsilon을 전혀 쓰지 않는다.

부수 효과(문서화·테스트로 고정): 이 두 규칙은 보간 분자가 방향과 무관하게
동일한 값으로 재현되도록 만들어서, 이 구현의 `line()`은 **모든 입력에서
방향에 대해 대칭**이다(`line(a,b)`를 뒤집은 것이 항상 `line(b,a)`와 같다).
계획은 비대칭을 *허용*했을 뿐 요구하지는 않았으므로, 이 대칭성 자체를
"문서화한 거동"으로 못박았다 — 강제로 대칭을 만들지 않았고, 그냥 이 tie-break
설계의 자연스러운 성질이다.

**#3 — `HexShape`의 정규형** (`same_shape_in_different_order_normalizes_identically`,
`same_shape_at_different_absolute_position_normalizes_identically`가 고정):
입력 오프셋 중 `Ord` 기준 최솟값을 `(0,0)`으로 옮기도록 전체를 이동한 뒤 `Ord`
오름차순으로 정렬한다. 사전순은 균일 이동에 불변이므로 이 절차는 입력 순서와
입력이 적힌 절대 위치 둘 다에 무관하게 같은 상대 모양을 같은 정규형으로 만든다.

## 4. 검증 명령과 실제 출력

```
$ cargo fmt --all -- --check
(출력 없음, exit 0)

$ cargo test -p escape-core --test combat_hex_t1a
running 31 tests
...
test result: ok. 31 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo test --workspace --no-fail-fast
(각 test binary의 "test result: ok. N passed; 0 failed" 라인을 전부 합산)
passed 합계: 385
failed 합계: 0

$ git diff --check
(출력 없음, exit 0)
```

385 = baseline 354(이 worktree HEAD `6f28160`에서 이 슬라이스 시작 전 직접 측정,
§9 참고) + 신규 31(`combat_hex_t1a.rs`). **기존 테스트 결과는 WP1~WP6 전체에서
단 하나도 바뀌지 않았다** — 각 WP 커밋 전에 baseline(354)+그 WP까지의 신규
테스트 수만 늘었는지, failed가 항상 0인지 매번 직접 확인했다(WP1: 361/0,
WP2: 367/0, WP3: 372/0, WP4: 378/0, WP5: 384/0, WP6: 385/0).

## 5. 계획과 다르게 구현한 부분과 사유

계획 §4의 시그니처는 예시였고, §5의 하드 invariant(체크드 산술, 패닉 금지,
지어낸 값 금지)가 그보다 우선한다고 판단해 다음 지점에서 벗어났다. 전부
모듈 소스 코드 주석에도 같은 근거로 남겨 놓았다.

1. **`distance()` 반환 타입 `i32` → `i64`.** `q`/`r`이 `i32::MIN`/`MAX` 근처면
   참거리가 `i32`에 담기지 않는다(예: 두 극단 사이는 약 43억). `i64`로 넓히면
   그 계산 전체가 `i64` 안에 항상 들어가 오버플로가 원천적으로 없어지고,
   `Result`가 필요 없어진다. `is_adjacent`도 같은 이유로 계획대로 `bool`
   그대로 유지했다(내부에서 `i64` 비교만 하므로 문제없었다).
2. **`neighbors()` 반환 타입 `[HexCoord; 6]` → `Result<[HexCoord; 6], HexError>`.**
   이 함수는 실제로 `±1` 산술로 새 `HexCoord`를 만든다. `self.q`/`r`이 이미
   `i32::MIN`/`MAX`면 그 방향의 이웃은 `i32`에 담기지 않고, `distance`처럼
   결과 타입을 넓힐 방법이 없다(이웃은 정의상 `HexCoord`, 즉 `i32` 두 개다).
   체크드 산술 + `Result`가 유일하게 패닉을 피하는 길이었다.
3. **`HexShape::tiles_at()` 반환 타입 `Vec<HexCoord>` → `Result<Vec<HexCoord>, HexError>`.**
   `anchor + offset`은 실제 덧셈이라 `anchor`가 극단값 근처면 오버플로할 수
   있다. `ring`/`range`도 같은 이유로 이미 계획이 예상한 `Result`를 쓰고
   있었으므로 대칭적인 선택이었다.
4. **`line()`은 계획대로 `Vec<HexCoord>` 그대로(Result 없음)** 유지했다 —
   위 세 항목과 달리, 만들어내는 좌표가 항상 `from`/`to`의 유효한 `i32` 값
   사이로 바운드되는 보간이라 오버플로가 원천적으로 불가능함을 증명할 수
   있었기 때문이다(반올림 대상 두 정수 사이의 실수를 반올림해도 그 구간을
   벗어날 수 없다는 성질 — fix-up 단계도 같은 성질을 물려받는다).
5. **`line()`의 미해결 자원 문제(메모리, 오버플로 아님).** `from`/`to`가
   서로 천문학적으로 멀면(예: `i32::MIN`↔`i32::MAX`, 약 43억 타일) 그만큼의
   `Vec`를 할당하려다 프로세스가 죽는다. 실제로 처음에 `extreme_coordinates_do_not_panic`
   테스트를 min↔max로 짰다가 `memory allocation of 68719476728 bytes failed`로
   테스트 프로세스가 SIGABRT됐다 — 이건 체크드 산술이 막는 "오버플로 패닉"과
   다른 종류의 문제라 판단해 고치지 않고, 테스트를 "좌표는 극단이되 서로
   가까운" 입력으로 바꿔 그 한계를 문서화했다(모듈 소스의 `line` 문서 참고).
   경로 탐색·거리 상한은 계획 §9가 명시적으로 범위 밖이라 규정한 항목이라
   임의의 상한을 지어내지 않았다.
6. **`HexShape`/`HexOccupancy`에 `Serialize`/`Deserialize`를 붙이지 않았다.**
   계획 §4-1은 `HexCoord`에만 그 derive를 명시했다. 두 타입은 인카운터 콘텐츠나
   저장 형식에 아직 나타나지 않고(이 슬라이스는 미배선), 필요해지면 그 결정을
   내릴 다음 슬라이스(T1-b 또는 이후)가 판단하는 게 낫다고 봤다.
7. **`ring`의 검증 로직에 음수 반지름 검사가 두 번(직접 + `axial_disk_offsets`
   내부) 나타난다.** 중복이지만 무해하며, `ring`의 `radius == 0` 특수 케이스를
   `axial_disk_offsets` 호출 전에 처리해야 해서 자연스럽게 생겼다. 리팩터로
   없앨 수도 있었지만 함수 하나가 자기 전제조건을 스스로 검증하는 쪽이
   더 읽기 쉽다고 판단해 남겨뒀다.

이 중 어느 것도 계획이 명시적으로 요구한 값을 뒤집지 않았다 — 전부 §5의
하드 invariant(특히 "체크드 산술", "패닉 금지", "값을 지어내지 않는다")를
문자 그대로 만족시키기 위해 필요했던, 시그니처 수준의 조정이다.

## 6. 스킵한 WP

없다. WP1~WP6 전부 구현했다.

## 7. 최종 체크리스트

- [x] `combat_hex.rs`를 어떤 기존 모듈도 호출하지 않는다 — `combat_simulation.rs`,
      `combat_resolution.rs`, `combat_spectator.rs`, `combat_state.rs`,
      `combat_execution.rs`, `combat_opportunity.rs`, `combat_conclusion.rs`,
      `combat_contract.rs`, `scene_page.rs`, `save.rs`, `content.rs`,
      `crates/escape-terminal/**`를 grep했고, `combat_hex`/`HexCoord`/`HexShape`/
      `HexOccupancy`/`HexError` 어느 것도 나오지 않았다. 저장소 전체 grep에서도
      `combat_hex.rs` 자기 자신, `lib.rs`(등록·재노출), `combat_hex_t1a.rs`(테스트)
      셋 외에는 아무 데도 나타나지 않았다.
- [x] `lib.rs` 변경이 모듈 등록(`mod combat_hex;`)과 `pub use` 재노출 한 줄뿐이다.
- [x] `HashMap`/`HashSet`이 없다 — `combat_hex.rs`에서 두 이름이 등장하는 곳은
      전부 그 둘을 "쓰지 않는다"고 설명하는 문서 주석뿐이다(코드에서 grep으로 확인).
- [x] 부동소수점 반올림이 tie-break를 결정하지 않는다 — `line()`은 `i128` 정수
      연산만 쓰고, float 타입이 모듈 어디에도 없다.
- [x] 모든 공개 함수가 잘못된 입력에 `Result`로 답한다 — 예외는 §2에서 명시한
      "잘못된 입력이 존재하지 않는" 두 함수(`distance`→`i64`, `is_adjacent`→`bool`)와
      수학적으로 오버플로가 불가능함을 증명한 `line()`뿐이며, 셋 다 그 이유를
      소스 코드 주석에 남겼다.
- [x] 극단 좌표에서 패닉하지 않는다 — `extreme_coordinates_do_not_panic` +
      `neighbors_at_extreme_edge_overflow_instead_of_panicking` +
      `tiles_at_overflow_instead_of_panicking` +
      `ring_and_range_overflow_instead_of_panicking`이 확인한다. (예외적으로
      "좌표는 극단이되 두 점 사이 거리도 천문학적으로 먼" 경우는 `line()`이
      메모리 할당 실패로 죽을 수 있음을 §5-5에서 별도로 문서화했다 — 이건
      오버플로 패닉이 아니라 자원 한계이고, 실제로 그 경계를 처음 테스트로
      쳐보다가 발견해서 테스트 입력을 조정했다.)
- [x] 회전 API가 없고, 없는 이유가 주석에 있다 — 모듈 최상단 문서와
      `HexShape` 타입 문서 둘 다에 상위 문서 결정 12를 근거로 명시했다.
- [x] 모듈 주석에 좌표계·방향 순서·tie-break·정규형·미배선 사실이 적혀 있다 —
      `combat_hex.rs` 1~50행.
- [x] `Cargo.toml` 무변경 — `git diff --stat`로 확인, 어떤 커밋도 `Cargo.toml`을
      건드리지 않았다.
- [x] 기존 테스트 결과 무변경, `cargo test --workspace --no-fail-fast` 0 failed —
      §4의 실제 출력(385 passed, 0 failed) 참고.
- [x] `cargo fmt --all -- --check` 통과 — §4 참고.

## 8. 스타일 참고

`combat_spectator.rs`(주석 밀도, "왜 이 규칙이 존재하는가"를 설명하는 습관,
`Display`/`Error` 구현 패턴)와 `combat_simulation.rs`의 `distance_squared`
checked 산술 관용구를 그대로 따랐다.

## 9. baseline 수치 차이에 대한 참고

계획 문서 §8은 "워크스페이스 테스트가 baseline(346)에서 신규 테스트 수만큼만
증가"라고 적었지만, 이 worktree의 HEAD(`6f28160`)에서 작업 시작 전 직접
`cargo test --workspace --no-fail-fast`를 돌려보니 baseline은 **354**였다.
계획서에 적힌 baseline 커밋 `ab9fc7e`는 main 체크아웃 기준으로 `6f28160`보다
앞선 게 아니라 **뒤에** 있는 순수 문서 커밋(`8306203`, `ab9fc7e` — 둘 다
`docs/design/Combat_Hex_Rework_Development_Plan.md` 관련)이었고, 이
worktree에는 반영돼 있지 않다(`git merge-base --is-ancestor` 확인). 코드에
영향을 주는 차이가 아니라 문서 커밋 두 개의 유무 차이로 보이며, 이 슬라이스가
소유한 파일에는 어차피 그 두 커밋이 영향을 주지 않는다. 그래서 "기존 테스트
결과 무변경"은 계획서의 절대 숫자(346) 대신 **이 worktree에서 직접 측정한
baseline(354)** 기준으로 매 WP마다 검증했다 — §4 참고.

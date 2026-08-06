# T1-a — 육각 좌표 수학 모듈 (미배선) — 완료 보고

plan: `fable_combat_hex_t1a_step1_2608061847.md`
baseline: `6f28160` (이 worktree의 HEAD — 계획서에 적힌 `ab9fc7e`는 main 체크아웃에서
이 슬라이스와 무관한 후속 문서 커밋 2개(`8306203`, `ab9fc7e`) 뒤에 있었고, 이
worktree에는 반영돼 있지 않았다. §9에서 상세)
head: `e5cfa66`

**개정.** 코디네이터가 이 보고서 초안(당시 head `f50b0ae`)을 직접 리뷰하고 두
가지 결함을 지적했다. 둘 다 고쳤다 — §10에 커밋과 사유를 적었고, 아래 §2/§3의
public API·계약 서술은 **수정 후 최종 상태**를 반영한다.

## 1. WP별 커밋

| WP | 커밋 | 한 줄 요약 |
|---|---|---|
| WP1 | `1582f86` | `HexCoord{q,r}` + `distance`/`is_adjacent`/`neighbors` + `NEIGHBOR_DIRECTIONS` 상수 |
| WP2 | `28a9a3c` | `ring`/`range` (반지름 조회, 음수 반지름 거부) |
| WP3 | `1e1523e` | `line()` — 정수 전용 tie-break (반올림 half-up + 최대오차축 x>y>z 우선) |
| WP4 | `c37d601` | `HexShape` — 앵커 기준 오프셋 정규형 + `tiles_at` |
| WP5 | `8d26b63` | `HexOccupancy` — `BTreeMap` 기반 점유 맵, all-or-nothing `try_occupy` |
| WP6 | `f50b0ae` | overflow 커버리지 보강(`ring`/`range`) + 슬라이스 마감 확인 |
| fix 1 | `6fd0511` | `HexShape` 정규형을 "정렬만"으로 수정 — 앵커 이동 제거 (§10) |
| fix 2 | `e5cfa66` | `line()`을 `Result`로 바꿔 43억 타일급 입력의 프로세스 abort를 명시적 에러로 (§10) |

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

/// `line()`이 반환할 수 있는 최대 타일 수. 메모리 안전 레일이며 게임 규칙이
/// 아니다 — 어떤 판정도 이 값을 근거로 삼지 않는다. (fix 2, §10)
pub const MAX_LINE_LENGTH: i64 = 65_536;
pub fn line(from: HexCoord, to: HexCoord) -> Result<Vec<HexCoord>, HexError>; // 계약 #2, 아래 §3

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
    PathTooLong(i64), // fix 2, §10 — line()이 MAX_LINE_LENGTH를 넘는 거리를 받았다
}
impl std::fmt::Display for HexError { .. } // "{self:?}"
impl std::error::Error for HexError {}
```

`lib.rs` re-export 한 줄:

```rust
pub use combat_hex::{
    line, range, ring, HexCoord, HexError, HexOccupancy, HexShape, MAX_LINE_LENGTH,
};
```

(`HexCoord::NEIGHBOR_DIRECTIONS`는 연관 상수라 `HexCoord`가 재노출되면 자동으로
`escape_core::HexCoord::NEIGHBOR_DIRECTIONS`로 접근 가능하다 — 별도 재노출 항목이
필요 없다.)

### `Result` 규칙 (설계 근거, §5-1에서 상세)

- **측정/판정** (`distance`, `is_adjacent`): `Result` 없음. 결과 타입을 `i64`로
  넓혀(또는 `bool`) 오버플로 자체를 원천 차단했다.
- **산술로 새 `HexCoord`를 만드는 모든 함수** (`neighbors`, `ring`, `range`,
  `HexShape::tiles_at`, `HexOccupancy::try_occupy`): `Result<_, HexError>`.
- **`line()`도 `Result`다 — 단, 이유가 다르다(fix 2, §10).** 만들어내는 좌표
  하나하나는 여전히 오버플로가 불가능하다(그 증명은 그대로 유효하고, `debug_assert`가
  테스트에서 자체 검증한다). `Result`가 필요해진 것은 반환 벡터의 **길이**
  때문이다 — `from`/`to`가 서로 아주 멀면 그만큼의 `Vec`를 할당하려다 프로세스가
  abort한다(패닉이 아니라 abort라 catch가 안 된다). `MAX_LINE_LENGTH`를 넘는
  거리는 좌표를 하나도 계산하기 전에 `HexError::PathTooLong`으로 거부한다.
  최초 제출본은 이걸 `Vec<HexCoord>`(Result 없음)로 남기고 문서 주석에만
  "메모리 한계가 있다"고 적어뒀는데, 코디네이터가 정확히 이 지점을 짚었다:
  문서화는 안전이 아니고, 이 함수는 `lib.rs`에서 재노출되는 public API다.

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
`translated_shape_is_not_the_same_shape`가 고정) — **정렬만 한다. 이동하지
않는다.** 입력 오프셋을 `Ord` 오름차순으로 정렬하는 것 외에는 아무것도 하지
않는다. `(0,0)`을 오프셋 집합에 강제로 넣지 않으므로, 오프셋 집합을 통째로
옮겨 적은 두 `HexShape`는 (같은 상대 모양이라도) **서로 다른 값**이다 — 앵커가
모양의 어디에 있는지에 대해 다른 것을 말하고 있기 때문이다.

**이 계약은 개정됐다(fix 1, §10).** 최초 제출본은 `Ord` 최솟값을 원점으로
옮기는 이동을 추가로 했었다. 그건 틀렸다 — 이동은 앵커의 위치 정보를 지운다.
중심+인접 6칸의 "꽃" 모양은 `Ord` 최솟값이 `(-1,0)`이라 이동을 하면
`tiles_at(A)`가 꽃의 중심을 `A`가 아니라 `A+(1,0)`에 놓았다. 계획이 실제로
요구한 성질("같은 모양을 다른 순서로 넣어도 같은 정규형")은 정렬만으로 충족되고,
이동은 애초에 필요 없었다.

## 4. 검증 명령과 실제 출력 (fix 1 + fix 2 반영, 최종 상태 `e5cfa66`)

```
$ cargo fmt --all -- --check
(출력 없음, exit 0)

$ cargo test -p escape-core --test combat_hex_t1a
running 32 tests
...
test result: ok. 32 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

$ cargo test --workspace --no-fail-fast
(각 test binary의 "test result: ok. N passed; 0 failed" 라인을 전부 합산)
passed 합계: 386
failed 합계: 0

$ git diff --check
(출력 없음, exit 0)
```

386 = baseline 354(이 worktree HEAD `6f28160`에서 이 슬라이스 시작 전 직접 측정,
§9 참고) + 신규 32(`combat_hex_t1a.rs`). **기존 테스트 결과는 WP1~WP6과 두
fix 전체에서 단 하나도 바뀌지 않았다** — 매 단계 전에 baseline(354)+그때까지의
신규 테스트 수만 늘었는지, failed가 항상 0인지 직접 확인했다.

| 단계 | passed | failed | 신규 테스트 수 |
|---|---|---|---|
| WP1 | 361 | 0 | 7 |
| WP2 | 367 | 0 | 13 |
| WP3 | 372 | 0 | 18 |
| WP4 | 378 | 0 | 24 |
| WP5 | 384 | 0 | 30 |
| WP6 | 385 | 0 | 31 |
| fix 1 (`6fd0511`) | 385 | 0 | 31 (순증 0 — 테스트 1개를 교체) |
| fix 2 (`e5cfa66`) | 386 | 0 | 32 (`line_over_the_length_rail_is_rejected` 추가) |

fix 1은 순수 교체(`same_shape_at_different_absolute_position_normalizes_identically`
제거, `translated_shape_is_not_the_same_shape` 추가)라 총계가 그대로 385였다.
fix 2에서 신규 테스트 1개(`line_over_the_length_rail_is_rejected`)가 더해져 386이 됐다.

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
4. **`line()`도 `Result<Vec<HexCoord>, HexError>`다 (fix 2로 확정, §10).**
   개별 좌표의 오버플로 불가능성 증명은 여전히 유효하다(반올림 대상 두 정수
   사이의 실수를 반올림해도 그 구간을 벗어날 수 없다는 성질 — fix-up 단계도
   같은 성질을 물려받는다). 하지만 반환 벡터의 **길이**는 그 증명이 덮지
   못하는 별개 문제라, 결국 `Result`가 필요했다. 최초 제출본은 이걸
   `Vec<HexCoord>`로 남기고 위험을 문서 주석에만 적어뒀는데, 코디네이터가
   "문서화는 안전이 아니다, 이 함수는 public API다"라고 짚었다 — §10에 상세.
5. **`HexShape`/`HexOccupancy`에 `Serialize`/`Deserialize`를 붙이지 않았다.**
   계획 §4-1은 `HexCoord`에만 그 derive를 명시했다. 두 타입은 인카운터 콘텐츠나
   저장 형식에 아직 나타나지 않고(이 슬라이스는 미배선), 필요해지면 그 결정을
   내릴 다음 슬라이스(T1-b 또는 이후)가 판단하는 게 낫다고 봤다.
6. **`ring`의 검증 로직에 음수 반지름 검사가 두 번(직접 + `axial_disk_offsets`
   내부) 나타난다.** 중복이지만 무해하며, `ring`의 `radius == 0` 특수 케이스를
   `axial_disk_offsets` 호출 전에 처리해야 해서 자연스럽게 생겼다. 리팩터로
   없앨 수도 있었지만 함수 하나가 자기 전제조건을 스스로 검증하는 쪽이
   더 읽기 쉽다고 판단해 남겨뒀다.
7. **`HexShape` 정규형이 fix 1에서 바뀌었다(§10).** 최초 제출본은 "정렬 +
   앵커 기준 이동"의 "이동" 부분을 "`Ord` 최솟값을 원점으로 옮긴다"로 읽고
   구현했다. 이건 계획 문구를 잘못 읽은 것이었고, 코디네이터가 지적한 뒤
   "정렬만" 하는 것으로 고쳤다 — 세부는 §3의 계약 #3과 §10을 본다.

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
- [x] 모든 공개 함수가 잘못된 입력에 `Result`로 답한다 — 유일한 예외는 §2에서
      명시한 "잘못된 입력이 존재하지 않는" 두 함수(`distance`→`i64`,
      `is_adjacent`→`bool`)뿐이며, 그 이유를 소스 코드 주석에 남겼다. `line()`은
      fix 2 이후 더 이상 예외가 아니다 — `Result`를 반환한다.
- [x] 극단 좌표에서 패닉하지 않는다 — `extreme_coordinates_do_not_panic` +
      `neighbors_at_extreme_edge_overflow_instead_of_panicking` +
      `tiles_at_overflow_instead_of_panicking` +
      `ring_and_range_overflow_instead_of_panicking` +
      `line_over_the_length_rail_is_rejected`가 확인한다. fix 2 이전에는 "좌표는
      극단이되 두 점 사이 거리도 천문학적으로 먼" 경우(`line(min, max)`)가
      메모리 할당 실패로 프로세스를 abort시켰고, `extreme_coordinates_do_not_panic`은
      그 입력을 피해서 이름이 보장하지 않는 걸 보장하는 척하고 있었다 —
      코디네이터가 지적한 뒤 그 테스트가 실제로 `line(min, max)`을 호출하고
      에러를 확인하도록 고쳤다(§10).
- [x] 회전 API가 없고, 없는 이유가 주석에 있다 — 모듈 최상단 문서와
      `HexShape` 타입 문서 둘 다에 상위 문서 결정 12를 근거로 명시했다.
- [x] 모듈 주석에 좌표계·방향 순서·tie-break·정규형·미배선 사실이 적혀 있다 —
      `combat_hex.rs` 상단 doc 주석.
- [x] `Cargo.toml` 무변경 — `git diff --stat`로 확인, 어떤 커밋도 `Cargo.toml`을
      건드리지 않았다.
- [x] 기존 테스트 결과 무변경, `cargo test --workspace --no-fail-fast` 0 failed —
      §4의 실제 출력(386 passed, 0 failed) 참고.
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

## 10. 코디네이터 리뷰로 고친 두 가지

코디네이터가 초안(head `f50b0ae`)을 직접 검증하고 나머지는 다 통과시켰지만
두 곳을 짚었다. 둘 다 별도 커밋으로 고쳤다.

### fix 1 — `HexShape` 정규형이 앵커 위치를 지웠다 (`6fd0511`)

계획 문구 "정렬 + 앵커 기준 이동"의 "이동"을 "`Ord` 최솟값을 원점(0,0)으로
옮긴다"로 읽고 구현했다. 이 읽기가 틀렸다는 게 코디네이터의 지적이었다(그리고
스스로도 계획 문구 자체가 그 읽기를 유도했다고 짚었다 — 내 잘못만은 아니라고
했지만, 구현한 건 나다).

문제: 이동은 앵커가 모양의 어디에 있는지에 대한 저자의 정보를 지운다. 예시
(코디네이터가 든 것과 같음): 중심+인접 6칸의 "꽃" 모양, 오프셋
`[(0,0),(1,0),(1,-1),(0,-1),(-1,0),(-1,1),(0,1)]`. `Ord` 최솟값은 `(-1,0)`이라,
이동을 하면 `tiles_at(A)`가 꽃의 중심을 `A`가 아니라 `A+(1,0)`에 놓는다 — 대칭
모양조차 자기 중심을 앵커에 놓을 수 없었다. 대형 유닛(보스 등)의 위치를
`A`라고 기록하는 시스템에서, 실제 점유 타일 중심이 `A`에서 벗어난다는 뜻이다.

계획이 실제로 요구한 성질("같은 모양을 다른 순서로 넣어도 같은 정규형")은
정렬만으로 충족된다는 걸 재확인하고, `HexShape::new`를 정렬만 하도록 고쳤다.
이동에만 쓰이던 `checked_sub` helper는 완전히 쓸모가 없어져서 함께 제거했다.
테스트는 `same_shape_at_different_absolute_position_normalizes_identically`
(더 이상 참이 아닌 성질을 검사하고 있었다)를 `translated_shape_is_not_the_same_shape`로
교체했다 — 정반대 성질(이동된 모양은 다른 값이다)을 고정한다.
`same_shape_in_different_order_normalizes_identically`는 그대로 유효해서 손대지 않았다.

`HexShape` 타입 문서 안에 이 결정의 이유를 직접 남겼다 — 다음 구현자가
"정리"랍시고 이동을 되살리지 않도록.

### fix 2 — `line()`이 문서화만으로 안전을 대체하려 했다 (`e5cfa66`)

`line()`은 반환 벡터 길이가 `distance(from,to)+1`이라, `from`/`to`가 서로
천문학적으로 멀면(예: `i32::MIN`↔`i32::MAX`, 약 43억) 그만큼의 `Vec`를
할당하려다 프로세스가 **abort**한다. 최초 제출본은 이걸 실제로 재현해서
발견했고(`extreme_coordinates_do_not_panic`을 min↔max로 처음 짰다가
`memory allocation of 68719476728 bytes failed`로 테스트 프로세스가 죽는 걸
직접 봤다), 함수 문서 주석에 "이건 오버플로가 아니라 자원 한계다, 범위 밖이다"
라고 적어두고, 테스트 입력을 극단이되 서로 가까운 값으로 바꿔서 우회했다.

코디네이터의 지적: 문서화는 안전을 대체하지 않는다. abort는 패닉보다 나쁘다
(catch가 안 된다). `line()`은 `lib.rs`에서 재노출되는 public API다. 그리고
결정적으로 — `extreme_coordinates_do_not_panic`이라는 이름을 단 테스트가
정확히 그 극단 입력을 피해서 통과하고 있었다. 이름이 보장하지 않는 걸
보장하는 척하는 셈이었다.

고친 내용:
- `line()` 반환 타입을 `Result<Vec<HexCoord>, HexError>`로 바꿨다.
- `MAX_LINE_LENGTH: i64 = 65_536` 상수를 추가했다. 게임 규칙이 아니라 메모리
  안전 레일이라고 명시적으로 문서화했다 — 어떤 판정도 이 숫자를 근거로
  삼아서는 안 된다(전투 보드는 이보다 세 자릿수 이상 작다).
  `steps + 1 > MAX_LINE_LENGTH`면 좌표를 하나도 계산하기 전에
  `HexError::PathTooLong(distance)`로 거부한다.
- `extreme_coordinates_do_not_panic`이 이제 실제로 `line(min, max)`을 호출하고
  `Err`인지 확인한다.
- `line_over_the_length_rail_is_rejected`를 새로 추가해 레일을 넘는 경우의
  정확한 에러와, 레일에 정확히 걸치는 경우(여전히 성공해야 함)의 경계를 둘 다
  고정했다.

tie-break 규칙과 그 테스트(`line_tie_break_is_pinned`,
`line_direction_asymmetry_is_pinned`)는 건드리지 않았다 — 바깥쪽 `Result`
포장만 바뀌었을 뿐, 보간·반올림 로직은 그대로다.

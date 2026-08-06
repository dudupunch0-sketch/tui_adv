//! 육각 좌표 수학 — **순수 모듈, 아직 배선되지 않았다.**
//!
//! 이 모듈은 지금 어떤 기존 모듈에서도 호출되지 않는다. `CombatPosition`을 대체하지
//! 않고, JSON 경계를 건드리지 않으며, 시뮬레이션 거동에 영향을 주지 않는다. 배선은
//! T1-b(`fable_combat_hex_t1a_step1_2608061847.md`가 예약한 다음 슬라이스)가 한다.
//! T1-b는 여기서 확정한 public API와 테스트를 보고 계획을 쓴다 — 그러니 여기서 정한
//! 것은 전부 계약이다.
//!
//! ## 좌표계
//!
//! **axial `(q, r)`**, 배치는 **flat-top**이다(레퍼런스 실측과 일치, 상위 문서 §1).
//! 축 좌표 수학 자체는 flat-top/pointy-top 어느 배치에서도 동일하다 — 화면에 투영할
//! 때만 배치가 달라지고, 그 투영은 이 모듈의 범위 밖이다(T9가 소유, "고정 타일
//! 메트릭 투영").
//!
//! ## 세 가지 확정 계약
//!
//! 이후 슬라이스가 의존할 세 가지를 여기서 못박는다.
//!
//! 1. **`HexCoord::NEIGHBOR_DIRECTIONS`의 순서.** 아래 상수 배열 순서 그대로가 계약이다.
//!    경로 탐색, 포위 판정, 타일 배정 해소가 이 순서에 의존하게 된다. 의미상 이름
//!    (동/서/남/북 등)은 붙이지 않는다 — 화면 방향은 투영에 의존하고 투영은 이
//!    모듈의 범위 밖이다.
//! 2. **`line()`의 tie-break.** [`line`]의 문서를 본다. 부동소수점 epsilon에 의존하지
//!    않는 정수 규칙이다.
//! 3. **`HexShape`의 정규형.** [`HexShape::new`]의 문서를 본다.
//!
//! ## 회전이 없다
//!
//! 이 모듈에는 회전 API가 없고, 앞으로도 추가하지 않는다. 상위 문서 결정 12
//! ("대형 유닛은 회전 없음, 점유 형태 고정")가 근거다. 대형 유닛의 형태는
//! [`HexShape`]가 담는 고정 오프셋 집합이며, 좌우가 마주 보는 배치에서 비대칭 형태가
//! 같은 방향을 향하는 것은 콘텐츠 가이드(대칭 형태를 쓰라는 안내)로 해소한다.
//! 다음 구현자는 여기에 회전 helper를 추가하지 말 것 — 필요해지면 그것은 새로운
//! 결정이고 새로운 계획서가 필요하다.
//!
//! ## 결정론
//!
//! `HashMap`/`HashSet`을 쓰지 않는다(`BTreeMap`만, [`HexOccupancy`] 참고). 반환하는
//! 모든 컬렉션은 [`HexCoord`]의 `Ord`(=(q,r) 사전순, 아래 참고) 기준 오름차순으로
//! 정렬돼 있거나, 순서가 상수로 고정·문서화돼 있다. RNG 호출은 0회다.
//!
//! ## 오버플로와 패닉
//!
//! 모든 산술은 checked다(`combat_simulation.rs`의 `distance_squared` 관용구를 따른다).
//! 규칙: **새 [`HexCoord`]를 산술로 만들어내는 함수는 `Result`를 반환한다** —
//! `i32` 범위를 벗어나면 값을 지어내거나 자르지 않고 [`HexError::Overflow`]로
//! 거부한다. 측정·판정만 하는 함수([`HexCoord::distance`], [`HexCoord::is_adjacent`])는
//! 결과 타입을 넓혀(`i64`) 애초에 오버플로가 발생하지 않게 만들었으므로 `Result`가
//! 필요 없다 — 이 두 부류를 섞지 않는다.

use serde::{Deserialize, Serialize};

/// 육각 타일 하나의 axial 좌표.
///
/// **`Ord`는 장식이 아니라 결정론 요구사항이다.** 정렬 기준은 **`(q, r)` 사전순**이다
/// (derive된 `Ord`가 필드 선언 순서 그대로, 즉 `q`를 먼저 비교하고 같으면 `r`을
/// 비교한다 — 이는 derive의 기본 동작이며 별도 구현이 없다). [`ring`]/[`range`]/
/// [`HexShape::tiles_at`]/[`HexOccupancy::iter`]가 이 순서로 정렬해 반환한다.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct HexCoord {
    pub q: i32,
    pub r: i32,
}

impl HexCoord {
    /// 이웃 방향 오프셋. **순서가 계약이다** — 이후 어떤 소비자도 이 순서를
    /// 인덱스로 참조할 수 있으므로 바꾸지 않는다.
    ///
    /// axial 좌표에서 이웃 오프셋은 배치(flat-top/pointy-top)와 무관하게 동일한
    /// 여섯 벡터다 — 배치는 화면 투영에서만 달라진다. 그래서 이 배열에 방향의
    /// 의미상 이름(동/서 등)을 붙이지 않는다: 그 이름은 투영을 정하는 T9의 몫이다.
    pub const NEIGHBOR_DIRECTIONS: [HexCoord; 6] = [
        HexCoord { q: 1, r: 0 },
        HexCoord { q: 1, r: -1 },
        HexCoord { q: 0, r: -1 },
        HexCoord { q: -1, r: 0 },
        HexCoord { q: -1, r: 1 },
        HexCoord { q: 0, r: 1 },
    ];

    /// 두 타일 사이의 axial 거리.
    ///
    /// 공식은 큐브 좌표 거리와 동치다: `max(|dq|, |dr|, |dq+dr|)`
    /// (흔히 쓰는 `(|dq|+|dr|+|dq+dr|)/2` 형태와 같은 값이지만, 나눗셈이 없어
    /// 정수 반올림 문제가 원천적으로 없다).
    ///
    /// 반환 타입이 `i32`가 아니라 `i64`인 것은 **계획 문서의 시그니처에서
    /// 의도적으로 벗어난 지점**이다: `q`/`r`이 `i32::MIN`/`MAX` 근처면 참거리가
    /// `i32`에 담기지 않을 수 있다(예: `q=i32::MIN`과 `q=i32::MAX` 사이는 약 43억).
    /// `i64`로 넓히면 `i32` 두 값의 차이·합·절댓값이 절대 `i64`를 넘지 않으므로
    /// (최대 폭이 약 2^33, `i64` 한계는 2^63) 오버플로가 원천적으로 불가능해지고,
    /// `Result`로 감쌀 필요가 없어진다. 이 함수는 "잘못된 입력"이 존재하지 않는
    /// 순수 측정이므로 이 넓히기가 `Result` 없이도 무결성을 보장하는 올바른 선택이다.
    pub fn distance(self, other: Self) -> i64 {
        let (dq, dr) = Self::delta_i64(self, other);
        axial_distance_i64(dq, dr)
    }

    /// `self`와 `other`가 정확히 거리 1인가(=인접 6칸 중 하나인가).
    ///
    /// [`distance`](Self::distance)와 마찬가지로 `i64`에서 비교하므로 극단
    /// 좌표에서도 패닉하지 않는다.
    pub fn is_adjacent(self, other: Self) -> bool {
        let (dq, dr) = Self::delta_i64(self, other);
        axial_distance_i64(dq, dr) == 1
    }

    /// 인접 6칸. **`NEIGHBOR_DIRECTIONS`의 순서 그대로** 반환한다(이것이 그 상수의
    /// 유일한 소비 지점이지만, 이후 슬라이스는 `HexCoord::NEIGHBOR_DIRECTIONS`를
    /// 직접 참조할 수도 있다 — 그래서 상수 자체도 `pub`이다).
    ///
    /// 반환 타입이 계획 문서의 `[HexCoord; 6]`이 아니라 `Result<[HexCoord; 6],
    /// HexError>`인 것은 **의도적 이탈**이다: 이 함수는 실제로 새 `HexCoord`를
    /// 산술(±1)로 만들어낸다. `self.q`나 `self.r`이 이미 `i32::MIN`/`MAX`면 그
    /// 방향의 이웃은 `i32`에 담기지 않는다 — [`distance`](Self::distance)처럼
    /// 결과 타입을 넓힐 수 없다(이웃은 `HexCoord`, 즉 `i32` 두 개여야 하므로). 그래서
    /// checked 산술 + `Result`가 유일하게 패닉을 피하는 길이다.
    pub fn neighbors(self) -> Result<[HexCoord; 6], HexError> {
        let mut out = [HexCoord { q: 0, r: 0 }; 6];
        for (slot, direction) in out.iter_mut().zip(Self::NEIGHBOR_DIRECTIONS.iter()) {
            *slot = self.checked_add(*direction)?;
        }
        Ok(out)
    }

    /// checked 벡터 덧셈. 모듈 내부 전용 — 오버플로 시 [`HexError::Overflow`].
    fn checked_add(self, delta: HexCoord) -> Result<HexCoord, HexError> {
        let q = self.q.checked_add(delta.q).ok_or(HexError::Overflow)?;
        let r = self.r.checked_add(delta.r).ok_or(HexError::Overflow)?;
        Ok(HexCoord { q, r })
    }

    /// checked 벡터 뺄셈.
    fn checked_sub(self, other: HexCoord) -> Result<HexCoord, HexError> {
        let q = self.q.checked_sub(other.q).ok_or(HexError::Overflow)?;
        let r = self.r.checked_sub(other.r).ok_or(HexError::Overflow)?;
        Ok(HexCoord { q, r })
    }

    /// `self - other`를 `i64`로. `i32` 두 값의 차는 `i64`에서 절대 오버플로하지
    /// 않으므로(폭이 최대 2^32) checked가 필요 없다.
    fn delta_i64(self, other: Self) -> (i64, i64) {
        (
            i64::from(self.q) - i64::from(other.q),
            i64::from(self.r) - i64::from(other.r),
        )
    }
}

/// axial 거리 공식의 `i64` 코어. `dq`/`dr`이 [`HexCoord::delta_i64`]에서 온 값이면
/// (즉 `i32` 두 값의 차이면) `dq+dr`의 절댓값도 `i64`를 넘지 않는다.
fn axial_distance_i64(dq: i64, dr: i64) -> i64 {
    dq.abs().max(dr.abs()).max((dq + dr).abs())
}

/// `center`에서 `dq`/`dr`만큼 떨어진 실제 좌표를 만든다. checked — `center`가
/// 극단값 근처이거나 오프셋이 너무 크면 `i32`를 벗어날 수 있으므로 [`HexError::Overflow`].
fn checked_offset(base: i32, delta: i64) -> Result<i32, HexError> {
    i32::try_from(i64::from(base) + delta).map_err(|_| HexError::Overflow)
}

/// `center + (dq, dr)` 목록을 실제 [`HexCoord`] 목록으로 옮기고 `Ord` 오름차순으로
/// 정렬한다. [`ring`]/[`range`]가 공유하는 마지막 단계.
fn coords_from_offsets(
    center: HexCoord,
    offsets: &[(i64, i64)],
) -> Result<Vec<HexCoord>, HexError> {
    let mut tiles = Vec::with_capacity(offsets.len());
    for &(dq, dr) in offsets {
        tiles.push(HexCoord {
            q: checked_offset(center.q, dq)?,
            r: checked_offset(center.r, dr)?,
        });
    }
    tiles.sort();
    Ok(tiles)
}

/// 반지름 `radius` 이내(경계 포함)의 axial 오프셋 `(dq, dr)` 전부를 생성한다.
///
/// 전부 `i64`로 계산한다 — `radius`가 `i32` 안에서 크더라도(예: `i32::MAX`에
/// 가까운 값) 경계 계산(`-dq - radius` 등)이 중간에 `i32`를 벗어날 수 있는데,
/// `i64`로 넓히면 `i32` 범위의 `radius` 하나로 만들 수 있는 이 계산 전체가
/// `i64` 안에 항상 들어간다(최대 폭이 `radius`의 약 2배, `i32::MAX`의 2배는
/// `i64`에 비하면 미미하다). 실제 좌표로 옮기는 마지막 단계([`coords_from_offsets`])
/// 에서만 `i32` 오버플로 여부를 checked로 판정한다.
fn axial_disk_offsets(radius: i32) -> Result<Vec<(i64, i64)>, HexError> {
    if radius < 0 {
        return Err(HexError::NegativeRadius);
    }
    let radius = i64::from(radius);
    let mut offsets = Vec::new();
    for dq in -radius..=radius {
        let r_min = (-radius).max(-dq - radius);
        let r_max = radius.min(-dq + radius);
        for dr in r_min..=r_max {
            offsets.push((dq, dr));
        }
    }
    Ok(offsets)
}

/// 반지름 `radius`의 테두리(정확히 그 거리에 있는 타일들). `radius == 0`이면
/// `[center]` 하나뿐이다. 음수 `radius`는 빈 벡터가 아니라 [`HexError::NegativeRadius`]다
/// — "반지름이 없다"를 빈 목록으로 지어내지 않는다.
///
/// 반환값은 [`HexCoord`]의 `Ord` 기준 오름차순이다(생성 순서를 그대로 흘리지 않는다).
pub fn ring(center: HexCoord, radius: i32) -> Result<Vec<HexCoord>, HexError> {
    if radius < 0 {
        return Err(HexError::NegativeRadius);
    }
    if radius == 0 {
        return Ok(vec![center]);
    }
    let radius_i64 = i64::from(radius);
    let boundary: Vec<(i64, i64)> = axial_disk_offsets(radius)?
        .into_iter()
        .filter(|&(dq, dr)| axial_distance_i64(dq, dr) == radius_i64)
        .collect();
    coords_from_offsets(center, &boundary)
}

/// 반지름 `radius` 이내 전체(중심 포함, 경계 포함). 음수 `radius`는
/// [`HexError::NegativeRadius`]다.
///
/// 반환값은 [`HexCoord`]의 `Ord` 기준 오름차순이다.
pub fn range(center: HexCoord, radius: i32) -> Result<Vec<HexCoord>, HexError> {
    let offsets = axial_disk_offsets(radius)?;
    coords_from_offsets(center, &offsets)
}

/// `from`에서 `to`까지의 직선 경로. 양 끝 포함. 대시·관통 이동(T2)이 소비할
/// helper다 — **`line()`은 장애물을 모른다.** 경로 탐색(A* 등 우회)은 이 함수의
/// 범위 밖이다.
///
/// 반환 타입이 계획 문서의 `Vec<HexCoord>` 그대로인 이유(=`Result`가 필요 없는 이유):
/// 아래 알고리즘이 만드는 각 좌표는 `from`과 `to`의 해당 큐브축 값 사이로
/// **바운드된 반올림**이다(증명: 보간값은 항상 두 끝값 사이의 구간에 있고,
/// 구간 끝이 이미 정수면 그 구간 안의 실수를 가장 가까운 정수로 반올림한 값도
/// 그 구간을 벗어나지 않는다 — 최종 축 fix-up도 같은 구간 성질을 물려받는다).
/// `from`/`to`가 이미 유효한 `i32`이므로 중간 결과도 항상 유효한 `i32`다.
/// 나눗셈이 필요한 중간 계산(`round_half_up`)은 `i128`로 하므로(`i32` 두 값의
/// 차 × 거리, 최악 폭이 대략 2^64 — `i128`의 2^127에 비하면 여유가 크다) 그
/// 계산 자체도 오버플로하지 않는다.
///
/// **주의(메모리, 오버플로와는 다른 문제).** 반환 벡터 길이는 `distance(from,
/// to) + 1`이다. `from`/`to`가 서로 아주 멀면(예: `i32::MIN`과 `i32::MAX`, 약
/// 43억 타일) 이 함수는 그만큼의 `Vec<HexCoord>`를 할당하려다 프로세스가
/// 죽는다. 이건 checked 산술이 막아주는 "오버플로 패닉"이 아니라 순수한 자원
/// 한계이며, 이 슬라이스의 범위 밖이다(경로 탐색·거리 상한은 계획 §9). 대시·
/// 관통 이동(T2)이 이 함수를 쓸 때는 호출부가 합리적인 사거리로 이미 걸러진
/// `from`/`to`를 넘긴다고 가정한다.
///
/// ## tie-break 규칙 (계약 — [`line_tie_break_is_pinned`] 테스트가 고정한다)
///
/// 직선이 정확히 두 타일의 경계(꼭짓점 등)를 지나는 지점은 부동소수점으로
/// 풀면 epsilon에 좌우된다. 이 구현은 float를 전혀 쓰지 않고 두 단계의 정수
/// 규칙으로 그 동점을 깬다.
///
/// 1. axial `(q, r)`을 큐브 `(x, y, z) = (q, r, -q-r)`으로 보고, 각 단계
///    `i = 0..=N`(N = [`HexCoord::distance`])에서 `x/y/z`를 `from`→`to` 선형
///    보간한 뒤 **반올림 동점(정확히 .5)은 큰 쪽으로 올린다** (round-half-up,
///    `-inf` 방향이 아니라 `+inf` 방향).
/// 2. 세 축을 독립적으로 반올림하면 `x+y+z=0`이 깨질 수 있으므로, 반올림
///    오차(=원래 값과의 차이의 절댓값)가 **가장 큰 축 하나를 나머지 둘의 합의
///    음수로 재계산**해 constraint를 되살린다. 오차가 동점이면 **`x`를 `y`나
///    `z`보다 먼저, `y`를 `z`보다 먼저** 재계산 대상으로 고른다 — 즉 `x >= y,
///    x >= z`면 `x`를, 아니면 `y > z`면 `y`를, 아니면 `z`를 고친다.
///
/// 이 두 규칙(반올림 동점은 위로, 축 동점은 `x > y > z` 우선)이 "정확히 두 타일
/// 사이"의 유일한 결정 기준이다.
///
/// ## 방향 비대칭
///
/// `line(a, b)`를 뒤집은 것이 `line(b, a)`와 다를 수 있다 — **허용된 거동이다**
/// (계획 §4-4). 억지로 대칭을 만들려고 tie-break를 흐리지 않는다. 실제로 어떤
/// 입력이 비대칭을 보이는지는 [`line_direction_asymmetry_is_pinned`] 테스트가
/// 고정한다.
pub fn line(from: HexCoord, to: HexCoord) -> Vec<HexCoord> {
    let steps = from.distance(to);
    if steps == 0 {
        return vec![from];
    }
    let (x0, z0) = (i128::from(from.q), i128::from(from.r));
    let (x1, z1) = (i128::from(to.q), i128::from(to.r));
    let (y0, y1) = (-x0 - z0, -x1 - z1);
    let n = i128::from(steps);

    let mut path = Vec::with_capacity((steps + 1) as usize);
    for i in 0..=steps {
        let i = i128::from(i);
        let xi = x0 * (n - i) + x1 * i;
        let yi = y0 * (n - i) + y1 * i;
        let zi = z0 * (n - i) + z1 * i;
        let (x, _y, z) = cube_round(xi, yi, zi, n);
        // 큐브 -> axial: q = x, r = z (y = -x-z는 버린다). x/y/z는 각각 from과
        // to의 해당 축 값 사이로 바운드돼 있으므로(위 문서의 증명) i32에 항상
        // 들어간다. `debug_assert`로 그 증명을 테스트에서 스스로 검증한다 —
        // 만약 증명이 틀렸다면 여기서 패닉하는 대신 `extreme_coordinates_do_not_panic`
        // 같은 디버그 빌드 테스트가 먼저 잡아낸다.
        debug_assert!(i32::try_from(x).is_ok(), "line() q out of i32 range");
        debug_assert!(i32::try_from(z).is_ok(), "line() r out of i32 range");
        path.push(HexCoord {
            q: x as i32,
            r: z as i32,
        });
    }
    path
}

/// [`line`]의 tie-break 2단계(반올림 half-up -> 최대오차축 fix-up)를 구현한다.
/// `n > 0` (호출부가 `steps == 0`을 먼저 걸러낸다).
fn cube_round(xi: i128, yi: i128, zi: i128, n: i128) -> (i128, i128, i128) {
    let mut x = round_half_up(xi, n);
    let mut y = round_half_up(yi, n);
    let mut z = round_half_up(zi, n);

    let err_x = (xi - x * n).abs();
    let err_y = (yi - y * n).abs();
    let err_z = (zi - z * n).abs();

    if err_x >= err_y && err_x >= err_z {
        x = -y - z;
    } else if err_y > err_z {
        y = -x - z;
    } else {
        z = -x - y;
    }
    (x, y, z)
}

/// `numerator / denom`을 가장 가까운 정수로 반올림한다. `denom > 0`이 전제다.
/// 정확히 절반(.5)인 동점은 **위로**(더 큰 정수로) 올린다 — 이것이 정한 규칙이다.
fn round_half_up(numerator: i128, denom: i128) -> i128 {
    let q = numerator.div_euclid(denom);
    let rem = numerator - q * denom; // [0, denom) 범위, div_euclid이 보장한다.
    if 2 * rem >= denom {
        q + 1
    } else {
        q
    }
}

/// 대형 유닛의 점유 형태 — 앵커 기준 오프셋 집합. **회전이 없다** (상위 문서
/// 결정 12: 대형 유닛은 회전 없이 점유 형태가 고정이다). 이 타입에도, 이
/// 모듈 어디에도 회전 helper를 추가하지 않는다 — 비대칭 형태가 좌우 배치에서
/// 같은 방향을 향하는 문제는 콘텐츠 가이드(대칭 형태를 쓰라는 안내)로 해소한다.
///
/// ## 정규형 (계약 — [`same_shape_in_different_order_normalizes_identically`]과
/// [`same_shape_at_different_absolute_position_normalizes_identically`]가 고정한다)
///
/// [`HexShape::new`]는 입력 오프셋을 다음 두 단계로 정규화한다.
///
/// 1. 입력 오프셋 중 `Ord`([`HexCoord`]의 `(q, r)` 사전순) 기준 **최솟값을
///    원점(0,0)으로 옮기도록 전체를 이동**한다. 사전순은 이동에 대해
///    불변이다(모든 오프셋에서 같은 상수를 빼도 상대 순서가 바뀌지 않는다) —
///    그래서 이동한 뒤에도 그 최솟값이 그대로 `(0,0)`이 되고, 여전히 정규화된
///    집합의 최솟값이다.
/// 2. 이동한 오프셋을 `Ord` 오름차순으로 정렬한다.
///
/// **결과적으로 정규형은 입력의 순서에도, 입력이 어떤 절대 위치를 기준으로
/// 적혀 있었는지에도 무관하다** — 같은 상대 모양이면 항상 같은 정규형이
/// 나온다. 이것이 이 슬라이스가 확정하는 세 번째 계약이다.
///
/// 빈 오프셋 집합과 중복 오프셋은 [`HexError::EmptyShape`]/[`HexError::DuplicateOffset`]로
/// 거부한다 — 빈 모양이나 겹친 타일을 조용히 정리해 지어내지 않는다.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HexShape {
    /// 정규화된 오프셋. 항상 비어 있지 않고, 중복이 없고, `Ord` 오름차순이며,
    /// 첫 원소가 `(0,0)`이다. `HexShape::new`를 통과한 값만 이 불변식을 만족한다.
    normalized_offsets: Vec<HexCoord>,
}

impl HexShape {
    /// 오프셋 집합으로부터 정규화된 [`HexShape`]를 만든다. 정규형 규칙은
    /// 타입 문서를 본다.
    pub fn new(offsets: Vec<HexCoord>) -> Result<HexShape, HexError> {
        if offsets.is_empty() {
            return Err(HexError::EmptyShape);
        }

        // 중복 검사. 입력 순서대로 훑어 처음 중복이 나타나는 오프셋을 보고한다
        // (결정론적이되, 그 값 자체는 계약이 아니다 — HexError 문서 참고).
        let mut seen: std::collections::BTreeSet<HexCoord> = std::collections::BTreeSet::new();
        for &offset in &offsets {
            if !seen.insert(offset) {
                return Err(HexError::DuplicateOffset(offset));
            }
        }

        // 안전: offsets가 비어있지 않음을 위에서 이미 확인했다.
        let anchor = *offsets.iter().min().expect("offsets is non-empty");
        let mut normalized_offsets = offsets
            .into_iter()
            .map(|offset| offset.checked_sub(anchor))
            .collect::<Result<Vec<HexCoord>, HexError>>()?;
        normalized_offsets.sort();

        Ok(HexShape { normalized_offsets })
    }

    /// 이 모양을 `anchor`에 놓았을 때 실제로 점유하는 타일들. `Ord` 오름차순이다
    /// (정규화된 오프셋이 이미 오름차순이고, 상수 이동이 그 순서를 보존하므로
    /// — [`HexShape`] 타입 문서의 정규형 절과 같은 논증 — 여기서 다시 정렬할
    /// 필요는 없지만, 방어적으로 한 번 더 정렬한다).
    ///
    /// 반환 타입이 계획 문서의 `Vec<HexCoord>`가 아니라 `Result<Vec<HexCoord>,
    /// HexError>`인 이유는 [`ring`]/[`range`]와 같다: `anchor + offset`은 실제
    /// 덧셈이고 `anchor`가 극단값 근처면 오버플로할 수 있다.
    pub fn tiles_at(&self, anchor: HexCoord) -> Result<Vec<HexCoord>, HexError> {
        let mut tiles = self
            .normalized_offsets
            .iter()
            .map(|&offset| anchor.checked_add(offset))
            .collect::<Result<Vec<HexCoord>, HexError>>()?;
        tiles.sort();
        Ok(tiles)
    }
}

/// 이 모듈의 모든 산술 실패·잘못된 입력을 표현한다. 값을 지어내거나 조용히
/// 기본값으로 대체하지 않고 항상 이 타입으로 거부한다.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HexError {
    /// checked 산술이 `i32` 범위를 벗어났다(극단 좌표 + 큰 오프셋/반지름 등).
    Overflow,
    /// [`ring`]/[`range`]에 음수 반지름이 들어왔다. 빈 결과를 지어내지 않고 거부한다.
    NegativeRadius,
    /// [`HexShape::new`]에 빈 오프셋 집합이 들어왔다.
    EmptyShape,
    /// [`HexShape::new`]에 중복된 오프셋이 들어왔다. 값은 처음 중복이 발견된
    /// 오프셋이다(입력 순서 기준 — 결정론적이되, 이 값 자체가 계약은 아니다).
    DuplicateOffset(HexCoord),
    /// [`HexOccupancy::try_occupy`]가 요청한 타일 중 이미 점유된 것을 만났다.
    TileOccupied(HexCoord),
}
impl std::fmt::Display for HexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for HexError {}

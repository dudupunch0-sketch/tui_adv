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

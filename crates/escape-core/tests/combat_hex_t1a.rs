//! T1-a `combat_hex` 회귀 테스트. 이 모듈은 아무 데도 배선되지 않았으므로 여기가
//! 실질적인 산출물이다(`fable_combat_hex_t1a_step1_2608061847.md` §7).
//!
//! WP 순서대로 늘어난다 — WP6(오버플로 방어·마무리)에서 끝난다.
use escape_core::*;

fn c(q: i32, r: i32) -> HexCoord {
    HexCoord { q, r }
}

// ---- 거리·인접 -------------------------------------------------------

#[test]
fn distance_to_self_is_zero() {
    assert_eq!(c(3, -2).distance(c(3, -2)), 0);
    assert_eq!(c(0, 0).distance(c(0, 0)), 0);
}

#[test]
fn every_neighbor_is_at_distance_one() {
    let center = c(4, -1);
    for neighbor in center.neighbors().unwrap() {
        assert_eq!(center.distance(neighbor), 1);
    }
}

#[test]
fn distance_is_symmetric() {
    let a = c(5, -3);
    let b = c(-2, 7);
    assert_eq!(a.distance(b), b.distance(a));
}

#[test]
fn neighbors_are_six_distinct_coords() {
    let neighbors = c(0, 0).neighbors().unwrap();
    for i in 0..neighbors.len() {
        for j in (i + 1)..neighbors.len() {
            assert_ne!(
                neighbors[i], neighbors[j],
                "neighbors[{i}] == neighbors[{j}]"
            );
        }
    }
}

/// 방향 순서 상수 계약을 그대로 고정한다. 이후 어떤 소비자도 이 배열을 인덱스로
/// 참조할 수 있으므로, 여기서 값이 바뀌면 이 테스트가 반드시 깨져야 한다.
#[test]
fn neighbor_direction_order_is_fixed() {
    assert_eq!(
        HexCoord::NEIGHBOR_DIRECTIONS,
        [c(1, 0), c(1, -1), c(0, -1), c(-1, 0), c(-1, 1), c(0, 1),]
    );
}

// ---- 범위 -------------------------------------------------------------

#[test]
fn ring_of_radius_zero_is_just_the_center() {
    let center = c(2, 2);
    assert_eq!(ring(center, 0).unwrap(), vec![center]);
}

#[test]
fn ring_size_is_six_times_radius() {
    let center = c(0, 0);
    for radius in 1..=5 {
        let tiles = ring(center, radius).unwrap();
        assert_eq!(tiles.len(), (6 * radius) as usize, "radius {radius}");
    }
}

#[test]
fn range_size_is_one_plus_three_r_times_r_plus_one() {
    let center = c(0, 0);
    for radius in 0..=5 {
        let tiles = range(center, radius).unwrap();
        let expected = 1 + 3 * radius * (radius + 1);
        assert_eq!(tiles.len(), expected as usize, "radius {radius}");
    }
}

#[test]
fn ring_and_range_results_are_sorted() {
    let center = c(1, -1);
    let ring_tiles = ring(center, 3).unwrap();
    let mut sorted = ring_tiles.clone();
    sorted.sort();
    assert_eq!(ring_tiles, sorted);

    let range_tiles = range(center, 3).unwrap();
    let mut sorted_range = range_tiles.clone();
    sorted_range.sort();
    assert_eq!(range_tiles, sorted_range);
}

#[test]
fn negative_radius_is_rejected() {
    assert_eq!(ring(c(0, 0), -1), Err(HexError::NegativeRadius));
    assert_eq!(range(c(0, 0), -1), Err(HexError::NegativeRadius));
}

/// 링·범위는 중심이 포함된 결과에서 서로 겹치지 않는 테두리로 정확히 분해돼야
/// 한다 — `range`의 정의("반지름 이내 전체")가 `ring`의 합과 일치하는지 확인한다.
#[test]
fn range_equals_union_of_rings_up_to_radius() {
    let center = c(-2, 3);
    for radius in 0..=4 {
        let range_tiles = range(center, radius).unwrap();
        let mut union: Vec<HexCoord> = (0..=radius)
            .flat_map(|r| ring(center, r).unwrap())
            .collect();
        union.sort();
        assert_eq!(range_tiles, union, "radius {radius}");
    }
}

// ---- 경로 -----------------------------------------------------------

#[test]
fn line_includes_both_endpoints() {
    let from = c(-2, 3);
    let to = c(4, -1);
    let path = line(from, to);
    assert_eq!(path.first(), Some(&from));
    assert_eq!(path.last(), Some(&to));
}

#[test]
fn line_length_equals_distance_plus_one() {
    let from = c(-3, 5);
    let to = c(6, -2);
    let path = line(from, to);
    assert_eq!(path.len() as i64, from.distance(to) + 1);
}

#[test]
fn consecutive_line_tiles_are_adjacent() {
    let from = c(-4, 6);
    let to = c(5, -3);
    let path = line(from, to);
    for pair in path.windows(2) {
        assert!(
            pair[0].is_adjacent(pair[1]),
            "{:?} and {:?} not adjacent",
            pair[0],
            pair[1]
        );
    }
}

/// tie-break 고정: `(0,0)` -> `(1,1)`은 큐브 좌표에서 중간 step(`i=1`)의 `x`,`z` 축이
/// 정확히 `.5` 반올림 동점이면서 동시에 fix-up 단계에서도 `err_x == err_z`인
/// 동점을 만든다(모듈 문서 참고). 규칙(반올림 half-up, 최대오차축 `x > y > z` 우선)이
/// 이 입력에서 정확히 어떤 타일을 고르는지 여기서 못박는다.
#[test]
fn line_tie_break_is_pinned() {
    let path = line(c(0, 0), c(1, 1));
    assert_eq!(path, vec![c(0, 0), c(0, 1), c(1, 1)]);
}

/// 방향 비대칭은 계획상 **허용되지만 강제되지 않는다.** 이 구현이 고른 tie-break
/// (반올림 half-up + 최대오차축 우선순위)는 보간 분자가 방향과 무관하게 동일한
/// 값으로 재현되도록 만들어서(모듈 문서의 증명 참고) **방향에 대해 대칭이다** —
/// float epsilon류의 방향 편향을 넣지 않은 결과다. 그 대칭성 자체가 "문서화한
/// 거동"이므로 여기서 고정한다.
#[test]
fn line_direction_asymmetry_is_pinned() {
    let cases = [
        (c(0, 0), c(2, -1)),
        (c(0, 0), c(1, 1)),
        (c(-3, 5), c(6, -2)),
        // 극단 좌표 근처지만 서로 가까운 두 점 — "극단값"과 "천문학적으로 먼 거리"는
        // 다른 얘기다(아래 extreme_coordinates_do_not_panic의 주석 참고).
        (c(i32::MAX, i32::MAX), c(i32::MAX - 4, i32::MAX)),
    ];
    for (a, b) in cases {
        let forward = line(a, b);
        let mut backward = line(b, a);
        backward.reverse();
        assert_eq!(
            forward, backward,
            "line({a:?},{b:?}) is not direction-symmetric"
        );
    }
}

// ---- 형태 -----------------------------------------------------------

#[test]
fn same_shape_in_different_order_normalizes_identically() {
    let a = HexShape::new(vec![c(0, 0), c(1, 0), c(0, 1)]).unwrap();
    let b = HexShape::new(vec![c(0, 1), c(0, 0), c(1, 0)]).unwrap();
    assert_eq!(a.tiles_at(c(0, 0)).unwrap(), b.tiles_at(c(0, 0)).unwrap());
}

/// 정규형의 두 번째 성질: 절대 위치가 달라도(=입력 오프셋을 통째로 옮겨 적어도)
/// 같은 상대 모양은 같은 정규형이 된다. [`HexShape`] 타입 문서의 정규형 절이
/// 명시하는 계약이다.
#[test]
fn same_shape_at_different_absolute_position_normalizes_identically() {
    let a = HexShape::new(vec![c(0, 0), c(1, 0), c(0, 1)]).unwrap();
    let b = HexShape::new(vec![c(5, 5), c(6, 5), c(5, 6)]).unwrap();
    assert_eq!(a.tiles_at(c(0, 0)).unwrap(), b.tiles_at(c(0, 0)).unwrap());
}

#[test]
fn tiles_at_translates_every_offset() {
    let shape = HexShape::new(vec![c(0, 0), c(1, 0), c(0, 1)]).unwrap();
    let anchor = c(10, -4);
    let mut expected = vec![c(10, -4), c(11, -4), c(10, -3)];
    expected.sort();
    assert_eq!(shape.tiles_at(anchor).unwrap(), expected);
}

#[test]
fn empty_shape_is_rejected() {
    assert_eq!(HexShape::new(vec![]), Err(HexError::EmptyShape));
}

#[test]
fn duplicate_offset_is_rejected() {
    let err = HexShape::new(vec![c(0, 0), c(1, 0), c(0, 0)]).unwrap_err();
    assert_eq!(err, HexError::DuplicateOffset(c(0, 0)));
}

// ---- 점유 -----------------------------------------------------------

#[test]
fn one_tile_holds_one_occupant() {
    let mut occ = HexOccupancy::new();
    occ.try_occupy(&[c(0, 0)], "a").unwrap();
    assert_eq!(occ.occupant_at(c(0, 0)), Some("a"));
    assert!(!occ.is_free(c(0, 0)));
}

#[test]
fn occupying_an_occupied_tile_fails() {
    let mut occ = HexOccupancy::new();
    occ.try_occupy(&[c(0, 0)], "a").unwrap();
    let err = occ.try_occupy(&[c(0, 0)], "b").unwrap_err();
    assert_eq!(err, HexError::TileOccupied(c(0, 0)));
}

#[test]
fn a_failed_multi_tile_occupy_leaves_no_partial_state() {
    let mut occ = HexOccupancy::new();
    occ.try_occupy(&[c(5, 5)], "blocker").unwrap();
    let err = occ
        .try_occupy(&[c(0, 0), c(1, 0), c(5, 5)], "big")
        .unwrap_err();
    assert_eq!(err, HexError::TileOccupied(c(5, 5)));
    // 실패한 요청의 다른 타일들은 점유되지 않은 채로 남아야 한다(all-or-nothing).
    assert!(occ.is_free(c(0, 0)));
    assert!(occ.is_free(c(1, 0)));
    assert_eq!(occ.occupant_at(c(5, 5)), Some("blocker"));
}

#[test]
fn vacate_frees_every_tile_of_that_id() {
    let mut occ = HexOccupancy::new();
    occ.try_occupy(&[c(0, 0), c(1, 0), c(0, 1)], "big").unwrap();
    occ.vacate("big");
    assert!(occ.is_free(c(0, 0)));
    assert!(occ.is_free(c(1, 0)));
    assert!(occ.is_free(c(0, 1)));
}

#[test]
fn vacating_an_id_with_no_tiles_is_a_harmless_no_op() {
    let mut occ = HexOccupancy::new();
    occ.try_occupy(&[c(0, 0)], "a").unwrap();
    occ.vacate("nobody");
    assert_eq!(occ.occupant_at(c(0, 0)), Some("a"));
}

#[test]
fn iteration_order_is_deterministic() {
    let mut occ = HexOccupancy::new();
    occ.try_occupy(&[c(3, 3)], "a").unwrap();
    occ.try_occupy(&[c(-1, -1)], "b").unwrap();
    occ.try_occupy(&[c(0, 0)], "c").unwrap();
    let coords: Vec<HexCoord> = occ.iter().map(|(coord, _)| *coord).collect();
    let mut sorted = coords.clone();
    sorted.sort();
    assert_eq!(coords, sorted);
}

// ---- 안전 -------------------------------------------------------------

#[test]
fn extreme_coordinates_do_not_panic() {
    let min = c(i32::MIN, i32::MIN);
    let max = c(i32::MAX, i32::MAX);
    // 좌표값이 극단인 것과 "두 점 사이의 거리가 천문학적으로 먼 것"은 다른
    // 문제다. min<->max 사이는 약 43억 타일 거리라 `line()`이 시도하면 43억
    // 개짜리 Vec를 할당하려다 프로세스가 죽는다 — 이건 checked 산술이 막을 수
    // 있는 "오버플로 패닉"이 아니라 순수한 메모리 자원 한계이고, 이 슬라이스가
    // 다룰 범위 밖이다(계획 문서 §9, "경로 탐색"도 범위 밖). 그래서 여기서는
    // 좌표는 극단이되 서로 가까운 입력으로 `line()`을 검증한다.
    let _ = min.distance(max);
    let _ = min.is_adjacent(max);
    let _ = min.neighbors();
    let _ = max.neighbors();
    let _ = ring(min, 3);
    let _ = range(max, 3);
    let _ = line(min, c(i32::MIN + 5, i32::MIN));
    let _ = line(c(i32::MAX, i32::MAX), c(i32::MAX, i32::MAX - 5));
    let shape = HexShape::new(vec![c(0, 0), c(1, 0)]).unwrap();
    let _ = shape.tiles_at(max);
    let _ = shape.tiles_at(min);
    let mut occ = HexOccupancy::new();
    occ.try_occupy(&[min, max], "extreme").unwrap();
    occ.vacate("extreme");
}

#[test]
fn tiles_at_overflow_instead_of_panicking() {
    let shape = HexShape::new(vec![c(0, 0), c(1, 0)]).unwrap();
    assert_eq!(shape.tiles_at(c(i32::MAX, 0)), Err(HexError::Overflow));
}

/// `ring`/`range`도 `neighbors`/`tiles_at`과 같은 규칙(경계 계산은 `i64`로 안전하게,
/// 실제 좌표로 옮기는 마지막 단계에서만 checked)을 따른다는 것을 직접 확인한다.
#[test]
fn ring_and_range_overflow_instead_of_panicking() {
    assert_eq!(ring(c(i32::MAX, 0), 5), Err(HexError::Overflow));
    assert_eq!(range(c(i32::MAX, 0), 5), Err(HexError::Overflow));
}

#[test]
fn neighbors_at_extreme_edge_overflow_instead_of_panicking() {
    assert_eq!(c(i32::MAX, 0).neighbors(), Err(HexError::Overflow));
    assert_eq!(c(i32::MIN, 0).neighbors(), Err(HexError::Overflow));
}

//! T1-a `combat_hex` 회귀 테스트. 이 모듈은 아무 데도 배선되지 않았으므로 여기가
//! 실질적인 산출물이다(`fable_combat_hex_t1a_step1_2608061847.md` §7).
//!
//! WP 순서대로 늘어난다 — 지금은 WP2(`ring`/`range`)까지다.
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

// ---- 안전 -------------------------------------------------------------

#[test]
fn extreme_coordinates_do_not_panic() {
    let min = c(i32::MIN, i32::MIN);
    let max = c(i32::MAX, i32::MAX);
    let _ = min.distance(max);
    let _ = min.is_adjacent(max);
    let _ = min.neighbors();
    let _ = max.neighbors();
    let _ = ring(min, 3);
    let _ = range(max, 3);
}

#[test]
fn neighbors_at_extreme_edge_overflow_instead_of_panicking() {
    assert_eq!(c(i32::MAX, 0).neighbors(), Err(HexError::Overflow));
    assert_eq!(c(i32::MIN, 0).neighbors(), Err(HexError::Overflow));
}

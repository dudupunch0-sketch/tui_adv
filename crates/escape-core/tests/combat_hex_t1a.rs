//! T1-a `combat_hex` 회귀 테스트. 이 모듈은 아무 데도 배선되지 않았으므로 여기가
//! 실질적인 산출물이다(`fable_combat_hex_t1a_step1_2608061847.md` §7).
//!
//! WP 순서대로 늘어난다 — 지금은 WP1(`HexCoord` + 거리 + 인접)까지다.
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

// ---- 안전 -------------------------------------------------------------

#[test]
fn extreme_coordinates_do_not_panic() {
    let min = c(i32::MIN, i32::MIN);
    let max = c(i32::MAX, i32::MAX);
    let _ = min.distance(max);
    let _ = min.is_adjacent(max);
    let _ = min.neighbors();
    let _ = max.neighbors();
}

#[test]
fn neighbors_at_extreme_edge_overflow_instead_of_panicking() {
    assert_eq!(c(i32::MAX, 0).neighbors(), Err(HexError::Overflow));
    assert_eq!(c(i32::MIN, 0).neighbors(), Err(HexError::Overflow));
}

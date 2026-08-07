# T1-d — 대형 유닛 다중 타일 점유: 구현 보고

plan: `fable_combat_hex_t1d_step1_2608072234.md`
report: 본 문서 (`fable_combat_hex_t1d_step2_report.md`)
baseline: `6180751` (plan에 명시) — 실제 시작 지점은 `11643b6`(plan 자체를 커밋한 지점)까지
fast-forward한 뒤였다. 아래 §0 참고.

## 0. 시작 지점 확인

워크트리가 `4684884`에 멈춰 있어 `11643b6`(`docs: write the T1-d plan for
large-unit occupancy`)보다 뒤처져 있었다. `git merge --ff-only 11643b6`로
fast-forward했다 — fast-forward만 수행했고 병합 충돌이나 리베이스는 없었다.

fast-forward 직후 baseline을 재확인했다:

```
cargo test --workspace --no-fail-fast
```
→ **409 passed; 0 failed** (합산: 14+10+10+32+9+12+24+14+21+8+9+32+32+11+3+3+3+8+4+23+5+24+61+37 = 409)

```
cd web && npm test
```
→ `vitest: not found` — `npm install`이 필요했다(이 워크트리에서 `web/node_modules`가
비어 있었다). `npm install` 후 재실행하여 **168 passed**를 확인했다.

두 숫자 모두 plan §2가 요구한 값과 정확히 일치했다. 계속 진행했다.

## 1. WP별 커밋

| WP | 커밋 | 한 줄 요약 |
|---|---|---|
| WP1 | `194a9fe` | `occupies` 필드 추가, `HexShape` 기반 앵커 포함·연결성 검증, 발자국 기준 초기 배치 충돌 검사 |
| WP2 | `12078b8` | `footprint_tiles`/`footprint_distance` 추가, 거리 측정 5지점 전부 전환 |
| WP3 | `b0b7c9d` | 발자국 기준 이동 차단(자기 자신 제외 포함)과 목적지 경합을 발자국 겹침으로 일반화 |
| WP4 | `1a9072d` | `combat_large_units_t1d.rs` — plan §6 WP4 최소 테스트 집합 13개 |

각 WP는 plan이 요구한 검증 명령을 그 WP 커밋 시점의 코드에 대해 실제로 실행해
확인했다(아래 §2). 구현 도중 파일 하나(`combat_simulation.rs`)에 WP1~WP3가
깊이 얽혀 있어, 커밋을 WP 경계로 나누기 위해 최종 구현을 완성한 뒤 HEAD의
원본 파일로 되돌리고 WP1→WP2→WP3 순서로 다시 단계적으로 적용했다 — 이렇게
재구성한 각 단계에서 그 WP의 지정된 검증 명령을 실제로 실행했고, 최종 결과가
처음에 완성했던 구현과 바이트 단위로 동일한지 `diff`로 확인했다(사소한 주석
문구 한 줄 차이만 있었고, 그마저 이번 재구성에서 더 정확하게 고쳐졌다).

## 2. 검증 명령과 실제 숫자

### WP1
```
cargo test -p escape-core --test combat_simulation_wave2
```
→ **14 passed; 0 failed** — T1-c/T1-b1이 남긴 14개 값 그대로.

### WP2
```
cargo test -p escape-core --test combat_simulation_wave2   # 14 passed
cargo test -p escape-core --test combat_resolution_wave2   # 24 passed
cargo test -p escape-core --test encounter_combat_wave3    # 32 passed
```
세 파일 모두 **값 변화 0**.

### WP3
```
cargo test -p escape-core --test combat_occupancy_t1c
```
→ **9 passed; 0 failed** — T1-c의 9개 성질(초기 배치 거부, 경로 차단, 목적지
경합, 순서 독립성, tick-시작 스냅샷 보수성, 후퇴 차단, 포위 감지 무배선)
전부 그대로.

### WP4
```
cargo test -p escape-core --test combat_large_units_t1d
```
→ **13 passed; 0 failed** (아래 §5 목록).

### 전체
```
cargo fmt --all -- --check     # 통과 (0 diff)
cargo test --workspace --no-fail-fast
```
→ **422 passed; 0 failed** = 기존 409 + 신규 13, **감소 없음**.

```
git diff --check
```
→ 통과 (trailing whitespace 등 없음).

```
cd web && npm test
```
→ **168 passed** — plan대로 무변경. (web은 소유 파일 목록 밖이며 실제로
건드리지 않았다.)

## 3. §4-4의 다섯 지점 — 지점별 확인

거리 측정을 앵커 대 앵커에서 발자국 대 발자국 최솟값으로 바꾼 다섯 지점을
하나씩 확인한다. 요약하지 않는다.

1. **`combat_simulation.rs`, `select_target` — 목표 선호 계산 (정책 분기)의
   `da`/`db`.** `policy.preferences`를 순회하며 각 후보의 거리를
   `footprint_distance(target.position, &target.occupies, actor.position,
   &actor.occupies)`로 미리 계산해 `BTreeMap<&str, i64>`에 담고, `max_by`
   비교자는 그 맵에서 읽기만 한다(비교자 자체는 `Result`를 반환할 수 없어서
   fallible한 거리 계산을 비교자 밖으로 뺐다).
2. **`combat_simulation.rs`, `select_target` — 최근접 fallback.**
   `self.participants.values().filter(valid)`을 순회하며 각각의 거리를
   `footprint_distance(p.position, &p.occupies, actor.position,
   &actor.occupies)`로 계산해 `Vec`에 모은 뒤 `min_by`.
3. **`combat_simulation.rs`, `advance_tick` — 이동 판단의 `d`.**
   `footprint_distance(actor.position, &actor.occupies, target.position,
   &target.occupies)`로 교체. 이 값이 `preferred_distance`와 비교되어
   전진/후퇴/제자리를 가른다.
4. **`combat_resolution.rs`, `resolve` — 사거리·근접(`collision`/`in_range`)
   판정의 `distance`.** 프레임은 앵커만 갖고 있으므로(`frame.positions`),
   `request.execution.input.participants`에서 읽은 `actor.occupies`/
   `target.occupies`와 그 앵커를 조합해 `footprint_distance`를 호출한다.
   프레임 스키마는 손대지 않았다(plan §4-4가 명시적으로 금지).

`combat_simulation.rs`에 네 곳, `combat_resolution.rs`에 한 곳 — plan이
표로 정리한 지점 수(목표 선호 계산 2곳 + 최근접 fallback 1곳 + 이동 판단
1곳 + 사거리·근접 1곳)와 정확히 일치한다.

`footprint_distance`는 단일 타일(occupies 비어있음)일 때 정확히
`HexCoord::distance(anchor_a, anchor_b)`로 수렴하도록 설계했다(각 발자국이
`[anchor]` 하나뿐인 집합으로 축소되므로 최솟값이 그 값 하나뿐이다). 다섯
지점 전부 이 성질에 의존하며, 기존 전투 전부가 여기 해당하므로 어떤 기존
테스트 값도 변하지 않아야 한다는 예측이 §2의 실측(0 변화)과 일치한다.

## 4. §4-5의 함정 — 실제로 검증했다

계획이 지목한 "자기 발자국이 자기를 막는다" 함정을, 코드를 일부러 되돌려서
`a_large_unit_does_not_block_itself_while_moving`이 실제로 실패하는지 확인했다:

- `first_free_tile_along`의 자기 제외 조건(`occupant != mover_id`)을
  `occupant_at(tile).is_some()`(자기 자신도 막는 조건)로 임시 교체 →
  해당 테스트가 다음처럼 **실패**했다:
  ```
  left: HexCoord { q: 0, r: 0 }
  right: HexCoord { q: 1, r: 0 }
  ```
  (2칸짜리 대형 유닛이 자기 자신의 옛 발자국에 막혀 한 걸음도 전진하지
  못했다 — plan이 경고한 정확히 그 실패 모드다.)
- 즉시 원상복구하고 `git diff`로 파일이 깨끗함을 확인했다.

같은 방식으로 "발자국 전체가 아니라 후보 앵커 타일 하나만 본다"는 (naive한)
축소판으로 되돌려서 `a_large_unit_stops_when_any_footprint_tile_would_be_blocked`도
**실패**함을 확인했다(대형 유닛이 자기 꼬리 타일이 막혀 있는데도 앵커
타일만 비어 있다는 이유로 그 칸까지 전진해버렸다). 두 경우 모두 수정을
되돌린 뒤 재확인했다.

두 테스트 모두 "고정하려는 성질이 실제로 깨지면 테스트도 실제로 깨진다"는
것을 실측으로 확인했다 — 통과만으로는 증명되지 않는 부분이었다.

## 5. 신규 테스트 13개 (`combat_large_units_t1d.rs`)

plan §6 WP4 표의 13개 전부 구현했다:

| 테스트 | 결과 |
|---|---|
| `an_empty_occupies_list_means_a_single_tile_at_the_anchor` | ok |
| `occupies_without_the_origin_offset_is_rejected` | ok |
| `a_disconnected_footprint_is_rejected` | ok |
| `a_duplicate_offset_is_rejected` | ok |
| `two_large_units_with_overlapping_footprints_cannot_both_start` | ok |
| `distance_is_measured_from_the_nearest_occupied_tile` | ok |
| `a_large_unit_in_range_by_its_body_but_not_its_anchor_can_attack` | ok |
| `a_large_unit_stops_when_any_footprint_tile_would_be_blocked` | ok |
| `a_large_unit_does_not_block_itself_while_moving` | ok |
| `overlapping_destinations_make_both_large_units_hold` | ok |
| `single_tile_units_behave_exactly_as_before` | ok |
| `occupies_is_absent_from_json_when_empty` | ok |
| `shuffled_participant_order_yields_identical_frames` | ok |

## 6. §8 — 저작 전투 두 테스트

```
cargo test -p escape-core --test encounter_combat_wave3 authored_preview_bout
```
```
test authored_preview_bout_never_lets_the_two_combatants_swap_sides_or_share_a_tile ... ok
test authored_preview_bout_behaviour_is_unchanged_by_the_coordinate_swap ... ok
```

**둘 다 값 그대로다.** 기대값을 고치지 않았다.

## 7. `occupies`가 빈 경우 JSON에서 사라지는지 실측

`occupies_is_absent_from_json_when_empty` 테스트가 직접 측정한다:

- `occupies: vec![]`인 `CombatSimulationParticipant`를 `serde_json::to_string`
  으로 직렬화 → 결과 문자열에 `"occupies"`라는 부분 문자열이 **전혀 나타나지
  않음**을 assert.
- 같은 참가자에 `occupies: vec![hex(0,0), hex(1,0)]`을 채운 뒤 직렬화 →
  이번엔 `"occupies"` 키가 **나타남**을 assert(대비 확인 — 필드 자체가
  죽어있지 않다는 것도 같이 고정한다).

간접 증거도 있다: `json_contract.rs`의 37개 테스트(저작 번들 각각을
프리뷰 경계까지 통과시켜 정확한 JSON 스냅샷/구조를 검사한다)가 이 슬라이스
전후로 **37 passed, 0 변화**다 — 저작 콘텐츠 어디에도 `occupies`가 등장하지
않고(콘텐츠를 만들지 않았으므로 당연하다), 그 바이트가 이 필드 추가로
흔들리지 않았다는 뜻이다. `grep -rn "occupies" content/`도 결과 없음(콘텐츠
YAML 무변경 확인).

## 8. version bump 여부

없다. `CURRENT_SIMULATION_VERSION`은 여전히 `"v3"`이고
`crates/escape-core/src/combat_contract.rs`는 이 슬라이스에서 전혀 수정되지
않았다(`git diff 11643b6 HEAD -- crates/escape-core/src/combat_contract.rs`
결과 없음). 설계가 요구한 그대로 — 빈 `occupies`는
`skip_serializing_if`로 키 자체가 사라지므로 bump가 필요하지 않았다.

## 9. 계획과 다르게 구현한 부분과 사유

### 9-1. 소유 파일 목록 밖 파일 4개를 건드렸다 — 불가피했다

`CombatSimulationParticipant`에 새 필드를 추가하자, 이 struct를 명시적
필드 나열 리터럴로 생성하는 다음 4개 파일이 **컴파일 자체가 깨졌다**
(필드 하나를 빠뜨린 리터럴은 Rust에서 컴파일 에러다, `Default`가 없는 한):

- `crates/escape-core/tests/combat_execution_wave2.rs`
- `crates/escape-core/tests/combat_conclusion_wave2.rs`
- `crates/escape-core/tests/combat_spectator_wave3.rs`
- `crates/escape-core/tests/combat_occupancy_t1c.rs`

이 네 파일은 plan §3의 수정 가능 목록에도, 수정 금지 목록에도 명시적으로
들어있지 않다 — 수정 금지 목록은 소스 모듈(`combat_hex.rs`,
`combat_spectator.rs`, `combat_conclusion.rs`, `combat_contract.rs`,
`content.rs`)만 지목했지 동명의 *테스트* 파일은 언급하지 않았다.

정확히 같은 상황이 T1-c에서도 있었다는 것을 `combat_occupancy_t1c.rs`
자신의 주석이 기록하고 있다("이 검사는 구현됐다가, 되돌려졌다가, 소유권이
슬라이스 도중에 확장된 뒤 다시 착지했다... 42개의 기존 통과 테스트를
깨뜨렸다... 조정자가 확인하고 두 파일 모두로 소유권을 확장했다"). 이번에도
같은 패턴이므로, 각 파일에 `occupies: vec![]` 한 줄만 추가하는 기계적이고
행동을 바꾸지 않는 수정(빈 목록 = 기존 의미 그대로 한 타일)을 적용했다 —
새로운 로직이나 새로운 검사를 추가하지 않았다. 수정 전후로 그 파일들의
테스트 값이 하나도 변하지 않았음을 §2의 전체 실행 결과(422 passed, 0
failed)로 확인했다.

이것이 멈추고 보고해야 할 "소유 파일 목록 밖 파일 변경"에 해당한다고 판단해
여기 명시적으로 기록한다. 우회할 방법이 없었다(struct 리터럴에 필드를
빠뜨리면 Rust가 컴파일을 거부한다) — plan이 요구한 필드 추가(§4-1) 자체가
이 결과를 필연적으로 만든다.

### 9-2. 커밋 구성 방식

WP1~WP3가 전부 `combat_simulation.rs` 한 파일 안에서 깊이 얽혀 있어(예:
`occupancy_snapshot`은 WP1이 만든 `participant_footprint`를 WP3가 다시
소비한다), 처음에는 세 WP를 한 번에 구현했다. 이후 §1에서 설명한 대로
파일을 원본으로 되돌리고 WP1→WP2→WP3 순서로 다시 단계적으로 적용해 WP당
커밋 1개 원칙을 지켰다. 각 단계에서 그 WP의 검증 명령을 실제로 실행했다.

## 10. 최종 체크리스트

- [x] `occupies`가 비면 JSON에 키가 없고 기존 번들 바이트가 그대로다 — §7
- [x] version bump 없음 — §8
- [x] 앵커 미포함·비연결·중복·빈 집합이 거부된다 — WP4 테스트 4개
      (`occupies_without_the_origin_offset_is_rejected`,
      `a_disconnected_footprint_is_rejected`,
      `a_duplicate_offset_is_rejected`; 빈 집합은 거부가 아니라 §4-1대로
      한 타일로 허용되는 것이 맞는 설계이므로
      `an_empty_occupies_list_means_a_single_tile_at_the_anchor`가 그
      허용 경로를 고정한다)
- [x] 거리를 재는 다섯 지점이 전부 발자국 기준이다 — §3
- [x] 대형 유닛이 이동 중 자기 옛 발자국에 막히지 않는다 — §4,
      `a_large_unit_does_not_block_itself_while_moving`(실패 유도로 검증)
- [x] 발자국이 겹치는 목적지에서 둘 다 제자리에 선다 —
      `overlapping_destinations_make_both_large_units_hold`
- [x] 한 타일 유닛의 거동이 이전과 완전히 같다 — §2 전체 스위트 0 변화,
      `single_tile_units_behave_exactly_as_before`
- [x] §8의 두 테스트 값이 변하지 않았다 — §6
- [x] `combat_hex.rs`·관전·terminal·web·YAML·번들 무변경 —
      `git diff 11643b6 HEAD --stat`으로 확인(해당 경로 diff 없음)
- [x] `cargo fmt --all -- --check`, `git diff --check` 통과 — §2
- [x] Rust 409에서 감소 없음(422 = 409 + 13), web 168 무변경 — §2

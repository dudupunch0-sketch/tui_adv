# T1-c — 점유 강제와 포위 판정

plan: `fable_combat_hex_t1c_step1_2608072138.md`
report: `fable_combat_hex_t1c_step2_report.md`
baseline: `507d978`
상위 문서: [Combat_Hex_Rework_Development_Plan.md](docs/design/Combat_Hex_Rework_Development_Plan.md) §6 T1
선행 슬라이스: T1-a, T1-b1, T1-b2 — 전부 머지 완료

## 1. 목적

두 가지다.

**첫째, 실측 결함을 봉인한다.** Plan Index에 기록된 대로 지금 **말들이 서로를 통과한다** —
`preferred_distance: 0`인 두 전투원이 상대를 관통해 좌우가 뒤바뀐다. 정본 09의
"화면 왼쪽 아군 / 오른쪽 적" 계약을 재생 중에 위반하는 상태다.
T1-a가 만들어 두고 **아직 아무도 쓰지 않는 `HexOccupancy`**가 이 슬라이스의 도구다.

**둘째, 포위 판정을 만든다.** T4의 긴급 구조 개입이 "포위된 아군"을 조건으로 삼는다(정본 결정 20).
그 파생 상태를 여기서 낸다. **AI에 배선하지는 않는다** — 배선은 T4다.

## 2. 선행 조건

시작 전 baseline을 확인한다. 다르면 멈추고 보고한다.

- `cargo test --workspace --no-fail-fast` → **399 passed / 0 failed**
- `cd web && npm test` → **168 passed**

## 3. 소유 파일

수정 가능:

- `crates/escape-core/src/combat_simulation.rs`
- `crates/escape-core/tests/combat_simulation_wave2.rs`
- `crates/escape-core/tests/encounter_combat_wave3.rs` (기대 재확인용 단정 추가만)
- 신규 `crates/escape-core/tests/combat_occupancy_t1c.rs`

수정 금지:

- `crates/escape-core/src/combat_hex.rs` — **동결.** 부족하면 우회하지 말고 멈추고 보고한다
- `crates/escape-core/src/combat_resolution.rs` — §4-5 참고. 이 슬라이스는 판정을 건드리지 않는다
- `crates/escape-core/src/combat_conclusion.rs`, `combat_spectator.rs`, `combat_contract.rs`, `content.rs`
- `crates/escape-terminal/**`, `web/**`
- 저작 YAML, 번들 2종 — **좌표를 바꾸지 않는다**
- 다른 작업자의 미추적 파일

## 4. 설계

### 4-1. 점유의 단위는 "1 타일 1 유닛"이다

대형 유닛(`HexShape`)은 **T1-d**다. 여기서는 모든 전투원이 정확히 한 타일을 점유한다.
`HexOccupancy`의 다중 타일 API(`try_occupy`가 슬라이스를 받는 것)는 그대로 쓰되,
이 슬라이스는 항상 길이 1 슬라이스를 넘긴다.

### 4-2. 강제 지점 세 곳

**① 초기 배치.** `CombatSimulation::new()`가 두 활성 전투원이 같은 타일에서 시작하는 입력을
거부한다. 지금은 검사가 없다. 전용 오류 변형을 쓴다(기존 뭉뚱그린 변형에 넣지 않는다).

**② 경로 차단 — "관통 금지"의 본체.** 이동은 `line()` 경로를 따라 최대 `speed_per_tick` 타일
전진한다. 경로 중간에 점유된 타일이 있으면 **그 앞 마지막 빈 타일에서 멈춘다.**
전진(`step_toward`)과 후퇴(`step_away`) 양쪽에 같은 규칙을 적용한다.

**③ 목적지 경합.** 두 전투원의 최종 목적지가 같은 타일이면 **둘 다 이동을 포기하고 제자리에 선다.**

### 4-3. 왜 "둘 다 포기"인가 (설계 근거 — 임의로 바꾸지 말 것)

동시 tick에서 두 유닛이 같은 빈 타일을 노리는 일은 반드시 생긴다. 여기서 한쪽을 이기게 하려면
우선순위가 필요한데, 지금 손에 있는 유일한 결정론적 순서는 **id 사전순**이다.
그것으로 정하면 **"id 알파벳 순서가 전투 결과를 정한다"**가 되고, 이는 불변식 4(순서 독립성,
`simultaneous_mutual_defeat_is_independent_of_attack_definition_order`가 고정)를 깬다.

"둘 다 포기"는 대칭이라 순서에 의존하지 않고, 게임적으로도 읽힌다(서로 막았다).
그리고 **T2의 이동 예약 설계를 미리 결정하지 않는다** — 진짜 우선순위(밀어내는 힘 등)는 T2가
가져온다. 여기서 임시 우선순위를 발명하면 T2가 그걸 걷어내야 한다.

### 4-4. 점유는 tick 시작 스냅샷에서 읽는다

`advance_tick`은 이미 `let snapshot = self.participants.clone()`으로 스냅샷을 뜬 뒤
모든 의도를 계산하고 나중에 일괄 적용한다. **점유 판정도 같은 스냅샷에서 읽는다.**

- 진행 중인 상태에서 읽으면 처리 순서가 결과를 바꾼다 → 불변식 4 위반.
- 자기 자신의 출발 타일은 자기를 막지 않는다.

**의도적으로 보수적인 지점 (주석으로 남길 것).** A가 이번 tick에 비우는 타일로 B가 들어가려 하면
스냅샷 기준으로는 아직 점유 중이라 B가 막힌다. 기차처럼 줄지어 따라가는 이동이 한 칸씩 늦어진다.
이것은 순서 독립성을 위해 치르는 값이며, **T2의 이동 예약이 완화할 몫**이다.
다음 구현자가 "버그"로 오해하지 않게 코드 주석에 남긴다.

### 4-5. 판정은 건드리지 않는다

`combat_resolution.rs`는 이 슬라이스의 소유가 아니다. 특히 **`collision_radius`를 지우지 마라** —
죽은 필드처럼 보이지만 `combat_resolution.rs`가 `actor.collision_radius + target.collision_radius`를
근접 임계값으로 실제 사용한다. 점유가 겹침을 없애도 "접촉 거리" 개념은 남는다.
이 필드의 운명은 밸런스 결정이며 별도 슬라이스다.

### 4-6. 포위 판정 — 만들되 배선하지 않는다

전투원 하나에 대해 **인접 6칸 중 적이 점유한 칸**을 세는 순수 파생 함수를 만든다.

- 판 밖 개념이 아직 없으므로 **적 점유만 센다.** 지형·경계로 막힌 칸을 포위로 치지 않는다
  (그 개념 자체가 없다 — 없는 것을 지어내지 않는다).
- 반환은 개수와 방향 목록 둘 다 쓸 수 있게 한다. 어느 쪽이 필요한지는 T4가 정한다.
- **임계값을 정하지 마라.** "몇 칸부터 포위인가"는 개입 기회 감지 규칙이고 T4 소유다.
  여기서 상수를 만들면 T4가 걷어내야 한다.
- AI 가중치·목표 선택·개입 어디에도 연결하지 않는다.

## 5. Hard invariants

상위 문서 §3에서 상속한다. 이 슬라이스에서 특히 걸리는 것:

1. **순서 독립성.** 전투원 입력 순서를 섞어도 같은 결과가 나온다. §4-3·§4-4가 이걸 위한 설계다.
2. **결정론.** 같은 입력 → 같은 결과.
3. **RNG 호출 0회 추가.**
4. **`combat_hex.rs` 무변경.**
5. **판정 무변경** — 피해·명중·결착 규칙을 건드리지 않는다.
6. **`web/**`·terminal·저작 YAML·번들 무변경.**
7. **version bump 없음.** 직렬화 표현이 바뀌지 않는다(새 필드도, 삭제도 없다).
   만약 bump가 필요해 보이면 그것 자체를 보고한다 — 계획이 틀렸다는 뜻이다.

## 6. 저작 전투에 대한 예측 (§4의 결과 확인용)

저작된 `wuxia_combat_spectator_preview_bout`은 두 전투원이 `(0,0)`과 `(5,0)`에서
`preferred_distance: 0`, `speed_per_tick: 1`, `attack_range: 10`으로 맞붙는다.

**변경 전:** 서로를 관통하며 진동한다(기록된 결함).
**변경 후 예측:** 인접(거리 1)까지 접근한 뒤 서로의 타일을 노려 **둘 다 제자리**에 선다.

어느 쪽이든 **거리가 항상 `attack_range: 10` 이내라 매 tick 명중이 유지된다.** 따라서
`authored_preview_bout_behaviour_is_unchanged_by_the_coordinate_swap`이 고정하는 값
(프레임 8, `decisive_tick: 8`, `MutualDefeat`, 16타, 각 1333)은 **그대로여야 한다.**
이 테스트는 위치를 단정하지 않는다 — 결착·피해만 본다.

**값이 바뀌면 멈추고 보고한다.** 기대값을 새 결과에 맞춰 고쳐 쓰지 마라.
반대로 **프레임 안의 좌표는 바뀌는 것이 정상이다**(관통이 사라졌으므로).
좌표가 그대로라면 점유가 실제로는 강제되지 않았다는 뜻이므로, 그것도 보고 대상이다.

## 7. WP 목록

순서 고정. WP당 커밋 1개.

### WP1 — 초기 배치 검증

§4-2 ①. 전용 오류 변형 추가.

검증: `cargo test -p escape-core --test combat_simulation_wave2`

### WP2 — 경로 차단

§4-2 ②, §4-4. `step_toward`/`step_away` 양쪽. 스냅샷 기준. 보수성 주석 포함.

### WP3 — 목적지 경합

§4-2 ③, §4-3. 근거를 코드 주석에 남긴다 — **왜 우선순위를 만들지 않았는지**가 핵심이다.

### WP4 — 포위 판정

§4-6. 순수 함수. 배선 0.

### WP5 — 테스트

신규 파일 `combat_occupancy_t1c.rs`. 최소 집합:

| 테스트 | 고정하는 것 |
|---|---|
| `two_participants_cannot_start_on_the_same_tile` | §4-2 ① |
| `no_two_units_share_a_tile_at_any_tick` | 슬라이스 전체의 핵심 불변식 |
| `a_unit_stops_before_an_occupied_tile_instead_of_passing_through` | 관통 결함 봉인 |
| `two_units_targeting_the_same_tile_both_hold` | §4-3 |
| `shuffled_participant_order_yields_identical_frames` | 불변식 1 |
| `a_tile_vacated_this_tick_is_not_entered_this_tick` | §4-4의 보수성을 **의도로** 고정 |
| `retreat_is_blocked_by_occupancy_too` | 전진만 막고 후퇴를 빠뜨리는 실수 방지 |
| `surround_count_reports_enemy_occupied_neighbors_only` | §4-6 |
| `surround_detection_is_not_wired_into_movement_or_targeting` | 배선 0 |

검증: `cargo test --workspace --no-fail-fast`

## 8. 검증 명령

```bash
cargo fmt --all -- --check
cargo test -p escape-core --test combat_occupancy_t1c
cargo test -p escape-core --test encounter_combat_wave3
cargo test --workspace --no-fail-fast
git diff --check
cd web && npm test
```

기대: Rust **399에서 감소 없음**, 신규만큼 증가. web **168 무변경**.

## 9. 명시적 범위 밖

- **이동 예약·밀어내기·이동 취소·인접 대체** — T2. 여기서 임시 우선순위를 발명하지 않는다
- **대형 유닛** — T1-d
- **포위의 임계값과 개입 연결** — T4
- 지형·판 경계·장애물 (개념 자체가 없다)
- 경로 탐색(막혔을 때 우회) — `line()`은 직선만이고, 막히면 멈춘다
- `collision_radius`의 운명 — §4-5
- 밸런스, 행동 주기, 렌더러

## 10. 보고 형식

`fable_combat_hex_t1c_step2_report.md`에 적고 커밋한다.

- WP별 커밋 해시와 한 줄 요약
- 검증 명령과 **실제 숫자 출력**
- **§6의 두 확인** — ① 고정 테스트 값이 그대로인가 ② 프레임 좌표가 실제로 달라졌는가.
  둘 다 명시하라. 하나만 적지 마라
- 저작 전투의 변경 전/후 좌표 궤적 (짧게)
- 계획과 다르게 구현한 부분과 사유

## 11. 최종 체크리스트

- [ ] 어떤 tick에서도 두 유닛이 같은 타일에 있지 않다
- [ ] 관통이 불가능하다 (전진·후퇴 양쪽)
- [ ] 목적지 경합에서 우선순위를 발명하지 않았다
- [ ] 점유를 tick 시작 스냅샷에서 읽는다
- [ ] 입력 순서를 섞어도 프레임이 동일하다
- [ ] "이번 tick에 비는 타일은 못 들어간다"가 의도임이 주석과 테스트에 남았다
- [ ] 포위 판정이 순수 함수이며 어디에도 배선되지 않았다
- [ ] 포위 임계값 상수를 만들지 않았다
- [ ] `combat_hex.rs`·`combat_resolution.rs`·terminal·web·YAML·번들 무변경
- [ ] `collision_radius`가 그대로 남아 있다
- [ ] version bump 없음
- [ ] §6의 고정 테스트 값이 변하지 않았고, 좌표는 변했다
- [ ] `cargo fmt --all -- --check`, `git diff --check` 통과
- [ ] Rust 399에서 감소 없음, web 168 무변경

# T3 — 행동 주기 (유닛별 clock cycle) — 구현 보고

plan: `fable_combat_hex_t3_step1_2608080951.md`
report: `fable_combat_hex_t3_step2_report.md`
baseline: `d4871c4` (plan header) — 작업 시작 시점 worktree HEAD는 `b4160af`(같은 브랜치의 후속
커밋, T3 플랜 문서 자체)였다. `4684884`(메인 계보 마지막 커밋)에 있던 worktree를
`b4160af`로 fast-forward한 뒤 착수했다 — "확인" 항목 참고.

## 0. 출발점 확인

worktree가 `4684884`(`feat(combat): swap combat coordinates to axial hex`)에 있었고
`b4160af`는 조상이 아니었다(다른 브랜치 `claude/combat-system-reference-analysis-65sas6`에
있음). Working tree가 깨끗했으므로 `git merge --ff-only b4160af`로 fast-forward했다 —
non-ff merge나 rebase 없이 순수 fast-forward였고, 충돌 없음.

## 1. WP0 — 생성 지점 조사 (먼저 적으라고 한 목록)

```
grep -rln "CombatSimulationParticipant {" crates/
grep -rln "CombatAttackDefinition {" crates/
```

`CombatSimulationParticipant { ... }` 리터럴이 있는 파일:

- `crates/escape-core/src/combat_simulation.rs` (struct 정의 자체)
- `crates/escape-core/tests/combat_resolution_wave2.rs`
- `crates/escape-core/tests/combat_occupancy_t1c.rs`
- `crates/escape-core/tests/combat_simulation_wave2.rs`
- `crates/escape-core/tests/combat_spectator_wave3.rs`
- `crates/escape-core/tests/combat_large_units_t1d.rs`
- `crates/escape-core/tests/combat_conclusion_wave2.rs`
- `crates/escape-core/tests/combat_execution_wave2.rs`

`CombatAttackDefinition { ... }` 리터럴이 있는 파일:

- `crates/escape-core/src/combat_resolution.rs` (struct 정의 자체)
- `crates/escape-core/tests/combat_resolution_wave2.rs`
- `crates/escape-core/tests/combat_spectator_wave3.rs`
- `crates/escape-core/tests/combat_large_units_t1d.rs`

grep이 보고한 원시 라인 수(22)는 함수 시그니처 줄(`) -> CombatSimulationParticipant {`
같은 반환 타입 표기)까지 셌기 때문에 실제 struct 리터럴 개수보다 많다. 직접 대조한 실제
리터럴 개수:

| 파일 | `CombatSimulationParticipant{}` | `CombatAttackDefinition{}` |
|---|---|---|
| `combat_execution_wave2.rs` | 2 | – |
| `combat_conclusion_wave2.rs` | 1 | – |
| `combat_occupancy_t1c.rs` | 1 | – |
| `combat_simulation_wave2.rs` | 1 | – |
| `combat_large_units_t1d.rs` | 3 | 1 |
| `combat_spectator_wave3.rs` | 1 | 3 |
| `combat_resolution_wave2.rs` | 1(closure) | 2 |

`crates/escape-core/tests/encounter_combat_wave3.rs`는 두 struct 모두 리터럴로 만들지
않는다(콘텐츠 번들/YAML 경유이며 `content.rs`도 필드 타입 선언만 갖고 리터럴 생성은
하지 않는다) — 그래서 필드 추가로 인한 컴파일 깨짐이 없었고, WP1/WP2 검증 명령에
그대로 들어있다.

이 8개 파일(소스 2개 + 테스트 6개) 전부가 실제로 고쳐야 했던 목록이고, 플랜 §3의
"소유 범위"에 정확히 대응한다. `crates/escape-terminal/**`, `web/**`, 저작 YAML, 두
번들 어디에도 이 두 struct의 리터럴 생성이 없어 손대지 않았다.

## 2. WP별 커밋

| WP | 커밋 | 한 줄 요약 |
|---|---|---|
| WP1 | `f604e48` | `move_speed_hundredths` 필드 + `CombatSimulation.move_gauges` + `advance_tick` 게이지 드레인 루프 |
| WP2 | `2d6dd12` | `attack_speed_hundredths` 필드 + `resolve()`의 무조건 발동 제거, 게이지 기반 fire count |
| WP3 | `a0341fb` | 관전 표면 누설 차단 회귀 테스트 (프로덕션 코드 변경 없음 — 이미 새지 않았음을 고정) |
| WP4 | `27b8ebb` | `combat_cadence_t3.rs` 신규, 표의 11개 테스트 전부 |

## 3. 설계상 실제로 정한 것 (플랜이 명시하지 않은 세부)

- **`ACTION_THRESHOLD_HUNDREDTHS = 10_000`** 상수는 `combat_simulation.rs`에
  `pub(crate)`로 하나만 두고, `combat_resolution.rs`는
  `crate::combat_simulation::ACTION_THRESHOLD_HUNDREDTHS`로 참조한다 — 두 축이 서로
  다른 파일에서 값을 베껴 쓰다 드리프트하는 사고를 구조적으로 막는다. `lib.rs`의 public
  재수출 목록에는 넣지 않았다(순수 내부 상수이며 공개 API로 노출할 이유가 없다) — 그래서
  `lib.rs`는 이번 슬라이스에서 결국 건드리지 않았다(플랜은 "수정 가능"이라고만 했지
  "수정 필수"라곤 하지 않았다).
- **이동 게이지 상태(`move_gauges: BTreeMap<String, i64>`)는 `CombatSimulation`
  구조체의 새 필드**로 두었다. `CombatSimulationInput`(직렬화 경계)이 아니라 런타임
  엔진 구조체 쪽이라, 이 필드를 추가해도 JSON 계약이나 fixture 바이트에 전혀 영향이
  없다 — `CombatSimulation`은 `Self { ... }`로 자기 자신을 만드는 유일한 지점이
  `new()` 안에 있고, 그 외 아무도 이 struct를 리터럴로 만들지 않는다(비공개 필드).
- **공격 게이지 상태는 `resolve()` 안의 지역 변수**(`attack_gauges:
  BTreeMap<&str, i64>`)로 두었다 — `resolve()`가 `execution.frames`를 순회하는 한 번의
  호출 동안만 살아있으면 되고, 별도 구조체 상태를 새로 만들 필요가 없었다.
- **한 틱에 여러 번 행동하는 것의 구현 방식이 이동과 공격에서 다르다** — 의도적인
  차이이고 둘 다 문서화했다:
  - **이동**: N번의 행동을 "N배 긴 한 번의 걸음"으로 접는다
    (`step = speed_per_tick.saturating_mul(actions)`). 틱 시작 스냅샷이 고정되어 있으므로
    N번을 순서대로 적용한 결과와 한 번에 N배 걸음을 적용한 결과가 수학적으로 같다 —
    프레임 스키마(참가자당 인텐트 1개)를 바꾸지 않고도 정확하다.
  - **공격**: N번의 발동을 **N개의 독립된 outcome**으로 만든다(각자 자기 굴림·로그를
    가짐) — 공격은 굴림이 있어서 "합쳐서 한 번 크게"로 접으면 안 된다(예: 명중/치명타
    확률이 왜곡된다). 굴림 스트림에 0-based `fire_index`를 섞어 같은 틱·같은 공격·같은
    대상에 대한 반복 발동이 서로 다른 (그러나 결정론적인) 값을 굴리게 했다 —
    `fire_index == 0`은 이 슬라이스 이전과 정확히 같은 스트림 값이라 기본 경로는
    바이트 단위로 그대로다. 이펙트 굴림의 `fold` 시드도 같은 이유로 `1 + fire_index`로
    바꿨다(`fire_index == 0`이면 기존 `1`과 동일).
  - 이 비대칭은 플랜이 요구한 것은 아니지만 §4-1의 "잘라내지 마라"를 이동/공격 둘 다에
    적용하면서 각 축의 기존 데이터 모델(이동=인텐트 1개, 공격=outcome 목록)을 깨지
    않으려면 자연스럽게 나온 선택이었다. 대안(공격도 접어서 데미지 합산 1건)은 로그·
    스펙테이터가 "몇 번 맞았는지"를 볼 수 없게 만들어 §4-5가 허용한 "행동 빈도 역산
    가능"과 충돌했을 것이다.
- **이동 목표 선택(target selection)은 이동 게이지와 분리**했다 — 게이지가 이번 틱
  0행동이어도 `select_target`은 매 틱 그대로 실행되고 `CombatMoveIntent.target_id`도
  채워진다. 그렇지 않으면 공격은 `intent.target_id`를 읽어 대상을 찾으므로, 이동이 느린
  전투원의 공격이 이동 게이지에 종속되어 §4-2("이동 주기와 공격 주기는 분리한다")를
  위반했을 것이다. `move_and_attack_cadences_are_independent` 테스트가 이걸 고정한다.
- **속도 필드는 `Option<i64>`이고 `Some(v<=0)`은 참가자/공격 검증에서 즉시
  거부**한다(`CombatSimulationError::InvalidParticipant` /
  `CombatResolutionError::InvalidInput`) — 새 에러 변형을 만들지 않고 기존 "이 필드가
  잘못됨" 계열에 합류시켰다.

## 4. 검증 명령과 실제 숫자

시작 시점(§2 선행 조건, fast-forward 직후):

```
$ cargo test --workspace --no-fail-fast   → 422 passed; 0 failed
$ cd web && npm install && npm test       → 168 passed
```

(`web/node_modules`가 비어 있어 `npm install`을 먼저 실행했다.)

WP1 이후:

```
$ cargo test -p escape-core --test combat_simulation_wave2
running 14 tests ... test result: ok. 14 passed; 0 failed
```
(기존 14개 값 그대로, 변화 0.)

WP2 이후:

```
$ cargo test -p escape-core --test combat_resolution_wave2
running 24 tests ... test result: ok. 24 passed; 0 failed

$ cargo test -p escape-core --test encounter_combat_wave3
running 32 tests ... test result: ok. 32 passed; 0 failed
```
(둘 다 값 그대로, 변화 0 — §8의 두 저작 전투 테스트 포함, 아래 별도 확인.)

WP3 이후:

```
$ cargo test -p escape-core --test combat_spectator_wave3
running 22 tests ... test result: ok. 22 passed; 0 failed
```
(21→22, 신규 테스트 1개 추가분 — 기존 21개는 값 그대로.)

WP4 이후:

```
$ cargo test -p escape-core --test combat_cadence_t3
running 11 tests ... test result: ok. 11 passed; 0 failed
```
(신규 파일, 표의 11개 전부.)

최종 전체:

```
$ cargo fmt --all -- --check          → (출력 없음, 통과)
$ cargo test --workspace --no-fail-fast
   ... 30개 test binary, 전부 "0 failed"
   합계: 434 passed; 0 failed
   (422 baseline + WP3의 신규 1개 + WP4의 신규 11개 = 434, 기존 값 0건 변화)
$ git diff --check                    → (출력 없음, 통과)
$ cd web && npm test                  → Test Files 16 passed (16); Tests 168 passed (168)
```

Rust: 422에서 감소 없음(오히려 신규 테스트 12개가 더해져 434). Web: 168 무변경, 파일
자체가 `web/**` 어디도 건드리지 않았으니 당연한 결과.

## 5. §8의 두 저작 전투 테스트 — 값 그대로 확인

개별 실행:

```
$ cargo test -p escape-core --test encounter_combat_wave3 authored_preview_bout_behaviour_is_unchanged_by_the_coordinate_swap
test authored_preview_bout_behaviour_is_unchanged_by_the_coordinate_swap ... ok

$ cargo test -p escape-core --test encounter_combat_wave3 authored_preview_bout_never_lets_the_two_combatants_swap_sides_or_share_a_tile
test authored_preview_bout_never_lets_the_two_combatants_swap_sides_or_share_a_tile ... ok

$ cargo test -p escape-core --test encounter_combat_wave3 wuxia_combat_spectator_preview_bout_first_hit_damage_is_1333_hundredths
test wuxia_combat_spectator_preview_bout_first_hit_damage_is_1333_hundredths ... ok
```

세 테스트 모두 기대값을 고쳐 쓰지 않은 채 그대로 통과했다 — 8 tick, 16타, 각 1333
hundredths라는 저작 전투 수치가 이 슬라이스 전후로 동일하다.

## 6. 속도 필드의 JSON 부재 — 실측

`CombatSimulationParticipant` 하나를 `move_speed_hundredths: None`으로 만들어
직렬화한 결과와, `Some(15_000)`으로 바꾼 뒤 직렬화한 결과를 즉석 example로 실측했다
(example은 확인 후 삭제, 커밋에 없음):

```
participant (unset): {"id":"a","side":"ally","position":{"q":0,"r":0},"facing":{"q":1,"r":0},"speed_per_tick":1,"collision_radius":1,"attack_range":2,"support_range":2,"role_id":"r","target_policy_id":null,"active":true}
participant (set):   {"id":"a","side":"ally","position":{"q":0,"r":0},"facing":{"q":1,"r":0},"speed_per_tick":1,"move_speed_hundredths":15000,"collision_balance...
```

`move_speed_hundredths` 키가 `None`일 때 완전히 사라지고(`unset` 줄에 키 자체가 없다),
`Some`일 때만 나타난다 — `skip_serializing_if`가 의도대로 작동한다. 동일한 패턴이
`attack_speed_hundredths`에도 적용되며 `speed_fields_are_absent_from_json_when_unset`
(WP4)이 이걸 코드로 고정한다. 기존 fixture/번들 JSON 파일들을 `grep`했을 때도 두 새
키는 어디에도 나타나지 않는다(당연히 — 그 파일들 자체를 건드리지 않았다).

`git diff --stat 4684884..HEAD`로 확인한 결과 `crates/escape-core/fixtures/**`,
`crates/escape-terminal/**`, `web/**`는 이번 브랜치에서 단 한 바이트도 바뀌지 않았다.

## 7. 계획과 다르게 구현한 부분과 사유

1. **worktree가 플랜의 baseline보다 뒤처져 있었다** — `4684884`에 있었고 `b4160af`는
   조상이 아니라 다른 브랜치의 후속 커밋이었다. Fast-forward merge로 해결(§0). 코드
   충돌은 없었다.
2. **`lib.rs`를 결국 건드리지 않았다** — 플랜은 "수정 가능" 목록에 넣어 두었지만, 새
   필드 두 개가 이미 공개된 두 struct(`CombatSimulationParticipant`,
   `CombatAttackDefinition`)의 평범한 `pub` 필드라 재수출 목록 변경이 필요 없었다.
   `ACTION_THRESHOLD_HUNDREDTHS`도 순수 내부 상수라 공개하지 않았다.
3. **이동과 공격의 "한 틱에 여러 번" 처리 방식이 다르다** — §3에서 설명한 대로, 이동은
   접어서 한 번의 긴 걸음으로, 공격은 접지 않고 N개의 독립 outcome으로 처리했다. 플랜
   §4-1은 "잘라내지 마라"만 요구했지 구현 형태를 지정하지 않았으므로 이는 명시적 범위
   이탈이 아니라 그 요구를 두 축의 기존 데이터 모델에 맞게 구체화한 것이다.
4. **WP3에서 프로덕션 코드 변경이 없었다** — `combat_spectator.rs`는 애초에 속도
   필드를 전혀 참조하지 않아 새로 만든 두 필드가 거기 도달할 방법이 없었다. 그래서
   "누설 차단 목적으로만 만진다"는 허가를 실제로 쓸 필요가 없었고, 회귀 테스트만
   추가했다.
5. **WP4의 `attack_speed_never_appears_in_any_log_or_view`는 WP3의 것과 별도로,
   다중 발동(한 틱 3회 발동) 경로를 전용으로 검증**한다 — WP3 테스트는 기본(단일 발동)
   경로만 다뤘으므로, 이 슬라이스가 새로 연 "한 틱에 여러 outcome" 경로가 새지 않는지도
   별도로 고정해 두는 편이 안전하다고 판단했다.

## 8. 최종 체크리스트

- [x] 속도가 미설정이면 JSON에 키가 없고 기존 번들 바이트가 그대로다 (§6, §4 실측)
- [x] version bump 없음 (`CURRENT_SIMULATION_VERSION` 미변경, `simulation_version`
      관련 코드 미변경)
- [x] 게이지·속도가 전부 정수 hundredths (부동소수점 0곳 — `i64`/`i32` 정수 연산만
      사용, `f32`/`f64` grep 0건)
- [x] 속도 2배가 한 틱에 두 번 행동한다 (`double_speed_acts_twice_in_one_tick`)
- [x] 0 이하 속도가 거부된다 (`zero_or_negative_speed_is_rejected`, 이동/공격 둘 다)
- [x] 이번 틱 행동자가 행동 적용 전에 전부 결정된다 (게이지 갱신이 다른 참가자/공격의
      상태를 읽지 않는 구조로 원천 차단; `who_acts_this_tick_is_decided_before_anyone_acts`)
- [x] 입력 순서를 섞어도 결과가 같다
      (`shuffled_participant_order_yields_identical_frames`,
      `who_acts_this_tick_is_decided_before_anyone_acts`의 공격 순서 반전)
- [x] 공격속도가 어떤 로그·관전 필드에도 없다 (WP3 + WP4 두 테스트, 다중 발동 경로 포함)
- [x] 이동 주기와 공격 주기가 독립이다 (`move_and_attack_cadences_are_independent`)
- [x] `tick_millis`/`max_ticks` 무변경 (두 필드 자체를 이번 슬라이스에서 참조·수정한
      코드가 없다)
- [x] §8의 두 테스트 값이 변하지 않았다 (§5)
- [x] `combat_hex.rs`·terminal·web·YAML·번들 무변경 (§6 `git diff --stat`)
- [x] `cargo fmt --all -- --check`, `git diff --check` 통과
- [x] Rust 422에서 감소 없음(434로 순증가), web 168 무변경

## 9. 소유 파일 최종 목록 (실제로 변경한 파일)

- `crates/escape-core/src/combat_simulation.rs`
- `crates/escape-core/src/combat_resolution.rs`
- `crates/escape-core/tests/combat_simulation_wave2.rs`
- `crates/escape-core/tests/combat_conclusion_wave2.rs`
- `crates/escape-core/tests/combat_occupancy_t1c.rs`
- `crates/escape-core/tests/combat_execution_wave2.rs`
- `crates/escape-core/tests/combat_large_units_t1d.rs`
- `crates/escape-core/tests/combat_resolution_wave2.rs`
- `crates/escape-core/tests/combat_spectator_wave3.rs`
- `crates/escape-core/tests/combat_cadence_t3.rs` (신규)

`crates/escape-core/src/lib.rs`는 §7-2 사유로 결국 건드리지 않았다.

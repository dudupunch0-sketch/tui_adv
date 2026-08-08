# T3 — 행동 주기 (유닛별 clock cycle)

plan: `fable_combat_hex_t3_step1_2608080951.md`
report: `fable_combat_hex_t3_step2_report.md`
baseline: `d4871c4`
상위 문서: [Combat_Hex_Rework_Development_Plan.md](docs/design/Combat_Hex_Rework_Development_Plan.md) §6 T3
선행 트랙: T1 완결 (좌표계·점유·대형 유닛)

## 1. 목적

**시간 층을 만든다.** 지금 전투에는 tick이 있지만 "누가 언제 행동하는가"가 없다.

- 모든 전투원이 **매 틱** 이동한다.
- `resolve()`가 **매 틱 모든 `CombatAttackDefinition`을 무조건 발동**한다.
- 쿨다운·공격속도·행동 주기가 전무하다.

레퍼런스는 유닛마다 다른 리듬으로 움직인다. 사용자 결정 15는 **공격속도를 숫자로 노출하지 않고
내부 계산**하라고 정한다. 이 슬라이스가 그 내부 계산을 만든다.

## 2. 선행 조건

시작 전 baseline 확인. 다르면 멈추고 보고한다.

- `cargo test --workspace --no-fail-fast` → **422 passed / 0 failed**
- `cd web && npm test` → **168 passed**

## 3. 소유 파일

**필드를 추가하기 전에 `grep`으로 생성 지점을 먼저 센다.** 지난 세 슬라이스에서 소유 목록이
세 번 모자랐고, 원인은 매번 "의미적으로 바뀔 파일"만 세고 **"컴파일되려면 바뀌어야 하는 파일"**을
빠뜨린 것이었다. `CombatSimulationParticipant`와 `CombatAttackDefinition`을 명시적 구조체
리터럴로 만드는 곳은 전부 소유 범위다.

수정 가능:

- `crates/escape-core/src/combat_simulation.rs`
- `crates/escape-core/src/combat_resolution.rs`
- `crates/escape-core/src/lib.rs`
- `crates/escape-core/src/combat_spectator.rs` — **누설 차단 목적으로만**(§4-5)
- `crates/escape-core/tests/**` 중 위 두 구조체를 구성하는 **모든** 파일
  (착수 시 `grep -rln "CombatSimulationParticipant {\|CombatAttackDefinition {" crates/`로 확정하고
  그 목록을 보고서에 적는다)
- 신규 `crates/escape-core/tests/combat_cadence_t3.rs`

수정 금지:

- `crates/escape-core/src/combat_hex.rs` — 동결
- `crates/escape-core/src/combat_contract.rs`, `combat_conclusion.rs`, `combat_opportunity.rs`,
  `combat_state.rs`, `content.rs`
- `crates/escape-terminal/**`, `web/**`
- 저작 YAML, 번들 2종 — **콘텐츠를 만들지 않는다**
- 다른 작업자의 미추적 파일

## 4. 설계

### 4-1. 게이지 누적 모델을 쓴다 (간격 정수가 아니라)

"N틱마다 한 번"을 정수 간격으로 표현하면 1.5배속을 표현할 수 없고, 속도 버프가 붙는 순간
반올림 규칙을 발명해야 한다. 대신 **누적 게이지**를 쓴다.

```
매 tick: gauge += speed
gauge >= ACTION_THRESHOLD 이면 이번 tick에 행동하고 gauge -= ACTION_THRESHOLD
```

`ACTION_THRESHOLD`는 `10_000`(= 100.00)으로 두고, 속도는 이 코드베이스의 관례대로
**hundredths 고정소수점 정수**로 표현한다. 부동소수점을 쓰지 않는다.

- 속도 `10_000` = 매 틱 행동 (현재 동작)
- 속도 `5_000` = 두 틱에 한 번
- 속도 `20_000` = 한 틱에 두 번 — **이 경우 행동 횟수가 2회다.** 게이지가 임계를 두 번 넘으면
  두 번 행동한다. "한 틱에 한 번만"으로 잘라내지 마라. 잘라내면 속도 버프가 상한에 부딪혀
  조용히 사라진다.

### 4-2. 이동 주기와 공격 주기는 분리한다

레퍼런스는 사격하면서 자리를 옮기는 유닛을 보여준다. 하나의 행동 게이지로 묶으면
"이번 틱엔 이동, 다음 틱엔 공격"이 되어 그 리듬을 만들 수 없다.

| 게이지 | 어디에 | 의미 |
|---|---|---|
| 이동 | `CombatSimulationParticipant.move_speed_hundredths` | 얼마나 자주 **이동 판단**을 하는가 |
| 공격 | `CombatAttackDefinition.attack_speed_hundredths` | 얼마나 자주 **그 공격이 발동**하는가 |

공격 게이지가 공격 정의에 붙는 이유: 한 전투원이 여러 공격을 가질 수 있고 각각 리듬이 다르다.

`speed_per_tick`은 **건드리지 않는다.** 그건 "한 번 이동할 때 몇 칸"이고 주기와 다른 축이다.
이름이 이제 오해를 부르지만 개명은 또 하나의 경계 변경이라 이 슬라이스에서 하지 않는다 —
주석으로 두 축의 차이를 남긴다.

### 4-3. 기본값은 현재 동작이다 — bump 없음

```rust
#[serde(default, skip_serializing_if = "Option::is_none")]
pub move_speed_hundredths: Option<i64>,
```

`None` = **매 틱 행동**(= `ACTION_THRESHOLD`). 트리의 모든 전투원·공격이 여기 해당한다.

`skip_serializing_if`로 `None`일 때 JSON에 키가 나타나지 않으므로 **기존 번들·픽스처 바이트가
그대로 유지되고 version bump가 필요 없다.** bump가 필요해 보이면 설계가 틀린 것이니 보고한다.

`Some(0)` 이하는 "영원히 행동 안 함"이 아니라 **입력 오류로 거부**한다 — 값을 지어내지 않는다.

### 4-4. 순서 독립성을 지키는 방법

이번 틱에 **누가 행동하는가는 틱 시작 시점의 게이지로 먼저 전부 결정**하고, 그다음 행동을 적용한다.
게이지를 갱신하면서 그때그때 행동시키면 처리 순서가 결과에 섞인다.

기존 불변식(`shuffled_participant_order_yields_identical_frames` 등)이 이걸 고정하고 있으므로,
입력 순서를 섞은 테스트가 통과하는지 반드시 확인한다.

### 4-5. 공격속도는 어디에도 노출하지 않는다

사용자 결정 15다. 정본 13의 누설 차단(공격 굴림·억제 사유·Hidden 효과 id)과 같은 계열이며
상위 문서 §3 불변식 5가 이미 "여기에 공격속도를 추가한다"고 적어 두었다.

- `CombatSpectatorView`의 `core_log`·`full_log` 어디에도 속도 값이 실리지 않는다.
- `CombatSpectatorPiece`에 속도 필드를 만들지 않는다.
- 관전 표면에서 **행동 빈도를 역산할 수 있는 것은 무방하다** — 말이 언제 움직였는지는 보이는 정보다.
  금지되는 것은 **수치 자체를 실어 보내는 것**이다.

`combat_spectator.rs`를 이 목적으로만 만진다. 새 cue나 새 판정을 만들지 않는다.

### 4-6. tick 길이는 이 슬라이스에서 정하지 않는다

상위 문서 §9 미결 1(논리 tick 굵기)은 **T4로 남긴다.**

게이지 모델을 쓰면 주기가 tick 개수가 아니라 게이지 단위로 표현되므로, tick을 굵게 하든 잘게 하든
전투의 리듬 자체는 보존된다. tick 굵기가 실제로 문제가 되는 곳은 프레임 데이터 규모이고
그건 T4가 소유한다. `tick_millis`·`max_ticks`를 건드리지 마라.

## 5. Hard invariants

상위 문서 §3에서 상속한다. 이 슬라이스에서 특히 걸리는 것:

1. **기존 전투 무변경.** 모든 기존 전투원·공격이 `None` = 매 틱이므로 **어떤 기존 테스트 값도
   변하면 안 된다.** 저작 전투는 여전히 8 tick, 16타, 각 1333이어야 한다.
2. **JSON 바이트 무변경**, **version bump 없음.**
3. **순서 독립성** (§4-4).
4. **누설 차단** (§4-5).
5. **부동소수점 금지.** 게이지·속도는 전부 정수 hundredths.
6. **RNG 호출 0회 추가.** 주기는 결정론적이며 무작위 지터를 넣지 않는다.
7. **`combat_hex.rs` 무변경**, terminal·web·YAML·번들 무변경.

## 6. WP 목록

순서 고정. WP당 커밋 1개.

### WP0 — 생성 지점 조사

`grep -rln "CombatSimulationParticipant {" crates/`와 `CombatAttackDefinition {`로 목록을 뽑아
**보고서에 적는다.** 코드는 아직 고치지 않는다. 이 WP는 커밋 없이 보고서 초안에만 남겨도 된다.

### WP1 — 이동 주기

`ACTION_THRESHOLD` 상수, `move_speed_hundredths` 필드와 검증, `advance_tick`의 게이지 누적.
§4-1의 "한 틱에 두 번"을 잘라내지 않는다.

검증: `cargo test -p escape-core --test combat_simulation_wave2` — **값 변화 0**

### WP2 — 공격 주기

`attack_speed_hundredths`, `resolve()`의 무조건 발동 제거.

검증:
```bash
cargo test -p escape-core --test combat_resolution_wave2
cargo test -p escape-core --test encounter_combat_wave3
```
전부 **값 변화 0**.

### WP3 — 누설 차단

§4-5. 속도가 어떤 로그·관전 필드에도 나가지 않음을 테스트로 고정한다.

검증: `cargo test -p escape-core --test combat_spectator_wave3`

### WP4 — 테스트

신규 `combat_cadence_t3.rs`. 최소 집합:

| 테스트 | 고정하는 것 |
|---|---|
| `absent_speed_means_acting_every_tick` | §4-3 기본값 |
| `half_speed_acts_every_other_tick` | 기본 주기 |
| `double_speed_acts_twice_in_one_tick` | §4-1 — **잘라내지 않음** |
| `a_non_integer_multiple_speed_drifts_deterministically` | 게이지 모델의 핵심 이점 (예: 6600 → 2,1,2,1… 리듬) |
| `zero_or_negative_speed_is_rejected` | §4-3 |
| `move_and_attack_cadences_are_independent` | §4-2 |
| `who_acts_this_tick_is_decided_before_anyone_acts` | §4-4 |
| `shuffled_participant_order_yields_identical_frames` | 순서 독립성 (기존 성질 유지 확인) |
| `attack_speed_never_appears_in_any_log_or_view` | §4-5 |
| `speed_fields_are_absent_from_json_when_unset` | 불변식 2 |
| `existing_single_speed_combat_is_unchanged` | 불변식 1 |

검증: `cargo test --workspace --no-fail-fast`

## 7. 검증 명령

```bash
cargo fmt --all -- --check
cargo test -p escape-core --test combat_cadence_t3
cargo test --workspace --no-fail-fast
git diff --check
cd web && npm test
```

기대: Rust **422에서 감소 없음**, web **168 무변경**.

## 8. 저작 전투에 대한 예측

`wuxia_combat_spectator_preview_bout`의 전투원과 공격은 속도가 없다 = 매 틱.
따라서 **거동이 전혀 바뀌지 않아야 한다.**

`authored_preview_bout_behaviour_is_unchanged_by_the_coordinate_swap`과
`authored_preview_bout_never_lets_the_two_combatants_swap_sides_or_share_a_tile`이
**둘 다 값 그대로**여야 한다. 하나라도 움직이면 멈추고 보고한다. 기대값을 고쳐 쓰지 마라.

## 9. 명시적 범위 밖

- **이동 패스와 판정 패스의 인터리빙** — T4. 그래서 이 슬라이스 이후에도
  "전투불능 전투원이 결착 전 tick에서 여전히 이동한다"는 기존 결함이 남는다.
  고치는 척하지 말고 그대로 둔다
- **한 유닛 = 틱당 한 행동** 모델 — 위와 같은 이유로 T4
- 세그먼트 실행·개입·save 저장 — T4
- 논리 tick 굵기, 프레임 데이터 규모 — T4 (§4-6)
- 밸런스 값, 표준 전투원 수치 재설정 — T11
- 속도에 영향을 주는 스탯·스킬·패시브·장비의 실제 계산 — 이 슬라이스는 **속도 값을 받는 그릇**만
  만든다. 무엇이 그 값을 만드는지는 별도 트랙이다
- 콘텐츠 저작

## 10. 보고 형식

`fable_combat_hex_t3_step2_report.md`에 적고 커밋한다.

- **WP0의 생성 지점 목록** — 이걸 먼저 적어라
- WP별 커밋 해시와 한 줄 요약
- 검증 명령과 **실제 숫자 출력**
- **§8의 두 테스트가 값 그대로인지** 명시
- 속도 필드가 미설정일 때 JSON에 없음을 실측한 결과
- 계획과 다르게 구현한 부분과 사유

## 11. 최종 체크리스트

- [ ] 속도가 미설정이면 JSON에 키가 없고 기존 번들 바이트가 그대로다
- [ ] version bump 없음
- [ ] 게이지·속도가 전부 정수 hundredths (부동소수점 0곳)
- [ ] 속도 2배가 한 틱에 두 번 행동한다 (잘라내지 않음)
- [ ] 0 이하 속도가 거부된다
- [ ] 이번 틱 행동자가 **행동 적용 전에** 전부 결정된다
- [ ] 입력 순서를 섞어도 결과가 같다
- [ ] 공격속도가 어떤 로그·관전 필드에도 없다
- [ ] 이동 주기와 공격 주기가 독립이다
- [ ] `tick_millis`/`max_ticks` 무변경
- [ ] §8의 두 테스트 값이 변하지 않았다
- [ ] `combat_hex.rs`·terminal·web·YAML·번들 무변경
- [ ] `cargo fmt --all -- --check`, `git diff --check` 통과
- [ ] Rust 422에서 감소 없음, web 168 무변경

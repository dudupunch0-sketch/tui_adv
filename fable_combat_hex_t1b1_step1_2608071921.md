# T1-b1 — 좌표계 교체 (Rust 측: core + terminal)

plan: `fable_combat_hex_t1b1_step1_2608071921.md`
report: `fable_combat_hex_t1b1_step2_report.md`
baseline: `240ab6b`
상위 문서: [Combat_Hex_Rework_Development_Plan.md](docs/design/Combat_Hex_Rework_Development_Plan.md) §6 T1
선행 슬라이스: T0(`fable_combat_hex_t0_step1_2608061847.md`), T1-a(`fable_combat_hex_t1a_step1_2608061847.md`) — 둘 다 머지 완료

## 1. 목적

`CombatPosition { x, y }`(무한 정수 평면)을 T1-a가 만든 axial `HexCoord { q, r }`로 **교체**한다.

**이 슬라이스는 쪼갤 수 없다.** 좌표 타입을 바꾸면 `escape-core`와 `escape-terminal`이 동시에
컴파일되지 않으므로 중간 상태가 존재할 수 없다. 그래서 T1-a에서 수학을 미리 못박아
여기 남은 위험을 "타입 교체" 하나로 줄여 두었다.

Web(TypeScript)은 별도 빌드라 이 슬라이스에 넣지 않는다 — T1-b2가 받는다(§9).

## 2. 선행 조건

- T1-a의 `combat_hex` 모듈이 `escape_core`에서 재노출되어 있다.
- T0의 `CURRENT_SIMULATION_VERSION`과 두 지점 강제 검증이 살아 있다.
  **이 슬라이스의 version bump를 그 기구가 검증한다** — 저작·픽스처 어느 하나라도 빠뜨리면
  index-time 또는 runtime에서 즉시 떨어진다.

시작 전 `cargo test --workspace --no-fail-fast`로 baseline이 **392 passed / 0 failed**인지 확인한다.

## 3. 소유 파일

수정 가능:

- `crates/escape-core/src/combat_simulation.rs` (좌표 타입·이동·사거리)
- `crates/escape-core/src/combat_resolution.rs` (사거리 판정)
- `crates/escape-core/src/combat_spectator.rs` (프레임 좌표 전달)
- `crates/escape-core/src/combat_contract.rs` (`CURRENT_SIMULATION_VERSION` 값만)
- `crates/escape-core/src/lib.rs` (re-export 조정)
- `crates/escape-terminal/src/snapshot.rs` (보드 렌더 좌표 읽기)
- `crates/escape-core/tests/*.rs` 중 좌표를 구성하는 것 전부
- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml` (좌표·facing·version)
- 번들 2종 — **손으로 고치지 말고 `scripts/export_web_data.py`로 재생성**한다

수정 금지:

- `crates/escape-core/src/combat_hex.rs` — T1-a가 확정했다. **이 슬라이스에서 손대지 않는다.**
  부족한 게 있으면 우회하지 말고 멈추고 보고한다.
- `crates/escape-core/src/combat_conclusion.rs`, `combat_opportunity.rs`, `combat_state.rs`
  (좌표를 다루지 않는다 — 만약 컴파일이 깨진다면 그것 자체를 보고한다)
- `web/**` 전부 — T1-b2 소관
- 다른 작업자의 미추적 파일

## 4. 설계

### 4-1. `CombatPosition`을 남기지 않고 `HexCoord`로 대체한다

새 타입을 만들거나 `CombatPosition`을 육각용으로 개명하지 않는다. **삭제하고 `HexCoord`를 쓴다.**
`HexCoord`는 이미 `Serialize`/`Deserialize`/`Ord`/`Hash`를 갖고 있어 기존 용도를 모두 덮는다.

없어지는 메서드와 대체:

| 기존 | 대체 |
|---|---|
| `CombatPosition::distance_squared` | `HexCoord::distance` (제곱이 아니라 실제 육각 거리) |
| `CombatPosition::in_range(other, range)` | `a.distance(b) <= i64::from(range)` |
| `CombatPosition::overlaps(other, radius)` | 이 슬라이스에서는 **호출부를 지운다**(아래 4-5) |

JSON 표현이 `{"x":1,"y":2}`에서 `{"q":1,"r":2}`로 바뀐다. **breaking change이며 §4-4의 bump가 이를 덮는다.**

### 4-2. `CombatFacing`도 함께 육각화한다

facing을 직교좌표로 남기면 좌표는 육각인데 방향은 직교인 앞뒤 안 맞는 상태가 되고,
나중에 고치려면 **bump를 한 번 더** 해야 한다. bump가 비싸므로 지금 같이 한다.

- `CombatFacing`을 삭제하고 facing도 `HexCoord`로 표현한다.
- **`HexCoord::NEIGHBOR_DIRECTIONS`의 6개 값 중 하나여야 한다.** 아니면 거부한다.
  기존 검증(`facing.x == 0 && facing.y == 0`이면 `InvalidFacing`)을 이 규칙으로 대체한다 —
  영벡터는 6개 방향에 없으므로 자동으로 계속 거부된다.

### 4-3. 이동은 타일 단위가 된다

`speed_per_tick`의 의미가 "틱당 좌표 단위"에서 **"틱당 타일 수"**로 바뀐다.

- `step_toward`/`step_away`의 dominant-axis 분해를 버리고, `line(from, to)`가 준 경로를 따라
  최대 `speed_per_tick` 타일만큼 전진/후퇴한다.
- `line()`은 `Result`다. 에러를 `unwrap`하지 말고 기존 `CombatSimulationError`로 전파한다.
- **점유 규칙은 이 슬라이스에 없다.** 두 말이 같은 타일에 설 수 있고 서로를 통과한다.
  이는 현재 동작 그대로이므로 회귀가 아니다 — 금지는 T1-c가 넣는다.
  이 사실을 코드 주석에 남겨 다음 구현자가 "빠뜨린 것"으로 오해하지 않게 한다.

### 4-4. `simulation_version`을 `v2`에서 `v3`으로 올린다

직렬화 표현이 바뀌었으므로 필수다. 바꿀 곳은 세 종류다.

1. `CURRENT_SIMULATION_VERSION`
2. 저작 YAML의 `simulation_version`
3. 번들 2종 — **재생성**으로 반영한다

Rust 테스트 픽스처는 T0가 이미 상수를 참조하도록 정규화해 두었으므로 **손댈 필요가 없다.**
그것이 T0의 목적이었다. 만약 어떤 테스트가 여전히 리터럴을 들고 있다면 그 자체를 보고한다.

### 4-5. 손대지 않는 것

- `collision_radius`: 육각 점유에서는 의미가 없어지지만 **필드를 지우지 않는다.**
  지우는 것 자체가 또 하나의 경계 변경이고, 지금 이미 이동 해석에서 읽히지 않는다.
  `overlaps` 호출부만 정리하고 필드는 T1-c/T1-d가 처리한다. 주석으로 그 사실을 남긴다.
- `attack_range` / `support_range`: 필드는 그대로, **의미만** 유클리드에서 육각 거리로 바뀐다.

### 4-6. 저작 좌표 변환 — 여기서 회귀가 날 수 있다

저작된 전투(`wuxia_combat_spectator_preview_bout`)의 현재 값은 이렇다.

```yaml
전투원 A: position { x: 0, y: 0 }, facing { x:  1, y: 0 }
전투원 B: position { x: 5, y: 0 }, facing { x: -1, y: 0 }
speed_per_tick: 1, attack_range: 10
```

**`q = x`, `r = y`로 기계적으로 옮긴다.** 이 픽스처는 둘 다 `r = 0`이므로
육각 거리 `(|dq| + |dq+dr| + |dr|) / 2`가 `|dq|`로 줄어들어 **기존 유클리드 거리와 정확히 같다**(5).
`attack_range: 10 ≥ 5`도 그대로다. facing `(1,0)`·`(-1,0)`도 축 방향이다.
따라서 **이 전투의 거동은 바뀌지 않아야 한다** — main이 사전 검산한 결과다.

그러므로 다음 두 테스트의 값이 **변하면 안 된다.**

- `wuxia_combat_spectator_preview_bout_first_hit_damage_is_1333_hundredths`
- 조기 결착 관련 8 tick 계열 (`simulation_stops_at_the_tick_that_concludes_the_fight` 등)

**둘 중 하나라도 값이 달라지면 멈추고 보고한다.** 이 검산이 틀렸다는 뜻이고,
기대값을 새 결과에 맞춰 고쳐 쓰는 것은 이 슬라이스에서 명시적으로 금지한다.
`r ≠ 0`인 Rust 테스트 픽스처는 거리 의미가 실제로 달라질 수 있다 —
그런 픽스처를 발견하면 §10에 목록으로 보고한다.

### 4-7. terminal 보드 렌더

`snapshot.rs`는 `piece.position.x/y`로 min/max를 잡고 `(y, x)` 키로 격자를 그린다.
`(q, r)`로 바꾸되 **텍스트 격자라는 성질은 유지한다** — 폭 32·높이 16 상한과
초과 시 좌표 목록으로 대체하는 기존 fallback을 그대로 둔다.

육각을 텍스트로 정확히 그리려 하지 않는다. 육각을 화면에 제대로 투영하는 것은 T9의 일이다.
여기서는 `(q, r)`을 격자 좌표로 그대로 쓰고, **접근성 대체 표기의 좌표 라벨이
`(x, y)`가 아니라 `(q, r)`임을 표시**한다.

## 5. Hard invariants

상위 문서 §3에서 상속한다. 이 슬라이스에서 특히 걸리는 것:

1. **결정론.** 같은 입력이 같은 결과를 낸다. 순서 독립성(불변식 4)이 유지된다.
2. **RNG 호출 0회 추가.** 난수원을 새로 만들지 않는다.
3. **version 없이 깨지 않는다.** 직렬화 표현을 바꾸므로 `v3` bump가 **반드시** 동반된다.
4. **과거 기록 역직렬화.** T0가 세운 성질을 깨지 않는다 — 임의 version 문자열이 박힌 JSON은
   여전히 역직렬화된다(`{x,y}` 좌표가 `{q,r}`로 읽히지 않는 것은 별개 문제이며,
   그건 version 검증이 잡는 몫이다).
5. **판정 재계산 금지.** `combat_spectator.rs`는 좌표를 **옮기기만** 한다. 새 판정을 만들지 않는다.
6. **`combat_hex.rs` 무변경.**
7. **`web/**` 무변경.**

## 6. WP 목록

순서 고정. WP당 커밋 1개. **WP1~WP4는 중간에 컴파일이 깨진다 — 각 WP 끝에서 `cargo build`가
통과하도록 묶어라.** 통과시킬 수 없으면 그 WP를 앞 WP와 합쳐 커밋하고 사유를 보고한다.

### WP1 — `CombatPosition`/`CombatFacing` 제거와 `HexCoord` 대체

타입 교체와 그에 따른 컴파일 오류 해소만 한다. 이동 로직은 아직 기존 의미를 유지해도 된다.
`lib.rs` re-export 정리 포함.

검증: `cargo build --workspace`

### WP2 — 이동을 타일 단위로

§4-3. `line()` 기반 전진/후퇴. `Result` 전파.

검증: `cargo test -p escape-core --test combat_simulation_wave2`

### WP3 — 사거리 판정을 육각 거리로

§4-1의 대체표. `combat_resolution.rs`의 사거리·충돌 호출부 정리(§4-5).

검증: `cargo test -p escape-core --test combat_resolution_wave2`

### WP4 — terminal 보드

§4-7.

검증: `cargo test -p escape-terminal`

### WP5 — `v3` bump와 저작 좌표 변환

§4-4, §4-6. 번들은 `scripts/export_web_data.py`로 재생성한다(손편집 금지).

검증:
```bash
cargo test -p escape-core --test encounter_combat_wave3
cargo test --workspace --no-fail-fast
```
§4-6의 두 테스트 값이 그대로인지 **명시적으로 확인**하고 보고한다.

### WP6 — 테스트 보강

새로 생긴 성질을 고정한다. 최소 집합:

| 테스트 | 고정하는 것 |
|---|---|
| `position_serializes_as_q_and_r_not_x_and_y` | 직렬화 표현 전환 |
| `facing_must_be_one_of_the_six_neighbor_directions` | §4-2 |
| `facing_zero_vector_is_still_rejected` | 기존 성질이 새 규칙 아래서도 유지됨 |
| `speed_per_tick_moves_that_many_tiles` | §4-3 |
| `attack_range_is_measured_in_hex_distance` | §4-1 |
| `authored_preview_bout_behaviour_is_unchanged_by_the_coordinate_swap` | §4-6 |
| `v2_authoring_is_rejected_after_the_bump` | T0 기구가 bump를 실제로 지킨다 |

검증: `cargo test --workspace --no-fail-fast`

## 7. 검증 명령

```bash
cargo fmt --all -- --check
cargo build --workspace
cargo test -p escape-core --no-fail-fast
cargo test -p escape-terminal
cargo test --workspace --no-fail-fast
git diff --check
grep -rn '"v2"' crates/ src/tui_adv/storypack-previews/   # 비어 있어야 한다
```

기대: **392에서 감소 없음**, 신규 테스트만큼 증가.

번들 재생성 후 `python -m pytest tests/test_web_data_export.py`가 있으면 함께 돌리고 결과를 적는다.

## 8. 명시적 범위 밖

- **점유 강제**(1타일 1유닛, 관통 금지) — T1-c
- **대형 유닛**(`HexShape` 적용) — T1-d
- **포위 판정** — T1-c
- 경로 탐색(장애물 우회) — `line()`은 직선만이다
- 화면 좌표 투영, 고정 타일 메트릭 — T9
- Web/TypeScript 일체 — T1-b2
- wasm 재빌드 — 전투는 아직 `combat_spectator_preview_unlocked` 게이트 뒤에 있어
  일반 플레이 경로에 노출되지 않는다. 라이브 검증은 게이트를 푸는 슬라이스의 몫이다
- 행동 주기, 개입, 밸런스

## 9. 이 슬라이스가 남기는 상태 (T1-b2 인계)

이 슬라이스가 끝나면 **Rust는 육각, Web은 아직 직교**인 불일치 상태가 된다.
생성 번들의 좌표는 `{q, r}`인데 `web/src/core/types.ts`의 `CombatPoint`는 `{x, y}`를 말한다.

전투가 게이트 뒤에 있고 web 테스트가 손으로 만든 TS 픽스처를 쓰므로 web 테스트는
아마 깨지지 않는다 — **그러나 확인은 해야 한다.** 보고서에 `cd web && npm test` 결과를 적어라
(고치지는 말고, 결과만). T1-b2 계획서를 그 결과를 보고 쓴다.

## 10. 보고 형식

`fable_combat_hex_t1b1_step2_report.md`에 적는다.

- WP별 커밋 해시와 한 줄 요약
- 검증 명령과 **실제 숫자 출력**
- **§4-6의 두 테스트 값이 그대로인지** — 이 항목을 빠뜨리지 마라
- `r ≠ 0`이라 거리 의미가 실제로 달라진 테스트 픽스처 목록(있다면)
- `cd web && npm test` 결과 (고치지 않은 채로)
- 계획과 다르게 구현한 부분과 사유
- 컴파일 때문에 합친 WP가 있다면 그 사유

## 11. 최종 체크리스트

- [ ] `CombatPosition`·`CombatFacing`이 트리에서 사라졌다
- [ ] facing이 6방향 중 하나로 제한되고, 영벡터는 여전히 거부된다
- [ ] `speed_per_tick`이 타일 수로 동작한다
- [ ] 사거리가 육각 거리로 측정된다
- [ ] `grep -rn '"v2"' crates/ src/tui_adv/storypack-previews/`가 비어 있다
- [ ] 번들 2종이 **재생성**으로 갱신됐다 (손편집 흔적 없음)
- [ ] §4-6의 두 테스트 값이 변하지 않았다
- [ ] `combat_hex.rs` 무변경, `web/**` 무변경
- [ ] 점유 미강제가 의도임이 주석에 남았다
- [ ] `cargo fmt --all -- --check` 통과, `git diff --check` 통과
- [ ] `cargo test --workspace --no-fail-fast` 0 failed, 392에서 감소 없음

# Combat — 조기 결착 보고서

플랜: `fable_combat_early_conclusion_step1_2608022130.md`
Baseline: `66cd15d` (PR #187)
브랜치: `claude/combat-early-conclusion`
워크트리: `/home/dudu/worktrees/tui-adv-combat-early-conclusion`
(메인 체크아웃이 다른 작업자의 파일로 붐벼서 별도 워크트리를 썼다)

## 결과

| 항목 | 이전 | 이후 |
|---|---|---|
| 관전 프레임 수 | 10 | **8** |
| 마지막 프레임 tick | 10 | **8** |
| `core_log` | 40 | **32** |
| `full_log` | 80 | **64** |
| 로그 최대 tick | 10 | **8** |
| `decisive_tick` | 10 | **8** |
| `duration_millis` | 1100 | **900** |
| `simulation_version` | v1 | **v2** |

저작된 인카운터(`wuxia_combat_spectator_preview_bout`, seed 2)를 직접 덤프해
얻은 실측값이다.

## 정본 근거

- **정본 03**: "생명력은 … **0이면 주인공은 전투 불능/패배다.**"
  → 체력 0인 전투원이 공격 판정을 만드는 것은 이 문장과 모순이다.
- **정본 03**: "승리·패배·도주·항복·포획·목표 달성·강제 중단을 포함한 모든
  **전투 결착** 시 [상태이상을] 제거한다."
  → 결착은 정의된 사건이며, 결착 이후를 계속 시뮬레이션할 근거가 없다.
- **정본 03**: "이 결정성은 **같은 version 내부에서만** 보장한다 … 이후 갱신된
  version이 과거 seed 결과를 재현할 필요는 없다."
  → resolver 의미가 바뀌면 version을 올리는 것이 정본이 정한 방식이다.
- **종료 조건은 새로 만들지 않았다.** `conclude`가 이미 소유한 "진영 전멸"
  조건을 공용 함수로 뽑아 resolver가 같이 쓴다.

## 커밋

| hash | 내용 |
|---|---|
| `6bcef88` | 플랜 |
| `b302cfa` | 결착 조건 공용 함수 추출 (동작 무변경) |
| `7fa800d` | 전투불능 전투원은 행동하지 않는다 |
| `6c37ed3` | 결착 tick 이후를 시뮬레이션하지 않는다 |
| `2e37c84` | 관전 화면이 판정된 tick까지만 보여준다 |
| `497cadc` | authoring version v1 → v2 + 번들 재생성 |
| `c8abd63` | mutation test가 찾은 테스트 구멍 2개 메움 |

## 검증 (WSL 실측)

| 명령 | 결과 |
|---|---|
| `cargo fmt --all -- --check` | 통과 |
| `cargo test --workspace --no-fail-fast` | **354 passed, 0 failed** (이전 346) |
| `cargo test -p escape-terminal` | 24 + 61 passed (렌더러 무변경 확인) |
| `pytest tests/ -q` | 98 passed |
| `cd web && npx vitest run` | 16 파일 148 passed |
| export `--check` | 두 번들 up to date |
| `git diff --check` | 통과 |

번들 diff는 `simulation_version` 한 줄뿐이다 — 인카운터 수·id 목록은 그대로이며
웹 카운트 단정에 영향이 없다.

### mutation test 8건 중 7건이 잡혔다

| # | 깨뜨린 규칙 | 잡은 테스트 |
|---|---|---|
| M1 | 전투불능 전투원이 다시 행동 | `incapacitated_actor_and_incapacitated_target_are_both_skipped` |
| M2 | tick 시작 스냅샷 대신 실시간 체력 사용 | 같음 |
| M3 | 결착 시 `break` 제거 | `simulation_stops_at_the_tick_that_concludes_the_fight` 외 2건 |
| M4 | 빈 진영 가드 제거 | `a_side_with_no_active_participant_does_not_read_as_wiped_on_tick_one` |
| M5 | 추적 안 되는 id를 전멸로 읽음 | **잡히지 않음 — 도달 불가** (아래) |
| M6 | 관전 프레임 범위 미제한 | `spectator_view_never_extends_past_the_last_resolved_tick` |
| M7 | 관전 로그 범위 미제한 | 같음 |
| M8 | `side_all_defeated`가 0 아닌 체력도 전멸로 봄 | `all_outcomes_and_mutual_precedence_are_stable` 외 1건 |

**M5는 의도적으로 잡히지 않는다.** `validate_inputs`가 참가자와 상태의 일치를
이미 보장하므로 그 분기는 도달할 수 없고 테스트로 고정할 수 없다. 코드 주석에
"도달 불가한 방어 코드"라고 명시했다 — 독자가 테스트가 덮는다고 오해하지
않도록.

### 첫 mutation 라운드에서 내 테스트 2개가 구멍이었다

- 관전 범위 테스트가 **1 tick 픽스처**를 써서 두 프레임 출처를 구별하지 못했다.
  어떤 구현이든 통과했다. 5 tick으로 바꾸고, resolution만 첫 tick으로 줄인 뒤
  execution은 5개로 남겨 프레임 수가 정확히 1이어야 한다고 고정했다.
- 빈 진영 가드를 덮는 테스트가 없었다.

## subagent 위임 결과

sonnet subagent에 위임했으나 WP1 커밋 뒤 WP2 작업 중 출력이 깨져 중단됐다
(최종 보고가 태국어 문자 몇 개로 나왔다). WP1의 공용 함수 추출과 WP2의 구현·
테스트는 커밋되지 않은 상태로 남아 있었고, 검토 후 그대로 채택했다 — 특히
"tick 시작 체력 스냅샷으로 판단해야 공격 처리 순서에 의존하지 않는다"는 판단과
그것을 고정하는 순서 뒤집기 테스트는 정확했다. WP3~WP5는 오케스트레이터가 직접
구현했다.

## 관전 화면 범위 — 플랜에 없던 발견

resolver를 멈춘 뒤 다시 덤프해 보니 **프레임은 여전히 10개였다.** `spectate`가
truncate된 `resolution.frames`가 아니라 `execution.frames`(이동·AI 패스, 항상
tick 상한까지 생성)를 돌기 때문이다. 결착 뒤 두 tick 동안 cue도 로그도 없는 말이
계속 움직이고, 보고서의 `decisive_tick`과도 어긋났다.

관전 화면의 시간 범위를 **판정된 마지막 tick**으로 묶고, 병합 로그도 같은
범위로 걸렀다 (execution의 로그가 그 tick들의 이동 의도를 담고 있다).

## 남은 갭 (인덱스에 기록)

- **전투불능 전투원이 결착 전 tick에서 여전히 이동한다.** 한쪽이 먼저 쓰러지고
  다른 쪽이 나중에 쓰러지는 전투에서는 그 사이 tick에 쓰러진 말이 움직인다.
  `execute_combat`이 `resolve`보다 먼저 통째로 돌기 때문이며, 고치려면 두 패스를
  tick 단위로 인터리빙해야 한다.
- **코드가 `simulation_version`을 검증하지 않는다.** 저작이 잘못된 version을
  적어도 잡지 못한다. 강제하려면 기존 픽스처 전체를 `v1`에서 옮겨야 하므로 별도
  슬라이스다.
- **나머지 결착 유형(도주·항복·포획·목표 달성·강제 중단)의 판정 규칙이 정본에
  없다.** 구현된 것은 코드가 이미 소유한 두 조건뿐이다.
- **표준 대련이 여전히 양측 전멸로 끝난다.** 조기 결착 뒤에도 `MutualDefeat`이며
  남은 것은 밸런스·AI(방어 행동)다.
- **로그 도배**가 40→32로 줄었을 뿐, 정본 13의 "원시 사건을 연관 전투 상황으로
  묶는다"는 미구현이다.
- 두 말이 서로를 통과하는 문제(정본 09의 화면 축 계약 위반)는 손대지 않았다.

## 범위 밖

`combat_execution.rs`(이동·AI), 충돌·관통 규칙, 밸런스 수치, Web·terminal
렌더러 코드, 게이트 플래그, 전투원 표시 이름.

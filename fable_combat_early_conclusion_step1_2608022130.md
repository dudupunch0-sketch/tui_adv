# Combat — 조기 결착 설계 플랜: 전투불능 전투원의 행동 중단과 결착 tick 이후 시뮬레이션 중단

Baseline: `66cd15d` (PR #187, Wave 3 Step 1d-3)
브랜치: `claude/combat-early-conclusion`
워크트리: `/home/dudu/worktrees/tui-adv-combat-early-conclusion`

인덱스의 "아직 없는 것" 첫 항목(`고급 다수전 AI 행동·조기 결착/전투 tick 중단
resolver`) 중 **조기 결착** 부분을 담당한다. AI 행동·충돌 규칙은 범위 밖이다.

## 0. 왜 이 슬라이스인가 (실측 근거)

Step 1d-3에서 저작된 인카운터의 `ScenePage.combat`을 덤프해 확인한 것:

```
t8  ally@(2,0) cues=[Attack, Hit, Incapacitated]  ||  challenger@(3,0) cues=[Attack, Hit, Incapacitated]
t9  ally@(3,0) cues=[Attack, Hit, Incapacitated]  ||  challenger@(2,0) cues=[Attack, Hit, Incapacitated]
t10 ally@(2,0) cues=[Attack, Hit, Incapacitated]  ||  challenger@(3,0) cues=[Attack, Hit, Incapacitated]
report outcome=MutualDefeat reason=BothSidesDefeated decisive=Some(10) duration=1100
```

두 전투원의 체력은 tick 8에 0이 되지만(100 체력 / 첫 명중 13 = 8타),
**tick 9·10에도 공격과 피해 로그가 계속 쌓인다.** `core_log` 40건 중 8건이
이미 쓰러진 전투원이 만든 것이다.

## 1. 정본 근거

- **정본 03 (핵심 상태 시스템)**: "생명력은 전투와 월드 상태가 공유하는 유일한
  신체 건강 수치이며, **0이면 주인공은 전투 불능/패배다.**"
  → 체력 0은 전투 불능이다. 전투 불능인 전투원이 공격 판정을 만드는 것은
  이 문장과 모순이다.
- **정본 03**: "승리·패배·도주·항복·포획·목표 달성·강제 중단을 포함한 모든
  **전투 결착** 시 [상태이상을] 제거한다."
  → 결착은 정의된 사건이며 그 시점에 정리가 일어난다. 결착 이후를 계속
  시뮬레이션할 근거가 없다.
- **정본 03 (RNG·재시도·버전)**: "같은 고정층 + 변경층 + seed + 개입 ID/tick +
  simulation version은 같은 결과를 낸다. 이 결정성은 **같은 version 내부에서만**
  보장한다. … 이후 갱신된 version이 과거 seed 결과를 재현할 필요는 없다."
  → resolver 의미가 바뀌면 version을 올리는 것이 정본이 정한 방식이다.
- **정본 03**: "전투 기록에는 version을 저장한다."
  → 이미 `CombatExecutionResult.provenance.simulation_version`이 담당한다.
- **종료 조건 자체는 이미 코드가 소유한다**: `combat_conclusion.rs::conclude`가
  "한 진영의 모든 활성 전투원의 `current_health_hundredths == 0`"을 전멸로 보고
  `AllEnemiesDefeated`/`AllAlliesDefeated`/`BothSidesDefeated`를 만든다. 이
  슬라이스는 **새 종료 규칙을 만들지 않고** 그 조건을 resolver가 같이 지키게
  한다.

### 정본에 없어서 이 슬라이스가 하지 않는 것

정본 01·09·13 어디에도 전투 종료 조건의 **목록**(도주·항복·포획·목표 달성·강제
중단의 판정 규칙)은 정의돼 있지 않다. 인덱스도 "대형·결속·배경 전투·증원과
전투 종료 조건"을 미구현으로 남겨 두었다. 따라서 이 슬라이스는 **이미 코드가
가진 두 조건(한쪽 전멸 / 양쪽 전멸)만** 다루고 나머지 결착 유형을 발명하지
않는다.

## 2. Scope

1. `combat_resolution.rs`: 체력이 0 이하인 전투원은 공격을 만들지 않는다.
2. `combat_resolution.rs`: 결착 조건이 성립한 tick까지만 프레임·로그를 만든다.
3. authoring: `simulation_version` `v1` → `v2` + 번들 재생성.
4. 기존 테스트 기대값 갱신 + 신규 테스트.
5. 문서: 인덱스의 core 결함 블록 갱신, 남은 갭 명시.

## 3. Hard invariants

### I1. 전투불능은 행동하지 않는다

`resolve`의 공격 루프에서 **actor의 그 tick 시작 시점 체력이 0 이하면 그
공격을 건너뛴다.** 판정도, 로그도, outcome도 만들지 않는다.

- 판정 기준은 **그 tick의 공격 적용 전 체력**이다. 같은 tick 안에서 먼저
  처리된 공격으로 죽은 전투원이 반격하지 못하게 되면 공격 처리 순서
  (`attack_map`의 BTreeMap 순서)가 결과를 바꿔 결정성이 순서 의존이 된다.
  **tick 시작 시점 스냅샷으로 판단한다** — 같은 tick의 상호 타격(동시 결착)은
  성립한다.
- 대상(target)이 이미 전투불능인 경우도 같은 규칙으로 건너뛴다 — 쓰러진 상대를
  계속 때리는 로그를 만들지 않는다.

### I2. 결착 tick 이후를 시뮬레이션하지 않는다

한 tick의 모든 공격을 적용한 뒤, `conclude`가 쓰는 것과 **같은 조건**으로
결착을 판정한다: 어느 한 진영의 모든 활성 전투원이 체력 0이면 결착이다.
결착이면 그 tick의 프레임을 마지막으로 남기고 루프를 끝낸다.

- 결착 판정 로직은 **한 곳에만** 둔다. `conclude`의 조건을 복사하지 않고
  공용 함수로 뽑아 둘이 같이 쓴다 — 두 곳에 같은 규칙을 적으면 갈라진다.
- `max_ticks` 도달로 끝나는 기존 경로는 그대로다.
- 결착 tick의 프레임은 **포함한다** (결착이 일어난 tick을 보여줘야 한다).

### I3. 종료 조건을 발명하지 않는다

도주·항복·포획·목표 달성·강제 중단은 만들지 않는다. 균형 붕괴·호흡 고갈로
결착시키지 않는다. 정본에 판정 규칙이 없다.

### I4. version을 올린다

resolver 의미가 바뀌므로 저작 매니페스트의 `simulation_version`을 `v2`로
올린다. 근거를 YAML 주석과 문서에 남긴다.

- **코드가 지원 version을 검증하는 것은 이 슬라이스 범위 밖이다** (기존 픽스처
  전체가 `v1`을 쓰므로 별도 슬라이스가 필요하다). 검증이 없다는 사실을 인덱스에
  **알려진 갭으로 기록한다** — 지금은 저작이 잘못된 version을 적어도 코드가
  잡지 못한다.

### I5. 결정성

같은 입력에서 같은 결과가 나온다. 공격 처리 순서에 의존하지 않는다(I1).
결착 판정은 tick 단위이며 `BTreeMap`/정렬된 순회만 쓴다.

### I6. fingerprint 변화는 계약 위반이 아니다

프레임 수·로그 수가 줄어 `resolution.fingerprint`·`execution.fingerprint`·
`report.fingerprint`·`view.fingerprint`가 모두 바뀐다. 정본 03이 "결정성은 같은
version 내부에서만"이라고 정했고 version을 올리므로 계약 위반이 아니다.
fingerprint를 **하드코딩한 기대값이 있으면 갱신**하고, 갱신한 곳마다 이유를
주석에 남긴다.

### I7. 건드리지 않는 것

- `combat_execution.rs`의 이동·AI (전투불능 전투원이 결착 전 tick에서 여전히
  **이동**하는 문제는 execution↔resolution 인터리빙이 필요하다 → 범위 밖,
  갭으로 기록)
- 충돌·관통 규칙 (두 말이 서로를 지나치는 문제)
- 밸런스 수치 (표준 전투원끼리 상호 전멸하는 것 자체)
- Web·terminal 렌더러 (core 결과만 바뀌면 렌더러는 그대로 따라간다)
- `crates/escape-terminal/tests/cli_smoke.rs`
- 게이트 플래그

### I8. 렌더러 회귀 확인

`web/`·`crates/escape-terminal/`의 코드는 수정하지 않지만, 프레임 수가 줄어
Web 재생 길이가 `(frames-1) × tick_millis`로 짧아진다. `npm test`와
`cargo test -p escape-terminal`이 그대로 통과해야 한다. 통과하지 않으면
렌더러가 프레임 수를 암묵적으로 가정하고 있었다는 뜻이므로 보고한다.

## 4. 예상 변경 파일

| 파일 | 변경 |
|---|---|
| `crates/escape-core/src/combat_resolution.rs` | 전투불능 actor/target 건너뛰기, 결착 tick에서 루프 종료 |
| `crates/escape-core/src/combat_conclusion.rs` | 결착 조건을 공용 함수로 추출 (동작 무변경) |
| `crates/escape-core/tests/combat_resolution_wave2.rs` | 신규 테스트 + 기대값 갱신 |
| `crates/escape-core/tests/combat_conclusion_wave2.rs` | 기대값 갱신 |
| `crates/escape-core/tests/encounter_combat_wave3.rs` | 기대값 갱신 (duration·frame 수) |
| `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml` | `simulation_version: v2` + 근거 주석 |
| 두 번들 JSON | export 재생성 (직접 편집 금지) |
| `docs/design/Combat_System_Implementation_Plan_Index.md` | core 결함 블록 갱신, 남은 갭 |

## 5. 작업 패키지 (WP당 커밋 1개)

### WP1 — 결착 조건 공용 함수 추출 (동작 무변경)
`conclude`에서 "진영 전멸" 판정을 순수 함수로 뽑는다. 기존 테스트가 전부 그대로
통과해야 한다(리팩터링만).
커밋: `refactor(combat): extract the side-defeated condition so one rule has one home`

### WP2 — 전투불능 전투원은 행동하지 않는다 (테스트 먼저 red)
테스트가 고정할 것:
- 체력 0인 actor의 공격이 outcome·로그에 나타나지 않는다.
- 체력 0인 target을 향한 공격도 만들어지지 않는다.
- **같은 tick 상호 타격은 성립한다** (tick 시작 스냅샷 기준이라 처리 순서에
  의존하지 않는다).
- 공격 정의 순서를 뒤집어도 같은 결과가 나온다 (결정성).
커밋: `fix(combat): an incapacitated combatant no longer acts`

### WP3 — 결착 tick 이후 중단 (테스트 먼저 red)
테스트가 고정할 것:
- 한쪽 전멸 tick이 마지막 프레임이고 그 이후 프레임·로그가 없다.
- 결착 tick의 프레임은 포함된다.
- 결착이 없으면 `max_ticks`까지 그대로 돈다.
- 양쪽 동시 전멸도 그 tick에서 멈춘다.
커밋: `fix(combat): stop simulating after the tick that concludes the fight`

### WP4 — authoring version bump + 번들 재생성
`simulation_version: v1` → `v2`, 근거 주석. export `--write` 후 `--check`.
커밋: `content(combat): bump the preview bout to simulation version v2`

### WP5 — 문서
인덱스의 core 결함 블록에서 해결된 항목을 해결로 옮기고, 남은 갭(전투불능
전투원의 이동, 관통, 밸런스, version 미검증, 나머지 결착 유형)을 명시한다.
수치를 쓰면 그 수치를 고정하는 테스트 함수명을 같이 적는다.
커밋: `docs(combat): record what early conclusion fixed and what it did not`

## 6. 검증

```bash
cargo fmt --all -- --check
cargo test --workspace --no-fail-fast
git diff --check
./.venv/bin/python -m pytest tests/ -q
cd web && npm test
```
번들 재생성 확인:
```bash
./.venv/bin/python scripts/export_web_data.py \
  --storypack-preview wuxia_jianghu_pack \
  --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --check
```

오케스트레이터가 추가로 확인할 것: 저작된 인카운터를 다시 덤프해 프레임 수·
로그 수·`decisive_tick`·`duration_millis`가 기대대로 바뀌었는지, 쓰러진 전투원의
공격 로그가 사라졌는지.

## 7. 명시적 범위 밖

- 전투불능 전투원의 **이동** 중단 (execution↔resolution 인터리빙 필요)
- 충돌·관통 규칙, 다수전 AI 행동
- 도주·항복·포획·목표 달성·강제 중단 결착 유형 (정본에 판정 규칙 없음)
- 균형·호흡 고갈에 의한 결착
- 코드의 simulation version 검증
- 밸런스 확정 (표준 전투원끼리 상호 전멸)
- 로그 묶기(정본 13의 "연관 전투 상황으로 묶어 로그 도배를 막는다")
- 전투원 표시 이름, 게이트 제거
- 치유·명줄·패배 결과

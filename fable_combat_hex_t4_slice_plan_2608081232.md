# T4 실행 모델·개입·지속 — 3~4 슬라이스 분해 플랜

status: in-progress
date: 2026-08-08
baseline_commit: `2ae781fdb427b94f10353529aaddb1341d986b1b`
baseline_test: `cargo test --workspace --no-fail-fast` = **434 passed / 0 failed**
owner: 오케스트레이터가 분해·검수하고, 각 구현 슬라이스는 `gpt-5.6-luna` medium subagent에 위임한다.

## 1. 이 문서의 역할

T4 전체를 한 번에 구현하지 않기 위한 **분해 플랜**이다. 이 문서는 구현자에게 바로 넘기는
step plan이 아니다. 선행 슬라이스가 확정한 public contract와 테스트를 읽은 뒤 다음 슬라이스의
`fable_combat_hex_t4_step1_<YYMMDDHHMM>.md`를 별도로 작성한다.

따라서 아래 S2~S4의 타입명·직렬화 모양은 예약하지 않는다. 다음 단계 schema는 앞 단계 보고서가
확정한 계약으로 다시 쓴다.

읽은 정본:

- `docs/design/Combat_Hex_Rework_Handoff.md`
- `docs/design/Combat_Hex_Rework_Development_Plan.md` §5~§10
- `docs/dev/Implementation_Slice_Discipline.md`
- `docs/dev/Combat_System_Operating_Guide.md`

## 2. 현재 측정값과 경계

현재 main은 PR #197 이후의 T1-c·T1-d·T3 구현과 PR #198 문서까지 포함한다. 실행 경로는 아직
다음과 같다.

```text
execute_combat: CombatSimulation::run_ticks 전체 실행
    → resolve_combat: execution.frames 전체를 순회하며 판정
    → conclude_combat
    → spectate_combat
```

즉, 실행과 판정이 한 tick씩 인터리브되지 않고, `ScenePage` producer는 렌더 호출마다 전투를
처음부터 재실행한다. `CombatOpportunityCatalog::evaluate`는 이미 결정론적 후보·옵션·0~3 예산을
계산하지만 현재 tick 흐름에 연결되어 있지 않다.

현 코드에서 T4가 소비하는 기존 계약:

- `CombatSimulation::advance_tick` / `run_ticks`
- `CombatExecutionRequest` / `CombatExecutionResult`
- `CombatResolutionRequest` / `CombatResolutionFrame` / `CombatResolutionState`
- `CombatOpportunityCatalog::evaluate`와 `CombatOpportunityEvaluation`
- `CombatState::active_effects`, `CombatEffectInstance`, 기존 `roles`/`policies`
- `CombatManifest::derived_seed` 및 `CombatRngNamespace`

동결·수정 금지 경계:

- `crates/escape-core/src/combat_hex.rs`의 좌표·이웃 순서·line tie-break·형태 정규형
- 렌더러가 판정을 재계산하지 않는 원칙
- 예측 RNG와 실제 전투 RNG 분리
- 전투불능을 회복으로 되돌리지 않는 규칙(부활은 T5)
- 정해지지 않은 밸런스 수치를 코드에 고정하지 않는 규칙

## 3. 모든 T4 슬라이스가 상속하는 불변식

1. 같은 manifest·seed·선택 이력·simulation version은 같은 결과·로그·fingerprint를 만든다.
2. tick 내부 순서는 **판정 적용 → 개입 기회 감지 → 결착 판정**이다. 개입 기회가 있으면 결착을
   보류하고 해당 tick에서 정지한다.
3. 개입 tick은 core가 결정한다. renderer는 marker까지 재생하고 멈출 뿐, wall-clock으로 정지
   시점을 계산하지 않는다.
4. 전투불능 actor/target은 이후 이동·공격의 주체가 되지 않는다. 결착 tick 뒤를 계산하지 않는다.
5. presentation speed는 판정·seed·fingerprint에 영향을 주지 않는다. 배속은 재생 계층의 값이다.
6. 상태·효과·선택 이력은 정렬 가능한 stable id와 명시적 provenance를 갖는다. 입력 오류는
   `Result`로 거부하고 패닉·`let _ =` 삼킴을 쓰지 않는다.
7. 기존 고정 테스트의 판정값·fingerprint가 바뀌면 조용히 기대값을 고치지 말고 정지·보고한다.
   표시/투영만 바뀐 경우에는 새 계산식과 변경 목록을 보고한다.

## 4. 선택한 분해: 4개 슬라이스

T4는 4개로 나눈다. 가장 틀리기 쉬운 tick resolver 추출, simulation↔resolution interleaving,
가장 되돌리기 비싼 save boundary를 분리하고, 개입 효과와 consumer 연결은 계약이 안정된 뒤에 한다.

| 순서 | 슬라이스 | 핵심 질문 | 다음 단계로 넘길 산출물 |
|---|---|---|---|
| S1 | tick resolver primitive·active 경계 | monolithic resolver를 한 tick씩 소비 가능한 stepper로 어떻게 추출하는가? | 확정된 `CombatResolutionStepper` 계약, 프레임/상태 불변식, 회귀 테스트 |
| S2 | interleaved runtime·개입 정지·세그먼트 seed | 이동·판정·KO 반영·어느 tick 정지와 선택 seed를 어떻게 묶는가? | 확정된 runtime/pause marker/selection history/seed provenance 계약 |
| S3 | 진행 상태 저장·델타·재개 | 매 렌더 재실행 없이 어디까지 저장하고 복원하는가? | 확정된 save/JSON schema, backward-compatibility 규칙, 크기 측정 |
| S4 | 개입 되먹임·재생/consumer 연결 | 효과·명령·긴급 구조를 적용하고 모든 표면이 core marker를 소비하는가? | 확정된 response application API, replay-speed/adapter 계약, end-to-end 테스트 |

S1 완료 후 S2 plan을 쓴다. S2 완료 후 S3, S3 완료 후 S4 plan을 쓴다. S1~S4를 미리 한
PR이나 한 coding agent에 묶지 않는다.

## 5. S1 — tick resolver primitive·active 경계

### 목적

현재 monolithic `resolve`의 판정·효과·로그 누적을 `CombatResolutionStepper`로 추출한다.
batch `resolve_combat`는 기존 execution frames를 stepper에 순서대로 공급한다. simulation과
stepper를 실제로 한 tick씩 묶고, KO를 다음 tick에 반영하고, 결착 뒤를 끊는 runtime은 S2에서
구현한다. 이 단계는 개입 선택을 받지 않는다.

### 후보 소유 범위

실제 step plan 작성 시 코드 실측으로 다시 확정한다. 예상 범위:

- `crates/escape-core/src/combat_simulation.rs`
- `crates/escape-core/src/combat_execution.rs`
- `crates/escape-core/src/combat_resolution.rs`
- S1에서는 신규 runtime 모듈을 만들지 않는다.
- 신규 `crates/escape-core/tests/combat_hex_t4_runtime_step1.rs` (stepper/active 경계 테스트)

`combat_hex.rs`, terminal/Web adapter, save schema는 S1 소유가 아니다.

### acceptance

- 기존 batch API의 결과·로그·fingerprint가 유지된다.
- 동일 입력을 두 번 stepper에 공급하면 같은 frame/state/fingerprint가 나온다.
- inactive 입력 participant는 이동·target pass에서 제외된다.
- stepper가 tick-start health snapshot, attack gauge, effect, full/core log를 누적한다.
- 공격 정의 순서·참가자 입력 순서를 바꿔도 결과가 같다.

### 범위 밖

simulation↔resolution interleaving, 결착 tick 중단, 개입 후보 평가, 선택 이력 seed, save/serde
schema, 델타 인코딩, 즉발 효과·명령 변경·긴급 구조, renderer 변경.

### S1 정지 조건

- `CombatTickFrame`/`CombatResolutionFrame` 변경이 기존 fingerprint를 바꾸는데 이유가 명확하지 않다.
- stepper의 state/log/effect ownership 경계가 두 가지 이상으로 해석된다.
- 기존 fixture 판정값이 바뀐다.
- `combat_hex.rs` 수정 없이는 구현할 수 있다고 확정할 수 없다.

## 6. S2 — core 소유 개입 정지·세그먼트 seed

### 목적

S1의 tick-step 결과 뒤에 기존 `CombatOpportunityCatalog::evaluate`를 연결한다. core가 개입
marker를 내고, renderer는 marker tick까지 재생한 뒤 입력을 기다린다. 선택 이력·segment 순서·
기존 manifest fingerprint를 seed provenance에 결합해 같은 선택 이력의 재개 결과를 보장한다.

### 입력·출력 경계(개념)

- 입력: 직전 tick 결과, opportunity instances/context, budget, 이전 선택 이력, 선택 response id
- 출력: 진행/정지 결정, 후보·옵션·free alert, consumed budget, 다음 구간 seed provenance

구체 타입·serde 필드는 S1 public contract를 읽은 뒤 S2 plan에서 확정한다.

### acceptance

- 기회가 감지된 정확한 tick에서만 정지 marker가 생긴다.
- 같은 manifest·seed·선택 이력·version은 같은 marker/evaluation/다음 segment fingerprint를 만든다.
- 다른 유효 response 선택은 선택 이력과 후속 결과를 바꾼다.
- 기존 후보 정렬, observer tie-break, 옵션 최대 4개, no-intervention, 0~3 예산 semantics를 재사용한다.
- forecast namespace가 actual combat RNG를 재사용하지 않는다.

### 범위 밖

save schema·델타 저장, renderer/Web/terminal 연결, 실제 response effect 적용, role/policy 변경,
긴급 구조 AI.

### S2 정지 조건

- 기존 opportunity evaluation이 선택 이력·재개 seed와 충돌한다.
- seed 파생에 새 RNG source가 필요해진다.
- no-intervention의 budget 소비 semantics가 현재 코드와 다르게 해석된다.
- pause marker가 renderer 시간이나 프레임 개수로 결정된다.

## 7. S3 — 진행 상태 저장·델타·재개

### 목적

S2가 확정한 segment/checkpoint 계약을 저장 가능한 진행 상태로 만든다. 매 render 전체 재실행을
없애고, 마지막 checkpoint에서 동일 runtime을 복원한다. 2분·12명 규모를 전제로 전체 frame 복제
대신 변경된 말 중심의 델타 표현을 측정하고, logical tick grain 결정이 필요하면 이 단계에서
명시적으로 열어 둔다.

### 후보 소유 범위

- S2가 확정한 core segment/progress 모듈
- `CombatExecutionResult`/`CombatResolutionResult`의 additive serde boundary
- 저장/복원 테스트와 1200 tick 규모 fixture
- 필요 시 ScenePage producer의 캐시 입력 경계(렌더 adapter 자체는 S4)

정확한 파일은 S2 report 이후 재측정한다.

### acceptance

- pause 상태를 serialize→deserialize→resume해도 같은 결과·로그·fingerprint가 나온다.
- 기존 저장 JSON(새 필드 없음)은 additive default로 읽힌다.
- segment provenance, simulation version, manifest fingerprint, seed, 선택 이력이 저장된다.
- 델타 복원 결과가 full snapshot 결과와 바이트 수준으로 같은지 테스트한다.
- 1200 tick/12 participant 측정값과 저장 크기를 보고서에 남긴다. 임의 상한을 발명하지 않는다.
- presentation speed 변경이 저장 payload·판정 결과에 섞이지 않는다.

### 범위 밖

새 전투 효과/부활/치명타, response effect semantics, UI markup, 콘텐츠 밸런스.

### S3 정지 조건

- schema가 기존 저장을 깨뜨리거나 simulation version bump 필요 여부가 불명확하다.
- 델타가 특정 입력 순서·HashMap 순서·wall-clock에 의존한다.
- 저장 경계가 renderer 전용 데이터와 core truth를 섞는다.

## 8. S4 — 개입 되먹임·재생/consumer 연결

### 목적

S3의 저장/복원 경계와 S2의 response 선택을 실제 전투 상태에 반영한다. 재개 직전 즉발 효과를
한 번 적용하고, 명령 변경은 이후 segment에 지속시키며, 긴급 구조는 T1의 포위 파생 상태와
기존 `protect_priority`를 사용한다. ScenePage/WASM/terminal/Web은 core가 만든 marker·segment·
replay speed를 소비한다.

### 후보 소유 범위

- core response application/runtime 모듈
- `crates/escape-core/src/scene_page.rs`
- `crates/escape-terminal/src/snapshot.rs`
- `web/src/ui/storybook/combat/` 및 관련 타입/테스트
- 필요 시 WASM JSON boundary와 adapter 테스트

S4 착수 전에 S3 report에서 public schema와 adapter payload를 확정한다.

### acceptance

- 성공/실패 effect가 재개 직전에 정확히 한 번 적용된다.
- 명령 변경이 후속 segment에 지속되고 동일 선택 이력에서 동일하게 재생된다.
- 긴급 구조 선택은 결정론적 tie-break로 보호 대상을 고르고 `protect_priority`를 반영한다.
- 모든 consumer가 core marker tick에서 멈추며 자체 판정·자체 정지 시계를 갖지 않는다.
- `CombatPresentationSpeed`가 replay layer에만 존재하고 actual/forecast 결과 fingerprint에 영향을 주지 않는다.
- ScenePage/WASM/terminal/Web 회귀와 core full suite가 통과한다.

### 범위 밖

T5의 치명타·회복·부활 모델, T8의 전체 관전 cue 확장, T11 콘텐츠·밸런스 확정.

### S4 정지 조건

- response effect가 전투 상태와 지속 세계 상태 중 어느 쪽인지 정본이 불명확하다.
- renderer가 누락된 marker를 임의로 추정해야 한다.
- 기존 Web/terminal JSON contract의 additive 여부가 확인되지 않는다.

## 9. 각 coding slice 공통 위임·검수 규칙

구현 subagent 지시서에는 해당 step plan의 정확한 경로, 선행 코드, 소유/금지 파일, 시작
baseline(434/0), 정지 조건, 기대값 규칙, 검증 명령과 실제 숫자 보고, 다음 단계에 필요한
public API 서명을 넣는다.

subagent가 PASS라고 보고해도 main 오케스트레이터가 WSL에서 직접 실행한다.

```bash
cd /home/dudu/work/tui-adv
cargo fmt --all -- --check
cargo test --workspace --no-fail-fast
git diff --check
# web/ 또는 terminal/을 건드린 slice는 해당 targeted test와 npm/tsc도 직접 실행
```

테스트 수 감소, fingerprint 변화, 소유 범위 밖 변경, fixture 기대값 조용한 수정, 패닉/오류
삼킴이 발견되면 완료 처리하지 않고 구현자에게 정지 보고·수정을 요청한다.

## 10. 다음 행동

1. S1은 `abe63b3` + `e696f0b`로 완료됐다. `CombatResolutionStepper`와 batch parity가 고정됐다.
2. S2a~S2c는 각각 `e842fde`, `270f178`, `6f304ff`로 완료됐다. interleaved runtime, KO roster
   sync, opportunity pause marker가 고정됐다.
3. S2d1은 `3d941f2`, S2d2는 `f848afb`로 완료됐다. canonical segment seed와 paused response
   history/transition이 내부 runtime 계약으로 추가됐다. 각 상세 보고서는
   `fable_combat_hex_t4_step5_2608081415.md`, `fable_combat_hex_t4_step6_2608081430.md`다.
4. 다음 계획은 S2d3 또는 S3로 분리 작성한다. response effect 적용·runtime 재구성·save/serde
   schema를 한 coding slice에 묶지 않는다. S3a 내부 checkpoint/restore는
   `505d5ff`~`ead0ec4`로 완료했고, 상세 보고서는
   `fable_combat_hex_t4_step7_2608081505.md`다. S3b public SaveEnvelope 경계도
   `be77d67`/`0cd7fab`로 완료했고, 상세 보고서는
   `fable_combat_hex_t4_step8_2608081545.md`다. 다음은 S3c delta/size 측정이다.

# 전투 개입 응답 모델 정본 수정 요청서

status: proposal-for-canonical-sync
date: 2026-08-08
owner: 정본 담당자 검토 후 `Combat_Hex_Rework_Handoff.md` 및 관련 canonical 문서에 반영
related_plan: `fable_combat_hex_t4_step12_2608081745.md`

## 1. 목적

T4의 전투 중 개입을 하나의 성공/실패 모델로 처리하지 않고, 다음 두 종류로 분리한다.

1. **전략 수정형 개입**: 전투 규칙·우선순위·명령을 바꾸는 개입. 개입 자체에는 성공/실패가
   없으며, 유효한 선택이면 deterministic하게 적용한다.
2. **특수 효과·특수 이벤트형 개입**: 효과나 이벤트를 발생시키는 개입. 선택지별 확률 계산식을
   사용해 성공/실패를 판정하고, 성공·실패 각각의 결과를 상태에 적용한다.

이 문서는 기존 정본의 “개입” 설명과 코드의 `CombatResponseDefinition`/`CombatResponseOption`
계약 사이의 차이를 정리한 handoff다. 이 문서 자체는 canonical 문서를 대체하지 않으며, 정본
담당자가 검토한 뒤 해당 문서들에 반영해야 한다.

## 2. 현재 코드와 정본의 차이

현재 `CombatOpportunityCatalog`는 개입 후보와 응답 선택지를 만들 수 있다. 응답 선택지에는
다음 정보가 있다.

- response id
- executor id
- success effect id 목록
- failure effect id 목록

`CombatResponseDefinition`에는 `resolution_kind`, `executor_selector`, `target_selector`가
있지만 이 문자열의 허용 값과 계산 규칙은 정본에 고정되어 있지 않다.

현재 `CombatRuntime::resume_with_response(response_id)`는 선택 이력과 segment seed만 갱신한다.
실제 역할·정책 변경, 효과 적용, 확률 판정, 대상 확정은 아직 하지 않는다.

따라서 다음을 임의로 구현하면 안 된다.

- 모든 응답을 성공으로 처리
- 임의의 전역 성공률 또는 새 RNG 추가
- `resolution_kind` 문자열을 코드가 임의로 해석
- `target_selector` 문자열을 입력 순서나 HashMap 순서로 해석
- 실패를 아무 효과도 없는 no-op로 처리

## 3. 정본에 추가할 핵심 개념

### 3-1. 개입 종류

응답 정의는 반드시 개입 종류를 선언한다.

```text
intervention_kind: strategy_modifier | special_effect
```

실제 스키마에서는 문자열을 그대로 허용하기보다 enum 또는 정본에 등록된 id 집합으로 제한하는
것을 권장한다.

### 3-2. 전략 수정형 개입

전략 수정형은 플레이어가 선택한 순간 유효성 검사를 통과하면 성공/실패 판정을 하지 않는다.

가능한 payload 예시는 다음과 같다.

- 아군의 `protect_priority` 상승
- 특정 적에 대한 `target_priority` 상승
- 포위된 아군 주변 적을 우선 공격하도록 target policy overlay 적용
- 특정 아군에게 돌진·이동·공격 명령 부여
- 특정 적을 회피하거나 공격 대상에서 제외

적용 규칙:

- 실제 전투 RNG를 소비하지 않는다.
- 적용 시점은 response 선택을 검증한 뒤, 다음 tick을 실행하기 전이다.
- modifier는 현재 segment에만 적용되는지, 이후 segment에도 지속되는지 정의에 명시한다.
- 대상 id·role·policy가 존재하지 않으면 gameplay 실패가 아니라 입력/콘텐츠 오류로 거부한다.
- modifier 적용 이후 실제 공격의 명중·피해·충돌은 기존 전투 판정을 따른다.

예를 들어 “돌진하라”는 전략 선택 자체는 성공/실패하지 않는다. 돌진 이후 공격이 빗나가는
것은 일반 공격 판정이며, 전략 개입의 성공/실패와 다른 사건이다.

### 3-3. 특수 효과·특수 이벤트형 개입

특수 효과형은 선택지에 성공·실패 계산식을 선언한다.

```text
intervention_kind: special_effect
resolution_formula_id: <registered formula id>
formula_parameters: <choice-specific data>
success_effect_ids: [...]
failure_effect_ids: [...]
target_selector: <registered selector id>
```

처리 순서는 다음과 같다.

```text
response 선택
→ 선택지 정의·대상·비용 검증
→ 선택지별 확률 계산식 실행
→ 실제 전투 RNG namespace에서 deterministic roll
→ 성공 효과 또는 실패 효과 적용
→ outcome·적용 효과·대상·formula provenance 기록
→ 다음 segment 실행
```

실패는 no-op가 아니다. `failure_effect_ids`에 정의된 효과·로그·상태 변화를 적용해야 한다.
효과의 stacking, lifetime, combat-only 여부는 기존 `CombatEffectDefinition` 규칙을 재사용한다.

## 4. 선택지별 확률 계산식 계약

모든 특수 효과에 하나의 전역 성공률을 적용하지 않는다. 각 선택지는 등록된 formula id와
parameter를 갖는다.

정본은 각 formula에 대해 최소한 다음을 정의해야 한다.

1. 입력값: executor 능력치, target 상태, 현재 tick, 환경 상태, 비용, active effect, role/policy
   등 어떤 값을 읽는가.
2. 기본값과 보정값: 확률·난이도·상한·하한은 코드 상수가 아니라 데이터로 둔다.
3. 출력: 성공 확률, deterministic roll, 성공/실패 outcome.
4. 경계: 확률의 최소·최대 clamp, 대상이 이미 전투불능인 경우, 비용 부족, 효과 중복.
5. 버전: formula id/version이 결과 provenance와 fingerprint에 포함되는가.
6. RNG namespace: forecast와 actual combat roll을 분리하는 기존 규칙을 유지하는가.

계산식은 순수 함수여야 하며 wall-clock, 렌더 프레임 수, HashMap 순서에 의존하지 않는다.
새 RNG source를 추가하지 않는다.

정본 담당자는 formula id만 먼저 등록하고 세부 밸런스 parameter를 후속 콘텐츠 작업에서 조정할
수 있도록 허용할 수 있다. 단, formula id가 같은데 계산 규칙이 바뀌면 simulation version 또는
formula version 변경이 필요하다.

## 5. 대상 선택 계약

`target_selector`는 임의 문자열이 아니라 허용 selector와 tie-break를 가져야 한다.

예시 후보:

- `executor_self`
- `selected_target`
- `lowest_health_ally`
- `surrounded_ally`
- `nearest_enemy`
- `all_allies`

각 selector에 대해 다음을 정본에 고정한다.

- 검색할 side와 active 조건
- 전투불능 대상 포함 여부
- 거리·체력·포위 상태 계산 기준
- 여러 대상이 같은 경우의 tie-break(권장: stable id 오름차순)
- pause 시점에 대상 확정인지, resume 직전 상태에서 재계산하는지
- 플레이어가 직접 대상을 고르는 선택지의 경우 target id를 이력에 포함하는지

대상 선택 결과는 동일한 manifest·seed·선택 이력에서 항상 동일해야 한다.

## 6. UI와 core의 책임 분리

UI·WASM·terminal 같은 consumer는 플레이어가 고른 `response_id`와, 정본이 허용한 경우에만
`target_id`를 전달한다.

성공/실패 판정은 consumer가 전달하지 않는다. GameCore가 선택지의 formula와 actual RNG를
사용해 판정하고 결과를 만든다.

```text
consumer: response_id 전달
core: 개입 종류 확인
core: 전략 modifier 적용 또는 특수 효과 판정
core: 상태·로그·fingerprint·checkpoint 갱신
consumer: core marker와 결과를 재생
```

## 7. 선택 이력·checkpoint provenance

현재 선택 이력의 `response_id`만으로 결과가 결정되지 않는다면 다음 정보를 canonical history에
추가해야 한다.

- intervention kind
- response id
- formula id/version
- executor id
- resolved target id
- outcome(success/failure)
- 적용된 success/failure effect id 목록 또는 canonical effect fingerprint

다만 outcome은 consumer가 임의로 넣는 입력이 아니라 core가 계산한 결과여야 한다. target이
순수 selector로 항상 재계산 가능하면 target id를 저장하지 않고 selector/version만 저장하는
선택도 가능하다. 이 선택은 정본 담당자가 결정해야 한다.

checkpoint restore 시 intervention을 중복 적용하지 않아야 한다. pause → 선택 → 효과/전략
적용 → checkpoint → restore → resume 결과가 uninterrupted 실행과 동일해야 한다.

## 8. 정본 문서에 필요한 수정 목록

정본 담당자는 다음 문서를 갱신해야 한다.

1. `docs/design/Combat_Hex_Rework_Handoff.md`
   - T4 개입을 전략 수정형과 특수 효과형으로 분리
   - “개입에는 성공/실패가 있다”처럼 읽힐 수 있는 문장 수정
   - tick 순서와 consumer/core 책임 유지
2. `docs/design/Combat_Hex_Rework_Development_Plan.md`
   - T4(d) 개입 되먹임을 두 종류로 분리
   - 전략 modifier와 특수 효과 formula의 구현 순서 명시
   - 선택지별 formula·selector가 데이터로 조정 가능하다는 원칙 추가
3. 전투 opportunity/response schema 정본
   - `intervention_kind`
   - strategy modifier payload
   - `resolution_formula_id` 및 parameter
   - success/failure effect 적용 규칙
   - target selector registry와 tie-break
4. 전투 RNG/provenance 정본
   - 특수 효과 판정 roll의 namespace
   - formula version과 simulation version 관계
   - 선택 이력·checkpoint 필드
5. 콘텐츠 authoring 지침
   - 전략 수정형 선택지 예시
   - 특수 효과형 선택지별 성공·실패 formula 작성 예시
   - 확정되지 않은 수치를 코드에 고정하지 않는 규칙

## 9. T4 구현 순서 제안

정본 반영 뒤 구현은 다음처럼 다시 나눈다.

1. **S4a 전략 수정형 primitive**: role/policy/target-priority overlay와 지속 범위, deterministic
   적용·checkpoint parity.
2. **S4b 특수 효과 resolution**: formula registry, per-choice parameters, actual/forecast RNG
   분리, success/failure effect 적용.
3. **S4c 긴급 구조**: 포위 상태와 `protect_priority`를 사용하는 전략/효과 조합.
4. **S4d consumer 연결**: ScenePage/WASM/terminal/Web이 core response marker와 결과를 소비.

T5의 회복·부활 판정 모델과 충돌하는 효과는 T5 정본을 먼저 확인한다. T4가 새 회복·부활 수치를
임의로 만들지 않는다.

## 10. 정본 담당자 결정 요청

다음 항목을 정본에 확정해 달라.

- 전략 수정형과 특수 효과형을 별도 intervention kind로 채택할 것인가.
- 특수 효과 formula를 선택지별로 등록하고 data-driven parameter로 둘 것인가.
- formula 결과를 core가 계산하고 consumer는 response id만 전달하는 책임 분리를 채택할 것인가.
- target selector 목록과 tie-break를 어떻게 고정할 것인가.
- outcome·target·formula provenance를 selection history/checkpoint에 어떤 형태로 저장할 것인가.
- roles/policies modifier의 segment 지속 범위와 다음 개입에서의 누적·대체 규칙은 무엇인가.

이 결정이 반영되기 전에는 T4 S4 response application 코드를 구현하지 않는다. 정본이 갱신되면
이 문서를 기준으로 구현 plan을 새로 작성한다.

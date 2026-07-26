---
status: implementation-verified
phase: combat-wave1
step: 3
owner: coding_implementer
date: 2026-07-26
---

# Wave 1 Step 3 — 전투 기회·탐지·대응 후보와 개입 예산

## 목적

Notion 전투 시스템의 canonical `01`, `02`, `04`, `12`가 정한 전투 중 후보 파이프라인을 Rust GameCore의 renderer-neutral 계약으로 연다.

```text
상황 인스턴스
  → 탐지 점수/단계 계산
  → 실행 가능한 대응 후보 필터
  → 중복·만료·예산·우선순위 중재
  → 최고 가치 1개를 제시하고 개입 슬롯 1회 소비
```

이번 단계는 후보를 결정적으로 만들고 성공/실패 effect ID를 검증하는 계약만 구현한다. 실제 tick, AI, RNG, 기술 판정, effect 적용, ScenePage/WASM/UI, 콘텐츠 authoring은 다음 단계의 범위다.

## 기준 정본

- 허브: <https://app.notion.com/p/36f37e69695e81168360ef11cf3a4449>
- `01. 전투 루프와 개입 예산`: 인카운터 중요도/유형이 정한 0~3 상한, 제시 순간 소비, 자동 전투는 비개입 처리
- `02. 자동 전투와 상황 트리거`: 상황 태그 기반 후보, 유효 후보 필터, 결정적 중재, 예산 소진 뒤 비정지 알림만 허용
- `04. 선택지 생성 규칙`: 탐지와 실행 판정 분리, 실패 effect 필수, effect catalog ID만 발행
- `12. 기술 기반 선택지·전투 기회 시스템`: 감지→판독→간파 단계, 대표 관찰자 1명, 최대 4개(항상 개입하지 않음 포함), 고정 tie-break

## 소유 파일

coding agent가 수정할 파일:

- `crates/escape-core/src/combat_opportunity.rs` (신규)
- `crates/escape-core/src/lib.rs` (모듈 및 public export만)
- `crates/escape-core/tests/combat_opportunity_wave1.rs` (신규)

main이 별도로 처리할 housekeeping:

- `crates/escape-terminal/tests/cli_smoke.rs`: 호출자가 없는 `raw_glyphfx_wave` helper 제거
- 이 plan/index/운영 문서의 상태 갱신과 최종 검증

수정 금지:

- `combat_contract.rs`, `combat_state.rs`의 기존 public 필드/serde 계약
- `ScenePage`, WASM, Web/terminal renderer, YAML/bundle/generated artifact
- fixed tick, AI, actual/forecast RNG, skill/ability schema, balance 숫자

## 구현 계약

### 1. 탐지 단계

`CombatDetectionLevel`을 `Undetected < Detected < Interpreted < Insightful` 순서로 제공한다.

`CombatDetectionThresholds { detected, interpreted, insightful }`를 두고 `detected <= interpreted <= insightful`을 검증한다. 탐지 점수는 이번 단계에서 계산하지 않는다. 호출자가 능력·숙련·기술/특성·시야·거리·압박 보정을 합산한 결정론적 정수 점수를 전달하며, `level_for_score(score)`는 경계만 비교한다. 음수/특정 직업/기술별 밸런스 상수는 추가하지 않는다.

### 2. 기회와 인스턴스

`CombatOpportunityDefinition`은 최소한 아래를 가진다.

- stable `id`
- trigger tag 목록과 required condition ID 목록
- 탐지 임계값
- optional expiry tick 및 recurrence/dedupe 정책
- scripted 여부, 패배 위험, 전황 영향, 고유 대응 여부, 일반 전술 우선도
- free/non-pausing information alert ID (없을 수 있음)

`CombatOpportunityInstance { id, definition_id }`로 동일 정의의 서로 다른 발생을 구분한다. 이미 제시된 instance ID와 만료된 instance는 후보에서 제거한다. 입력에는 현재 tick, active tag/condition ID, 대표 관찰자 후보, 현재 `CombatInterventionBudget`, manifest fingerprint를 둔다.

### 3. 관찰자와 대응 후보

관찰자 입력은 stable ID, 사전 계산된 detection score, capability/skill/trait ID, 실행 가능 여부를 가진다. 팀 점수를 합산하지 않고 관찰자별로 계산하며, 최고 탐지 단계 → 최고 점수 → stable ID 순으로 대표 관찰자를 정한다.

`CombatResponseDefinition`은 opportunity ID, 최소 탐지 단계, 요구 capability/condition ID, 실행자·대상 selector, 비용 tag, resolution kind, success effect IDs, failure effect IDs, 고유 대응 여부, priority를 가진다. 성공과 실패 effect bundle은 모두 비어 있으면 안 되며, effect ID는 기존 `CombatEffectCatalog`에 존재해야 한다.

`CombatOpportunityCandidate`는 instance/definition ID, 대표 관찰자, 탐지 단계, 실행 가능한 response option을 담는다. option은 최대 4개이며, 항상 `no_intervention`을 포함한다. no-op만 남으면 유효 후보가 아니므로 제시하지 않는다.

### 4. 예산·중재

`CombatInterventionBudget`은 `maximum`과 `consumed`를 가지고 maximum/remaining을 검증한다. maximum은 0~3만 허용한다. `present()`가 성공한 순간 consumed를 1 증가시키며, 선택/관찰/무시와 무관하게 후보 제시 자체가 슬롯을 소비한다.

후보 중재 순서는 다음 고정 비교를 사용한다.

1. scripted/각본 필수 우선
2. defeat/loss risk
3. battlefield impact
4. unique capability response
5. general tactical priority
6. expiry tick이 빠른 후보
7. instance ID

동일 인스턴스는 한 번만 제시한다. 예산이 0이면 선택 후보를 만들지 않으며 free information alert만 반환한다. 예산 소진 뒤 AI에 숨은 무료 실행을 만들지 않는다. 전체 결과는 입력 정렬과 무관하게 canonicalize한다.

### 5. 결과 경계

이번 단계의 output은 `CombatOpportunityEvaluation`/`CombatInterventionSelection` 같은 pure data다. 성공·실패 effect를 실제 `CombatState`에 적용하거나 RNG를 호출하지 않는다. 선택 결과는 다음 단계가 사용할 stable response ID와 effect ID 목록으로만 남긴다.

## 하드 검증 규칙

- 빈/중복 stable ID, 잘못된 threshold 순서, maximum/consumed 범위 오류는 hard error
- 존재하지 않는 opportunity/effect/capability 참조는 hard error
- success 또는 failure effect bundle 누락은 hard error
- 감지 단계가 낮은 대응, 조건/능력이 없는 대응, 실행 불가 관찰자는 후보에서 제외
- 동일 response ID, no-op 중복, 4개 초과 option은 hard error 또는 deterministic filter로 명확히 처리
- 예산 우회, 동일 instance 재제시, seed 밖 RNG, renderer 판정은 구현하지 않음

## acceptance criteria

1. catalog가 opportunity/response를 validate하고 canonical JSON을 결정적으로 만든다.
2. 같은 context/score/seed-free input은 같은 대표 관찰자·탐지 단계·response ordering·selected candidate를 만든다.
3. `detected <= interpreted <= insightful` 단계와 미탐지 숨김이 테스트된다.
4. 0, 1, 3 예산과 네 번째 제시 거부, 제시 순간 소비가 테스트된다.
5. 후보 중재 tie-break와 instance dedupe/expiry가 테스트된다.
6. 항상 `no_intervention`이 포함되고 action option은 4개를 넘지 않으며 no-op만 있는 기회는 제시되지 않는다.
7. 성공/실패 effect 참조와 누락/unknown effect 오류가 테스트된다.
8. 순서가 다른 입력의 evaluation fingerprint/canonical JSON이 동일하다.
9. `cargo fmt --all -- --check`, targeted test, `cargo test --workspace --no-fail-fast`, `git diff --check`가 main에서 통과한다.

## 검증 명령

```bash
cargo fmt --all -- --check
cargo test -p escape-core --test combat_opportunity_wave1
cargo test --workspace --no-fail-fast
git diff --check
```

## 보고 형식

- 변경 파일과 public API
- opportunity → detection → response → budget 흐름
- 실행하지 않은 범위(non-goal)
- subagent가 실행한 명령과 main이 재실행할 명령
- 남은 Wave 2 의존성

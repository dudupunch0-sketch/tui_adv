# Fable Combat Wave 2 Step 2 — 실행 모드 parity와 이중 로그

status: implementation-verified
기준일: 2026-07-26

## 목적

Wave 2 Step 1의 고정 정수 좌표·role/target·snapshot tick 계약을 실행 모드 계약으로 감싼다. 일반 관전, 재도전, 자동 전투, 고속 관전은 같은 실제 전투 seed와 같은 simulation을 사용하고, forecast는 실제 전투 RNG를 재사용하지 않는 별도 namespace seed를 사용한다. 모든 모드는 renderer가 재판정하지 않고 core가 만든 frame과 로그를 소비한다.

이번 문서는 한 번의 coding subagent 작업으로 끝낼 수 있는 계약 slice만 다룬다. 아직 공격·충돌·피해·기술/상태 effect resolver가 없으므로, 실행 결과는 현재의 이동/목표 frame을 기준으로 한다.

## 원본 정본

- [01. 전투 루프와 개입 예산](https://app.notion.com/p/36f37e69695e812c92efd2c11edabb66)
- [03. 핵심 상태 시스템](https://app.notion.com/p/36f37e69695e81a9a36fcbe1df5b527f)
- [07. UI·템포·리스크](https://app.notion.com/p/36f37e69695e81258c60fc669f4d6800)
- [09. 다수전 전투 시스템](https://app.notion.com/p/3a737e69695e81fc9ab1fe94e2dd98d7)
- [11. 능력치·숙련·전투 스킬 시스템](https://app.notion.com/p/3a837e69695e818eafbccfa309d08149)
- [12. 기술 기반 선택지·전투 기회 시스템](https://app.notion.com/p/3a837e69695e81d090b1f0503af26ebe)
- [13. 감독형 관전·전략 피드백 시스템](https://app.notion.com/p/3a937e69695e81daa01df6f79823c4d6)

## 적용 계약

1. 같은 고정층·변경층·seed·선택 이력·simulation version은 같은 결과와 로그를 만든다.
2. actual combat와 forecast ensemble RNG namespace는 분리한다. forecast는 manifest의 ForecastEnsemble namespace에서 파생한 seed를 사용하고 actual seed를 재사용하지 않는다.
3. retry는 직전 준비를 유지한 실제 전투 재실행이다. auto는 선택 가능한 개입을 개입하지 않음으로 처리하고, fast는 2배속 표현일 뿐 simulation 규칙을 바꾸지 않는다.
4. 1배속·2배속·즉시 결과는 처리/표현 속도만 다르고 frame 순서, 판정 입력, 전체 로그가 같아야 한다.
5. 전체 로그는 모든 현재 frame 사건을 stable tick/sequence 순서로 보존한다. 핵심 로그는 등록된 사건 태그와 중요도 규칙으로 전체 로그에서 파생하며 자유 문장이나 renderer-local 판정을 만들지 않는다.
6. 숨은 적 능력이나 미관측 판정을 새로 노출하지 않는다. 이번 primitive slice에서는 이미 manifest/participant에 공개된 이동·목표 정보만 기록한다.

## 소유 파일

coding agent는 다음 파일만 추가·수정한다.

- crates/escape-core/src/combat_execution.rs (신규 실행 mode, speed, dual log, result contract)
- crates/escape-core/src/lib.rs (public module/export)
- crates/escape-core/tests/combat_execution_wave2.rs (contract tests)

plan/index/운영 문서의 상태 갱신은 main orchestrator가 검증 후 수행한다. .claude/worktrees/와 기존 Wave 1~2 파일은 건드리지 않는다.

## Acceptance criteria

- [x] Actual, Retry, Auto, Fast가 같은 입력에서 동일한 tick frame, full log, core log, run fingerprint를 만든다. mode/presentation metadata만 다를 수 있다.
- [x] Forecast는 manifest 기반 forecast namespace seed를 사용하고 actual seed와 다르며, 같은 forecast 입력을 반복하면 동일 결과·로그·fingerprint를 만든다.
- [x] OneX, TwoX, Instant presentation 선택은 simulation tick 수·frame 순서·로그 내용을 바꾸지 않는다.
- [x] full log가 각 이동/목표 frame을 누락 없이 stable order로 담고, core log가 중요도 필터로 deterministic하게 파생된다. 로그 사건은 enum/tag 기반이어야 한다.
- [x] 결과 fingerprint가 simulation version, mode의 seed namespace, canonical setup, frame/full log를 바인딩한다. 입력 순서가 달라도 canonical 결과는 동일해야 한다.
- [x] zero tick, max tick 초과, 잘못된 mode 입력은 기존 CombatSimulationError 계열과 일관된 오류로 거부한다.
- [x] targeted 테스트는 mode parity, forecast namespace 분리, presentation parity, dual-log 순서/필터, fingerprint determinism을 고정한다.

## Non-goal

- 실제 공격·충돌·생명력/균형/호흡 피해, 기술 비용·쿨다운, 상태 effect resolver
- 승률 100회 ensemble, 전투 종료 보고서의 캐릭터별 피해·치유 수치
- ScenePage/WASM/Web Storybook/SuperLightTUI renderer adapter
- 전략 조언, 자동 원인 분석, MVP 평가, 신규 밸런스 상수
- 개입 후보 생성/예산 소비 로직의 재구현

## 검증 명령

- cargo fmt --all -- --check
- cargo test -p escape-core --test combat_execution_wave2 --test combat_simulation_wave2 --test combat_contract_wave1 --test combat_state_wave1 --test combat_opportunity_wave1
- cargo test --workspace --no-fail-fast
- git diff --check

## 구현 보고 형식

- 변경 파일: combat_execution.rs, lib.rs, combat_execution_wave2.rs
- public API: CombatRunMode, CombatPresentationSpeed, CombatExecutionRequest/Result, dual-log event/tag/importance
- mode/seed: actual/retry/auto/fast는 ActualCombat, forecast는 ForecastEnsemble derived seed
- 로그: TargetSelection과 MoveIntent를 full log에 stable tick/sequence로 기록하고 core log는 importance 필터로 파생
- 검증: targeted 5개 suite와 workspace 전체, fmt, diff check 통과
- non-goal: 실제 공격·피해·effect·renderer·승률 ensemble·밸런스 상수
- 다음: 실제 공격/충돌/피해 resolver를 별도 Wave 2 slice로 설계

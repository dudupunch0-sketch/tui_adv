# 전투 시스템 구현 계획 인덱스

status: wave2-step5-complete
기준일: 2026-08-02

이 문서는 Notion `전투 시스템` 허브와 canonical 문서 00~13을 Rust GameCore 구현 순서로 쪼갠 인덱스다. 각 단계 문서는 한 번의 coding subagent 작업으로 완료할 수 있는 크기를 목표로 한다.

## 원본과 우선순위

- 허브: [전투 시스템](https://app.notion.com/p/36f37e69695e81168360ef11cf3a4449)
- 하위 정본: 00~13 문서. 허브와 충돌하면 하위 정본을 우선한다.
- 저장소 truth: `crates/escape-core` → `ScenePage`/WASM JSON → Web Storybook·SuperLightTUI.
- 기존 `docs/design/Combat_System_Auto_Brawl.md`는 이전 schema-less 방향의 설계 기록이다. 새 구현 계약은 Notion 정본을 우선하되, 기존 renderer-neutral 원칙과 non-goal은 유지한다.

## 현재 코드와 정본의 경계

Wave 1 Step 1~3과 Wave 2 Step 1~4가 `escape-core`에 구현·검증되어 initial manifest/RNG 분리, 전투원 상태/effect catalog, opportunity 후보, 고정 정수 좌표·role/target·동시 tick frame, 실행 mode parity·dual log, 실제 collision/attack/damage/effect resolution sidecar, 다수전 결착/종료 조건 sidecar 계약을 제공한다. Wave 3 Step 1a가 여기에 `escape-core` 전용 관전 view 어댑터(`combat_spectator.rs`)를 더해 tick별 체스말 프레임, 공용 연출 cue(Attack/Hit/Evade), 템플릿 id 기반 이중 로그, 누설 차단(숨은 판정·억제 사유·Hidden/Conditional 효과 id 마스킹)을 제공한다 (`crates/escape-core/tests/combat_spectator_wave3.rs`, 현재 19개 테스트: `spectate_is_deterministic_for_identical_input`, `attack_roll_and_effect_suppressed_never_leak_into_any_log`, `hidden_conditional_and_unregistered_effect_ids_are_masked` 등). Wave 2 Step 5가 여기에 `CombatResolutionFrame.combatants`(tick 종료 시점 전투원 스냅샷, additive-optional)를 더하고 이를 소비해 `BalanceBroken`(균형 붕괴)·`Incapacitated`(전투불능) cue 2개를 파생한다 (`crates/escape-core/tests/combat_resolution_wave2.rs`의 `frame_snapshot_is_id_sorted_and_covers_every_combatant`·`last_frame_snapshot_matches_final_state_combatants`, `crates/escape-core/tests/combat_spectator_wave3.rs`의 `cue_ordering_is_fixed_attack_hit_evade_balance_broken_incapacitated` 등). 이로써 정본 13의 공용 연출 문법 5개(공격/피격/회피/균형 붕괴/전투불능)가 모두 확보됐다. 다음 계약은 아직 없다.

- 고급 다수전 AI 행동·조기 결착/전투 tick 중단 resolver
- 대형·결속·배경 전투·증원과 전투 종료 조건
- 전투 종료 narrative/report consumer (전투 시간·캐릭터별 피해/치유/처치 수 확장) → Wave 3 Step 1b
- `ScenePage` 필드 추가·WASM 노출 → Wave 3 Step 1c
  - **선결 과제(2026-08-02 실측)**: `CombatResolutionResult.fingerprint`는 `frames`를 `serde_json`으로 직렬화해 해싱하므로 frame에 필드를 추가하면 값이 바뀐다 (Wave 2 Step 5에서 실제로 바뀌었다). `CombatConclusionReport`·`CombatSpectatorView` fingerprint도 이를 체이닝한다. 아직 save·JSON boundary에 노출된 적이 없어 호환 문제는 없지만, Step 1c에서 밖으로 내보내기 전에 fingerprint 안정성 계약(schema 추가 시 값을 고정할지 여부)을 확정해야 한다.
- terminal/Web 렌더러, 상단/하단 레이아웃, 색·아이콘 동기화 → Wave 3 Step 1d
- 프리셋 저장/재도전 유지, 우선 목표 규칙
- 치유(healing)·명줄(life thread) 파이프라인
- 시스템형/혼합형/각본형 authoring 구분 → Wave 3 Step 2

## 단계 순서

| 단계 문서 | 한 번의 구현 단위 | 핵심 non-goal |
| --- | --- | --- |
| `fable_combat_wave1_step1_2607261845.md` | 결정론 계약 primitive와 manifest fingerprint | 실제 전투 진행·UI·밸런스 |
| `fable_combat_wave1_step2_2607261845.md` | 전투원 상태·effect catalog·전투 전 투영 | tick resolver·콘텐츠 확장 |
| `fable_combat_wave1_step3_2607261845.md` | opportunity/response 후보와 0~3 개입 예산 | renderer·실시간 시뮬레이션 |
| `fable_combat_wave2_step1_2607261845.md` | 고정 tick·AI 역할·목표·연속 위치 resolver | Web 연출·밸런스 확정값 |
| `fable_combat_wave2_step2_2607261845.md` | actual/forecast/retry/auto/fast 결과 parity와 이중 로그 | 전략 조언·자동 원인 분석 |
| `fable_combat_wave2_step3_2607261845.md` | 실제 collision/attack/damage/effect resolver와 fixed-point sidecar 상태 | renderer adapter·결착·밸런스 확정값 |
| `fable_combat_wave2_step4_2607261845.md` | 다수전 결착·전투 종료 조건 sidecar와 cleanup report | 고급 AI·증원·패주·renderer adapter |
| `fable_combat_wave3_step1a_2608020020.md` | 관전 view 어댑터 (core 전용): tick별 프레임·공용 cue·이중 로그·누설 차단 | ScenePage/WASM/renderer 노출, 밸런스 확정값, BalanceBroken/Incapacitated cue |
| `fable_combat_wave2_step5_2608020117.md` | resolution frame per-tick 전투원 스냅샷과 균형 붕괴·전투불능 cue | renderer 노출·보고서 확장·밸런스 확정값 |
| (플랜 미작성) — Wave 3 Step 1b | 전투 종료 보고서 확장 (전투 시간, 캐릭터별 피해/치유/처치 수) | renderer 노출 |
| (플랜 미작성) — Wave 3 Step 1c | `ScenePage` 필드 추가, WASM 노출 | terminal/Web 렌더러 |
| (플랜 미작성) — Wave 3 Step 1d | terminal/Web 렌더러, 상단/하단 레이아웃, 색·아이콘 동기화 | seed·판정·AI·로그 순서 재구현 |
| (플랜 미작성) — Wave 3 Step 2 | 시스템형 1개 + 혼합형 1개 + 각본형 1개 authoring slice | 대규모 콘텐츠·보스 밸런스 |

Wave 2 Step 4 구현 위치: `crates/escape-core/src/combat_conclusion.rs`, `crates/escape-core/tests/combat_conclusion_wave2.rs`.
Wave 3 Step 1a 구현 위치: `crates/escape-core/src/combat_spectator.rs`, `crates/escape-core/tests/combat_spectator_wave3.rs` (12 테스트).
Wave 2 Step 5 구현 위치: `crates/escape-core/src/combat_resolution.rs`, `crates/escape-core/src/combat_spectator.rs`, `crates/escape-core/tests/combat_resolution_wave2.rs` (16 테스트), `crates/escape-core/tests/combat_spectator_wave3.rs` (19 테스트).

각 단계는 선행 단계의 public contract와 테스트만 사용한다. 단계 사이에 새 필드가 필요하면 먼저 해당 단계 plan을 갱신하고, 기존 저장/JSON backward compatibility를 검토한다.

## 구현 운영 규칙

1. 오케스트레이터가 해당 단계 plan을 먼저 확정한다.
2. `coding_implementer`는 plan의 소유 파일만 수정한다. 다른 작업자의 변경이나 미추적 `.claude/worktrees/`를 건드리지 않는다.
3. subagent 보고와 별개로 오케스트레이터가 WSL에서 핵심 테스트를 재실행한다.
4. 정확한 기술 비용·회복률·상태 계수·직업별 수치는 데이터 조정 항목으로 남기고 코드 상수로 임의 확정하지 않는다.
5. 렌더러는 Rust core가 만든 결과를 표시만 한다. seed, 판정, AI, 로그 순서를 Web/terminal에서 재구현하지 않는다.

# 전투 시스템 구현 계획 인덱스

status: wave2-step6-complete
기준일: 2026-08-02

이 문서는 Notion `전투 시스템` 허브와 canonical 문서 00~13을 Rust GameCore 구현 순서로 쪼갠 인덱스다. 각 단계 문서는 한 번의 coding subagent 작업으로 완료할 수 있는 크기를 목표로 한다.

## 원본과 우선순위

- 허브: [전투 시스템](https://app.notion.com/p/36f37e69695e81168360ef11cf3a4449)
- 하위 정본: 00~13 문서. 허브와 충돌하면 하위 정본을 우선한다.
- 저장소 truth: `crates/escape-core` → `ScenePage`/WASM JSON → Web Storybook·SuperLightTUI.
- 기존 `docs/design/Combat_System_Auto_Brawl.md`는 이전 schema-less 방향의 설계 기록이다. 새 구현 계약은 Notion 정본을 우선하되, 기존 renderer-neutral 원칙과 non-goal은 유지한다.

## 현재 코드와 정본의 경계

Wave 1 Step 1~3과 Wave 2 Step 1~4가 `escape-core`에 구현·검증되어 initial manifest/RNG 분리, 전투원 상태/effect catalog, opportunity 후보, 고정 정수 좌표·role/target·동시 tick frame, 실행 mode parity·dual log, 실제 collision/attack/damage/effect resolution sidecar, 다수전 결착/종료 조건 sidecar 계약을 제공한다. Wave 3 Step 1a가 여기에 `escape-core` 전용 관전 view 어댑터(`combat_spectator.rs`)를 더해 tick별 체스말 프레임, 공용 연출 cue(Attack/Hit/Evade), 템플릿 id 기반 이중 로그, 누설 차단(숨은 판정·억제 사유·Hidden/Conditional 효과 id 마스킹)을 제공한다 (`crates/escape-core/tests/combat_spectator_wave3.rs`, 현재 19개 테스트: `spectate_is_deterministic_for_identical_input`, `attack_roll_and_effect_suppressed_never_leak_into_any_log`, `hidden_conditional_and_unregistered_effect_ids_are_masked` 등). Wave 2 Step 5가 여기에 `CombatResolutionFrame.combatants`(tick 종료 시점 전투원 스냅샷, additive-optional)를 더하고 이를 소비해 `BalanceBroken`(균형 붕괴)·`Incapacitated`(전투불능) cue 2개를 파생한다 (`crates/escape-core/tests/combat_resolution_wave2.rs`의 `frame_snapshot_is_id_sorted_and_covers_every_combatant`·`last_frame_snapshot_matches_final_state_combatants`, `crates/escape-core/tests/combat_spectator_wave3.rs`의 `cue_ordering_is_fixed_attack_hit_evade_balance_broken_incapacitated` 등). 이로써 정본 13의 공용 연출 문법 5개(공격/피격/회피/균형 붕괴/전투불능)가 모두 확보됐다. Wave 3 Step 1b가 여기에 `combat_conclusion.rs`의 `CombatConclusionReport`를 확장해 `tick_millis` 입력 기반 `duration_millis`(전투 시간), 캐릭터별 `damage_dealt_hundredths`·`damage_taken_hundredths`·`kills`·`incapacitated` 집계(`combatants`, id 오름차순), 최대 피해 가한/받은 캐릭터 하이라이트(`top_damage_dealt_id`/`top_damage_taken_id`, 발생하지 않으면 `None`)를 더한다. 판정은 `request.resolution.frames[].outcomes`·`combatants` 스냅샷만 집계하며 재계산하지 않는다 (`crates/escape-core/tests/combat_conclusion_wave2.rs`, 현재 14개 테스트: `combatants_report_sums_damage_and_marks_incapacitated`, `kills_are_attributed_to_last_valid_lethal_outcome_in_the_ko_tick`, `top_damage_highlights_pick_max_with_lowest_id_tie_break`, `same_input_conclude_twice_yields_identical_report_and_fingerprint` 등). Wave 2 Step 6이 여기에 `combat_execution.rs`의 `CombatExecutionResult.provenance`(`CombatProvenance`: `simulation_version`·`tick_millis`·`manifest_fingerprint`, additive-optional)를 더해 `execute()`가 이미 손에 쥔 입력 맥락을 결과에 심고, `spectate()`/`conclude()`가 각자 받던 중복 `tick_millis` 파라미터를 제거해 그 provenance에서 읽게 한다 (`crates/escape-core/tests/combat_execution_wave2.rs`, 현재 10개 테스트: `provenance_matches_input_manifest_version_tick_millis_and_fingerprint`, `forecast_mode_reports_the_same_provenance_as_input`, `deserializing_result_json_without_provenance_field_yields_none`, `same_input_executed_twice_yields_identical_provenance` 등; `combat_spectator_wave3.rs`의 `view_reports_the_tick_millis_from_provenance`·`missing_provenance_is_rejected`; `combat_conclusion_wave2.rs`의 `missing_provenance_is_rejected`). provenance가 없거나 `tick_millis == 0`이면 값을 지어내지 않고 `MissingProvenance` 에러를 낸다. 다음 계약은 아직 없다.

- 고급 다수전 AI 행동·조기 결착/전투 tick 중단 resolver
- 대형·결속·배경 전투·증원과 전투 종료 조건
- `ScenePage` 필드 추가·WASM 노출 → Wave 3 Step 1c
  - **선결 과제(fingerprint 안정성) — 해소 (2026-08-02, Wave 2 Step 6)**: `CombatResolutionResult.fingerprint`는 `frames`를 `serde_json`으로 직렬화해 해싱하므로 frame에 필드를 추가하면 값이 바뀐다 (Wave 2 Step 5에서 실제로 바뀌었다). `CombatConclusionReport`·`CombatSpectatorView` fingerprint도 이를 체이닝한다. 정본 03 「핵심 상태 시스템」의 RNG·재시도·버전 절은 "같은 고정층 + 변경층 + seed + 개입 ID/tick + simulation version은 같은 결과를 낸다. 이 결정성은 같은 version 내부에서만 보장한다 … 활성 전투와 즉시 재시도는 같은 simulation version을 사용하고, 전투 기록에는 version을 저장한다. 이후 갱신된 version이 과거 seed 결과를 재현할 필요는 없다."라고 명시한다. 즉 결정성은 같은 `simulation_version` 안에서만 보장되므로, schema 추가로 fingerprint 값이 바뀌는 것은 계약 위반이 아니다 — fingerprint 자체를 schema 변경에 대해 고정할 필요가 없다. 대신 전투 기록이 `simulation_version`을 저장해야 하며, Wave 2 Step 6이 `CombatExecutionResult.provenance.simulation_version`으로 이를 구현했다. **계약: fingerprint를 비교하는 consumer는 반드시 `simulation_version`도 함께 비교해야 한다.** simulation_version이 다르면 fingerprint 불일치는 예상된 결과이지 오류가 아니다; simulation_version이 같은데 fingerprint가 다르면 그것이 실제 회귀다. Step 1c 이후 renderer/save가 fingerprint를 노출·비교할 때 이 계약을 지켜야 한다.
- terminal/Web 렌더러, 상단/하단 레이아웃, 색·아이콘 동기화 → Wave 3 Step 1d
- 프리셋 저장/재도전 유지, 우선 목표 규칙
- 치유량·최대 치유량 캐릭터 — combat 파이프라인에 회복 개념이 없어 보류 (healing slice 선행 필요; `combat_resolution.rs`의 체력 갱신은 감소 전용이고 `CombatAttackDefinition`/`CombatEffectDefinition`에 회복 필드가 없다)
- 명줄 소모·패배 결과 — 정본 10 기준 런 단위 메타 자원이며 인카운터 패배 결과 정의가 소유한다. 패배 결과 스키마 slice 선행 필요
- 시스템형/혼합형/각본형 authoring 구분 → Wave 3 Step 2

Wave 3 Step 1b는 정본 13이 금지하는 전략 수행 평가·핵심 전환점·자동 원인 분석·전략 조언·종합 MVP·이전 전투 결과 자동 비교를 의도적으로 구현하지 않았다.

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
| `fable_combat_wave3_step1b_2608020437.md` | 전투 종료 보고서 확장 (전투 시간, 캐릭터별 입힌/받은 피해·처치 수·전투불능, 최대 피해 가한/받은 하이라이트) | 치유량·명줄, renderer 노출 |
| `fable_combat_wave2_step6_2608020838.md` | 전투 기록 provenance(simulation version·tick 길이·manifest fingerprint)와 중복 tick_millis 파라미터 제거 | renderer 노출·밸런스 확정값 |
| (플랜 미작성) — Wave 3 Step 1c | `ScenePage` 필드 추가, WASM 노출 | terminal/Web 렌더러 |
| (플랜 미작성) — Wave 3 Step 1d | terminal/Web 렌더러, 상단/하단 레이아웃, 색·아이콘 동기화 | seed·판정·AI·로그 순서 재구현 |
| (플랜 미작성) — Wave 3 Step 2 | 시스템형 1개 + 혼합형 1개 + 각본형 1개 authoring slice | 대규모 콘텐츠·보스 밸런스 |

Wave 2 Step 4 구현 위치: `crates/escape-core/src/combat_conclusion.rs`, `crates/escape-core/tests/combat_conclusion_wave2.rs`.
Wave 3 Step 1a 구현 위치: `crates/escape-core/src/combat_spectator.rs`, `crates/escape-core/tests/combat_spectator_wave3.rs` (Step 1a 완료 당시 12 테스트, Wave 2 Step 5에서 19로 늘었다 — 현재 수치는 아래 Step 5 줄을 본다).
Wave 2 Step 5 구현 위치: `crates/escape-core/src/combat_resolution.rs`, `crates/escape-core/src/combat_spectator.rs`, `crates/escape-core/tests/combat_resolution_wave2.rs` (16 테스트), `crates/escape-core/tests/combat_spectator_wave3.rs` (19 테스트).
Wave 3 Step 1b 구현 위치: `crates/escape-core/src/combat_conclusion.rs`, `crates/escape-core/tests/combat_conclusion_wave2.rs` (14 테스트).
Wave 2 Step 6 구현 위치: `crates/escape-core/src/combat_execution.rs`, `crates/escape-core/src/combat_spectator.rs`, `crates/escape-core/src/combat_conclusion.rs`, `crates/escape-core/tests/combat_execution_wave2.rs` (10 테스트), `crates/escape-core/tests/combat_spectator_wave3.rs` (19 테스트, 개수 유지), `crates/escape-core/tests/combat_conclusion_wave2.rs` (14 테스트, 개수 유지). `CombatSpectatorRequest.tick_millis`/`CombatConclusionRequest.tick_millis` 필드와 두 `InvalidTickMillis` 에러 변형을 제거하고 `MissingProvenance`로 대체했다.

각 단계는 선행 단계의 public contract와 테스트만 사용한다. 단계 사이에 새 필드가 필요하면 먼저 해당 단계 plan을 갱신하고, 기존 저장/JSON backward compatibility를 검토한다.

## 구현 운영 규칙

1. 오케스트레이터가 해당 단계 plan을 먼저 확정한다.
2. `coding_implementer`는 plan의 소유 파일만 수정한다. 다른 작업자의 변경이나 미추적 `.claude/worktrees/`를 건드리지 않는다.
3. subagent 보고와 별개로 오케스트레이터가 WSL에서 핵심 테스트를 재실행한다.
4. 정확한 기술 비용·회복률·상태 계수·직업별 수치는 데이터 조정 항목으로 남기고 코드 상수로 임의 확정하지 않는다.
5. 렌더러는 Rust core가 만든 결과를 표시만 한다. seed, 판정, AI, 로그 순서를 Web/terminal에서 재구현하지 않는다.

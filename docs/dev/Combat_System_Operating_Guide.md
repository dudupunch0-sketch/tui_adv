# 전투 시스템 운영 기준

상태: active
기준일: 2026-08-02
목적: context window가 압축되거나 새 세션으로 이어져도 전투 시스템 구현을 같은 방식으로 재개하기 위한 짧은 운영 정본.

## 1. 역할과 책임

- 오케스트레이터(main)는 Notion 정본 조사, 범위 판단, plan 파일 작성, 충돌 해결, 최종 검증과 사용자 보고를 담당한다.
- `coding_implementer`는 `tui_adv_coder` 역할의 5.6 luna / medium 구현 agent다. 승인된 한 slice의 코드·콘텐츠·테스트만 수정한다.
- subagent의 PASS/FAIL 보고는 참고일 뿐이다. cargo, pytest, npm, generated artifact 검증은 main이 WSL에서 다시 실행한다.
- 동일 저장소를 여러 작업자가 공유하므로 다른 작업자의 변경과 미추적 파일을 되돌리거나 정리하지 않는다.

## 2. 기준 문서와 우선순위

충돌 시 다음 순서를 따른다.

1. 사용자가 현재 세션에서 명시한 결정
2. Notion [전투 시스템 허브](https://app.notion.com/p/36f37e69695e81168360ef11cf3a4449)와 하위 canonical 00~13
3. `docs/dev/Development_Plan.md`
4. [Combat_System_Implementation_Plan_Index.md](../design/Combat_System_Implementation_Plan_Index.md)와 해당 step plan
5. 기존 설계 기록과 테스트

하위 canonical 문서가 허브 요약과 다르면 하위 문서를 따른다. Notion 원문을 repo에 복사해 source of truth로 만들지 말고, plan에는 URL과 적용한 규칙만 기록한다.

## 3. 현재 구현 기준점

완료:

- `fable_combat_wave1_step1_2607261845.md`
- `fable_combat_wave1_step2_2607261845.md`
- `fable_combat_wave1_step3_2607261845.md`
- `crates/escape-core/src/combat_contract.rs`
- `crates/escape-core/src/combat_opportunity.rs`
- `crates/escape-core/src/combat_state.rs`
- `CombatSimulationVersion`, 5개 RNG namespace, `CombatManifest`, effect reason/suppressed effect, canonical JSON/fingerprint/derived seed, validation
- `crates/escape-core/tests/combat_contract_wave1.rs` 5개 테스트
- `crates/escape-core/tests/combat_state_wave1.rs` 8개 테스트
- `crates/escape-core/tests/combat_opportunity_wave1.rs` 12개 테스트
- Step 2는 전투원 지속 수치·자세·무기 제어·팀/관계/환경 상태, effect catalog 검증·canonicalization, deterministic pre-combat projection과 persistent/combat-only 결착 계약까지 구현·검증했다.
- Step 3은 opportunity → detection → response 후보, 대표 관찰자/실행자 분리, 0~3 개입 예산, dedupe/expiry/free alert/no-op, effect catalog 참조 검증과 deterministic evaluation fingerprint까지 구현·검증했다.
- `fable_combat_wave2_step1_2607261845.md`
- `crates/escape-core/src/combat_simulation.rs`
- `crates/escape-core/tests/combat_simulation_wave2.rs` 10개 테스트
- Wave 2 Step 1은 고정 정수 좌표·범위 helper, active 4/8 상한, role weights·target fallback, snapshot 동시 tick, canonical simulation fingerprint까지 구현·검증했다.
- `fable_combat_wave2_step2_2607261845.md`
- `crates/escape-core/src/combat_execution.rs`
- `crates/escape-core/tests/combat_execution_wave2.rs` 6개 테스트
- Wave 2 Step 2는 Actual/Retry/Auto/Fast parity, ForecastEnsemble seed 분리, OneX/TwoX/Instant presentation parity, stable dual log와 core importance filter를 구현·검증했다.
- `fable_combat_wave2_step3_2607261845.md`
- `crates/escape-core/src/combat_resolution.rs`
- `crates/escape-core/tests/combat_resolution_wave2.rs` 11개 테스트
- Wave 2 Step 3은 execution frame sidecar resolver, collision/range/accuracy, fixed-point damage/defense, health/balance clamp, effect catalog stacking, Actual/Forecast namespace, resolution full/core log를 구현·검증했다.
- Wave 2 Step 4는 resolution sidecar를 소비하는 다수전 결착/종료 조건 evaluator, mutual-defeat precedence, max-tick stalemate, stable survivor/defeated report, combat-only cleanup sidecar를 구현·검증했다. 고급 AI utility·조기 tick 중단·renderer adapter는 아직 별도 plan이다.
- `fable_combat_wave3_step1a_2608020020.md`
- `crates/escape-core/src/combat_spectator.rs`
- `crates/escape-core/tests/combat_spectator_wave3.rs` 12개 테스트 (`spectate_is_deterministic_for_identical_input`, `frame_positions_facing_side_and_active_match_input`, `unknown_participant_is_rejected`, `attack_hit_and_evade_cues_follow_the_three_rules_only`, `cues_are_sorted_attack_then_hit_then_evade_with_no_duplicates`, `log_entries_use_registered_template_ids_not_free_sentences`, `full_log_is_ordered_by_tick_then_sequence`, `core_log_is_a_subset_of_full_log_filtered_by_importance_and_keeps_order`, `attack_roll_and_effect_suppressed_never_leak_into_any_log`, `hidden_conditional_and_unregistered_effect_ids_are_masked`, `fingerprint_chains_the_resolution_fingerprint`, `participant_input_order_does_not_affect_view`)
- Wave 3 Step 1a는 `CombatResolutionResult`를 입력으로 받아 새 판정 없이 tick별 체스말 프레임(좌표·facing·활성 여부), 공용 연출 cue(Attack/Hit/Evade, 판정 파생·정렬 고정), 템플릿 id 기반 이중 로그, 누설 차단(AttackRoll/EffectSuppressed 제외, Hidden/Conditional/미등록 효과 id 마스킹)까지 구현·검증했다. `BalanceBroken`/`Incapacitated` cue는 `CombatResolutionFrame`에 tick별 상태 스냅샷이 없어 의도적으로 제외했다(후속 slice 선행 필요). ScenePage/WASM/renderer 노출은 아직 별도 plan(Step 1b/1c/1d)이다.
- `fable_combat_wave2_step5_2608020117.md`
- Wave 2 Step 5는 `CombatResolutionFrame.combatants`(tick 종료 시점 전투원 스냅샷, `#[serde(default)]` additive-optional, id 오름차순)를 추가하고 `combat_spectator.rs`가 이를 소비해 `BalanceBroken`(균형 붕괴, `balance_hundredths <= 0`)·`Incapacitated`(전투불능, `current_health_hundredths <= 0`) cue 2개를 파생하도록 구현·검증했다. `crates/escape-core/tests/combat_resolution_wave2.rs`에 5개 테스트 추가(총 16개: `frame_snapshot_is_id_sorted_and_covers_every_combatant`, `frame_snapshot_reflects_the_tick_damage_from_its_own_outcomes`, `last_frame_snapshot_matches_final_state_combatants`, `combatants_field_is_additive_optional_for_deserialization`, `frame_snapshots_are_deterministic_across_identical_runs`), `crates/escape-core/tests/combat_spectator_wave3.rs`에 5개 테스트 추가(총 19개: `incapacitated_cue_marks_a_combatant_whose_health_snapshot_hit_zero`, `balance_broken_cue_marks_a_combatant_whose_balance_snapshot_hit_zero`, `neither_state_cue_applies_when_health_and_balance_stay_above_zero`, `cue_ordering_is_fixed_attack_hit_evade_balance_broken_incapacitated`, `empty_combatant_snapshot_yields_no_state_cues_and_no_error`). fingerprint 공식은 변경하지 않았다(`CombatResolutionFrame.fingerprint`는 여전히 `tick`+`outcomes`만 입력). 이로써 정본 13의 공용 연출 문법 5개(공격/피격/회피/균형 붕괴/전투불능)가 모두 확보됐다. renderer 노출·보고서 확장·밸런스 확정값은 여전히 별도 plan(Step 1b/1c/1d)이다.
- `fable_combat_wave3_step1b_2608020437.md`
- Wave 3 Step 1b는 `CombatConclusionRequest.tick_millis`(0은 `InvalidTickMillis`로 거부)를 추가하고, `CombatConclusionReport`에 `duration_millis`(결착 tick × tick_millis, 없으면 frame 수 × tick_millis), `combatants: Vec<CombatCombatantReport>`(id 오름차순, `damage_dealt_hundredths`/`damage_taken_hundredths`/`kills`/`incapacitated`), `top_damage_dealt_id`/`top_damage_taken_id`(발생하지 않으면 `None`, 동점은 id 최소)를 additive-optional로 추가했다. 집계는 `request.resolution.frames[].outcomes`·`combatants` 스냅샷만 읽고 판정을 재계산하지 않는다. `crates/escape-core/tests/combat_conclusion_wave2.rs`에 10개 테스트 추가(총 14개: `zero_tick_millis_is_rejected`, `duration_millis_uses_decisive_tick_plus_one_when_terminal`, `duration_millis_uses_frame_count_when_not_terminal`, `combatants_report_sums_damage_and_marks_incapacitated`, `kills_are_attributed_to_last_valid_lethal_outcome_in_the_ko_tick`, `top_damage_highlights_hidden_when_no_damage_occurs`, `top_damage_highlights_pick_max_with_lowest_id_tie_break`, `same_input_conclude_twice_yields_identical_report_and_fingerprint`, `shuffled_participant_order_yields_identical_report`, `deserializing_report_json_without_new_fields_uses_defaults`). 치유량·명줄은 아래 "아직 열지 않음"에 남긴다. 정본 13이 금지하는 전략 평가·핵심 전환점·자동 원인 분석·전략 조언·종합 MVP·이전 전투 비교는 의도적으로 구현하지 않았다.
- `fable_combat_wave2_step6_2608020838.md`
- Wave 2 Step 6은 `combat_execution.rs`에 `CombatProvenance`(`simulation_version`·`tick_millis`·`manifest_fingerprint`)를 정의하고 `execute()`가 항상 `Some(..)`으로 채우도록 구현했다(additive-optional, 구 JSON은 `None`). 이어서 `CombatSpectatorRequest.tick_millis`와 `CombatConclusionRequest.tick_millis` 중복 파라미터를 제거하고 `spectate()`/`conclude()`가 `resolution.execution.provenance`에서 `tick_millis`를 읽도록 바꿨으며, provenance가 없거나 `tick_millis == 0`이면 값을 지어내지 않고 두 에러 enum의 `InvalidTickMillis`를 대체한 `MissingProvenance`를 낸다. 판정 재계산·`effective_seed`/`namespace` 결정·fingerprint 계산 호출부는 손대지 않았다. `crates/escape-core/tests/combat_execution_wave2.rs`에 4개 테스트 추가(총 10개: `provenance_matches_input_manifest_version_tick_millis_and_fingerprint`, `forecast_mode_reports_the_same_provenance_as_input`, `deserializing_result_json_without_provenance_field_yields_none`, `same_input_executed_twice_yields_identical_provenance`). `combat_spectator_wave3.rs`(19개, 개수 유지: `view_reports_the_requested_tick_millis`→`view_reports_the_tick_millis_from_provenance`, `zero_tick_millis_is_rejected`→`missing_provenance_is_rejected`로 의미 갱신)와 `combat_conclusion_wave2.rs`(14개, 개수 유지: `zero_tick_millis_is_rejected`→`missing_provenance_is_rejected`)도 같은 방식으로 갱신했다. 이로써 Wave 3 Step 1c의 선결 과제(fingerprint 안정성)가 해소됐다 — 자세한 내용은 [Combat_System_Implementation_Plan_Index.md](../design/Combat_System_Implementation_Plan_Index.md)를 본다.
- `fable_combat_wave3_step1c_2608021109.md`
- `crates/escape-core/src/combat_spectator.rs`, `crates/escape-core/src/scene_page.rs`, `crates/escape-core/src/lib.rs`
- `crates/escape-core/tests/combat_spectator_wave3.rs` 20개 테스트 (`view_reports_the_simulation_version_from_provenance` 1개 추가)
- `crates/escape-core/tests/scene_page_combat_boundary.rs` 신규, 5개 테스트 (`content_backed_scene_page_has_no_combat_producer_and_no_combat_key_in_json`, `filled_combat_serializes_with_simulation_version_alongside_fingerprint`, `filled_combat_scene_page_round_trips_through_serde`, `scene_page_json_missing_combat_key_deserializes_to_none_without_error`, `combat_spectator_page_with_no_report_omits_report_key`)
- `crates/escape-wasm/tests/json_contract.rs` 기존 테스트 무수정, 1개 추가로 37개 테스트 (`json_boundary_scene_page_has_no_combat_key_before_combat_authoring_exists`)
- Wave 3 Step 1c는 `CombatSpectatorView.simulation_version`을 기존 `tick_millis` provenance 읽기 지점에서 함께 파생하고, `CombatSpectatorPage`(`view` + optional `report`)를 정의해 `ScenePage.combat: Option<CombatSpectatorPage>`로 renderer 경계 밖에 노출했다. 전투를 시작하는 인카운터 authoring이 아직 없어(Wave 3 Step 2) `scene_page_from_turn_view`는 `combat: None`만 낸다 — 억지 producer를 만들지 않았다. `#[serde(default, skip_serializing_if = "Option::is_none")]`로 `combat`이 `None`일 때 기존 `ScenePage` JSON이 바이트 단위로 동일하게 유지됨을 신규 테스트로 고정했다. RNG 호출·새 판정은 없다. terminal/Web 렌더러와 combat producer(authoring)는 여전히 별도 plan(Step 2/1d)이다.

아직 열지 않음:

- 다수전 AI 행동·결착·전투 종료 조건 resolver
- 치유량·최대 치유량 캐릭터 — combat 파이프라인에 회복 개념이 없어 보류 (healing slice 선행 필요)
- 명줄 소모·패배 결과 — 정본 10 기준 런 단위 메타 자원이며 인카운터 패배 결과 정의가 소유. 패배 결과 스키마 slice 선행 필요
- ScenePage/WASM/Web/terminal 전투 화면
- 기술 비용·호흡 회복률·피해·방어·쿨타임 등 밸런스 상수

## 4. context 압축 후 재개 절차

새 세션 또는 요약 직후에는 전체 repo를 다시 읽지 말고 다음 순서만 따른다.

1. `AGENTS.md`의 WSL·subagent·검증 규칙을 읽는다.
2. 이 문서와 [Combat_System_Goal_Prompt.md](Combat_System_Goal_Prompt.md)를 읽는다.
3. `docs/dev/Development_Plan.md` 상단과 `docs/design/Combat_System_Implementation_Plan_Index.md`의 현재 상태를 읽는다.
4. `git status --short --branch -uall`과 `git rev-parse HEAD`를 WSL에서 실행한다. 미추적 `.claude/worktrees/`는 보존한다.
5. 현재 step plan 한 파일과 그 plan이 소유한 코드만 읽는다.
6. 필요한 Notion canonical 문서만 다시 fetch한다. 이미 확인한 00~13 전체를 매번 반복 fetch하지 않는다.
7. coding agent에게 자기완결형 지시를 보내고, main 검증 전에는 완료로 표시하지 않는다.

## 5. plan 파일 분할 규칙

- 한 plan 파일은 한 coding agent가 한 번에 수행할 수 있는 한 slice만 소유한다.
- 각 plan에는 목적, 소유 파일, 선행 조건, acceptance criteria, non-goal, 검증 명령, 보고 형식을 적는다.
- 다음 단계의 schema를 미리 확정하지 않는다. 앞 단계에서 얻은 public contract와 테스트를 보고 다음 plan을 만든다.
- 실제 전투 엔진을 한 PR로 열지 않는다. 권장 순서는 contract → state/effect → opportunity/response → resolver → parity/로그 → renderer adapter → content QA다.
- 정확한 밸런스 값이 Notion에서 TODO면 data-driven placeholder로 남기고 임의 숫자를 넣지 않는다.

## 6. Notion 전투 불변식

- 같은 manifest·seed·선택 이력·simulation version은 같은 결과와 로그를 만든다. 동일 version 내부에서만 재현을 보장한다.
- story resolution, encounter composition, actual combat, forecast ensemble, cosmetic presentation RNG를 분리한다. forecast는 실제 전투 RNG를 재사용하지 않는다.
- 전투는 체스말이 움직이는 seed 기반 결정론적 실시간 시뮬레이션이며, 텍스트 전투나 renderer-local physics가 아니다.
- 플레이어는 전투 전 배치·역할·우선 목표·전략을 설정하고 전투 중에는 제한된 감독형 개입만 한다. 전술 개입 상한은 인카운터 중요도/유형이 정하는 0~3회다.
- 생명력은 월드와 공유하는 유일한 신체 건강 수치다. 균형·호흡·공포·분노·자세·무기 제어·거리·압박·시야·대형·결속은 서로 다른 상태 층이다.
- 상태이상 (전투)은 결착 시 제거하고, 상태이상 (지속)은 사건/패배 결과가 명시할 때만 부여한다.
- 선택지 후보는 기회 감지와 실행 성공을 분리한다. 실패도 no-op가 아니라 새 상태·결과를 만든다.
- UI는 체스판과 핵심 로그를 보여 주고, 원인 분석·전략 조언·MVP 자동 평가는 제공하지 않는다.

## 7. WSL 명령 규칙

모든 개발 명령은 다음 저장소에서 실행한다.

```bash
cd /home/dudu/work/tui-adv
```

기본 확인:

```bash
git status --short --branch -uall
git rev-parse HEAD
git log -3 --oneline
```

Rust 변경 후 기본 검증:

```bash
cargo fmt --all -- --check
cargo test -p escape-core --test <targeted_test>
cargo test --workspace --no-fail-fast
git diff --check
```

commit/push/PR은 사용자가 명시했을 때만 수행한다. 그때도 commit 직전과 push 직전에 HEAD·status를 재확인하고, 의도하지 않은 변경이 보이면 멈춘다.

## 8. subagent 지시 템플릿

지시는 반드시 다음을 포함한다.

- 읽어야 할 정확한 plan 경로
- 소유/수정 가능 파일 목록
- 수정 금지 파일과 금지 범위
- Notion 계약 중 이번 slice에 적용할 항목
- acceptance criteria와 검증 명령
- 보고 형식

subagent가 “환경 문제”라고 보고하면 `which cargo`, `which wasm-pack`, `which gh`부터 main이 확인한다. subagent가 생성물을 만들었다고 보고해도 `git status`, diff, 실제 import 경로로 확인한다.

## 9. goal 운영법

장기 작업에는 `/goal`이 유용하지만 목표를 “전투 시스템 전체 완성”으로 잡지 않는다. Wave 또는 한두 개의 plan slice를 하나의 goal로 잡고, 완료 시 goal을 닫은 뒤 다음 goal을 만든다.

좋은 goal:

> Wave 1 Step 2의 전투원 상태·effect catalog 계약을 구현하고, manifest 투영·검증·회귀 테스트를 WSL에서 통과시킨다.

나쁜 goal:

> Notion의 전투 시스템을 전부 구현한다.

목표 완료 조건은 코드뿐 아니라 plan 상태 갱신, main 직접 검증, 미완료/non-goal 보고까지 포함한다.

## 10. 압축용 상태 checkpoint

context가 부족해지면 다음 형식으로 짧은 checkpoint를 남긴다.

```text
COMBAT_CHECKPOINT
HEAD: <git rev-parse HEAD>
BRANCH: <branch>
DIRTY: <git status 요약; .claude/worktrees 보존>
NOTION: hub + canonical 00~13 확인 여부, 마지막 fetch 문서
DONE: 완료된 plan 파일과 핵심 public API
ACTIVE_PLAN: 현재 coding agent가 수행 중인 파일
NEXT: 다음 plan 파일과 선행 조건
VERIFY: main에서 실행한 명령과 결과
BLOCKERS: 사용자 결정이 필요한 항목
GOAL: 활성 goal id/목표 또는 없음
```

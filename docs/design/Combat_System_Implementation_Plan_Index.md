# 전투 시스템 구현 계획 인덱스

status: wave3-step1d2-complete
기준일: 2026-08-02

이 문서는 보존된 Notion `전투 시스템` 허브와 local design source의 관련 canonical 문서를 Rust GameCore 구현 순서로 쪼갠 인덱스다. 각 단계 문서는 한 번의 coding subagent 작업으로 완료할 수 있는 크기를 목표로 한다.

## 원본과 우선순위

- 허브: [전투 시스템](https://app.notion.com/p/36f37e69695e81168360ef11cf3a4449)
- 하위 설계 provenance: 00~13 문서. current design records와 충돌하면 local design source를 우선하고, 필요하면 mirror 검수만 수행한다.
- 저장소 truth: `crates/escape-core` → `ScenePage`/WASM JSON → Web Storybook·SuperLightTUI.
- 기존 `docs/design/Combat_System_Auto_Brawl.md`는 이전 schema-less 방향의 설계 기록이다. 새 구현 계약은 local design source와 runtime schema를 우선하되, 기존 renderer-neutral 원칙과 non-goal은 유지한다.

## 현재 코드와 정본의 경계

Wave 1 Step 1~3과 Wave 2 Step 1~4가 `escape-core`에 구현·검증되어 initial manifest/RNG 분리, 전투원 상태/effect catalog, opportunity 후보, 고정 정수 좌표·role/target·동시 tick frame, 실행 mode parity·dual log, 실제 collision/attack/damage/effect resolution sidecar, 다수전 결착/종료 조건 sidecar 계약을 제공한다.

Wave 3 Step 1a가 여기에 `escape-core` 전용 관전 view 어댑터(`combat_spectator.rs`)를 더해 tick별 체스말 프레임, 공용 연출 cue(Attack/Hit/Evade), 템플릿 id 기반 이중 로그, 누설 차단(숨은 판정·억제 사유·Hidden/Conditional 효과 id 마스킹)을 제공한다 (`crates/escape-core/tests/combat_spectator_wave3.rs`, 현재 19개 테스트: `spectate_is_deterministic_for_identical_input`, `attack_roll_and_effect_suppressed_never_leak_into_any_log`, `hidden_conditional_and_unregistered_effect_ids_are_masked` 등).

Wave 2 Step 5가 여기에 `CombatResolutionFrame.combatants`(tick 종료 시점 전투원 스냅샷, additive-optional)를 더하고 이를 소비해 `BalanceBroken`(균형 붕괴)·`Incapacitated`(전투불능) cue 2개를 파생한다 (`crates/escape-core/tests/combat_resolution_wave2.rs`의 `frame_snapshot_is_id_sorted_and_covers_every_combatant`·`last_frame_snapshot_matches_final_state_combatants`, `crates/escape-core/tests/combat_spectator_wave3.rs`의 `cue_ordering_is_fixed_attack_hit_evade_balance_broken_incapacitated` 등). 이로써 정본 13의 공용 연출 문법 5개(공격/피격/회피/균형 붕괴/전투불능)가 모두 확보됐다.

Wave 3 Step 1b가 여기에 `combat_conclusion.rs`의 `CombatConclusionReport`를 확장해 `tick_millis` 입력 기반 `duration_millis`(전투 시간), 캐릭터별 `damage_dealt_hundredths`·`damage_taken_hundredths`·`kills`·`incapacitated` 집계(`combatants`, id 오름차순), 최대 피해 가한/받은 캐릭터 하이라이트(`top_damage_dealt_id`/`top_damage_taken_id`, 발생하지 않으면 `None`)를 더한다. 판정은 `request.resolution.frames[].outcomes`·`combatants` 스냅샷만 집계하며 재계산하지 않는다 (`crates/escape-core/tests/combat_conclusion_wave2.rs`, 현재 14개 테스트: `combatants_report_sums_damage_and_marks_incapacitated`, `kills_are_attributed_to_last_valid_lethal_outcome_in_the_ko_tick`, `top_damage_highlights_pick_max_with_lowest_id_tie_break`, `same_input_conclude_twice_yields_identical_report_and_fingerprint` 등).

Wave 2 Step 6이 여기에 `combat_execution.rs`의 `CombatExecutionResult.provenance`(`CombatProvenance`: `simulation_version`·`tick_millis`·`manifest_fingerprint`, additive-optional)를 더해 `execute()`가 이미 손에 쥔 입력 맥락을 결과에 심고, `spectate()`/`conclude()`가 각자 받던 중복 `tick_millis` 파라미터를 제거해 그 provenance에서 읽게 한다 (`crates/escape-core/tests/combat_execution_wave2.rs`, 현재 10개 테스트: `provenance_matches_input_manifest_version_tick_millis_and_fingerprint`, `forecast_mode_reports_the_same_provenance_as_input`, `deserializing_result_json_without_provenance_field_yields_none`, `same_input_executed_twice_yields_identical_provenance` 등; `combat_spectator_wave3.rs`의 `view_reports_the_tick_millis_from_provenance`·`missing_provenance_is_rejected`; `combat_conclusion_wave2.rs`의 `missing_provenance_is_rejected`). provenance가 없거나 `tick_millis == 0`이면 값을 지어내지 않고 `MissingProvenance` 에러를 낸다.

Wave 3 Step 1c가 여기에 `CombatSpectatorView.simulation_version`(provenance에서 파생, 기존 `tick_millis` 읽기 지점과 같은 곳에서 함께 꺼낸다)과 `CombatSpectatorPage`(`view` + optional `report`)를 더하고, `ScenePage.combat: Option<CombatSpectatorPage>`로 이를 renderer 경계 밖에 additive-optional 노출한다 (`crates/escape-core/tests/combat_spectator_wave3.rs`의 `view_reports_the_simulation_version_from_provenance`; `crates/escape-core/tests/scene_page_combat_boundary.rs`, 신규 5개 테스트: `content_backed_scene_page_has_no_combat_producer_and_no_combat_key_in_json`, `filled_combat_serializes_with_simulation_version_alongside_fingerprint`, `filled_combat_scene_page_round_trips_through_serde`, `scene_page_json_missing_combat_key_deserializes_to_none_without_error`, `combat_spectator_page_with_no_report_omits_report_key`; `crates/escape-wasm/tests/json_contract.rs`의 `json_boundary_scene_page_has_no_combat_key_before_combat_authoring_exists`).

Wave 3 Step 2a가 여기에 `EncounterCombatDef`/`EncounterCombatKind` 스키마와 `EncounterDef.combat`(additive-optional)을 더하고, `scene_page_from_turn_view`에 시스템형(`EncounterCombatKind::Systemic`) 전용 producer를 연결해 `ScenePage.combat`을 실제로 채운다(`resolve_combat`이 내부에서 `execute_combat`을 부른 뒤 → `conclude_combat` → `spectate_combat` 순으로 이어진다). 실제 전투 seed는 authoring의 `manifest.actual_seed`를 쓰지 않고 런 상태(`GameState.seed`) + 인카운터 id를 해싱한 값으로 그 필드를 먼저 덮어쓴 뒤, 기존 `CombatManifest::derived_seed(ActualCombat)` FNV 파이프라인으로 나머지 manifest 내용을 섞어 최종 seed를 얻는다 — 새 난수원은 도입하지 않았다 (`crates/escape-core/tests/encounter_combat_wave3.rs`의 `systemic_combat_producer_result_is_independent_of_authoring_actual_seed`가 authoring seed 무관성을 증명한다). index-time 검증이 개입 예산 0~3 초과, `mixed`/`scripted` kind(둘 다 Wave 3 Step 2b/2c 소관, 오류 메시지에 인카운터 id 포함), tick 설정, attack/defense 참조 무결성, participants/combatants id 집합 일치, effect catalog·manifest 유효성 등 11개 규칙을 하드 오류로 거부한다 (같은 테스트 파일, 21개 테스트). 파이프라인 실패는 `ScenePage.combat`을 `None`으로 두지 않고 `ContentTurnError::CombatProducer { encounter_id, reason }`으로 전파한다. 구현 당시에는 기존 `UnknownStateLocation(String)` payload를 재사용했으나, 터미널에 "unknown state location: combat producer failed …"로 출력되어 사용자에게 보이는 표면에서 원인을 잘못 지목하므로 리뷰에서 전용 변형으로 교체했다 (`encounter_combat_wave3.rs`의 `combat_producer_failures_report_as_their_own_error_variant`가 Display 문자열과 `UnknownStateLocation`이 아님을 고정한다). 이 producer는 `scene_page_from_content` 호출마다 전투를 처음부터 재실행한다 — 결정론적이라 결과는 같지만 렌더마다 비용이 든다; 캐싱은 save schema 결정이 필요해 후속 slice로 미룬다.

Wave 3 Step 2b가 여기에 **첫 실 콘텐츠 확보분**을 더한다: 이구학지 preview `encounters.yaml`에 시스템형 전투 인카운터 1개(`wuxia_combat_spectator_preview_bout`, 위치 `cheongryu_outer_courtyard`)를 authoring해 Step 2a producer가 실제로 구동하도록 배선했다. 전투원 2명 모두 정본 11의 표준 전투원 수치(위력 40/`power_hundredths: 4000`, 능력배율 1.0/`ability_multiplier_hundredths: 100`, 명중 `accuracy_percent: 100`, 방어 5/`defense_hundredths: 500`, 생명력·호흡 100/100)를 그대로 썼고 새 밸런스 값을 발명하지 않았다. 정본에 없는 필드(균형 최대치·이동 속도·충돌 반경·사거리·tick 길이·균형 피해)는 모두 YAML 주석에 provisional로 표시했다(아래 "아직 없는 것" 참고). `intervention_budget: 0`·`kind: systemic`이며, staged `event`(Story→Choice→per-choice Result)와 illustration placeholder(`placeholder: true`)를 갖췄고 서술은 승패를 단정하지 않는다. `crates/escape-core/tests/encounter_combat_wave3.rs`에 7개 테스트를 추가했다(총 28개): `wuxia_combat_spectator_preview_bout_first_hit_damage_is_1333_hundredths`가 정본 §8 공식 검산(`pre = 4000*5*100/1200 = 1666`, `reduction = 1666*500/2500 = 333`, `damage = 1333`)으로 authoring 수치와 resolver 공식을 함께 고정하고, 나머지가 게이트 부재 시 선택 불가·게이트 시 `ScenePage.combat` 충전·2인 리포트·결정론·staged event 존재를 고정한다. 인카운터 수가 51→52로 늘어 `crates/escape-core/tests/event_stage_wave3.rs`의 카운트 단정(`wuxia_preview_has_full_51_event_coverage`→`wuxia_preview_has_full_52_event_coverage`로 함수명도 갱신)과 `content_bundle.rs`·`reward_pipeline_wave1.rs`·`tests/test_web_data_export.py`의 51 카운트 단정을 52로 갱신했다(로직은 그대로).

Wave 3 Step 1d-1이 여기에 **terminal(SuperLightTUI) 관전 렌더러**를 더한다: `crates/escape-terminal/src/snapshot.rs`가 `ScenePage.combat`을 텍스트로만 포맷하고(판정·집계·cue 재계산 없음, `resolve_combat`/`conclude_combat`/`spectate_combat` 호출 0회), 등록된 6개 `template_id`를 한국어 문장 형식표로 매핑하며(알 수 없는 id는 `template_id` 자체를 노출하는 fallback 줄), `view.frames`의 마지막 프레임을 체스말 보드로 그리되 폭 32·높이 16을 넘으면 좌표 목록으로 대체하고, cue 5종(Attack/Hit/Evade/BalanceBroken/Incapacitated) 전부에 텍스트 표식(`>`/`<`/`~`/`!`/`x`)을 붙이고, `core_log`만 문장화하고 `full_log`는 개수만 표시하며(줄 수 상한 초과 시 생략 개수 명시), `combat.report`가 `Some`일 때만 전투 종료 보고서(승패·사유·전투 시간·생존/전투불능·최대 피해 하이라이트는 `None`이면 줄 자체를 숨김·캐릭터별 상세)를 그린다. `page.combat`이 `None`이면 이 렌더러는 한 줄도 추가하지 않아 스냅샷 출력이 이 slice 이전과 바이트 단위로 동일하다(`snapshot::tests::scene_snapshot_unchanged_bytes_when_combat_is_none`). `crates/escape-terminal/tests/cli_smoke.rs`(다른 작업자의 uncommitted 변경, 무수정)의 기존 61개 테스트가 그대로 통과해 회귀가 없음을 증명한다. `crates/escape-terminal/src/snapshot.rs`의 `#[cfg(test)] mod tests`에 24개 단위 테스트를 새로 추가했다(`cargo test -p escape-terminal`: 24 + 61 = 85, `cargo test --workspace --no-fail-fast`: 322 + 24 = 346, 0 failed). 게이트 플래그(`combat_spectator_preview_unlocked`)는 그대로 뒀다 — Web 렌더러가 없는 상태에서 게이트를 풀면 일반 플레이 경로에 노출되기 때문이며, 게이트 제거는 Web 렌더러가 갖춰지는 Step 1d-2 소관이다.

Wave 3 Step 1d-2가 여기에 **Web Storybook 관전 표면(정지 프레임)**과 그 TS 타입 계약을 더한다: `web/src/core/types.ts`에 `CombatSpectatorPage`/`CombatSpectatorView`/`CombatSpectatorFrame`/`CombatSpectatorPiece`/`CombatSpectatorLogEntry`/`CombatConclusionReport` 등을 Rust serde 표현과 1:1로 추가하고 `ScenePage.combat?`으로 additive-optional 노출했다. `web/src/ui/storybook/combat/combatLogTemplates.ts`가 terminal의 `combat_log_template_line`과 글자 단위로 같은 6개 `template_id` 문장 테이블을 renderer 쪽에 두되(알 수 없는 id는 `template_id`를 노출하는 fallback 문장), 한 가지 의도적 차이를 둔다 — terminal은 `value_hundredths: None`을 `unwrap_or(0)`로 0으로 채우지만 이 슬라이스의 플랜(I6)이 명시한 대로 "(수치 없음)"을 쓴다(terminal 쪽 이 경로는 테스트로 고정돼 있지 않다). `web/src/ui/storybook/combat/renderCombatStage.ts`가 `view.frames`의 마지막 프레임만 체스말 보드로 투영하고(좌표 span이 0이면 0으로 나누지 않고 50% 중앙, 물리 좌표 `--piece-x`/`--piece-y` + `translate`로 RTL에서 좌우가 뒤집히지 않게 하며), semantic `<table>`(말 id·진영·좌표·상태·cue) 접근 대체를 `sr-only`로 병행 제공하고, `core_log`만 문장화하고 `full_log`는 개수만 표시하며(`WEB_CORE_LOG_LIMIT = 40` 초과 시 "…(생략 N줄)" 명시), `combat.report`가 있을 때만 전투 종료 보고서를 그리되 `top_damage_dealt_id`/`top_damage_taken_id`/`decisive_tick`이 `null`이면 그 줄 자체를 생략하고(`renderCombatStage.test.ts`) 금지 문구(전략 평가·핵심 전환점·자동 원인 분석·전략 조언·종합 MVP·이전 전투 비교)를 만들지 않는다. `combat.log.damage_applied` 로그 줄만 Hit cue와 같은 색·글리프(`打`/`--seal-red-lit`)를 쓰고 나머지 5개 template id는 중립 잉크색으로 둔다(core에 대응하는 cue가 없어 대응을 발명하지 않는다, I11). `web/src/ui/storybook/render.ts`의 `renderStorybookPage`가 `renderCombatStage(page.combat)`을 `.storybook-page` 안에 삽입하며, `page.combat`이 `undefined`면 빈 문자열을 반환해 기존 52개 인카운터의 출력에 `combat-stage`/`data-region="combat"` 문자열이 전혀 나타나지 않는다(`render.test.ts`의 `I5: emits no combat markup at all when page.combat is absent`). `web/src/styles/storybook.css`에 `.combat-stage`(`grid-template-rows: minmax(0, 70fr) minmax(0, 30fr)`로 보드:로그 = 70:30, `inline-size: 100%` + `aspect-ratio` + `max-block-size: min(70dvh, 44rem)` 사이징, 고정 px 높이 없음), 체스 폰 실루엣(타원 `border-radius`, 아군=먹 채움·적=짙은 종이 채움 + 2px 파선 윤곽), cue 5종 색 토큰(`--seal-red`/`--seal-red-lit`/`--jade`/`--gold-leaf`/`--ink-faded`), `@media (forced-colors: active)` 대체(`CanvasText`/`Canvas`/`Highlight`/`GrayText`)를 추가했다 — 기존 14개 토큰 밖의 신규 색상 리터럴 0개, `transition`/`animation` 선언 0개, 기존 `.storybook-shell`/`.game-viewport`/`.game-topbar`/`.storybook-hud`/`.storybook-dock` 규칙 무수정. 테스트를 먼저 red로 만든 뒤 구현했다: `combatLogTemplates.test.ts`(11 테스트), `renderCombatStage.test.ts`(22 테스트: 보드 9 + 로그/보고서/통합 13), `render.test.ts`에 통합 2건 추가. `cd web && npm test`: 15 파일 108 테스트, 0 failed(이 슬라이스 이전 73 + 신규 35). `crates/`·YAML·두 번들 JSON·게이트 플래그(`combat_spectator_preview_unlocked`)·`web/src/main.ts`·`web/package.json`은 무변경이다 — 이 슬라이스는 관전 표면을 렌더할 수는 있지만 게이트가 남아 있어 일반 플레이 경로에서 여전히 도달할 수 없다.

한국어 라벨 참고: `renderCombatStage.ts`의 전투 종료 사유·결과 라벨(`OUTCOME_LABELS`/`REASON_LABELS`)은 `crates/escape-terminal/src/snapshot.rs`의 실제 문자열(`적 승리`/`양측 전멸`/`무승부`/`종료 조건 없음`/`적 전멸`/`최대 tick 도달`)을 그대로 따랐다 — 이 플랜 문서 §6의 라벨 표(`적군 승리`/`양측 패배`/`교착`/`종료 조건 미충족` 등)는 실제 terminal 소스와 어긋난 초안 문구였다. §6이 "terminal과 같게"를 요구하므로 Rust 소스를 기준으로 삼았다. §6 라벨 표는 이 사실을 반영해 갱신했다 — 이제 플랜이 "표를 믿지 말고 `snapshot.rs`의 `combat_outcome_label`/`combat_reason_label`을 열어 그 문자열을 쓰라"고 지시한다.

**실측으로 확인한 core/authoring 쪽 결함 (Wave 3 Step 1d-2 시점, 렌더러 범위 밖)**

저작된 시스템형 인카운터(`wuxia_combat_spectator_preview_bout`, seed 2)의
`ScenePage.combat`을 직접 덤프해 프레임·로그·보고서를 읽었다. 렌더러는 이
데이터를 충실히 옮기고 있지만 데이터 자체에 다음 문제가 있다. 게이트를 계속
유지해야 하는 근거이기도 하다.

- **`piece.active`는 생존 여부가 아니다.** `participant.active`(authoring 정적
  값)에서 오므로 체력이 0이 된 tick 8 이후에도 계속 `true`다. 전투불능은
  `Incapacitated` cue로만 나타난다. → 렌더러가 이 값을 "생존"으로 표시하면
  거짓이 된다. Web 접근 대체 표는 "참전/비참전"으로 바꿨고
  `renderCombatStage.test.ts`의 `never calls an active piece "생존" — active is
  participation, not liveness`가 고정한다. 생존·전투불능은 보고서의
  `survivor_ids`/`defeated_ids`가 소유한다.
- ~~**조기 결착이 없다.**~~ **해결됨** (`fable_combat_early_conclusion_step1_2608022130.md`).
  전투불능(체력 0) 전투원이 더 이상 공격하지 않고, 한쪽이 전멸한 tick에서
  시뮬레이션이 멈춘다. 관전 화면의 시간 범위도 판정된 tick까지로 묶었다 —
  `spectate`가 `execution.frames`(이동·AI 패스, 항상 tick 상한까지 생성)를
  그대로 돌아서 결착 뒤에도 말이 움직이고 있었다.
  실측 변화: 프레임 10→**8**, 마지막 tick 10→**8**, `core_log` 40→**32**,
  `decisive_tick` 10→**8**, `duration_millis` 1100→**900**.
  고정 테스트: `simulation_stops_at_the_tick_that_concludes_the_fight`,
  `the_concluding_tick_frame_is_included_with_its_outcomes`,
  `a_fight_with_no_conclusion_runs_every_tick`,
  `simultaneous_mutual_defeat_stops_at_that_tick`,
  `a_side_with_no_active_participant_does_not_read_as_wiped_on_tick_one`,
  `incapacitated_actor_and_incapacitated_target_are_both_skipped`,
  `simultaneous_mutual_defeat_is_independent_of_attack_definition_order`,
  `spectator_view_never_extends_past_the_last_resolved_tick`.
- **두 전투원이 서로를 통과해 좌우가 뒤바뀐다.** 아군 x는 1,2,3,2,3,…,
  도전자 x는 4,3,2,3,2,… 로 진동하며 tick 3부터 아군이 도전자보다 오른쪽에
  놓인다. 정본 09의 **"화면 왼쪽: 아군 영역 / 화면 오른쪽: 적 영역"** 계약을
  재생 중에 위반한다. 역할 가중치 `preferred_distance: 0`이 관통을 만든다.
  → 렌더러가 좌표를 왜곡해 맞추면 거리 읽기가 망가지므로 고칠 곳은 AI·충돌
  규칙이다. 저작 시점 좌표만 검사하는 테스트로는 잡히지 않는다.
- **로그 도배 (일부 완화).** 조기 결착으로 `core_log`가 40→32건이 됐지만
  여전히 `move_intent`/`damage_applied` 반복이다. 정본 13의 **"원시 사건은
  연관 전투 상황으로 묶어 로그 도배를 막는다"**는 아직 구현되지 않았다.
- **표준 대련이 양측 전멸로 끝난다.** 정본 11의 표준 전투원끼리 대칭 교전이라
  구조적으로 그렇게 되지만, "관전용 표준 대련" 서사와 어긋난다. 조기 결착이
  들어간 뒤에도 결과는 `MutualDefeat`이다 — 남은 것은 밸런스·AI(방어 행동)
  쪽이다.
- **전투불능 전투원이 결착 전 tick에서 여전히 이동한다.** 조기 결착은 결착
  *이후*를 없앴지만, 한쪽이 먼저 쓰러지고 다른 쪽이 나중에 쓰러지는 전투에서는
  그 사이 tick에 쓰러진 말이 계속 움직인다. `execute_combat`(이동·AI)이
  `resolve`(피해)보다 먼저 통째로 돌기 때문이며, 고치려면 두 패스를 tick 단위로
  인터리빙해야 한다.
- **코드가 `simulation_version`을 검증하지 않는다.** 저작이 선언한 값을 그대로
  기록에 실어 보내므로, 코드가 실제로 구현하지 않는 version을 적어도 아무도
  잡지 못한다. 정본 03은 "결정성은 같은 version 내부에서만 보장한다"고 정했고
  이 계약이 코드로 강제되지 않는다. 강제하려면 기존 픽스처 전체를 `v1`에서
  옮겨야 하므로 별도 슬라이스다.
- **나머지 결착 유형이 없다.** 정본 03은 "승리·패배·도주·항복·포획·목표 달성·
  강제 중단"을 결착으로 열거하지만 판정 규칙은 정본 어디에도 없다. 구현된 것은
  코드가 이미 소유한 두 조건(한쪽 전멸 / 양쪽 전멸)뿐이다.

- 고급 다수전 AI 행동 (조기 결착/전투 tick 중단은 `fable_combat_early_conclusion_step1_2608022130.md`에서 완료)
- 대형·결속·배경 전투·증원과 전투 종료 조건
- **혼합형·각본형의 개입 일시정지 흐름** — `EncounterCombatKind::Mixed`/`Scripted`는 스키마로 받되 index-time에서 명시적 오류로 거부한다(정본 12 하드 오류 원칙). 개입 기회/대응 제시(`combat_opportunity.rs`와 encounter의 연결), 행동 선택지 최대 4개 + "개입하지 않는다" 선택지 → Wave 3 Step 2c.
- **전투 결과 저장(캐싱)** — 현재는 매 렌더 재실행 중이다. save schema 결정이 선행돼야 한다.
- **혼합형·각본형 인카운터 콘텐츠 authoring** — Step 2c 소관이다. 시스템형 1개는 Wave 3 Step 2b에서 authoring됐다(`wuxia_combat_spectator_preview_bout`).
- **이 인카운터는 `combat_spectator_preview_unlocked` 게이트 뒤에 있다** — terminal·Web 렌더러 모두 확보됐고(Step 1d-1, Step 1d-2) 재생 연출도 확보됐지만(Step 1d-3) 게이트는 아직 제거하지 않았다. Step 1d-4(전투원 표시 이름 결정 이후)에서 게이트를 풀어 정식 경로로 승격한다 — Step 1d-1에서 세운 "실시간 연출이 없는 화면을 플레이어 경로에 올리지 않는다"는 판단을 유지한다.

Step 1d-3이 확보한 것: `frames` 배열 → 말별 `@keyframes` 생성(`combatMotion.ts`), 투영 범위를 마지막 프레임에서 전체 프레임으로 확장, cue 5종 연출 문법(`attack`/`hit`/`evade`/`balance_broken`/`incapacitated`), `core_log` tick 시각 동기 노출, 모두 `@media (prefers-reduced-motion: no-preference)` 안에만(`reduce`는 1d-2의 정지 출력 그대로). 게이트 플래그는 유지했다(정본 09 축 계약이 재생 중 깨지는 사실을 이번에 확인했기 때문 — 아래 참고).

**Wave 3 Step 1d-4 (남은 범위, 별도 플랜 필요)**:
  - 게이트 플래그(`combat_spectator_preview_unlocked`) 제거.
  - wasm 재빌드, 5뷰포트(`web/scripts/storybook-reference-qa.mjs`의 `VIEWPORTS`: 390x844/414x896/800x1440/810x1644/1440x1000) 실화면 QA(`npm run qa:storybook:visual`) — Step 1d-2·1d-3은 게이트가 남아 있어 시작 화면에서 인카운터에 도달할 수 없으므로 이 QA를 실행할 수 없었다.
  - **전투원 표시 이름** — 현재 관전 화면과 보고서는 core가 주는 내부 id(`wuxia_spectator_bout_ally` 등)를 그대로 플레이어에게 보여준다. `CombatSpectatorPiece`/`CombatCombatantReport`에 표시 이름이 없고 인카운터 authoring에도 이름 필드가 없어 renderer가 유도할 수 없다. 이름을 발명하지 않고 남겨 두었다 — 게이트를 푸는 슬라이스 전에 인카운터 combat authoring에 표시 이름 필드를 추가할지 결정해야 한다.
  - **정본 09 축 계약 위반(재생 중)** — Step 1d-3에서 실측 확인: 저작 시점 좌표(아군 x=0/도전자 x=5)는 "화면 왼쪽=아군, 오른쪽=적" 계약을 지키지만, 실제 프레임을 덤프하면 아군 x가 1,2,3,2,3,… 도전자 x가 4,3,2,3,2,… 로 진동하며 tick 3부터 아군이 도전자보다 오른쪽에 놓인다(역할 가중치 `preferred_distance: 0`이 두 말을 서로 통과시킨다). 저작 시점 좌표만 검사하는 테스트로는 잡히지 않는다. 고칠 곳은 `crates/escape-core`의 AI·충돌 규칙이며(`combat_resolution.rs`), Step 1d-3은 문서화만 했다(`docs/design/Mobile_Ink_Storybook_UI.md`) — 렌더러가 좌표를 왜곡해 "고치면" 거리 읽기가 망가지므로 web 슬라이스의 범위가 아니다. 게이트를 유지하는 근거 중 하나다.
  - **실화면 검증 방법** — 게이트가 남아 있는 동안에는 `npm run qa:storybook:visual`이 이 화면에 도달하지 못한다. Step 1d-2에서는 오케스트레이터가 `web/dist`의 실제 빌드 CSS를 링크한 임시 하네스 HTML을 만들어 320/390/1280에서 실측했고(하네스는 커밋하지 않았다), 그 과정에서 단위 테스트가 잡지 못한 결함 3건(cue 표식 겹침, 적 말 대비 부족, 데스크톱에서 보드 너비가 764px 칸 안에서 420px로 축소)을 찾아 고쳤다. Step 1d-3의 재생 연출(총 길이, 보드 이탈 여부, reduced-motion 경로, 로그 타이밍)은 같은 이유로 같은 방식의 실측이 아직 없다 — Step 1d-4에서 게이트를 푼 뒤 정식 5뷰포트 QA로 대체한다.

**전체 로그(`full_log`) 열람 — Web 완료** (`fable_combat_full_log_viewer_step1_2608022300.md`): `renderCombatStage.ts`에 `renderCombatFullLog`를 더해 `view.full_log`를 상한 없이 보여주는 `<details>`/`<summary>` 열람 섹션을 `combat.report`가 있을 때만(정본 07/13의 "전투 종료 뒤" — 이 슬라이스는 일시정지 흐름은 만들지 않는다) `.combat-stage`의 70:30 그리드 밖·보고서와 같은 층에 추가했다. 각 줄은 `combatLogTemplateLine`으로 문장화하고(새 문장 형식 없음), `entry.importance`를 그대로 써 `data-importance`(routine/important/decisive) + 정본 13 한국어 라벨(일반/중요/결정적)을 낸다. `core_log`는 `full_log`의 `importance >= important` 부분집합이라는 정의를 그대로 판정에 써 `data-in-core-log="true"`를 붙이되, **대응 관계는 목록 앞에서 한 번만 문장으로 밝힌다**(`.combat-full-log__legend`). 처음에는 줄마다 "핵심 로그에도 있음" 배지를 붙였는데, 실측에서 64줄 중 32줄이 그 배지 때문에 두 줄로 늘어나 목록을 훑을 수 없었고 중요도 칩이 이미 말하는 것을 반복하는 것뿐이었다(core_log가 정확히 `importance >= 중요`이므로). 배지를 빼자 390px에서 줄 높이가 43px로 균일해지고 목록 전체 높이가 3551→3039px로 줄었다. `states the core-log correspondence once, not on every row`가 배지 재도입을 잡는다. 로그 영역 메타 줄(`.combat-log__meta`)도 갱신했다 — `combat.report`가 있을 때만 "아래 전체 로그 열람에서 확인할 수 있다"고 말하고, 없을 때는(전투 진행 중) 읽을 수 있다고 주장하지 않는다. `storybook.css`에 `.combat-full-log` 계열 스타일을 기존 14개 토큰만으로 추가했다(신규 색상 리터럴 0개, `animation`/`transition` 선언 0개, `@media (forced-colors: active)` 대체 포함). 테스트를 먼저 red로 만든 뒤 구현했다: `renderCombatStage.test.ts`가 49 테스트로 늘었다(1d-3 시점 33 + 신규 16). `cd web && npx vitest run`: 16 파일 164 테스트, 0 failed(1d-3 시점 148 + 신규 16). `crates/`·YAML·두 번들 JSON·게이트 플래그·`web/src/main.ts`·`web/package.json`·`combatMotion.ts`·`combatLogTemplates.ts`는 무변경이다. **terminal(SuperLightTUI)은 여전히 개수만 표시한다** — terminal 쪽 열람 UI는 이 슬라이스의 범위 밖이며 별도 슬라이스로 남는다(아래 갭 항목). 실화면 검증: `renderCombatStage()` 출력을 실제 빌드 CSS와 함께 감싼 임시 하네스를 `web/dist`(gitignore됨)에 만들고 WSL 안에서 Playwright로 **320/390/1280** 폭을 계측했다(하네스·스크립트 모두 커밋하지 않음). 확인한 것: 64줄 전부 렌더, legend 1회, 배지 0개, 세 폭 모두 가로 스크롤 0(`listScrollW == listClientW`), 목록 내부 스크롤 동작, 보드:로그 0.700 유지, `<summary>` 높이 38px(터치 타겟 24px 이상). 게이트가 남아 있어 `npm run qa:storybook:visual`(시작 화면에서 실제 플레이로 진입)은 여전히 이 화면에 도달하지 못하며, 그 정식 5뷰포트 QA는 게이트를 푸는 Step 1d-4 소관이다.

- **전체 로그(`full_log`) 열람 UI — terminal, 일시정지 흐름** — Web 관전 화면은 전투 종료 뒤 전체 로그 열람 섹션을 갖췄다(위 문단). terminal(SuperLightTUI) 관전 화면은 여전히 개수만 보여주며 열람 UI가 없다. 일시정지 중 열람(정본 07/13의 나머지 절반)도 아직 없다 — 일시정지 흐름 자체가 개입(Step 2c)과 얽혀 있어 별도 슬라이스다.
- 프리셋 저장/재도전 유지, 우선 목표 규칙
- 치유량·최대 치유량 캐릭터 — combat 파이프라인에 회복 개념이 없어 보류 (healing slice 선행 필요; `combat_resolution.rs`의 체력 갱신은 감소 전용이고 `CombatAttackDefinition`/`CombatEffectDefinition`에 회복 필드가 없다)
- 명줄 소모·패배 결과 — 정본 10 기준 런 단위 메타 자원이며 인카운터 패배 결과 정의가 소유한다. 패배 결과 스키마 slice 선행 필요

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
| `fable_combat_wave3_step1c_2608021109.md` | `CombatSpectatorView.simulation_version`, `CombatSpectatorPage`, `ScenePage.combat` (additive-optional, 항상 `None`) | terminal/Web 렌더러, combat producer(authoring) |
| `fable_combat_wave3_step2a_2608021137.md` | `EncounterCombatDef` 스키마·index-time 검증·시스템형 producer(런 상태 파생 seed) | 실제 콘텐츠 authoring, 혼합형·각본형 흐름, 캐싱 |
| `fable_combat_wave3_step2b_2608021228.md` | 시스템형 인카운터 1개 authoring(`wuxia_combat_spectator_preview_bout`, 게이트 플래그 뒤) | 혼합형·각본형, 개입 일시정지 흐름, 밸런스 수치 확정 |
| (플랜 미작성) — Wave 3 Step 2c | 혼합형 1개 + 각본형 1개 authoring, 개입 일시정지 흐름 | 대규모 콘텐츠·보스 밸런스 |
| `fable_combat_wave3_step1d1_2608021329.md` | terminal(SuperLightTUI) 관전 렌더러: 로그 템플릿 표, 체스말 보드(cue 5종 표식), 핵심 로그, 전투 종료 보고서 | Web 렌더러, 게이트 플래그 제거, 판정·집계·cue 재계산 |
| `fable_combat_wave3_step1d2_2608021618.md` | Web Storybook 관전 표면(정지 프레임): TS 타입, 로그 템플릿 표, 체스말 보드+접근 대체 표, 핵심 로그, 전투 종료 보고서, 70:30 CSS | 재생 연출(모션), 게이트 플래그 제거, wasm 재빌드, 5뷰포트 실화면 QA, seed·판정·AI·로그 순서 재구현 |
| `fable_combat_wave3_step1d3_2608021755.md` | Web 재생 연출: 데이터 구동 `@keyframes` 생성(`combatMotion.ts`), 전체 프레임 투영 확장, cue 5종 연출 문법, 로그 tick 동기 노출 | 게이트 플래그 제거, wasm 재빌드, 5뷰포트 실화면 QA, 전투원 표시 이름, seed·판정·AI·로그 순서 재구현, 밸런스 확정값 |
| (플랜 미작성) — Wave 3 Step 1d-4 | 게이트 플래그 제거, wasm 재빌드, 5뷰포트 실화면 QA, 전투원 표시 이름(authoring 필드 추가 여부 결정) | seed·판정·AI·로그 순서 재구현, 밸런스 확정값, 정본 09 축 계약 위반 수정(별도 core 슬라이스) |

**단계 순서 조정 (Wave 3 Step 1c 완료 시점)**: `Step 2`를 `Step 1d` 앞으로 옮겼다 — 전투를 시작하는 인카운터 authoring(Step 2)이 없으면 `ScenePage.combat`의 producer가 없어 Step 1d 렌더러가 표시할 데이터가 없기 때문이다.

Wave 2 Step 4 구현 위치: `crates/escape-core/src/combat_conclusion.rs`, `crates/escape-core/tests/combat_conclusion_wave2.rs`.
Wave 3 Step 1a 구현 위치: `crates/escape-core/src/combat_spectator.rs`, `crates/escape-core/tests/combat_spectator_wave3.rs` (Step 1a 완료 당시 12 테스트, Wave 2 Step 5에서 19로 늘었다 — 현재 수치는 아래 Step 5 줄을 본다).
Wave 2 Step 5 구현 위치: `crates/escape-core/src/combat_resolution.rs`, `crates/escape-core/src/combat_spectator.rs`, `crates/escape-core/tests/combat_resolution_wave2.rs` (16 테스트), `crates/escape-core/tests/combat_spectator_wave3.rs` (Step 5 완료 당시 19 테스트 — Wave 3 Step 1c에서 20으로 늘었다, 아래 Step 1c 줄을 본다).
Wave 3 Step 1b 구현 위치: `crates/escape-core/src/combat_conclusion.rs`, `crates/escape-core/tests/combat_conclusion_wave2.rs` (14 테스트).
Wave 2 Step 6 구현 위치: `crates/escape-core/src/combat_execution.rs`, `crates/escape-core/src/combat_spectator.rs`, `crates/escape-core/src/combat_conclusion.rs`, `crates/escape-core/tests/combat_execution_wave2.rs` (10 테스트), `crates/escape-core/tests/combat_spectator_wave3.rs` (19 테스트, 개수 유지), `crates/escape-core/tests/combat_conclusion_wave2.rs` (14 테스트, 개수 유지). `CombatSpectatorRequest.tick_millis`/`CombatConclusionRequest.tick_millis` 필드와 두 `InvalidTickMillis` 에러 변형을 제거하고 `MissingProvenance`로 대체했다.
Wave 3 Step 1c 구현 위치: `crates/escape-core/src/combat_spectator.rs`, `crates/escape-core/src/scene_page.rs`, `crates/escape-core/src/lib.rs`, `crates/escape-core/tests/combat_spectator_wave3.rs` (20 테스트, `view_reports_the_simulation_version_from_provenance` 추가), `crates/escape-core/tests/scene_page_combat_boundary.rs` (신규, 5 테스트), `crates/escape-wasm/tests/json_contract.rs` (기존 테스트 무수정, `json_boundary_scene_page_has_no_combat_key_before_combat_authoring_exists` 1개 추가로 37 테스트).
Wave 3 Step 2a 구현 위치: `crates/escape-core/src/content.rs`(`EncounterCombatKind`/`EncounterCombatDef`/`EncounterDef.combat`/`RawEncounterDef.combat`/`ContentIndexError::InvalidEncounterCombat`/`validate_encounter_combat`), `crates/escape-core/src/scene_page.rs`(`combat_spectator_page_for_encounter`/`derive_combat_seed`/`combat_producer_error`), `crates/escape-core/src/lib.rs`(re-export), `crates/escape-core/tests/encounter_combat_wave3.rs` (신규, 21 테스트: 검증 규칙 11개 - `rule1_intervention_budget_over_three_is_rejected` ~ `rule11_attack_references_unknown_effect_id_is_rejected`, producer 결정론·seed 파생 4개 - `systemic_combat_producer_is_deterministic_for_the_same_state`/`systemic_combat_producer_seed_changes_with_run_seed`/`systemic_combat_producer_result_is_independent_of_authoring_actual_seed`/`systemic_combat_producer_fills_scene_page_combat`, additive-optional·JSON 경계 5개). 기존 `scene_page_combat_boundary.rs`(5 테스트)·`content_bundle.rs`(9 테스트)·`event_stage.rs` 계열 테스트는 수정 없이 그대로 통과한다.
Wave 3 Step 2b 구현 위치: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`(`wuxia_combat_spectator_preview_bout` 신규 인카운터, source of truth), `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`·`web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`(export 재생성, 직접 편집 없음), `crates/escape-core/tests/encounter_combat_wave3.rs`(7 테스트 추가, 총 28 테스트: `spectator_preview_bout_is_unreachable_without_the_gate_flag`, `gate_flag_selects_the_bout_and_fills_scene_page_combat`, `report_covers_both_combatants_with_non_negative_damage_totals`, `wuxia_combat_spectator_preview_bout_first_hit_damage_is_1333_hundredths`, `gated_combat_is_deterministic_for_the_same_state`, `spectator_preview_bout_has_a_staged_event`). 카운트 단정만 갱신(로직 무수정): `crates/escape-core/tests/event_stage_wave3.rs`(`wuxia_preview_has_full_51_event_coverage`→`wuxia_preview_has_full_52_event_coverage`, 51→52), `crates/escape-core/tests/content_bundle.rs`(`preview_fixture_indexes_wuxia_first_fight`의 51→52 두 곳), `crates/escape-core/tests/reward_pipeline_wave1.rs`(`wave1_adds_seven_staged_cards_and_all_29_mapping_rows`의 51→52), `tests/test_web_data_export.py`(51→52 두 곳). `web/src/core/contentBundles.test.ts`(웹 renderer 소유 디렉터리, 이 slice의 절대 규칙상 수정 금지)는 새 encounter id가 목록에 없다는 고정 리스트 단정 때문에 1개 테스트가 깨진 채로 남아 있다 — 알려진 이탈 항목이며 보고서에 기록했다.

Wave 3 Step 1d-1 구현 위치: `crates/escape-terminal/src/snapshot.rs`(로그 템플릿 표·체스말 보드·핵심 로그·전투 종료 보고서 렌더 + `#[cfg(test)] mod tests`, 24 테스트: `template_move_intent_renders`~`template_unknown_id_falls_back_and_exposes_id`(템플릿 6종+fallback), `round_hundredths_rounds_half_up_both_signs`, `board_renders_last_frame_with_tick_and_elapsed_time`·`board_exceeding_caps_falls_back_to_coordinate_list`·`board_shows_all_five_cue_symbols`·`board_handles_empty_pieces_without_panic`·`board_handles_no_frames_without_panic`, `core_log_shows_full_log_count_only`·`core_log_truncates_and_states_omitted_count`, `report_hides_highlight_lines_when_none`·`report_shows_highlight_lines_when_some`·`report_lists_one_row_per_combatant`·`report_contains_no_forbidden_phrases`·`report_fingerprint_shares_line_with_simulation_version`, `combat_section_adds_nothing_when_combat_is_none`·`scene_snapshot_unchanged_bytes_when_combat_is_none`·`scene_snapshot_includes_combat_sections_in_order_when_present`·`scene_snapshot_omits_report_section_when_combat_in_progress`), `docs/dev/TUI_Layout.md`(terminal 관전 표시 계약 절 추가). `crates/escape-terminal/tests/cli_smoke.rs`는 수정하지 않았고 기존 61개 테스트가 그대로 통과한다. `cargo test --workspace --no-fail-fast`: 346 테스트(322 + 24), 0 failed.
Wave 3 Step 1d-2 구현 위치: `web/src/core/types.ts`(`CombatSpectatorPage`/`CombatSpectatorView`/`CombatSpectatorFrame`/`CombatSpectatorPiece`/`CombatSpectatorLogEntry`/`CombatCombatantReport`/`CombatConclusionReport`, `ScenePage.combat?`), `web/src/ui/storybook/combat/combatLogTemplates.ts`(+`combatLogTemplates.test.ts`, 11 테스트: 6개 template id 문장 + fallback + null 값 마커 + 반올림), `web/src/ui/storybook/combat/renderCombatStage.ts`(+`renderCombatStage.test.ts`, 22 테스트: 보드 9 - `renders only the last frame`·`centers pieces at 50% when the coordinate span is zero`·`shows all 5 cue symbols`·`handles an empty frames array`·`handles a frame with zero pieces`·`includes every piece id...in the semantic alternative table`·`escapes a piece id containing markup`, 로그/보고서/통합 13 - `sentences only core_log entries`·`truncates core_log at 40 rows and states the omitted count`·`marks the damage_applied row with data-cue="hit"`·`hides top_damage_dealt_id / top_damage_taken_id lines`·`hides the decisive_tick line when null`·`puts the fingerprint and simulation_version in the same element`·`never contains forbidden strategic-analysis phrases`·`omits the combat-report section entirely when report is absent` 등), `web/src/ui/storybook/render.ts`(`renderStorybookPage`에 `renderCombatStage(page.combat)` 삽입), `web/src/ui/storybook/render.test.ts`(통합 2건: `mounts the combat spectator surface only when page.combat is present`·`I5: emits no combat markup at all when page.combat is absent`), `web/src/styles/storybook.css`(`.combat-stage` 70:30 그리드·체스 폰 실루엣·cue 색 토큰·forced-colors 대체, 신규 색상 리터럴 0개·`transition`/`animation` 0개). `cd web && npm test`: 15 파일 108 테스트(이 슬라이스 이전 73 + 신규 35), 0 failed. `cargo test --workspace --no-fail-fast`: Rust 무변경이므로 346 테스트 그대로(WP1~WP7 전부 `web/`·`docs/`만 수정, `crates/` 무변경 확인용으로 재실행함).
Wave 3 Step 1d-3 구현 위치: `web/src/ui/storybook/combat/combatMotion.ts`(신규, `buildCombatMotionCss`/`keyframeNameForPiece`, +`combatMotion.test.ts` 29 테스트: I1 총 길이·오프셋·frames<=1 5건, I5 결정론 1건, I3 media wrap 1건, I9 translate/opacity/filter만 1건, `<style>` breakout 안전장치 2건, keyframe 이름 안전성 3건, WP3 cue 문법 12건), `web/src/ui/storybook/combat/renderCombatStage.ts`(투영 범위를 마지막 프레임에서 전체 프레임으로 확장, `<style>` 방출, 로그 `animation-delay`, +`renderCombatStage.test.ts` 22→33 테스트: WP2 5건·WP3 e2e 1건·WP4 3건 추가), `web/src/styles/storybook.css`(`.combat-stage__board`에 `container-type: size`, `.combat-log__row` tick 동기 reveal `@keyframes`, 신규 색상 리터럴 0개). `cd web && npm test`: 16 파일 148 테스트(이 슬라이스 이전 108 + 신규 40), 0 failed. `cargo test --workspace --no-fail-fast`: `crates/` 무변경이므로 그대로.

계획 문서 대비 알려진 이탈(모두 보고 완료): (1) §3 I9의 허용 속성 목록에 `rotate`가 빠져 있었다 — 정본 13은 균형 붕괴를 "흔들림/기울어짐"으로 정의하고 `rotate`도 `translate`와 같은 개별 transform 속성이라 컴포지터에서 처리되므로, 오케스트레이터가 I9를 정본에 맞춰 `rotate` 허용으로 바로잡고 기울어짐을 구현했다(`combatMotion.test.ts`의 `tilts by alternating degrees while the cue is present`·`emits no rotate at all when no frame carries balance_broken`). subagent는 원래 I9를 문자대로 따라 흔들림만 구현했고 그 판단 자체는 절차상 옳았다. (2) §4-2가 예시로 든 `--dx`/`--dy` CSS 커스텀 프로퍼티 + `var()` 간접 참조 대신, 매 keyframe에 절대 `translate` 값을 직접 굽는 방식을 썼다 — 애니메이션되는 커스텀 프로퍼티를 부드럽게 보간하려면 `@property` 등록이 필요한데(계획에 언급 없음) 실제 `translate` 애니메이션은 모든 브라우저가 네이티브로 보간하므로 더 안전한 선택이다; `cqw`/`cqh` 단위 요구사항(rule 7)과 모든 Hard invariant는 그대로 만족한다. (3) I11은 "`crates/` 예외: WP5의 authoring 테스트 1개"라고 적었지만 §WP5 본문은 "`crates/` 무변경"·"테스트를 추가하지 않는다"라고 명시한다 — 더 상세하고 구체적인 §WP5 본문과 "Allowed/forbidden edits"의 무예외 forbidden 목록을 따라 `crates/`를 전혀 건드리지 않았다(축 계약 위반은 문서에만 기록, 위 참고).

Wave 3 Step 1d-3 오케스트레이터 리뷰에서 고친 것(모두 실화면 계측에서 나왔다):

- **보드와 로그의 시간 원점이 한 tick 어긋나 있었다.** 보드는 프레임 인덱스 k를
  `k × tick_millis`에 놓으므로 첫 프레임(실측 데이터의 첫 tick은 0이 아니라 1)이
  0ms인데, 로그는 `entry.tick × tick_millis`를 써서 같은 사건이 보드보다 100ms
  늦게 나타났다. 정본 13의 "상단 연출과 하단 로그 동기화" 위반이다. 원점을 빼서
  맞췄다(`renderCombatStage.test.ts`의 `anchors log reveal to the first frame tick
  so the board and the log share one origin`·`never produces a negative reveal
  delay when a log tick precedes the first frame`). 계측 확인: 재생 900ms, 로그
  지연 0~900ms 100ms 간격.
- **전투불능 감광이 애니메이션 안에만 있었다.** `prefers-reduced-motion: reduce`
  에서 마지막 프레임이 전투불능인데도 말이 멀쩡하게 보여 재생 경로와 정지 경로의
  그림이 달라졌다. `[data-cue-incapacitated="true"]` 정적 규칙으로 올려 두 경로를
  일치시켰다.
- **감광 속성을 매 stop에 내보내 정적 감광을 덮어썼다.** `opacity: 1`이 재생
  내내 `[data-active="false"]`의 감광을 지웠다. `incapacitated` cue가 한 번이라도
  있는 트랙에서만 감광 속성을 내보낸다(`omits opacity/filter entirely when no
  frame carries incapacitated`).
- **`.combat-log__row { opacity: 0 }` 기본값을 제거했다.** `animation-fill-mode:
  both`가 delay 구간에 이미 `from` 상태를 적용하므로 불필요하고, 기본값으로 두면
  애니메이션이 돌지 않을 때 로그가 영구히 보이지 않는 실패 모드가 된다.
- **속성 선택자 값의 개행 이스케이프.** CSS 문자열에 raw 개행이 들어가면 그 규칙과
  뒤따르는 규칙까지 파싱이 깨진다. `\A ` 코드포인트 이스케이프로 바꿨다.

실화면 계측(Playwright, `reducedMotion` 컨텍스트 옵션, 320/390/1280):
`no-preference`에서 애니메이션 42개(말 2개 × 900ms linear both + 로그 40행),
`reduce`에서 **애니메이션 0개**이고 로그 40행이 전부 즉시 보이며 말은 마지막
프레임 위치(38%/62%)에 있다. 보드:로그 = 0.700, 가로 스크롤 0, 재생 전 구간에서
말이 보드 경계에서 잘리지 않는다(최소 여백 32px). 재생 종료 상태와 `reduce`
정지 상태의 말 위치가 일치한다.

재생에서 정본 09 위반이 눈으로 보인다: 두 말이 중간 시점에 거의 겹쳐 지나가며
아군이 도전자 오른쪽으로 넘어간다. 렌더러 결함이 아니라 AI·충돌 규칙 쪽 결함이며
위 core 결함 블록에 기록했다.

각 단계는 선행 단계의 public contract와 테스트만 사용한다. 단계 사이에 새 필드가 필요하면 먼저 해당 단계 plan을 갱신하고, 기존 저장/JSON backward compatibility를 검토한다.

## 구현 운영 규칙

1. 오케스트레이터가 해당 단계 plan을 먼저 확정한다.
2. `coding_implementer`는 plan의 소유 파일만 수정한다. 다른 작업자의 변경이나 미추적 `.claude/worktrees/`를 건드리지 않는다.
3. subagent 보고와 별개로 오케스트레이터가 WSL에서 핵심 테스트를 재실행한다.
4. 정확한 기술 비용·회복률·상태 계수·직업별 수치는 데이터 조정 항목으로 남기고 코드 상수로 임의 확정하지 않는다.
5. 렌더러는 Rust core가 만든 결과를 표시만 한다. seed, 판정, AI, 로그 순서를 Web/terminal에서 재구현하지 않는다.

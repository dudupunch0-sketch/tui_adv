# Wave 3 Step 2a — 구현 보고서

작성: 2026-08-02
플랜: `fable_combat_wave3_step2a_2608021137.md`
기준 브랜치: `claude/combat-wave3-step1a-v2`
slice 시작 커밋: `46ddd54` (Baseline: `cargo test --workspace --no-fail-fast` → 294 passed / 0 failed)

## 요약

WP-1~WP-5 모두 완료. `EncounterCombatDef` 스키마(additive-optional), 11개 index-time
하드 오류 규칙, 시스템형(`kind: systemic`) 전용 combat producer를 추가했다. 실제 전투
seed는 authoring이 아니라 런 상태(`GameState.seed`) + 인카운터 id + manifest 내용에서
결정론적으로 파생하며, 새 난수원은 도입하지 않았다. 기존 294개 테스트는 전부 그대로
통과하고, 신규 21개 테스트(`encounter_combat_wave3.rs`)가 더해져 최종
`cargo test --workspace --no-fail-fast` = **315 passed / 0 failed**.

## WP별 변경 파일 · 내용 · 검증

### WP-1 — 스키마 (커밋 `97faabf`)

파일: `crates/escape-core/src/content.rs`, `crates/escape-core/src/lib.rs`

- `EncounterCombatKind`(`systemic`/`mixed`/`scripted`, `#[serde(rename_all = "snake_case")]`)
- `EncounterCombatDef` — 플랜의 공개 API 그대로: `kind`, `intervention_budget: u8`,
  `manifest: CombatManifest`, `state: CombatState`, `config: CombatSimulationConfig`,
  `participants: Vec<CombatSimulationParticipant>`, `roles: Vec<CombatRolePreset>`,
  `policies: Vec<CombatTargetPolicy>`(`#[serde(default)]`), `attacks: Vec<CombatAttackDefinition>`,
  `defenses: Vec<CombatDefenseProfile>`, `effect_catalog: CombatEffectCatalog`, `ticks: u32`,
  `termination: CombatTerminationPolicy`. **seed 필드 없음.**
- `EncounterDef.combat: Option<EncounterCombatDef>`, `RawEncounterDef.combat`도
  `#[serde(default)]`로 추가하고 `parse_encounter`에서 그대로 옮긴다.
- `lib.rs`에 `EncounterCombatDef`, `EncounterCombatKind` re-export 추가.
- 검증 로직·producer는 아직 없음(이 WP는 스키마만).

검증:
```
cargo build -p escape-core            → 성공 (경고 없음)
cargo fmt --all -- --check            → 통과
cargo test --workspace --no-fail-fast → 294 passed; 0 failed
```
**WP-1 직후 294 유지 확인: 통과.** (14+5+10+12+16+10+20+8+9+32+11+3+3+3+8+4+23+5+61+37 = 294)

### WP-2 — index-time 검증 (커밋 `64659ff`)

파일: `crates/escape-core/src/content.rs`, `crates/escape-core/tests/encounter_combat_wave3.rs`(신규)

`ContentIndexError::InvalidEncounterCombat { encounter_id, message }`를 추가하고(기존
`InvalidEvent`와 같은 스타일), `validate_encounter_combat`이 `index_content_bundle`의
encounter 루프에서 `validate_encounter_insights` 다음에 호출된다. 11개 규칙 전부 구현:

1. `intervention_budget > 3`
2. `kind`가 `mixed`/`scripted` — 메시지에 인카운터 id + "Wave 3 Step 2b/2c 소관" 포함
3. `config.tick_millis == 0`
4. `ticks == 0` 또는 `ticks > config.max_ticks`
5. `attacks[].actor_id`가 `state.combatants`에 없음
6. `defenses[].combatant_id`가 `state.combatants`에 없음
7. `state.combatants`에 있는데 `defenses`에 짝이 없음
8. `participants` id 집합 ≠ `state.combatants` id 집합
9. `effect_catalog.validate()` 실패
10. `manifest.validate()` 실패
11. `attacks[].effects[].effect_id`가 `effect_catalog`에 없음

**red 캡처 (구현 전, `cargo test -p escape-core --test encounter_combat_wave3` 원문 발췌):**
```
running 20 tests
test rule2_mixed_kind_is_rejected_and_names_the_encounter ... FAILED
test rule10_invalid_manifest_is_rejected ... FAILED
test rule3_zero_tick_millis_is_rejected ... FAILED
test encounter_without_combat_still_yields_no_combat_key_in_json ... ok
test bundle_without_any_combat_field_still_indexes ... ok
test rule4_ticks_exceeding_max_ticks_is_rejected ... FAILED
test rule11_attack_references_unknown_effect_id_is_rejected ... FAILED
test rule2_scripted_kind_is_rejected_and_names_the_encounter ... FAILED
test rule4_zero_ticks_is_rejected ... FAILED
test rule5_attack_actor_id_not_in_combatants_is_rejected ... FAILED
test rule6_defense_combatant_id_not_in_combatants_is_rejected ... FAILED
test rule8_participant_id_set_mismatch_is_rejected ... FAILED
test systemic_combat_producer_fills_scene_page_combat ... FAILED
test systemic_combat_producer_is_deterministic_for_the_same_state ... ok
test rule7_combatant_missing_a_defense_profile_is_rejected ... FAILED
test systemic_combat_producer_seed_changes_with_run_seed ... FAILED
test rule9_invalid_effect_catalog_is_rejected ... FAILED
test valid_systemic_combat_indexes_without_error ... ok
test systemic_combat_producer_result_is_independent_of_authoring_actual_seed ... FAILED

---- rule1_intervention_budget_over_three_is_rejected stdout ----
thread 'rule1_intervention_budget_over_three_is_rejected' panicked at
crates/escape-core/tests/encounter_combat_wave3.rs:156:18:
expected combat validation to reject this bundle

test result: FAILED. 4 passed; 16 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```
(구현 전 단계에서는 `ContentIndexError::InvalidEncounterCombat` 변형 자체가 없어, WP-2 구현이
붙기 전에는 rule1 테스트의 정확 variant 검사(`matches!`)를 `error.to_string()` 검사만 남긴
버전으로 임시 완화한 뒤 위 red를 캡처했다 — variant가 생기자마자 원래 검사로 복원했다.)

구현 후 (`cargo test -p escape-core --test encounter_combat_wave3`):
```
test result: ok. 17 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```
11개 규칙 관련 13개 테스트 전부 green. 남은 3개 실패는 producer 미구현(WP-3 소관)이다.
`cargo test --workspace --no-fail-fast`는 이 시점에도 기존 테스트 전부 그대로 통과(WP-2
전용 3개 producer 테스트만 예상대로 red).

### WP-3 — 시스템형 producer (커밋 `a6c6255`)

파일: `crates/escape-core/src/scene_page.rs`

- `scene_page_from_content`/(private) `scene_page_from_turn_view`가 `Result<ScenePage, ContentTurnError>`를
  낸다(기존 `scene_page_from_turn_view`는 평범한 `ScenePage`를 반환했으나 이제 fallible).
- `combat_spectator_page_for_encounter(state, encounter)`: `encounter.combat`이
  `Some`이고 `kind == Systemic`이면 `resolve_combat`(내부에서 `execute_combat` 호출) →
  `conclude_combat` → `spectate_combat` 순으로 실제 파이프라인을 돌려
  `CombatSpectatorPage { view, report: Some(report) }`를 만든다. `Mixed`/`Scripted`가
  방어적으로 도달하면(index-time에서 이미 거부되므로 정상 흐름에선 발생하지 않는다)
  오류를 낸다. 파이프라인 어느 단계가 실패해도 `ScenePage.combat`을 `None`으로 두지
  않고 오류로 전파한다(invariant 3).
- `derive_combat_seed(run_seed, encounter_id, manifest)`: manifest를 복제해
  `actual_seed`를 `fnv1a_64("{run_seed}:{encounter_id}")`로 먼저 덮어써 authoring 값을
  완전히 버린 뒤, 기존 `CombatManifest::derived_seed(CombatRngNamespace::ActualCombat)`
  FNV 파이프라인을 그대로 호출한다(새 함수를 새로 만들지 않고 재사용). 결과를 최종
  `manifest.actual_seed`와 `CombatSimulationInput.seed` 양쪽에 넣어 manifest와 실제
  seed가 어긋나지 않게 한다.
- `combat_producer_error(encounter_id, message)`: **의도적 범위 트레이드오프.**
  `ContentTurnError`(정의 위치 `crates/escape-core/src/turn.rs`)는 이 slice의 예상 변경
  파일 표 밖이라 새 variant를 추가하지 않았다. 기존 유일한 variant
  `UnknownStateLocation(String)`에 `"combat producer failed for encounter '{id}': {message}"`
  형태의 메시지를 담아 재사용한다. 실제 실패 사유(인카운터 id + 원인)는 메시지에 그대로
  남는다.

**red (WP-2 red 캡처와 동시에 기록됨, 위 로그의 3개 producer 테스트):**
```
---- systemic_combat_producer_fills_scene_page_combat stdout ----
thread 'systemic_combat_producer_fills_scene_page_combat' panicked at
crates/escape-core/tests/encounter_combat_wave3.rs:296:30:
systemic combat should fill ScenePage.combat

---- systemic_combat_producer_seed_changes_with_run_seed stdout ----
thread 'systemic_combat_producer_seed_changes_with_run_seed' panicked at
crates/escape-core/tests/encounter_combat_wave3.rs:332:39:
called `Option::unwrap()` on a `None` value

---- systemic_combat_producer_result_is_independent_of_authoring_actual_seed stdout ----
thread 'systemic_combat_producer_result_is_independent_of_authoring_actual_seed' panicked at
crates/escape-core/tests/encounter_combat_wave3.rs:360:23:
called `Option::unwrap()` on a `None` value
```

구현 후: `cargo test -p escape-core --test encounter_combat_wave3` → **20 passed; 0 failed**
(WP-2의 17 + 3개 producer 테스트 green).

### WP-4 — 회귀 테스트 확인 (커밋 `85afe67`)

파일: `crates/escape-core/tests/encounter_combat_wave3.rs`

플랜의 13개 최소 케이스는 WP-2/WP-3 사이에 이미 전부 작성돼 있었다(각 규칙/producer red→green
과정에서 필요했기 때문). WP-4에서는 이를 재확인하고, 실제 producer가 만든 `ScenePage`가
`scene_page_combat_boundary.rs`의 synthetic `CombatSpectatorPage`가 아니라 진짜 producer
결과로 serde round-trip을 통과하는지 검증하는 테스트를 1개 추가했다:
`systemic_combat_scene_page_round_trips_through_serde`.

최종 `cargo test -p escape-core --test encounter_combat_wave3` → **21 passed; 0 failed.**

13개 최소 케이스 ↔ 테스트 매핑:

| # | 케이스 | 테스트 |
|---|---|---|
| 1 | combat 없음 → `None` + JSON에 키 없음 | `encounter_without_combat_still_yields_no_combat_key_in_json` |
| 2 | 시스템형 주입 → `Some` + frames 비어있지 않음 | `systemic_combat_producer_fills_scene_page_combat` |
| 3 | report `Some`, `duration_millis > 0`, combatants 수 일치 | 위와 동일 테스트 |
| 4 | 같은 상태 두 번 호출 → 완전 동일 | `systemic_combat_producer_is_deterministic_for_the_same_state` |
| 5 | 런 seed 다르면 fingerprint 다름 | `systemic_combat_producer_seed_changes_with_run_seed` |
| 6 | authoring `actual_seed` 무관성 | `systemic_combat_producer_result_is_independent_of_authoring_actual_seed` |
| 7 | `intervention_budget = 4` → 오류 | `rule1_intervention_budget_over_three_is_rejected` |
| 8 | `kind = mixed` → 오류(인카운터 id 포함) | `rule2_mixed_kind_is_rejected_and_names_the_encounter` |
| 9 | `kind = scripted` → 오류 | `rule2_scripted_kind_is_rejected_and_names_the_encounter` |
| 10 | 없는 `actor_id` → 오류 | `rule5_attack_actor_id_not_in_combatants_is_rejected` |
| 11 | 없는 effect id → 오류 | `rule11_attack_references_unknown_effect_id_is_rejected` |
| 12 | `ticks > max_ticks` → 오류 | `rule4_ticks_exceeding_max_ticks_is_rejected` |
| 13 | combat 없는 기존 bundle 그대로 인덱싱 | `bundle_without_any_combat_field_still_indexes` |

### WP-5 — 문서 (커밋 `5d6c81b`)

- `docs/dev/Data_Schema.md`: "인카운터 전투 스키마 (`EncounterCombatDef`, Wave 3 Step 2a)"
  절 신설(필드 표, seed를 authoring에 두지 않는 이유, 11개 검증 규칙, 현재 systemic-only
  지원 상태). `ScenePage.combat` 절의 stale한 "producer가 없다" 문장도 갱신.
- `docs/design/Combat_System_Implementation_Plan_Index.md`: `status: wave3-step2a-complete`,
  `(플랜 미작성) — Wave 3 Step 2` 행을 2a(이 플랜)/2b/2c로 분할, "현재 코드와 정본의 경계"
  단락과 gap bullet 갱신(혼합형·각본형 개입 일시정지 흐름, 실제 콘텐츠 authoring, 결과
  캐싱을 남은 gap으로 명시).
- `docs/dev/Combat_System_Operating_Guide.md`, `docs/dev/Combat_System_Goal_Prompt.md`:
  각각 Step 2a 완료 항목 한 줄(+ 상세 단락)과 "아직 열지 않음"/미구현 목록 갱신.

문서 크기(수정 후, `wc -c`):
```
 11602 docs/dev/Combat_System_Goal_Prompt.md
 41729 docs/dev/Data_Schema.md
 16200 docs/design/Combat_System_Implementation_Plan_Index.md
 20424 docs/dev/Combat_System_Operating_Guide.md
```
전부 100KB 미만.

## seed 파생 구현 방식

1. authoring `EncounterCombatDef.manifest`에는 seed가 없다 — `CombatManifest.actual_seed`
   필드는 있지만 producer가 그 값을 신뢰하지 않는다.
2. producer가 `manifest`를 복제하고, `actual_seed`를 `fnv1a_64(format!("{run_seed}:{encounter_id}"))`
   (기존 `combat_contract.rs`/`combat_execution.rs`/`combat_resolution.rs` 등이 이미 쓰는
   FNV-1a 64bit 알고리즘과 동일)로 먼저 덮어쓴다. 이 시점에 authoring이 넣은 원래
   `actual_seed`는 완전히 버려진다.
3. 이 임시 manifest에 대해 기존 `CombatManifest::derived_seed(CombatRngNamespace::ActualCombat)`를
   호출한다 — 이 메서드는 `simulation_version`·namespace·(방금 덮어쓴) `actual_seed`·
   manifest의 canonical fingerprint(월드 상태 fingerprint, 전투원/배치/환경/팀/규칙/
   public info id 목록 등)를 함께 해싱한다.
4. 그 결과값을 최종 `manifest.actual_seed`와 `CombatSimulationInput.seed`에 똑같이
   넣는다 — `CombatRunMode::Actual`은 `input.seed`를 그대로 `effective_seed`로 쓰므로
   (`combat_execution.rs::execute`), manifest와 실제 로 쓰이는 seed가 어긋나지 않는다.
5. 새 난수원(`rand` 크레이트 등)은 도입하지 않았다 — 기존 FNV 해시 기반 결정론적
   "난수" 관례를 그대로 재사용했다.
6. 증명: `systemic_combat_producer_result_is_independent_of_authoring_actual_seed`가
   authoring `actual_seed`를 1과 999999로 바꿔도 최종 `view.fingerprint`가 동일함을
   검증한다. `systemic_combat_producer_seed_changes_with_run_seed`는 런 seed(1 vs 2)가
   다르면 fingerprint가 달라짐을 검증한다.

## 매 렌더 전투 재실행 비용

`combat_spectator_page_for_encounter`는 `scene_page_from_content`가 호출될 때마다 전투
시뮬레이션 전체(`resolve_combat`이 내부에서 부르는 `execute_combat` 포함)를 처음부터
다시 돈다. 결과는 결정론적이라 값 자체는 항상 같지만, 렌더 한 번마다 계산이 낭비된다.
이 slice에서는 의도적으로 그대로 두었다 — 인위적 캐시를 만들면 캐시 무효화·저장
시점의 정확성 문제가 생기고, 전투 결과를 어디에(어떤 save schema 필드로) 캐싱할지는
별도 결정이 필요하기 때문이다. **캐싱은 후속 slice 과제로 남긴다.**

## 스킵/이탈 항목과 사유

1. **`ContentTurnError`에 전용 variant를 추가하지 않음.** 정의 위치
   `crates/escape-core/src/turn.rs`가 이 slice의 예상 변경 파일 표 밖이라, 기존
   `UnknownStateLocation(String)` payload를 재사용해 인카운터 id + 실패 사유를 문자열로
   담았다. 시맨틱하게는 variant 이름이 정확하지 않지만("unknown state location"이라는
   접두 문구가 최종 메시지에 남는다), 구조적으로는 `ContentTurnError`로 정상 전파되고
   메시지에 필요한 정보가 다 들어 있다. 필요하면 후속 slice에서 `turn.rs`를 손대는
   plan을 별도로 잡아 전용 variant로 교체할 수 있다.
2. **WP-2/WP-3 red를 같은 커밋 이전 시점에 한 번에 캡처.** 두 WP의 테스트가 같은 신규
   파일(`encounter_combat_wave3.rs`)에 함께 있어, red 캡처도 한 번의 실행(20개 중 16개
   실패)으로 WP-2(13개)·WP-3(3개) 몫이 동시에 나왔다. WP-2 구현 후 재실행해 WP-3의 3개만
   남은 red임을 별도로 재확인했다(위 WP-2 절의 "17 passed; 3 failed").
3. **plan 파일 자체(`fable_combat_wave3_step2a_2608021137.md`)는 git에 add하지 않음.**
   예상 변경 파일 표에 없고, orchestrator가 이미 별도 경로로 관리 중인 파일이라
   판단했다. WSL 저장소에는 그대로 남아 있다(untracked).
4. **`.claude/worktrees/caveman-repo-sync-8a6b94/`, `crates/escape-terminal/tests/cli_smoke.rs`**:
   지시대로 읽지도 쓰지도 않았다 — `cli_smoke.rs`는 다른 작업자가 이미 수정한 상태 그대로
   두었다.

## 최종 체크리스트

- [x] `EncounterCombatDef`에 seed 필드가 없다
- [x] 전투 seed가 런 상태 + 인카운터 id + manifest fingerprint에서 파생된다
- [x] 새 난수원(`rand` 등)을 도입하지 않았다
- [x] `Mixed`/`Scripted`가 조용히 무시되지 않고 명시적 오류다
- [x] `intervention_budget > 3`이 오류다
- [x] 전투 없는 인카운터의 JSON에 `combat` 키가 없다
- [x] fixture/generated bundle JSON 무변경 (테스트는 주입 방식)
- [x] `crates/escape-terminal/`·`web/src/` 무변경
- [x] `HashMap` 순회 의존 없음 (모두 `BTreeMap`/`BTreeSet`/정렬 `Vec`)
- [x] WP-2·WP-3 red 출력 기록 (위 절)
- [x] `cargo fmt --all -- --check`, `git diff --check` 통과
- [x] `cargo test --workspace --no-fail-fast` 0 failed (315 passed)
- [x] WP-5 문서 4개 갱신, Step 2가 2a/2b/2c로 분할됨
- [x] 보고서에 "매 렌더 전투 재실행 비용, 캐싱은 후속" 기록
- [x] `cli_smoke.rs`·`.claude/worktrees/` 무변경
- [x] 보고서 `fable_combat_wave3_step2a_report.md` 작성

## 최종 검증 명령 실행 결과

```
cargo fmt --all -- --check                              → 통과
cargo test -p escape-core --test encounter_combat_wave3 → 21 passed; 0 failed
cargo test -p escape-core --test scene_page_combat_boundary → 5 passed; 0 failed
cargo test -p escape-core --test content_bundle         → 9 passed; 0 failed
cargo test -p escape-core --test event_stage             → (workspace 실행에 포함, 통과)
cargo test -p escape-wasm --test json_contract           → 37 passed; 0 failed
cargo test --workspace --no-fail-fast                    → 315 passed; 0 failed
git diff --check                                         → 통과
```

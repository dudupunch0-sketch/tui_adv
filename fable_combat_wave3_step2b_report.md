# Combat Wave 3 Step 2b — 구현 보고서

작성: 2026-08-02 (coding subagent)
플랜: `fable_combat_wave3_step2b_2608021228.md`
브랜치: `claude/combat-wave3-step2b`

## 요약

WP-1~WP-4를 순서대로 완료했다. `wuxia_combat_spectator_preview_bout` 시스템형 전투
인카운터 1개를 이구학지 preview에 authoring했고, Step 2a producer가 이를 실제로
구동해 `ScenePage.combat`을 채우는 것을 회귀 테스트로 고정했다. `cargo test
--workspace --no-fail-fast` 322 passed / 0 failed, pytest 98 passed, npm test는
73개 중 72 passed / 1 failed(아래 "스킵/이탈 항목" 참고, 절대 규칙상 수정 금지
디렉터리의 기존 테스트).

## WP별 변경 파일과 내용

### WP-1 — YAML authoring (commit `720ebed`)

- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml` (+220줄)
  - 인카운터 `wuxia_combat_spectator_preview_bout` 신규 추가, 위치
    `cheongryu_outer_courtyard`.
  - `conditions.required_flags: [combat_spectator_preview_unlocked]`,
    `forbidden_flags: [combat_spectator_preview_bout_resolved]`, `repeatable: false`.
  - staged `event`: story 1개(illustration placeholder 포함) → choice 2개
    (`watch_the_bout_closely`, `keep_a_measured_distance`) → 각 choice의
    result stage. 서술은 승패를 단정하지 않는다("승패는 아직 말해지지 않았지만…").
  - `combat` 블록: `kind: systemic`, `intervention_budget: 0`, 전투원 2명 모두
    정본 11 표준 전투원 수치, 공격 1개씩(정본 11 §7 초반 템플릿), 효과 없음.
- 검증: 아래 "export --check 결과", `cargo test -p escape-core --test
  content_bundle` (WP-2 절에 포함해 보고, 51→52 갱신 후 통과).

### WP-2 — 번들 재생성 (commit `1e823e5`)

- `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
  (+288/-2, export `--write`로 재생성, 직접 편집 없음)
- `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
  (동일)
- 갱신한 카운트 단정 (로직 무수정, 숫자만 51→52):
  - `crates/escape-core/tests/content_bundle.rs` ::
    `preview_fixture_indexes_wuxia_first_fight` — `bundle.manifest.counts.get("encounters")`,
    `index.encounters_len()` 두 곳
  - `crates/escape-core/tests/event_stage_wave3.rs` ::
    `wuxia_preview_has_full_51_event_coverage` → **함수명 자체를**
    `wuxia_preview_has_full_52_event_coverage`로 변경(저장소 관례), 내부
    `index.encounters_len()`/`.filter(|e| e.event.is_some()).count()` 두 단정
    51→52
  - `crates/escape-core/tests/reward_pipeline_wave1.rs` ::
    `wave1_adds_seven_staged_cards_and_all_29_mapping_rows` —
    `index.encounters_len()` 1곳
  - `tests/test_web_data_export.py` ::
    `test_export_web_data_builds_wuxia_storypack_preview_bundle` —
    `bundle["manifest"]["counts"]["encounters"]`, `len(encounter_ids)` 두 곳

### WP-3 — producer 회귀 테스트 (commit `442cdaa`)

- `crates/escape-core/tests/encounter_combat_wave3.rs` (+178/-1, 총 21→28 테스트)
  - 신규 상수: `WUXIA_BUNDLE`(실제 wuxia preview 번들 include), `SPECTATOR_BOUT_ID`,
    `SPECTATOR_LOCATION_ID`, `SPECTATOR_GATE_FLAG`
  - `spectator_preview_bout_is_unreachable_without_the_gate_flag` — 게이트
    없이는 `turn_view`의 `encounter_id`가 이 인카운터가 아니고 `ScenePage.combat`도
    `None`
  - `gate_flag_selects_the_bout_and_fills_scene_page_combat` — 플래그를 세우면
    `encounter_id`가 일치하고 `combat.view.frames`가 비어있지 않으며
    `combat.report`가 `Some`
  - `report_covers_both_combatants_with_non_negative_damage_totals` —
    `report.combatants.len() == 2`, 모든 `damage_dealt_hundredths`/
    `damage_taken_hundredths >= 0`
  - `wuxia_combat_spectator_preview_bout_first_hit_damage_is_1333_hundredths`
    — `combat.view.full_log`에서 첫 `"combat.log.damage_applied"` 항목의
    `value_hundredths == Some(1333)` (핵심 검산, 아래 절 참고)
  - `gated_combat_is_deterministic_for_the_same_state` — 같은 상태 두 번 호출 시
    `ScenePage.combat` 완전 동일
  - `spectator_preview_bout_has_a_staged_event` — Story→Choice, illustration
    1개+비어있지 않은 alt
  - 기존 21개 테스트 본문은 무수정.

### WP-4 — 문서 갱신 (commit `6d971d4`)

- `docs/design/Combat_System_Implementation_Plan_Index.md`
  - `status: wave3-step2a-complete` → `wave3-step2b-complete`
  - 단계 표의 `(플랜 미작성) — Wave 3 Step 2b` 행 → `fable_combat_wave3_step2b_2608021228.md`
  - "현재 코드와 정본의 경계" 문단에 Step 2b 요약 추가, non-goal 목록에서
    "시스템형 1개 authoring"을 "완료"로 옮기고 게이트 승격 조건 명시
  - Step 2b 구현 위치 줄 추가(수정 파일·테스트 함수명·카운트 단정 갱신 목록 포함)
- `docs/dev/Development_Plan.md` 10번 항목에 Wave 3 Step 2a/2b 요약 반영
- `docs/dev/Combat_System_Operating_Guide.md` 3절 "완료" 목록에 Step 2b 블록 추가,
  "아직 열지 않음" 목록에서 "시스템형 1개" 항목 제거하고 게이트 승격 조건 항목 추가
- `docs/dev/Combat_System_Goal_Prompt.md` baseline 목록에 Step 2b 한 줄 추가

## Provisional 필드 목록 (정본 11에 없어 임시로 정한 값)

모두 YAML 주석으로 표시했다 (`src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`):

1. `state.combatants[].balance` / `maximum_balance` (양쪽 모두 100/100) — 균형
   최대치 정본 미정의, 생명력·호흡 스케일에 맞춤
2. `participants[].position`, `facing`, `speed_per_tick`, `collision_radius`,
   `attack_range`, `support_range` (양쪽 모두) — 이동 속도·충돌 반경·사거리
   정본 미정의, 첫 틱부터 충돌·사거리 안에 들어오는 최소값
3. `attacks[].attack_range` (양쪽 공격) — participants와 일관되게 provisional
4. `attacks[].collision_balance_hundredths` / `balance_power_hundredths`
   (양쪽 공격, 100씩) — 균형 피해 표준값 정본 미정의, 작은 값
5. `defenses[].balance_resistance_hundredths` (양쪽, 0) — 정본 미정의
6. `config.tick_millis` (100), `config.max_ticks` (10) — tick 길이·상한 정본 미정의
7. `ticks` (10, top-level), `termination.max_ticks` (10) — 8타 결착에 여유를
   둔 권장값

## 첫 명중 피해 1333 검산

정본 §8 공식과 `combat_resolution.rs::damage`가 일치함을 코드로 직접 확인했다.

- `pre = power_hundredths(4000) * 5 * ability_multiplier_hundredths(100) / 1200`
  `= 2,000,000 / 1200 = 1666` (정수 나눗셈)
- `effective = defense_hundredths(500) - penetration_hundredths(0) = 500`
- `reduction = 1666 * 500 / 2500 = 333` (정수 나눗셈)
- `damage = 1666 - 333 = 1333`

테스트 `wuxia_combat_spectator_preview_bout_first_hit_damage_is_1333_hundredths`가
`combat.view.full_log`의 첫 `"combat.log.damage_applied"` 로그 항목의
`value_hundredths == Some(1333)`을 단정하며 **PASS**했다.

## 실행한 검증 명령과 출력 요약

1. `python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle <core> --preview-bundle <web> --write`
   → `wrote storypack preview bundle to ...` × 2
2. 같은 명령 `--check` → `storypack preview bundle is up to date` × 2 (**PASS**)
3. `cargo fmt --all -- --check` → 최초 1회 diff 발생(신규 테스트 파일 포매팅),
   `cargo fmt --all` 적용 후 재확인 **PASS** (exit 0, 출력 없음)
4. `cargo test -p escape-core --test encounter_combat_wave3` → **28 passed; 0 failed**
5. `cargo test -p escape-core --test event_stage_wave3` → **8 passed; 0 failed**
6. `cargo test -p escape-core --test content_bundle` → **9 passed; 0 failed**
7. `cargo test --workspace --no-fail-fast` → **322 passed; 0 failed** (baseline
   316 → +6 net; encounter_combat_wave3 자체는 21→28, +7)
8. `git diff --check` → 출력 없음 (**PASS**, whitespace 오류 없음)
9. `./.venv/bin/python -m pytest tests/ -q` → **98 passed** (2회 실행, YAML
   변경 직후와 문서 변경 직후 모두 98 passed)
10. `cd web && npm test` → **72 passed, 1 failed** (아래 "스킵/이탈 항목" 참고)

## 스킵/이탈 항목과 사유

1. **`web/src/core/contentBundles.test.ts`의 `uses 이구학지 as the default Web
   runtime storypack` 1개 실패** — 이 테스트는 이구학지 전체 encounter id 51개를
   하드코딩한 리스트와 `toEqual`로 비교하는데, 새로 추가한
   `wuxia_combat_spectator_preview_bout`가 목록에 없어 실패한다. 플랜 문서는
   "web 테스트 카운트 단정이 깨지면 수치만 갱신"하라고 안내하지만, 오케스트레이터의
   절대 규칙 6번은 `web/src/core/`를 **수정 금지 디렉터리**로 명시하고
   (`crates/escape-terminal/`, `web/src/ui/`와 동급, 다른 작업자 소유 추정)
   위반 시 작업 실패로 규정한다. 두 지시가 충돌하는 지점이라 절대 규칙을
   우선해 이 파일을 손대지 않았다. 결과적으로 `npm test`는 73개 중 1개 실패
   상태로 남아 있다 — web/src/core 소유 작업자가 이 리스트 단정에 새 id를
   추가하거나, 게이트 인카운터를 리스트에서 의도적으로 제외할지 결정해야 한다.
2. **5뷰포트 시각 QA, wasm 재빌드** — 플랜의 명시적 범위 밖 판단을 그대로
   따른다: 새 인카운터가 `combat_spectator_preview_unlocked` 게이트 뒤에 있어
   일반 플레이에서 도달 불가능하고, 전투 관전 렌더러가 아직 없어(Step 1d)
   Web 화면 출력 자체가 달라지지 않는다.
3. **`crates/escape-terminal/tests/cli_smoke.rs`** — 다른 작업자의 진행 중 변경
   (세션 시작 시점부터 이미 수정 상태). 읽기만 했고 건드리지 않았다.
4. **`.claude/worktrees/`** — 읽지도 쓰지도 않았다.
5. **혼합형·각본형 authoring, 개입 기회 제시, 치유·명줄·패배 결과, 전투 결과
   캐싱, terminal/Web 관전 렌더러** — 모두 플랜의 "명시적 범위 밖" 그대로 유지.
6. 새 의존성 추가 없음, `Cargo.toml`/`package.json` 무수정.

## 수정 문서 크기 (`wc -c`)

- `docs/design/Combat_System_Implementation_Plan_Index.md`: 19,998 bytes
- `docs/dev/Development_Plan.md`: 48,269 bytes
- `docs/dev/Combat_System_Operating_Guide.md`: 22,918 bytes
- `docs/dev/Combat_System_Goal_Prompt.md`: 12,747 bytes

모두 100KB 이하.

## 최종 workspace 상태

```
$ git log --oneline -8
6d971d4 docs(combat): record Wave 3 Step 2b in the combat docs (WP-4)
442cdaa test(combat): pin the spectator-preview producer regression (Wave 3 Step 2b, WP-3)
1e823e5 feat(combat): regenerate wuxia preview bundles for the spectator-preview encounter (Wave 3 Step 2b, WP-2)
720ebed feat(combat): author systemic spectator-preview combat encounter (Wave 3 Step 2b, WP-1)
09b30a5 feat(combat): encounter combat schema and systemic spectator producer (Wave 3 Step 2a) (#180)
de9861e feat(combat): spectator adapter, per-tick snapshot, conclusion report, provenance, ScenePage boundary (#179)
3bb8ad5 fix(core): ResultStage branch filter on every text surface + Event/Stage closeout (#178)
f9035d2 feat-combat-add-conclusion-sidecar (#177)
```

```
$ git diff --stat origin/main..HEAD
 .../wuxia_jianghu_pack.content.bundle.json         | 288 ++++++++++++++++++++-
 crates/escape-core/tests/content_bundle.rs         |   4 +-
 crates/escape-core/tests/encounter_combat_wave3.rs | 179 ++++++++++++-
 crates/escape-core/tests/event_stage_wave3.rs      |   6 +-
 crates/escape-core/tests/reward_pipeline_wave1.rs  |   2 +-
 .../Combat_System_Implementation_Plan_Index.md     |  10 +-
 docs/dev/Combat_System_Goal_Prompt.md              |   3 +-
 docs/dev/Combat_System_Operating_Guide.md          |   7 +-
 docs/dev/Development_Plan.md                       |   2 +-
 .../wuxia_jianghu_pack/encounters.yaml             | 220 ++++++++++++++++
 tests/test_web_data_export.py                      |   4 +-
 .../wuxia_jianghu_pack.content.bundle.json         | 288 ++++++++++++++++++++-
 12 files changed, 995 insertions(+), 18 deletions(-)
```

`git status --short -uall`은 `crates/escape-terminal/tests/cli_smoke.rs`(다른
작업자 변경, 세션 시작 시점부터 이미 수정 상태), `.claude/worktrees/`(미추적,
무변경), `fable_combat_wave3_step2b_2608021228.md`(플랜 파일, 미추적)만 남고
이 slice가 만든 4개 커밋 외에 다른 변경은 없다.

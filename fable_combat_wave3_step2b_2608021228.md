# Combat Wave 3 Step 2b — 시스템형 전투 인카운터 authoring 1개

작성: 2026-08-02
작성자: Fable (orchestrator plan)
구현 담당: coding subagent (sonnet, effort medium)

## Baseline

- 기준 브랜치: `claude/combat-wave3-step2b` (`origin/main` = `09b30a5`, PR #180 머지 직후)
- Baseline 검증 상태: `cargo test --workspace --no-fail-fast` → **316 passed / 0 failed** (2026-08-02 WSL 실측)
- Step 2a가 `EncounterDef.combat` 스키마와 시스템형 producer를 이미 제공한다. 스키마를 다시 만들지 마라.

## 정본 근거와 확정된 수치

- [11. 능력치·숙련·전투 스킬 시스템](https://app.notion.com/p/3a837e69695e818eafbccfa309d08149) — **표준 전투원이 확정 수치로 정의돼 있다.** 이 slice는 그 수치만 쓰고 새 밸런스 값을 발명하지 않는다.
  - 표준 전투원(전투력 100 기준): 기초능력 모두 **8**, 주분야·장착기술 숙련 **8**, 무기 공격력 **20**, 방어력 **5**, 초반 기술 1개(**위력 40 / 명중 100 / 효과 없음**)
  - `최대 생명력 = 20 + 지구력 기여도 × 10` → 지구력 8이면 **100**. 최대 호흡도 같다
  - `평범한 무기의 평타 위력 = 20`, `적용기술위력 = 기술기본위력 + (현재무기위력 − 20)` → 무기 20이면 위력 그대로
  - `사전방어 생명력피해 = 적용위력 × 5/12 × 능력배율`, 기본공격 능력배율은 모두 8이면 **1**
  - `감소율 = 유효방어 / (유효방어 + 20)` → 방어 5면 **20% 감소**
- [01. 전투 루프와 개입 예산](https://app.notion.com/p/36f37e69695e812c92efd2c11edabb66): 개입 기회 상한 0~3, **시스템형은 즉시 결과가 가능하다**
- [04. 선택지 생성 규칙](https://app.notion.com/p/36f37e69695e81a090ebe5f63ab5932e): **시스템형은 공유 효과만 사용한다**

### resolver 공식이 정본과 일치함을 실측 확인했다 (2026-08-02)

`combat_resolution.rs::damage`:
- `pre = power_hundredths × 5 × ability_multiplier_hundredths / 1200` → `ability_multiplier_hundredths = 100`(=1.0)일 때 `위력 × 5/12`. 정본 §8과 일치
- `reduction = pre × effective / (effective + 2000)` → 2000 hundredths = 20. `감소율 = 유효방어/(유효방어+20)`과 일치

표준 전투원 대 표준 전투원 검산: 위력 40(=`power_hundredths: 4000`), 능력배율 1(=`ability_multiplier_hundredths: 100`), 방어 5(=`defense_hundredths: 500`)
- `pre = 4000 × 5 × 100 / 1200 = 1666` (16.66)
- `reduction = 1666 × 500 / 2500 = 333` (20% 감소)
- **최종 피해 = 1333 hundredths = 13.33**, 생명력 100 → 결착까지 8타

## 콘텐츠 파이프라인 (중요)

이구학지 번들의 **진짜 source는 YAML**이다. 번들 JSON은 생성물이므로 직접 편집하지 마라.

- source: `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- 생성물 2개: `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`, `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- 재생성:
  ```bash
  python3 scripts/export_web_data.py \
    --storypack-preview wuxia_jianghu_pack \
    --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
    --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
    --write
  ```
  확인은 같은 명령의 `--check`. Python 인터프리터는 repo `.venv`를 쓴다 (`./.venv/bin/python`가 있으면 그것을 쓰고, 없으면 `python3`).
- `export_web_data.py`는 YAML 항목을 통째로 통과시킨다(`bundle[root_key] = entries`). **`combat:` 키를 위해 Python을 수정할 필요가 없다.** Python 쪽 combat 검증 mirror는 이 slice 범위 밖이다.

## 이 slice가 만드는 것

이구학지 preview에 **시스템형 전투 인카운터 1개**를 authoring한다. Step 2a의 producer가 이 인카운터에서 `ScenePage.combat`을 실제로 채우는 것이 이 slice의 성공 기준이다.

### 플레이어 경로에서 게이트한다 (필수)

전투 관전 렌더러가 아직 없다(Step 1d). 지금 이 인카운터를 일반 진행 경로에 노출하면 플레이어는 전투를 언급하는 서술과 선택지만 보고 전투 자체는 보지 못한다. 이는 회귀다.

따라서 `conditions.required_flags`에 **일반 플레이가 절대 세우지 않는 전용 게이트 플래그** `combat_spectator_preview_unlocked`를 넣는다. Step 1d가 렌더러를 붙인 뒤 이 플래그를 제거해 정식 경로로 승격한다. 이 사실을 YAML 주석과 문서에 남긴다.

## Hard invariants (위반 금지)

1. **생성된 번들 JSON을 직접 편집하지 않는다.** YAML만 고치고 export로 재생성한다. `--check`가 통과해야 한다.
2. **정본 11의 표준 전투원 수치만 쓴다.** 위 수치 표를 벗어나는 값을 발명하지 마라. 정본에 없는 필드(균형 최대치, 이동 속도, 충돌 반경, 사거리, tick 수)는 **YAML 주석에 provisional임을 명시**하고 보고서에 목록으로 남긴다.
3. **코드 상수 금지**: Rust/TS에 밸런스 수치를 넣지 마라. 전부 YAML에서 온다.
4. **`intervention_budget: 0`**: 개입 기회 제시는 Step 2c 소관이다. 0이 아닌 값을 쓰면 제시할 수 없는 예산을 선언하는 것이다.
5. **`kind: systemic`만.** 혼합형·각본형은 Step 2a 검증이 거부한다.
6. **staged event 필수**: 이구학지 전 인카운터가 ordered Story → Choice → per-choice ResultStage로 전환돼 있다. 새 인카운터도 `event` stages를 갖춰야 하고, 각 choice가 대응 ResultStage를 가져야 한다 (`docs/design/Event_Stage_Content_Model.md`).
7. **illustration slot 1개 이상**: 에셋이 없어도 `placeholder: true`와 `alt`를 넣는다. 가짜 파일 경로를 만들지 마라.
8. **게이트 플래그 유지**: `combat_spectator_preview_unlocked`를 세우는 콘텐츠를 만들지 마라. 이 slice에서 이 인카운터는 일반 경로에서 도달 불가여야 한다.
9. **기존 인카운터 수정 금지.** 새 항목만 추가한다.
10. **renderer 미접촉**: `crates/escape-terminal/`, `web/src/ui/`, `web/src/core/` 를 수정하지 않는다. `web/src/data/generated/`는 export 생성물이므로 재생성 결과만 반영된다.
11. **다른 작업자 변경 보존**: `crates/escape-terminal/tests/cli_smoke.rs` 읽기만. `.claude/worktrees/` 읽지도 쓰지도 않는다.
12. **신규 의존성 금지.**

## authoring 계약

- 인카운터 id: `wuxia_combat_spectator_preview_bout`
- 위치: 기존 위치 중 하나를 쓴다 (`cheongryu_outer_courtyard` 권장 — 수련 맥락)
- `conditions.required_flags`: `["combat_spectator_preview_unlocked"]`
- `repeatable: false`, `weight`는 기존 관례를 따른다
- 완료 플래그: `combat_spectator_preview_bout_resolved` (반복 방지)
- 전투 서술은 **결과를 단정하지 않는다.** 전투 결과는 core가 판정하므로, 서술은 "겨루기가 시작된다" 수준으로 두고 승패를 텍스트로 확정하지 마라.

### 전투원 2명 (표준 전투원 × 2)

`state.combatants` 2개, `participants` 2개, id 집합 일치 (Step 2a 검증 8번).

| 필드 | 값 | 근거 |
|---|---|---|
| `current_health` / `maximum_health` | 100 / 100 | 정본 11 §8, 지구력 8 |
| `current_breath` / `maximum_breath` | 100 / 100 | 정본 11 §8 |
| `balance` / `maximum_balance` | 100 / 100 | **provisional** — 정본에 균형 최대치 없음. 생명력·호흡 스케일에 맞춤 |
| `fear` / `anger` | 0 / 0 | 중립 시작 |
| `posture` | `neutral` | |
| `weapon_control` | `stable` | |

`participants`: 한쪽 `side: ally`, 한쪽 `side: enemy`. 위치·`facing`·`speed_per_tick`·`collision_radius`·`attack_range`·`support_range`는 **provisional**로 표시하고, 두 전투원이 서로 사거리 안에 들어오도록 최소값으로 잡는다. `role_id`는 `roles`에 정의한 preset id를 가리켜야 한다.

### attacks / defenses

각 전투원에게 공격 1개씩 (정본 11 §7 초반 템플릿 40/명중100/효과없음):

| 필드 | 값 |
|---|---|
| `power_hundredths` | 4000 (위력 40) |
| `ability_multiplier_hundredths` | 100 (능력배율 1.0 = 모두 8) |
| `accuracy_percent` | 100 |
| `penetration_hundredths` | 0 |
| `effects` | 빈 배열 (효과 없음) |
| `attack_range` | participants의 `attack_range`와 일관되게 (**provisional**) |
| `collision_balance_hundredths` / `balance_power_hundredths` | **provisional** — 정본에 균형 피해 표준값 없음. 작은 값으로 두고 주석에 명시 |

`defenses`: 두 전투원 모두 `defense_hundredths: 500` (방어력 5), `balance_resistance_hundredths: 0` (**provisional**).

### 나머지

- `effect_catalog`: 빈 `effects` 배열 (정본 04: 시스템형은 공유 효과만 사용하며, 이 인카운터는 효과를 쓰지 않는다)
- `config.tick_millis`: **provisional** (권장 100)
- `config.max_ticks` / `ticks`: 표준 전투원끼리 13.33 피해/타 → 100 생명력에 8타이므로, `ticks`를 결착이 나기 충분하게 잡고 `max_ticks`를 그 이상으로 둔다. 둘 다 **provisional**
- `termination`: `max_ticks`를 `config.max_ticks`와 맞추고 `conclude_on_max_ticks: true`
- `manifest`: `simulation_version`은 기존 combat 테스트가 쓰는 값 형식을 따른다. `actual_seed`는 **producer가 런 상태에서 파생해 덮어쓰므로 값에 의미가 없다** — YAML 주석에 그 사실을 남긴다. `combatant_ids`·`rule_ids`·`public_info_ids` 등 필수 필드를 채운다 (`manifest.validate()` 통과 필요)

## Work packages (순서 고정, WP당 커밋 1개)

### WP-1 — YAML authoring
`encounters.yaml`에 인카운터 1개 추가. `event` stages(Story → Choice → per-choice Result), illustration placeholder, `combat` 블록, 게이트 플래그.
검증: export `--write` 후 `--check` 통과. `cargo test -p escape-core --test content_bundle` 통과.

### WP-2 — 번들 재생성
위 export 명령으로 두 번들 재생성. 생성물 diff가 YAML 변경분만 반영하는지 확인한다.
검증: export `--check`, `cargo test --workspace --no-fail-fast`.

**이 지점에서 기존 카운트 단정이 깨진다.** 최소한 `crates/escape-core/tests/event_stage_wave3.rs`의 `wuxia_preview_has_full_51_event_coverage`가 51을 단정한다. 52로 갱신하고 **함수명도 `wuxia_preview_has_full_52_event_coverage`로 바꾼다** (문서 수치와 테스트명을 일치시키는 이 저장소 관례). 다른 카운트 단정(`content_bundle.rs`, pytest, `web/src/core/contentBundles.test.ts`)이 깨지면 **수치만** 갱신하고 어떤 파일의 어떤 단정을 바꿨는지 보고서에 나열한다. 로직은 바꾸지 마라.

### WP-3 — producer 회귀 테스트 (`crates/escape-core/tests/encounter_combat_wave3.rs`에 추가)
게이트 플래그를 세운 상태에서 이 인카운터가 실제로 전투를 만드는지 고정한다. 기존 22개 테스트 본문은 수정하지 마라.

1. 게이트 플래그 없이는 이 인카운터가 선택되지 않는다
2. 게이트 플래그를 세우면 선택되고 `ScenePage.combat`이 `Some`이다
3. `combat.view.frames`가 비어 있지 않고 `combat.report`가 `Some`이다
4. `report.combatants`가 2행이고 각 행의 `damage_dealt_hundredths`/`damage_taken_hundredths`가 0 이상이다
5. **정본 수치 검산**: 첫 명중의 `damage_hundredths`가 **1333**이다 (위력 40, 능력배율 1, 방어 5 → 정본 §8 공식). 이 단정이 authoring 수치와 resolver 공식을 함께 고정한다
6. 같은 상태로 두 번 호출하면 `ScenePage.combat`이 완전히 동일하다
7. 이 인카운터가 staged `event`를 갖는다 (invariant 6)

### WP-4 — 문서 갱신 (생략 금지)
- `docs/design/Combat_System_Implementation_Plan_Index.md`
  - `status:` → `wave3-step2b-complete`
  - 단계 표의 `(플랜 미작성) — Wave 3 Step 2b` 행을 이 플랜 파일명으로 교체
  - "현재 코드와 정본의 경계"에 첫 실 콘텐츠 확보분을 적고, **아직 없는 것**에 "이 인카운터는 `combat_spectator_preview_unlocked` 게이트 뒤에 있다 — Step 1d 렌더러 완료 후 게이트를 제거해 정식 경로로 승격한다"를 명시
  - 정본 11의 표준 전투원 수치를 authoring 기준으로 썼음과, provisional 필드 목록을 남긴다
- `docs/dev/Development_Plan.md` 10번(combat) 항목에 Step 2b를 반영
- `docs/dev/Combat_System_Operating_Guide.md`, `docs/dev/Combat_System_Goal_Prompt.md`에 한 줄씩 추가
- 문서에 수치를 적을 때는 **그 수치를 고정하는 테스트 함수명을 같이 적는다.** stale 수치는 갱신한다
- 각 문서 100KB 이하 유지

## 검증 명령

```bash
# 번들 최신 여부
python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack \
  --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json \
  --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check

cargo fmt --all -- --check
cargo test -p escape-core --test encounter_combat_wave3
cargo test -p escape-core --test event_stage_wave3
cargo test -p escape-core --test content_bundle
cargo test --workspace --no-fail-fast
git diff --check
```

pytest와 web 테스트도 돌린다 (`./.venv/bin/python -m pytest tests/ -q`, `cd web && npm test`). 깨지면 카운트 단정만 갱신하고 보고서에 나열한다.

**5뷰포트 시각 QA와 wasm 재빌드는 이 slice 범위 밖이다.** 근거: 새 인카운터가 게이트 플래그 뒤에 있어 일반 플레이에서 도달 불가이고, 전투 렌더러가 없어 Web 화면 결과가 달라지지 않는다. 이 판단을 보고서에 명시한다.

## 명시적 범위 밖

- 혼합형·각본형 authoring, 개입 기회/대응 제시, 행동 선택지 4개 + "개입하지 않는다" → Step 2c
- terminal/Web 관전 렌더러, 게이트 플래그 제거 → Step 1d
- Python 쪽 combat 검증 mirror
- 전투 결과 캐싱·save 저장
- 치유·명줄·패배 결과
- 밸런스 수치 확정 (provisional 필드는 provisional로 남긴다)
- 5뷰포트 QA, wasm 재빌드

## 최종 체크리스트

- [ ] 번들 JSON을 직접 편집하지 않았다 (YAML → export)
- [ ] export `--check` 통과
- [ ] 정본 11 표준 전투원 수치를 그대로 썼다 (위력 4000, 능력배율 100, 명중 100, 방어 500, 생명력/호흡 100)
- [ ] provisional 필드가 YAML 주석과 보고서에 목록으로 남았다
- [ ] `intervention_budget: 0`, `kind: systemic`
- [ ] 게이트 플래그 `combat_spectator_preview_unlocked`가 붙어 있고 이를 세우는 콘텐츠가 없다
- [ ] staged `event`와 illustration placeholder가 있다
- [ ] 첫 명중 피해가 **1333**임을 테스트로 고정했다
- [ ] 기존 인카운터 무수정, 기존 테스트 22개 본문 무수정
- [ ] 갱신한 카운트 단정을 보고서에 파일·단정 단위로 나열했다
- [ ] `cargo fmt --all -- --check`, `git diff --check` 통과
- [ ] `cargo test --workspace --no-fail-fast` 0 failed
- [ ] pytest·npm test 결과 기록
- [ ] WP-4 문서 4개 갱신
- [ ] `cli_smoke.rs`·`.claude/worktrees/`·`crates/escape-terminal/`·`web/src/ui`·`web/src/core` 무변경
- [ ] 보고서 `fable_combat_wave3_step2b_report.md` 작성
- [ ] **보고서/커밋 메시지에 backtick 있는 마크다운을 셸 heredoc으로 넣지 말 것.** python 스크립트도 heredoc으로 넘기지 말고 파일로 써서 실행한다 (이 세션에서 3회 유실 사고)

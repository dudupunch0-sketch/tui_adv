# Event / Stage / ContentBlock 전환 closeout — Step 1

작성: 2026-08-01
작성자: Fable (orchestrator plan)
구현 담당: coding subagent (sonnet, effort medium)

## Baseline

- 기준 커밋: `f9035d2` (= `origin/main`, PR #177 `feat-combat-add-conclusion-sidecar` 머지 직후)
- 작업 브랜치: `claude/eventstage-closeout-step1` (`origin/main`에서 분기)
- Baseline 검증 상태: `cargo test --workspace --no-fail-fast` → 23 test binary / **246 passed / 0 failed** (2026-08-01 WSL 실측)
- Combat Wave 2 Step 4 (conclusion sidecar)는 PR #177로 이미 머지됨. **재구현 금지.**

## 배경: 왜 이 slice인가

`docs/dev/Development_Plan.md` 192번은 Event/Stage/ContentBlock closeout의 남은 순서를 이렇게 적어 놨다.

1. 수동 acceptance + Notion reverse sync
2. ordered Stage/ContentBlock schema + legacy adapter/validation
3. Event 내부 cursor/save migration
4. renderer-neutral ordered stream
5. Web Storybook 고정 5영역 제거
6. 이구학지 대표 Event 점진 마이그레이션

코드 실측 결과 **2~6은 이미 구현·테스트로 고정되어 있다.**

| 항목 | 실제 상태 | 근거 |
|---|---|---|
| 2. schema | 완료 | `crates/escape-core/src/content.rs:273-305` (`EventDef`/`EventStageDef`/`ContentBlockDef`) |
| 2. index-time validation | 완료 | `content.rs:903-1046` (`validate_event`: stage id 유일성, choice→result 인접, dangling ref, branch 위치, block 필수 필드) |
| 3. cursor/save migration | 완료 | `state.rs:78-84` (`active_event_id`/`event_stage_index`/`event_next_stage_id`, 전부 `#[serde(default)]`), `turn.rs:281-320`, `turn.rs:681-734` |
| 4. renderer-neutral ordered stream | 완료 | `scene_page.rs:37-39` (`content_stream`), `scene_page.rs:462-587` |
| 5. Web 고정 5영역 제거 | 완료 | `web/src/ui/storybook/render.ts:158-234` (`renderOrderedStoryFlow`), `render.test.ts:184` |
| 6. 이구학지 Event 마이그레이션 | 완료 51/51 | `crates/escape-core/tests/event_stage_wave3.rs:144-145` |

즉 이 트랙에서 코드로 남은 것은 위 목록이 아니라 **branch 해석 누락 결함 1건**이고, 문서 쪽으로 남은 것은 **stale해진 closeout 서술**이다. 이 slice가 그 두 개를 닫는다.

### 실측된 결함 (재현 완료)

`wuxia_heuksa_bang_first_fight` → `event:continue` → `choice:run_toward_open_street` (seed 1, check 성공) 상태에서 core가 만드는 표면들:

| 표면 | 성공 분기 텍스트 | 실패 분기 텍스트 | 판정 |
|---|---|---|---|
| `ScenePage.content_stream` | 포함 | 미포함 | 정상 |
| `TurnView.body` | 포함 | **포함** | 결함 |
| `ScenePage.body_blocks` | 포함 | **포함** | 결함 |
| `ScenePage.dialogue_entries` | 포함 | **포함** | 결함 (`view.body`에서 파생, `scene_page.rs:251-260`) |

원인: branch 필터가 `scene_page.rs:484-496`에 **인라인으로 한 번만** 구현되어 있고, 같은 stage의 텍스트를 만드는 `turn.rs:672-679` `event_stage_text`는 branch를 모른다. `ScenePage.body_blocks`/`dialogue_entries`는 `view.body`(=`event_stage_text` 결과)에서 파생되므로 같이 오염된다 (`scene_page.rs:256`, `scene_page.rs:363`).

도달 경로 (실사용):

- terminal `--scene content` 비-play 경로: `headless.rs:35` `println!("{}", view.body)`
- terminal `--tui-smoke` snapshot: `snapshot.rs:165` `for block in &page.body_blocks`

결과적으로 플레이어가 **성공·실패 서술을 동시에** 보게 되는 스포일러 누출이다. Web Storybook은 `content_stream`을 우선 렌더하므로 현재 화면상으로는 드러나지 않지만, `body_blocks`/`dialogue_entries`는 renderer-neutral 공개 계약이므로 core가 같은 stage에 대해 서로 모순되는 텍스트를 내보내는 상태 자체가 결함이다.

## Scope

- **P1 (code)**: branch 해석을 core 한 곳으로 통합하고, stage 텍스트를 만드는 모든 표면에 적용한다.
- **P2 (test)**: 결함 재현 회귀를 `escape-core` 테스트로 고정한다 (성공/실패/판정없음 3케이스).
- **P3 (docs)**: Event/Stage closeout 상태와 Combat Wave 2 Step 4 완료를 canonical 문서에 반영한다. legacy adapter 항목은 명시적 결정으로 닫는다.

## Hard invariants (이전 슬라이스에서 상속 — 위반 금지)

1. **additive-optional 직렬화**: `CONTENT_BUNDLE_SCHEMA_VERSION`, `SAVE_SCHEMA_VERSION`을 올리지 않는다. 새 필드를 추가하지 않는다 (이 slice는 새 필드가 필요 없다).
2. **renderer boundary**: 판정·branch 선택은 Rust core만 한다. Web/terminal에 branch 판정 로직을 추가하지 않는다.
3. **action prefix / 저장 키 동결**: `choice:`, `event:continue`, `use:`, `train:` prefix와 save/localStorage key를 바꾸지 않는다.
4. **신규 의존성 금지**: `Cargo.toml`, `package.json`을 건드리지 않는다.
5. **route graph 불변**: encounter 선택 의미, eligibility, cursor 전이 규칙을 바꾸지 않는다. 이 slice는 **텍스트 필터링만** 바꾼다.
6. **콘텐츠 데이터 불변**: `crates/escape-core/fixtures/content/**`, `web/src/data/generated/**` bundle JSON을 수정하지 않는다.
7. **밸런스 수치 금지**: 확정되지 않은 수치를 코드 상수로 넣지 않는다.
8. **다른 작업자 변경 보존**: `crates/escape-terminal/tests/cli_smoke.rs`는 **읽기만** 하고 수정·되돌리기 금지 (현재 uncommitted `M` 상태). `.claude/worktrees/`도 건드리지 않는다.

## 검증 명령 (WP마다 실행)

```bash
cargo fmt --all -- --check
cargo test -p escape-core --test event_stage
cargo test -p escape-core --test event_stage_wave1
cargo test -p escape-core --test event_stage_wave3
cargo test --workspace --no-fail-fast
git diff --check
```

기준: workspace 246 passed 유지 + 신규 테스트 증가분. 실패가 생기면 숨기지 말고 출력 그대로 보고한다.

## Work packages (순서 고정, WP당 커밋 1개 — 단, 커밋은 오케스트레이터 지시가 있을 때만)

### WP-1 — branch 해석을 `EventStageDef`로 단일화

파일: `crates/escape-core/src/content.rs`

`EventStageDef`에 branch 해석 메서드를 추가한다. 이것이 이후 유일한 branch 판정 지점이다.

```rust
impl EventStageDef {
    /// ResultStage branch 해석 후 이 stage에서 보여줄 block만 원래 순서대로 돌려준다.
    ///
    /// `check_success`는 직전 ChoiceStage의 ability check resolution이다.
    /// `Some(true)`/`Some(false)`면 branch 없는 block과 일치하는 branch block을,
    /// `None`이면 branch 없는 block만 남긴다. result stage가 아니면 전부 통과한다.
    pub fn visible_blocks(
        &self,
        check_success: Option<bool>,
    ) -> impl Iterator<Item = &ContentBlockDef> {
        let is_result = self.kind == "result";
        self.blocks.iter().filter(move |block| {
            if !is_result {
                return true;
            }
            match block.branch.as_deref() {
                None => true,
                Some(branch) => check_success
                    .map(|success| branch == if success { "success" } else { "failure" })
                    .unwrap_or(false),
            }
        })
    }
}
```

요구사항:

- `pub`으로 노출하되 새 struct/enum/필드는 만들지 않는다.
- 기존 `"result"` / `"success"` / `"failure"` 문자열 리터럴 규약을 그대로 쓴다 (enum 타입화는 이 slice 범위 밖).
- doc comment는 한국어로 위 형태를 유지한다.

검증: `cargo fmt --all -- --check`, `cargo test -p escape-core` (이 WP만으로는 동작 변화 없음).

### WP-2 — `scene_content_stream`이 새 메서드를 쓰도록 교체

파일: `crates/escape-core/src/scene_page.rs`

`scene_content_stream`의 인라인 branch 필터(현재 `:481-508`의 `.filter(|block| { ... })` 블록)를 삭제하고 `stage.visible_blocks(...)`로 바꾼다.

```rust
let mut stream: Vec<_> = stage
    .visible_blocks(state.last_check.as_ref().map(|check| check.success))
    .map(|block| SceneContentItem { /* 기존 매핑 그대로 */ })
    .collect();
```

요구사항:

- `SceneContentItem` 매핑 필드와 순서, `choice`/`continue` sentinel push 로직(`:509-533`), flat encounter fallback(`:537-586`)은 **그대로 둔다**.
- 이 WP는 순수 리팩터이며 출력이 1바이트도 달라지면 안 된다.

검증: `cargo test -p escape-core --test event_stage --test event_stage_wave1` 포함 위 전체 명령. `event_stage_wave1.rs:151 heuksa_checked_result_streams_keep_only_the_matching_branch`가 계속 통과해야 한다.

### WP-3 — 회귀 테스트 먼저 (red 확인 필수)

파일: `crates/escape-core/tests/event_stage_branch_surfaces.rs` (신규)

WP-4 fix **전에** 작성하고 **실패하는 것을 확인한 뒤** 그 출력을 보고서에 남긴다. red 확인 없이 WP-4로 넘어가지 않는다.

테스트 3개:

1. `result_stage_turn_view_body_keeps_only_matching_branch`
   - `wuxia_heuksa_bang_first_fight` / `jianghu_market_street` / flags `["wuxia_arrival_hidden"]`
   - seed 1..=256 sweep으로 성공·실패 케이스를 각각 확보한다 (`event_stage_wave1.rs:150-201` 패턴 재사용).
   - 성공 케이스: `turn_view_from_content(...).body`가 `"민첩하게 큰길로 물러섰다"`를 포함하고 `"비틀거리다 몽둥이에 쓸리며"`를 **포함하지 않는다**.
   - 실패 케이스: 반대.
2. `result_stage_scene_page_body_blocks_and_dialogue_keep_only_matching_branch`
   - 같은 상태에서 `ScenePage.body_blocks`의 `text`들과 `ScenePage.dialogue_entries`의 `text`들에 대해 1과 같은 배타성을 검증한다.
3. `result_stage_without_check_resolution_keeps_only_common_blocks`
   - office fixture(`crates/escape-core/fixtures/content/content.bundle.json`)에 `event`를 주입하는 `crates/escape-core/tests/event_stage.rs:20-60` 패턴을 재사용해, `last_check`가 `None`인 result stage에서 `TurnView.body`와 `ScenePage.body_blocks`에 branch block 텍스트가 하나도 없고 공통 block만 남는지 검증한다.
   - fixture JSON 파일 자체는 수정하지 않는다. 테스트 안에서 `serde_json`으로 주입한다 (`event_stage.rs`가 이미 쓰는 방식).

요구사항:

- 테스트에서 `escape_core`의 기존 public API만 쓴다 (`load_content_bundle`, `index_content_bundle`, `new_game_from_content_at`, `apply_action_from_content`, `turn_view_from_content`, `scene_page_from_content`).
- seed sweep은 상한을 두고, 성공·실패 둘 다 못 찾으면 `expect`로 명확히 실패하게 한다.
- 기존 테스트 파일을 수정하지 않는다.

검증: `cargo test -p escape-core --test event_stage_branch_surfaces` → **FAIL 3건(또는 최소 1·2번 FAIL)** 확인 후 출력 기록.

### WP-4 — `event_stage_text`에 branch 해석 적용 (green)

파일: `crates/escape-core/src/turn.rs`

- `event_stage_text(stage: &EventStageDef) -> String`를 `event_stage_text(stage: &EventStageDef, state: &GameState) -> String`로 바꾸고 내부에서 `stage.visible_blocks(state.last_check.as_ref().map(|check| check.success))`를 순회한다. `join("\n\n")` 동작은 유지.
- 호출부 `turn.rs:188-191`을 `stage.map(|stage| event_stage_text(stage, state))`로 고친다.
- 다른 호출부가 있으면 모두 갱신한다. 새 public API를 만들지 않는다 (`event_stage_text`는 private 유지).

검증: `cargo test -p escape-core --test event_stage_branch_surfaces` → **3건 PASS**, 그리고 위 전체 검증 명령. `cargo test --workspace --no-fail-fast`가 **246 + 신규 3 = 249 passed / 0 failed**여야 한다.

**주의**: 이 변경으로 `crates/escape-terminal/tests/cli_smoke.rs`의 기대 출력이 깨질 수 있다. 깨지면 **그 파일을 고치지 말고** 실패 출력을 그대로 보고서에 적고 멈춘다 (다른 작업자 소유 파일).

### WP-5 — canonical 문서 truth-up

파일 4개. 각 파일 100KB 제한을 넘기지 않는다. 아래 사실만 반영하고 새 계획을 발명하지 않는다.

1. `docs/dev/Data_Schema.md` — 87~91행 단락 수정
   - 현재 문장은 branch를 "core가 `content_stream`을 만들 때만 해석한다"고 적어 놨다. 이를 "core가 해당 stage의 텍스트 표면(`content_stream`, `TurnView.body`, 그리고 그로부터 파생되는 `ScenePage.body_blocks`·`dialogue_entries`)을 만들 때 **동일하게** 해석한다"로 고친다.
   - 해석 지점이 `EventStageDef::visible_blocks` 한 곳임을 1문장으로 명시한다.
   - 나머지 규칙(공통 block 항상 표시, resolution 없으면 공통만, 순서 유지, renderer 재판정 금지)은 문장 그대로 유지한다.

2. `docs/dev/Development_Plan.md` — 192번 항목 재작성
   - "현재 최우선"이라는 표현을 유지한 채, 2~6번이 완료임을 위 근거 표(파일:라인, 테스트명)로 압축해 적는다.
   - 완료 근거 테스트명을 명시: `event_stage.rs`, `event_stage_wave1.rs`, `event_stage_wave2.rs`, `event_stage_wave3.rs` (`wuxia_preview_has_full_51_event_coverage` = 51/51), `web/src/ui/storybook/render.test.ts`.
   - 190행이 적어 둔 `44/44`는 Wave 3 시점 수치이고 Reward Pipeline Wave 1에서 사건 7개가 추가되어 현재는 **51/51**임을 각주 1문장으로 남긴다. 190행 원문은 당시 기록이므로 지우지 않는다.
   - 남은 closeout 항목을 3개로 정리한다: (a) 수동 acceptance, (b) Notion reverse sync (`idea_box/notion_sources.yml` 기준 커밋 + Notion "13. 런타임 시스템 현황" 갱신), (c) office/isolation pack staged 마이그레이션 — 계속 명시적 비범위.
   - **legacy adapter 결정 기록**: 별도 adapter 모듈을 만들지 않는다. flat encounter는 core가 `scene_page.rs:537-586` fallback으로 ordered `content_stream`을 이미 만들어 주고 있으며, office pack 21개 encounter의 staged 전환이 승격되기 전까지 새 adapter를 추가하지 않는다. (근거: office bundle의 `"event"` 필드 0건, `Development_Plan.md:188`의 office 비범위 결정.)
   - 이 slice에서 고친 branch 누출 결함을 1~2문장으로 기록한다 (증상, 영향 표면, 단일 해석 지점).

3. `docs/design/Event_Stage_Content_Model.md` — 45행 보강
   - "adapter로 해석할 수 있다"는 문장 뒤에, 현재 구현은 별도 adapter 모듈이 아니라 core의 ordered stream fallback이며 Encounter selection 의미는 그대로라는 1~2문장을 덧붙인다.
   - branch 섹션(59~63행)의 "renderer는 `content_stream`을 받은 뒤 branch를 다시 판정하지 않는다"는 유지하고, core의 모든 텍스트 표면이 같은 필터를 쓴다는 점을 1문장 추가한다.

4. `docs/design/Combat_System_Implementation_Plan_Index.md` — Step 4 반영
   - `status: wave2-step3-complete` → `wave2-step4-complete`.
   - "현재 코드와 정본의 경계"(17행)에 Wave 2 Step 4 conclusion sidecar가 구현·검증됨을 추가하고, 아직 없는 계약 목록에서 이미 확보된 항목을 제거하지 말고 Step 4가 채운 부분만 정확히 반영한다 (Step 4 = deterministic conclusion sidecar. **조기 결착/tick 중단, 패주·항복·증원, 전투 종료 narrative/report consumer는 여전히 없음**).
   - 단계 순서 표에 `fable_combat_wave2_step4_2607261845.md` 행을 Wave 2 Step 3 다음에 추가한다. 한 줄 구현 단위: "결정론적 conclusion sidecar", 핵심 non-goal: "조기 결착·tick 중단·전투 종료 보고서·renderer adapter".
   - 구현 위치를 1줄로 남긴다: `crates/escape-core/src/combat_conclusion.rs`, `crates/escape-core/tests/combat_conclusion_wave2.rs`.

또한 `docs/dev/Development_Plan.md`의 combat 항목(현재 130행, "Wave 1 Step 1~3 and Wave 2 Step 1~3 ... are implemented and verified")에 Wave 2 Step 4를 추가한다. 다음 미착수 항목(다수전 AI 조기 결착/tick 중단, 패주·항복·증원·대형·결속·배경 전투, 전투 종료 narrative/report consumer, Wave 3 Step 1 관전 adapter, Wave 3 Step 2 authoring, data-driven 밸런스)은 그대로 남긴다.

검증: `git diff --check`, 각 문서 크기 확인(`wc -c`)해서 100KB 초과 없음 확인. 문서 WP는 cargo 검증 불필요하지만 마지막에 `cargo test --workspace --no-fail-fast`를 한 번 더 돌린다.

## 명시적 범위 밖

- office/isolation pack 21개 encounter의 staged Event 전환
- `EventStageDef.kind` / `ContentBlockDef.kind`를 `String`에서 enum으로 타입화
- 새 ContentBlock taxonomy 추가, `cheongirok` block 의미 변경
- Notion reverse sync 실제 수행 (외부 쓰기 — 사용자 승인 필요)
- 수동 acceptance 플레이
- Web/Storybook DOM·CSS 변경, wasm 재빌드, 5뷰포트 QA (이 slice는 Rust core 텍스트 필터 + 문서만 바꾸므로 Web 표시 결과가 달라지지 않는다)
- `crates/escape-terminal/tests/cli_smoke.rs` 수정
- combat Wave 3 Step 1/2 구현
- 밸런스 수치 확정
- commit / push / PR 생성 (오케스트레이터 별도 지시 시에만)

## 최종 체크리스트

- [ ] WP-1 `EventStageDef::visible_blocks` 추가, 새 필드/의존성 없음
- [ ] WP-2 `scene_content_stream` 리팩터, 출력 무변화 (`event_stage_wave1.rs:151` 통과)
- [ ] WP-3 신규 테스트 3건 작성 후 **red 출력 기록**
- [ ] WP-4 `event_stage_text` 수정 후 **3건 green**
- [ ] `cargo fmt --all -- --check` 통과
- [ ] `cargo test --workspace --no-fail-fast` = 249 passed / 0 failed (246 baseline + 3)
- [ ] `git diff --check` 통과
- [ ] WP-5 문서 4개 수정, 각 100KB 이하
- [ ] `cli_smoke.rs`, `.claude/worktrees/`, fixture/generated bundle JSON 무변경 (`git status --short -uall`로 확인)
- [ ] 보고서 `fable_eventstage_closeout_step2_report.md`에 red→green 출력, 실행한 명령, 스킵한 항목과 사유 기록

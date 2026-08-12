# 전투 투자자 데모 Tier-1 일반 플레이 경로·후속 진행 계약

Status: canonical implementation contract / not implemented
Baseline: `origin/main` `411d0f939ca4098dec64fbd739091b968b5de4ca`
Scope: ordinary-play systemic combat entry, existing authored post-combat continuation, production Web/WASM evidence
Primary storypack: `wuxia_jianghu_pack` / **이구학지 — 천기록**

## 1. 이 문서가 확정하는 것

Tier-1은 기존 시스템형 대련 하나를 **새 게임에서 정상 선택지만 눌러** 도달하고, Rust
GameCore가 만든 전투 지역·로그·종료 보고서를 production Web/WASM 경로에서 본 뒤, 기존
authored 선택 결과를 적용하고 다음 기존 encounter로 진행하는 최소 세로 슬라이스다.

이 계약은 terminal settlement 구현이나 투자자 데모 전체 완료 선언이 아니다. 현재 대련은
`intervention_budget: 0`인 관전 대련이다. 플레이 도중 개입, paused checkpoint, terminal
outcome persistence, claim/entitlement, strategy-only/fixed-chance 응답을 보여주지 않는다.

핵심 결정은 다음과 같다.

1. 기존 `wuxia_cheongryu_chore_sparring`의 네 정상 선택 결과가 기존 플래그
   `combat_spectator_preview_unlocked`를 추가한다. 테스트나 UI가 플래그를 직접 주입하지
   않는다.
2. core의 명시적 early-courtyard encounter 순서가 이 플래그가 생긴 직후 기존
   `wuxia_combat_spectator_preview_bout`을 선택한다.
3. 대련이 화면에 있는 동안 `CombatConclusionReport`의 outcome/reason/fingerprint와 frame/log
   순서는 동일 GameState reload에서 결정론적으로 같아야 한다.
4. 결착 보고서를 본 뒤 기존 선택 `watch_the_bout_closely` 또는
   `keep_a_measured_distance`를 적용하면 기존 경험치·플래그·목적지·result stage만 그대로
   적용한다.
5. encounter를 떠난 뒤 outcome 자체는 durable하지 않다. 이를 새 GameState field나 flag로
   보존하지 않는다. durable terminal fact와 intervention/claim persistence는 반드시
   I2b→I7a→I7b→I7c 순서 뒤에 구현한다.

## 2. 권한과 구현 read order

충돌 시 다음 순서를 따른다.

1. `docs/dev/Development_Plan.md`의 active priority와 병렬 작업 경계
2. `docs/content/design_source/contracts/combat_contract_index.md`
3. `docs/content/design_source/contracts/intervention.yml`
4. `docs/content/design_source/handoffs/combat_contract_handoff.md`의 필수 구현 순서
   **I2b → I7a → I7b → I7c**
5. 이 문서 — Tier-1 일반 진입과 기존 authored 후속 진행 증거의 정본
6. `docs/design/Combat_System_Implementation_Plan_Index.md` — 현재 구현 inventory와 gap
7. `docs/design/ThreeJS_Combat_Visual_Architecture.md` — Web combat renderer의 truth/fallback
8. 실제 Rust/YAML/TS 코드와 테스트 — 현재 capability 증거

GameCore가 판정, outcome/reason, fingerprint, 좌표, cue/log 순서를 소유한다. Content
authoring은 진입 조건과 선택 보상·플래그·목적지를 소유한다. Web은 이 사실을 표시하고
정상 action id만 전달한다. Tier-1은 outcome을 persistent state나 authored reward로
재해석하지 않는다.

이 계약은 baseline main만으로 구현해야 한다. open PR `#217`, `#219`, `#220`, `#221`
중 어느 것도 선행 조건으로 두지 않는다. 그 PR의 타입, fixture, Three.js adapter 또는
mount가 없어도 모든 Tier-1 content/Web acceptance가 통과해야 한다.

## 3. baseline에서 확인한 사실과 gap

### 3.1 이미 있는 production 경로

- 전투 encounter는
  `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`의
  `wuxia_combat_spectator_preview_bout`이다.
- 위치는 기존 `cheongryu_outer_courtyard`, event 첫 stage는 기존
  `combat_spectator_preview_bout_story`다.
- `EncounterCombatKind::Systemic` producer는 `scene_page_from_content` 경로에서
  `resolve_combat → conclude_combat → spectate_combat`을 호출하고
  `ScenePage.combat`을 채운다.
- `ScenePage.combat`은 WASM `scene_page_json`을 거쳐 실제 Web runtime으로 전달되며,
  Storybook의 기존 DOM combat surface가 region/log/report를 표시한다.
- Web player는 기본 `wuxia_jianghu_pack` bundle, 실제 WASM binding, 실제
  `apply_action_json`, 실제 `localStorage` key `igu-hakji.rust.save.v1`을 사용한다.
- 현재 Web은 serialized `GameState`를 저장한다. 같은 state/content에서 combat producer가
  다시 실행되므로 encounter를 떠나기 전 report parity는 결정론으로 검증할 수 있다.

### 3.2 현재 일반 플레이에서 막힌 지점

- 대련은 `combat_spectator_preview_unlocked`를 요구한다.
- baseline에는 이 플래그를 만드는 ordinary choice가 없다. 기존 Rust 테스트는 상태에
  직접 push해 producer만 검증하므로 일반 플레이 증거가 아니다.
- `current_content_encounter`는 reward-pipeline local priority 뒤에 BTreeMap lexical scan을
  사용한다. 플래그만 풀어도 "장작 마당 첫 겨루기 직후"라는 안정된 timing은 보장되지
  않는다.
- 기존 두 대련 선택은 관찰 거리의 선택이다. 두 선택 모두
  `combat_spectator_preview_bout_resolved`, 각자의 관찰 플래그, 경험치 5,
  `cheongryu_outer_courtyard` 목적지를 이미 authoring했다.
- 현재 fixture/seed의 실제 outcome은 `MutualDefeat`다. 기존 resolved/branch 플래그는 combat
  outcome별 플래그가 아니며 그렇게 재라벨링하면 안 된다.
- action 적용 후 encounter를 떠나면 `CombatConclusionReport`의 outcome/reason/fingerprint를
  보존하는 canonical persistent fact가 없다.

### 3.3 canonical persistence 경계

`combat_contract_handoff.md`는 response/preflight delta를 I2b, atomic transaction을 I7a,
durable ledger·claim·SaveEnvelope v2를 I7b, lifecycle/terminal E2E를 I7c에 배정한다.
필수 순서는 **I2b → I7a → I7b → I7c**다.

따라서 Tier-1은 다음을 추가하지 않는다.

- `GameState.combat_settlements` 또는 유사 terminal result map
- 새 settlement/result receipt나 outcome별 persistent flag
- SaveEnvelope/checkpoint schema 변경
- terminal claim, loot entitlement, intervention ledger
- combat choice와 별도의 atomic settlement transaction

Tier-1의 reload evidence는 기존 GameState가 아직 같은 encounter/stage에 있을 때의
결정론과, 기존 choice outcome이 이미 가진 일반 state 필드의 parity만 증명한다.

## 4. 고정 ID와 ordinary entry

### 4.1 재사용하는 기존 ID

| 의미 | 고정 ID |
| --- | --- |
| ordinary trigger encounter | `wuxia_cheongryu_chore_sparring` |
| trigger의 공통 완료 플래그 | `cheongryu_chore_sparring_resolved` |
| 기존 gate 플래그 | `combat_spectator_preview_unlocked` |
| systemic combat encounter | `wuxia_combat_spectator_preview_bout` |
| combat location/destination | `cheongryu_outer_courtyard` |
| combat story stage | `combat_spectator_preview_bout_story` |
| combat choice stage | `combat_spectator_preview_bout_choice` |
| post-combat choice A | `watch_the_bout_closely` |
| post-combat choice B | `keep_a_measured_distance` |
| common authored completion flag | `combat_spectator_preview_bout_resolved` |
| choice A authored flag | `combat_spectator_preview_watched_closely` |
| choice B authored flag | `combat_spectator_preview_kept_distance` |
| exact next encounter | `wuxia_cheongryu_training_first_failure` |

새 location, encounter, event, choice, balance, reward, outcome flag ID를 만들지 않는다.

### 4.2 선택한 entry mechanism

`wuxia_cheongryu_chore_sparring`의 기존 네 choice outcome의 `add_flags`에
`combat_spectator_preview_unlocked`를 additive하게 추가한다. 기존 resource delta, clue,
experience 15, log, destination은 그대로 유지한다.

`current_content_encounter`의 현재 reward-pipeline 우선순위를 이름이 목적에 맞는
early-courtyard 순서로 좁게 정리하고, `wuxia_combat_spectator_preview_bout`을
`wuxia_cheongryu_training_first_failure`보다 앞에 둔다. gate가 없을 때는 선택될 수 없고,
gate가 생긴 다음 turn에는 대련이 선택된다. 대련에 기존 resolved flag가 생기면 forbidden
condition 때문에 다시 선택되지 않고, 기존 조건이 충족된
`wuxia_cheongryu_training_first_failure`가 다음 encounter가 된다.

이 방식은 새 데모 버튼, query parameter, seed shortcut, dev menu, hidden keyboard shortcut,
직접 state mutation을 만들지 않는다. 플레이어는 default storypack에서 기존 사건의 기존
선택을 누른다.

## 5. outcome과 기존 authored continuation의 관계

현재 `CombatConclusionOutcome` 전수 관계는 다음과 같이 취급한다.

| outcome | encounter 안의 report evidence | 기존 choice outcome | encounter 이후 durable outcome |
| --- | --- | --- | --- |
| `InProgress` | report가 terminal evidence가 아니므로 Tier-1 경로 실패 | 적용하지 않음 | 없음 |
| `AllyVictory` | core report를 그대로 표시·reload 비교 | 기존 A/B outcome 그대로 | 없음 |
| `EnemyVictory` | core report를 그대로 표시·reload 비교 | 기존 A/B outcome 그대로 | 없음 |
| `MutualDefeat` | core report를 그대로 표시·reload 비교 | 기존 A/B outcome 그대로 | 없음 |
| `Stalemate` | core report를 그대로 표시·reload 비교 | 기존 A/B outcome 그대로 | 없음 |

Tier-1은 outcome을 reward 분기로 매핑하지 않는다. 기존 A/B 선택은 관찰 방식이며 combat
outcome과 독립적이다. 선택 A는 기존 경험치 5, 공통 completion flag, A flag, 기존 log,
`cheongryu_outer_courtyard`를 적용한다. 선택 B도 기존 경험치 5, 공통 completion flag,
B flag, 기존 log, 같은 destination을 적용한다.

현재 deterministic bout의 `MutualDefeat`를 ally victory, defeat reward, battle-loss route로
재해석하지 않는다. report를 떠난 뒤 outcome을 조회해야 하는 기능은 I7c 이후 terminal
contract가 제공할 때까지 구현하지 않는다.

## 6. reload evidence 경계

Tier-1의 reload acceptance는 세 지점으로 제한한다.

1. **post-combat choice 전:** 아직 combat encounter 안인 기존 GameState를 reload하고 동일
   encounter id, outcome, reason, resolution/report fingerprint, frame/log 순서를 다시 얻는다.
2. **기존 choice 적용 뒤 result stage:** reload하고 경험치 delta, 공통/branch flag,
   destination, active result stage가 같은지 확인한다. outcome persistence는 주장하지 않는다.
3. **`event:continue` 뒤:** reload하고 exact next encounter가 같은지 확인한다. 이 시점에는
   combat outcome이 durable하지 않음을 명시적으로 확인한다.

`InProgress`, pause snapshot, response selection, transaction scratch, Three.js/DOM state를
저장하지 않는다. paused combat save/reload, terminal fact, claims는 I7b/I7c 전에는 구현할 수
없다. 따라서 Tier-1은 full intervention demo나 terminal settlement completion을 만족하지
않는다.

## 7. production Web/WASM browser scenario

QA는 임시 HTML, synthetic `ScenePage`, direct `renderCombatStage`, fixture import, Rust state
mutation을 사용하지 않는다.

1. WSL에서 canonical wuxia content bundle을 export한다.
2. WSL에서 `wasm-pack build crates/escape-wasm --target web --out-dir web/src/core/wasm-pkg`를
   실행한다.
3. 실제 Vite app을 열고 `localStorage`를 비운다.
4. start screen의 default 이구학지 새 게임으로 시작한다.
5. UI에 노출된 `event:continue`와 authored choice action만 클릭해
   `wuxia_cheongryu_chore_sparring`까지 진행한다. script가 state JSON이나 flag를 쓰거나
   고치지 않는다.
6. 장작 마당의 기존 choice 하나를 선택하고, 다음 page가
   `wuxia_combat_spectator_preview_bout`인지 확인한다.
7. `data-region="combat"`, board/log, `data-region="combat-report"`, terminal outcome/reason,
   non-empty fingerprints가 실제 DOM에 있는지 확인한다.
8. post-combat choice 전 `window.location.reload()`하고 report/fingerprint/ordered log parity를
   확인한다.
9. `watch_the_bout_closely`를 선택한다. experience가 정확히 기존 값 5만큼 증가하고,
   `combat_spectator_preview_bout_resolved`,
   `combat_spectator_preview_watched_closely`, `cheongryu_outer_courtyard`, 정확한 result stage가
   존재하는지 확인한다.
10. reload 뒤 같은 authored state를 확인하고 `event:continue`를 누른다.
11. 다음 encounter가 정확히 `wuxia_cheongryu_training_first_failure`인지 확인하고 다시
    reload해 parity를 확인한다. combat outcome durable state를 기대하지 않는다.
12. 전체 scenario에서 `console.error`, page error, unhandled rejection, failed WASM request가
    0인지 확인한다. 예상치 않은 error를 문자열 allowlist로 숨기지 않는다.

기존 `npm run qa:storybook:visual -- --base-url <dev-url> --out-dir <scratch>` 5 viewport도
통과해야 한다. start-page visual pass만으로 live route를 대체할 수 없다.

## 8. 구현 슬라이스

각 슬라이스는 자기 테스트와 함께 단독 logical commit으로 merge 가능해야 한다. 범위 밖
파일이 필요하면 먼저 이 계약을 갱신하고 멈춘다.

### Slice A — ordinary route, priority, generated artifacts

선행: 없음. I2b/I7 및 PR #217/#219/#220/#221 불필요.
최대 owned files: 7.

- source YAML `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- early-courtyard priority owner `crates/escape-core/src/turn.rs`
- focused ordinary-route Rust integration test
- exporter가 소유하는 Rust fixture bundle JSON
- exporter가 소유하는 Web generated bundle JSON
- 필요 시 기존 bundle count/list assertion 파일 최대 2개

Acceptance:

- 네 chore choice 모두 기존 outcome을 유지하면서 기존 gate flag만 추가한다.
- gate 없는 state에서는 bout가 선택되지 않는다.
- ordinary chore choice 직후 bout가 선택되고 `ScenePage.combat.report`가 terminal이다.
- 현재 authored bout report는 기존 `MutualDefeat`/`BothSidesDefeated` 결과를 유지한다.
- A/B choice 각각 기존 경험치 5, 공통/branch flag, destination, result stage를 유지한다.
- result continue 뒤 exact next encounter는 `wuxia_cheongryu_training_first_failure`다.
- source YAML과 두 generated bundle이 exporter 결과와 일치한다.
- 새 GameState field, save/checkpoint schema, settlement/receipt/claim 코드는 0개다.

Stop condition: exact route를 보장하려고 새 location/encounter/choice ID, balance/reward 값,
terminal persistence를 만들어야 하면 중단한다.

### Slice B — production browser route/reload evidence

선행: Slice A.
최대 owned files: 3.

- 새 focused live-route QA script 1개
- `web/package.json`의 명령 1개
- 필요할 때만 그 script의 focused test/helper 1개

WASM output은 build artifact이며 commit-owned file이 아니다. `web/src/main.ts`, storage key,
renderer, types, CSS를 바꾸지 않는다.

Acceptance는 §6~7 전체와 기존 Web unit/build/5-viewport gate다. artifact에는 normal action
trace, pre-choice report/fingerprint parity, post-choice XP/flags/location/result stage, next
encounter parity, console/page error count를 남긴다. outcome이 encounter 이후 durable하다고
assert하지 않는다.

Stop condition: production app에서 필요한 화면·저장 상태를 관찰할 안전한 방법이 없어
script가 private module 호출, state injection, synthetic ScenePage에 의존해야 하면 임시 hook을
만들지 않고 관찰 경계의 별도 승인을 요청한다.

### Slice C — canonical docs closeout

선행: A, B 모두 pass.
최대 owned files: 2.

- `docs/dev/Development_Plan.md`
- `docs/design/Combat_System_Implementation_Plan_Index.md`

실제 artifact와 commit SHA를 링크하고 **Tier-1 ordinary route evidence**만 complete로 표시한다.
terminal settlement, I7b/I7c, Three.js WP, boss/intervention/performance를 complete로 바꾸지 않는다.

## 9. 전체 acceptance gate

필수 명령은 WSL에서 실행한다.

```bash
cargo fmt --all -- --check
cargo test -p escape-core --no-fail-fast
cargo test -p escape-wasm --no-fail-fast
cargo test --workspace --no-fail-fast
python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --write
python3 scripts/export_web_data.py --storypack-preview wuxia_jianghu_pack --preview-bundle crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json --preview-bundle web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json --check
wasm-pack build crates/escape-wasm --target web --out-dir web/src/core/wasm-pkg
cd web && npm test
cd web && npm run build
cd web && npm run qa:combat-demo-route -- --base-url <dev-url> --out-dir <scratch>
cd web && npm run qa:storybook:visual -- --base-url <dev-url> --out-dir <scratch>
git diff --check
```

Pass evidence:

- normal story action trace와 direct flag/state injection 0건
- combat region, ordered log, terminal report
- encounter 안 pre-choice reload의 report/fingerprint parity
- 기존 XP5/result flags/destination/result stage의 post-choice reload parity
- exact next encounter와 reload parity
- encounter 이탈 뒤 durable outcome을 주장하는 assertion 0건
- console error, page error, unhandled rejection, failed WASM load 0건
- 기존 no-combat output과 office save key 무변경

## 10. 금지 사항

- `GameState.combat_settlements` 또는 다른 terminal result 저장 경로 추가
- I2b→I7a→I7b→I7c 전에 terminal fact, receipt, ledger, claim, paused persistence 구현
- 테스트, query string, dev menu, localStorage 편집으로 gate flag 직접 주입
- `new_game_from_content_at` 뒤 state에 flag를 push한 것만으로 ordinary-path PASS 선언
- synthetic `ScenePage`, direct renderer import, 임시 HTML harness를 production browser 증거로 사용
- `CombatConclusionOutcome`을 Web/terminal에서 다시 판정
- authored completion/branch flag를 outcome별 persistent flag로 재라벨링
- `MutualDefeat`를 ally victory, defeat reward, battle-loss route로 재해석
- 새 reward/balance/destination/outcome flag를 임의 authoring
- 기존 bout 경험치 5 또는 chore 경험치 15/resource/clue/log 변경
- `SAVE_SCHEMA_VERSION` 또는 `COMBAT_RUNTIME_CHECKPOINT_SCHEMA_VERSION` 변경
- Three.js dependency/mount/canvas/GLB/VFX 추가
- boss combat, mixed/scripted combat, strategy-only/fixed-chance intervention 구현
- open PR의 unmerged API를 hidden dependency로 사용
- Tier-1 PASS를 terminal settlement, full investor demo, intervention demo complete로 표현

## 11. 명시적 defer와 독립성

다음은 Tier-1 뒤에도 열린다.

- I2b runtime provenance/RNG/selector delta
- I7a atomic intervention transaction
- I7b SaveEnvelope v2, ledger, entitlement/claim persistence
- I7c lifecycle/terminal E2E와 durable terminal result 정책
- paused checkpoint/save/reload와 intervention-before-settlement
- boss combat 및 final combat resolution
- strategy-only intervention과 fixed-chance special effect
- mixed/scripted authoring과 pause UI
- Three.js WP1 이후 board/stage/mount, GLB, VFX, context-loss integration
- display-name identity registry와 internal-ID 제거
- combat caching, 축 관통/AI·collision 수정, authored balance 확정
- PC performance benchmark와 mobile optimization

Tier-1은 이 항목들과 데이터/API dependency를 만들지 않는다. 특히 existing completion flag와
choice branch flag는 향후 I7c terminal truth의 대체물이 아니다.

## 12. stop rules와 남은 제품 한계

다음 중 하나가 확인되면 숫자나 ID를 발명하지 않고 해당 owner 결정 전 중단한다.

- 기존 chore choice 뒤 exact bout/next-event 순서를 narrow priority로 보장할 수 없다.
- 같은 encounter state reload에서 deterministic report/fingerprint parity가 깨진다.
- production browser가 실제 WASM/bundle이 아니라 fallback/synthetic path를 사용한다.
- existing authored reward/destination/flag semantics를 바꿔야만 route가 이어진다.
- 요구사항이 encounter 이후 durable outcome, claim, paused state를 필요로 한다.

마지막 항목은 Tier-1로 최소화할 수 없다. 해당 요구가 생기면 이 route를 확장하지 않고
I2b→I7a→I7b→I7c를 진행한다.

현재 baseline 조사에서 ordinary entry 자체의 blocking product decision은 없다. 남은 제품
한계는 명확하다. 이 route는 **관전자 선택과 기존 후속 진행**을 보여줄 뿐, 플레이어 개입이나
durable terminal settlement를 보여주지 않는다. 이 한계를 마케팅 문구로 덮지 않는다.

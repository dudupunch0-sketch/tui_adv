# 전투 투자자 데모 Tier-1 일반 플레이 경로 계약

Status: canonical implementation contract / not implemented
Baseline: `origin/main` `411d0f939ca4098dec64fbd739091b968b5de4ca`
Scope: ordinary-play systemic combat entry, terminal settlement persistence, production Web/WASM evidence
Primary storypack: `wuxia_jianghu_pack` / **이구학지 — 천기록**

## 1. 이 문서가 확정하는 것

Tier-1은 기존 시스템형 대련 하나를 **새 게임에서 정상 선택지만 눌러** 도달하고, Rust
GameCore가 만든 전투 지역·로그·종료 보고서를 Web Storybook에서 본 뒤, 기존 authored
선택 결과를 적용하고, 종료된 전투의 결과 사실을 저장·재로드해도 동일하게 보존하는 최소
세로 슬라이스다.

이 계약은 투자자 데모 전체 완료 선언이 아니다. 특히 플레이 도중 개입하는 데모가 아니며,
현재 `intervention_budget: 0`인 관전 대련만 사용한다. 개입 일시정지, paused checkpoint,
전략-only 응답, fixed-chance 응답, 보스전, Three.js mount가 없어도 Tier-1은 완료할 수 있다.
반대로 Tier-1 완료를 근거로 이 항목들이 준비됐다고 말하면 안 된다.

핵심 결정은 다음과 같다.

1. 기존 `wuxia_cheongryu_chore_sparring`의 네 정상 선택 결과가 기존 플래그
   `combat_spectator_preview_unlocked`를 추가한다. 테스트나 UI가 플래그를 직접 주입하지
   않는다.
2. core의 명시적 early-courtyard encounter 순서가 이 플래그가 생긴 직후 기존
   `wuxia_combat_spectator_preview_bout`을 선택한다.
3. 전투 결과의 persistent truth는 outcome별 새 플래그가 아니라
   `GameState.combat_settlements[encounter_id]`의 typed terminal fact다. 현재 authored
   플래그는 의미를 바꾸지 않는다.
4. 결착 뒤 기존 선택 `watch_the_bout_closely` 또는 `keep_a_measured_distance`를 적용할
   때만 settlement fact와 기존 보상/플래그/목적지를 한 action 결과로 확정한다.
5. 저장·재로드 경계는 **terminal combat only**다. `InProgress` 또는
   `paused_for_intervention` 상태 저장은 Tier-1 범위가 아니며 I7b/I7c 소관이다.

## 2. 권한과 구현 read order

충돌 시 다음 순서를 따른다.

1. `docs/dev/Development_Plan.md`의 active priority와 병렬 작업 경계
2. `docs/content/design_source/contracts/combat_contract_index.md`
3. `docs/content/design_source/handoffs/combat_contract_handoff.md`
4. 이 문서 — Tier-1 일반 진입·terminal settlement·브라우저 증거의 정본
5. `docs/design/Combat_System_Implementation_Plan_Index.md` — 현재 구현 inventory와 gap
6. `docs/design/ThreeJS_Combat_Visual_Architecture.md` — Web combat renderer의 truth/fallback
7. 실제 Rust/YAML/TS 코드와 테스트 — 현재 capability 증거

GameCore가 판정, outcome/reason, fingerprint, 좌표, cue/log 순서를 소유한다. Content
authoring은 진입 조건과 선택 보상·플래그·목적지를 소유한다. Web은 이 사실을 표시하고
정상 action id만 전달한다. 이 문서는 balance, reward, outcome을 renderer에서 다시
판정할 권한을 만들지 않는다.

이 계약은 baseline main만으로 구현해야 한다. open PR `#217`, `#219`, `#220`, `#221`
중 어느 것도 선행 조건으로 두지 않는다. 그 PR의 타입, fixture, Three.js adapter 또는
mount가 없어도 모든 Tier-1 core/content/Web acceptance가 통과해야 한다. 병합 시 동일
파일 충돌은 최신 main에 기계적으로 재착지하되 이 문서의 의미를 확장하지 않는다.

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
  `apply_action_json`, 실제 `localStorage` save key `igu-hakji.rust.save.v1`을 사용한다.
- 현재 Web 저장값은 `SaveEnvelope`가 아니라 serialized `GameState`다. core의
  `SaveEnvelope`는 schema v1이며 optional combat checkpoint는 비어 있으면 생략한다.

### 3.2 현재 일반 플레이에서 막힌 지점

- 대련은 `combat_spectator_preview_unlocked`를 요구한다.
- baseline에는 이 플래그를 만드는 ordinary choice가 없다. 기존 Rust 테스트는 상태에
  직접 push해 producer만 검증하므로 일반 플레이 증거가 아니다.
- `current_content_encounter`는 reward-pipeline local priority 뒤에 BTreeMap lexical scan을
  사용한다. 플래그만 풀어도 "장작 마당 첫 겨루기 직후"라는 안정된 entry timing은
  보장되지 않는다.
- `CombatConclusionReport`는 `ScenePage`에 표시되지만 action 적용 후 GameState에 남지
  않는다. encounter가 끝나면 report를 다시 찾을 canonical persistent fact가 없다.
- 기존 두 대련 선택은 관찰 거리의 선택이다. 두 선택 모두
  `combat_spectator_preview_bout_resolved`, 각자의 관찰 플래그, 경험치 5,
  `cheongryu_outer_courtyard` 목적지를 이미 authoring했다. 이 값은 전투 outcome을 뜻하지
  않으며 바꾸거나 재해석하면 안 된다.
- 현재 fixture/seed의 실제 outcome은 `MutualDefeat`이다. 이 한 결과만을 전체 enum의
  의미로 일반화하면 안 된다.
- paused combat persistence는 구현되지 않았다. I7b가
  `GameState.combat_intervention_ledger`, SaveEnvelope v2, checkpoint 정책을 소유하고,
  I7c가 lifecycle/terminal E2E를 소유한다.

### 3.3 현재 품질 gap은 그대로 남는다

표시 이름 canonical identity, 이동 중 좌우 축 관통, 반복 로그 집계, provisional
balance/movement/range/tick 수치, 나머지 종료 유형은 Tier-1이 고치지 않는다. 기존 authored
수치와 reward를 바꾸지 않는 것이 이 계약의 hard invariant다.

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
| settlement choice A | `watch_the_bout_closely` |
| settlement choice B | `keep_a_measured_distance` |
| common resolved flag | `combat_spectator_preview_bout_resolved` |
| choice A flag | `combat_spectator_preview_watched_closely` |
| choice B flag | `combat_spectator_preview_kept_distance` |
| exact next encounter | `wuxia_cheongryu_training_first_failure` |

새 location, encounter, event, choice, balance, reward ID를 만들지 않는다.

### 4.2 선택한 entry mechanism

`wuxia_cheongryu_chore_sparring`의 기존 네 choice outcome의 `add_flags`에
`combat_spectator_preview_unlocked`를 additive하게 추가한다. 기존 resource delta, clue,
experience 15, log, destination은 byte-for-byte 의미를 유지한다.

`current_content_encounter`의 현재 reward-pipeline 우선순위를 이름이 목적에 맞는
early-courtyard 순서로 좁게 정리하고, `wuxia_combat_spectator_preview_bout`을
`wuxia_cheongryu_training_first_failure`보다 앞에 둔다. gate가 없을 때는 선택될 수 없고,
gate가 생긴 다음 turn에는 대련이 선택된다. 대련이 resolved되면 forbidden flag 때문에
다시 선택되지 않고, 기존 조건이 충족된 `wuxia_cheongryu_training_first_failure`가 다음
encounter가 된다.

이 방식은 새 "투자자 데모" 버튼, query parameter, seed shortcut, dev menu, hidden keyboard
shortcut을 만들지 않는다. 플레이어는 default storypack에서 기존 사건의 기존 선택을
누른다.

## 5. terminal settlement fact

### 5.1 persistent shape와 소유권

GameState에 additive-default map을 둔다. 이름과 필드 의미는 다음 계약을 따른다.

```text
combat_settlements: BTreeMap<encounter_id, EncounterCombatSettlementFact>

EncounterCombatSettlementFact:
  encounter_id: String
  outcome: CombatConclusionOutcome
  reason: CombatConclusionReason
  resolution_fingerprint: String
  report_fingerprint: String
```

map key와 `fact.encounter_id`는 같아야 한다. fingerprint는 core report가 이미 제공한 값을
그대로 복사한다. renderer fingerprint, timestamp, wall clock, authored display label,
reward amount는 저장하지 않는다. `#[serde(default)]`로 old GameState JSON은 빈 map으로
읽힌다.

이 fact는 completed systemic encounter의 최소 result truth다. I7b의 intervention ledger,
loot entitlement, decision receipt, paused checkpoint를 대신하지 않는다. Tier-1 때문에
`SAVE_SCHEMA_VERSION` 또는 `COMBAT_RUNTIME_CHECKPOINT_SCHEMA_VERSION`을 올리지 않는다.

### 5.2 enum 전수 mapping

| `CombatConclusionOutcome` | 허용 reason | terminal fact | authored outcome 적용 |
| --- | --- | --- | --- |
| `InProgress` | `NoTerminalCondition` | 저장 금지 | choice/reward/flag/destination 적용 금지 |
| `AllyVictory` | `AllEnemiesDefeated` | enum 값을 그대로 저장 | 기존 choice outcome을 그대로 적용 |
| `EnemyVictory` | `AllAlliesDefeated` | enum 값을 그대로 저장 | 기존 choice outcome을 그대로 적용 |
| `MutualDefeat` | `BothSidesDefeated` | enum 값을 그대로 저장 | 기존 choice outcome을 그대로 적용 |
| `Stalemate` | `MaxTicksReached` | enum 값을 그대로 저장 | 기존 choice outcome을 그대로 적용 |

현재 enum/reason에 없는 flee, surrender, capture, objective, forced stop을 Tier-1 플래그로
흉내 내지 않는다. 정본 termination 계약이 runtime enum을 확장하면 exhaustive match가
컴파일 실패하거나 테스트가 실패해야 하며, 해당 owner가 이 표와 persistence migration을
먼저 갱신한다.

outcome/reason 조합이 표와 다르거나 report/fingerprint가 없으면 settlement action 전체를
거부한다. partial fact, reward만 적용, flag만 적용, 목적지만 이동은 모두 금지한다.

### 5.3 적용 시점과 원자성

`wuxia_combat_spectator_preview_bout`의 choice action을 적용하기 직전에 GameCore가 같은
state/content/encounter에서 production systemic producer를 호출한다.

1. report가 terminal인지 확인한다.
2. enum/reason 조합과 non-empty fingerprints를 검증한다.
3. 기존 map entry가 없으면 candidate state에 fact를 넣는다.
4. 같은 encounter와 같은 fact가 이미 있으면 idempotent success로 취급한다.
5. 같은 encounter에 다른 fact가 있으면 stale/conflict error로 전체 action을 거부한다.
6. 같은 candidate state에 기존 authored choice outcome을 적용한다.
7. 모든 단계가 성공할 때만 next state를 반환한다.

따라서 선택 A의 현재 결과는 경험치 **5**, 공통 resolved flag, A flag,
`cheongryu_outer_courtyard`, 기존 log다. 선택 B도 경험치 **5**, 공통 resolved flag, B flag,
같은 destination, 기존 log다. 전투 outcome에 따라 이 보상 값을 올리거나 내리지 않는다.
현재 deterministic bout가 `MutualDefeat`여도 두 authored 관찰 선택의 보상은 그대로다.

result stage의 `event:continue`는 새 settlement를 만들지 않고 active event를 닫는다. 다음
page는 `wuxia_cheongryu_training_first_failure`여야 한다.

## 6. 저장·재로드 경계

Tier-1에서 "terminal-only"는 CLI terminal을 뜻하지 않고 **전투가 결착된 뒤에만**
save/reload acceptance를 연다는 뜻이다.

필수 경계는 두 개다.

- **결착 보고서 화면, choice 전:** raw GameState 저장·페이지 reload 뒤 동일
  `encounter_id`, outcome, reason, resolution fingerprint, report fingerprint, frame/log 순서를
  다시 얻는다. 현재 producer가 렌더마다 재실행되므로 이 parity는 결정론으로 증명한다.
- **choice 적용 뒤:** 저장·reload 뒤 동일 settlement fact, 경험치 delta, 공통/branch flag,
  destination, result stage를 얻는다. `event:continue` 뒤 reload해도 정확한 next encounter가
  유지된다.

`InProgress`, pause snapshot, response selection, mid-transaction scratch, Three.js/DOM state를
저장하지 않는다. paused combat save/reload는 I7b 이후 I7c acceptance가 있어야 하며,
Tier-1은 full intervention demo를 만족할 수 없다.

## 7. production Web/WASM browser scenario

QA는 임시 HTML, synthetic `ScenePage`, direct `renderCombatStage`, fixture import, Rust state
mutation을 사용하지 않는다. 다음 production path를 그대로 쓴다.

1. WSL에서 canonical content bundle을 export한다.
2. WSL에서 `wasm-pack build crates/escape-wasm --target web --out-dir web/src/core/wasm-pkg`를
   실행한다.
3. 실제 Vite app을 열고 `localStorage`를 비운다.
4. start screen의 default 이구학지 새 게임으로 시작한다.
5. UI에 노출된 `event:continue`와 authored choice action만 클릭해
   `wuxia_cheongryu_chore_sparring`까지 진행한다. state JSON, flag, encounter를 script가
   쓰거나 고치지 않는다.
6. 장작 마당의 기존 choice 하나를 선택하고, 다음 page가
   `wuxia_combat_spectator_preview_bout`인지 확인한다.
7. `data-region="combat"`, combat board/log, `data-region="combat-report"`, outcome/reason,
   non-empty fingerprints가 실제 DOM에 있는지 확인한다.
8. settlement choice 전 `window.location.reload()`하고 report/fingerprint/ordered log의
   parity를 확인한다.
9. `watch_the_bout_closely`를 선택한다. 저장된 state에서 experience가 정확히 기존 값 5만큼
   증가하고, `combat_spectator_preview_bout_resolved`,
   `combat_spectator_preview_watched_closely`, matching settlement fact,
   `cheongryu_outer_courtyard`가 존재하는지 확인한다.
10. reload 뒤 같은 result stage와 fact를 확인하고 `event:continue`를 누른다.
11. 다음 encounter가 정확히 `wuxia_cheongryu_training_first_failure`인지 확인하고 다시
    reload해 parity를 확인한다.
12. 전체 scenario에서 `console.error`, page error, unhandled rejection, failed WASM request가
    0인지 확인한다. 예상치 않은 error를 문자열 allowlist로 숨기지 않는다.

기존 `npm run qa:storybook:visual -- --base-url <dev-url> --out-dir <scratch>` 5 viewport도
통과해야 한다. 다만 그 스크립트의 start-page visual pass만으로 위 live route를 대체할 수
없다. Tier-1 live route용 Playwright scenario는 별도 명령으로 재실행 가능해야 한다.

## 8. 구현 슬라이스

각 슬라이스는 자기 테스트와 함께 단독 logical commit으로 merge 가능해야 한다. 다음
슬라이스는 명시된 선행 슬라이스의 public 결과만 사용한다. 범위 밖 파일이 필요하면 먼저
이 계약을 갱신하고 멈춘다.

### Slice A — terminal settlement persistence

선행: 없음. PR #217/#219/#220/#221 불필요.
최대 owned files: 6.

- `crates/escape-core/src/state.rs`
- `crates/escape-core/src/scene_page.rs`
- `crates/escape-core/src/turn.rs`
- 필요할 때만 새 renderer-neutral systemic producer helper 1개
- 필요할 때만 `crates/escape-core/src/lib.rs`
- 새 focused Rust integration test 1개

Acceptance:

- old GameState JSON은 빈 `combat_settlements`로 load된다.
- enum 5개를 전수 검사하고 `InProgress`는 mutation 0으로 거부한다.
- 네 terminal variant는 exact enum/reason/fingerprint fact로 저장된다.
- duplicate equal fact는 idempotent, conflict fact는 reward 포함 mutation 0이다.
- systemic producer의 판정 코드는 한 곳만 있고 `scene_page`와 settlement가 같은 helper를
  소비한다.
- 기존 no-combat action/save tests가 byte/semantic parity를 유지한다.

Stop condition: shared producer를 추출하지 않고 turn에서 resolver pipeline을 복제해야 하거나,
atomic candidate 적용 없이 기존 state 일부를 먼저 mutate해야 하면 구현하지 않는다.

### Slice B — ordinary route와 authored artifacts

선행: Slice A.
최대 owned files: 7.

- source YAML `src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml`
- early-courtyard priority owner `crates/escape-core/src/turn.rs`
- Rust route/settlement integration test
- exporter가 소유하는 Rust fixture bundle JSON
- exporter가 소유하는 Web generated bundle JSON
- 필요 시 기존 bundle count/list assertion 파일 최대 2개

Acceptance:

- 네 chore choice 모두 기존 outcome을 유지하면서 기존 gate flag를 추가한다.
- gate 없는 state에서는 bout가 선택되지 않는다.
- ordinary chore choice 직후 bout가 선택되고 `ScenePage.combat.report`가 terminal이다.
- 현재 authored bout에서 `MutualDefeat`/`BothSidesDefeated` fact가 저장된다.
- A/B choice 각각 기존 경험치 5, 공통/branch flag, destination을 정확히 유지한다.
- result continue 뒤 exact next encounter는
  `wuxia_cheongryu_training_first_failure`다.
- source YAML과 두 generated bundle이 exporter 결과와 일치한다.

Stop condition: exact next encounter를 보장하려고 새 location/encounter/choice ID 또는
authoring balance/reward 값을 만들어야 하면 중단한다.

### Slice C — production browser acceptance

선행: A, B.
최대 owned files: 3.

- 새 focused live-route QA script 1개
- `web/package.json`의 명령 1개
- 필요할 때만 그 script의 focused test/helper 1개

WASM output은 build artifact이며 commit-owned file이 아니다. `web/src/main.ts`, storage key,
renderer, types, CSS를 바꾸지 않는다.

Acceptance는 §7 전체와 기존 Web unit/build/5-viewport gate다. test artifact에는 route action
trace, pre/post reload fingerprints, final flags/fact/experience/location/next encounter, console와
page error count를 남긴다. screenshot은 보조 증거이며 JSON/assertion 결과를 대체하지 않는다.

Stop condition: production app에서 encounter identity나 saved state를 관찰할 안전한 방법이
없어 browser script가 private module 호출, state injection, synthetic ScenePage에 의존해야
하면 임시 hook을 만들지 말고 계약 owner에게 관찰 경계를 요청한다.

### Slice D — canonical 문서 closeout

선행: A, B, C 모두 pass.
최대 owned files: 2.

- `docs/dev/Development_Plan.md`
- `docs/design/Combat_System_Implementation_Plan_Index.md`

실제 테스트 artifact와 commit SHA를 링크하고 Tier-1만 complete로 표시한다. I7b/I7c,
Three.js WP, boss/intervention/performance를 complete로 바꾸지 않는다.

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

Pass에는 다음 evidence가 모두 있어야 한다.

- normal story action trace와 direct flag/state injection 0건
- combat region, ordered log, terminal report
- pre-settlement reload의 report/fingerprint parity
- typed settlement fact와 기존 reward/result flags
- post-settlement reload parity와 exact next event
- console error, page error, unhandled rejection, failed WASM load 0건
- 기존 no-combat story output과 office save key 무변경

## 10. 금지 사항

- 테스트, query string, dev menu, localStorage 편집으로 gate flag를 직접 주입
- `new_game_from_content_at` 뒤 state에 flag를 push한 것만으로 ordinary-path PASS 선언
- synthetic `ScenePage`, direct renderer import, 임시 HTML harness를 production browser 증거로 사용
- `CombatConclusionOutcome`을 Web/terminal에서 다시 판정
- `MutualDefeat`를 ally victory나 실패로 재라벨링
- outcome별 새 flag ID, 새 reward 수치, 새 balance 수치, 새 destination을 임의 authoring
- existing choice의 경험치 5 또는 chore choice의 경험치 15/resource/clue/log 변경
- paused checkpoint, transaction scratch, renderer state 직렬화
- `SAVE_SCHEMA_VERSION`을 Tier-1 때문에 v2로 올려 I7b 권한 침범
- Three.js dependency/mount/canvas/GLB/VFX 추가
- boss combat, mixed/scripted combat, strategy-only/fixed-chance intervention 구현
- raw internal combatant ID 노출 문제를 이 route의 scope expansion으로 해결
- open PR의 unmerged 파일이나 API를 hidden dependency로 사용
- Tier-1 PASS를 full investor demo 또는 intervention demo complete로 표현

## 11. 명시적 defer와 독립성

다음은 Tier-1 뒤에도 열린다.

- boss combat 및 final combat resolution
- strategy-only intervention과 fixed-chance special effect
- I7b SaveEnvelope v2, intervention ledger, entitlement/claim persistence
- I7c paused lifecycle, checkpoint restore, intervention-before-settlement E2E
- mixed/scripted authoring과 pause UI
- Three.js WP1 이후 board/stage/mount, GLB, VFX, context-loss integration
- paused full-log access
- display-name identity registry와 internal-ID 제거
- combat caching
- 축 관통/AI·collision 수정과 authored balance 확정
- PC performance benchmark, shadow/postprocess 비교, mobile optimization

Tier-1은 이 항목들과 데이터/API dependency를 만들지 않는다. 특히 terminal settlement fact는
I7b ledger에 들어가지 않고 별도 `GameState` field로 남는다. 향후 I7c가 terminal claim이나
intervention receipt를 결착에 연결할 때는 이 fact를 덮어쓰지 말고 같은 encounter/fingerprint
precondition으로 조합해야 한다.

## 12. premise와 stop rules

다음 중 하나가 확인되면 숫자나 ID를 발명하지 않고 해당 owner 결정 전 중단한다.

- 기존 chore choice 뒤 exact bout/next-event 순서를 narrow priority로 보장할 수 없다.
- current outcome/reason enum으로 terminal 여부를 전수·무손실 표현할 수 없다.
- 동일 state/content의 systemic producer 재실행이 deterministic fingerprint parity를 잃는다.
- old GameState JSON에 additive-default fact를 넣는 것이 save compatibility를 깬다.
- settlement fact가 I7b ledger/checkpoint schema와 필수적으로 같은 transaction이어야 한다.
- production browser가 실제 WASM/bundle이 아니라 fallback/synthetic path를 사용한다.
- existing authored reward/destination/flag semantics가 outcome별 보상을 요구한다는 새 product
  결정이 나온다.

현재 baseline 조사에서는 위 collapse condition이 확인되지 않았다. 따라서 새 제품 결정 없이
Slice A부터 구현할 수 있다. 남은 제품적 한계는 하나다: 이 route는 **관전자 선택**과
terminal 결과 증거를 보여줄 뿐 플레이어 개입을 보여주지 않는다. 그 한계는 축소하거나
마케팅 문구로 덮지 않고 I7b/I7c 이후 별도 Tier로 유지한다.

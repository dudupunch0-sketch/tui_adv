# 이구학지 게임 루프 1차 확장 플랜 — 레퍼런스 플레이 경험 도입 (Plan by Fable, implementation by codex/gemini)

Date: 2026-07-12
Baseline: `main` HEAD `c1af8fa` ("feat(web-ui): redesign drawer toggle as seal button and fix drawer close UX")
Reference: [docs/reference/Life_in_Adventure_Play_Reference.md](docs/reference/Life_in_Adventure_Play_Reference.md) — **이 플랜의 모든 요구는 이 문서를 기준으로 한다. 원본 이미지를 다시 읽을 필요 없음.**

이 문서는 구현 단계의 완전한 지시서다. 처음부터 끝까지 읽은 뒤 워크패키지를
**순서대로, 하나씩** 실행한다. 각 워크패키지가 끝나면: 검증 명령 실행 → git
커밋 1개 → 다음 워크패키지로 이동. 완료/스킵 내역과 편차는
`fable_gameloop_step2_report.md`(repo root)에 기록한다.

빌드/테스트 환경 주의: `cargo`/`wasm-pack`/`gh`는 **WSL에만 있다**
(`AGENTS.md`). 모든 검증 명령은 WSL `~/work/tui-adv`에서 실행한다.

---

## 0. 이 작업이 무엇이고 무엇이 아닌가

레퍼런스 문서 §5(UX 문법)와 §6(적용 노트)에서 **1차로 채택할 뼈대 4가지**를
엔진/렌더러에 도입한다:

1. **P1 — 캐릭터 스탯 노출**: 코어에 이미 있는 6스탯(`PlayerState.abilities`)을
   ScenePage에 실어 드로어에서 보이게 한다.
2. **P2 — 판정 확률 사전 공개**: 2d6 판정 선택지에 "관련 스탯 + 성공확률 %"를
   선택 전에 표기한다 (레퍼런스 UX 문법 1 "정보의 정직성").
3. **P3 — 특성(칭호) 시스템**: 캐릭터 이름 앞에 붙는 칭호와 회차 중 승급
   (`- 칼날의 / + 검호` 패턴의 무협 재해석). HUD/기록에 즉시 반영.
4. **P4 — 경험(진행 게이지)과 회차 메타**: 경험 누적 → 목표 도달 시 회차 종료
   문법의 엔진 지원 + 렌더러 쪽 회차 카운트/엔딩 도감(수집) 최소 구현.

**이번 범위가 아닌 것** (레퍼런스에 있지만 의도적으로 후속 슬라이스로 미룸):
d20/전투 연출, 사망·기사회생 연출, 성향(협/마) 축, 로맨스, 상점/경제,
모의전투, 랭킹, 캐릭터 생성 화면. §4 "범위 밖" 참조.

**시각 폴리시는 이번 범위가 아니다.** 이 플랜의 web WP는 데이터가 화면에
"구조적으로" 나타나게만 한다(기존 수묵 토큰 재사용, 새 시각 언어 발명 금지).
스탯 표기·확률 표기·칭호 표기의 미감 다듬기는 Claude가 별도 step에서 직접
작업한다 — 그때 바꾸기 쉽도록 신규 마크업에는 아래 지정된 클래스/데이터
속성을 정확히 붙일 것.

### 0.1 절대 불변 — Hard invariants

1. **Renderer boundary 유지**: eligibility/outcome/ending/확률 계산은 전부
   Rust GameCore. 렌더러는 ScenePage를 표시만 한다. 확률을 TS에서 재계산하지
   않는다.
2. **ScenePage 신규 필드는 전부 additive-optional**:
   - Rust: `Option<T>` + `#[serde(skip_serializing_if = "Option::is_none")]`
     (또는 기본 빈 컬렉션 + `skip_serializing_if = "Vec::is_empty"`).
   - TS(`web/src/core/types.ts`): `?` optional로 미러링.
   - 목적: 값이 없을 때 직렬화 JSON이 baseline과 동일 → 기존 고정 계약 테스트
     (`core_contract.rs`, `escape-wasm/tests/json_contract.rs`)의 대량 수정 회피.
     그래도 깨지는 테스트는 각 WP의 "테스트 수정 허용" 명시분만 고친다.
3. **기존 필드/액션 id/저장 키 불변**: `choice:*`/`move:*`/`use:*` 접두사,
   `ScenePage` 기존 필드명, localStorage 기존 키
   (`igu-hakji.rust.save.v1`, `igu-hakji.last-run-summary.v1`,
   `tui-adv.player-settings.v1`) 전부 그대로. **새 키 추가는 WP-W4에 지정된
   것만** 허용.
4. **GameState 신규 필드는 `#[serde(default)]`** — 기존 세이브가 그대로
   로드돼야 한다. `SAVE_SCHEMA_VERSION`은 1 유지.
5. **콘텐츠 번들 스키마 확장은 additive**: 신규 섹션/필드는 없으면 무시되는
   optional. `CONTENT_BUNDLE_SCHEMA_VERSION` 1 유지. YAML/생성 산출물을 수정한
   WP에서는 반드시 `python scripts/export_web_data.py --write` 후 `--check`
   통과 + Rust 픽스처 동기화까지 한 커밋에 포함.
6. **QA 계약 셀렉터 유지**: `storybook-reference-qa.mjs`의
   REQUIRED/FORBIDDEN 셀렉터, `render.test.ts`의 기존 assert
   (`.storybook-hud`, `[data-region="choices"]`, `button.choice-row[data-action-id]` 등).
7. **새 npm/cargo 의존성 금지.**
8. **저작권**: 레퍼런스 게임의 문구·아트·고유명사를 복제하지 않는다.
   모든 신규 라벨/서사는 이구학지 세계관의 창작 텍스트로 작성한다.
9. **접근성**: 색만으로 정보를 전달하지 않는다(획득/상실은 색 + `+`/`-` 기호
   + sr 텍스트). 확률/스탯 수치는 항상 텍스트로도 존재.
10. **터미널 렌더러(escape-terminal)는 컴파일만 유지** — 신규 필드를 표시할
    의무 없음. `route_parity.rs`가 깨지면 로직 변경이 잘못된 것.

### 0.2 검증 명령 (모든 WP 후, WSL에서)

```bash
cargo test --workspace
python scripts/export_web_data.py --check   # 콘텐츠 산출물을 만진 WP만
cd web && npm test && npx tsc --noEmit && npm run build && cd ..
./.venv/bin/python -m pytest tests/ -q      # 문서/데이터 계약을 만진 WP만
```

전체 완료 후 1회:

```bash
cd web && npm run build:player
npm run qa:storybook:visual -- --require-wasm --base-url http://127.0.0.1:4173/ --out-dir /tmp/tui-adv-gameloop-qa
```

### 0.3 커밋 컨벤션

WP당 커밋 1개.

```
feat(core): <설명> [WP-S<n>]
feat(web): <설명> [WP-W<n>]
feat(content): <설명> [WP-C<n>]
docs(design): <설명> [WP-D<n>]
```

---

## 1. 현재 상태 진단 (2026-07-12 코드 조사 결과)

- `PlayerState.abilities`(`crates/escape-core/src/state.rs`,
  `default_abilities()`: logic/empathy/volition/composure/interface/physical 각 2)는
  존재하지만 **ScenePage 어디에도 실리지 않는다**. 플레이어는 자기 스탯을
  볼 수 없고, 판정 선택지가 어떤 스탯을 쓰는지도 모른다.
- 판정(`AbilityCheckDef`, `content.rs`)은 `roll_2d6(해시) + ability >= difficulty`
  (`turn.rs`의 `ability_check_succeeds`)로 해소되지만 **성공 확률은 어디에도
  계산/노출되지 않는다**.
- **경험/레벨/회차 카운트/도감/칭호: 전부 없음.** `turn`이 유일한 진행 카운터.
  업적(`unlocked_achievements`)이 유일한 수집형 시스템.
- ScenePage는 `core_contract.rs`(필드별 정확값), `json_contract.rs`(3970줄,
  wuxia 전 루트 재생), `render.test.ts`(`samplePrinterPage` fixture),
  `contentBundles.test.ts`가 고정하고 있다 → invariant 2의 optional 전략 필수.

## 2. 대상 시스템 설계 (이대로 구현)

### 2.1 ScenePage 확장 (신규 필드 3개, 전부 optional)

`crates/escape-core/src/scene_page.rs`의 `ScenePage`에 추가:

```rust
/// P1+P3: 캐릭터 요약. content 기반 씬에서 항상 Some.
pub character_summary: Option<CharacterSummary>,
// CharacterSummary {
//   name: String,              // 지금은 고정 주인공 호칭(§2.4), 후속 생성 시스템에서 대체
//   title_label: Option<String>, // 현재 특성(칭호) 라벨. 없으면 None
//   abilities: Vec<AbilityStatus>, // { id, label, value } 6종 고정 순서
// }

/// P4: 진행 게이지. 번들에 progression 설정이 있을 때만 Some.
pub progression: Option<ProgressionStatus>,
// ProgressionStatus { experience: u32, target: u32, label: String }
```

`SceneAction`(+`SceneBlockedAction`)에 추가:

```rust
/// P2: 판정 선택지 정보. check가 있는 choice에만 Some.
pub check: Option<ActionCheckInfo>,
// ActionCheckInfo {
//   ability_id: String,
//   ability_label: String,   // ability_label() 헬퍼 (resource_label 패턴과 동일하게 core 소유)
//   success_percent: f32,    // 소수 1자리로 반올림된 사전 확률 (예: 58.3)
// }
```

능력 한글 라벨은 core에 헬퍼 `ability_label(id)`로 둔다(`resource_label`과
같은 파일·같은 문법). 매핑: logic=논리 / empathy=공감 / volition=의지 /
composure=평정 / interface=인터페이스 / physical=신체. wuxia 세계관용 개명은
후속 슬라이스에서 콘텐츠 레이어로 처리하므로 지금은 이 6개 고정.

### 2.2 판정 확률 계산 (core 신설 함수)

`turn.rs`(또는 인접 모듈)에 순수 함수:

```rust
/// P(2d6 + ability >= difficulty)를 백분율로. 2d6 분포는 고정 표.
pub fn ability_check_success_percent(ability: i32, difficulty: i32) -> f32
```

- 필요값 `need = difficulty - ability`에 대해 P(2d6 >= need)를 계산.
  need <= 2 → 100.0, need > 12 → 0.0. 분포: 36분의 {1,2,3,4,5,6,5,4,3,2,1}
  (2~12 눈). 소수 1자리 반올림.
- 단위 테스트 필수: need=7 → 58.3, need=12 → 2.8, 경계(<=2, >12) 포함.
- 주의: 실제 굴림 `roll_2d6`는 seed/turn 기반 결정론 해시다. 표기 확률은
  "시드 전체에 대한 사전 확률"이며 이는 레퍼런스의 사전 공개 문법과 부합한다.
  이 사실을 함수 doc comment에 명시할 것.

### 2.3 특성(칭호) — 콘텐츠 주도

- 번들 신규 섹션 `traits`(optional): `TraitDef { id, name, description }`.
  효과 수치는 이번 슬라이스 범위 밖 — 칭호는 우선 서사·표시 장치다.
- `GameState.trait_id: Option<String>` (`#[serde(default)]`).
- `OutcomeDef`에 `set_trait: Option<String>` 추가(optional). outcome 적용 시
  교체하고, 교체가 일어나면 로그 2줄을 생성한다: 이전 칭호가 있으면
  `- 특성: {이전 이름}`, 새 칭호 `+ 특성: {새 이름}` (§2.5 delta 로그 문법).
- `CharacterSummary.title_label` = 현재 trait의 name.
- 번들 검증(`validate_content_bundle`/`index_content_bundle`): trait id 중복
  금지, outcome이 참조하는 set_trait id 존재 검사.

### 2.4 주인공 표시 이름

캐릭터 생성 시스템은 이번 범위 밖이므로 `CharacterSummary.name`은 번들
runtime 메타의 optional `protagonist_name`(기본값 `"당신"`)에서 온다.
wuxia preview 번들에는 이구학지 주인공 호칭을 넣는다(WP-C1). 칭호가 있으면
렌더러가 `"{title_label} {name}"` 형태로 조합 표시한다.

### 2.5 결과 델타 로그 (색 코딩의 데이터 기반)

현재 outcome 로그는 문자열 한 덩어리다. 레퍼런스 문법(획득 초록 `+`,
상실 빨강 `-`)을 구조적으로 지원하기 위해, `apply_action` 결과 로그를
생성할 때 자원/아이템/특성/경험 변화를 **각각 별도 로그 라인**으로 만들고
접두사를 규격화한다: 증가 `+ {라벨} {n}` / 감소 `- {라벨} {n}` /
획득 `+ {아이템명}` / 상실 `- {아이템명}`. 렌더러는 이 접두사(`+ `/`- `)로만
분류한다(파싱 최소화). 기존 로그 문구를 바꾸는 것이 아니라 **델타 라인을
추가**하는 것임에 주의 — 기존 서사 로그는 그대로 둔다.
`json_contract.rs`에서 로그를 정확값으로 고정한 assert가 깨지면 해당
assert에 델타 라인을 추가 반영하는 수정만 허용.

### 2.6 경험/진행 게이지 — 번들 opt-in

- `GameState.experience: u32` (`#[serde(default)]`).
- `OutcomeDef.experience: Option<i32>` — outcome 적용 시 가감(0 미만 방지).
  델타 로그 `+ 경험 {n}` 생성.
- `ContentConditions.min_experience: Option<u32>` — 엔딩/인카운터/선택지
  조건에서 사용 가능. 이것으로 "경험 N 도달 → 종장 개방/엔딩" 문법을
  콘텐츠가 조립한다 (엔진에 하드코딩된 "경험 100 = 종료" 규칙을 만들지
  않는다 — 레퍼런스의 규칙을 콘텐츠 규칙으로 일반화).
- 번들 runtime 메타에 optional `progression: { experience_target: u32, label: String }`
  — 있으면 `ScenePage.progression`이 채워지고 게이지가 보인다. 없으면(오피스
  팩) 종전과 완전 동일.

### 2.7 회차 메타(도감) — 렌더러 로컬

회차 카운트/엔딩 수집은 게임 규칙이 아니라 **플레이어 로컬 메타**이므로
web에 둔다 (core 순수성 유지, `last-run-summary` 선례).

- 새 localStorage 키: `igu-hakji.meta.v1` =
  `{ schema_version: 1, run_count: u32, endings_seen: string[], achievements_seen: string[] }`.
- 갱신 시점: 새 모험 시작 시 `run_count += 1`; ending 페이지 렌더 시
  해당 ending id를 `endings_seen`에(중복 없이), `achievement_summary.unlocked`를
  `achievements_seen`에 병합.
- 시작 화면에 "{run_count+1}번째 기록" 표기 + 본 엔딩 수(`N편의 결말`)를
  드로어/시작 메뉴에 표기. 목록 UI(도감 페이지)는 이번 범위 밖 — 카운트와
  저장 구조까지만.

---

## 3. 워크패키지

실행 순서: **D1 → S1 → S2 → S3 → S4 → W1 → W2 → W3 → W4 → C1 → D2**.
담당: S/C = codex 권장, W = codex 또는 gemini. 어느 WP가 설명보다 위험해
보이면 건너뛰고 커밋 메시지에 사유를 남긴 뒤 계속한다.

### WP-D1: 설계 문서 신설 (docs 계약 선행)

`docs/design/Progression_and_Title_Model.md` 신설 — §2 전체(2.1~2.7)를
설계 계약으로 기술하고, `docs/00_Index.md`의 design/ 트리와
"현재 생성된 문서" 목록에 등록. `docs/dev/Development_Plan.md`에 이번
슬라이스 항목 추가. `tests/test_docs_contract.py`가 요구하는 동기화가 있으면
함께 충족(실패 메시지를 읽고 지시대로 갱신).
**검증**: pytest docs 계약 통과.

### WP-S1: 스탯/칭호 ScenePage 노출 (P1, P3 표시부)

1. `scene_page.rs`: `CharacterSummary`/`AbilityStatus` 타입 + `ability_label()`
   + `ScenePage.character_summary`(invariant 2 규칙) + 조립 로직.
   abilities 순서는 `default_abilities()`의 BTreeMap 순회가 아니라
   **명시적 고정 배열**(logic, empathy, volition, composure, interface, physical)로.
2. `state.rs`: `GameState.trait_id` 추가(`#[serde(default)]`).
3. `content.rs`: `traits` 섹션 + `TraitDef` + 검증/인덱싱 (§2.3).
4. 신규 단위 테스트: character_summary 직렬화 shape, trait 있는/없는 경우,
   기존 세이브(trait_id 없는 JSON) 로드 하위호환.

**테스트 수정 허용**: `core_contract.rs`에 character_summary 관련 assert
추가는 자유. 기존 assert는 optional 전략 덕에 무수정 통과가 목표 —
깨지면 구현이 invariant 2를 어긴 것.

### WP-S2: 판정 확률 계산·노출 (P2)

1. `ability_check_success_percent` 구현 + 단위 테스트 (§2.2).
2. `scene_page.rs`: `ActionCheckInfo` + `SceneAction.check`/
   `SceneBlockedAction.check` 채움 (choice의 `AbilityCheckDef`에서).
3. `cost_text`처럼 터미널도 쓸 수 있게 사람이 읽는 요약 문자열을 만들지
   말 것 — 구조화 필드만. (표기는 렌더러 몫.)

**테스트 수정 허용**: `json_contract.rs`에서 check 있는 choice의 action
객체를 정확값으로 고정한 assert에 `check` 필드 추가 반영.

### WP-S3: 결과 델타 로그 + set_trait outcome (P3 규칙부)

1. `content.rs`: `OutcomeDef.set_trait` (§2.3).
2. `turn.rs` `apply_outcome`(및 인접): 자원/아이템/특성 델타 로그 라인 생성
   (§2.5). 로그 순서: 서사 로그 → 델타 라인들.
3. trait 교체 적용 + `- 특성:`/`+ 특성:` 로그.
4. 단위 테스트: 델타 로그 포맷, trait 교체 시나리오.

**테스트 수정 허용**: 로그 정확값 assert(`core_contract.rs`,
`json_contract.rs`, `route_parity.rs`)에 델타 라인 추가 반영. **assert의
기존 서사 문구 자체는 바꾸지 않는다.**

### WP-S4: 경험/진행 게이지 (P4 엔진부)

§2.6 전체: `GameState.experience`, `OutcomeDef.experience`,
`ContentConditions.min_experience`, 번들 runtime `progression` 메타,
`ScenePage.progression`. 단위 테스트: 경험 가감·음수 방지,
min_experience 조건 매칭, progression 메타 유/무에 따른 직렬화 차이,
기존 세이브 하위호환.

### WP-W1: 타입 미러 + 번들 산출물 동기화

1. `web/src/core/types.ts`: 신규 optional 필드 미러(§2.1 — snake_case 유지).
2. `scripts/export_web_data.py`: `traits` 섹션·runtime `progression`·
   `protagonist_name` 통과 지원(DATA_FILES/스키마 반영). 아직 YAML 소스에
   신규 데이터가 없으므로 산출물 diff는 없어야 정상 — `--check` 통과 확인.
3. `render.test.ts`의 `samplePrinterPage` fixture는 optional이므로 무수정
   통과가 목표. 신규 필드를 넣은 두 번째 fixture 추가는 자유.

**검증**: `npx tsc --noEmit` + `npm test` + `export_web_data.py --check`.

### WP-W2: 드로어 스탯 표시 + 선택지 판정 표기

1. `render.ts` 드로어(상태 상세 섹션)에 `character_summary` 렌더:
   칭호+이름 한 줄(`.character-name-line`, `data-region="character"`),
   능력 6종 목록(`.ability-row`, `data-ability-id` 속성, 라벨+값 텍스트).
   character_summary가 없으면 섹션 자체 생략(오피스/프린터 데모 경로).
2. `renderActionButton`/`renderBlockedAction`: `action.check`가 있으면
   라벨 아래 줄에 `<span class="choice-check" data-ability-id="...">`
   `{ability_label} 판정 · 성공 {NN.N}%</span>` 표기. cost_text와 공존 시
   cost 먼저. 44px 터치 타깃 유지, 숫자키 계약 불변.
3. HUD 칭호: 기존 folio/각주 스트립 구조를 바꾸지 말고, 드로어 상태 섹션
   상단의 칭호+이름 라인으로만 노출(시각 재배치는 Claude 후속 step).

**테스트 수정 허용**: `render.test.ts`에 신규 assert 추가 자유. 기존
assert·QA 셀렉터는 유지.

### WP-W3: 결과 델타 로그 색 코딩

`render.ts`의 인라인 결과 로그/기록 렌더에서 로그 라인이 `+ `로 시작하면
`.result-gain`, `- `로 시작하면 `.result-loss` 클래스 부여(그 외 무클래스).
CSS는 기존 토큰만 사용: gain = `--jade`, loss = `--seal-red`. 기호가 이미
텍스트에 있으므로 색 제거 환경에서도 정보 손실 없음(invariant 9).

### WP-W4: 회차 메타 저장 + 시작 화면 카운트 (P4 메타부)

§2.7 전체. `web/src/core/storage.ts`에 meta 키 상수 + read/write 헬퍼
(스키마 검증, 파손 시 초기화). `main.ts`의 new-game 경로에서 run_count 증가,
ending 페이지 렌더 경로에서 endings_seen/achievements_seen 병합.
`startScreen.ts`에 "{run_count+1}번째 기록" + "지금까지 본 결말 {n}편" 표기
(빈 메타면 "첫 번째 기록"). 단위 테스트: 헬퍼 round-trip, 파손 JSON 복구,
중복 병합 없음.

### WP-C1: wuxia preview 번들 적용 (콘텐츠 opt-in)

1. `src/tui_adv/storypack-previews/wuxia_jianghu_pack/`의 runtime 메타에
   `protagonist_name`(이구학지 주인공 호칭 — 기존 서사에서 쓰는 호칭을 찾아
   일치시킬 것)과 `progression: { experience_target, label: "천기" }`
   (target은 현재 preview 분량 기준 도달 가능한 값으로 — 주요 인카운터
   경험 합산으로 산출) 추가.
2. 주요 스토리 인카운터 outcome에 `experience` 보상 부여(전 분기 합이
   target에 못 미치지 않게 표 계산 후). 판정 choice가 있는 인카운터는
   그대로 두면 WP-S2로 자동 확률 표기됨 — 콘텐츠 수정 불필요.
3. 특성 1쌍 정의(`traits`): 초기 칭호 1개 + 승급 칭호 1개. 기존 승급성
   인카운터 하나(무명 관련 대결 계열에서 선택)의 성공 outcome에
   `set_trait` 연결. 신규 인카운터 창작 금지 — 기존 인카운터에 붙인다.
4. `python scripts/export_web_data.py --write` → `--check` → Rust 픽스처
   동기화까지 한 커밋. `contentBundles.test.ts`의 encounter id 목록은
   변하지 않아야 한다(인카운터 추가/삭제 없음).

**테스트 수정 허용**: `json_contract.rs`/`route_parity.rs`에서 이번 콘텐츠
변경으로 값이 바뀌는 assert(로그, actions의 check 필드, progression) 반영.
**단, 도달 가능 루트/엔딩 자체가 바뀌면 안 된다** — parity 테스트가 경로
변화를 감지하면 콘텐츠 수정을 되돌려 원인 제거.

### WP-D2: 마감 — 문서/체크리스트 동기화

`docs/dev/Checklist.md`·`Development_Plan.md`에 완료 반영,
`docs/design/Progression_and_Title_Model.md`를 실제 구현과 대조(어긋나면
구현을 고친다), `docs/00_Index.md` 구현 지표 갱신 필요 시 갱신.
pytest 전체 + §0.2 전체 검증 + `qa:storybook:visual` 1회.

---

## 4. 명시적 범위 밖 (하지 말 것)

- 캐릭터 생성 화면/무작위 생성기 (`Character_Stats_and_Generator.md`는 후속).
- 전투 시스템 개편, d20/주사위 연출, 사망·기사회생, 성향 축, 로맨스,
  상점/경제/가격, 도감 목록 페이지 UI, 랭킹/온라인, 유료 재화류 일체.
- 능력치 성장(레벨업 분배)·특성의 수치 효과 — 칭호는 이번 슬라이스에서
  표시/서사 장치까지만.
- ScenePage 기존 필드 변경, 시각 리디자인, 터미널 렌더러 신규 표시,
  tsconfig/의존성 변경.
- 레퍼런스 게임 텍스트·고유명사·아트의 복제 (invariant 8).

## 5. 최종 체크리스트

- [ ] WP당 커밋 1개, 지정 순서 (D1 → S1~S4 → W1~W4 → C1 → D2).
- [ ] `cargo test --workspace` green.
- [ ] `cd web && npm test && npx tsc --noEmit && npm run build` green.
- [ ] `python scripts/export_web_data.py --check` green (C1 이후 포함).
- [ ] `./.venv/bin/python -m pytest tests/ -q` green.
- [ ] progression 메타 없는 번들(오피스)에서 ScenePage JSON이 baseline과
      동일함을 확인 (optional 전략 검증).
- [ ] 기존 세이브 JSON(trait_id/experience 없는 형태) 로드 하위호환 테스트 존재.
- [ ] 수동 플로우: 새 모험 → 판정 choice에 확률 표기 확인 → 드로어에서
      스탯/칭호 확인 → 승급 인카운터에서 칭호 교체 로그 확인 → 엔딩 →
      시작 화면 회차 카운트/결말 수 증가 확인.
- [ ] 완료/스킵 WP와 편차를 `fable_gameloop_step2_report.md`에 기록.

# 이구학지 게임 루프 확장 — 경험치 및 특성(칭호) 모델 설계

이 문서는 이구학지(및 office-family 공용) 게임 루프에 경험치(Progression) 및 캐릭터 특성/칭호(Title) 모델을 도입하는 설계 계약서이다.

## 1. ScenePage 확장 설계

`ScenePage`에 새로운 optional 필드를 도입하여 캐릭터 상태와 게임 진행상황 정보를 전달한다.

### 1.1 CharacterSummary (캐릭터 요약)
- 씬 구성 시 항상 스냅샷 형태로 플레이어의 기초 스탯 및 칭호를 제공한다.
```json
{
  "name": "당신",
  "title_label": "무명",
  "abilities": [
    { "id": "logic", "label": "논리", "value": 2 },
    { "id": "empathy", "label": "공감", "value": 2 },
    { "id": "volition", "label": "의지", "value": 2 },
    { "id": "composure", "label": "평정", "value": 2 },
    { "id": "interface", "label": "인터페이스", "value": 2 },
    { "id": "physical", "label": "신체", "value": 2 }
  ]
}
```
- **능력치 정렬 순서**: logic, empathy, volition, composure, interface, physical 순으로 항상 고정된다.
- **칭호 구성**: `title_label`이 존재하는 경우 렌더러는 `"{title_label} {name}"` 형태로 이름을 조립한다.

### 1.2 ActionCheckInfo (판정 정보)
- 주사위 굴림(2d6)이 있는 선택지(check)의 성공 확률 정보를 제공한다.
```json
{
  "ability_id": "logic",
  "ability_label": "논리",
  "success_percent": 58.3
}
```
- **성공 확률 계산 규칙**:
  - `need = difficulty - ability_value`
  - 2d6 합이 `need` 이상일 확률을 36개 분기 분포를 기준으로 계산하여 소수 첫째 자리로 반올림 표기한다.
  - `need <= 2` 이면 `100.0%`, `need > 12` 이면 `0.0%`.

### 1.3 ProgressionStatus (진행 게이지)
- 번들의 progression 메타가 활성화된 경우에 한해 현재 모험의 진행도(경험치)를 표시한다.
```json
{
  "experience": 45,
  "target": 100,
  "label": "천기"
}
```

---

## 2. 콘텐츠 데이터 모델 확장

- **Traits 정의**: 번들 상위 레벨에 `traits: Vec<TraitDef>`가 정의될 수 있다.
  - `TraitDef { id: String, name: String, description: String }`
- **set_trait**: `OutcomeDef`에 `set_trait: Option<String>`을 제공하여 특정 사건 해소 시 캐릭터의 칭호를 변경할 수 있다.
- **experience**: `OutcomeDef`에 `experience: Option<i32>`를 제공하여 결과 처리 시 경험치를 가감한다. (0 미만으로 떨어지지 않게 보정)
- **min_experience**: `ContentConditions`에 `min_experience: Option<u32>`를 추가하여 특정 경험치 이상일 때만 인카운터/선택지가 노출되게 조건을 통제한다.

---

## 3. 결과 델타 로그 표준화

결과 델타 로그는 서사적 로그 뒤에 덧붙여 개별 자원/경험/아이템의 변동 사항을 규격화된 접두사로 줄바꿈 전달한다:
- 증가: `+ {라벨} {n}`
- 감소: `- {라벨} {n}`
- 획득: `+ {아이템명}`
- 상실: `- {아이템명}`

---

## 4. 로컬 회차 메타데이터 (Local Run Metadata)

웹 플레이어는 로컬 스토리지에 `{ schema_version: 1, run_count: u32, endings_seen: string[], achievements_seen: string[] }` 구조의 로컬 메타를 유지한다.
- 새 게임 시작 시 `run_count`가 증가한다.
- 엔딩에 도달할 때마다 해당 엔딩 ID와 해금된 업적들이 기록에 누적된다.

---

## 5. 게임 루프 2차 확장 (Slice 2) 설계 사양

### 5.1 콘텐츠 소유 라벨링 (Content-Owned Labels)

아이템 및 업적 등의 디스플레이용 한글 라벨(명칭)은 콘텐츠 번들 내 데이터(`ItemDef.name`, `AchievementDef.name`)를 기준으로 단일화한다. 웹 렌더러가 하드코딩된 사전(`labels.ts`)에 의존하는 설계를 탈피하여, `ScenePage`에 라벨 정보를 담아 내려주도록 확장한다.

#### 5.1.1 ScenePage 추가 필드
```json
{
  "content_labels": {
    "items": [
      { "id": "iron_sword", "label": "철검" }
    ],
    "achievements": [
      { "id": "first_kill", "label": "첫 번째 승리" }
    ]
  }
}
```
- **Rust 정의**: `ScenePage.content_labels: Option<ContentLabels>`
  - `ContentLabels { items: Vec<LabeledId>, achievements: Vec<LabeledId> }`
  - `LabeledId { id: String, label: String }`
- **라벨 매핑 규칙**:
  - `ScenePage` 생성 시점의 `inventory_summary.items` 목록 및 `achievement_summary.unlocked/newly_unlocked`에 포함된 ID 목록을 기준으로 `ContentIndex`를 조회하여 명칭을 가져온다.
  - 번들 인덱스에 매핑 데이터가 없는 특수 ID인 경우 매핑 생략 가능.
- **웹 렌더러 조회 우선순위**:
  1. `ScenePage.content_labels` 내의 라벨 정의
  2. 웹 렌더러 로컬 `labels.ts` 내의 하드코딩 딕셔너리 (레거시/하위 호환성용 폴백)
  3. ID 문자열 휴머나이즈 + `" (미번역)"` 접사 폴백

### 5.2 판정 연출 및 상세 결과 노출 (Check Resolution Reveal)

2d6 판정(체크 선택지)의 해소 상세 내역(주사위 값, 스탯 값, 보정치 합산, 난이도 목표, 최종 결과)을 다음 씬 페이지에 구조화된 데이터로 전달하고, 화면 상단 배너에 시각적으로 노출한다.

#### 5.2.1 데이터 모델 구조
- **GameState (Rust/serde default)**:
  `pub last_check: Option<CheckResolution>`
  - 매 turn의 `apply_action` 시작 시 `None`으로 초기화되며, 체크 판정이 해소될 때 결괏값을 생성해 보존한다.
- **ScenePage (Rust/Option)**:
  `pub check_result: Option<CheckResolution>`
  - 플레이어가 액션을 선택하여 씬이 재생성될 때 `GameState.last_check`를 스냅샷 형태로 복사하여 전달한다.
- **CheckResolution 구조**:
  ```json
  {
    "ability_id": "logic",
    "ability_label": "논리",
    "dice": [4, 2],
    "ability_value": 2,
    "difficulty": 9,
    "total": 8,
    "success": false
  }
  ```

#### 5.2.2 렌더러 상단 배너 스키마
- 주사위 굴림 정보는 `.storybook-body` 상단, 서사적 로그(`.story-result-log`) 바로 위에 HTML 구조체로 배치한다:
  ```html
  <aside class="check-resolution" data-region="check-result" data-check-outcome="success|failure" data-ability-id="logic">
    <span class="check-resolution__dice" aria-hidden="true">⚃ ⚁</span>
    <span class="check-resolution__math">2d6 (4+2) +논리 2 = 8 / 목표 9</span>
    <span class="check-resolution__verdict">실패</span>
  </aside>
  ```
- 주사위 눈금 1~6은 ⚀⚁⚂⚃⚄⚅ 유니코드 글자로 맵핑한다.
- 웹 접근성을 위해 `aria-label="판정 결과: 실패"` 등의 설명을 부모 태그에 부착하고, 유니코드 주사위는 `aria-hidden="true"`로 숨긴다.

### 5.3 붕괴 게이트 및 기사회생 (Collapse Gate & Second Wind)

체력이 0 이하가 되었을 때 즉시 모험을 끝내지 않고, 회차당 단 1회에 한해 "기사회생"의 기회를 부여하는 시스템을 범용적인 번들 메타와 인카운터 제어로 구현한다.

#### 5.3.1 번들 메타 설정
```yaml
runtime:
  collapse:
    encounter_id: wuxia_collapse_gate
    resource_id: health
    used_flag: second_wind_used
```
- 번들 데이터 검증 시 `collapse.encounter_id`에 해당하는 인카운터가 데이터베이스에 실재하는지, `resource_id`가 `"health"`로 설정되어 있는지, `used_flag`가 유효한 비어있지 않은 문자열인지 검증한다.

#### 5.3.2 게임 엔진 처리 규칙
- `turn.rs` 등에서 결과 액션 및 자원 변동 처리가 끝난 직후, 플레이어 체력이 `0` 이하이고 다음 조건들이 충족될 경우 강제로 다음 이동 타겟 인카운터를 `collapse.encounter_id`로 우회시킨다:
  1. `collapse` 메타가 번들에 설정되어 있음.
  2. `player.health <= 0` 임.
  3. `used_flag` 플래그가 플레이어 `GameState.flags` 목록에 없음.
  4. 현재 플레이 중인 인카운터 자체가 `collapse.encounter_id`가 아님 (무한 루프 방지).
- 조건을 만족하여 강제 우회가 발동하면, 일반적인 엔딩 분기 조건(체력 0으로 인한 일반 사망 등) 검사는 스킵한다.
- `used_flag`가 세팅된 두 번째 0 이하 체력 도달 시에는 우회가 발동하지 않고 일반적인 데스 엔딩 판정(accept_final_rest 플래그를 통한 사망 엔딩 분기)을 거치게 된다.

---

## 6. 게임 루프 3차 확장 (Slice 3) — 레벨링·기연·아이템 상세

이 절은 `fable_gameloop3_step1_2607161330.md`의 D1 설계 계약이다. 목표는 플레이어가
판정에 영향을 주는 성장(레벨링)과 기연을 이해하고, 드로어에서 아이템과 칭호의 의미를
확인할 수 있게 하는 것이다. 수치 계산과 소유권은 Rust GameCore가 담당하고 Web
Storybook/terminal은 `ScenePage`를 표시한다.

### 6.1 레벨링과 수련 포인트 (Leveling)

번들 `runtime` 메타에 선택적으로 `leveling.thresholds: [u32, ...]`를 둔다. 배열은 비어
있지 않고 엄격히 오름차순이어야 하며, 각 값은 현재 경험치에서 수련 포인트 1점을
얻는 문턱이다.

```yaml
runtime:
  leveling:
    thresholds: [30, 80]
```

- `GameState.spent_stat_points: u32`는 `#[serde(default)]`로 저장한다. 획득 포인트는
  현재 경험치가 넘은 문턱의 개수로 매번 계산하고, 사용 가능 포인트는
  `earned.saturating_sub(spent)`로 계산한다. 별도의 earned 카운터를 저장하지 않는다.
- 수련 액션은 `train:{ability_id}`이며 고정 6능력치(logic, empathy, volition,
  composure, interface, physical)만 허용한다. 메타가 없거나 포인트가 없거나 현재
  능력치가 **5 cap**이면 무효다.
- 유효한 수련은 능력치를 1 올리고 `spent_stat_points`를 1 증가시키며
  `+ {능력치 라벨} 수련 1` 델타 로그를 남긴다. 수련은 같은 턴에 처리하고 턴을
  진행하거나 인카운터/위험도/엔딩을 다시 판정하지 않는다.
- 결과의 경험치 획득으로 하나 이상의 문턱을 처음 넘으면
  `+ 수련 기회 {n}` 로그를 추가한다. 이 로그는 기존 결과 비트/플로트가 표시한다.
- `CharacterSummary.stat_points`는 항상 직렬화하며 사용 가능한 포인트만 담는다.
  포인트가 0이면 Web 드로어에 배지나 `+` 버튼을 렌더링하지 않는다. 기존 번들에
  leveling 메타가 없을 때는 액션과 필드가 기존 동작을 바꾸지 않아야 한다.

### 6.2 기연 (Insights)

번들 상위 레벨의 선택적 `insights` 배열은 사건의 결과로 얻는 영구 판정 보정을
정의한다. 각 항목은 고유한 `id`를 가져야 한다.

```yaml
insights:
  - id: cheongryu_heart_method
    name: 청류심법 입문
    description: 숨을 고르는 법이 판정의 바닥을 받친다.
    check_bonus:
      ability: composure
      bonus: 1
```

- `InsightDef { id, name, description, check_bonus }`의 `check_bonus.ability`는 6능력치
  중 하나이고 `bonus`는 1..=2다. 잘못된 참조나 중복 id는 번들 검증에서 거부한다.
- `OutcomeDef.add_insights: Vec<String>`가 기존 id를 가리키면 결과 처리 시
  `GameState.insights: Vec<String>`에 한 번만 추가한다. 중복 보상은 무시한다.
  획득 시 `+ 기연: {name}` 델타 로그를 남긴다.
- `insight_bonus(state, content, ability)`는 보유한 기연 중 해당 능력치의 보정치를
  합산하는 순수 조회 함수다. 판정의 최종 합은 `2d6 + ability_value + insight_bonus`이고,
  사전 표시 확률도 같은 유효 능력치로 계산한다. 주사위 굴림 해시는 보정치를
  포함하지 않아 동일 seed/turn의 주사위가 바뀌지 않는다.
- `CheckResolution.insight_bonus: i32`는 실제 적용된 합산값을 전달한다.
  `ScenePage.insights: Vec<InsightStatus>`는 id/name/description/effect_text를 담고,
  보정이 없으면 `effect_text`를 빈 문자열로 둔다. 두 필드는 빈 경우 optional/생략해
  office 등 기존 JSON의 바이트 동일성을 보존한다.
- Web 드로어에는 인물과 소지품 사이에 `기연` 섹션을 둔다. 각 행은 이름과
  `effect_text`를 보여 주고 탭하면 설명을 펼친다. 목록이 비어 있으면
  `아직 맺은 기연이 없다.`를 표시한다. 판정 배너와 결과 비트의 수학 줄에는
  ` +기연 {n}`을 덧붙여 보정의 출처를 숨기지 않는다.

### 6.3 아이템 상세 (Item Details)

`ScenePage.inventory_details: Vec<ItemDetail>`은 현재 `inventory_summary.items`에
있는 id만, 동일한 순서로 제공한다. 각 상세는 `id`, `name`, `description`, `item_type`,
`usable`을 포함하며 빈 목록은 직렬화하지 않는다. 설명은 ContentIndex에서 가져오고,
Web이 번들을 직접 읽어 재판정하지 않는다.

- 드로어의 소지품 행은 disclosure 버튼이다. 각 행에는 결정적 색조를 가진
  `data-item-icon="{id}"` 픽셀 아이콘 placeholder가 있어 이후 CSS/asset만으로 실제
  스프라이트를 교체할 수 있다.
- 펼친 상세에는 설명을 표시한다. 해당 페이지에 `use:{id}` 액션이 있으면 `[사용]`
  버튼을 같은 action-id wiring으로 제공한다. 아이템이 usable이어도 이번 턴에 액션이
  없으면 `지금은 쓸 수 없다` disabled 상태를 표시한다.
- 펼침은 DOM disclosure만으로 처리하며 core 턴을 진행하지 않는다. 사용을 누르면
  기존 `use:{id}` 경로로 페이지가 다시 렌더링되고 결과 비트를 재생한다.
- 버리기/폐기/드롭 버튼은 이 모델에 포함하지 않는다. 아이템의 효과 적용과 소비
  여부는 기존 Rust `ItemDef.use_effects`/`usable` 규칙을 그대로 따른다.

### 6.4 판정 결과와 성장 보정의 표시 계약

기존 §5.2의 `CheckResolution` 예시에 `insight_bonus`를 추가한다. `ability_value`는
보정 전 캐릭터 스탯이며, `total`은 다음 식으로 계산한 실제 합이다.

```json
{
  "ability_id": "composure",
  "ability_label": "평정",
  "dice": [4, 2],
  "ability_value": 2,
  "insight_bonus": 1,
  "difficulty": 9,
  "total": 9,
  "success": true
}
```

Web 수학 줄은 `2d6 (4+2) +평정 2 +기연 1 = 9 / 목표 9`처럼 원시 스탯과
기연 보정을 분리해서 표시한다. `insight_bonus == 0`이면 `+기연` 조각을 생략한다.
이 계약은 판정 확률/결과를 renderer가 다시 계산하지 않도록 하며, Rust가 제공한
`success_percent`, `total`, `insight_bonus`를 그대로 노출한다.

### 6.5 저장·호환성·검증

- 새 `GameState` 필드(`spent_stat_points`, `insights`)와 새 `ScenePage` 필드는
  `serde(default)`/optional로 정의한다. `SAVE_SCHEMA_VERSION`과
  `CONTENT_BUNDLE_SCHEMA_VERSION`은 1을 유지하고, 옛 저장 파일은 누락 필드를
  기본값으로 읽는다.
- leveling/insights 메타가 없는 office 번들의 ScenePage JSON은 새 빈 필드를
  생략하여 기존 바이트와 동일해야 한다. item 상세도 inventory가 비어 있으면
  생략한다.
- D1 이후 구현 순서는 `W1 → S1 → S2 → S3 → W2 → W3 → W4 → C1 → D2`이며,
  각 WP마다 `cargo test --workspace`, 필요한 exporter `--check`, 그리고 Web의
  `vitest`, `tsc --noEmit`, `build`를 실행한다. 최종에는 WASM을 재빌드하고
  `qa:storybook:visual --require-wasm` 5개 뷰포트를 통과시킨다.

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

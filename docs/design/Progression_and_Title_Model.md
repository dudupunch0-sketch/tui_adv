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

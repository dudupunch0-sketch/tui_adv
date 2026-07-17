# Event / Stage / Content Block 모델

Status: canonical content grammar. Event 내부 서사 진행 단위와 표현 순서의 의미는 이 문서가 소유한다. 저장 형식은 `docs/dev/Data_Schema.md`, 화면 배치는 `docs/design/Mobile_Ink_Storybook_UI.md`가 소유한다.

## 목적

플레이 화면을 `제목 / 천기록 / 결과 / 일러스트 / 선택지`라는 고정 칸으로 해석하지 않는다. 하나의 사건은 여러 이야기, 선택, 결과와 그림이 시간 순서대로 이어지는 흐름이며, renderer는 그 순서를 보존한다.

## Canonical 용어

- **Event**: 플레이어가 경험하는 하나의 완결된 사건 단위. 하나 이상의 Stage를 순서대로 포함한다.
- **Encounter**: 조건·가중치·재등장 규칙에 따라 Event를 시작시키는 engine/runtime container. Event의 동의어가 아니며 eligibility, selection, seen 처리를 소유한다.
- **Stage**: Event 안에서 현재 진행 의미가 같은 구간. `StoryStage`, `ChoiceStage`, `ResultStage` 중 하나다.
- **StoryStage**: 상황, 행동, 대화, 서술을 전달하는 구간. 선택 없이 다음 StoryStage로 이어질 수 있다.
- **ChoiceStage**: 플레이어가 하나의 분기를 고르는 구간. 하나 이상의 `Choice`를 포함한다.
- **ResultStage**: 바로 앞 ChoiceStage에서 고른 선택의 크고 작은 결과를 보여주는 구간. 수치 변화뿐 아니라 후속 서술·대화를 포함할 수 있고, 뒤에 StoryStage 또는 ChoiceStage가 이어질 수 있다.
- **ContentBlock**: StoryStage 또는 ResultStage 안에서 순서를 갖는 표현 단위. 최소 taxonomy는 `narration`, `dialogue`, `illustration`, `document`, `system`, `result_summary`다. world 전용 표현은 이 공통 의미를 깨지 않는 확장 block으로 둔다.
- **IllustrationBlock**: 고정 그림 칸이 아니라 ordered content 안에 삽입되는 그림 block.

ResultStage 안에 작은 StoryStage를 중첩하지 않는다. ResultStage가 `narration`·`dialogue` 같은 이야기 block을 직접 포함하게 하여 진행 cursor와 authoring 의미를 명확히 한다.

## 구조와 순서 계약

```text
Event
├─ StoryStage
│  ├─ NarrationBlock
│  ├─ IllustrationBlock
│  └─ DialogueBlock
├─ StoryStage
├─ ChoiceStage
├─ ResultStage
│  ├─ ResultSummaryBlock
│  ├─ NarrationBlock
│  └─ IllustrationBlock
├─ ChoiceStage
└─ ResultStage
```

- Event에는 StoryStage와 ChoiceStage가 각각 하나 이상 있어야 한다.
- 모든 ChoiceStage의 각 선택은 그 선택에 대응하는 ResultStage로 이어져야 한다. 결과가 짧아도 생략하지 않는다.
- StoryStage 수가 ChoiceStage 수보다 많거나 같은 구성을 기본 authoring pattern으로 권장하지만 validation 오류로 강제하지 않는다.
- Stage와 ContentBlock 배열 순서가 곧 표현 순서다. renderer가 종류별 고정 영역으로 재정렬해서는 안 된다.
- 한 Event 안에서 이야기와 선택은 여러 번 교차할 수 있다. ChoiceStage에서만 입력을 기다리고, 그 전후의 ordered content는 연속해서 읽힌다.
- 기존 단일 본문/선택지 encounter는 migration 기간에 adapter로 `StoryStage → ChoiceStage → ResultStage` Event로 해석할 수 있다. Encounter selection 의미는 유지한다.

## ResultStage branch blocks

checked choice의 성공·실패에 따라 결과 서술이 달라질 때는 ResultStage의 ordered
ContentBlock에 optional `branch`를 붙인다.

```text
ResultStage
├─ ResultSummaryBlock          (branch 없음: 항상 표시)
├─ NarrationBlock              (branch: success)
└─ NarrationBlock              (branch: failure)
```

- `branch: success`와 `branch: failure`는 바로 앞 ChoiceStage의 check resolution을 뜻한다.
- `branch`가 없는 block은 항상 표시하고, 분기 block은 일치하는 resolution일 때만 표시한다.
- check가 없는 선택이거나 resolution을 확인할 수 없으면 공통 block만 표시한다.
- 필터링 뒤에도 남은 block의 원래 순서를 유지한다. renderer는 `content_stream`을 받은 뒤 branch를 다시 판정하지 않는다.
- ResultStage 안에서 branch block을 사용하더라도 작은 StoryStage를 중첩하지 않는다. 결과의 공통 요약과 성공·실패별 narration/dialogue를 같은 ResultStage의 block 배열에 직접 둔다.

## 일러스트 계약

- Event는 하나 이상의 illustration slot을 갖는 것을 기본 콘텐츠 계약으로 한다. 일반 Event는 대부분 1개, 보스전·특별 Event는 최대 3개를 권장한다.
- IllustrationBlock은 StoryStage 또는 ResultStage의 위, 중간, 끝 어디에도 올 수 있다. 별도의 고정 `일러스트 영역`은 없다.
- 제작 중 에셋이 없더라도 block을 삭제하지 않는다. renderer는 등록되지 않은 `visual_id`를 안전한 placeholder와 event 이름 기반 alt text로 표시한다.
- `이벤트 이름.png` 같은 가짜 경로를 실제 에셋으로 저장하지 않는다. stable `visual_id`, `alt`, placeholder 상태를 데이터로 표현하고 나중에 manifest asset을 연결한다.
- 1개 이상/최대 3개는 콘텐츠 작성 기본값이다. 접근성 전용 무삽화 Event나 더 많은 그림이 필요한 예외는 명시적 설계 검토를 거친다.

## 천기록 surface

`천기록`은 모든 이야기 본문의 speaker나 고정 칸 이름이 아니다. 기본 서술은 제목 없는 이야기 흐름 또는 실제 화자/문서 이름으로 표시한다.

천기록은 다음처럼 세계관 장치가 실제로 개입할 때만 `document` 또는 storypack 확장 `cheongirok` block/surface로 나타난다.

- 새 기록·기록의 쪽·천외편린·귀환 단서를 제시할 때
- 기록자의 시점, 실시간 필사, 문서의 반응을 연출할 때
- 천기록 자체의 존재가 장면 의미인 특별 Event일 때

이 특수 surface는 범용 Event/Stage 구조를 대체하지 않는다. office 등 다른 storypack은 같은 block 위치에 자기 세계의 문서·단말 surface를 사용할 수 있어야 한다.

## 콘텐츠 작성 체크리스트

- 사건의 시작과 끝이 Event 경계로 설명되는가?
- 각 StoryStage, ChoiceStage, ResultStage의 역할이 섞이지 않았는가?
- 각 Choice가 대응 ResultStage를 가지며 결과의 서사와 상태 변화를 함께 설명하는가?
- block 순서만 읽어도 의도한 이야기·그림·선택 순서가 재현되는가?
- illustration slot 1개 이상을 계획했고 미완성 asset도 placeholder로 남겼는가?
- `천기록` 표기가 실제 천기록 개입 장면에만 사용되는가?

## 문서 책임

- 이 문서: 용어, Stage 문법, ordered content 의미, illustration/천기록 authoring 원칙
- `docs/dev/Data_Schema.md`: YAML/JSON/Rust 표현, cursor, validation, migration
- `docs/design/Mobile_Ink_Storybook_UI.md`: Web Storybook DOM/layout/rendering
- `docs/design/Storypack_Encounter_DB.md`: 후보 상황 카드에서 Encounter/Event로 승격하는 workflow
- `docs/content/encounter_db/README.md`: 콘텐츠 작성자용 승격 전 체크리스트
- `docs/dev/Development_Methodology.md`: schema·renderer·bundle·test·docs 동기화 절차

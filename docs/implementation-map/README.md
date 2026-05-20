# 구현 지도 업데이트 방법

`index.html`은 단일 진입점이다. 내용은 카테고리별 JS 파일에서 온다.

- `data/overview.js`: 지표, 현재 상태, 다음 작업, 업데이트 규칙
- `data/systems.js`: 엔진, TUI, 저장, 안전, 테스트 기능 카드
- `data/routes.js`: 루트 그래프 노드, 원본 YAML 기준 노드별 배경/대사/전체 선택지/다음 상황, 루트별 smoke 흐름
- `data/content.js`: 위치, 인카운터, 엔딩, 아이템, 업적 목록
- `assets/app.js`: 검색, 필터, 노드 클릭 상호작용
- `assets/styles.css`: Notion식 위키 레이아웃과 디자인 토큰

새 구현 slice 후 체크리스트:

1. 새 기능이 시스템 성격이면 `data/systems.js`에 카드 추가.
2. 새 루트나 엔딩이면 `data/routes.js`의 `nodes`, `scene.choices`, `links`, `routes`를 함께 갱신.
3. YAML 콘텐츠 수량이나 이름이 바뀌면 `data/content.js`와 `data/overview.js` 지표 갱신.
4. 출처 파일이 바뀌면 카드의 `files` 항목을 갱신.
5. 브라우저에서 검색, 필터, 노드 클릭을 확인.

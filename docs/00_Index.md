# escape from the office 문서 인덱스

이 문서는 `tui_adv` 프로젝트의 문서 계층과 각 문서의 책임 범위를 정의한다.
현재 실제 구현 기준 문서와 데이터 파일만 “현재 생성된 문서”로 나열한다.

## 현재 핵심 컨셉

- 장르: TUI 기반 랜덤 인카운터 선택지 생존 게임
- 기본 배경: 회사 사무실 아포칼립스(`office_apocalypse`)
- 개발 방향: storypack/world 기반 선택지 생존 엔진. 기본 office isolation 계열을 유지하되, office-family 후보 `야근몽`(`office_dream`)과 첫 비-office 기준팩 **이구학지 — 천기록**(`wuxia_jianghu`)을 후보 DB로 관리한다.
- 핵심 자원: 체력, 정신력, 배터리, 허기, 갈증
- 주요 목표: 탈출, 정복, 진실 발견, 히든 현실 연결 루트
- 특별 요소: 게임 속 단서를 통해 실제 사무실에 숨겨진 메모/보물 위치를 자연스럽게 안내하는 ARG식 현실 연결
- 활성 렌더러 방향: Rust GameCore 공통 + Web Storybook/GlyphFX primary UX + SuperLightTUI terminal renderer/fallback
- 주의: 기존 Python/Textual, TypeScript mirror core, browser fake-TUI는 전환기 legacy/parity surface이며 새 게임 규칙의 장기 소유자는 아니다.

## 현재 구현 지표

| 항목 | 수 |
|---|---:|
| 위치 YAML | 16 |
| 아이템 YAML | 13 |
| 인카운터 YAML | 20 |
| 엔딩 YAML | 13 |
| 업적 YAML | 11 |
| 공개 현실 힌트 예시 | 3 |

## 문서 계층

```text
docs/
  00_Index.md                       # 전체 문서 목차와 문서 운영 규칙
  01_Game_Overview.md               # 게임 한 줄 설명, 핵심 판타지, 목표 요약

  story/
    Story.md                        # 메인 서사와 시작 상황
    Disaster_Cause.md               # 재난 원인과 진실 루트 공개-safe 설계
    Disaster_Type_Extension.md      # 재난 타입 확장 규칙과 후보 백로그
    Dream_Ending_Branching.md       # 꿈 엔딩/깨어난 뒤 재시험 문법 설계 후보
    Real_Escape_Ending_Branching.md # 현실 탈출 후 결과 정산/후일담 설계 + active first slice contract
    Reality_Link.md                 # 현실세계 연결 원칙. 실제 위치는 넣지 않음

  design/
    Game_Loop.md                    # 턴 진행, 인카운터, 선택지 처리 루프
    Combat_System_Auto_Brawl.md     # 자동 난투 + 상황 개입 전투 설계 후보
    Player_State.md                 # 체력/정신력/배터리/허기/갈증 규칙
    Character_Stats_and_Generator.md # 6스탯 등장인물/LLM 생성기 설계 후보
    Storypack_World_Model.md        # office-only 편향을 줄이는 world/storypack 일반화 기준
    Storypack_Encounter_DB.md       # 스토리팩/인카운터 상황 카드/NPC DB 설계
    Map.md                          # 사무실 위치, 연결, 구역별 역할
    UI_Rules.md                     # 사내 시스템형 TUI, 글리치, 선택지 오염 규칙
    TUI_Storybook_GlyphFX_Concept.md # Web primary UX로 채택한 TUI풍 스토리북 + GlyphFX 방향
    Mobile_Pixel_Storybook_UI.md     # 모바일 세로형 픽셀 게임북 board UI contract

  content/
    Location_List.md                # 런타임 위치 목록
    Item_List.md                    # 런타임 아이템 목록
    Encounter_List.md               # 런타임 인카운터 목록
    Ending_List.md                  # 런타임 엔딩 목록
    Survivor_System_Routes.md       # 생존자 설득과 시스템 제압 루트 설계
    Secret_List.md                  # 게임 내 비밀 목록. 실제 현실 위치 제외
    Horror_Ideas.md                 # 호러 연출 아이디어 저장소
    storypacks/                     # 스토리팩 후보 문서 DB
      yageunmong_pack.md            # 야근몽 office-dream 후보 storypack
    characters/                     # 6스탯 반복 등장인물 후보 DB
    encounter_db/                   # 런타임 승격 전 인카운터 상황 카드 문서 DB
      yageunmong_pack.md            # 야근몽 후보 상황 카드
    storypack_db/                   # storypack/card 후보의 machine-readable JSON DB

  runtime preview sources/
    src/tui_adv/storypack-previews/wuxia_jianghu_pack/ # wuxia_commute_rift_arrival preview source YAML
    crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json # Rust/GameCore preview fixture
    web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json # Web/WASM preview bundle

  dev/
    Development_Plan.md             # canonical main plan: 현재 방향, 다음 작업, 우선순위
    Checklist.md                    # 완료 여부 추적용 체크리스트
    Storypack_Runtime_Preview_Mode.md # non-office runtime prototype preview mode 결정
    Architecture.md                 # 코드 구조와 모듈 경계
    Rust_Core_Dual_Renderer_Architecture.md # Rust GameCore + Web Storybook + SuperLightTUI terminal 활성 방향
    Data_Schema.md                  # YAML/JSON 데이터 스키마 + renderer-neutral ScenePage/WASM contract
    TUI_Layout.md                   # 화면 배치, 입력, 위젯 설계
    Save_Slot_UX.md                 # 저장 슬롯 이름 변경 UX 후보
    Balance_QA_Packaging.md         # 밸런싱, QA smoke, 패키징/릴리즈 기준
    Web_Distribution_Decision.md    # Web-only 배포 표면과 Tauri/Electron defer 결정
    Web_Player_PokeRogue_Style_Plan.md # 포켓로그식 URL 즉시 플레이 Web player 배포/UX 구현 계획
    Final_QA_Log.md                 # 실제 Textual/터미널 크기/10회 새 게임 QA 기록
    Reality_Secret_Safety_Checklist.md # 로컬 현실 힌트 안전 점검

  templates/
    local-secrets.template.yaml     # private/secrets.local.yaml 작성용 공개 안전 템플릿

  implementation-map/
    index.html                      # 현재 구현을 한 번에 보는 interactive HTML 구현 지도
    README.md                       # 구현 지도 업데이트 방법
    assets/                         # 구현 지도 CSS/JS
    data/                           # 카테고리별 구현 지도 데이터

  archive/
    idea_0515.md                    # 2026-05-15 원본 아이디어 노트

private/
  secrets.local.yaml                # 실제 사무실 위치. .gitignore로 커밋 차단
```

## 문서 작성 원칙

1. `story/`는 서사와 분위기만 다룬다. 구현 세부사항은 넣지 않는다.
2. `design/`은 게임 규칙과 시스템 설계를 다룬다. 실제 코드 구조는 넣지 않는다.
3. `content/`는 실제 YAML 데이터 목록을 공개 문서 형태로 설명한다.
4. `dev/`는 구현 계획, 코드 구조, 데이터 스키마, 테스트·검증 방식을 다룬다.
5. `private/`에는 실제 사무실 위치와 보물 위치만 둔다. 이 정보는 공개 저장소에 커밋하지 않는다.
6. 현실 연결 힌트는 단계적으로 공개한다. 위험한 장소, 개인 책상, 잠긴 공간, 전기설비 근처는 사용하지 않는다.
7. 새 런타임 콘텐츠를 추가하면 `src/tui_adv/data/*.yaml`, `docs/content/*.md`, `docs/implementation-map/data/*.js`를 함께 갱신한다.

## 현재 생성된 문서

- `docs/00_Index.md`
- `docs/01_Game_Overview.md`
- `docs/story/Story.md`
- `docs/story/Disaster_Cause.md`
- `docs/story/Disaster_Type_Extension.md`
- `docs/story/Dream_Ending_Branching.md`
- `docs/story/Real_Escape_Ending_Branching.md`
- `docs/story/Reality_Link.md`
- `docs/design/Player_State.md`
- `docs/design/Character_Stats_and_Generator.md`
- `docs/design/Storypack_World_Model.md`
- `docs/design/Storypack_Encounter_DB.md`
- `docs/design/Game_Loop.md`
- `docs/design/Combat_System_Auto_Brawl.md`
- `docs/design/Map.md`
- `docs/design/UI_Rules.md`
- `docs/design/TUI_Storybook_GlyphFX_Concept.md`
- `docs/design/Mobile_Pixel_Storybook_UI.md`
- `docs/content/Location_List.md`
- `docs/content/Item_List.md`
- `docs/content/Encounter_List.md`
- `docs/content/Ending_List.md`
- `docs/content/Survivor_System_Routes.md`
- `docs/content/Secret_List.md`
- `docs/content/Horror_Ideas.md`
- `docs/content/storypacks/README.md`
- `docs/content/storypacks/isolation_pack.md`
- `docs/content/storypacks/yageunmong_pack.md`
- `docs/content/storypacks/wuxia_jianghu_pack.md`
- `docs/content/characters/README.md`
- `docs/content/characters/recurrent_npcs.md`
- `docs/content/encounter_db/README.md`
- `docs/content/encounter_db/isolation_pack.md`
- `docs/content/encounter_db/yageunmong_pack.md`
- `docs/content/encounter_db/wuxia_jianghu_pack.md`
- `docs/content/storypack_db/README.md`
- `docs/content/storypack_db/storypacks.json`
- `docs/content/storypack_db/encounter_situations.json`
- `src/tui_adv/storypack-previews/wuxia_jianghu_pack/`
- `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`
- `docs/dev/Architecture.md`
- `docs/dev/Rust_Core_Dual_Renderer_Architecture.md`
- `docs/dev/Data_Schema.md`
- `docs/dev/TUI_Layout.md`
- `docs/dev/Save_Slot_UX.md`
- `docs/dev/Balance_QA_Packaging.md`
- `docs/dev/Web_Distribution_Decision.md`
- `docs/dev/Web_Player_PokeRogue_Style_Plan.md`
- `docs/dev/Final_QA_Log.md`
- `docs/dev/Reality_Secret_Safety_Checklist.md`
- `docs/dev/Development_Plan.md`
- `docs/dev/Checklist.md`
- `docs/dev/Storypack_Runtime_Preview_Mode.md`
- `docs/templates/local-secrets.template.yaml`
- `docs/implementation-map/index.html`
- `docs/implementation-map/README.md`
- `docs/archive/idea_0515.md`

## 다음 문서/정리 후보

현재 없는 문서명을 인덱스에 현재 문서처럼 나열하지 않는다.
다음 주제가 커지면 별도 문서로 분리하고, 그때 이 인덱스와 implementation-map도 같이 갱신한다.

1. `document_contamination_pack`, `meeting_reservation_pack`, `compensation_strike_pack`의 별도 후보 문서
2. `wuxia_jianghu_pack` / 이구학지 — 천기록의 후속 preview slice: `wuxia_heuksa_bang_first_fight` 구현 완료 후 `preview launcher/UI wiring` opt-in UX 필요 여부 결정
3. `yageunmong_pack` / 야근몽의 첫 runtime preview 후보: `yageunmong_late_night_desk_awake` 또는 각성편린 3택 preview
4. 실시간 UI/UX 점검 후 확정된 화면/입력 변경 사항 기록

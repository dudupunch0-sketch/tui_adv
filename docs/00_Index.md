# escape from the office 문서 인덱스

이 문서는 `tui_adv` 프로젝트의 문서 계층과 각 문서의 책임 범위를 정의한다.

## 현재 핵심 컨셉

- 장르: TUI 기반 랜덤 인카운터 선택지 생존 게임
- 배경: 회사 사무실 아포칼립스
- 핵심 자원: 체력, 정신력, 배터리, 허기, 갈증
- 주요 목표: 탈출, 정복, 진실 발견, 히든 현실 연결 루트
- 특별 요소: 게임 속 단서를 통해 실제 사무실에 숨겨진 메모/보물 위치를 자연스럽게 안내하는 ARG식 현실 연결

## 문서 계층

```text
docs/
  00_Index.md                       # 전체 문서 목차와 문서 운영 규칙
  01_Game_Overview.md               # 게임 한 줄 설명, 핵심 판타지, 목표 요약

  story/
    Story.md                        # 메인 서사와 시작 상황
    World.md                        # 회사 아포칼립스 세계관 규칙
    Disaster_Types.md               # 좀비, 외계인, 코스믹 호러, 백룸 등 재난 타입
    Factions.md                     # 생존자, 사내 시스템, 괴현상 세력
    Endings.md                      # 탈출/정복/진실/히든 엔딩 설계
    Hidden_Routes.md                # 숨겨진 루트와 조건 체인
    Reality_Link.md                 # 현실세계 연결 원칙. 실제 위치는 넣지 않음

  design/
    Game_Loop.md                    # 턴 진행, 인카운터, 선택지 처리 루프
    Player_State.md                 # 체력/정신력/배터리/허기/갈증 규칙
    Resources.md                    # 자원 간 상호작용과 임계치 효과
    Map.md                          # 사무실 위치, 연결, 구역별 역할
    Encounters.md                   # 인카운터 설계 규칙
    Choices.md                      # 선택지 비용, 조건, 성공/실패 규칙
    Items.md                        # 아이템 설계 규칙
    Flags.md                        # 플래그, 단서, 히든 루트 조건
    Balance.md                      # 난이도와 자원 밸런스 기준

  content/
    Encounter_List.md               # 실제 인카운터 목록
    Item_List.md                    # 실제 아이템 목록
    Location_List.md                # 실제 위치 목록
    Ending_List.md                  # 실제 엔딩 목록
    Secret_List.md                  # 게임 내 비밀 목록. 실제 현실 위치 제외

  dev/
    Development_Plan.md             # 전체 개발 계획
    Checklist.md                    # 단계별 체크리스트
    Architecture.md                 # 코드 구조와 모듈 경계
    Data_Schema.md                  # YAML/JSON 데이터 스키마
    TUI_Layout.md                   # 화면 배치, 입력, 위젯 설계
    Save_Load.md                    # 저장/불러오기 설계
    Testing.md                      # 테스트 전략과 검증 명령
    Roadmap.md                      # 마일스톤 단위 로드맵

private/
  Real_Office_Secrets.local.md      # 실제 사무실 위치. .gitignore로 커밋 차단
```

## 문서 작성 원칙

1. `story/`는 서사와 분위기만 다룬다. 구현 세부사항은 넣지 않는다.
2. `design/`은 게임 규칙과 시스템 설계를 다룬다. 실제 코드 구조는 넣지 않는다.
3. `content/`는 실제 데이터 목록을 관리한다. 인카운터/아이템/위치/엔딩은 가능하면 데이터 파일로 옮길 수 있게 쓴다.
4. `dev/`는 구현 계획, 코드 구조, 테스트, 검증 방식을 다룬다.
5. `private/`에는 실제 사무실 위치와 보물 위치만 둔다. 이 정보는 공개 저장소에 커밋하지 않는다.
6. 현실 연결 힌트는 단계적으로 공개한다. 위험한 장소, 개인 책상, 잠긴 공간, 전기설비 근처는 사용하지 않는다.

## 현재 생성된 문서

- `docs/00_Index.md`
- `docs/01_Game_Overview.md`
- `docs/story/Story.md`
- `docs/story/Reality_Link.md`
- `docs/design/Player_State.md`
- `docs/design/Game_Loop.md`
- `docs/design/Map.md`
- `docs/content/Location_List.md`
- `docs/content/Item_List.md`
- `docs/content/Encounter_List.md`
- `docs/content/Ending_List.md`
- `docs/content/Secret_List.md`
- `docs/dev/Architecture.md`
- `docs/dev/Data_Schema.md`
- `docs/dev/TUI_Layout.md`
- `docs/dev/Development_Plan.md`
- `docs/dev/Checklist.md`

## 다음에 만들 문서 우선순위

1. `docs/story/Disaster_Types.md`
2. `docs/design/Encounters.md`
3. `docs/design/Choices.md`
4. `docs/design/Items.md`
5. `docs/design/Flags.md`
6. `docs/dev/Testing.md`
7. 실제 Python 프로젝트 스캐폴딩
8. YAML 데이터 파일 작성

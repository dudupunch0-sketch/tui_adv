# escape from the office

TUI 기반 랜덤 인카운터 선택지 생존 게임.

플레이어는 갑자기 사람이 모두 사라지고 공간 또는 차원 단위로 격리된 대기업 연구개발동에서 시작한다. 외부 인터넷은 제한되고, 사내망과 회의실 예약 패널, 복합기, 사내 방송만 간헐적으로 작동한다. 체력, 정신력, 배터리, 허기, 갈증을 관리하며 탈출하거나, 회사를 장악하거나, 진실을 파헤치거나, 현실 사무실에 숨겨진 메모/보물로 이어지는 히든 루트를 발견하는 것이 목표다.

## 현재 단계

현재는 Phase 7 콘텐츠 런타임 전환 이후, 스크립트 기반 다중 턴 루프와 1차 확장 콘텐츠 팩까지 구현된 단계다.
게임 구조와 안전한 현실 연결 원칙을 문서화했고, 순수 게임 상태 모델, 자원 임계치/실패 판정, 1차 사무실 위치 모델, 인접 위치 이동, 인카운터/선택지 조건·비용·결과 적용, 능력치 기반 선택지, 2d6 성공/실패 분기, 현재 상태 기반 인카운터 선택, 공간 왜곡 탈출/실패 엔딩 판정, YAML 공개 콘텐츠 로더/검증, YAML 기반 런타임 기본 위치/인카운터/엔딩, 로컬 비공개 현실 힌트 로더, 복합기/커피머신 IP 숫자 합계 퍼즐 더미 값, 현실 연결 히든 엔딩 보상 출력, CLI 한 턴 실행, CLI 다중 턴 스크립트 실행, Textual 레이아웃 smoke, Textual 저장/불러오기 연결, TUI 저장/종료 단축키, TUI 저장 파일 목록 패널, 압박 경고 패널, 소모품 아이템 사용, 물품창고 보급품, 엘리베이터/옥상 경로, 옥상 외부 신호 탈출 엔딩, 저정신력 선택지 왜곡, 고갈증 정수기 환각, 엘리베이터-보안실 우회 분기, 임계 자원 1회성 경고 로그, 보안실-서버실 격리 권한 정복 루트, 지하주차장 키태그/차단기 탈출 루트, 로비 방문증/회전문 탈출 루트, 대표실 결재 콘솔 정복 루트, 두 번째 현실 연결 힌트 체인을 추가했다.

## 실행/테스트

개발 중에는 설치 없이 다음처럼 실행할 수 있다.

```bash
PYTHONPATH=src python -m tui_adv --new --seed 123
PYTHONPATH=src python -m tui_adv --new --seed 123 --choice 1
PYTHONPATH=src python -m tui_adv --new --seed 123 --action choice:1 --action move:dev_office --action move:hallway --action move:server_room_front --action choice:1
PYTHONPATH=src python -m tui_adv --new --seed 123 --location printer_area --action choice:2 --action move:pantry --action choice:3
PYTHONPATH=src python -m tui_adv --new --seed 123 --location printer_area --action choice:3 --action move:pantry --action choice:3
PYTHONPATH=src python -m tui_adv --new --seed 123 --location emergency_stairs --flag escape_puzzle_ready --choice 1
PYTHONPATH=src python -m tui_adv --new --seed 123 --action choice:1 --action move:dev_office --action move:supply_closet --action choice:2 --action use:power_bank
PYTHONPATH=src python -m tui_adv --new --seed 123 --action choice:1 --action move:dev_office --action move:hallway --action move:elevator_hall --action choice:1 --action choice:1
PYTHONPATH=src python -m tui_adv --new --seed 123 --location elevator_hall --action choice:2 --action choice:1 --action move:hallway --action move:server_room_front --action choice:4 --action choice:3
PYTHONPATH=src python -m tui_adv --new --seed 123 --action choice:1 --action move:dev_office --action move:hallway --action move:parking_lot --action choice:1 --action choice:1
PYTHONPATH=src python -m tui_adv --new --seed 123 --action choice:1 --action move:dev_office --action move:hallway --action move:lobby --action choice:1 --action choice:1
PYTHONPATH=src python -m tui_adv --new --seed 123 --action choice:1 --action move:dev_office --action move:hallway --action move:lobby --action choice:2 --action choice:1
PYTHONPATH=src python -m tui_adv --new --seed 123 --location pantry --resource thirst=70 --action choice:1
PYTHONPATH=src python -m tui_adv --tui-smoke --seed 123 --resource sanity=30
PYTHONPATH=src python -m tui_adv --tui-smoke --seed 123 --save saves/autosave.json
PYTHONPATH=src python -m tui_adv --tui-smoke --seed 123
PYTHONPATH=src python -m tui_adv --tui --seed 123 --save saves/autosave.json  # Textual 설치 환경에서 실행, s 저장/q 종료
PYTHONPATH=src python -m tui_adv --tui --load saves/autosave.json --save saves/autosave.json
PYTHONPATH=src python -m tui_adv --version
```

테스트:

```bash
python -m pytest tests -q
```

패키지 설치 후에는 `tui-adv` console script를 사용할 수 있다.

## 핵심 설정

- 제목: `escape from the office`
- 장르: TUI 선택지 생존 게임
- 톤: 블랙코미디 회사 괴담 + 코스믹 호러
- 1차 재난 타입: 불명 재난
- 상황: 사람 실종, 연구개발동 규모의 공간/차원 격리, 제한된 외부 인터넷, 간헐적 사내망 연락
- 핵심 자원: 체력, 정신력, 배터리, 허기, 갈증
- 판정 능력치: 논리, 공감, 의지, 침착, 인터페이스, 신체
- 주요 루트: 탈출, 정복, 진실 발견, 히든 현실 연결

## 문서

- `docs/00_Index.md`: 전체 문서 구조
- `docs/01_Game_Overview.md`: 게임 개요
- `docs/story/Story.md`: 스토리와 오프닝
- `docs/story/Reality_Link.md`: 현실 연결 원칙
- `docs/design/Player_State.md`: 플레이어 상태 규칙
- `docs/design/Game_Loop.md`: 턴/선택/인카운터 루프
- `docs/design/Map.md`: 1차 맵 설계
- `docs/design/UI_Rules.md`: 사내 시스템형 TUI, 글리치, 선택지 오염 규칙
- `docs/dev/Development_Plan.md`: 전체 개발 계획
- `docs/dev/Checklist.md`: 단계별 체크리스트
- `docs/content/Location_List.md`: 1차 위치 목록
- `docs/content/Item_List.md`: 1차 아이템 목록
- `docs/content/Encounter_List.md`: 1차 인카운터 목록
- `docs/content/Ending_List.md`: 1차 엔딩 목록
- `docs/content/Secret_List.md`: 공개 가능한 히든 루트/비밀 목록
- `docs/content/Horror_Ideas.md`: 호러 연출 아이디어 저장소
- `docs/dev/Architecture.md`: 코드 구조와 모듈 경계
- `docs/dev/Data_Schema.md`: YAML/JSON 데이터 스키마
- `docs/dev/TUI_Layout.md`: TUI 화면 설계
- `docs/archive/idea_0515.md`: 2026-05-15 원본 아이디어 노트

## 현실 연결 안전 원칙

이 게임은 현실 사무실의 메모/보물 위치를 히든 루트로 연결할 수 있다.
단, 공개 저장소에는 실제 최종 위치를 넣지 않는다.

- 공개 문서에는 구역/사물/행동 수준의 중간 힌트까지만 둔다.
- 실제 최종 위치는 `private/` 또는 `*.local.*` 파일에만 둔다.
- `private/`와 local secret 파일은 `.gitignore`로 커밋을 차단한다.
- 개인 책상, 잠긴 공간, 위험 설비, 회사 기밀과 관련된 위치는 사용하지 않는다.

## 다음 작업 후보

1. TUI 시작 화면에서 저장 슬롯을 숫자로 직접 선택하는 로드 UX
2. 도움말/키바인딩 패널과 이동용 단축키 확장
3. 추가 현실 연결 힌트 체인과 로컬 비공개 힌트 파일 예시 확장

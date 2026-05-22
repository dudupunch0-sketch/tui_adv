# 밸런싱, QA, 패키징 기준

이 문서는 Phase 10의 큰 덩어리인 밸런싱, QA, 패키징/릴리즈 전 검증을 현재 구현 기준으로 고정한다.
목표는 “지금의 vertical slice가 README만 보고 실행 가능하고, 대표 정상/실패/히든 루트가 자동으로 재현되며, 공개 저장소에 비밀 정보가 섞이지 않는다”는 상태를 유지하는 것이다.

## 밸런싱 기준

현재 밸런스는 짧은 수직 슬라이스 플레이를 기준으로 한다. 긴 캠페인 난이도보다 대표 루트 재현성과 자원 압박 신호의 가독성을 우선한다.

| 항목 | 현재 기준 | 이유 |
|---|---:|---|
| 턴당 허기 증가량 | `+1` | 짧은 루트에서는 압박만 주고 즉시 실패를 강제하지 않는다. |
| 턴당 갈증 증가량 | `+2` | 정수기 환각 조건(`thirst>=60`)까지 더 빠르게 접근하게 한다. |
| 배터리 사용량 | 선택지별 `-2`~`-12` | 정보 접근/외부 신호/관리자 콘솔을 비용 있는 선택으로 유지한다. |
| 체력 피해량 | 위험 행동 중심 `-4` 이상 | 실패 엔딩은 경로 선택의 결과로 보여주고 기본 이동으로는 과하게 깎지 않는다. |
| 정신력 피해량 | 진실/왜곡 선택지 중심 `-2`~`-25` | 코스믹 호러 루트가 정신력 리스크를 담당한다. |
| 음식/물 회복량 | 아이템별 허기·갈증 감소 | 소모품은 루트를 연장하는 안전장치로 둔다. |
| 인카운터 발생률 | 현재 위치/조건 기반 deterministic 노출 | 수직 슬라이스는 랜덤보다 route parity와 테스트 재현성을 우선한다. |
| 엔딩 도달 난이도 | 대표 루트 3~6행동 내 도달 | README/QA 명령으로 정상, 실패, 히든 루트를 빠르게 검증한다. |

## 자동 QA smoke

Phase 10 smoke는 `scripts/qa_smoke.py`가 담당한다.

```bash
PYTHONPATH=src python scripts/qa_smoke.py --list
PYTHONPATH=src python scripts/qa_smoke.py
PYTHONPATH=src python scripts/qa_smoke.py --case escape-ending
```

현재 case:

| case | 확인 항목 |
|---|---|
| `escape-ending` | 비상계단 정상 탈출 엔딩 도달 |
| `failure-ending` | 비상계단 실패 엔딩 도달 |
| `hidden-hint` | 첫 번째 현실 연결 히든 힌트 도달 |
| `invalid-input` | 잘못된 선택지 입력이 traceback 없이 오류 처리됨 |
| `save-load` | 저장 후 로드해서 다음 행동을 이어감 |
| `secret-scan` | `private/`/`.local.*` 추적 금지와 공개 JSON/bundle 최신성 확인 |
| `new-game-10` | seed 100~109 새 게임 10회 시작 smoke |
| `terminal-size` | 80x24, 100x32, 120x40 TUI snapshot 표시 smoke |

Textual 설치 환경에서 실제 Textual widget tree를 확인하는 항목은 `scripts/textual_qa_smoke.py`와 `docs/dev/Final_QA_Log.md`에 둔다.

## 패키징/README 기준

README는 다음을 즉시 제공해야 한다.

- 게임 컨셉과 현재 구현 단계
- Python/Textual 직접 플레이 명령
- Rust content-backed 직접 플레이 명령
- cloud server helper 명령
- smoke/테스트 명령
- 브라우저 fake-TUI export/test/build 명령
- 조작법: 숫자, 이동 단축키, `?`, `i`, `l`, `s`, `q`
- 현실 연결 안전 원칙과 `private/`/local secret 경계
- Phase 10 자동 QA 명령

## 릴리즈 전 비밀 정보 기준

공개 저장소에는 다음이 없어야 한다.

- 실제 사무실 최종 위치
- 실제 개인 이름, 고객 정보, 회사 기밀
- `private/` 파일
- `.local.*` 파일
- 공개 JSON/content bundle 안의 `final_hint`, `actual_ip_address`, `office_location`, `treasure_location`

`secret-scan`은 tracked path와 public export/bundle freshness를 검사한다. 사람이 읽어야 하는 서사 문서의 민감 정보 검토는 PR 리뷰에서 한 번 더 확인한다.

## 수동/환경 QA 경계

GUI/터미널 환경 차이는 자동 snapshot만으로 충분하지 않으므로 별도 기록으로 관리한다.

- 실제 Textual widget tree QA: `PYTHONPATH=src python scripts/textual_qa_smoke.py`
- 터미널 크기별 표시 확인: `PYTHONPATH=src python scripts/qa_smoke.py --case terminal-size`
- 새 게임 10회 기록: `PYTHONPATH=src python scripts/qa_smoke.py --case new-game-10`

최종 결과와 확인 기준은 `docs/dev/Final_QA_Log.md`에 기록한다.

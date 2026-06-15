# 최종 QA 기록

이 문서는 Checklist에 남아 있던 실제 Textual 화면 QA, 새 게임 10회 플레이 기록, 터미널 크기별 화면 확인을 재현 가능한 명령과 결과로 보관한다.
사람이 다시 확인할 수 있도록 모든 명령은 저장소 루트에서 실행한다.

> **§0.88 이전 legacy 기록**: 아래 명령과 결과는 Python 게임 로직(`scripts/qa_smoke.py`, `scripts/textual_qa_smoke.py`)이 존재하던 §0.88 이전 기준이다. §0.88 이후 게임 로직 QA truth는 `cargo test --workspace`(WSL)이며, 해당 스크립트는 삭제됐다.

## 실행 명령 (§0.88 이전 legacy)

기본 QA smoke:

```bash
PYTHONPATH=src python scripts/qa_smoke.py --case new-game-10 --case terminal-size
```

Textual 설치 환경에서 실제 Textual widget tree를 띄우는 QA smoke:

```bash
PYTHONPATH=src python scripts/textual_qa_smoke.py
```

확인 결과 (당시):

```text
PASS new-game-10
PASS terminal-size
PASS start-save-slot
```

## 실제 Textual 화면 QA

`scripts/textual_qa_smoke.py`의 `start-save-slot` case는 `textual>=0.85` 환경에서 `App.run_test()`로 실제 Textual 앱을 구성한 뒤 키 입력을 실행한다.
문자열 snapshot만 검사하지 않고, `build_office_escape_app()`이 만드는 실제 `Static` widget tree와 `#game-grid` 구성을 대상으로 한다.

확인한 흐름:

1. `--save` 디렉터리에 저장 파일이 있으면 시작 화면이 저장 슬롯 선택 모드로 열린다.
2. `[컨트롤]` 패널에 `숫자: 저장 파일 불러오기 / n: 새 게임 / d: 저장 파일 삭제 모드`가 표시된다.
3. 최근 저장 파일이 1번으로 표시된다.
4. `1` 입력으로 최근 저장 파일을 불러오고 선택 모드를 빠져나간다.
5. `d` 입력으로 삭제 모드에 들어가며 문구가 `숫자: 저장 파일 삭제 / n: 새 게임`으로 바뀐다.
6. 삭제 모드에서 `1` 입력은 최근 저장 파일만 삭제하고, 다른 저장 파일은 보존한다.
7. 삭제 후 로그 패널에 `저장 슬롯 삭제: recent.json`이 남는다.
8. `n` 입력은 새 게임으로 돌아가며 저장 슬롯 선택 모드를 빠져나간다.

## 새 게임 10회 플레이 기록

`scripts/qa_smoke.py --case new-game-10`는 seed `100`부터 `109`까지 10개 새 게임을 실행한다.
각 실행은 `PYTHONPATH=src python -m tui_adv --new --seed <seed>`를 통해 실제 CLI 경로를 사용한다.

확인 기준:

- process exit code 0
- 제목 `escape from the office` 표시
- `[LOCAL STATUS]` 표시
- 시작 인카운터 표시
- traceback 없음

## 터미널 크기별 화면 확인

`scripts/qa_smoke.py --case terminal-size`는 다음 화면 프로필을 기준으로 TUI snapshot을 검사한다.

| 프로필 | 용도 |
|---|---|
| `80x24` | 작은 터미널에서 스크롤 전제의 최소 표시 확인 |
| `100x32` | 문서상 권장 최소 크기 확인 |
| `120x40` | 넓은 터미널에서 기본 패널 구성 확인 |

확인 기준:

- 시작 인카운터 snapshot과 이동 후 snapshot이 모두 생성된다.
- `[위치]`, `[컨트롤]`, `[최근 로그]` 패널이 빠지지 않는다.
- 각 프로필의 columns보다 20칸 이상 긴 비정상 라인이 없다.
- 작은 화면에서는 Textual의 `overflow-y: auto`를 전제하므로 전체 내용이 한 화면에 모두 들어가는 것을 완료 조건으로 삼지 않는다.

## 남은 범위

이 문서까지 완료하면 `docs/dev/Checklist.md`의 체크박스 기준 미완료 항목은 없다.
다음 작업은 기능 완료가 아니라 신규 후보이며, README의 “다음 작업 후보”에서 별도로 관리한다.

# 객패귀로 1~2막 cadence audit

요약: 16/48 PASS. 총 65 choices, 고유 script/slot/event ref 16개. 각 장은 closure, hook, reentry, 6축(`time`, `place`, `companions`, `injury`, `information`, `hostile_pressure`)을 갖춘다.

## 1막: 1/01–1/08

| 슬롯 | closure | hook | reentry | 6축 |
|---|---|---|---|---|
| 1/01 | 사냥꾼 오해 종료, 처마 확보 | 사원증·나무패 흔적 | 첫 끼·처마·기록 뒤 장터 시비 | PASS |
| 1/02 | 공개 채무 시비 종료, 치료 처마 확보 | 약방 장부의 청류문 표기 | 손상 확인·생활 간격 뒤 개입 | PASS |
| 1/03 | 치료·추격을 약방에서 종료 | 흰 매듭·확인 표시 | 부상 안정·동행 조건 뒤 안내 | PASS |
| 1/04 | 임시 수습생의 방·식사·회복 확보 | 객패 문장과 빈 줄 | 생활 기록 뒤 회복 결정 | PASS |
| 1/05 | 식사·방·치료 계획으로 수렴 | 심법을 적을 빈 칸 | 회복 생활·통증 확인 뒤 호흡 | PASS |
| 1/06 | 수련 중단과 회복으로 종료 | 검은 줄과 손목 경고 | 몸 기록·회복 뒤 복기 | PASS |
| 1/07 | 패배를 식사·잡역으로 흡수 | 탄 나무의 객패 가장자리 | 패배·생활 기록 뒤 조사 | PASS |
| 1/08 | 증거 봉인·경계 강화, 즉시 출발 없음 | 표식·빈 장부·환대 관습 | 며칠 전조 뒤 2/09 | PASS |

## 2막: 2/09–2/16

| 슬롯 | closure | hook | reentry | 6축 |
|---|---|---|---|---|
| 2/09 | 준비 상태로 종료, 습격 없음 | 붉은 재·전달끈 | 며칠 뒤 동시 징후 | PASS |
| 2/10 | 습격·불길·응급 부상 종료 | 벽돌·연락첩의 같은 이름 | 회복·잔해 정리 뒤 호명 | PASS |
| 2/11 | 생존자 호명·돌봄 상태 확정 | 빈 문패·돌봄표 | 기록 보관 뒤 해독 | PASS |
| 2/12 | 부분 해독·애도·수선으로 종료 | 반쪽 방향과 재건표 | 회복·보관 뒤 재건 | PASS |
| 2/13 | 한 기능 복구, 나머지 봉인 | 빈 당번·약재·발놀림 | 생활 기록 뒤 외부 방향 | PASS |
| 2/14 | 채무선만 확인, 추격 없음 | 장부 조각·물자 공백 | 봉인·회복 뒤 공동 책임 | PASS |
| 2/15 | 치료·당번 조정으로 손목 안정 | 객패 장부와 빈 먹점 | 비용·해독 뒤 출발 비용 | PASS |
| 2/16 | 비용 기록 후 안정된 야영지 | 끊긴 선이 있는 두 길표 | 인계장·생활 뒤 3/17 | PASS |

## 2막 점검

- narrative: 누적 전조→습격→호명·해독·재건→채무·돌봄→제한된 출발 비용으로 이어진다. 긴박함은 각 장 안에서 닫힌다.
- bridge: 슬롯과 event ref가 순서대로 이어지고 회복·식사·기록·재건 간격을 거친다. 즉시 추격·카운트다운·영구 귀환/정착 확정은 없다.
- source-gap: companion은 `authoring_draft`이며 수치·희귀도·runtime 효과는 미정이다.
- runtime-overlay: `review_status: authoring_review_required`, `runtime_status: not_implemented`를 유지한다. 구현 완료나 승인 선언이 아니다.
- reward-exclusivity: 실제 보상 예시는 `젖은 출입명부`, `생명의 부적`, `청류 파진보`, `오늘의 호명 장부`, `검은 비늘 장부 조각`, `서하린의 손수건`, `세 번 접은 외출패`다. `검은 비늘 장부 조각`은 2/14 `take` 선택에만 귀속된다.

## 검증 근거

- canonical validator: PASS.
- tests: 114 PASS.
- canonical validator는 companions를 스캔하지 않는다.
- diff-check: PASS.
- YAML inventory: 16 files / 65 choices / 65 unique choice IDs / 65 unique reward names.

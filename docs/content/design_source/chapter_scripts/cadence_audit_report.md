# 객패귀 1~4막 cadence audit

요약: Act4 등록 후 YAML inventory는 32/48 companion slots, 총 130 choices, 고유 script/slot/event ref 32개다. 최종 검증은 design validator PASS, pytest 129 passed, git diff-check PASS, independent Act4 audit PASS이며, 각 장은 closure, hook, reentry, 6축(`time`, `place`, `companions`, `injury`, `information`, `hostile_pressure`)을 갖춘다.

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

## 3막: 3/17–3/24

| 슬롯 | closure | safe state | hook | reentry |
|---|---|---|---|---|
| 3/17 | 역참 주인이 주인공을 파괴 세력의 대리인이 아닌 오늘 밤의 손님이자 객패 전달자로 기록 | 작은 방에서 출근복·사원증·객패 정리, 식사·수면·탄 자국 대조. side 1~2장과 random 약 7개 가능 | 새벽 장부에 고현묵 객잔 별칭이 보이지만 밤중 추적 없음 | `guestpass_first_relay_identity_chosen`과 역참 숙박·아침 확인 후 3/18 |
| 3/18 | 고현묵이 객패를 소유로 넘기지 않고 수탁 매듭으로 묶고, 급보 뒤 밤을 식사·수면으로 닫음 | 객잔에서 장부·설거지·식사·전령 회복. side 1~2장과 random 약 7개 가능 | 압류 문서의 빈 줄 순서가 객잔 장부와 같음 | required flags·수탁 매듭·`qingliu_empty_room_seizure_notice_received` 확인 후 다음 장 |
| 3/19 | 압류 철회·임시 봉인·제한부 유예 중 하나로 현장을 닫고, 부재한 이름과 물리 흔적을 기록 | 며칠간 수면·식사·재건 당번·증언 기록·채무 책임 대화·문패/객패 대조 가능. 추심 추적·전투 자동 시작 없음 | 문패의 검은 표식과 객패 방향 대조, 익명 증언 보존, 제한부 채무 조항 재분담 중 선택 가능 | 회복·기록 정리 후 `guestpass_route_to_baesan_opened`가 재진입을 열고 3/20으로 연결 |
| 3/20 | 대치를 장 안에서 비살상으로 끝내고 배산귀 생존, 제한적 환상은 이름돌이 잠깐 겹쳐 보인 현상으로 기록 | 며칠간 산문 밖 식사·수면·장비 수선·패찰 판독·역할 기한 확인 가능. 재자극·즉시 추적전 없음 | 패찰 이름의 익명 기록, 흙/발자국 비교, 책임의 끝을 서하린과 합의할 수 있음 | 안전한 휴식·증거 정리 후 `guestpass_route_to_next_movement_trace_opened`가 `wuxia_route_return_scent_clue`를 재진입 |
| 3/21 | 객잔과 감정 동요를 정리하고 포털·귀환·추격을 시작하지 않음 | 식사·수면·빨래·사원증 보관·객패 대조, side 1~2개와 random 약 7개 가능 | 창고 선반의 젖은 검은 천 조각을 흑사방 물건이라고 단정하지 않음 | `guestpass_return_scent_clue_resolved`와 장내 정리·다음 이동 흔적, 하루 이상 휴식/기록 후 3/22 |
| 3/22 | 두건을 네 방식 중 하나로 처리하고 장터를 평온하게 정리; 흑사방 회수선만 객패 노선과 연결 | 두건·장부 봉인, 식사·세탁·장터 심부름·손목 휴식. side 1~2장과 random 약 7개 가능 | 장터 끝에서 청류문식 발걸음을 흉내 내는 사람이 한 번 보였다는 말만 남음 | `black_serpent_old_hood_resolved`와 회수선 기록, 하루 이상 장터 생활/휴식 후 3/23 |
| 3/23 | 목격은 네 방식 중 하나의 기록으로 닫히고 심부름꾼은 외원 밖으로 사라진다. 무명과의 대치·추격·출동은 보류되고 식사·손목 휴식을 먼저 선택할 수 있음 | 장작 정리·점심 식사·서하린 손목 휴식·목격 기록 봉인·객잔 장부 대조 가능. side 1~2개와 random 약 7개 가능 | 물길 쪽 젖은 돌에 청류문식 발끝과 흑사방 표식이 겹친 흔적이 남지만 사람의 이름·목적지는 미확정 | `mumyeong_first_sighting_resolved`·`midgame_continuity_started` 기록 후 하루 이상 식사·휴식·대조 뒤 3/24 젖은 봉투 사건. referenced event=`implemented_preview`, companion=`not_synced` |
| 3/24 | 네 선택 모두 생존 답장과 개인 구조 요청을 분리한다. 봉투는 봉인·반환·제한 공개·증인 확인 중 하나로 정리되고 즉시 출동·발신자 대치는 없음 | 답장 봉인·원문 보관·객잔 장부 대조·서하린 손목 휴식·식사·수면 가능. side 1~2개와 random 약 7개 가능 | 답장 가장자리 물때가 남쪽 물길보다 깊은 지하 수맥 냄새를 품지만 물길 위치·다음 사람 이름은 미확정 | `guestpass_first_reply_to_qingliu_resolved`·`qingliu_survival_reply_sealed` 기록 후 답장·봉투를 하루 이상 보관하고 `wuxia_guestpass_huangquan_waterway_testimony`로 진입. referenced event=`design_only_imported`, companion=`not_synced` |

## 4막: 4/25–4/32

| 슬롯 | closure | safe state | hook | reentry |
|---|---|---|---|---|
| 4/25 | 최초 소유자 마지막 경로와 장내 조사를 봉인하고 닫음 | 쉼터에서 사원증·객패·답장 건조, 손목 휴식·식사·수면·기록 복사 | 무명과 같은 보폭의 발자국만 남기고 추적·전투 없음 | 공통 flags 기록과 하루 휴식 뒤 4/26 대치 |
| 4/26 | 다섯 선택 모두 무명 대치를 마당 안에서 종결; 버티기·분석·이탈 성과와 서하린 중단 권한을 남김 | 객잔 식사·수면·손목 휴식·사원증 끈 수선·출근복 정리·초식 대조; 즉시 추격 없음 | 이름 질문과 물길 창고 장부 경로가 겹치지만 보스·계산자 결론은 유보 | guestpass_mumyeong_name_confrontation_resolved·guestpass_name_claim_tested 후 4/27 |
| 4/27 | 원장부를 재소유하지 않고 위치·보관자·대조 경로의 최소 증거를 남김; 채무·물리 파괴·계산을 분리 | 객잔 기록방 식사·수면·손목 휴식·사원증/출근복 정리·세 칸 대조; 즉시 전투 없음 | 봉인 자국이 보관함 위치를 가리키나 보관자·대조 경로 우선 | guestpass_ledger_location_lead_opened·guestpass_master_ledger_location_lead 후 4/28 |
| 4/28 | 네 선택 모두 보스 거래를 장내 종결; 제한 채무·조사·거절·증인 항목화 중 하나만 남기고 즉시 전투 없음 | 객잔 식사·수면·손목 휴식·영수선 대조·사원증/출근복 정리; 즉시 추적 없음 | 지연 표시는 원장부 탈취가 아니라 다음 확인 시각·전달 순서 경고 | black_serpent_boss_bargain_resolved·black_serpent_debt_model_exposed 후 4/29 |
| 4/29 | 네 기록 방식 중 하나를 확정하고 원본·보호본·미끼 책임선을 장부실에서 봉인 | 장부실/객잔에서 식사·손목 휴식·출근복/사원증 정리·기록 재확인; 하루 준비 | 납인에 사도의 시차 계산 흔적만 남고 추적·전투 없음 | guestpass_ledger_delay_resolved·sado_first_bargain_opened 후 4/30 |
| 4/30 | 네 분기 모두 세 요소 분리 통제와 원장부 회수 경로를 남기고 장부실 대화를 종결 | 객실 식사·서하린 재검토·손목 휴식·출근복/사원증 정리·증거 재확인 | 세 요소 문턱 전조만 드러나며 즉시 대결 없이 회수 자격 확인 | guestpass_master_ledger_recovery_route_opened 후 4/31 |
| 4/31 | 약초꾼 거래와 흑사방 채무를 분리 기록하고 회수 경로/부족분을 봉인; 추적·잠입·전투 없음 | 창고 식사·약재 정리·손목 휴식·보호본 재확인; 하루 준비·증언 대조 | 검은 재·봉인 실밥이 본거지 문턱을 가리키나 세 요소 자격은 미확정 | guestpass_three_element_evidence_partial_or_complete 후 부족분 확인, 4/32 |
| 4/32 | 완전 증거는 본부 진입 eligibility만 열고, 부족하면 recovery route를 남김; 즉시 잠입·추격·전투 없음 | 여관 식사·수면·출근복 세탁·사원증 확인·손목 휴식·장부 경로 조사; 준비·재확인 후 재개 | 문턱 봉인은 침투 초대가 아닌 결과 표지이며 부족한 증거를 먼저 회수 | partial/complete flag 후 complete는 guestpass_network_headquarters_entry_opened, partial은 recovery route로 재진입 |

### Act4 source/runtime provenance

- 4/25: authored design source, companion not_implemented.
- 4/26: referenced event implemented_preview, companion not_synced.
- 4/27~4/28: referenced event design_only_imported, companion not_synced.
- 4/29~4/30: authored design source, companion not_implemented.
- 4/31: referenced event design_only_imported, companion not_synced.
- 4/32: referenced event imported_runtime_semantic_conflict; original runtime has death/resurrection choices while companion is a three-element eligibility gate, companion not_synced.

### Act3 source/runtime provenance

- 3/17: imported 원본, `imported_unreviewed_design_only_companion_runtime_not_implemented`.
- 3/18: authored design source, `authored_design_source_companion_runtime_not_implemented`.
- 3/19~3/20: `source_gap: true` companion 보강이며 companion runtime은 미구현.
- 3/21~3/22: referenced event `design_only_imported`, companion `not_synced`.
- 3/23: referenced event `implemented_preview`, companion `not_synced`.
- 3/24: referenced event `design_only_imported`, companion `not_synced`; authored source readiness는 `wuxia_guestpass_huangquan_waterway_testimony`다.

## 2막 점검

- narrative: 누적 전조→습격→호명·해독·재건→채무·돌봄→제한된 출발 비용으로 이어진다. 긴박함은 각 장 안에서 닫힌다.
- bridge: 슬롯과 event ref가 순서대로 이어지고 회복·식사·기록·재건 간격을 거친다. 즉시 추격·카운트다운·영구 귀환/정착 확정은 없다.
- source-gap: companion은 `authoring_draft`이며 수치·희귀도·runtime 효과는 미정이다.
- runtime-overlay: `review_status: authoring_review_required`, `runtime_status: not_implemented`를 유지한다. 구현 완료나 승인 선언이 아니다.
- reward-exclusivity: 실제 보상 예시는 `젖은 출입명부`, `생명의 부적`, `청류 파진보`, `오늘의 호명 장부`, `검은 비늘 장부 조각`, `서하린의 손수건`, `세 번 접은 외출패`다. `검은 비늘 장부 조각`은 2/14 `take` 선택에만 귀속된다.

## 검증 근거

- final validation: design validator PASS.
- tests: pytest 129 passed.
- canonical validator는 companions를 스캔하지 않는다.
- diff-check: PASS.
- independent Act4 audit: PASS.
- YAML inventory: 32 files / 130 choices / 130 unique choice IDs / 130 unique reward names.

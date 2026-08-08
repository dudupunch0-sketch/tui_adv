# 객패귀로 메인 스토리 v1.1 — 48개 필수 사건 canonical mapping

design_status: approved_mapping_draft
review_status: authoring_review_required
runtime_status: not_implemented
source_arc: guestpass_homecoming_main_story_v1_1

이 문서는 [객패귀로 메인 스토리 통합 설계 v1.1](guestpass_homecoming_main_story_v1_1.md)의 6막 48개 필수 슬롯을 현재 로컬 정본의 기존 event ID와 대조한 authoring용 매핑 초안이다. 기존 events/imported 파일은 보존하며 수정하지 않는다. existing은 기능을 거의 유지할 수 있는 사건, adapt existing은 기존 사건을 중심축에 맞게 확장·재배치하는 사건, demote existing은 기존 사건을 메인 필수에서 사이드로 내리는 경우, merge는 여러 기존 사건을 하나의 필수 슬롯으로 묶는 경우, new는 기능상 대응하는 기존 ID가 없어 신규 저작이 필요한 경우다.

## 집계

| 판정 | 슬롯 수 |
| --- | ---: |
| 유지 (existing) | 14 |
| 흡수 (adapt existing) | 21 |
| 사이드 전환 (demote existing) | 0 |
| 병합 (merge) | 3 |
| 신규 필요 (new) | 10 |
| 합계 | 48 |

모든 행의 authoring_review_required는 개별 사건 저작·선택지·보상 검수가 아직 끝나지 않았다는 뜻이며, 상위 방향 승인이나 runtime 구현 완료를 뜻하지 않는다.

## 48개 필수 슬롯 전수 매핑

| Act/slot | 사건명·기능 | 후보 기존 ID | 판정 | 핵심 변경점 | 선행 → 후속 | 검수 |
| --- | --- | --- | --- | --- | --- | --- |
| 1/01 | 출근길 균열: 현대인 전이·출근복장 | wuxia_prologue_commute_rift | 유지 | 전이 직후 혼란과 현대 물품을 시작점으로 명시 | 없음 → 1/02 | authoring_review_required |
| 1/02 | 낯선 장터와 흑사방 첫 시비 | wuxia_arrival_market_confusion | 유지 | 시장 오해를 흑사방 채무·착취 징후와 연결 | 1/01 → 1/03 | authoring_review_required |
| 1/03 | 서하린 개입과 치료 | wuxia_seoharin_intervention | 유지 | 실제 서하린 개입 사건을 보호·관찰·책임의 시작으로 유지 | 1/02 → 1/04 | authoring_review_required |
| 1/04 | 청류문 임시 수습생 편입 | wuxia_qingliu_apprentice_entry | 유지 | 소속·잡역·치료비를 객패 환대와 연결 | 1/03 → 1/05 | authoring_review_required |
| 1/05 | 첫 식사·잠자리·치료비 | wuxia_cheongryu_first_night_shelter + wuxia_cheongryu_recovery_meal_debt | 병합 | 쉼과 비용·노동을 한 감정 장면으로 통합 | 1/04 → 1/06 | authoring_review_required |
| 1/06 | 첫 호흡 수련 | wuxia_cheongryu_first_breathing_lesson | 흡수 | 현대인의 몸·출근복장 부적응과 서하린 지도 추가 | 1/05 → 1/07 | authoring_review_required |
| 1/07 | 서하린 복기와 청류문 적응 | wuxia_cheongryu_training_first_failure | 흡수 | 패배를 복기·소속감 장면으로 확장 | 1/06 → 1/08 | authoring_review_required |
| 1/08 | 불탄 객패와 마지막 이동 기록 | 없음 | 신규 필요 | 귀로객망 표식·최초 소유자 기록 신규 부여 | 1/07 → 2/09 | authoring_review_required |
| 2/09 | 혈월교 습격 전조 | wuxia_cheongryu_raid_omen | 흡수 | 객패 표식 추적과 연결, 천기록은 보조 | 1/08 → 2/10 | authoring_review_required |
| 2/10 | 혈월교의 청류문·귀로객망 물리 파괴 | 없음 | 신규 필요 | 현재 습격 본체에 대응하는 기존 ID가 없어 외부 물리 폭력 사건을 신규 저작 | 2/09 → 2/11 | authoring_review_required |
| 2/11 | 부상자·실종자·파괴 시설 확인 | wuxia_cheongryu_raid_aftermath_roll_call | 유지 | 피해자 이름 보존과 서하린 공동 책임 유지 | 2/10 → 2/12 | authoring_review_required |
| 2/12 | 객패 표식과 최초 소유자 기록 해독 | wuxia_cheongryu_survivor_ledger | 흡수 | 생존자 장부 기능을 객패 기록 해독으로 변경 | 2/11 → 2/13 | authoring_review_required |
| 2/13 | 청류문 재건과 환대의 빚 | wuxia_cheongryu_raid_repair_first_debt + wuxia_cheongryu_rebuild_last_beam | 병합 | 복구·빚 결산을 공동 노동으로 통합 | 2/12 → 2/14 | authoring_review_required |
| 2/14 | 흑사방 채무 장부 흔적 | wuxia_black_serpent_ledger_trace | 흡수 | 객패 경로와 원장부 연결 명시 | 2/13 → 2/15 | authoring_review_required |
| 2/15 | 서하린 손목 부상과 공동 책임 | wuxia_seoharin_hides_training_injury | 유지 | 신뢰와 책임의 증거로 유지 | 2/14 → 2/16 | authoring_review_required |
| 2/16 | 객패 추적 출발 | wuxia_cheongryu_first_departure_cost | 흡수 | 출발 비용을 객패·환대·추적 목표로 묶음 | 2/15 → 3/17 | authoring_review_required |
| 3/17 | 첫 역참 신분 선택 | wuxia_route_explain_choice_to_seoharin | 흡수 | 가명·객패 제시 선택으로 확장 | 2/16 → 3/18 | authoring_review_required |
| 3/18 | 고현묵 객잔과 객패 계승 | 없음 | 신규 필요 | 고현묵·최초 소유자·객잔 거점 신규 저작 | 3/17 → 3/19 | authoring_review_required |
| 3/19 | 청류문 빈방 압류 방어 | [wuxia_guestpass_qingliu_empty_room_seizure_defense](../events/authored/wuxia_guestpass_qingliu_empty_room_seizure_defense.yml) | 신규 필요 | 신규 canonical authoring draft 작성; 청우진·서하린이 문패·객패·공동체 증거 제시 | 3/18 → 3/20, 3/24 | authoring_draft / authoring_review_required / not_implemented |
| 3/20 | 외안배산귀 통과 | 없음 | 신규 필요 | 이름과 시선의 시험 규칙·대가·선택 신규 저작 | 3/19 → 3/21 | authoring_review_required |
| 3/21 | 객패 다음 이동지 추적 | wuxia_route_return_scent_clue | 흡수 | 흔적을 객패 다음 역참과 연결 | 3/20 → 3/22 | authoring_review_required |
| 3/22 | 흑사방 장부·귀로객망 경로 연결 | wuxia_black_serpent_old_hood | 흡수 | 조직 흔적을 채무 추적·귀로 노선 중첩으로 재기능화 | 3/21 → 3/23 | authoring_review_required |
| 3/23 | 무명의 첫 흔적 | wuxia_mumyeong_first_sighting | 흡수 | 무명을 이름·기술·선택의 거울로 배치 | 3/22 → 3/24 | authoring_review_required |
| 3/24 | 청류문에 첫 답장 | wuxia_chain_wet_message_sender | 흡수 | 생존 보고·압류 방어 후속으로 변경 | 3/23 → 4/25 | authoring_review_required |
| 4/25 | 황천수맥과 최초 소유자 마지막 기록 | 없음 | 신규 필요 | 괴이 규칙·기록 회수·이동 기록 신규 저작 | 3/24 → 4/26 | authoring_review_required |
| 4/26 | 무명과 객패·이름 대치 | wuxia_mumyeong_first_confrontation | 흡수 | 객패를 누구 이름으로 부를지 묻는 장면으로 재배치 | 4/25 → 4/27 | authoring_review_required |
| 4/27 | 흑사방 원장부 위치 | wuxia_black_serpent_ledger_trace | 유지 | 기존 흔적을 원장부 위치로 수렴; 중복 소유 여부 검수 | 4/26 → 4/28 | authoring_review_required |
| 4/28 | 흑사방 보스 채무 거래 | wuxia_heuksa_bang_debt_bargain | 유지 | 보스는 채무 거래, 사도는 최종 계산으로 분리 | 4/27 → 4/29 | authoring_review_required |
| 4/29 | 객패 장부 지연 계략 | [wuxia_guestpass_ledger_delay_stratagem](../events/authored/wuxia_guestpass_ledger_delay_stratagem.yml) | 신규 필요 | 신규 canonical authoring draft 작성; 서하린이 낮 작성·밤 재배열로 보호 시간차 생성 | 4/28 → 4/30, 5/33 | authoring_draft / authoring_review_required / not_implemented |
| 4/30 | 사도의 첫 거래와 계산 논리 | 없음 | 신규 필요 | 세 요소 분리 통제·가격표를 첫 대면으로 저작 | 4/29 → 4/31 | authoring_review_required |
| 4/31 | 원장부 확보 또는 회수 경로 | wuxia_chain_herbalist_second_ledger | 흡수 | 약재 장부 구조를 원장부 우회 경로로 변경 | 4/30 → 4/32 | authoring_review_required |
| 4/32 | 본거지 문이 세 요소를 요구함 | wuxia_collapse_gate | 흡수 | 객패·원장부·귀환 단서 자격 조건으로 재정의 | 4/31 → 5/33 | authoring_review_required |
| 5/33 | 귀로객망 본거지 귀환 잠입 | 없음 | 신규 필요 | 세 요소를 잠입 목표로 삼는 신분·기록 선택 신규 저작 | 4/32 → 5/34 | authoring_review_required |
| 5/34 | 묵돌·흉터·매듭·빈방의 단계적 재인식 | wuxia_mumyeong_name_returns | 흡수 | 이름 회복을 세 단계 증거 구조로 확장 | 5/33 → 5/35 | authoring_review_required |
| 5/35 | 청우진·고현묵의 행동 확인 | wuxia_route_qingliu_short_report | 흡수 | 보고를 두 인물의 실제 행동·지속 증거로 변경 | 5/34 → 5/36 | authoring_review_required |
| 5/36 | 객패·원장부·귀환 단서 공개 자격 시험 | wuxia_alliance_inspector_three_questions | 흡수 | 세 질문을 공개 자격 시험으로 전환 | 5/35 → 5/37 | authoring_review_required |
| 5/37 | 사도의 세 요소 분리 통제 공개 | wuxia_sado_ownerless_token | 흡수 | 토큰을 분리 가격화 표식으로 변경 | 5/36 → 5/38 | authoring_review_required |
| 5/38 | 귀향소 목소리 시험 | wuxia_return_modern_object_rumor | 유지 | 현대 물품 소문을 기억 시험과 연결; 정답 제시 금지 | 5/37 → 5/39 | authoring_review_required |
| 5/39 | 현대 귀환 자유 확보 | 없음 | 신규 필요 | 세 요소 확보 뒤 귀환 선택을 정상 엔딩으로 열기 | 5/38 → 5/40, 6/41 | authoring_review_required |
| 5/40 | 서하린과 진실 공유 | wuxia_seoharin_truth_share_before_boss | 유지 | 귀환 자유를 숨기지 않는 관계 장면 | 5/39 → 6/41 | authoring_review_required |
| 6/41 | 사도 최종전 가격표 | wuxia_sado_final_phase_1_price_tag | 유지 | 최종전 1단계와 세 엔딩 축 연결 | 5/40 → 6/42 | authoring_review_required |
| 6/42 | 약점 장악 | wuxia_sado_final_phase_2_weakpoint_control | 유지 | 관계·청류문·타인 이름을 인질화 | 6/41 → 6/43 | authoring_review_required |
| 6/43 | 계산식 밖 | wuxia_sado_final_phase_3_outside_calculation | 유지 | 세 축을 누적 증거·행동으로 결산 | 6/42 → 6/44 | authoring_review_required |
| 6/44 | 혈월교 파괴·흑사방 채무 거래 결산 | wuxia_boss_resolution | 흡수 | 실제 보스 결산에 2/10 혈월교 파괴 결과를 참조하도록 확장 | 6/43 → 6/45 | authoring_review_required |
| 6/45 | 무명·청류문 선택 결산 | wuxia_mumyeong_resolution | 유지 | 무명 구원과 청류문 소속을 이름 보존 축으로 결산 | 6/44 → 6/46 | authoring_review_required |
| 6/46 | 보복 연쇄 중단 | wuxia_sado_final_battle | 흡수 | 승패와 별개로 보복 반복 중단 선택 추가 | 6/45 → 6/47 | authoring_review_required |
| 6/47 | 빈 밥그릇과 이름 | wuxia_final_prep_last_meal | 흡수 | 최종전 전 식사를 결말 후 공동체 상징으로 재배치 | 6/46 → 6/48 | authoring_review_required |
| 6/48 | 현대 귀환·청류문 잔류·실리·트루 엔딩 | wuxia_epilogue_first_morning_after_boss + wuxia_epilogue_qingliu_guest_arrival + wuxia_seoharin_unsaid_stay | 병합 | 후일담을 3축 결과 카드로 묶고 귀환을 정상 엔딩으로 명시 | 6/47 → 종료 | authoring_review_required |

## 우선 저작안

### 3/19 청류문 빈방 압류 방어

- 목표: 외부 채권자가 떠난 제자의 빈방·문패·물품을 압류하지 못하게 하여 사람을 빚의 담보로 취급하지 않는 원칙을 지킨다.
- 갈등: 흑사방 또는 대리인이 치료비·수리비를 근거로 방을 계산한다. 내부 악의적 배신은 없다. 청우진은 규정·증빙을, 서하린은 사람·현장 안전을 대표한다.
- 선택 방향: 장부와 문패 대조 공개 항의 / 가족·이웃 증언 수집 / 압류 물품 은닉으로 시간 벌기 / 빚 일부 부담과 사람 기록 보존.
- 분기 결과: 방을 지키면 공동체 신뢰·귀로객망 협력 단서, 방을 내주면 물리 손실과 압류 문서의 원장부 표식, 시간을 벌면 서하린 위험 부담과 추적 플래그를 남긴다.
- 보상 후보: 서하린이 맡긴 빈방 열쇠, 빈방 문패 뒷면 표식, 환대의 빚 증언, 청류문 신뢰·서하린 공동 책임. 수치 효과와 희귀도는 미정.
- 후속 플래그 후보: qingliu_empty_room_defended, qingliu_empty_room_seized, black_serpent_ledger_mark_seen, seoharin_shared_burden, guestpass_network_witness_opened.

### 4/29 객패 장부 지연 계략

- 목표: 혈월교·흑사방이 실시간 장부로 귀로객망 사람들의 위치를 계산하지 못하게 낮에는 기록하고 밤에는 안전한 순서·빈칸·지연을 남긴다.
- 갈등: 이는 배신이 아니라 보호를 위한 시간차다. 흑사방은 원장부 대조를 요구하고 사도는 불일치를 가격 협상의 근거로 삼는다. 내부 악의적 배신은 없다.
- 선택 방향: 정직한 기록 후 전달 순서만 변경 / 위험한 이름을 빈칸과 암호로 보존 / 미끼 장부로 추적선 유도 / 서하린과 공동 기록자 되기.
- 분기 결과: 사람을 지키면 안전한 이동 순서와 공동 신뢰, 완전 삭제는 추적 약화와 원장부 회수 난도 상승, 미끼 발각은 사도 계산 노출과 잠입 압박을 남긴다.
- 보상 후보: 밤에만 읽히는 매듭 장부, 서하린 필체의 빈 칸, 원장부 대조용 납인, 서하린 공동 주역·안전 경로·사도 계산 패턴 단서. 수치 효과와 희귀도는 미정.
- 후속 플래그 후보: guestpass_ledger_delay_opened, seoharin_defensive_recording, night_knot_ledger_seen, black_serpent_realtime_trace_delayed, sado_calculation_pattern_revealed.

## 기존 사건 정책과 가드레일

48개 필수 슬롯에 억지로 넣지 않고 사이드·랜덤·후일담으로 유지할 사건:
- wuxia_random_* 및 wuxia_weather_*: 여행과 청류문 생활의 보조 사건.
- wuxia_side_*: 인물·생활 보강용 사이드 사건. 핵심 인과를 새로 열지 않으면 메인 승격 금지.
- wuxia_epilogue_*: 6/48 결과 카드의 표현 후보. 필수 사건의 선행으로 배치하지 않는다.
- wuxia_fallback_*: 실패·누락 증거의 안전한 귀결.
- wuxia_tianjilu_first_fragment 및 wuxia_cheongirok_blank_margin: 천기록 직접 기능 최대 3회, 막 진행 게이트 금지.

청류문 내부 악의적 배신은 추가하지 않는다. 빈방 압류와 장부 지연의 갈등 상대는 외부 채권·추적 세력이다. 혈월교는 물리 파괴, 흑사방 보스는 채무 거래, 사도는 계산·통제의 최종 적으로 분리한다. 이 문서는 설계 매핑일 뿐 imported event와 runtime/code를 수정하거나 구현 완료로 승격하지 않는다. 새 사건은 핵심·보조·선택·보류 등급과 중심축 영향 평가를 거쳐야 하며 청류문·서하린·객패귀로를 강화하지 않거나 6막 순서·세 요소 인과를 덮으면 승격하지 않는다.

3/19 빈방 압류 방어와 4/29 객패 장부 지연 계략은 canonical authoring draft가 작성되었으며 개별 검수는 남아 있다. 다음 신규 저작 순서는 1/08 불탄 객패 → 4/25 황천수맥 기록 기능이다. 두 우선 초안의 검수 뒤 나머지 adapt existing 슬롯을 저작하며, runtime 구현은 별도 handoff와 승인 이후에만 시작한다.

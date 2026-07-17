# 보상 파이프라인 웨이브 1 — 리뷰 및 콘텐츠 재작업 스펙 (rework spec)

- 작성: fable / 2026-07-17 20:20
- 대상: #173 (feat: reward pipeline wave 1, 935cce3) 리뷰 결과
- 구현: codex (본 문서만으로 Notion 접근 없이 재작업 가능하도록 원문 전량 수록)

## 1. 리뷰 결론

**엔진 계층(WP-R1/R4/R5 골격)은 합격, 콘텐츠 계층(WP-R2/R3)은 불합격 — 재작업 필요.**

- 합격: `GameState.skills/titles/relationships`(additive serde), `OutcomeDef.add_skills/add_titles/relationship_deltas`, 중복 무시, ScenePage/WASM 경계, 드로어 등급 위계·마스킹·획득 비트, 기존 44 사건 무변경. 게이트 전부 통과 확인(cargo workspace, pytest 72, export --check, vitest 73, tsc).
- 불합격 사유: 사건 7개의 선택지·보상 매핑·보상 정의 텍스트가 **19번 매핑 DB(정본)·09 사건 카드·17/18/14/15 DB와 대규모 불일치**. 선택지 문구와 보상 이름이 전부 창작이며(29건 중 대응 일치 약 5건), 판정 성공 매핑이 엉뚱한 사건에 부여되고, 히든 양립 칭호가 단일 선택 즉시 지급으로 강등되고, 등급이 위조됨(예: 기척 끊기 보통→전설). 원인: codex의 Notion 접근 불가 + 플랜에 원문 미수록 — 본 스펙이 그 공백을 메운다.
- **정정: 매핑은 총 29건이다** (플랜의 "30건"은 DB 오집계였음. 19번 DB 실측 29행).

## 2. 재작업 원칙

1. **아래 표의 텍스트는 전부 verbatim.** ID·이름·설명·선택지 라벨·획득 연출 문구를 그대로 사용한다 (문장 경계 분리만 허용).
2. 런타임 보상 ID는 DB 원장 ID 그대로: `wuxia_skill_*`, `wuxia_title_*`, `wuxia_insight_*`, `wuxia_item_*`, `wuxia_pendant_*`. 현재 머지된 무접두 ID(`match_the_pulse` 등)는 전부 개명한다. (프리릴리즈이므로 세이브 마이그레이션 불필요 — 보고서에 명시만.)
3. 사건별 choice id = 매핑 ID에서 `wuxia_rewardmap_` 제거한 값 (예: `breath_ask_trust`). choice label = 아래 표의 "선택지 키" verbatim.
4. 사건 body/스토리 블록은 §5의 09 카드 결과 서술에 기반해 작성 (기존 창작 본문 폐기). 결과 스테이지에는 해당 매핑의 "획득 연출" 문구를 반영한다.
5. 판정(check)은 **판정 성공/히든 발견(추격) 매핑 4건에만** 부여 (§3 표의 시점 열). 능력치는 카드 맥락에서 codex 재량, 난이도 보통(9~10). success branch에서만 지급, failure는 무보상 아닌 로그·플래그(실패도 기록) 유지.
6. 후속 회수 4건은 `reward_pending_<보상ID>` 플래그만 세팅 (지급은 후속 사건 웨이브).
7. `wuxia_title_footprints_of_two_paths`(히든 양립)는 **이번 웨이브에서 지급 경로 없음(휴면)**: 정의·도감 잠금 표시만 구현. 산문 확인 선택은 `raid_omen_gate_checked`, 폐서고 선택은 `raid_omen_archive_checked` 플래그를 남기고, 두 플래그 동시 보유 시 지급하는 훅은 다회차(웨이브 C)에서 연결한다. 단일 선택 즉시 지급(현행)은 제거.
8. 관계 보상은 §3의 4건만. 그 외 관계 델타 금지 (현행 gate_patrol의 창작 인물 호감도 제거).
9. 사건 선행 조건은 §5를 따른다 (현행의 일률 순차 체인 폐기).
10. 가드 테스트 강화: substring count 방식 폐기, **29건 각각에 대해 (사건, choice, 보상 종류/ID, 시점) 단위 assert**. 기존 44 사건 무변경·51 커버리지·중복 무시·관계 누적·branch 배타 테스트는 유지.

## 3. 정본 매핑 29건 (19번 DB 전량)

| 매핑 ID (`wuxia_rewardmap_` 생략) | 사건 | 선택지 키 (라벨 verbatim) | 보상 | 시점 | 배타 | 획득 연출 (verbatim) |
|---|---|---|---|---|---|---|
| first_night_stay_guest | first_night_shelter | 서하린이 정한 자리에 머물기 | 칭호 wuxia_title_guest_of_cheongryu | 즉시 | 일반 | 서하린이 말없이 이불을 다시 펴고 다음날 장작 당번을 알려 준다. |
| first_night_separate_boundary | first_night_shelter | 방을 따로 달라고 하기 | 칭호 wuxia_title_not_yet_disciple | 즉시 | 일반 | 서하린은 방을 내주지만 다음날 설명을 줄이고 직접 묻기를 기다린다. |
| first_night_why_fidelity | first_night_shelter | 왜 자신을 받았는지 묻기 | 기연 wuxia_insight_measure_fidelity | 즉시 | 일반 | 서하린은 문밖에 두면 죽으니까라고 답하고, 말하지 않은 이유가 남는다. |
| first_night_leave_name | first_night_shelter | 내일 바로 나가겠다고 말하기 | 칭호 wuxia_title_keeper_of_returning_name | 즉시 | 대체 경로 | 서하린은 붙잡지 않고 회복되면 나가도 된다고 말한다. |
| breath_copy_first_current | first_breathing_lesson | 서하린의 호흡을 그대로 따라 하기 | 기연 wuxia_insight_first_current_breath | **판정 성공** | 일반 | 서하린이 처음으로 손목과 어깨를 직접 교정한다. |
| breath_own_pulse | first_breathing_lesson | 자신의 방식으로 관찰해 따라 하기 | 스킬 wuxia_skill_match_the_pulse | **판정 성공** | 일반 | 서하린은 따라 하지 않았다는 사실보다 스스로 멈출 줄 안 것을 기억한다. |
| breath_ask_trust | first_breathing_lesson | 왜 힘을 빼야 하는지 묻기 | 인물 호감도 relationship_person_seoharin_affection | 즉시 | 일반 | 서하린은 문훈의 한 구절만 말하고 다음날 같은 시간에 나오라고 한다. |
| breath_stop_flow | first_breathing_lesson | 아픈 척하지 않고 중단하기 | 스킬 wuxia_skill_fallen_leaf_flow_step | 즉시 | 대체 경로 | 서하린은 실패라고 하지 않고 오늘은 여기까지라고 말한다. |
| failure_no_excuse | training_first_failure | 변명하지 않고 복기하기 | 기연 wuxia_insight_recording_defeat | 즉시 | 일반 | 서하린이 정답 대신 한 가지 오류만 짚고, 다음날 같은 지점을 다시 보게 한다. |
| failure_explain_gap | training_first_failure | 방금 본 동작을 설명하기 | 스킬 wuxia_skill_record_the_gap | 즉시 | 일반 | 주인공의 설명 중 틀린 부분 하나가 다음 훈련의 목표가 된다. |
| failure_rematch_blade | training_first_failure | 서하린에게 재대련 요청하기 | 스킬 wuxia_skill_turning_blade | **판정 성공** | 일반 | 서하린은 바로 겨루지 않고 목검의 방향만 다시 잡아 준다. |
| failure_escape_chore | training_first_failure | 다음 잡일로 도망가기 | 칭호 wuxia_title_not_yet_disciple | 즉시 | 일반 | 장작을 패면서도 수련장 쪽 소리가 들리는 짧은 장면이 남는다. |
| medicine_alone_pouch | medicine_errand | 시장까지 혼자 다녀오기 | 아이템 wuxia_item_modern_first_aid_pouch | **후속 회수** | 일반 | 시장에서 주인공의 파우치를 본 약재상이 낯선 붕대법을 묻는다. |
| medicine_together_affection | medicine_errand | 서하린과 함께 가기 | 인물 호감도 relationship_person_seoharin_affection | 즉시 | 일반 | 서하린이 시장에서 주인공의 걸음을 맞추지만 대화는 짧다. |
| medicine_badge_title | medicine_errand | 사원증·출근 물건을 교환 제안하기 | 칭호 wuxia_title_badge_bearer | 즉시 | 일반 | 사원증을 받은 상인이 이름을 읽지 못하고, 주인공은 처음으로 그 이름을 지키려 한다. |
| medicine_empty_ledger | medicine_errand | 빈손으로 돌아와 부족을 알리기 | 퀘스트 아이템 wuxia_item_empty_medicine_ledger | **후속 회수** | 일반 | 서하린이 장부의 빈칸을 손가락으로 짚으며 누군가 먼저 가져갔을 가능성을 말한다. |
| omen_gate_register | raid_omen | 산문을 직접 확인하기 | 퀘스트 아이템 wuxia_item_wet_gate_register | **후속 회수** | 일반 | 비에 젖은 명부의 마지막 이름만 번져 있다. |
| omen_injured_talisman | raid_omen | 부상자와 약재를 점검하기 | 패물 wuxia_pendant_life_talisman | 즉시 | 일반 | 문 안쪽에 걸린 부적을 서하린이 떼어 주며 살아 돌아오라고만 말한다. |
| omen_archive_documents | raid_omen | 폐서고 기록을 찾기 | 기연 wuxia_insight_sort_like_documents | 즉시 | 일반 | 낡은 장부의 빈칸과 산문 순찰표의 빈칸이 같은 날을 가리킨다. |
| omen_rest_threshold | raid_omen | 서하린의 지시에 따라 휴식하기 | 스킬 wuxia_skill_guard_the_threshold | 즉시 | 일반 | 서하린은 산문으로 나가고, 주인공은 문 안쪽 사람들의 자리를 지키게 된다. |
| omen_hidden_two_paths | raid_omen | 산문 조사와 기록 조사 조건을 모두 충족 | 칭호 wuxia_title_footprints_of_two_paths | **히든 발견(휴면)** | 히든 양립 | 서로 다른 기록 두 장이 한 장면에서 겹쳐 보인다. |
| patrol_follow_thread | gate_patrol_first_trouble | 소리를 따라가기 | 퀘스트 아이템 wuxia_item_red_thread_fragment | **히든 발견(판정)** | 일반 | 풀숲 끝에서 휘파람 대신 붉은 실만 발견한다. |
| patrol_report_faction | gate_patrol_first_trouble | 서하린에게 알리기 | 단체 호감도 relationship_faction_cheongryu_affection | 즉시 | 일반 | 서하린은 칭찬하지 않고 다음 순찰표의 빈칸을 함께 확인한다. |
| patrol_ignore_retreat | gate_patrol_first_trouble | 아무 일 없는 척 순찰 계속하기 | 스킬 wuxia_skill_two_steps_back | 즉시 | 일반 | 돌아온 뒤에도 주인공은 휘파람의 박자를 기억한다. |
| patrol_fake_whistle | gate_patrol_first_trouble | 가짜 순찰 신호를 남기기 | 아이템 wuxia_item_cracked_whistle | 즉시 | 일반 | 금 간 호루라기의 소리가 한 번 끊기고, 산문 밖에서 대답이 오지 않는다. |
| wrist_ask_read | seoharin_hides_training_injury | 손목을 직접 묻기 | 기연 wuxia_insight_read_the_wrist | 즉시 | 일반 | 서하린은 쓸 수는 있다고 말하며 오래된 손목을 잠깐 내어 보인다. |
| wrist_ointment_handkerchief | seoharin_hides_training_injury | 약초 연고를 가져오기 | 패물 wuxia_pendant_seoharin_handkerchief | **후속 회수** | 일반 | 서하린은 고맙다는 말 대신 낡은 손수건을 접어 돌려준다. |
| wrist_report_faction | seoharin_hides_training_injury | 장문인에게 알리기 | 단체 호감도 relationship_faction_cheongryu_affection | 즉시 | 일반 | 치료는 시작되지만 서하린은 당분간 주인공과 거리를 둔다. |
| wrist_look_away_presence | seoharin_hides_training_injury | 못 본 척하기 | 스킬 wuxia_skill_cut_the_presence | 즉시 | 일반 | 서하린도 모른 척하고, 두 사람이 같은 침묵을 나눈다. |

사건 ID 접두는 전부 `wuxia_cheongryu_*` (서하린 카드만 `wuxia_seoharin_hides_training_injury`).

`patrol_follow_thread`의 연계 조건 "낮은 확률의 짧은 추격을 통과한 경우" → check로 구현(성공 시 지급, 실패 시 무획득+로그).

## 4. 정본 보상 정의 (이름·설명 verbatim, 현행 창작 텍스트 전량 교체)

### skills.yaml (7) — 컨셉이 도감 문구
| ID | 이름 | 컨셉 | 등급 | 성격 |
|---|---|---|---|---|
| wuxia_skill_match_the_pulse | 심박 맞추기 | 호흡이 무너질 때 심장 박동을 다시 몸의 리듬에 맞추는 현대식 자기 관찰. | 보통 | 사이드 |
| wuxia_skill_fallen_leaf_flow_step | 낙엽 회류보 | 직선으로 맞서지 않고 한 걸음 비껴 흐르는 청류문의 보법. | 희귀 | 사이드 |
| wuxia_skill_record_the_gap | 빈틈 기록 | 상대의 움직임을 머릿속이 아니라 기록으로 남겨 다음 시도에 쓰는 습관. | 희귀 | 사이드 |
| wuxia_skill_turning_blade | 돌아보는 칼끝 | 한 번 물러난 칼끝이 상대의 다음 발을 기다렸다가 돌아오는 반격. | 희귀 | 사이드 |
| wuxia_skill_guard_the_threshold | 문지방 지키기 | 문밖으로 나갈 수 없는 밤에도 누군가의 자리를 지키는 기술. | 희귀 | 메인 |
| wuxia_skill_two_steps_back | 두 걸음 물러서기 | 싸움에서 이기기 위해 먼저 거리를 포기하는 실전 감각. | 보통 | 랜덤 |
| wuxia_skill_cut_the_presence | 기척 끊기 | 산길의 풀잎과 숨소리 사이에 자신을 섞는 정통 은신 기술. | **보통** | **랜덤** |

전원 reveal_immediate true. (현행 cut_the_presence 전설/히든은 위조 — 정정.)

### titles.yaml (5)
| ID | 이름 | 컨셉 | 등급 | 성격 |
|---|---|---|---|---|
| wuxia_title_not_yet_disciple | 제자 아닌 제자 | 문파의 호칭은 받지 못했지만 수련장에 계속 남은 외부인. | **희귀** | 사이드 |
| wuxia_title_guest_of_cheongryu | 청류문의 손님 | 아직 제자는 아니지만 밥상과 장작 당번의 자리를 받은 사람. | 보통 | 메인 |
| wuxia_title_keeper_of_returning_name | 돌아갈 이름을 품은 자 | 새 세계에 머물면서도 원래 이름을 완전히 버리지 않은 사람. | 희귀 | 다회차 |
| wuxia_title_badge_bearer | 사원증을 품은 자 | 낯선 세계에서도 자기 이름과 과거를 버리지 않은 사람. | 보통 | 사이드 |
| wuxia_title_footprints_of_two_paths | 두 길의 발자국 | 한쪽을 택했지만 다른 길의 흔적도 끝까지 추적한 사람. | 전설 | 히든 |

전원 reveal_immediate true.

### items.yaml (7) — 설명이 소지품 상세 노출 문구, 전원 usable false
| ID | 이름 | type | 설계 분류 | 설명 | 등급 | 즉시 공개 |
|---|---|---|---|---|---|---|
| wuxia_item_modern_first_aid_pouch | 현대식 응급 파우치 | consumable | 소모품 | 무림에서 낯설지만 상처를 씻고 묶는 순서만큼은 분명한 파우치. | 희귀 | YES |
| wuxia_item_empty_medicine_ledger | 약재 창고의 빈 장부 | quest | 퀘스트 아이템 | 약재가 없다는 사실보다 누가 언제 비워 갔는지를 말하는 장부. | 희귀 | NO |
| wuxia_item_cracked_whistle | 금 간 호루라기 | tool | 도구 | 불면 맑은 소리 대신 끊긴 숨 같은 음이 나는 호루라기. | 보통 | YES |
| wuxia_item_red_thread_fragment | 붉은 실 한 토막 | quest | 퀘스트 아이템 | 누군가 풀숲과 나뭇가지 사이에 묶어 둔 붉은 실. | 보통 | NO |
| wuxia_item_wet_gate_register | 젖은 출입명부 | clue | 퀘스트 아이템 | 비에 젖은 이름 몇 개가 산문을 드나든 사람들의 순서를 드러낸다. | 희귀 | NO |
| wuxia_pendant_life_talisman | 생명의 부적 | tool | 패물 | 누군가 살아 돌아오기를 바라며 문 안쪽에 걸어 둔 낡은 부적. | 희귀 | YES |
| wuxia_pendant_seoharin_handkerchief | 서하린의 손수건 | tool | 패물 | 서하린이 말없이 건넨 손수건. 약 냄새와 오래된 비 냄새가 함께 남아 있다. | 전설 | YES |

### insights.yaml (5) — 설명이 드로어 노출 문구, bonus 0(수치 미정), 기존 3종 무변경
| ID | 이름 | 설명 | 등급 | 즉시 공개 |
|---|---|---|---|---|
| wuxia_insight_first_current_breath | 청류심법의 첫 숨 | 힘을 고정하는 대신 흐르게 해야 한다는 청류심법의 첫 이해. | 희귀 | YES |
| wuxia_insight_recording_defeat | 패배를 받아 적는 습관 | 패배를 손실이 아니라 다음 시도의 자료로 바꾸는 기연. | 희귀 | YES |
| wuxia_insight_measure_fidelity | 신의를 저울질하는 마음 | 신의가 언제나 이익이 되는 것은 아니지만, 무엇을 잃는지는 스스로 재는 마음. | 희귀 | YES |
| wuxia_insight_sort_like_documents | 서류처럼 정리하는 머리 | 현대의 기록 습관이 약재 장부와 문파의 결핍을 연결하는 방식으로 재해석된 기연. | 보통 | YES |
| wuxia_insight_read_the_wrist | 손목의 맥을 읽는 눈 | 서하린이 숨긴 손목의 미세한 움직임에서 말하지 않은 상태를 읽는 이해. | 희귀 | YES |

주의: 기연은 즉시 공개 전원 YES — 현행 구현의 `reveal_immediate: false`(first_current_breath, sort_like_documents, read_the_wrist) 및 pending 처리(§3에서 즉시로 정정된 항목)는 제거. 지연 공개는 퀘스트 아이템 3종(NO)에만 적용.

## 5. 사건 authoring 근거 (09 카드) 및 선행 조건

| 사건 | 카드 결과 서술 (본문 근거) | 선행 조건 (runtime 플래그화) |
|---|---|---|
| first_night_shelter (첫날 밤 — 빌린 자리) | 네 선택 모두 잔류·회복 기간으로 이어진다. 머무르기=생활 hook, 이유 묻기=정보·신뢰, 나가기=회복 전 외출 제한+귀환 욕구 flag 유지, 따로 방=정보 감소·대화 난이도 상승. | apprentice_entry 해결 직후, 습격 전 |
| first_breathing_lesson (첫 호흡) | 청류심법 핵심 = 힘을 흐르게 하는 것. 실패와 서하린의 관찰이 다음 훈련을 준비. | shelter 해결 |
| training_first_failure (첫 패배와 복기) | 패배가 재능 판정이 아니라 관찰·복기·관계의 재료. 서하린은 한 가지 오류만 짚는다. | breathing + chore_sparring 해결, 습격 전 |
| medicine_errand (약재 창고의 빈칸) | 약재 부족·치료비 부담 체감. 성공해도 큰 보상 없이 다음 부상 장면의 회복 여지. | shelter 해결(생활 적응), 습격 전 — **failure 이후로 체인 금지** |
| raid_omen (습격 전날의 이상 징후) | 습격이 누적된 전조의 결과로 느껴지게. 선택은 승패가 아니라 누구를 먼저 지키는지를 바꾼다. | breathing + chore_sparring 해결, raid_route_split 이전 |
| gate_patrol_first_trouble (산문 순찰 중 휘파람) | 습격 세력 미확정, 순찰 리듬을 관찰하는 존재 암시. 작은 추격 또는 안전한 귀환. | 수습생 상태(apprentice_entry), raid_route_split 이전. "낮은 확률"은 웨이브 B 확률화 — 이번엔 결정론 유지 |
| seoharin_hides_training_injury (숨긴 손목) | 서하린이 먼저 수련장에 오는 이유와 부상 은닉 습관. 상호 돌봄의 비대칭 인식. | failure 또는 chore_sparring 해결 |

- yaml 배치는 raid_route_split 이전, raid_route_split 조건 무변경.
- 스토리 스테이지: illustration 1개(placeholder true + 한국어 장면 묘사 alt) 유지. 기존 alt는 재사용 가능.

## 6. 알려진 설계 불일치 (기획자 확인 대기 — 구현은 매핑 DB 기준)

1. 15번 DB `wuxia_insight_sort_like_documents` 획득 사건 = medicine_errand, 그러나 19번 매핑은 raid_omen 폐서고에서 지급 — **매핑 DB 기준으로 구현**, 15번 DB 행 수정은 검수 시.
2. 매핑 총수는 29 (16번 감사 페이지 표기 정정 대상).

## 7. 검증 게이트 (재작업 후)

- 강화된 `reward_pipeline_wave1.rs` (29건 개별 assert) + 기존 웨이브 가드 전부.
- cargo workspace / pytest / export --check(양 번들) / vitest / tsc / build / wasm-pack / 5-viewport QA.
- fable 리뷰에서 실화면 수동 QA: 7사건 플레이·드로어·마스킹·pending·판정 분기·save/reload.

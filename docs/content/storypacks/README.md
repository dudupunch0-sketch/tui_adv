# 스토리팩 DB

Status: 후보 콘텐츠 DB

이 폴더는 `tui_adv`의 storypack 후보를 정리한다. `escape from the office`는 현재 기본 storypack이지만, 폴더의 장기 목적은 office가 아닌 세계관까지 같은 엔진/renderer 계약으로 다루는 것이다.

스토리팩은 런타임에 바로 들어가는 확정 데이터가 아니라, 여러 인카운터 상황 카드와 NPC 후보를 묶는 콘텐츠 설계 단위다. 확정된 일부만 나중에 `src/tui_adv/data/*.yaml` 또는 Rust content bundle로 승격한다.

## 원칙

- 메인 스토리 spine을 대체하지 않는다.
- storypack은 surface, anomaly palette, NPC slot, route hook을 제공한다.
- 실제 회사명, 실제 조직명, 실제 사람, 실제 내부자료처럼 보이는 세부사항은 쓰지 않는다.
- 모든 storypack은 `source_refs`로 idea_box 또는 기존 문서 출처를 남긴다.
- `status: promoted`는 runtime content로 실제 승격된 뒤에만 사용한다.

## status

| status | 의미 |
|---|---|
| `raw` | 아이디어에서 막 추출한 원재료 |
| `candidate` | DB 형식으로 정리된 후보 |
| `curated` | 톤/안전/구조 검토 통과 |
| `promoted` | 런타임 YAML/content bundle로 승격됨 |
| `merged` | 다른 팩에 흡수됨 |
| `rejected` | 톤/안전/중복/범위 문제로 폐기됨 |

## 현재 후보 목록

Machine-readable mirror는 `docs/content/storypack_db/storypacks.json`에 둔다. 문서 `.md` 파일은 사람용 설명/톤/해설이고, JSON은 `src/tui_adv/game/storypack_db.py`의 `validate_storypack_db()`가 참조 무결성을 검사하는 후보 DB다.

| id | 이름 | status | 한 줄 컨셉 | 주 surface | 문서 |
|---|---|---|---|---|---|
| `isolation_pack` | 차원격리팩 | candidate | 회사 일부 공간이 격리되고, 사내 시스템으로 다른 격리자들과 연결된다. | messenger, cctv, meeting_minutes, organization_chart | `isolation_pack.md` |
| `yageunmong_pack` | 야근몽 | candidate | 회사에서 잠깐 엎드려 잠든 주인공이 자각몽 상태의 회사 악몽에서 깨어나려 한다. | messenger, approval_system, meeting_minutes, office_object, security_gate | `yageunmong_pack.md` |
| `wuxia_jianghu_pack` | 이구학지 — 천기록 | candidate | 현대 회사원이 본인 몸과 출근복장 그대로 무협 세계 시장에 전이되고, 흑사방 첫 전투와 청류문 수습생 구간을 거쳐 천기록/천외편린 성장 구조를 경험한다. | commute_rift, market_street, sect_courtyard, cheonggi_record, fragment_choice | `wuxia_jianghu_pack.md` |
| `document_contamination_pack` | 문서오염팩 | raw | 평범한 업무 문서를 열람한 사람들이 현실을 다르게 인식한다. | document_viewer, intranet, organization_chart | 추후 분리 |
| `meeting_reservation_pack` | 회의실예약팩 | raw | 회의실 예약 조건이 과거/미래/다른 세계선 회의 공간으로 이어진다. | reservation_panel, meeting_minutes | 추후 분리 |
| `compensation_strike_pack` | 연봉협상-파업팩 | raw | 보상 시뮬레이션과 파업/협상 시스템이 사람의 가치와 존재를 재계산한다. | payroll_sheet, approval_system, intranet | 추후 분리 |

## 우선순위

첫 설계 slice는 `isolation_pack`으로 시작했다. 2026-05-29부터는 office-only 편향을 줄이기 위해 `wuxia_jianghu_pack`을 첫 비-office 기준 storypack으로 함께 관리한다. 현재 무협팩의 canonical story는 Notion에서 갱신된 **이구학지 — 천기록**이며, 이전의 generic 무협 placeholder는 superseded다. 2026-05-31에는 Notion-origin `야근몽`을 별도 office-family 후보인 `yageunmong_pack`으로 문서화하고, live Notion Markdown 대조 후 관련 idea entry를 done 처리했다. 이는 기본 office runtime을 대체한다는 뜻이 아니다.

나머지 팩은 다음 조건을 만족할 때 별도 문서로 승격한다.

1. 해당 팩의 후보 encounter situation card가 최소 6개 있다.
2. public-safe 민감도 검토를 통과할 수 있다.
3. 메인 story spine과 연결되는 route hook이 있다.
4. 기존 runtime encounter와 중복되지 않는 역할이 있다.

## 관련 문서

- `docs/design/Storypack_World_Model.md`
- `docs/design/Storypack_Encounter_DB.md`
- `docs/content/encounter_db/README.md`
- `docs/content/encounter_db/yageunmong_pack.md`
- `docs/content/characters/README.md`
- `idea_box/done/2026-05-29-notion-storypack-system.md`
- `idea_box/done/2026-05-29-notion-office-yageunmong.md`
- `idea_box/done/2026-05-22-general-corporate-storypacks.md`
- `idea_box/done/2026-05-22-semiconductor-sw-storypacks.md`
- `idea_box/done/2026-05-29-notion-wuxia-igu-hakji-cheonggi-record.md`

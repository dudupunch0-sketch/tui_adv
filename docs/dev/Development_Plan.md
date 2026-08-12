# tui_adv 전체 개발 계획

> **Canonical main plan:** 이 repo의 현재 개발 우선순위, 다음 작업 순서, active direction은 이 파일이 기준이다. 다른 LLM/agent에게 작업을 맡길 때는 “`docs/dev/Development_Plan.md`를 메인 플랜으로 보고 다음 작업을 진행해”라고 지시한다.
>
> 이 문서는 처음 작성된 구현 전 기준점도 포함하므로, 충돌이 있으면 상단의 최신 방향/다음 액션을 우선한다. `README.md`는 요약/실행법, `docs/dev/Checklist.md`는 완료 여부 추적, 아키텍처/스키마 문서는 계약 참조, `idea_box/`는 active plan이 없을 때 보는 backlog다. `.hermes/plans/`는 세션용 작업 계획이며 canonical source가 아니다.
>
> 전투 작업 read order: 이 파일의 active priority → [combat contract index](../content/design_source/contracts/combat_contract_index.md)와 관련 handoff → gameplay 시뮬레이션은 해당 Rust contract, Web 전투 표현은 [Three.js 전투 비주얼 아키텍처](../design/ThreeJS_Combat_Visual_Architecture.md)를 따른다. Storybook shell/story UI 효과는 [TUI Storybook + GlyphFX](../design/TUI_Storybook_GlyphFX_Concept.md)가 소유한다.
>
> 아이디어-설계 흐름은 local design-source 우선이다. `docs/content/design_source/`의 Git-tracked normalized records가 current design records의 정본이며, Notion은 역사적 provenance와 읽기·검수 미러다. 새 변경은 local design source → review → 필요 시 runtime handoff/구현 → Notion mirror 순서로 운영한다.
> 객패귀로 메인 스토리 방향: [통합 설계 v1.1](../content/design_source/arcs/guestpass_homecoming_main_story_v1_1.md)은 승인된 메인 스토리 방향이며, 개별 authoring 검수가 필요하고 runtime에는 아직 구현되지 않았다. 이 링크는 현재 최우선 코드/runtime 작업 순서나 완료 상태를 변경하지 않는다.

> 후일담 source-unstructured 127건은 `docs/content/design_source/reports/afterthought_triage.yml`에서 원문을 보존한 채 분류한다. 해당 ledger는 coverage·중복·누락을 검증하며, 실제 event의 후일담 연결 필드는 별도 기획 검수 전까지 변경하지 않는다.

> WP-P1 Phase B 기준: 복수 후일담은 허용하되 동일 `exclusive_group`에서는 하나만 허용한다. eligibility는 사건 종료 시 판정하고 카드 공개는 `ending_resolution` 또는 `run_end`로 제한한다. 77개 descriptive condition은 조건 overlay로 승인하고, 50개 `designer_review_required`는 승인 전까지 graph/runtime 입력에서 제외한다.

> WP-L18 전투 계약 정본화: [combat contract index](../content/design_source/contracts/combat_contract_index.md)와 [Claude handoff](../content/design_source/handoffs/combat_contract_handoff.md)가 종료 조건·simulation version·표시명·로그·전술 구역의 owner와 검증 경계를 정의한다. 현재 runtime 구현은 완료로 표시하지 않고 handoff_required로 유지한다.
>
> 결정된 종료 우선순위는 forced_stop > captured > surrendered > fled > objective_completed > both_sides_defeated > one_side_defeated > max_ticks다. 낮은 숫자가 우선이며 priority tie는 validator error다. 실제 코드에서 관찰된 simulation version은 v3이며 selector/formula registry v1은 simulation version과 독립된 축이다.
>
> 전투 개입 계약 정본화: 승인된 composite response, typed outcome actions, versioned selector/formula registry, typed strategy overlay, pause lifecycle, resolved decision receipt·checkpoint 및 원자 적용 규칙은 [intervention contract](../content/design_source/contracts/intervention.yml)와 [schema](../content/design_source/schema/combat_intervention.schema.json)가 정본이다. 구현 담당자는 [contract index](../content/design_source/contracts/combat_contract_index.md) → intervention contract → [combat contract handoff](../content/design_source/handoffs/combat_contract_handoff.md)의 WP-I1~I6 순서로 읽는다. 현재 `runtime_status`는 `handoff_required`이며 Rust/TS 구현 완료를 뜻하지 않는다.
>
> 후속 read order: 정본 02 자동전투/상황 트리거 → 정본 05 무기 세부 → 정본 08 전투 예시. 이 세 항목은 이번 WP-L18에서 구현 완료로 승격하지 않는다.

## 0.0 계획 문서 우선순위

1. `docs/dev/Development_Plan.md`: 단일 메인 플랜. 현재 방향, 다음 작업, 우선순위, phase 순서를 여기서 판단한다.
2. `docs/dev/Checklist.md`: 완료 여부 추적용 체크리스트. 독립적인 다음 계획을 두지 않는다.
3. `docs/dev/Rust_Core_Dual_Renderer_Architecture.md`, `docs/dev/Data_Schema.md`, `docs/design/ThreeJS_Combat_Visual_Architecture.md`, `docs/design/UI_Rules.md`, `docs/dev/TUI_Layout.md`: 설계 계약/참조 문서. 작업 순서의 source of truth가 아니다.
4. `README.md`: 사람용 빠른 안내와 실행법. 긴 다음 작업 목록은 이 파일로 복제하지 않는다.
5. `idea_box/`: active plan/todo가 없거나 사용자가 명시적으로 요청했을 때 처리하는 backlog. Notion-origin entry는 보존된 provenance를 참고하되 current design records는 local design source를 기준으로 삼는다.
6. `.hermes/plans/`: 일회성 세션 artifact. 완료되었거나 이 파일에 흡수된 계획은 정리한다.

## 0.0a 2026-05-31 아이디어-설계 운영 규칙 (local design-source 기준)

앞으로 새 설계 아이디어는 다음 흐름으로 처리한다.

1. 사용자가 local design source에 아이디어를 기록하거나, 기존 Notion provenance를 검토 대상으로 지정한다. current design records의 정본은 local design source다.
2. agent는 manifest/governance를 먼저 읽고 관련 local records만 선택 참조한다. Notion-origin 내용이 필요할 때만 보존된 provenance와 page id/title/url을 확인하고, `idea_box/inbox/*.md`에는 해당 provenance와 `related_docs`를 남긴다.
3. 다음에 실제 설계할 항목은 설계 아이디어 문서 중 하나를 이 파일의 active main plan / “현재 최우선 남은 작업”으로 격상시킨 뒤 진행한다.
4. 설계가 끝나면 local design source와 결과 설계 문서를 비교해 방향, 톤, 핵심 제약, non-goals가 일치하는지 확인한다. 필요하면 Notion mirror에도 검수 결과를 반영한다.
5. local design source 반영 또는 명시적 폐기/병합 기록이 끝난 경우에만 해당 idea entry를 `done` 처리한다. 단순 import나 설계 아이디어 문서 작성만으로는 `done` 처리하지 않는다.

2026-06-01 추가 규칙: `이구학지 — 천기록`의 current design records는 `docs/content/design_source/`의 manifest/governance와 관련 normalized records를 우선한다. 과거 Notion source의 상위 문서·하위 관리 문서·DB row는 provenance와 mirror 검수 자료로 보존하며, Notion row를 읽었다는 이유만으로 runtime 구현 완료나 기본 bundle 반영으로 표시하지 않는다. runtime status는 local manifest/record에서 먼저 갱신하고, 필요할 때 Notion mirror를 갱신한다.

## 0.0b 2026-06-01 default storypack 전환

현재 메인/default storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**으로 전환한다. 새 Web player 기본 UX, UI/UX QA, 이후 runtime slice는 이구학지를 우선한다.

첫 비-office 기준팩은 `wuxia_jianghu_pack` / **이구학지 — 천기록**이다.

- Web player의 새 게임과 terminal `--scene content` 기본 실행은 이구학지 bundle을 기본으로 로드한다.
- 이구학지 기본 run은 `igu-hakji.rust.save.v1` / `igu-hakji.last-run-summary.v1` 계열 localStorage key를 사용해 기존 office save와 섞이지 않게 한다.
- 기존 `escape from the office` content와 `escape-office.*` save key는 legacy/parity/cleanup 대상으로 남긴다.
- Rust/WASM GameCore가 이구학지 Web player의 필수 경로다. generated wasm package가 없을 때 office TypeScript mirror로 조용히 fallback하지 않는다.
- 아래 과거 섹션의 “기본 office bundle 유지”, “preview only” 문구는 당시 slice 기록이다. 최신 작업 판단은 이 섹션과 상단 우선순위를 우선한다.

## 0. 2026-05-22 방향 갱신

현재 개발 방향은 다음과 같이 고정한다.

```text
Rust GameCore
  ├─ Web Storybook shell / DOM HUD
  │   ├─ GlyphFX: story·UI effects
  │   └─ Three.js: ScenePage.combat 3D hex / combat VFX
  └─ SuperLightTUI terminal renderer
      └─ terminal-native fallback / horror edition
```

- Web Storybook이 플레이어용 shell/HUD다. GlyphFX는 이미지/장면 컷, 대화, 텍스트 등 story/UI 효과를 맡고 Three.js는 `ScenePage.combat` 전투 보드와 공간 VFX를 맡는다.
- 전투 renderer는 고정 `OrthographicCamera`, flat-top axial 3D hex, semi-SD modular GLB 방향이다. 7×6 보드와 12명은 첫 검증 fixture이며 영구 gameplay cap이 아니다. PC-first 1080p/60은 전투 성능 기준이고 기존 모바일 텍스트RPG 게임 프레임을 desktop layout으로 교체하지 않는다.
- Rust terminal 경로는 SuperLightTUI 기반 renderer로 유지한다. fallback은 우선순위/환경 호환성의 의미이며, 단순 debug dump를 뜻하지 않는다.
- Python/Textual과 TypeScript mirror core는 전환기 legacy/parity oracle이다. 새 게임 규칙을 그쪽에 계속 복제하지 않는다.
- 세부 아키텍처는 `docs/dev/Rust_Core_Dual_Renderer_Architecture.md`를 따른다.
- wire schema와 구현 계약은 `docs/dev/Data_Schema.md`의 renderer-neutral content bundle, `ScenePage`, action id, `EffectCue`, WASM JSON boundary 설계를 따른다.


> **과거 히스토리 아카이브 안내:**
> 과거 0.1 ~ 0.96 슬라이스 완료 이력, 초기 Phase 0~10 설계, 테스트 전략, 이전의 완료된 기반 목록 등은 AI Agent 컨텍스트 관리 효율성을 위한 100KB 문서 제한 규정에 따라 별도의 아카이브 문서인 [Development_Plan_Archive.md](file:///C:/Users/82105/.gemini/antigravity/worktrees/tui-adv/enable-wsl-worktree-support/docs/dev/Development_Plan_Archive.md)로 분리/이전되었습니다.

---
현재 최우선 남은 작업:

- (게임루프 트랙) **완료·review-fixed** — Game Loop Expansion, Slice 3 (Leveling·Insights·Item Details). 기준 구현 `c33fc75` 이후 RF1 `3230df5`, RF2 `256aa4f`, RF3 `2f39cd2`에서 terminal training guard, ordered insight dedupe, optional inventory fallback, 390/414 HUD geometry gate를 보정했다. RF4 문서 closeout까지 완료 후 다음 active work는 Event/Stage/ContentBlock 전환으로 유지한다. 계획의 `.story-progress-mini` 명칭과 실제 renderer/QA contract의 `.story-progress-rail`은 naming deviation으로 기록한다.
- (게임루프 트랙) **완료** — 이구학지 게임 루프 2차 확장 (Slice 2: Content-owned labels, Check resolution reveal, Collapse gate & second wind).
- (UI 트랙) **ACTIVE Web renderer track: Three.js WP2 board contract; implementation blocked on WP1 merge/review.** PR #217의 WP1 pure adapter/producer fixture는 open·green이지만 아직 unreviewed/unmerged다. merge 뒤 그 public API와 fixture SHA를 정본과 다시 대조한 다음에만 WP2 구현을 시작한다. WP2는 Web host가 7×6 bounds를 소유하고 공식 `three@0.185.1`, fixed OrthographicCamera, 42-tile board, final-frame anchor marker, fail-soft DOM fallback과 mount/dispose lifecycle만 추가한다. exact 8 owned files, API, acceptance, browser/performance evidence와 stop condition은 `docs/design/ThreeJS_Combat_Visual_Architecture.md` §11.2가 소유한다. `render.ts`, Rust/GameCore/schema/fixture, GLB/VFX/animation, gate 해제는 비범위다. 이 UI 트랙은 Rust I2b/I7과 다른 game/content track의 우선순위를 대체하지 않는다. 다음 액션은 **WP1 review/merge → merge SHA/API/fixture 재대조 → 한 implementation subagent가 WP2 한 커밋 구현 → 독립 review** 순서다.
- (UI 트랙 방향) Web Storybook shell/GlyphFX story·UI 효과와 Three.js 전투 renderer를 분리한다. 전투 시각 정본은 `docs/design/ThreeJS_Combat_Visual_Architecture.md`다. `docs/design/Mobile_Ink_Storybook_UI.md`는 모바일 텍스트RPG layout/gamefeel 기록만 유지하며 수묵 art direction은 superseded다. 과거 작업 지시서 `fable_ui_step1_2607111330.md` (Rev 2)의 완료 이력은 보존한다.

1. 무협 storypack preview/main의 다음 작업은 `wuxia_sado_battle_loss_route_bridge_implementation` runtime slice다. `wuxia_sado_final_battle_container_followup_handoff`가 playable defeat-route bridge / battle-loss route UX를 다음 runtime 후보로 선택했으므로, 기존 encounter schema로 `final_combat_result_battle_loss_seeded`를 실제 플레이 경로에서 만들고 기존 Rust final epilogue battle-loss consumer로 넘긴다. `docs/design/Wuxia_Final_State_Routing.md`가 canonical final inputs/result priority/alias policy/final epilogue seed-consumption contract와 return/settlement/battle-loss/final-state-collapse/final-battle-container/follow-up handoff를 소유한다.
   - 현재 Web/terminal default storypack은 `wuxia_jianghu_pack` / **이구학지 — 천기록**이다.
   - `escape from the office` / office isolation 계열은 legacy/parity content로 유지한다.
   - machine-readable storypack DB, preview mode 결정, `wuxia_commute_rift_arrival`, `wuxia_heuksa_bang_first_fight`, `wuxia_cheonggi_record_first_fragment`, `wuxia_seo_harin_rescue`, `wuxia_cheongryu_apprentice_entry`, `wuxia_cheongryu_chore_sparring`, `wuxia_cheongryu_raid_route_split`, `wuxia_cheongryu_raid_wounded_fallback`, `wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`, `wuxia_wounded_shelter_dawn_offers`, `wuxia_mumyeong_first_sighting`, `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_reads_orthodox_style`, `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`, `wuxia_mumyeong_request_for_aid`, `wuxia_mumyeong_awakening`, `wuxia_qingliu_attack_after_war`, `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_seoharin_empty_place`, `wuxia_seoharin_left_meal`, `wuxia_sado_final_phase_1_price_tag`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`, `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, `wuxia_seoharin_qingliu_resolution`, `wuxia_seoharin_unsaid_stay`, `wuxia_cheongirok_resolution`, `wuxia_black_serpent_aftermath`, boss follow-up handoff, failed-aid follow-up handoff, Web/default 이구학지 start/save wiring, terminal default 이구학지 bundle 선택은 완료했다.
   - copy-style reveal은 `copy_style_hint_recorded`, `copied_form_family_seen`, `copy_is_surface_not_root`, `breath_mismatch_marks_copy`, `understanding_is_not_copying`, `fragment_candidate_variation_foreshadowed` hook을 남겼다.
   - orthodox style trace는 `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `hyeonakmun_trace_suspected`, `bokho_geumsaesu_name_recorded`, `departure_truth_still_incomplete` hook을 남겼다.
   - midgame reunion은 `mumyeong_midgame_reunion_resolved`, `mumyeong_mirror_thread_deepened`, `seoharin_does_not_call_mumyeong_traitor`, `boss_used_mumyeongs_wound`, `mumyeong_truth_still_incomplete`, `rival_mirror_relationship_deepened`, `hyeonakmun_trace_shared_without_accusation` hook을 남겼다.
   - boss first appearance는 `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `boss_reads_people_not_forms`, `boss_is_final_logic_wall`, `mumyeong_follows_power_that_saw_his_wound`, `qingliu_cannot_outmuscle_boss_yet` hook을 남겼다.
   - mumyeong request for aid는 `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `rejected_aid_letters_read`, `inn_rumor_thread_followed`, `seoharin_failed_aid_question_asked`, `failed_aid_record_kept_unshown`, `mumyeong_tried_to_save_qingliu`, `orthodox_refusal_broke_mumyeong`, `boss_logic_found_mumyeongs_wound`, `aid_refusal_precedes_departure_truth`, `seoharin_does_not_know_failed_aid` hook과 `rejected_aid_letter_fragment` item을 남겼다.
   - mumyeong awakening은 `mumyeong_awakening_resolved`, `mumyeong_awakening_thread_opened`, `copy_corruption_thread_opened`, `mumyeong_copy_bloomed_from_anger`, `copy_is_wound_not_growth`, `protagonist_understands_where_mumyeong_overlays`, `awakening_points_to_hyeonakmun_without_full_truth`, `salvation_truth_still_unready` hook을 남겼다.
   - `wuxia_qingliu_attack_after_war` 구현은 `mumyeong_awakening_resolved`, `mumyeong_awakening_thread_opened`, `copy_corruption_thread_opened`, `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `midgame_continuity_started`를 요구하고, `qingliu_attack_after_war_resolved`로 반복을 막는다.
   - stable choice id는 `inspect_bokho_lock_scars`, `compare_hyeonakmun_trace_to_qingliu_wounds`, `ask_seo_harin_what_she_saw_afterward`, `stop_before_replaying_the_attack`로 고정한다.
   - common hook은 `qingliu_attack_after_war_resolved`, `qingliu_attack_trace_confirmed`, `hyeonakmun_attack_thread_opened`, `qingliu_attack_trace_points_to_hyeonakmun`, `bokho_geumsaesu_used_on_qingliu`, `seoharin_saw_aftermath_not_full_truth`, `main_sect_not_directly_accused`, `full_flashback_still_unopened`, `destination_id: cheongryu_outer_courtyard`다.
   - `wuxia_mumyeong_destroys_orthodox_sect` 구현은 `qingliu_attack_after_war_resolved`, `qingliu_attack_trace_confirmed`, `hyeonakmun_attack_thread_opened`, `mumyeong_awakening_resolved`, `midgame_continuity_started`를 요구하고, `mumyeong_destroys_orthodox_sect_resolved`로 반복을 막는다.
   - stable choice id는 `read_hyeonakmun_empty_gate_record`, `trace_bokho_lock_to_mumyeong`, `ask_why_seoharin_never_heard_full_story`, `stop_before_counting_the_dead`로 고정한다.
   - common hook은 `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `destination_id: cheongryu_outer_courtyard`다.
   - `wuxia_boss_recruits_mumyeong` 구현은 `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `midgame_continuity_started`를 요구하고, `boss_recruits_mumyeong_resolved`로 반복을 막는다.
   - stable choice id는 `trace_boss_offer_after_hyeonakmun`, `read_mumyeong_choice_without_excusing_it`, `search_black_serpent_recruitment_record`, `stop_before_following_him_into_black_serpent`로 고정한다.
   - common hook은 `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `destination_id: cheongryu_outer_courtyard`다.
   - `wuxia_mumyeong_departure_truth_summary` 구현은 `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `mumyeong_awakening_resolved`, `midgame_continuity_started`를 요구하고, `mumyeong_departure_truth_summary_resolved`로 반복을 막는다.
   - stable choice id는 `assemble_departure_truth_without_delivering`, `compare_failed_aid_to_recruitment_offer`, `ask_seoharin_what_she_is_ready_to_hear`, `seal_truth_until_mumyeong_faces_it`로 고정한다.
   - common hook은 `mumyeong_departure_truth_summary_resolved`, `sealed_departure_truth_summary_prepared`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard`다.
   - `wuxia_seoharin_empty_place` 구현은 `mumyeong_departure_truth_summary_resolved`, `sealed_departure_truth_summary_prepared`, `truth_delivery_still_unopened`, `midgame_continuity_started`를 요구하고, `seoharin_empty_place_resolved`로 반복을 막는다.
   - stable choice id는 `ask_who_kept_the_empty_place`, `leave_the_place_unclaimed`, `set_down_the_work_notebook_briefly`, `step_back_without_naming_mumyeong`로 고정한다.
   - common hook은 `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard`다.
   - presentation은 `visual_id: wuxia_seoharin_empty_place`, `speaker: 서하린`, `layout: empty_place_memory`, stable terms `[서하린, 무명, 청류문, 목검]`다.
   - `preview launcher/UI wiring`은 이미 구현했으므로 후속 slice에서 다시 구현하지 않는다.
   - route opener 후속도 faction/route graph schema를 열지 않고 flags/clues/log/presentation으로만 남긴다.
   - `yageunmong_pack`은 docs/data 후보로 반영됐지만 기본 office runtime을 대체하지 않는다. 야근몽 runtime은 별도 preview 후보로만 연다.
   - legacy office `content.bundle.json`, `src/tui_adv/data/*.yaml`, `escape-office` save/localStorage key는 계속 바꾸지 않는다.
   - 천외편린/각성편린 3택 성장 schema, relation/debt/faction/companion schema는 별도 검증 전까지 열지 않는다.
   - 보스 첫 등장, 무명의 도움 요청 실패 기록, 무명의 각성, 청류문 흔적 조사, 현악문 멸문 consequence trace, 보스 스카웃 trace, 무명 이탈 진실 sealed summary, 서하린 empty-place bridge는 열었다.
   - `wuxia_seoharin_left_meal`은 `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened`, `midgame_continuity_started`를 요구하고, `seoharin_left_meal_resolved`로 반복을 막는다.
   - stable choice id는 `eat_the_left_meal_quietly`, `thank_seoharin_for_the_bowl`, `joke_about_who_ordered_extra_rice`, `pass_without_eating_the_meal`다.
   - common hook은 `seoharin_left_meal_resolved`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard`이며 긍정 선택은 `seoharin_axis_deepened`/`qingliu_belonging_warmed`, 거절 선택은 `seoharin_axis_still_open`/`left_meal_left_untouched`를 남긴다.
   - `wuxia_sado_final_phase_1_price_tag`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`, `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, `wuxia_seoharin_qingliu_resolution`, `wuxia_seoharin_unsaid_stay`, `wuxia_cheongirok_resolution`, `wuxia_black_serpent_aftermath`, `wuxia_final_epilogue_renderer_contract`, `wuxia_return_settlement_epilogue_contract`은 preview/main runtime 구현 완료다. full return/settlement ending, return/settlement save/archive schema, full combat resolver 후보는 보류한다.
   - Rust GameCore / `ScenePage` / WASM JSON boundary 책임 분리와 renderer-neutral 원칙을 유지한다.

전환 중 유지:

1. Python/Textual 직접 플레이와 smoke는 legacy/parity oracle로 유지하되 새 gameplay rule을 추가하지 않는다.
2. TypeScript mirror core와 fake-TUI browser shell은 generated wasm package가 없는 개발 환경의 fallback/parity oracle로 유지한다.
3. 새 게임 규칙, route truth, eligibility, outcome, ending, achievement는 renderer가 아니라 Rust core에 추가한다.
4. 현실 탈출 후일담 slice에서도 renderer가 후일담을 재판정하지 않는다. Web Storybook과 SuperLightTUI는 core `ScenePage.body_blocks`를 표시한다.
5. 모바일 픽셀 스토리북 UI redesign에서도 Web renderer는 `ScenePage` semantic field와 action id를 표시/전달만 하며, gameplay truth와 renderer-neutral schema를 변경하지 않는다.

나중:

1. 대표 Web/Rust route smoke가 legacy coverage를 대체하면 Python/Textual과 TypeScript mirror retire 여부를 다시 결정한다.
2. 정복/진실/재난 타입별 변형 콘텐츠 확대
3. 현실 탈출 후일담 다중 변형: `escape_rooftop_signal`, `escape_parking_lot`, `escape_lobby_revolving_door` 같은 다른 escape 엔딩으로 확장할지 결정한다.
4. 후일담 변형이 2개 이상이 되고 단순 text blob이 부족해지면 별도 `aftermath` schema/field slice를 검토한다.
5. 꿈 엔딩을 실제 콘텐츠로 구현할지 결정한다.
6. Tauri/Electron desktop wrapper 재검토: native file dialog, offline file import/export, OS-level 알림/업데이트 같은 Web-only 한계를 실제 요구로 확인한 뒤 별도 slice로 연다.
7. optional inline image는 terminal cell/GlyphFX baseline 밖 future backlog로 둔다. Kitty/Sixel/iTerm2 capability 요구가 실제로 생길 때 별도 slice로 연다.
8. Web player start/save UX first slice 후속: save JSON export/import, settings/reduce-motion UI, 오늘의 seed는 별도 승격 전까지 열지 않는다.
9. 여러 히든 현실 보물
10. Combat implementation is now tracked by docs/design/Combat_System_Implementation_Plan_Index.md; Wave 1 Step 1~3, Wave 2 Step 1~4, and Wave 3 Step 1a (fixed integer positions, role/target contract, simultaneous tick frame, execution mode parity and dual logs, data-driven collision/attack/damage/effect resolution, deterministic multi-agent conclusion/termination sidecar, spectator view adapter) are implemented and verified. Wave 2 Step 4 landed the deterministic conclusion sidecar (`crates/escape-core/src/combat_conclusion.rs`, `crates/escape-core/tests/combat_conclusion_wave2.rs`). Wave 3 Step 1a landed a core-only spectator adapter (`crates/escape-core/src/combat_spectator.rs`, `crates/escape-core/tests/combat_spectator_wave3.rs`, 12 tests: `spectate_is_deterministic_for_identical_input`, `attack_roll_and_effect_suppressed_never_leak_into_any_log`, `hidden_conditional_and_unregistered_effect_ids_are_masked`, etc.) that derives per-tick chess-piece frames, Attack/Hit/Evade cues, and a template-id dual log from an existing `CombatResolutionResult` without recomputing judgement; `BalanceBroken`/`Incapacitated` cues were deliberately deferred because `CombatResolutionFrame` has no per-tick combatant state snapshot yet. Wave 3 Step 2a landed `EncounterCombatDef`/`EncounterCombatKind` schema, index-time validation (11 hard-error rules), and a systemic-kind combat producer wired into `scene_page_from_turn_view`, but authored no real combat content yet (`crates/escape-core/tests/encounter_combat_wave3.rs`, 21 tests). Wave 3 Step 2b landed the first real content: one systemic combat encounter (`wuxia_combat_spectator_preview_bout`, `cheongryu_outer_courtyard`) authored in the wuxia storypack-preview bundle using only the canonical 정본 11 standard-combatant numbers (power 40 / ability multiplier 1.0 / accuracy 100 / defense 5 / health+breath 100), gated behind `combat_spectator_preview_unlocked` (an ordinary-play-unreachable flag) because the spectator renderer does not exist yet; `wuxia_combat_spectator_preview_bout_first_hit_damage_is_1333_hundredths` pins the authoring numbers and the resolver formula together (`crates/escape-core/tests/encounter_combat_wave3.rs`, 28 tests total). Early-termination/tick abort, rout/surrender/reinforcement/formation/cohesion/background combat, mixed/scripted encounter authoring and their intervention-pause flow (Wave 3 Step 2c), terminal/Web renderer (Step 1d), combat result caching, and data-driven balance remain unstarted. Execute one plan file per coding subagent slice; advanced AI behavior, renderer adapter, and balance remain split by approval.
11. 무협 storypack 후속: 정파/사파/천기·귀환 opener(`wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`), deferred-offer card `wuxia_wounded_shelter_dawn_offers`, common midgame bridge `wuxia_mumyeong_first_sighting`, rival first confrontation `wuxia_mumyeong_first_confrontation`, copy-style reveal `wuxia_mumyeong_copy_style_reveal`, orthodox style trace `wuxia_mumyeong_reads_orthodox_style`, midgame reunion `wuxia_mumyeong_midgame_reunion`, boss first appearance `wuxia_boss_first_appearance`, Mumyeong aid request `wuxia_mumyeong_request_for_aid`, Mumyeong awakening `wuxia_mumyeong_awakening`, Qingliu attack trace `wuxia_qingliu_attack_after_war`, Hyeonakmun consequence trace `wuxia_mumyeong_destroys_orthodox_sect`, boss recruitment trace `wuxia_boss_recruits_mumyeong`, sealed departure truth summary `wuxia_mumyeong_departure_truth_summary`, Seo Harin empty-place bridge `wuxia_seoharin_empty_place`, Seo Harin left-meal bridge `wuxia_seoharin_left_meal`, Sado final battle container `wuxia_sado_final_battle`, Sado final phase 1 price-tag/ledger bridge `wuxia_sado_final_phase_1_price_tag`, Sado final phase 2 weakpoint-control bridge `wuxia_sado_final_phase_2_weakpoint_control`, Sado final phase 3 outside-calculation bridge `wuxia_sado_final_phase_3_outside_calculation`, boss resolution route seed bridge `wuxia_boss_resolution`, Mumyeong resolution route seed bridge `wuxia_mumyeong_resolution`, Seo Harin/Qingliu resolution route seed bridge `wuxia_seoharin_qingliu_resolution`, Seo Harin return/settlement trigger `wuxia_seoharin_unsaid_stay`, Cheonggi Record resolution route seed bridge `wuxia_cheongirok_resolution`, Black Serpent aftermath seed bridge `wuxia_black_serpent_aftermath`, final epilogue seed consumer `wuxia_final_epilogue_renderer_contract`, return/settlement branch consumer `wuxia_return_settlement_epilogue_contract`, battle-loss epilogue branch consumer `wuxia_battle_loss_epilogue_contract`, battle-loss epilogue follow-up handoff, final-state canonical collapse runtime, final-state canonical collapse follow-up handoff, Sado final battle container runtime, Sado final battle container follow-up handoff까지 구현/검증 완료했다. 다음 후보는 `wuxia_sado_battle_loss_route_bridge_implementation` runtime slice다.
12. 천외편린/각성편린 3택 reward/ability schema는 schema-less bridge가 충분히 검증된 뒤 별도 slice로 검토한다.
13. 야근몽 storypack preview 후속: `yageunmong_late_night_desk_awake` 또는 각성편린 3택 preview를 별도 storypack preview로 열지 결정한다.

## Phase 16: Reward Pipeline Wave 1 구현 (2026-07-17)

`fable_rewardpipeline_wave1_step1_2607171903.md`의 WP-R1~R5를 구현했다. GameCore는 스킬·칭호·관계 상태와 보상 지급을 additive로 소유하고, preview에는 매핑 등장분 7개 스킬·5개 칭호·5개 기연·7개 아이템을 정의했다. 청류문 수련기 신규 사건 7개를 Story → Choice → ResultStage로 authoring해 이구학지 coverage를 44/44에서 51/51로 확장했다.

Web Storybook은 기존 드로어 계약 안에서 스킬·칭호 섹션, 등급 위계, `reveal_immediate: false` 마스킹, 보상 획득 로그/비트를 표시한다. 관계 수치는 `ScenePage`나 Web에 노출하지 않는다. 효과 수치·전투 resolver·웨이브 B 확률 스케줄러·계정 도감은 범위 밖으로 유지한다.

검증 가드는 중복 지급 무시, 관계 델타 누적, 판정 성공/실패 배타성, 신규 카드·매핑 coverage를 고정한다. Notion DB 30행 원문은 repo snapshot에 없으므로 ID·획득 시점 규칙을 플랜 기준으로 authoring했으며, 원문 대조와 수동 7개 사건 acceptance는 후속 Fable review에서 수행한다. 상세 내용은 `fable_rewardpipeline_wave1_step2_report.md`에 기록했다.

## 9. 주요 리스크

### 범위 과대

재난 타입, 엔딩, 현실 연결이 모두 커질 수 있다.

대응:

- 1차는 불명 재난(`unknown_isolation`)만 구현한다.
- 엔딩은 탈출과 첫 히든 힌트만 우선 구현한다.
- 정복/진실/재난별 특수 규칙은 구조만 열어 둔다.

### 콘텐츠와 코드 결합

인카운터가 코드에 박히면 확장이 어려워진다.

대응:

- 가능한 빨리 데이터 파일로 분리한다.
- 데이터 스키마와 검증 테스트를 둔다.

### 현실 위치 정보 노출

실제 사무실 위치가 Git에 올라갈 수 있다.

대응:

- 실제 위치는 `private/` 또는 `.local` 파일에만 둔다.
- 공개 예시는 `secrets.example.yaml`로 따로 둔다.
- 릴리즈 체크리스트에 비밀 정보 검사 항목을 넣는다.

### Renderer와 core 결합

Web 또는 terminal renderer가 게임 규칙을 다시 구현하면 Rust GameCore 공통화가 깨진다.

대응:

- `escape-core`가 action eligibility, outcome, ending, achievement의 truth를 소유한다.
- Web Storybook과 SuperLightTUI terminal은 `ScenePage`/`ActionResult`를 표시하고 action id만 전달한다.
- SuperLightTUI는 `escape-terminal`에만 추가하고 `escape-core`에는 절대 넣지 않는다.

## 10. 다음 액션

> **2026-07-17 업데이트:** `fable_eventstage_step1_2607171255.md`의 Wave 1(WP-D1/S1/C1/C2/W1)을 완료했다. 10개 이구학지 사건을 ordered Story → Choice → per-choice ResultStage로 전환했고, `branch: success|failure` 필터와 직접 ResultStage cursor/save 호환을 추가했다. 다음 우선순위는 Wave 2 사건 마이그레이션과 renderer-neutral stream 추가 검증이며, 아래의 기존 전환 순서는 이 완료 상태를 전제로 해석한다.

> **2026-07-17 Wave 2 업데이트:** `fable_eventstage_wave2_step1_2607171454.md`의 WP-S1/C1/C2/C3/W1/D1/D2를 완료했다. 이구학지 staged coverage를 14개에서 30개/44개로 확장했고, midgame 16개 사건을 ordered Story → Choice → per-choice ResultStage로 전환했다. index-time branch 위치·dangling stage/choice ref 검증과 `document` block surface를 추가했다. 다음 우선순위는 Wave 3(최종전/결산/붕괴 게이트는 별도 설계)이며, 일러스트 자산 트랙과 office pack은 계속 비범위다.

> **2026-07-17 Wave 3 업데이트:** fable_eventstage_wave3_step1_2607171715.md와 Event_Stage_Wave3_Design.md의 WP-S1→WP-D1을 구현했다. 최종전·결산·귀환/정착 후일담·붕괴 게이트 14개를 ordered Story → Choice → per-choice ResultStage로 전환했고, 이구학지 preview staged coverage는 44/44가 되었다. collapse active-event precedence, revive/no-retrigger, rest/death ending supersede와 final epilogue ending supersede를 Rust 회귀 테스트로 고정했다. generated preview bundle, WASM, Web build, art gate, 5 viewport QA가 통과했으며, 수동 acceptance와 Notion 원격 ledger reverse sync는 Fable review/연결 복구 후 수행한다.

> 각주(2026-08-01): 위 44/44는 Wave 3 시점 수치다. 이후 Reward Pipeline Wave 1에서 청류문 수련기 신규 사건 7개가 추가되어 이구학지 preview staged coverage는 현재 51/51이다 (`event_stage_wave3.rs`의 `wuxia_preview_has_full_51_event_coverage`).

1. **현재 최우선: Event/Stage/ContentBlock 전환 closeout.** 2026-08-01 코드 실측 결과 남은 순서 2~6은 이미 구현·테스트로 고정되어 있다: (2) ordered Stage/ContentBlock schema + index-time validation은 `content.rs:273-305`(`EventDef`/`EventStageDef`/`ContentBlockDef`)와 `content.rs:903-1046`(`validate_event`), (3) Event 내부 cursor/save migration은 `state.rs:78-84`(`active_event_id`/`event_stage_index`/`event_next_stage_id`, 전부 `#[serde(default)]`)와 `turn.rs:281-320,681-734`, (4) renderer-neutral ordered stream은 `scene_page.rs:37-39,462-587`(`content_stream`), (5) Web 고정 5영역 제거는 `web/src/ui/storybook/render.ts:158-234`(`renderOrderedStoryFlow`, `web/src/ui/storybook/render.test.ts:184`로 고정), (6) 이구학지 대표 Event 점진 마이그레이션은 51/51 완료(`event_stage.rs`, `event_stage_wave1.rs`, `event_stage_wave2.rs`, `event_stage_wave3.rs`의 `wuxia_preview_has_full_51_event_coverage`)로 각각 검증된다. `fable_eventstage_closeout_step1` slice는 남아 있던 branch 해석 결함 하나를 닫았다: ResultStage의 success/failure 분기가 `content_stream`에서만 필터링되고 `TurnView.body`·`ScenePage.body_blocks`·`dialogue_entries`에는 두 분기가 동시에 노출되던 스포일러 누출을, `EventStageDef::visible_blocks`(`content.rs`) 단일 해석 지점으로 통합해 모든 텍스트 표면에 동일 적용했다 (`crates/escape-core/tests/event_stage_branch_surfaces.rs`로 회귀 고정). **legacy adapter 결정**: 별도 adapter 모듈은 만들지 않는다. flat encounter는 core가 `scene_page.rs:537-586`의 fallback으로 ordered `content_stream`을 이미 만들어 주고 있고 (office bundle의 `event` 필드 0건, 위 188행의 office 비범위 결정이 근거), office/isolation pack 21개 encounter의 staged 전환이 승격되기 전까지 새 adapter를 추가하지 않는다. 남은 closeout 항목은 1개이며 저장소 밖 수동 acceptance다. local design source의 runtime status/coverage를 먼저 갱신하고, 필요하면 Notion mirror에 반영한다.
2. `wuxia_sado_battle_loss_route_bridge_implementation` runtime slice는 구현/검증 완료됐다. `wuxia_sado_final_battle`에 loss-route 선택지 `throw_away_every_lever_against_sado`를 추가했고, 새 bridge encounter `wuxia_sado_battle_loss_route_bridge`가 `final_combat_result_battle_loss_seeded`와 7개 completion flags를 방출해 기존 BattleLoss epilogue consumer로 routing한다. `wuxia_sado_final_phase_1_price_tag`에 `sado_battle_loss_route_chosen`이 `forbidden_flags`에 추가되어 victory/loss route가 상호 배제된다. `wuxia_sado_final_battle`, `wuxia_sado_final_phase_1_price_tag`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`, `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, `wuxia_seoharin_qingliu_resolution`, `wuxia_seoharin_unsaid_stay`, `wuxia_cheongirok_resolution`, `wuxia_black_serpent_aftermath`, `wuxia_final_epilogue_renderer_contract`, `wuxia_return_settlement_epilogue_contract`, `wuxia_battle_loss_epilogue_contract`, `wuxia_final_state_canonical_collapse_contract`, final epilogue UX/playtest follow-up, return/settlement contract handoff, return/settlement epilogue follow-up handoff, battle-loss epilogue follow-up handoff, final-state canonical collapse follow-up handoff, Sado final battle container follow-up handoff, Sado battle-loss route bridge는 final state routing contract 기준으로 구현/검증 완료했다. 이 작업의 후속 handoff는 Event/Stage 전환 뒤 재평가한다.
   - `wuxia_commute_rift_arrival`, `wuxia_heuksa_bang_first_fight`, `wuxia_cheonggi_record_first_fragment`, `wuxia_seo_harin_rescue`, `wuxia_cheongryu_apprentice_entry`, `wuxia_cheongryu_chore_sparring`, `wuxia_cheongryu_raid_route_split`, `wuxia_cheongryu_raid_wounded_fallback`, `wuxia_baekdo_medicine_debt`, `wuxia_black_heaven_escape_price`, `wuxia_heavenly_archive_previous_outsiders`, `wuxia_wounded_shelter_dawn_offers`, `wuxia_mumyeong_first_sighting`, `wuxia_mumyeong_first_confrontation`, `wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_reads_orthodox_style`, `wuxia_mumyeong_midgame_reunion`, `wuxia_boss_first_appearance`, `wuxia_mumyeong_request_for_aid`, `wuxia_mumyeong_awakening`, `wuxia_qingliu_attack_after_war`, `wuxia_mumyeong_destroys_orthodox_sect`, `wuxia_boss_recruits_mumyeong`, `wuxia_mumyeong_departure_truth_summary`, `wuxia_seoharin_empty_place`, `wuxia_seoharin_left_meal`, `wuxia_sado_final_phase_1_price_tag`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`, `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, `wuxia_seoharin_qingliu_resolution`, `wuxia_seoharin_unsaid_stay`, `wuxia_cheongirok_resolution`, `wuxia_black_serpent_aftermath`는 이미 이구학지 runtime bundle에 구현되어 있다.
   - `wuxia_final_epilogue_renderer_contract_handoff`는 추가 seed bridge 없이 implementation slice를 열 수 있다고 결정했고, 해당 slice는 structured `ScenePage.body_blocks` convention으로 구현 완료됐다. Rust GameCore가 candidate seed consumption, `final_result_priority`, suppress, card ordering을 소유하고 Web Storybook/SuperLightTUI는 core 결과를 표시만 한다.
   - Web/terminal default storypack은 이구학지이며, terminal도 `--scene content` 기본 실행에서 같은 bundle을 사용한다. `--storypack-preview wuxia_jianghu_pack`는 명시적 동일 경로로 남겼고, Web의 별도 preview launcher는 이구학지가 기본이 되면서 목록에서 비워 두었다.
   - 이구학지 runtime은 계속 `storypack_preview` 계열 bundle metadata와 `default_location: wuxia_commute_rift` 시작점을 유지하되, Web player에서는 이를 `storypack_main`으로 감싼 default bundle JSON으로 사용한다.
   - `wuxia_mumyeong_copy_style_reveal` 구현으로 `mumyeong_copy_style_reveal_resolved`, `copy_style_hint_recorded`, `copied_form_family_seen`, `copy_is_surface_not_root`, `breath_mismatch_marks_copy`, `understanding_is_not_copying`, `fragment_candidate_variation_foreshadowed` hook이 생겼다.
   - `wuxia_mumyeong_reads_orthodox_style` 구현으로 `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `hyeonakmun_trace_suspected`, `bokho_geumsaesu_name_recorded`, `mumyeong_eye_variation_noted`, `orthodox_control_is_violence`, `departure_truth_still_incomplete` hook이 생겼다.
   - `wuxia_mumyeong_midgame_reunion` 구현으로 `mumyeong_midgame_reunion_resolved`, `mumyeong_mirror_thread_deepened`, `seoharin_does_not_call_mumyeong_traitor`, `boss_used_mumyeongs_wound`, `mumyeong_truth_still_incomplete`, `rival_mirror_relationship_deepened`, `hyeonakmun_trace_shared_without_accusation` hook이 생겼다.
   - `wuxia_boss_first_appearance` 구현으로 `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `boss_reads_people_not_forms`, `boss_is_final_logic_wall`, `mumyeong_follows_power_that_saw_his_wound`, `qingliu_cannot_outmuscle_boss_yet` hook이 생겼다.
   - `wuxia_mumyeong_request_for_aid` 구현으로 `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `rejected_aid_letters_read`, `inn_rumor_thread_followed`, `seoharin_failed_aid_question_asked`, `failed_aid_record_kept_unshown`, `mumyeong_tried_to_save_qingliu`, `orthodox_refusal_broke_mumyeong`, `boss_logic_found_mumyeongs_wound`, `aid_refusal_precedes_departure_truth`, `seoharin_does_not_know_failed_aid` hook과 `rejected_aid_letter_fragment` item이 생겼다.
   - `wuxia_mumyeong_awakening` 구현으로 `mumyeong_awakening_resolved`, `mumyeong_awakening_thread_opened`, `copy_corruption_thread_opened`, `mumyeong_copy_bloomed_from_anger`, `copy_is_wound_not_growth`, `protagonist_understands_where_mumyeong_overlays`, `awakening_points_to_hyeonakmun_without_full_truth`, `salvation_truth_still_unready` hook이 생겼다.
   - `wuxia_mumyeong_followup_after_awakening` handoff에서 정한 `wuxia_qingliu_attack_after_war`는 preview runtime 구현 완료다. 구현 범위는 full flashback이 아니라 현악문/복호금쇄수 흔적 조사였다.
   - required flags는 `mumyeong_awakening_resolved`, `mumyeong_awakening_thread_opened`, `copy_corruption_thread_opened`, `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `mumyeong_reads_orthodox_style_resolved`, `orthodox_style_trace_recorded`, `midgame_continuity_started`다.
   - stable choice id는 `inspect_bokho_lock_scars`, `compare_hyeonakmun_trace_to_qingliu_wounds`, `ask_seo_harin_what_she_saw_afterward`, `stop_before_replaying_the_attack`다.
   - `wuxia_qingliu_attack_after_war_followup` handoff에서 정한 `wuxia_mumyeong_destroys_orthodox_sect`는 preview runtime 구현 완료다. 구현 범위는 현악문 멸문 전투가 아니라 빈 현악문 산문/기록/풍문을 확인하는 trace encounter였다.
   - required flags는 `qingliu_attack_after_war_resolved`, `qingliu_attack_trace_confirmed`, `hyeonakmun_attack_thread_opened`, `mumyeong_awakening_resolved`, `midgame_continuity_started`다.
   - stable choice id는 `read_hyeonakmun_empty_gate_record`, `trace_bokho_lock_to_mumyeong`, `ask_why_seoharin_never_heard_full_story`, `stop_before_counting_the_dead`다.
   - common hook은 `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `destination_id: cheongryu_outer_courtyard`다.
   - `wuxia_mumyeong_destroys_orthodox_sect_followup` handoff에서 정한 `wuxia_boss_recruits_mumyeong`는 preview runtime 구현 완료다. 구현 범위는 구원이나 최종 결산이 아니라 흑사방 보스의 recruitment trace였다.
   - required flags는 `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `boss_first_appearance_resolved`, `boss_wall_thread_opened`, `black_serpent_core_pressure_opened`, `midgame_continuity_started`다.
   - stable choice id는 `trace_boss_offer_after_hyeonakmun`, `read_mumyeong_choice_without_excusing_it`, `search_black_serpent_recruitment_record`, `stop_before_following_him_into_black_serpent`다.
   - common hook은 `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `destination_id: cheongryu_outer_courtyard`다.
   - `wuxia_boss_recruits_mumyeong_followup` handoff에서 정한 `wuxia_mumyeong_departure_truth_summary`는 preview runtime 구현 완료다.
   - required flags는 `boss_recruits_mumyeong_resolved`, `boss_recruitment_thread_opened`, `mumyeong_destroys_orthodox_sect_resolved`, `hyeonakmun_destruction_thread_opened`, `departure_truth_thread_deepened`, `mumyeong_request_for_aid_resolved`, `mumyeong_failed_aid_thread_opened`, `orthodox_hypocrisy_thread_opened`, `mumyeong_awakening_resolved`, `midgame_continuity_started`다.
   - stable choice id는 `assemble_departure_truth_without_delivering`, `compare_failed_aid_to_recruitment_offer`, `ask_seoharin_what_she_is_ready_to_hear`, `seal_truth_until_mumyeong_faces_it`다.
   - common hook은 `mumyeong_departure_truth_summary_resolved`, `sealed_departure_truth_summary_prepared`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard`다.
   - presentation은 `visual_id: wuxia_mumyeong_departure_truth_summary`, `speaker: 천기록`, `layout: sealed_departure_truth_summary`, stable terms `[무명, 서하린, 현악문, 흑사방주]`다.
   - `wuxia_mumyeong_departure_truth_summary_followup` handoff는 다음 runtime 후보를 `wuxia_seoharin_empty_place`로 결정했다.
   - required flags는 `mumyeong_departure_truth_summary_resolved`, `sealed_departure_truth_summary_prepared`, `truth_delivery_still_unopened`, `midgame_continuity_started`다.
   - stable choice id는 `ask_who_kept_the_empty_place`, `leave_the_place_unclaimed`, `set_down_the_work_notebook_briefly`, `step_back_without_naming_mumyeong`다.
   - common hook은 `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard`다.
   - presentation은 `visual_id: wuxia_seoharin_empty_place`, `speaker: 서하린`, `layout: empty_place_memory`, stable terms `[서하린, 무명, 청류문, 목검]`다.
   - generated artifacts는 `crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json`와 `web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json`에 반영했다.
   - `wuxia_seoharin_empty_place_followup` handoff는 다음 runtime 후보를 `wuxia_seoharin_left_meal`로 결정했고, 해당 slice는 preview runtime 구현 완료다.
   - required flags는 `seoharin_empty_place_resolved`, `seoharin_axis_opened`, `empty_place_remembered`, `truth_delivery_still_unopened`, `midgame_continuity_started`다.
   - stable choice id는 `eat_the_left_meal_quietly`, `thank_seoharin_for_the_bowl`, `joke_about_who_ordered_extra_rice`, `pass_without_eating_the_meal`다.
   - common hook은 `seoharin_left_meal_resolved`, `truth_delivery_still_unopened`, `destination_id: cheongryu_outer_courtyard`이며, 긍정 선택은 `seoharin_axis_deepened`/`qingliu_belonging_warmed`, 거절 선택은 `seoharin_axis_still_open`/`left_meal_left_untouched`를 남긴다.
   - `wuxia_sado_final_battle`, `wuxia_sado_final_phase_1_price_tag`, `wuxia_sado_final_phase_2_weakpoint_control`, `wuxia_sado_final_phase_3_outside_calculation`, `wuxia_boss_resolution`, `wuxia_mumyeong_resolution`, `wuxia_seoharin_qingliu_resolution`, `wuxia_seoharin_unsaid_stay`, `wuxia_cheongirok_resolution`, `wuxia_black_serpent_aftermath`, `wuxia_final_epilogue_renderer_contract`는 preview/main runtime 구현 완료다. Sado final battle container, ledger/evidence/pressure/item-log seed, weakpoint/final-method seed, outside-calculation result candidate seed, boss-resolution route seed, Mumyeong-resolution route seed, Seo Harin/Qingliu epilogue candidate seed, Seo Harin return/settlement trigger seed, Cheonggi Record last-page seed, Black Serpent aftermath seed, final epilogue card output을 Rust GameCore-owned path로 연결했다. full return/settlement, playable defeat route, combat resolver, 남은 final/late companion 후보는 계속 보류한다.
   - seed 기반 random copy-style system/table, 천외편린 3택 reward/ability schema, boss combat/final resolution, 서하린에게 진실 전달, 무명 구원 확정, `told_seoharin_truth`, 무명/보스 결산, epilogue/return system은 바로 열지 않는다.
   - legacy office `content.bundle.json`, `src/tui_adv/data/*.yaml`, `escape-office` save/localStorage key는 바꾸지 않는다.
   - Rust GameCore / `ScenePage` / WASM JSON boundary가 가진 gameplay truth를 renderer가 재계산하지 않는다.
   - route graph/faction reputation/debt ledger/relation schema, return system, 천기록 정체 reveal, 천외편린 3택 성장/reward/ability schema는 아직 열지 않고, 필요한 경우 `flags`/`clues`/`log`/`presentation` hook으로만 future work를 남긴다.
3. 야근몽 runtime 후보는 `yageunmong_late_night_desk_awake` 또는 각성편린 3택 preview로만 열고, 기본 office bundle을 자동 rewrite하지 않는다.
4. 실제 음악/SFX asset과 soundtrack은 저작권/라이선스 정책이 정리되기 전까지 열지 않는다.


---

## §0.88 Rust core 정본화 — gameplay 로직 단일화 (2026-06-15)

**목표**: 3개 병렬 game impl(Rust/TS/Python)을 Rust 단일 정본으로 통합.

**사용자 결정**:
- Python → 툴 전용 유지 (game/*.py 제거, scripts 유지)
- Web TS fallback → 완전 제거 (wasm-only)
- 외부 사용자 없음

**Phase B 완료** (branch: rust-core-consolidation):
- `web/src/game/` 12 TS files 삭제 (actions, state, types, save, parity.test 등)
- `web/src/main.ts` WASM-only 경로로 단순화 — `REQUIRE_WASM` 플래그 제거, TS fallback 분기 완전 제거
- `scenePageFromTurn.ts`, `render.ts`, 관련 테스트 삭제
- vitest 36 pass

**Phase C 완료**:
- `src/tui_adv/game/` 11 py files 전체 삭제
- `src/tui_adv/tui/app.py`, `encounter.py`, `status.py` 삭제
- `src/tui_adv/main.py` deprecation stub으로 교체
- game logic 의존 pytest 16개 삭제
- pytest 94 pass (3 fail = 기존 Windows 환경 문제)

**commit**: 5decc4d — PR 대기 중 (rust-core-consolidation → main)

**다음**: 사용자 지시 대기.

## §0.89 후일담 본문 + 문서/방법론 최신화 + §0.88 후속 정리 (2026-06-15)

이구학지 in-scope 완성 후 사용자 지시로 진행한 문서/정리 슬라이스. branch: rust-core-consolidation, PR #119.

**후일담 카드 DB 18 본문**: `final_epilogue.rs` `build_candidates()`의 placeholder body를 Notion 후일담 카드 DB(`wuxia_10_epilogue_card_db`) 서사 텍스트로 교체. wasm 런타임 출력 경로(JSON 번들 아님)이므로 wasm 재빌드로 반영 확인. `route_parity` alliance_silence assertion을 새 본문에 맞게 갱신. commit fb2111b.

**방법론/트러블슈팅 문서 신규**:
- `docs/dev/Development_Methodology.md` — 재사용 가능한 plan→implement→verify→report 루프, 완성 판정(canonical 추적 문서 기준), 검증 신뢰 원칙(subagent 보고 직접 재검증), 산출물 경로 모델(wasm vs bundle), 충돌 해결.
- `docs/dev/Troubleshooting.md` — 도구 위치(cargo/gh = WSL only, pytest = .venv), subagent 환각 사례, epilogue body=wasm 경로, 테스트 스냅샷 동기화, 알려진 pytest 환경 실패 3건.

**§0.88 후속 정리**: §0.88이 Python game 로직을 지웠으나 죽은 QA 스크립트가 남아 있었다. `scripts/qa_smoke.py`, `scripts/textual_qa_smoke.py` 삭제. README/AGENTS/00_Index, `Balance_QA_Packaging.md`, `Data_Schema.md`, `Final_QA_Log.md`, `Save_Slot_UX.md`, `Checklist.md`의 죽은 명령 참조를 post-§0.88 현실(cargo test / pytest / npm test)로 동기화. `test_docs_contract.py`, `test_web_wasm_build_standardization.py`의 stale assertion 갱신.

**검증**: pytest 94 pass / 3 fail(known env), cargo test --workspace PASS, npm test PASS.

**다음**: 사용자 지시 대기 (B묶음/야근몽/기타 팩은 보류 유지).

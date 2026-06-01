from pathlib import Path


def test_checklist_tracks_completed_terminal_ux_slices():
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")

    assert "- [x] 위험도 변화 규칙 적용" in checklist
    assert "- [x] 선택 불가 선택지 이유 표시" in checklist


def test_superlighttui_terminal_polish_is_checklisted_and_documented():
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    readme = Path("README.md").read_text(encoding="utf-8")
    architecture = Path("docs/dev/Rust_Core_Dual_Renderer_Architecture.md").read_text(
        encoding="utf-8"
    )

    assert "- [x] terminal visual card가 visual_id/layout/alt를 ASCII/Unicode card로 표시" in checklist
    assert "- [x] GlyphFX fallback이 intensity meter, stable terms, fallback text를 보존" in checklist
    assert "- [x] 직접 플레이 입력 안내가 현재 턴 번호 범위와 action id 사용법을 표시" in checklist
    assert "SuperLightTUI terminal visual card/GlyphFX/input polish 완료" in plan
    assert "Web/Tauri/Electron 패키징 결정 완료" in plan
    assert "terminal full-screen app loop/tick/raw-draw GlyphFX 완료" in plan
    next_slice = plan.split("현재 최우선 남은 작업:", 1)[1]
    assert "모바일 픽셀 스토리북 UI redesign 완료" in plan
    assert "terminal full-screen app loop와 tick/raw-draw GlyphFX slice를 진행한다" not in next_slice
    assert "Web/Tauri/Electron 패키징 검토" not in next_slice
    assert "visual card/GlyphFX/input 안내 polish" in readme
    assert "--app-smoke --tick" in readme
    assert "full-screen SuperLightTUI app loop" in readme
    assert "printer_anomaly stable terms를 terminal visual card 안에 보존" in architecture
    assert "tick/raw-draw capability를 쓰는 app smoke와 full-screen loop" in architecture


def test_terminal_app_loop_slice_is_checklisted_and_documented():
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    layout_doc = Path("docs/dev/TUI_Layout.md").read_text(encoding="utf-8")

    assert "- [x] `escape-terminal --app` full-screen SuperLightTUI app loop 추가" in checklist
    assert "- [x] `--app-smoke --tick` headless app-frame smoke 추가" in checklist
    assert "- [x] raw-draw GlyphFX layer가 tick 변화와 stable terms/fallback text를 함께 검증" in checklist
    assert "- [x] inline image는 baseline 밖 optional future로 deferred 결정" in checklist
    assert "optional inline image는 terminal cell/GlyphFX baseline 밖 future backlog" in plan
    assert "`--app`" in layout_doc
    assert "`--app-smoke --tick`" in layout_doc


def test_readme_next_work_points_to_canonical_main_plan_instead_of_completed_slice_list():
    readme = Path("README.md").read_text(encoding="utf-8")
    next_work = readme.split("## 다음 작업 기준", 1)[1]

    assert "선택 불가 선택지의 이유 표시" not in next_work
    assert "색상 테마" not in next_work
    assert "밸런싱" not in next_work
    assert "docs/dev/Development_Plan.md" in next_work
    assert "source of truth" in next_work
    assert "긴 next-task 목록을 복제하지 않는다" in next_work
    assert "## 다음 작업 후보" not in readme


def test_notion_first_idea_design_workflow_is_documented():
    agents = Path("AGENTS.md").read_text(encoding="utf-8")
    readme = Path("README.md").read_text(encoding="utf-8")
    idea_readme = Path("idea_box/README.md").read_text(encoding="utf-8")
    backlog_order = Path("idea_box/BACKLOG_ORDER.md").read_text(encoding="utf-8")
    intake = Path("idea_box/IDEA_INTAKE_GUIDE.md").read_text(encoding="utf-8")
    handoff = Path("idea_box/LLM_DESIGN_HANDOFF.md").read_text(encoding="utf-8")
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")

    for doc in (agents, readme, idea_readme, backlog_order, intake, handoff, plan):
        assert "Notion" in doc
        assert "원본 reference" in doc

    assert "Notion 정리 → repo 설계 아이디어 문서 변환" in readme
    assert "설계 아이디어 문서 변환 → main plan 격상 → 설계 후 Notion reference 대조" in checklist
    assert "main plan" in idea_readme
    assert "reference_check" in idea_readme
    assert "notion_page_id" in intake
    assert "Notion reference 대조" in handoff
    assert "단순 import나 설계 아이디어 문서 작성만으로는 `done`" in plan


def test_phase6_textual_style_slice_is_checklisted_and_documented():
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    layout_doc = Path("docs/dev/TUI_Layout.md").read_text(encoding="utf-8")

    assert "- [x] 분리된 Textual 위젯/그리드 패널 스타일링" in checklist
    assert "- [x] Textual CSS 색상 테마 위젯 연결" in checklist
    assert "#game-grid" in layout_doc
    assert "office-panel--wide" in layout_doc


def test_mobile_pixel_storybook_ui_doc_is_indexed_checklisted_and_current_plan():
    index = Path("docs/00_Index.md").read_text(encoding="utf-8")
    readme = Path("README.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    doc_path = Path("docs/design/Mobile_Pixel_Storybook_UI.md")

    assert doc_path.exists()
    doc = doc_path.read_text(encoding="utf-8")
    main_ts = Path("web/src/main.ts").read_text(encoding="utf-8")
    package_json = Path("web/package.json").read_text(encoding="utf-8")
    assert "docs/design/Mobile_Pixel_Storybook_UI.md" in index
    assert "docs/design/Mobile_Pixel_Storybook_UI.md" in readme
    assert "모바일 세로형 픽셀 게임북 board" in doc
    assert "idea_box/플레이화면0.bmp" in doc
    assert "story-progress-rail" in doc
    assert "choice-row" in doc
    assert "@fontsource/noto-serif-kr" in doc
    assert "@fontsource/noto-serif-kr" in main_ts
    assert "@fontsource/noto-serif-kr" in package_json
    assert "Renderer는 status를 계산하지 않는다" in doc
    assert "- [x] Web Storybook 모바일 픽셀 board contract 문서화" in checklist
    assert "- [x] HUD/rail/dock/sentence-choice renderer contract 구현" in checklist
    assert "- [x] reference-size browser visual QA" in checklist
    assert "모바일 픽셀 스토리북 UI redesign 완료" in plan
    assert "Web Storybook visual regression 자동화 완료" in plan
    assert "Web player deployment readiness 완료" in plan
    assert "Web player start/save UX first slice 완료" in plan
    completed_foundation = plan.split("현재 완료된 기반:", 1)[1].split("현재 최우선 남은 작업:", 1)[0]
    assert "web/scripts/storybook-reference-qa.mjs" in completed_foundation
    assert "qa:storybook:visual" in completed_foundation
    assert "VITE_BASE_PATH" in completed_foundation
    assert "VITE_REQUIRE_WASM" in completed_foundation
    assert "PR C audio engine skeleton 완료" in completed_foundation
    assert "lazy Web Audio generated oscillator backend" in completed_foundation
    assert "schema-less combat encounter prototype 완료" in completed_foundation
    next_slice = plan.split("현재 최우선 남은 작업:", 1)[1]
    assert "무협 storypack preview" in next_slice
    assert "wuxia_jianghu_pack" in next_slice
    assert "Rust GameCore / `ScenePage` / WASM JSON boundary" in next_slice


def test_transition_audio_readiness_is_current_active_plan():
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    web_plan = Path("docs/dev/Web_Player_PokeRogue_Style_Plan.md").read_text(encoding="utf-8")

    assert "Web Storybook transition/audio readiness" in plan
    assert "escape-office.player-settings.v1" in plan
    assert "motion/audio preference" in plan
    assert "transition plan type" in plan
    assert "audio default는 muted/off" in plan
    assert "PR B transition controller 완료" in plan
    assert "PR C audio engine skeleton 완료" in plan
    assert "Web Audio skeleton은 binary asset 없이 generated oscillator/no-op backend" in plan

    next_slice = plan.split("현재 최우선 남은 작업:", 1)[1].split("전환 중 유지:", 1)[0]
    assert "현재 active main plan 기준 즉시 진행할 구현 작업은 없다" not in next_slice
    assert "무협 storypack preview" in next_slice
    assert "wuxia_jianghu_pack" in next_slice
    assert "wuxia_heuksa_bang_first_fight" in next_slice
    assert "preview launcher/UI wiring" in next_slice
    assert "wuxia_cheonggi_record_first_fragment" in next_slice
    assert "wuxia_seo_harin_rescue" in next_slice
    assert "wuxia_cheongryu_apprentice_entry" in next_slice
    assert "wuxia_cheongryu_chore_sparring" in next_slice
    assert "wuxia_cheongryu_raid_route_split" in next_slice
    assert "wuxia_cheongryu_raid_wounded_fallback" in next_slice
    assert "renderer-neutral" in next_slice
    assert "`wuxia_heuksa_bang_first_fight`를 구현한다" not in next_slice

    next_actions = plan.split("## 10. 다음 액션", 1)[1]
    assert "Storypack_Runtime_Preview_Mode.md" in plan
    assert "wuxia_cheonggi_record_first_fragment" in next_actions
    assert "wuxia_seo_harin_rescue" in next_actions
    assert "wuxia_cheongryu_apprentice_entry" in next_actions
    assert "wuxia_cheongryu_chore_sparring" in next_actions
    assert "wuxia_cheongryu_raid_route_split" in next_actions
    assert "wuxia_cheongryu_raid_wounded_fallback" in next_actions
    assert "preview launcher/UI wiring`도 완료" in next_actions
    assert "preview mode" in next_actions or "storypack_preview" in next_actions
    assert "`wuxia_heuksa_bang_first_fight`를 같은 `wuxia_jianghu_pack` storypack preview mode에 추가한다" not in next_actions

    assert "### 0.2m 2026-05-26 Web Storybook transition/audio readiness" in checklist
    assert "- [x] transition/audio readiness active main plan 승격" in checklist
    assert "- [x] player settings localStorage contract 구현" in checklist
    assert "- [x] transition plan type과 reduced-motion no-op 구조 추가" in checklist
    assert "- [x] audio muted default policy와 opt-in skeleton 추가" in checklist
    assert "- [x] transition controller 적용" in checklist
    assert "- [x] Web Audio API lazy/no-op engine 추가" in checklist
    assert "- [x] muted no schedule + user-gesture opt-in one-shot cue API 구현" in checklist
    assert "- [x] looping ambience API와 binary asset 없는 generated oscillator backend 구현" in checklist
    assert "- [x] visual QA motion/audio 안정화 확인" in checklist
    assert "### 0.2o 2026-05-29 schema-less combat encounter prototype runtime" in checklist
    assert "- [x] 기존 encounter/choice/outcome schema만 사용한 물품창고 자동 난투 구현" in checklist
    assert "- [x] Rust `ScenePage` / SuperLightTUI / Web generated data parity 검증 추가" in checklist

    assert "Transition/audio readiness first slice" in web_plan
    assert "PR A settings/motion foundation implemented" in web_plan
    assert "PR B transition controller implemented" in web_plan
    assert "PR C audio engine skeleton implemented" in web_plan
    assert "binary-asset-free policy" in web_plan


def test_phase9_story_route_design_docs_are_indexed_and_checklisted():
    index = Path("docs/00_Index.md").read_text(encoding="utf-8")
    readme = Path("README.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    disaster_doc = Path("docs/story/Disaster_Cause.md")
    route_doc = Path("docs/content/Survivor_System_Routes.md")

    assert disaster_doc.exists()
    assert route_doc.exists()
    assert "docs/story/Disaster_Cause.md" in index
    assert "docs/content/Survivor_System_Routes.md" in index
    assert "docs/story/Disaster_Cause.md" in readme
    assert "docs/content/Survivor_System_Routes.md" in readme
    assert "- [x] 생존자 또는 시스템 제압/설득 설계" in checklist
    assert "- [x] 재난 원인 문서 설계" in checklist


def test_phase9_story_route_design_docs_define_public_safe_runtime_hooks():
    disaster_doc = Path("docs/story/Disaster_Cause.md")
    route_doc = Path("docs/content/Survivor_System_Routes.md")

    disaster_text = disaster_doc.read_text(encoding="utf-8")
    route_text = route_doc.read_text(encoding="utf-8")

    assert "격리 프로토콜" in disaster_text
    assert "공개 저장소 금지 정보" in disaster_text
    assert "isolation_protocol_revealed" in disaster_text
    assert "truth_isolation_protocol" in disaster_text
    assert "생존자 설득 루트" in route_text
    assert "시스템 제압 루트" in route_text
    assert "server_room_broadcast_controlled" in route_text
    assert "public-safe" in route_text


def test_phase10_balance_qa_packaging_doc_script_and_checklist_are_synced():
    index = Path("docs/00_Index.md").read_text(encoding="utf-8")
    readme = Path("README.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    qa_doc = Path("docs/dev/Balance_QA_Packaging.md")
    qa_script = Path("scripts/qa_smoke.py")

    assert qa_doc.exists()
    assert qa_script.exists()
    assert "docs/dev/Balance_QA_Packaging.md" in index
    assert "docs/dev/Balance_QA_Packaging.md" in readme
    assert "scripts/qa_smoke.py" in qa_doc.read_text(encoding="utf-8")
    assert "PYTHONPATH=src python scripts/qa_smoke.py" in readme
    assert "- [x] 턴당 허기 증가량 조정" in checklist
    assert "- [x] 턴당 갈증 증가량 조정" in checklist
    assert "- [x] 엔딩 도달 난이도 조정" in checklist
    assert "- [x] 탈출 엔딩 도달 테스트" in checklist
    assert "- [x] 실패 엔딩 도달 테스트" in checklist
    assert "- [x] 히든 힌트 도달 테스트" in checklist
    assert "- [x] 저장/로드 반복 테스트" in checklist
    assert "- [x] 키 입력 오류 처리 확인" in checklist
    assert "- [x] 릴리즈 전 비밀 정보 검사" in checklist


def test_final_qa_leftover_checks_are_documented_and_completed():
    index = Path("docs/00_Index.md").read_text(encoding="utf-8")
    readme = Path("README.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    qa_doc = Path("docs/dev/Final_QA_Log.md")
    textual_script = Path("scripts/textual_qa_smoke.py")

    assert qa_doc.exists()
    assert textual_script.exists()
    assert "docs/dev/Final_QA_Log.md" in index
    assert "docs/dev/Final_QA_Log.md" in readme
    assert "PYTHONPATH=src python scripts/textual_qa_smoke.py" in qa_doc.read_text(encoding="utf-8")
    assert "- [x] 실제 Textual 화면 수동 QA" in checklist
    assert "- [x] 새 게임 10회 수동 플레이 기록" in checklist
    assert "- [x] 터미널 크기별 화면 확인" in checklist
    legacy_final_qa_section = checklist.split("### 0.2h 2026-05-23 Web Storybook 모바일 픽셀 board redesign", 1)[0]
    assert "- [ ]" not in legacy_final_qa_section
    assert "- [x] `web/scripts/storybook-reference-qa.mjs` Playwright viewport runner 추가" in checklist


def test_readme_next_work_keeps_role_split_instead_of_final_qa_queue():
    readme = Path("README.md").read_text(encoding="utf-8")
    next_work = readme.split("## 다음 작업 기준", 1)[1]

    assert "시작 화면 저장 슬롯 UX" not in next_work
    assert "밸런싱/QA/패키징" not in next_work
    assert "재난 타입 확장" not in next_work
    assert "저장 슬롯 이름 변경" not in next_work
    assert "docs/dev/Checklist.md" in next_work
    assert "구현 계약 참조" in next_work
    assert ".hermes/plans/" in next_work


def test_disaster_type_extension_doc_is_indexed_and_runtime_bound():
    index = Path("docs/00_Index.md").read_text(encoding="utf-8")
    readme = Path("README.md").read_text(encoding="utf-8")
    doc_path = Path("docs/story/Disaster_Type_Extension.md")

    assert doc_path.exists()
    doc = doc_path.read_text(encoding="utf-8")
    assert "docs/story/Disaster_Type_Extension.md" in index
    assert "docs/story/Disaster_Type_Extension.md" in readme
    assert "unknown_isolation" in doc
    assert "conditions.disaster_types" in doc
    assert "새 타입 후보 백로그" in doc
    assert "시작 UI 타입 선택 | 아직 없음" in doc


def test_save_slot_rename_ux_doc_is_indexed_and_linked_from_layout():
    index = Path("docs/00_Index.md").read_text(encoding="utf-8")
    readme = Path("README.md").read_text(encoding="utf-8")
    layout_doc = Path("docs/dev/TUI_Layout.md").read_text(encoding="utf-8")
    doc_path = Path("docs/dev/Save_Slot_UX.md")

    assert doc_path.exists()
    doc = doc_path.read_text(encoding="utf-8")
    assert "docs/dev/Save_Slot_UX.md" in index
    assert "docs/dev/Save_Slot_UX.md" in readme
    assert "docs/dev/Save_Slot_UX.md" in layout_doc
    assert "slot_name" in doc
    assert "schema_version" in doc
    assert '"state"' in doc
    assert "`state` 바깥의 metadata" in doc
    assert "r: 이름 변경 모드" in doc
    assert "파일명 자체를 바꾸기보다" in doc

def test_combat_system_auto_brawl_doc_is_indexed_and_backlog_done():
    index = Path("docs/00_Index.md").read_text(encoding="utf-8")
    readme = Path("README.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    idea = Path("idea_box/combat_system.md").read_text(encoding="utf-8")
    doc_path = Path("docs/design/Combat_System_Auto_Brawl.md")
    basic_doc_path = Path("docs/design/Basic_Combat_Action_Model.md")

    assert doc_path.exists()
    assert basic_doc_path.exists()
    doc = doc_path.read_text(encoding="utf-8")
    basic_doc = basic_doc_path.read_text(encoding="utf-8")
    assert "docs/design/Combat_System_Auto_Brawl.md" in index
    assert "docs/design/Basic_Combat_Action_Model.md" in index
    assert "docs/design/Combat_System_Auto_Brawl.md" in readme
    assert "docs/design/Basic_Combat_Action_Model.md" in readme
    assert "전투 시스템 아이디어 문서화" in checklist
    assert "schema-less combat encounter prototype runtime" in checklist
    assert "docs/design/Combat_System_Auto_Brawl.md" in plan
    assert "docs/design/Basic_Combat_Action_Model.md" in plan
    assert "`supply_closet_auto_brawl`" in plan
    assert "`wuxia_cheongryu_chore_sparring`" in plan
    assert "자동 난투 + 상황 개입" in doc
    assert "Rust GameCore" in doc
    assert "ScenePage" in doc
    assert "전투당 개입 요구는 0~3회" in doc
    assert "schema-less combat encounter prototype" in doc
    assert "`supply_closet_auto_brawl`" in doc
    assert "wuxia_cheongryu_chore_sparring" in basic_doc
    assert "office 대응 전투 후보 1개 설계/구현" in basic_doc
    assert "CombatState" in basic_doc
    assert "status: done" in idea
    assert "used_by: docs/design/Combat_System_Auto_Brawl.md" in idea
    assert "이번 처리에서는 런타임 YAML/Rust/Web 코드는 변경하지 않았다" in idea
    assert "후속 런타임 slice" in idea
    assert "`supply_closet_auto_brawl`" in idea


def test_storypack_world_model_and_wuxia_pack_are_indexed_and_current():
    index = Path("docs/00_Index.md").read_text(encoding="utf-8")
    readme = Path("README.md").read_text(encoding="utf-8")
    agents = Path("AGENTS.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    storypack_readme = Path("docs/content/storypacks/README.md").read_text(encoding="utf-8")
    storypack_design = Path("docs/design/Storypack_Encounter_DB.md").read_text(encoding="utf-8")
    backlog_order = Path("idea_box/BACKLOG_ORDER.md").read_text(encoding="utf-8")
    wuxia_idea_path = Path("idea_box/done/2026-05-29-notion-wuxia-igu-hakji-cheonggi-record.md")
    world_doc_path = Path("docs/design/Storypack_World_Model.md")
    wuxia_pack_path = Path("docs/content/storypacks/wuxia_jianghu_pack.md")
    wuxia_cards_path = Path("docs/content/encounter_db/wuxia_jianghu_pack.md")
    storypacks_json_path = Path("docs/content/storypack_db/storypacks.json")
    cards_json_path = Path("docs/content/storypack_db/encounter_situations.json")

    assert world_doc_path.exists()
    assert wuxia_pack_path.exists()
    assert wuxia_cards_path.exists()
    assert storypacks_json_path.exists()
    assert cards_json_path.exists()
    assert wuxia_idea_path.exists()

    world_doc = world_doc_path.read_text(encoding="utf-8")
    wuxia_pack = wuxia_pack_path.read_text(encoding="utf-8")
    wuxia_cards = wuxia_cards_path.read_text(encoding="utf-8")
    wuxia_idea = wuxia_idea_path.read_text(encoding="utf-8")

    assert "docs/design/Storypack_World_Model.md" in index
    assert "docs/content/storypacks/wuxia_jianghu_pack.md" in index
    assert "docs/content/encounter_db/wuxia_jianghu_pack.md" in index
    assert "docs/content/storypack_db/storypacks.json" in index
    assert "docs/content/storypack_db/encounter_situations.json" in index
    assert "docs/design/Storypack_World_Model.md" in readme
    assert "docs/content/storypacks/wuxia_jianghu_pack.md" in readme
    assert "docs/content/storypack_db/storypacks.json" in readme

    assert "storypack/world 기반" in agents
    assert "이구학지 — 천기록" in agents
    assert "wuxia_jianghu_pack" in storypack_readme
    assert "이구학지 — 천기록" in storypack_readme
    assert "wuxia_jianghu" in storypack_design
    assert "commute_rift" in storypack_design
    assert "cheonggi_record" in storypack_design
    assert "docs/content/storypack_db/storypacks.json" in storypack_design
    assert "validate_storypack_db" in storypack_design

    assert "회사”는 엔진의 정체성이 아니라 첫 번째 기본 storypack" in world_doc
    assert "world_id" in world_doc
    assert "이구학지 — 천기록" in world_doc
    assert "wuxia_commute_rift_arrival" in world_doc
    assert "wuxia_heuksa_bang_first_fight" in world_doc

    assert "이구학지 — 천기록" in wuxia_pack
    assert "previous_placeholder_status: superseded" in wuxia_pack
    assert "흑사방" in wuxia_pack
    assert "청류문" in wuxia_pack
    assert "천기록" in wuxia_pack
    assert "천외편린" in wuxia_pack
    assert "commute_rift" in wuxia_pack
    assert "fragment_choice" in wuxia_pack
    assert "wuxia_commute_rift_arrival" in wuxia_cards
    assert "wuxia_heuksa_bang_first_fight" in wuxia_cards
    assert "wuxia_cheonggi_record_first_fragment" in wuxia_cards
    assert "wuxia_cheongryu_chore_sparring" in wuxia_cards
    assert "wuxia_office_worker_arrival" not in wuxia_cards
    assert "wuxia_duel_bridge_intervention" not in wuxia_cards

    assert "storypack/world 일반화와 무협 기준팩" in checklist
    assert "이구학지 — 천기록 최신화" in checklist
    assert "machine-readable storypack DB 검증" in checklist
    assert "storypack/world 일반화" in plan
    assert "machine-readable storypack DB 검증 완료" in plan
    assert "무협 storypack preview runtime prototype 완료" in plan
    assert "무협 `wuxia_heuksa_bang_first_fight` preview runtime slice 완료" in plan
    assert "무협 preview launcher/UI wiring 완료" in plan
    assert "무협 `wuxia_cheonggi_record_first_fragment` preview runtime slice 완료" in plan
    assert "무협 `wuxia_cheongryu_chore_sparring` preview runtime slice 완료" in plan
    assert "무협 preview launcher/UI wiring" in checklist
    assert "무협 `wuxia_cheongryu_chore_sparring` preview runtime slice" in checklist
    assert "무협 `wuxia_cheonggi_record_first_fragment` preview runtime slice" in checklist
    assert "첫 비-office 기준팩은 `wuxia_jianghu_pack` / **이구학지 — 천기록**" in plan
    next_slice = plan.split("현재 최우선 남은 작업:", 1)[1].split("전환 중 유지:", 1)[0]
    assert "wuxia_jianghu_pack" in next_slice
    assert "wuxia_heuksa_bang_first_fight" in next_slice
    assert "preview launcher/UI wiring" in next_slice
    assert "wuxia_cheonggi_record_first_fragment" in next_slice
    assert "wuxia_seo_harin_rescue" in next_slice
    assert "wuxia_cheongryu_apprentice_entry" in next_slice
    assert "wuxia_cheongryu_chore_sparring" in next_slice
    assert "wuxia_cheongryu_raid_route_split" in next_slice
    assert "wuxia_cheongryu_raid_wounded_fallback" in next_slice
    assert "`wuxia_heuksa_bang_first_fight`를 구현한다" not in next_slice
    assert "PR C audio engine skeleton을 리뷰/머지" not in next_slice

    assert "status: done" in wuxia_idea
    assert "docs/content/storypacks/wuxia_jianghu_pack.md" in wuxia_idea
    assert "docs/content/encounter_db/wuxia_jianghu_pack.md" in wuxia_idea
    assert "idea_box/done/2026-05-29-notion-wuxia-igu-hakji-cheonggi-record.md" in backlog_order
    assert "2026-05-29-02 | `idea_box/inbox/2026-05-29-notion-wuxia" not in backlog_order


def test_wuxia_runtime_preview_mode_decision_is_documented_before_runtime_content():
    index = Path("docs/00_Index.md").read_text(encoding="utf-8")
    readme = Path("README.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    schema = Path("docs/dev/Data_Schema.md").read_text(encoding="utf-8")
    world_doc = Path("docs/design/Storypack_World_Model.md").read_text(encoding="utf-8")
    decision_path = Path("docs/dev/Storypack_Runtime_Preview_Mode.md")

    assert decision_path.exists()
    decision = decision_path.read_text(encoding="utf-8")

    assert "Decision: separate preview mode first" in decision
    assert "`src/tui_adv/data/*.yaml` remains default office runtime content" in decision
    assert "`wuxia_jianghu_pack` enters runtime only through explicit preview bundle or preview flag" in decision
    assert "`escape-office` save/localStorage keys remain unchanged" in decision
    assert "renderer는 `ScenePage`만 표시" in decision
    assert "no default bundle mixing" in decision

    assert "docs/dev/Storypack_Runtime_Preview_Mode.md" in index
    assert "docs/dev/Storypack_Runtime_Preview_Mode.md" in readme
    assert "Storypack runtime preview mode" in schema
    assert "separate preview mode first" in world_doc
    assert "storypack runtime preview mode 결정" in checklist
    assert "preview mode 결정 완료" in plan
    assert "무협 `wuxia_heuksa_bang_first_fight` preview runtime slice 완료" in plan
    assert "무협 preview launcher/UI wiring 완료" in plan
    assert "무협 `wuxia_cheonggi_record_first_fragment` preview runtime slice 완료" in plan
    assert "--storypack-preview wuxia_jianghu_pack" in decision
    assert "Web start screen preview launcher" in decision

    next_actions = plan.split("## 10. 다음 액션", 1)[1]
    assert "gating 또는 별도 preview mode를 결정" not in next_actions
    assert "wuxia_cheonggi_record_first_fragment" in next_actions
    assert "wuxia_seo_harin_rescue" in next_actions
    assert "wuxia_cheongryu_apprentice_entry" in next_actions
    assert "wuxia_cheongryu_raid_route_split" in next_actions
    assert "wuxia_cheongryu_raid_wounded_fallback" in next_actions
    assert "preview launcher/UI wiring`도 완료" in next_actions
    assert "preview mode" in next_actions or "storypack_preview" in next_actions
    assert "`wuxia_heuksa_bang_first_fight`를 같은 `wuxia_jianghu_pack` storypack preview mode에 추가한다" not in next_actions


def test_wuxia_commute_rift_arrival_preview_runtime_is_documented_and_indexed():
    index = Path("docs/00_Index.md").read_text(encoding="utf-8")
    readme = Path("README.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    schema = Path("docs/dev/Data_Schema.md").read_text(encoding="utf-8")
    decision = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(encoding="utf-8")

    source_dir = Path("src/tui_adv/storypack-previews/wuxia_jianghu_pack")
    rust_preview_bundle = Path(
        "crates/escape-core/fixtures/content/storypack-preview/wuxia_jianghu_pack.content.bundle.json"
    )
    web_preview_bundle = Path(
        "web/src/data/generated/storypack-preview/wuxia_jianghu_pack.content.bundle.json"
    )

    assert source_dir.exists()
    assert (source_dir / "encounters.yaml").exists()
    assert rust_preview_bundle.exists()
    assert web_preview_bundle.exists()

    for text in (index, readme, schema, decision, plan):
        assert "src/tui_adv/storypack-previews/wuxia_jianghu_pack" in text
        assert "wuxia_jianghu_pack.content.bundle.json" in text

    for text in (decision, plan):
        assert "wuxia_commute_rift_arrival" in text

    for text in (schema, decision, plan):
        assert "default_location: wuxia_commute_rift" in text

    assert "무협 storypack preview runtime prototype" in checklist
    assert "Python exporter, Rust content metadata, WASM JSON boundary, SuperLightTUI smoke 테스트 추가" in checklist
    assert "무협 `wuxia_heuksa_bang_first_fight` preview runtime slice" in checklist
    assert "무협 preview launcher/UI wiring" in checklist
    assert "무협 storypack preview runtime prototype 완료" in plan
    assert "무협 `wuxia_heuksa_bang_first_fight` preview runtime slice 완료" in plan
    assert "무협 preview launcher/UI wiring 완료" in plan
    assert "무협 `wuxia_cheonggi_record_first_fragment` preview runtime slice 완료" in plan
    assert "기본 office bundle은 계속 `dev_desk`에서 시작" in plan
    assert "jianghu_market_street" in decision
    assert "wuxia_cheonggi_record_first_fragment" in decision
    assert "wuxia_cheongryu_chore_sparring" in decision
    assert "content_tui_smoke_renders_wuxia_storypack_preview_first_fight" in decision
    assert "content_tui_smoke_launches_wuxia_storypack_preview_by_opt_in_flag" in decision
    assert "content_tui_smoke_reaches_wuxia_cheonggi_record_first_fragment" in decision


def test_wuxia_cheongryu_raid_wounded_fallback_runtime_slice_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    decision = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(encoding="utf-8")
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(encoding="utf-8")

    assert "무협 `wuxia_cheongryu_raid_wounded_fallback` preview runtime slice 완료" in plan
    assert "### 0.2ah 2026-06-01 무협 `wuxia_cheongryu_raid_wounded_fallback` preview runtime slice" in checklist
    assert "wuxia_cheongryu_raid_wounded_fallback` — 구현 완료" in decision
    assert "implemented_in_storypack_preview" in wuxia_cards
    assert "`wuxia_cheongryu_raid_wounded_fallback` — preview runtime 구현 완료" in wuxia_pack
    assert "route opener docs-only handoff" in plan
    assert "wuxia_baekdo_medicine_debt" in plan
    assert "wuxia_baekdo_medicine_debt" in next_goal
    assert "current_goal: implement_wuxia_baekdo_medicine_debt" in next_goal
    assert "Route opener docs-only handoff" in coverage
    next_slice = plan.split("현재 최우선 남은 작업:", 1)[1].split("전환 중 유지:", 1)[0]
    assert "wuxia_baekdo_medicine_debt" in next_slice
    assert "righteous_route_started" in next_slice
    assert "cheongryu_rebuild_thread" in next_slice
    assert "기본 `content.bundle.json`, Web 기본 generated bundle, `src/tui_adv/data/*.yaml`, `escape-office` save/localStorage key는 계속 바꾸지 않는다" in next_slice


def test_wuxia_baekdo_medicine_debt_route_opener_handoff_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    decision = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(encoding="utf-8")
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(encoding="utf-8")
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(encoding="utf-8")

    assert "## 0.20 2026-06-01 docs-only route opener handoff: `wuxia_baekdo_medicine_debt`" in plan
    assert "### 0.2ai 2026-06-01 무협 route opener docs-only handoff" in checklist
    assert "route opener docs-only handoff 결과: 첫 route opener는 정파/백도맹 약상자 채무 축인 `wuxia_baekdo_medicine_debt`" in decision
    assert "| `wuxia_baekdo_medicine_debt` | `route_commitment`" in wuxia_pack
    assert "## 9. `wuxia_baekdo_medicine_debt`" in wuxia_cards
    assert "runtime_preview_design_status: designed_next_not_implemented" in wuxia_cards
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 9개." in storypack_db_readme
    assert "current_goal: implement_wuxia_baekdo_medicine_debt" in next_goal
    assert "runtime-preview-implementation" in next_goal
    assert "required_flags: [righteous_route_started, cheongryu_rebuild_thread]" in next_goal
    assert "runtime YAML/Rust/Web/generated artifact" in next_goal

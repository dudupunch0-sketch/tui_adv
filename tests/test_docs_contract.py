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
    assert "Web의 별도 preview launcher는 이구학지가 기본이 되면서 목록에서 비워 두었다" in next_actions
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

    assert "회사”는 엔진의 정체성이 아니라 첫 번째 legacy storypack" in world_doc
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
    assert "`src/tui_adv/data/*.yaml` remains legacy office runtime content" in decision
    assert "`wuxia_jianghu_pack` is the Web/default storypack" in decision
    assert "legacy `escape-office` save/localStorage keys remain unchanged" in decision
    assert "renderer는 `ScenePage`만 표시" in decision
    assert "no legacy bundle mixing" in decision

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
    assert "Web 새 게임도 이구학지 default bundle" in decision

    next_actions = plan.split("## 10. 다음 액션", 1)[1]
    assert "gating 또는 별도 preview mode를 결정" not in next_actions
    assert "wuxia_cheonggi_record_first_fragment" in next_actions
    assert "wuxia_seo_harin_rescue" in next_actions
    assert "wuxia_cheongryu_apprentice_entry" in next_actions
    assert "wuxia_cheongryu_raid_route_split" in next_actions
    assert "wuxia_cheongryu_raid_wounded_fallback" in next_actions
    assert "Web의 별도 preview launcher는 이구학지가 기본이 되면서 목록에서 비워 두었다" in next_actions
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
    assert "Route opener implementation" in coverage
    next_slice = plan.split("현재 최우선 남은 작업:", 1)[1].split("전환 중 유지:", 1)[0]
    assert "wuxia_baekdo_medicine_debt" in next_slice
    assert "wuxia_black_heaven_escape_price" in next_slice
    assert "wuxia_heavenly_archive_previous_outsiders" in next_slice
    assert "wuxia_wounded_shelter_dawn_offers" in next_slice
    assert "wuxia_mumyeong_first_sighting" in next_slice
    assert "legacy office `content.bundle.json`, `src/tui_adv/data/*.yaml`, `escape-office` save/localStorage key는 계속 바꾸지 않는다" in next_slice


def test_wuxia_baekdo_medicine_debt_runtime_slice_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    decision = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(encoding="utf-8")
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(encoding="utf-8")
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(encoding="utf-8")

    assert "## 0.20 2026-06-01 docs-only route opener handoff: `wuxia_baekdo_medicine_debt`" in plan
    assert "## 0.21 2026-06-01 무협 `wuxia_baekdo_medicine_debt` preview runtime slice" in plan
    assert "### 0.2ai 2026-06-01 무협 route opener docs-only handoff" in checklist
    assert "### 0.2aj 2026-06-01 무협 `wuxia_baekdo_medicine_debt` preview runtime slice" in checklist
    assert "`wuxia_baekdo_medicine_debt` — preview runtime 구현 완료" in decision
    assert "| `wuxia_baekdo_medicine_debt` | `route_commitment`" in wuxia_pack
    assert "`wuxia_baekdo_medicine_debt` — preview runtime 구현 완료" in wuxia_pack
    assert "## 9. `wuxia_baekdo_medicine_debt`" in wuxia_cards
    assert "runtime_preview_design_status: implemented" in wuxia_cards
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "wuxia_black_heaven_escape_price" in storypack_db_readme


def test_wuxia_black_heaven_escape_price_runtime_slice_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    decision = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(encoding="utf-8")
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(encoding="utf-8")
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(encoding="utf-8")

    assert "## 0.22 2026-06-01 docs-only route opener follow-up handoff: `wuxia_black_heaven_escape_price`" in plan
    assert "## 0.23 2026-06-02 무협 `wuxia_black_heaven_escape_price` preview runtime slice" in plan
    assert "### 0.2ak 2026-06-01 무협 route opener follow-up docs-only handoff" in checklist
    assert "### 0.2al 2026-06-02 무협 `wuxia_black_heaven_escape_price` preview runtime slice" in checklist
    assert "`wuxia_black_heaven_escape_price` — preview runtime 구현 완료" in decision
    assert "Route opener follow-up implementation" in coverage
    assert "wuxia_black_heaven_escape_price" in coverage
    assert "| `wuxia_black_heaven_escape_price` | `route_commitment`" in wuxia_pack
    assert "## 10. `wuxia_black_heaven_escape_price`" in wuxia_cards
    assert "runtime_preview_design_status: implemented" in wuxia_cards
    assert "required_flags: [sapa_route_started, dowol_debt]" in wuxia_cards
    assert "flavor_flags_only: [black_heaven_deal_marked, black_heaven_escape_marker]" in wuxia_cards
    assert "accept_dowol_marker_for_safehouse" in wuxia_cards
    assert "ask_who_collects_the_price" in wuxia_cards
    assert "keep_cheongryu_names_off_ledger" in wuxia_cards
    assert "map_exit_before_following_dowol" in wuxia_cards
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "wuxia_heavenly_archive_previous_outsiders" in storypack_db_readme


def test_wuxia_heavenly_archive_previous_outsiders_runtime_slice_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    decision = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(encoding="utf-8")
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(encoding="utf-8")
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(encoding="utf-8")

    assert "## 0.24 2026-06-02 docs-only route opener follow-up handoff: `wuxia_heavenly_archive_previous_outsiders`" in plan
    assert "## 0.25 2026-06-02 무협 `wuxia_heavenly_archive_previous_outsiders` preview runtime slice" in plan
    assert "### 0.2am 2026-06-02 무협 route opener follow-up after black heaven docs-only handoff" in checklist
    assert "### 0.2an 2026-06-02 무협 `wuxia_heavenly_archive_previous_outsiders` preview runtime slice" in checklist
    assert "`wuxia_heavenly_archive_previous_outsiders` — preview runtime 구현 완료" in decision
    assert "Route opener follow-up after black heaven" in coverage
    assert "Route opener follow-up after black heaven implementation" in coverage
    assert "wuxia_heavenly_archive_previous_outsiders" in coverage
    assert "| `wuxia_heavenly_archive_previous_outsiders` | `route_commitment` / `cheonggi_return`" in wuxia_pack
    assert "## 11. `wuxia_heavenly_archive_previous_outsiders`" in wuxia_cards
    assert "runtime_preview_design_status: implemented" in wuxia_cards
    assert "required_flags: [cheonggi_return_route_started, cheonggi_record_targeted]" in wuxia_cards
    assert "flavor_flags_only: [heavenly_archive_contact, heavenly_archive_triage_map_seen]" in wuxia_cards
    assert "read_previous_outsider_margins" in wuxia_cards
    assert "ask_yeon_soha_what_not_to_read" in wuxia_cards
    assert "mark_current_worldline_without_answer" in wuxia_cards
    assert "compare_rift_terms_to_commute_memory" in wuxia_cards
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme


def test_wuxia_wounded_shelter_dawn_offers_runtime_slice_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    decision = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(encoding="utf-8")
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(encoding="utf-8")
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(encoding="utf-8")
    world_model = Path("docs/design/Storypack_World_Model.md").read_text(encoding="utf-8")
    encounter_model = Path("docs/design/Storypack_Encounter_DB.md").read_text(encoding="utf-8")

    assert "## 0.26 2026-06-02 docs-only route opener follow-up handoff: `wuxia_wounded_shelter_dawn_offers`" in plan
    assert "## 0.27 2026-06-02 무협 `wuxia_wounded_shelter_dawn_offers` preview runtime slice" in plan
    assert "### 0.2ao 2026-06-02 무협 route opener follow-up after heavenly archive docs-only handoff" in checklist
    assert "### 0.2ap 2026-06-02 무협 `wuxia_wounded_shelter_dawn_offers` preview runtime slice" in checklist
    assert "`wuxia_wounded_shelter_dawn_offers` — preview runtime 구현 완료" in decision
    assert "Route opener follow-up after heavenly archive" in coverage
    assert "Route opener follow-up after heavenly archive implementation" in coverage
    assert "wuxia_wounded_shelter_dawn_offers" in coverage
    assert "| `wuxia_wounded_shelter_dawn_offers` | `route_commitment`" in wuxia_pack
    assert "## 12. `wuxia_wounded_shelter_dawn_offers`" in wuxia_cards
    assert "runtime_preview_design_status: implemented" in wuxia_cards
    assert "required_flags: [cheongryu_raid_wounded_fallback_resolved, route_commitment_deferred, deferred_route_reopened, wounded_shelter_stabilized]" in wuxia_cards
    assert "flavor_flags_only: [survivor_roll_call_complete, route_delay_cost_recorded]" in wuxia_cards
    assert "keep_wounded_shelter_until_noon" in wuxia_cards
    assert "accept_baekdo_medicine_after_roll_call" in wuxia_cards
    assert "send_word_to_dowol_for_quiet_exit" in wuxia_cards
    assert "show_archive_map_to_yeon_soha" in wuxia_cards
    assert "runtime_preview_implementation_notes" in wuxia_cards
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "route_midgame_continuity_after_wounded_shelter" in world_model
    assert "route_midgame_continuity_after_wounded_shelter" in encounter_model
    assert "wuxia_wounded_shelter_dawn_offers" in world_model
    assert "wuxia_wounded_shelter_dawn_offers" in encounter_model


def test_wuxia_mumyeong_first_sighting_handoff_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    decision = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(encoding="utf-8")
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(encoding="utf-8")
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(encoding="utf-8")
    world_model = Path("docs/design/Storypack_World_Model.md").read_text(encoding="utf-8")
    encounter_model = Path("docs/design/Storypack_Encounter_DB.md").read_text(encoding="utf-8")

    assert "## 0.28 2026-06-02 docs-only midgame continuity handoff: `wuxia_mumyeong_first_sighting`" in plan
    assert "### 0.2aq 2026-06-02 무협 post-opener midgame continuity docs-only handoff" in checklist
    assert "`wuxia_mumyeong_first_sighting` — preview runtime 구현 완료" in decision
    assert "Post-opener midgame continuity handoff" in coverage
    assert "| 10 | `wuxia_mumyeong_first_sighting` | 무명 첫 목격 | `wuxia_mumyeong_first_sighting`" in coverage
    assert "| `wuxia_mumyeong_first_sighting` | `midgame_rival`" in wuxia_pack
    assert "## 13. `wuxia_mumyeong_first_sighting`" in wuxia_cards
    assert "runtime_preview_design_status: implemented" in wuxia_cards
    assert "required_flags: [route_opener_resolved, cheongryu_raid_survived, cheongryu_trial_started, first_fragment_seen]" in wuxia_cards
    assert "flavor_flags_only: [righteous_route_opened, sapa_route_opened, cheonggi_return_route_opened" in wuxia_cards
    assert "watch_the_stolen_qingliu_flow" in wuxia_cards
    assert "check_seo_harin_silence" in wuxia_cards
    assert "follow_black_serpent_runner" in wuxia_cards
    assert "pretend_not_to_see_the_form" in wuxia_cards
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "wuxia_mumyeong_first_sighting" in world_model
    assert "wuxia_mumyeong_first_sighting" in encounter_model


def test_wuxia_mumyeong_first_sighting_runtime_slice_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    decision = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(encoding="utf-8")
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(encoding="utf-8")
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(encoding="utf-8")
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(encoding="utf-8")
    world_model = Path("docs/design/Storypack_World_Model.md").read_text(encoding="utf-8")
    encounter_model = Path("docs/design/Storypack_Encounter_DB.md").read_text(encoding="utf-8")

    assert "## 0.29 2026-06-02 무협 `wuxia_mumyeong_first_sighting` preview runtime slice" in plan
    assert "### 0.2ar 2026-06-02 무협 `wuxia_mumyeong_first_sighting` preview runtime slice" in checklist
    assert "`wuxia_mumyeong_first_sighting` — preview runtime 구현 완료" in decision
    assert "Post-opener midgame continuity implementation" in coverage
    assert "preview runtime implemented as common post-opener midgame bridge" in coverage
    assert (
        "`wuxia_mumyeong_first_sighting`, `wuxia_mumyeong_first_confrontation`, "
        "`wuxia_mumyeong_copy_style_reveal`, `wuxia_mumyeong_reads_orthodox_style` "
        "runtime은 separate storypack preview bundle에서 완료"
    ) in wuxia_pack
    assert "| `wuxia_mumyeong_first_sighting` | `midgame_rival`" in wuxia_pack
    assert "| `wuxia_mumyeong_first_sighting` | `midgame_rival` | `sect_courtyard`, `market_street`, `training_chore` |" in wuxia_pack
    assert "## 13. `wuxia_mumyeong_first_sighting`" in wuxia_cards
    assert "mapping_status: preview_runtime_implemented" in wuxia_cards
    assert "runtime_preview_implementation_notes" in wuxia_cards
    assert "implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml" in wuxia_cards
    assert "next_handoff: wuxia_mumyeong_first_confrontation_after_sighting" in wuxia_cards
    assert "무명 첫 목격/첫 대치/카피 무공 공개" in storypack_db_readme
    assert "wuxia_mumyeong_followup_after_copy_style_reveal" in storypack_db_json
    assert "wuxia_mumyeong_first_confrontation_after_sighting" in world_model
    assert "wuxia_mumyeong_first_confrontation_after_sighting" in encounter_model


def test_wuxia_mumyeong_first_confrontation_runtime_slice_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    decision = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    situations_json = Path("docs/content/storypack_db/encounter_situations.json").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    world_model = Path("docs/design/Storypack_World_Model.md").read_text(encoding="utf-8")
    encounter_model = Path("docs/design/Storypack_Encounter_DB.md").read_text(
        encoding="utf-8"
    )

    assert (
        "## 0.30 2026-06-02 docs-only rival confrontation handoff: "
        "`wuxia_mumyeong_first_confrontation`"
    ) in plan
    assert (
        "## 0.31 2026-06-02 무협 `wuxia_mumyeong_first_confrontation` "
        "preview runtime slice"
    ) in plan
    assert "### 0.2as 2026-06-02 무협 rival confrontation docs-only handoff" in checklist
    assert (
        "### 0.2at 2026-06-02 무협 `wuxia_mumyeong_first_confrontation` "
        "preview runtime slice"
    ) in checklist
    assert "`wuxia_mumyeong_first_confrontation` — preview runtime 구현 완료" in decision
    assert "Rival confrontation handoff" in coverage
    assert "| 11 | `wuxia_mumyeong_first_confrontation` | 무명 첫 대치 | `wuxia_mumyeong_first_confrontation`" in coverage
    assert "| `wuxia_mumyeong_first_confrontation` | `midgame_rival` / `rival_confrontation`" in wuxia_pack
    assert "## 14. `wuxia_mumyeong_first_confrontation`" in wuxia_cards
    assert "mapping_status: preview_runtime_implemented" in wuxia_cards
    assert "runtime_preview_design_status: implemented" in wuxia_cards
    assert "required_flags: [mumyeong_first_sighting_resolved, midgame_continuity_started, cheongryu_raid_survived, first_fragment_seen]" in wuxia_cards
    assert "flavor_flags_only: [mumyeong_shadow_seen, copied_qingliu_flow_noted" in wuxia_cards
    assert "meet_mumyeong_head_on" in wuxia_cards
    assert "endure_until_copy_flow_breaks" in wuxia_cards
    assert "watch_seo_harin_hold_back" in wuxia_cards
    assert "read_mumyeongs_copied_form" in wuxia_cards
    assert "do_not_provoke_mumyeong" in wuxia_cards
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "무명 첫 목격/첫 대치/카피 무공 공개" in storypack_db_readme
    assert "wuxia_mumyeong_first_confrontation" in storypack_db_json
    assert "wuxia_mumyeong_followup_after_copy_style_reveal" in storypack_db_json
    assert '"id": "wuxia_mumyeong_first_confrontation"' in situations_json
    assert '"runtime_preview_design_status": "implemented"' in situations_json
    assert '"implemented_source": "src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml"' in situations_json
    assert '"next_handoff": "wuxia_mumyeong_followup_after_copy_style_reveal"' in situations_json
    assert "wuxia_mumyeong_first_confrontation" in world_model
    assert "wuxia_mumyeong_first_confrontation" in encounter_model


def test_wuxia_mumyeong_copy_style_reveal_runtime_slice_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    decision = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    situations_json = Path("docs/content/storypack_db/encounter_situations.json").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    world_model = Path("docs/design/Storypack_World_Model.md").read_text(encoding="utf-8")
    encounter_model = Path("docs/design/Storypack_Encounter_DB.md").read_text(
        encoding="utf-8"
    )

    assert (
        "## 0.32 2026-06-02 docs-only post-confrontation handoff: "
        "`wuxia_mumyeong_copy_style_reveal`"
    ) in plan
    assert (
        "## 0.33 2026-06-02 무협 `wuxia_mumyeong_copy_style_reveal` "
        "preview runtime slice"
    ) in plan
    assert (
        "### 0.2au 2026-06-02 무협 post-confrontation follow-up "
        "docs-only handoff"
    ) in checklist
    assert (
        "### 0.2av 2026-06-02 무협 `wuxia_mumyeong_copy_style_reveal` "
        "preview runtime slice"
    ) in checklist
    assert "`wuxia_mumyeong_copy_style_reveal` — preview runtime 구현 완료" in decision
    assert "Post-confrontation follow-up handoff" in coverage
    assert "Post-confrontation follow-up implementation" in coverage
    assert (
        "| 6 | `wuxia_mumyeong_copy_style_reveal` | 무명의 카피 무공 공개 | "
        "`wuxia_mumyeong_copy_style_reveal`"
    ) in coverage
    assert (
        "| 6 | `wuxia_mumyeong_copy_style_reveal` | 무명의 카피 무공 공개 | "
        "`wuxia_mumyeong_copy_style_reveal` | preview runtime implemented |"
    ) in coverage
    assert "| `wuxia_mumyeong_copy_style_reveal` | `midgame_rival` / `copy_style_analysis`" in wuxia_pack
    assert "15. `wuxia_mumyeong_copy_style_reveal` — preview runtime 구현 완료" in wuxia_pack
    assert "## 15. `wuxia_mumyeong_copy_style_reveal`" in wuxia_cards
    assert "mapping_status: preview_runtime_implemented" in wuxia_cards
    assert "runtime_preview_design_status: implemented" in wuxia_cards
    assert "required_flags: [mumyeong_first_confrontation_resolved, mumyeong_rival_thread_opened, midgame_continuity_started]" in wuxia_cards
    assert "flavor_flags_only: [copied_flow_weakness_noted, cheonggi_copy_contrast_noted" in wuxia_cards
    assert "read_the_stolen_blade_path" in wuxia_cards
    assert "watch_mumyeongs_footwork" in wuxia_cards
    assert "listen_for_breath_mismatch" in wuxia_cards
    assert "wait_for_body_to_shudder" in wuxia_cards
    assert "runtime_preview_implementation_notes" in wuxia_cards
    assert "implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml" in wuxia_cards
    assert "next_handoff: wuxia_mumyeong_followup_after_copy_style_reveal" in wuxia_cards
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "무명 첫 목격/첫 대치/카피 무공 공개/정파 무공 간파" in storypack_db_readme
    assert "wuxia_mumyeong_copy_style_reveal" in storypack_db_json
    assert "wuxia_mumyeong_followup_after_copy_style_reveal" in storypack_db_json
    assert '"id": "wuxia_mumyeong_copy_style_reveal"' in situations_json
    assert '"runtime_preview_design_status": "implemented"' in situations_json
    assert '"runtime_preview_implementation_notes"' in situations_json
    assert '"implemented_source": "src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml"' in situations_json
    assert '"next_handoff": "wuxia_mumyeong_followup_after_copy_style_reveal"' in situations_json
    assert "wuxia_mumyeong_copy_style_reveal" in world_model
    assert "wuxia_mumyeong_followup_after_copy_style_reveal" in world_model
    assert "wuxia_mumyeong_copy_style_reveal" in encounter_model
    assert "wuxia_mumyeong_followup_after_copy_style_reveal" in encounter_model


def test_wuxia_mumyeong_reads_orthodox_style_runtime_slice_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    decision = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    situations_json = Path("docs/content/storypack_db/encounter_situations.json").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    world_model = Path("docs/design/Storypack_World_Model.md").read_text(encoding="utf-8")
    encounter_model = Path("docs/design/Storypack_Encounter_DB.md").read_text(
        encoding="utf-8"
    )

    assert (
        "## 0.34 2026-06-02 docs-only post-copy-style handoff: "
        "`wuxia_mumyeong_reads_orthodox_style`"
    ) in plan
    assert (
        "## 0.35 2026-06-02 무협 `wuxia_mumyeong_reads_orthodox_style` "
        "preview runtime slice"
    ) in plan
    assert "### 0.2aw 2026-06-02 무협 post-copy-style follow-up docs-only handoff" in checklist
    assert "### 0.2ax 2026-06-02 무협 `wuxia_mumyeong_reads_orthodox_style` preview runtime slice" in checklist
    assert "### 0.2ay 2026-06-02 terminal default storypack 전환" in checklist
    assert "`wuxia_mumyeong_reads_orthodox_style` — preview runtime 구현 완료" in decision
    assert "terminal `--scene content` also defaults to the same built-in bundle" in decision
    assert "Post-copy-style follow-up handoff" in coverage
    assert "Post-copy-style follow-up implementation" in coverage
    assert (
        "| 16 | `wuxia_mumyeong_reads_orthodox_style` | 무명의 정파 무공 간파 | "
        "`wuxia_mumyeong_reads_orthodox_style` | preview runtime implemented |"
    ) in coverage
    assert "| `wuxia_mumyeong_reads_orthodox_style` | `midgame_rival` / `orthodox_style_trace`" in wuxia_pack
    assert "16. `wuxia_mumyeong_reads_orthodox_style` — preview runtime 구현 완료" in wuxia_pack
    assert "## 16. `wuxia_mumyeong_reads_orthodox_style`" in wuxia_cards
    assert "mapping_status: preview_runtime_implemented" in wuxia_cards
    assert "runtime_preview_design_status: implemented" in wuxia_cards
    assert "required_flags: [mumyeong_copy_style_reveal_resolved, copy_style_hint_recorded, midgame_continuity_started, first_fragment_seen]" in wuxia_cards
    assert "compare_copied_form_to_old_wound" in wuxia_cards
    assert "trace_qingliu_eye_variation" in wuxia_cards
    assert "reconstruct_mumyeongs_sightline" in wuxia_cards
    assert "stop_before_truth_becomes_accusation" in wuxia_cards
    assert "hyeonakmun_trace_suspected" in wuxia_cards
    assert "bokho_geumsaesu_name_recorded" in wuxia_cards
    assert "runtime_preview_implementation_notes" in wuxia_cards
    assert "implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml" in wuxia_cards
    assert "next_handoff: wuxia_mumyeong_followup_after_orthodox_style_trace" in wuxia_cards
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "`wuxia_mumyeong_reads_orthodox_style`는 preview runtime에 구현" in storypack_db_readme
    assert "wuxia_mumyeong_reads_orthodox_style" in storypack_db_json
    assert "wuxia_mumyeong_followup_after_orthodox_style_trace" in storypack_db_json
    assert '"id": "wuxia_mumyeong_reads_orthodox_style"' in situations_json
    assert '"runtime_preview_design_status": "implemented"' in situations_json
    assert '"runtime_preview_implementation_notes"' in situations_json
    assert '"implemented_source": "src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml"' in situations_json
    assert '"next_handoff": "wuxia_mumyeong_followup_after_orthodox_style_trace"' in situations_json
    assert "wuxia_mumyeong_reads_orthodox_style" in world_model
    assert "wuxia_mumyeong_followup_after_orthodox_style_trace" in world_model
    assert "wuxia_mumyeong_reads_orthodox_style" in encounter_model
    assert "wuxia_mumyeong_followup_after_orthodox_style_trace" in encounter_model


def test_wuxia_mumyeong_midgame_reunion_runtime_slice_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    situations_json = Path("docs/content/storypack_db/encounter_situations.json").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    world_model = Path("docs/design/Storypack_World_Model.md").read_text(encoding="utf-8")
    encounter_model = Path("docs/design/Storypack_Encounter_DB.md").read_text(
        encoding="utf-8"
    )
    notion_sources = Path("idea_box/notion_sources.yml").read_text(encoding="utf-8")

    assert (
        "## 0.36 2026-06-02 docs-only orthodox-style follow-up handoff: "
        "`wuxia_mumyeong_midgame_reunion`"
    ) in plan
    assert (
        "## 0.37 2026-06-02 무협 `wuxia_mumyeong_midgame_reunion` "
        "preview runtime slice"
    ) in plan
    assert "### 0.2az 2026-06-02 무협 orthodox-style follow-up docs-only handoff" in checklist
    assert "### 0.2ba 2026-06-02 무협 `wuxia_mumyeong_midgame_reunion` preview runtime slice" in checklist
    assert "Orthodox-style follow-up handoff" in coverage
    assert "Orthodox-style follow-up implementation" in coverage
    assert (
        "| 8 | `wuxia_mumyeong_midgame_reunion` | 무명 중반 재회 | "
        "`wuxia_mumyeong_midgame_reunion` | preview runtime implemented |"
    ) in coverage
    assert "| `wuxia_mumyeong_midgame_reunion` | `midgame_rival` / `rival_reunion`" in wuxia_pack
    assert "17. `wuxia_mumyeong_midgame_reunion` — preview runtime 구현 완료" in wuxia_pack
    assert "## 17. `wuxia_mumyeong_midgame_reunion`" in wuxia_cards
    assert "mapping_status: preview_runtime_implemented" in wuxia_cards
    assert "runtime_preview_design_status: implemented" in wuxia_cards
    assert "required_flags: [mumyeong_reads_orthodox_style_resolved, orthodox_style_trace_recorded, mumyeong_first_confrontation_resolved, mumyeong_rival_thread_opened]" in wuxia_cards
    assert "forbidden_flags: [mumyeong_midgame_reunion_resolved]" in wuxia_cards
    assert "ask_why_seoharin_never_called_him_traitor" in wuxia_cards
    assert "show_the_hyeonakmun_trace_without_accusing" in wuxia_cards
    assert "point_out_the_copied_form_gap" in wuxia_cards
    assert "keep_blades_low_and_watch_his_answer" in wuxia_cards
    assert "boss_used_mumyeongs_wound" in wuxia_cards
    assert "runtime_preview_implementation_notes" in wuxia_cards
    assert "implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml" in wuxia_cards
    assert "next_handoff: wuxia_mumyeong_followup_after_midgame_reunion" in wuxia_cards
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "`wuxia_mumyeong_midgame_reunion`는 preview runtime에 구현" in storypack_db_readme
    assert "wuxia_mumyeong_followup_after_midgame_reunion" in storypack_db_json
    assert "wuxia_mumyeong_midgame_reunion" in storypack_db_json
    assert '"id": "wuxia_mumyeong_midgame_reunion"' in situations_json
    assert '"runtime_preview_design_status": "implemented"' in situations_json
    assert '"runtime_preview_implementation_notes"' in situations_json
    assert '"implemented_source": "src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml"' in situations_json
    assert '"insert_after": "wuxia_mumyeong_reads_orthodox_style"' in situations_json
    assert '"next_handoff": "wuxia_mumyeong_followup_after_midgame_reunion"' in situations_json
    assert "wuxia_mumyeong_midgame_reunion" in world_model
    assert "wuxia_mumyeong_followup_after_midgame_reunion" in world_model
    assert "wuxia_mumyeong_midgame_reunion" in encounter_model
    assert "wuxia_mumyeong_followup_after_midgame_reunion" in encounter_model
    assert "runtime_status: \"implemented_in_storypack_preview\"" in notion_sources


def test_wuxia_boss_first_appearance_runtime_slice_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    situations_json = Path("docs/content/storypack_db/encounter_situations.json").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    world_model = Path("docs/design/Storypack_World_Model.md").read_text(encoding="utf-8")
    encounter_model = Path("docs/design/Storypack_Encounter_DB.md").read_text(
        encoding="utf-8"
    )
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    notion_sources = Path("idea_box/notion_sources.yml").read_text(encoding="utf-8")

    assert (
        "## 0.38 2026-06-02 docs-only midgame-reunion follow-up handoff: "
        "`wuxia_boss_first_appearance`"
    ) in plan
    assert (
        "## 0.39 2026-06-02 무협 `wuxia_boss_first_appearance` "
        "preview runtime slice"
    ) in plan
    assert "### 0.2bb 2026-06-02 무협 midgame-reunion follow-up docs-only handoff" in checklist
    assert "### 0.2bc 2026-06-02 무협 `wuxia_boss_first_appearance` preview runtime slice" in checklist
    assert "Midgame reunion follow-up handoff" in coverage
    assert "Boss first appearance implementation" in coverage
    assert (
        "| 12 | `wuxia_boss_first_appearance` | 보스 첫 등장 | "
        "`wuxia_boss_first_appearance` | preview runtime implemented |"
    ) in coverage
    assert "| `wuxia_boss_first_appearance` | `midgame_boss` / `boss_wall_pressure`" in wuxia_pack
    assert "18. `wuxia_boss_first_appearance` — preview runtime 구현 완료" in wuxia_pack
    assert "## 18. `wuxia_boss_first_appearance`" in wuxia_cards
    assert "mapping_status: preview_runtime_implemented" in wuxia_cards
    assert "runtime_preview_design_status: implemented" in wuxia_cards
    assert "required_flags: [mumyeong_midgame_reunion_resolved, mumyeong_mirror_thread_deepened, cheongryu_raid_survived, midgame_continuity_started]" in wuxia_cards
    assert "forbidden_flags: [boss_first_appearance_resolved]" in wuxia_cards
    assert "read_the_boss_flow_and_fail_to_move" in wuxia_cards
    assert "pull_seo_harin_behind_broken_gate" in wuxia_cards
    assert "watch_mumyeong_answer_the_boss" in wuxia_cards
    assert "retreat_before_the_second_step" in wuxia_cards
    assert "boss_reads_people_not_forms" in wuxia_cards
    assert "implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml" in wuxia_cards
    assert "next_handoff: wuxia_boss_followup_after_first_appearance" in wuxia_cards
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "`wuxia_boss_first_appearance`는 preview runtime에 구현" in storypack_db_readme
    assert "wuxia_boss_first_appearance" in storypack_db_json
    assert "wuxia_boss_followup_after_first_appearance" in storypack_db_json
    assert '"id": "wuxia_boss_first_appearance"' in situations_json
    assert '"runtime_preview_design_status": "implemented"' in situations_json
    assert '"implemented_source": "src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml"' in situations_json
    assert '"insert_after": "wuxia_mumyeong_midgame_reunion"' in situations_json
    assert '"next_handoff": "wuxia_boss_followup_after_first_appearance"' in situations_json
    assert "wuxia_boss_first_appearance" in world_model
    assert "wuxia_mumyeong_request_for_aid" in world_model
    assert "wuxia_boss_first_appearance" in encounter_model
    assert "wuxia_mumyeong_request_for_aid" in encounter_model
    assert "wuxia_boss_first_appearance" in runtime_preview
    assert "runtime_status: \"implemented_in_storypack_preview\"" in notion_sources


def test_wuxia_mumyeong_request_for_aid_runtime_slice_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    situations_json = Path("docs/content/storypack_db/encounter_situations.json").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    world_model = Path("docs/design/Storypack_World_Model.md").read_text(encoding="utf-8")
    encounter_model = Path("docs/design/Storypack_Encounter_DB.md").read_text(
        encoding="utf-8"
    )
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    notion_sources = Path("idea_box/notion_sources.yml").read_text(encoding="utf-8")

    assert (
        "## 0.40 2026-06-02 docs-only boss follow-up handoff: "
        "`wuxia_mumyeong_request_for_aid`"
    ) in plan
    assert (
        "## 0.41 2026-06-02 무협 `wuxia_mumyeong_request_for_aid` "
        "preview runtime slice"
    ) in plan
    assert "### 0.2bd 2026-06-02 무협 boss follow-up docs-only handoff" in checklist
    assert "### 0.2be 2026-06-02 무협 `wuxia_mumyeong_request_for_aid` preview runtime slice" in checklist
    assert "Boss follow-up handoff" in coverage
    assert "Mumyeong aid request implementation" in coverage
    assert (
        "| 18 | `wuxia_mumyeong_request_for_aid` | 무명의 도움 요청 | "
        "`wuxia_mumyeong_request_for_aid` | preview runtime implemented |"
    ) in coverage
    assert "| `wuxia_mumyeong_request_for_aid` | `midgame_rival` / `failed_aid_records`" in wuxia_pack
    assert "19. `wuxia_mumyeong_request_for_aid` — preview runtime 구현 완료" in wuxia_pack
    assert "## 19. `wuxia_mumyeong_request_for_aid`" in wuxia_cards
    assert "mapping_status: preview_runtime_implemented" in wuxia_cards
    assert "status: implemented_in_storypack_preview" in wuxia_cards
    assert "runtime_preview_design_status: implemented" in wuxia_cards
    assert "required_flags: [boss_first_appearance_resolved, boss_wall_thread_opened, black_serpent_core_pressure_opened, mumyeong_mirror_thread_deepened, orthodox_style_trace_recorded, midgame_continuity_started]" in wuxia_cards
    assert "forbidden_flags: [mumyeong_request_for_aid_resolved]" in wuxia_cards
    assert "search_the_rejected_aid_letters" in wuxia_cards
    assert "follow_old_inn_rumors_about_mumyeong" in wuxia_cards
    assert "ask_seo_harin_what_help_never_came" in wuxia_cards
    assert "keep_the_failed_aid_record_unshown" in wuxia_cards
    assert "mumyeong_tried_to_save_qingliu" in wuxia_cards
    assert "orthodox_refusal_broke_mumyeong" in wuxia_cards
    assert "implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml" in wuxia_cards
    assert "insert_after: wuxia_boss_first_appearance" in wuxia_cards
    assert "next_handoff: wuxia_mumyeong_followup_after_failed_aid" in wuxia_cards
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "`wuxia_mumyeong_request_for_aid`는 preview runtime에 구현됐고" in storypack_db_readme
    assert "wuxia_mumyeong_request_for_aid" in storypack_db_json
    assert "wuxia_mumyeong_followup_after_failed_aid" in storypack_db_json
    assert '"id": "wuxia_mumyeong_request_for_aid"' in situations_json
    assert '"runtime_preview_design_status": "implemented"' in situations_json
    assert '"implemented_source": "src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml"' in situations_json
    assert '"insert_after": "wuxia_boss_first_appearance"' in situations_json
    assert '"next_handoff": "wuxia_mumyeong_followup_after_failed_aid"' in situations_json
    assert "wuxia_mumyeong_request_for_aid" in world_model
    assert "wuxia_mumyeong_followup_after_failed_aid" in world_model
    assert "wuxia_mumyeong_request_for_aid" in encounter_model
    assert "wuxia_mumyeong_followup_after_failed_aid" in encounter_model
    assert "wuxia_mumyeong_request_for_aid" in runtime_preview
    assert "wuxia_mumyeong_followup_after_failed_aid" in runtime_preview
    assert "runtime_status: \"implemented_in_storypack_preview\"" in notion_sources


def test_wuxia_mumyeong_awakening_runtime_slice_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    situations_json = Path("docs/content/storypack_db/encounter_situations.json").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    notion_sources = Path("idea_box/notion_sources.yml").read_text(encoding="utf-8")

    assert (
        "## 0.42 2026-06-02 docs-only failed-aid follow-up handoff: "
        "`wuxia_mumyeong_awakening`"
    ) in plan
    assert "## 0.43 2026-06-02 무협 `wuxia_mumyeong_awakening` preview runtime slice" in plan
    assert "### 0.2bf 2026-06-02 무협 failed-aid follow-up docs-only handoff" in checklist
    assert "### 0.2bg 2026-06-02 무협 `wuxia_mumyeong_awakening` preview runtime slice" in checklist
    assert "Failed-aid follow-up handoff" in coverage
    assert "Mumyeong awakening implementation" in coverage
    assert (
        "| 14 | `wuxia_mumyeong_awakening` | 무명의 각성 | "
        "`wuxia_mumyeong_awakening` | preview runtime implemented |"
    ) in coverage
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "`wuxia_mumyeong_awakening`도 preview runtime에 구현" in storypack_db_readme
    assert "wuxia_mumyeong_awakening" in storypack_db_json
    assert "wuxia_mumyeong_followup_after_awakening" in storypack_db_json
    assert '"id": "wuxia_mumyeong_awakening"' in situations_json
    assert '"runtime_preview_design_status": "implemented"' in situations_json
    assert '"implemented_source": "src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml"' in situations_json
    assert '"insert_after": "wuxia_mumyeong_request_for_aid"' in situations_json
    assert '"generated_artifacts": [' in situations_json
    assert '"next_handoff": "wuxia_mumyeong_followup_after_awakening"' in situations_json
    assert "| `wuxia_mumyeong_awakening` | `midgame_rival` / `anger_copy_bloom`" in wuxia_pack
    assert "20. `wuxia_mumyeong_awakening` — preview runtime 구현 완료" in wuxia_pack
    assert "## 20. `wuxia_mumyeong_awakening`" in wuxia_cards
    assert "mapping_status: preview_runtime_implemented" in wuxia_cards
    assert "status: implemented_in_storypack_preview" in wuxia_cards
    assert "runtime_preview_design_status: implemented" in wuxia_cards
    assert "required_flags: [mumyeong_request_for_aid_resolved, mumyeong_failed_aid_thread_opened, orthodox_hypocrisy_thread_opened, mumyeong_reads_orthodox_style_resolved, orthodox_style_trace_recorded, mumyeong_copy_style_reveal_resolved, copy_style_hint_recorded, midgame_continuity_started]" in wuxia_cards
    assert "forbidden_flags: [mumyeong_awakening_resolved]" in wuxia_cards
    assert "implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml" in wuxia_cards
    assert "generated_artifacts:" in wuxia_cards
    assert "default_bundle_changed: false" in wuxia_cards
    assert "new_schema_opened: false" in wuxia_cards
    assert "next_handoff: wuxia_mumyeong_followup_after_awakening" in wuxia_cards
    assert "wuxia_mumyeong_awakening` — 구현 완료" in runtime_preview
    assert "wuxia_mumyeong_followup_after_awakening" in runtime_preview
    assert "runtime_status: \"implemented_in_storypack_preview\"" in notion_sources


def test_wuxia_qingliu_attack_after_war_handoff_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    situations_json = Path("docs/content/storypack_db/encounter_situations.json").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    world_model = Path("docs/design/Storypack_World_Model.md").read_text(encoding="utf-8")
    encounter_model = Path("docs/design/Storypack_Encounter_DB.md").read_text(
        encoding="utf-8"
    )
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    notion_sources = Path("idea_box/notion_sources.yml").read_text(encoding="utf-8")

    assert (
        "## 0.44 2026-06-02 docs-only awakening follow-up handoff: "
        "`wuxia_qingliu_attack_after_war`"
    ) in plan
    assert "### 0.2bh 2026-06-02 무협 awakening follow-up docs-only handoff" in checklist
    assert "Awakening follow-up handoff" in coverage
    assert (
        "| 17 | `wuxia_qingliu_attack_after_war` | 무너져가는 청류문 습격 | "
        "`wuxia_qingliu_attack_after_war` | preview runtime implemented"
    ) in coverage
    assert "## 0.45 2026-06-02 무협 `wuxia_qingliu_attack_after_war` preview runtime slice" in plan
    assert "### 0.2bi 2026-06-02 무협 `wuxia_qingliu_attack_after_war` preview runtime slice" in checklist
    assert "Qingliu attack trace implementation" in coverage
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "`wuxia_qingliu_attack_after_war`도 preview runtime에 구현" in storypack_db_readme
    assert "wuxia_qingliu_attack_after_war" in storypack_db_json
    assert "wuxia_qingliu_attack_after_war_followup" in storypack_db_json
    assert '"id": "wuxia_qingliu_attack_after_war"' in situations_json
    assert '"runtime_preview_design_status": "implemented"' in situations_json
    assert '"implemented_source": "src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml"' in situations_json
    assert '"insert_after": "wuxia_mumyeong_awakening"' in situations_json
    assert '"generated_artifacts": [' in situations_json
    assert '"default_bundle_changed": false' in situations_json
    assert "| `wuxia_qingliu_attack_after_war` | `midgame_backstory` / `attack_trace_investigation`" in wuxia_pack
    assert "21. `wuxia_qingliu_attack_after_war` — preview runtime 구현 완료" in wuxia_pack
    assert "## 21. `wuxia_qingliu_attack_after_war`" in wuxia_cards
    assert "mapping_status: preview_runtime_implemented" in wuxia_cards
    assert "runtime_preview_design_status: implemented" in wuxia_cards
    assert "implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml" in wuxia_cards
    assert "generated_artifacts:" in wuxia_cards
    assert "next_handoff: wuxia_qingliu_attack_after_war_followup" in wuxia_cards
    assert "wuxia_qingliu_attack_after_war_followup` docs-only handoff" in world_model
    assert "청류문 흔적 조사도 기존 encounter schema로 구현 완료" in encounter_model
    assert "`wuxia_qingliu_attack_after_war` — 구현 완료" in runtime_preview
    assert "runtime_status: \"implemented_in_storypack_preview\"" in notion_sources


def test_wuxia_mumyeong_destroys_orthodox_sect_runtime_slice_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    situations_json = Path("docs/content/storypack_db/encounter_situations.json").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    world_model = Path("docs/design/Storypack_World_Model.md").read_text(encoding="utf-8")
    encounter_model = Path("docs/design/Storypack_Encounter_DB.md").read_text(
        encoding="utf-8"
    )
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    notion_sources = Path("idea_box/notion_sources.yml").read_text(encoding="utf-8")

    assert (
        "## 0.46 2026-06-02 docs-only post-Qingliu trace handoff: "
        "`wuxia_mumyeong_destroys_orthodox_sect`"
    ) in plan
    assert "## 0.47 2026-06-02 무협 `wuxia_mumyeong_destroys_orthodox_sect` preview runtime slice" in plan
    assert "### 0.2bj 2026-06-02 무협 post-Qingliu trace docs-only handoff" in checklist
    assert "### 0.2bk 2026-06-02 무협 `wuxia_mumyeong_destroys_orthodox_sect` preview runtime slice" in checklist
    assert "Post-Qingliu trace handoff" in coverage
    assert "Hyeonakmun consequence trace implementation" in coverage
    assert (
        "| 13 | `wuxia_mumyeong_destroys_orthodox_sect` | 정파 문파 멸문 | "
        "`wuxia_mumyeong_destroys_orthodox_sect` | preview runtime implemented |"
    ) in coverage
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "`wuxia_mumyeong_destroys_orthodox_sect`도 preview runtime에 구현" in storypack_db_readme
    assert "wuxia_mumyeong_destroys_orthodox_sect_followup" in storypack_db_json
    assert '"id": "wuxia_mumyeong_destroys_orthodox_sect"' in situations_json
    assert '"status": "implemented_in_storypack_preview"' in situations_json
    assert '"mapping_status": "preview_runtime_implemented"' in situations_json
    assert '"runtime_preview_design_status": "implemented"' in situations_json
    assert '"implemented_source": "src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml"' in situations_json
    assert '"insert_after": "wuxia_qingliu_attack_after_war"' in situations_json
    assert '"generated_artifacts": [' in situations_json
    assert '"next_handoff": "wuxia_mumyeong_destroys_orthodox_sect_followup"' in situations_json
    assert "| `wuxia_mumyeong_destroys_orthodox_sect` | `midgame_backstory` / `hyeonakmun_consequence_trace`" in wuxia_pack
    assert "22. `wuxia_mumyeong_destroys_orthodox_sect` — preview runtime 구현 완료" in wuxia_pack
    assert "## 22. `wuxia_mumyeong_destroys_orthodox_sect`" in wuxia_cards
    assert "status: implemented_in_storypack_preview" in wuxia_cards
    assert "mapping_status: preview_runtime_implemented" in wuxia_cards
    assert "runtime_preview_design_status: implemented" in wuxia_cards
    assert "implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml" in wuxia_cards
    assert "generated_artifacts:" in wuxia_cards
    assert "next_handoff: wuxia_mumyeong_destroys_orthodox_sect_followup" in wuxia_cards
    assert "서른네 개 implemented preview encounter" in world_model
    assert "현악문 consequence trace도 기존 encounter schema로 구현 완료" in encounter_model
    assert "Latest implemented runtime: `wuxia_black_serpent_aftermath`" in runtime_preview
    assert "Next design/runtime gate: `wuxia_final_epilogue_renderer_contract` handoff" in runtime_preview
    assert "runtime_status: \"implemented_in_storypack_preview\"" in notion_sources


def test_wuxia_boss_recruits_mumyeong_runtime_slice_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    situations_json = Path("docs/content/storypack_db/encounter_situations.json").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    world_model = Path("docs/design/Storypack_World_Model.md").read_text(encoding="utf-8")
    encounter_model = Path("docs/design/Storypack_Encounter_DB.md").read_text(
        encoding="utf-8"
    )
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    notion_sources = Path("idea_box/notion_sources.yml").read_text(encoding="utf-8")

    assert (
        "## 0.48 2026-06-02 docs-only Hyeonakmun consequence follow-up handoff: "
        "`wuxia_boss_recruits_mumyeong`"
    ) in plan
    assert "## 0.49 2026-06-02 무협 `wuxia_boss_recruits_mumyeong` preview runtime slice" in plan
    assert "### 0.2bl 2026-06-02 무협 Hyeonakmun consequence follow-up docs-only handoff" in checklist
    assert "### 0.2bm 2026-06-02 무협 `wuxia_boss_recruits_mumyeong` preview runtime slice" in checklist
    assert "Hyeonakmun consequence follow-up handoff" in coverage
    assert "Boss recruitment trace implementation" in coverage
    assert (
        "| 15 | `wuxia_boss_recruits_mumyeong` | 흑사방 보스의 스카웃 | "
        "`wuxia_boss_recruits_mumyeong` | preview runtime implemented |"
    ) in coverage
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "`wuxia_boss_recruits_mumyeong`도 preview runtime에 구현" in storypack_db_readme
    assert "Historical handoffs: wuxia_mumyeong_followup_after_copy_style_reveal" in storypack_db_json
    assert "wuxia_boss_recruits_mumyeong_followup" in storypack_db_json
    assert "wuxia_black_serpent_aftermath" in storypack_db_json
    assert '"id": "wuxia_boss_recruits_mumyeong"' in situations_json
    assert '"insert_after": "wuxia_mumyeong_destroys_orthodox_sect"' in situations_json
    assert '"next_handoff": "wuxia_boss_recruits_mumyeong_followup"' in situations_json
    assert '"next_runtime_candidate": "wuxia_mumyeong_departure_truth_summary"' in situations_json
    assert "| `wuxia_boss_recruits_mumyeong` | `midgame_backstory` / `boss_recruitment_trace`" in wuxia_pack
    assert "23. `wuxia_boss_recruits_mumyeong` — preview runtime 구현 완료" in wuxia_pack
    assert "## 23. `wuxia_boss_recruits_mumyeong`" in wuxia_cards
    assert "layout: boss_recruitment_trace" in wuxia_cards
    assert "stable_terms: [흑사방주, 무명, 현악문]" in wuxia_cards
    assert "next_handoff: wuxia_boss_recruits_mumyeong_followup" in wuxia_cards
    assert "## 24. `wuxia_mumyeong_departure_truth_summary`" in wuxia_cards
    assert "서른네 개 implemented preview encounter" in world_model
    assert "보스 recruitment trace도 기존 encounter schema로 구현 완료" in encounter_model
    assert "Latest implemented runtime: `wuxia_black_serpent_aftermath`" in runtime_preview
    assert "Next design/runtime gate: `wuxia_final_epilogue_renderer_contract` handoff" in runtime_preview
    assert 'notion_event_id: "wuxia_boss_recruits_mumyeong"' in notion_sources
    assert 'repo_encounter_id: "wuxia_boss_recruits_mumyeong"' in notion_sources


def test_wuxia_boss_recruits_followup_handoff_selects_departure_truth_summary():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    situations_json = Path("docs/content/storypack_db/encounter_situations.json").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    world_model = Path("docs/design/Storypack_World_Model.md").read_text(encoding="utf-8")
    encounter_model = Path("docs/design/Storypack_Encounter_DB.md").read_text(
        encoding="utf-8"
    )
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    notion_sources = Path("idea_box/notion_sources.yml").read_text(encoding="utf-8")

    assert (
        "## 0.50 2026-06-02 docs-only boss recruitment follow-up handoff: "
        "`wuxia_mumyeong_departure_truth_summary`"
    ) in plan
    assert "### 0.2bn 2026-06-02 무협 boss recruitment follow-up docs-only handoff" in checklist
    assert "Boss recruitment follow-up handoff" in coverage
    assert (
        "| 4 | `wuxia_mumyeong_departure_truth_summary` | 무명 이탈의 진실 정리 | "
        "`wuxia_mumyeong_departure_truth_summary` | preview runtime implemented; "
        "sealed summary scope, not truth delivery |"
    ) in coverage
    assert "`wuxia_mumyeong_departure_truth_summary`도 preview runtime에 구현" in storypack_db_readme
    assert "wuxia_black_serpent_aftermath" in storypack_db_json
    assert '"next_runtime_scope": "sealed_departure_truth_summary"' in situations_json
    assert "| `wuxia_mumyeong_departure_truth_summary` | `midgame_backstory` / `sealed_departure_truth_summary`" in wuxia_pack
    assert "24. `wuxia_mumyeong_departure_truth_summary` — preview runtime 구현 완료" in wuxia_pack
    assert "## 24. `wuxia_mumyeong_departure_truth_summary` — preview runtime 구현 완료" in wuxia_cards
    assert "layout: sealed_departure_truth_summary" in wuxia_cards
    assert "assemble_departure_truth_without_delivering" in wuxia_cards
    assert "compare_failed_aid_to_recruitment_offer" in wuxia_cards
    assert "ask_seoharin_what_she_is_ready_to_hear" in wuxia_cards
    assert "seal_truth_until_mumyeong_faces_it" in wuxia_cards
    assert "implementation_status: implemented" in wuxia_cards
    assert "next_handoff: wuxia_mumyeong_departure_truth_summary_followup" in wuxia_cards
    assert "`wuxia_boss_resolution` handoff" in world_model
    assert "sealed departure truth summary도 기존 encounter schema로 구현 완료" in encounter_model
    assert "Next design/runtime gate: `wuxia_final_epilogue_renderer_contract` handoff" in runtime_preview
    assert 'notion_event_id: "wuxia_mumyeong_departure_truth_summary"' in notion_sources
    assert 'runtime_status: "implemented_in_storypack_preview"' in notion_sources


def test_wuxia_mumyeong_departure_truth_summary_runtime_slice_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    situations_json = Path("docs/content/storypack_db/encounter_situations.json").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    notion_sources = Path("idea_box/notion_sources.yml").read_text(encoding="utf-8")
    encounters_yaml = Path(
        "src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml"
    ).read_text(encoding="utf-8")

    assert "## 0.51 2026-06-02 무협 `wuxia_mumyeong_departure_truth_summary` preview runtime slice" in plan
    assert "### 0.2bo 2026-06-02 무협 `wuxia_mumyeong_departure_truth_summary` preview runtime slice" in checklist
    assert "Mumyeong departure truth summary implementation" in coverage
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "wuxia_black_serpent_aftermath" in storypack_db_json
    assert '"id": "wuxia_mumyeong_departure_truth_summary"' in situations_json
    assert '"status": "implemented_in_storypack_preview"' in situations_json
    assert '"insert_after": "wuxia_boss_recruits_mumyeong"' in situations_json
    assert '"next_handoff": "wuxia_mumyeong_departure_truth_summary_followup"' in situations_json
    assert "24. `wuxia_mumyeong_departure_truth_summary` — preview runtime 구현 완료" in wuxia_pack
    assert "status: implemented_in_storypack_preview" in wuxia_cards
    assert "mapping_status: preview_runtime_implemented" in wuxia_cards
    assert "runtime_preview_design_status: implemented" in wuxia_cards
    assert "implemented_source: src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml" in wuxia_cards
    assert "next_handoff: wuxia_mumyeong_departure_truth_summary_followup" in wuxia_cards
    assert "Latest implemented runtime: `wuxia_black_serpent_aftermath`" in runtime_preview
    assert "Next design/runtime gate: `wuxia_final_epilogue_renderer_contract` handoff" in runtime_preview
    assert 'runtime_status: "implemented_in_storypack_preview"' in notion_sources
    assert "told_seoharin_truth" not in encounters_yaml


def test_wuxia_departure_truth_summary_followup_selects_seoharin_empty_place():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    situations_json = Path("docs/content/storypack_db/encounter_situations.json").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    world_model = Path("docs/design/Storypack_World_Model.md").read_text(
        encoding="utf-8"
    )
    encounter_model = Path("docs/design/Storypack_Encounter_DB.md").read_text(
        encoding="utf-8"
    )
    notion_sources = Path("idea_box/notion_sources.yml").read_text(encoding="utf-8")

    assert (
        "## 0.52 2026-06-02 docs-only departure-truth-summary follow-up handoff: "
        "`wuxia_seoharin_empty_place`"
    ) in plan
    assert "### 0.2bp 2026-06-02 무협 departure truth summary follow-up docs-only handoff" in checklist
    assert "Departure truth summary follow-up handoff" in coverage
    assert (
        "| 3 | `wuxia_seoharin_empty_place` | 비워둔 자리 | "
        "`wuxia_seoharin_empty_place` | preview runtime implemented; "
        "late empty-place bridge, not truth delivery |"
    ) in coverage
    assert "current_goal: wuxia_final_epilogue_renderer_contract_implementation" in next_goal
    assert "previous_current_goal: wuxia_final_epilogue_renderer_contract_handoff" in next_goal
    assert "mode: contract-first-runtime-implementation" in next_goal
    assert "docs/design/Wuxia_Final_State_Routing.md" in next_goal
    assert "wuxia_sado_final_phase_1_price_tag" in next_goal
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "wuxia_black_serpent_aftermath" in storypack_db_json
    assert "wuxia_final_epilogue_renderer_contract_handoff" in storypack_db_json
    assert "wuxia_final_epilogue_renderer_contract" in storypack_db_json
    assert '"id": "wuxia_seoharin_empty_place"' in situations_json
    assert '"mapping_status": "preview_runtime_implemented"' in situations_json
    assert '"runtime_preview_design_status": "implemented"' in situations_json
    assert '"next_runtime_scope": "empty_place_memory_bridge"' in situations_json
    assert '"insert_after": "wuxia_mumyeong_departure_truth_summary"' in situations_json
    assert "25. `wuxia_seoharin_empty_place` — preview runtime 구현 완료" in wuxia_pack
    assert "## 25. `wuxia_seoharin_empty_place` — preview runtime 구현 완료" in wuxia_cards
    assert "runtime_preview_handoff:" in wuxia_cards
    assert "runtime_preview_implementation:" in wuxia_cards
    assert "ask_who_kept_the_empty_place" in wuxia_cards
    assert "leave_the_place_unclaimed" in wuxia_cards
    assert "set_down_the_work_notebook_briefly" in wuxia_cards
    assert "step_back_without_naming_mumyeong" in wuxia_cards
    assert "seoharin_axis_opened" in wuxia_cards
    assert "empty_place_remembered" in wuxia_cards
    assert "Latest implemented runtime: `wuxia_black_serpent_aftermath`" in runtime_preview
    assert "Next design/runtime gate: `wuxia_final_epilogue_renderer_contract` handoff" in runtime_preview
    assert "`wuxia_black_serpent_aftermath`까지 YAML/generated bundle에 반영" in world_model
    assert "late empty-place memory bridge도 기존 encounter schema로 구현 완료" in encounter_model
    assert 'notion_event_id: "wuxia_seoharin_empty_place"' in notion_sources
    assert 'runtime_status: "implemented_in_storypack_preview"' in notion_sources


def test_wuxia_seoharin_left_meal_runtime_slice_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    situations_json = Path("docs/content/storypack_db/encounter_situations.json").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    world_model = Path("docs/design/Storypack_World_Model.md").read_text(
        encoding="utf-8"
    )
    encounter_model = Path("docs/design/Storypack_Encounter_DB.md").read_text(
        encoding="utf-8"
    )
    notion_sources = Path("idea_box/notion_sources.yml").read_text(encoding="utf-8")
    encounters_yaml = Path(
        "src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml"
    ).read_text(encoding="utf-8")

    assert (
        "## 0.54 2026-06-02 docs-only Seo Harin empty-place follow-up handoff: "
        "`wuxia_seoharin_left_meal`"
    ) in plan
    assert "## 0.55 2026-06-02 무협 `wuxia_seoharin_left_meal` preview runtime slice" in plan
    assert "### 0.2br 2026-06-02 무협 Seo Harin empty-place follow-up docs-only handoff" in checklist
    assert "### 0.2bs 2026-06-02 무협 `wuxia_seoharin_left_meal` preview runtime slice" in checklist
    assert "Seo Harin empty-place follow-up handoff" in coverage
    assert "Seo Harin left-meal implementation" in coverage
    assert (
        "| 2 | `wuxia_seoharin_left_meal` | 남겨둔 밥 | "
        "`wuxia_seoharin_left_meal` | preview runtime implemented; "
        "daily-care belonging bridge, not final return choice |"
    ) in coverage
    assert "current_goal: wuxia_final_epilogue_renderer_contract_implementation" in next_goal
    assert "previous_current_goal: wuxia_final_epilogue_renderer_contract_handoff" in next_goal
    assert "wuxia_sado_final_phase_1_price_tag" in next_goal
    assert "canonical_final_inputs" in next_goal
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "wuxia_black_serpent_aftermath" in storypack_db_json
    assert "wuxia_final_epilogue_renderer_contract_handoff" in storypack_db_json
    assert "wuxia_final_epilogue_renderer_contract" in storypack_db_json
    assert '"id": "wuxia_seoharin_left_meal"' in situations_json
    assert '"surface": [' in situations_json
    assert '"food"' in situations_json
    assert '"daily_care"' in situations_json
    assert '"next_runtime_scope": "seoharin_daily_care_bridge"' in situations_json
    assert '"next_handoff": "wuxia_seoharin_left_meal_followup"' in situations_json
    assert "26. `wuxia_seoharin_left_meal` — preview runtime 구현 완료" in wuxia_pack
    assert "## 26. `wuxia_seoharin_left_meal` — preview runtime 구현 완료" in wuxia_cards
    assert "layout: left_meal_memory" in wuxia_cards
    assert "eat_the_left_meal_quietly" in wuxia_cards
    assert "thank_seoharin_for_the_bowl" in wuxia_cards
    assert "joke_about_who_ordered_extra_rice" in wuxia_cards
    assert "pass_without_eating_the_meal" in wuxia_cards
    assert "Latest implemented runtime: `wuxia_black_serpent_aftermath`" in runtime_preview
    assert "Next design/runtime gate: `wuxia_final_epilogue_renderer_contract` handoff" in runtime_preview
    assert "`wuxia_black_serpent_aftermath`까지 YAML/generated bundle에 반영" in world_model
    assert "left-meal daily-care bridge도 기존 encounter schema로 구현 완료" in encounter_model
    assert 'notion_event_id: "wuxia_seoharin_left_meal"' in notion_sources
    assert 'runtime_status: "implemented_in_storypack_preview"' in notion_sources
    assert "told_seoharin_truth" not in encounters_yaml


def test_wuxia_final_state_routing_contract_is_docs_synced():
    index = Path("docs/00_Index.md").read_text(encoding="utf-8")
    readme = Path("README.md").read_text(encoding="utf-8")
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    notion_sources = Path("idea_box/notion_sources.yml").read_text(encoding="utf-8")
    doc_path = Path("docs/design/Wuxia_Final_State_Routing.md")

    assert doc_path.exists()
    doc = doc_path.read_text(encoding="utf-8")
    assert "docs/design/Wuxia_Final_State_Routing.md" in index
    assert "docs/design/Wuxia_Final_State_Routing.md" in readme
    assert "Wuxia Final State Routing" in doc
    assert "canonical_final_inputs" in doc
    assert "combat_result" in doc
    assert "boss_resolution_route" in doc
    assert "evidence_state" in doc
    assert "network_handling" in doc
    assert "seoharin_axis" in doc
    assert "mumyeong_salvation" in doc
    assert "item_logs" in doc
    assert "item_unpriced_wooden_sword" in doc
    assert "final_result_priority" in doc
    assert "final_epilogue_master_matrix" in doc
    assert "state_alias_and_deprecation_policy" in doc
    assert "item_log_state" in doc
    assert "local_helper_only" in doc
    assert "wuxia_sado_final_phase_1_price_tag" in doc
    assert "no combat resolver" in doc
    assert "no HP numeric battle" in doc

    assert "## 0.56 2026-06-02 docs-only Seo Harin left-meal follow-up handoff: final state routing contract" in plan
    assert "### 0.2bt 2026-06-02 무협 Seo Harin left-meal follow-up docs-only handoff" in checklist
    assert "### 0.2bu 2026-06-02 무협 final state routing contract docs slice" in checklist
    assert "Seo Harin left-meal follow-up handoff" in coverage
    assert "최종장 결산 라우팅 마스터" in coverage
    assert "사도 최종전 상태값 사전" in coverage
    assert "Next design/runtime gate: `wuxia_final_epilogue_renderer_contract` handoff" in runtime_preview
    assert "Latest implemented runtime is `wuxia_black_serpent_aftermath`" in runtime_preview
    assert "wuxia_sado_final_phase_1_price_tag" in wuxia_pack
    assert "wuxia_sado_final_phase_1_price_tag" in wuxia_cards
    assert "wuxia_final_epilogue_renderer_contract" in storypack_db_json
    assert "current_goal: wuxia_final_epilogue_renderer_contract_implementation" in next_goal
    assert "previous_current_goal: wuxia_final_epilogue_renderer_contract_handoff" in next_goal
    assert "wuxia_final_routing_master" in notion_sources
    assert "37237e69-695e-81d2-8ce2-d1c738c3e923" in notion_sources
    assert "wuxia_sado_final_state_glossary" in notion_sources
    assert "37337e69-695e-81c7-a9fd-e0a0e22005e2" in notion_sources


def test_wuxia_sado_final_phase_1_price_tag_runtime_slice_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    notion_sources = Path("idea_box/notion_sources.yml").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    situations_json = Path("docs/content/storypack_db/encounter_situations.json").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    world_model = Path("docs/design/Storypack_World_Model.md").read_text(encoding="utf-8")
    encounter_model = Path("docs/design/Storypack_Encounter_DB.md").read_text(
        encoding="utf-8"
    )
    encounters_yaml = Path(
        "src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml"
    ).read_text(encoding="utf-8")

    assert "## 0.57 2026-06-02 무협 `wuxia_sado_final_phase_1_price_tag` preview runtime slice" in plan
    assert "### 0.2bv 2026-06-02 무협 `wuxia_sado_final_phase_1_price_tag` preview runtime slice" in checklist
    assert "Sado final phase 1 price-tag implementation" in coverage
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "wuxia_black_serpent_aftermath" in storypack_db_json
    assert "wuxia_final_epilogue_renderer_contract_handoff" in storypack_db_json
    assert "wuxia_final_epilogue_renderer_contract" in storypack_db_json
    assert "current_goal: wuxia_final_epilogue_renderer_contract_implementation" in next_goal
    assert "previous_current_goal: wuxia_final_epilogue_renderer_contract_handoff" in next_goal
    assert "wuxia_sado_final_phase_2_weakpoint_control" in next_goal
    assert 'key: wuxia_sado_final_phase_1_price_tag' in notion_sources
    assert 'runtime_status: "implemented_in_storypack_preview"' in notion_sources
    assert 'key: wuxia_sado_final_phase_2_weakpoint_control' in notion_sources
    assert 'runtime_status: "implemented_in_storypack_preview"' in notion_sources

    assert '"id": "wuxia_sado_final_phase_1_price_tag"' in situations_json
    assert '"status": "implemented_in_storypack_preview"' in situations_json
    assert '"runtime_preview_design_status": "implemented"' in situations_json
    assert '"insert_after": "wuxia_seoharin_left_meal"' in situations_json
    assert '"next_handoff": "wuxia_sado_final_phase_2_weakpoint_control_handoff"' in situations_json
    assert '"final_evidence_strong_seeded"' in situations_json
    assert '"item_blackscale_ledger_logged"' in situations_json
    assert "27. `wuxia_sado_final_phase_1_price_tag` — preview runtime 구현 완료" in wuxia_pack
    assert "## 27. `wuxia_sado_final_phase_1_price_tag` — preview runtime 구현 완료" in wuxia_cards
    assert "layout: final_phase_price_tag" in wuxia_cards
    assert "approach_sado_before_the_ledger" in wuxia_cards
    assert "burn_the_blackscale_ledger" in wuxia_cards
    assert "secure_the_blackscale_ledger" in wuxia_cards
    assert "ease_hostage_pressure_first" in wuxia_cards
    assert "Latest implemented runtime: `wuxia_black_serpent_aftermath`" in runtime_preview
    assert "Next design/runtime gate: `wuxia_final_epilogue_renderer_contract` handoff" in runtime_preview
    assert "`wuxia_black_serpent_aftermath`까지 YAML/generated bundle에 반영" in world_model
    assert "`wuxia_black_serpent_aftermath`도 기존 encounter schema로 구현 완료" in encounter_model
    assert "told_seoharin_truth" not in encounters_yaml
    assert "wuxia_sado_final_battle_started" not in encounters_yaml


def test_wuxia_sado_final_phase_3_outside_calculation_runtime_slice_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    notion_sources = Path("idea_box/notion_sources.yml").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    situations_json = Path("docs/content/storypack_db/encounter_situations.json").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    world_model = Path("docs/design/Storypack_World_Model.md").read_text(encoding="utf-8")
    encounter_model = Path("docs/design/Storypack_Encounter_DB.md").read_text(
        encoding="utf-8"
    )
    encounters_yaml = Path(
        "src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml"
    ).read_text(encoding="utf-8")

    assert "## 0.59 2026-06-02 무협 `wuxia_sado_final_phase_3_outside_calculation` preview runtime slice" in plan
    assert "## 0.60 2026-06-02 무협 `wuxia_boss_resolution` preview runtime slice" in plan
    assert "## 0.61 2026-06-02 무협 `wuxia_mumyeong_resolution` preview runtime slice" in plan
    assert "### 0.2bx 2026-06-02 무협 `wuxia_sado_final_phase_3_outside_calculation` preview runtime slice" in checklist
    assert "### 0.2by 2026-06-02 무협 `wuxia_boss_resolution` preview runtime slice" in checklist
    assert "### 0.2bz 2026-06-02 무협 `wuxia_mumyeong_resolution` preview runtime slice" in checklist
    assert "Sado final phase 3 outside-calculation implementation" in coverage
    assert "Boss resolution implementation" in coverage
    assert "Mumyeong resolution implementation" in coverage
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "wuxia_black_serpent_aftermath" in storypack_db_json
    assert "wuxia_final_epilogue_renderer_contract_handoff" in storypack_db_json
    assert "wuxia_final_epilogue_renderer_contract" in storypack_db_json
    assert "current_goal: wuxia_final_epilogue_renderer_contract_implementation" in next_goal
    assert "previous_current_goal: wuxia_final_epilogue_renderer_contract_handoff" in next_goal
    assert "phase 3 stable choice id는 `remember_the_empty_place`, `let_mumyeong_choose_own_flow`, `endure_with_qingliu_will`, `point_to_blank_in_ledger`, `answer_with_sado_calculation`" in next_goal
    assert "boss resolution stable choice id는 `confirm_true_route_outside_calculation`, `confirm_meaningful_victory_with_evidence`, `confirm_incomplete_victory_residue`, `confirm_mumyeong_unsaved_successor_risk`, `confirm_corrupted_victory`" in next_goal
    assert "mumyeong resolution stable choice id는 `ask_mumyeong_for_own_flow`, `reveal_boss_used_mumyeongs_wound`, `leave_room_for_unsent_apology`, `let_stolen_forms_end`, `confirm_black_serpent_successor_risk`, `judge_with_sado_style_calculation`" in next_goal
    assert 'key: wuxia_sado_final_phase_3_outside_calculation' in notion_sources
    assert 'runtime_status: "implemented_in_storypack_preview"' in notion_sources
    assert 'key: wuxia_boss_resolution' in notion_sources
    assert 'key: wuxia_mumyeong_resolution' in notion_sources
    assert 'runtime_status: "next_implementation_candidate"' in notion_sources

    assert '"id": "wuxia_sado_final_phase_3_outside_calculation"' in situations_json
    assert '"status": "implemented_in_storypack_preview"' in situations_json
    assert '"mapping_status": "preview_runtime_implemented"' in situations_json
    assert '"runtime_preview_design_status": "implemented"' in situations_json
    assert '"insert_after": "wuxia_sado_final_phase_2_weakpoint_control"' in situations_json
    assert '"next_handoff": "wuxia_boss_resolution_handoff"' in situations_json
    assert '"remember_the_empty_place"' in situations_json
    assert '"let_mumyeong_choose_own_flow"' in situations_json
    assert '"answer_with_sado_calculation"' in situations_json
    assert '"id": "wuxia_boss_resolution"' in situations_json
    assert '"insert_after": "wuxia_sado_final_phase_3_outside_calculation"' in situations_json
    assert '"next_handoff": "wuxia_mumyeong_resolution_handoff"' in situations_json
    assert '"confirm_true_route_outside_calculation"' in situations_json
    assert '"confirm_corrupted_victory"' in situations_json
    assert '"id": "wuxia_mumyeong_resolution"' in situations_json
    assert '"insert_after": "wuxia_boss_resolution"' in situations_json
    assert '"next_handoff": "wuxia_seoharin_qingliu_resolution_handoff"' in situations_json
    assert '"ask_mumyeong_for_own_flow"' in situations_json
    assert '"judge_with_sado_style_calculation"' in situations_json

    assert "29. `wuxia_sado_final_phase_3_outside_calculation` — preview runtime 구현 완료" in wuxia_pack
    assert "30. `wuxia_boss_resolution` — preview runtime 구현 완료" in wuxia_pack
    assert "31. `wuxia_mumyeong_resolution` — preview runtime 구현 완료" in wuxia_pack
    assert "## 29. `wuxia_sado_final_phase_3_outside_calculation` — preview runtime 구현 완료" in wuxia_cards
    assert "## 30. `wuxia_boss_resolution` — preview runtime 구현 완료" in wuxia_cards
    assert "## 31. `wuxia_mumyeong_resolution` — preview runtime 구현 완료" in wuxia_cards
    assert "layout: final_phase_outside_calculation" in wuxia_cards
    assert "stable_terms: [계산식, 서하린, 무명, 목검]" in wuxia_cards
    assert "required_flags: [sado_final_phase_2_weakpoint_control_resolved, final_phase_2_weakpoint_control_resolved, final_state_routing_seeded]" in wuxia_cards
    assert "remember_the_empty_place" in wuxia_cards
    assert "let_mumyeong_choose_own_flow" in wuxia_cards
    assert "endure_with_qingliu_will" in wuxia_cards
    assert "point_to_blank_in_ledger" in wuxia_cards
    assert "answer_with_sado_calculation" in wuxia_cards
    assert "final_boss_resolution_true_route_candidate_seeded" in wuxia_cards
    assert "final_boss_resolution_corrupted_candidate_seeded" in wuxia_cards
    assert "next_handoff: wuxia_boss_resolution_handoff" in wuxia_cards
    assert "layout: boss_resolution_seed" in wuxia_cards
    assert "stable_terms: [보스 결산, 흑사방, 무명, 무림맹]" in wuxia_cards
    assert "required_flags: [sado_final_phase_3_outside_calculation_resolved, final_phase_3_outside_calculation_resolved, final_combat_result_battle_victory_seeded, final_state_routing_seeded]" in wuxia_cards
    assert "confirm_true_route_outside_calculation" in wuxia_cards
    assert "confirm_meaningful_victory_with_evidence" in wuxia_cards
    assert "confirm_incomplete_victory_residue" in wuxia_cards
    assert "confirm_mumyeong_unsaved_successor_risk" in wuxia_cards
    assert "confirm_corrupted_victory" in wuxia_cards
    assert "final_boss_resolution_true_route_confirmed_seeded" in wuxia_cards
    assert "final_boss_resolution_corrupted_victory_seeded" in wuxia_cards
    assert "next_handoff: wuxia_mumyeong_resolution_handoff" in wuxia_cards
    assert "layout: mumyeong_resolution_seed" in wuxia_cards
    assert "stable_terms: [무명, 자기 흐름, 사과, 검은 뱀]" in wuxia_cards
    assert "required_flags: [boss_resolution_resolved, final_result_priority_applied_seeded, final_combat_result_battle_victory_seeded, final_state_routing_seeded]" in wuxia_cards
    assert "ask_mumyeong_for_own_flow" in wuxia_cards
    assert "reveal_boss_used_mumyeongs_wound" in wuxia_cards
    assert "leave_room_for_unsent_apology" in wuxia_cards
    assert "let_stolen_forms_end" in wuxia_cards
    assert "confirm_black_serpent_successor_risk" in wuxia_cards
    assert "judge_with_sado_style_calculation" in wuxia_cards
    assert "final_mumyeong_resolution_own_flow_salvation_seeded" in wuxia_cards
    assert "final_mumyeong_resolution_corrupted_unsaved_seeded" in wuxia_cards
    assert "next_handoff: wuxia_seoharin_qingliu_resolution_handoff" in wuxia_cards

    assert "Latest implemented runtime: `wuxia_black_serpent_aftermath`" in runtime_preview
    assert "Next design/runtime gate: `wuxia_final_epilogue_renderer_contract` handoff" in runtime_preview
    assert "`wuxia_black_serpent_aftermath`까지 YAML/generated bundle에 반영" in world_model
    assert "`wuxia_black_serpent_aftermath`도 기존 encounter schema로 구현 완료" in encounter_model
    assert "told_seoharin_truth" not in encounters_yaml
    assert "wuxia_sado_final_battle_started" not in encounters_yaml
    assert "item_unpriced_wooden_sword" not in encounters_yaml


def test_wuxia_seoharin_qingliu_resolution_runtime_slice_is_docs_synced():
    readme = Path("README.md").read_text(encoding="utf-8")
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    notion_sources = Path("idea_box/notion_sources.yml").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    situations_json = Path("docs/content/storypack_db/encounter_situations.json").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    world_model = Path("docs/design/Storypack_World_Model.md").read_text(encoding="utf-8")
    encounter_model = Path("docs/design/Storypack_Encounter_DB.md").read_text(
        encoding="utf-8"
    )
    encounters_yaml = Path(
        "src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml"
    ).read_text(encoding="utf-8")

    assert "기본 storypack: `wuxia_jianghu_pack` / **이구학지 — 천기록**" in readme
    assert "## 0.62 2026-06-02 무협 `wuxia_seoharin_qingliu_resolution` preview runtime slice" in plan
    assert "### 0.2ca 2026-06-02 무협 `wuxia_seoharin_qingliu_resolution` preview runtime slice" in checklist
    assert "Seo Harin/Qingliu resolution implementation" in coverage
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "wuxia_black_serpent_aftermath" in storypack_db_json
    assert "wuxia_final_epilogue_renderer_contract_handoff" in storypack_db_json
    assert "wuxia_final_epilogue_renderer_contract" in storypack_db_json
    assert "current_goal: wuxia_final_epilogue_renderer_contract_implementation" in next_goal
    assert "previous_current_goal: wuxia_final_epilogue_renderer_contract_handoff" in next_goal

    assert '"id": "wuxia_seoharin_qingliu_resolution"' in situations_json
    assert '"status": "implemented_in_storypack_preview"' in situations_json
    assert '"mapping_status": "preview_runtime_implemented"' in situations_json
    assert '"runtime_preview_design_status": "implemented"' in situations_json
    assert '"insert_after": "wuxia_mumyeong_resolution"' in situations_json
    assert '"next_handoff": "wuxia_cheongirok_resolution_handoff"' in situations_json
    assert '"leave_the_gate_unclosed"' in situations_json
    assert '"record_qingliu_rebuild_without_glory"' in situations_json
    assert '"keep_empty_place_for_return_or_absence"' in situations_json
    assert '"mark_qingliu_pressure_still_unresolved"' in situations_json
    assert '"close_the_gate_with_sado_logic"' in situations_json

    assert "32. `wuxia_seoharin_qingliu_resolution` — preview runtime 구현 완료" in wuxia_pack
    assert "## 32. `wuxia_seoharin_qingliu_resolution` — preview runtime 구현 완료" in wuxia_cards
    assert "layout: seoharin_qingliu_resolution_seed" in wuxia_cards
    assert "stable_terms: [서하린, 청류문, 산문, 목검]" in wuxia_cards
    assert "required_flags: [mumyeong_resolution_resolved, boss_resolution_resolved, final_result_priority_applied_seeded, final_combat_result_battle_victory_seeded, final_state_routing_seeded]" in wuxia_cards
    assert "final_epilogue_seoharin_open_gate_candidate_seeded" in wuxia_cards
    assert "open_gate_is_not_possession" in wuxia_cards
    assert "next_handoff: wuxia_cheongirok_resolution_handoff" in wuxia_cards

    assert "Latest implemented runtime: `wuxia_black_serpent_aftermath`" in runtime_preview
    assert "Next design/runtime gate: `wuxia_final_epilogue_renderer_contract` handoff" in runtime_preview
    assert "`wuxia_black_serpent_aftermath`까지 YAML/generated bundle에 반영" in world_model
    assert "`wuxia_black_serpent_aftermath`도 기존 encounter schema로 구현 완료" in encounter_model
    assert "key: wuxia_seoharin_qingliu_resolution" in notion_sources
    assert "key: wuxia_cheongirok_resolution" in notion_sources
    assert "runtime_status: \"next_implementation_candidate\"" in notion_sources

    assert "told_seoharin_truth" not in encounters_yaml
    assert "wuxia_sado_final_battle_started" not in encounters_yaml
    assert "item_unpriced_wooden_sword" not in encounters_yaml


def test_wuxia_cheongirok_resolution_runtime_slice_is_docs_synced():
    readme = Path("README.md").read_text(encoding="utf-8")
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    notion_sources = Path("idea_box/notion_sources.yml").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    situations_json = Path("docs/content/storypack_db/encounter_situations.json").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    world_model = Path("docs/design/Storypack_World_Model.md").read_text(encoding="utf-8")
    encounter_model = Path("docs/design/Storypack_Encounter_DB.md").read_text(
        encoding="utf-8"
    )
    encounters_yaml = Path(
        "src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml"
    ).read_text(encoding="utf-8")

    assert "기본 storypack: `wuxia_jianghu_pack` / **이구학지 — 천기록**" in readme
    assert "runtime은 arrival/first fight/first fragment부터 `wuxia_black_serpent_aftermath`까지 구현됐다" in readme
    assert "## 0.63 2026-06-02 무협 `wuxia_cheongirok_resolution` preview runtime slice" in plan
    assert "### 0.2cb 2026-06-02 무협 `wuxia_cheongirok_resolution` preview runtime slice" in checklist
    assert "Cheongirok resolution implementation" in coverage
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "wuxia_black_serpent_aftermath" in storypack_db_json
    assert "wuxia_final_epilogue_renderer_contract" in storypack_db_json
    assert "current_goal: wuxia_final_epilogue_renderer_contract_implementation" in next_goal
    assert "previous_current_goal: wuxia_final_epilogue_renderer_contract_handoff" in next_goal

    assert '"id": "wuxia_cheongirok_resolution"' in situations_json
    assert '"status": "implemented_in_storypack_preview"' in situations_json
    assert '"mapping_status": "preview_runtime_implemented"' in situations_json
    assert '"runtime_preview_design_status": "implemented"' in situations_json
    assert '"insert_after": "wuxia_seoharin_qingliu_resolution"' in situations_json
    assert '"next_handoff": "wuxia_black_serpent_aftermath_handoff"' in situations_json
    assert '"turn_the_last_page_without_question"' in situations_json
    assert '"leave_blank_as_unpriced_place"' in situations_json
    assert '"read_the_lines_that_align_like_ledger"' in situations_json
    assert '"close_record_before_it_becomes_answer"' in situations_json
    assert '"let_record_reflect_the_method"' in situations_json

    assert "33. `wuxia_cheongirok_resolution` — preview runtime 구현 완료" in wuxia_pack
    assert "## 33. `wuxia_cheongirok_resolution` — preview runtime 구현 완료" in wuxia_cards
    assert "layout: cheongirok_resolution_seed" in wuxia_cards
    assert "stable_terms: [천기록, 마지막 장, 빈칸, 기록자]" in wuxia_cards
    assert "required_flags: [seoharin_qingliu_resolution_resolved, mumyeong_resolution_resolved, boss_resolution_resolved, final_result_priority_applied_seeded, final_combat_result_battle_victory_seeded, final_state_routing_seeded]" in wuxia_cards
    assert "final_cheongirok_state_high_use_not_corruption_seeded" in wuxia_cards
    assert "final_epilogue_tianjilu_true_route_variant_seeded" in wuxia_cards
    assert "next_handoff: wuxia_black_serpent_aftermath_handoff" in wuxia_cards

    assert "Latest implemented runtime: `wuxia_black_serpent_aftermath`" in runtime_preview
    assert "Next design/runtime gate: `wuxia_final_epilogue_renderer_contract` handoff" in runtime_preview
    assert "`wuxia_black_serpent_aftermath`까지 YAML/generated bundle에 반영" in world_model
    assert "`wuxia_black_serpent_aftermath`도 기존 encounter schema로 구현 완료" in encounter_model
    assert "key: wuxia_cheongirok_resolution" in notion_sources
    assert "key: wuxia_black_serpent_aftermath" in notion_sources
    assert "runtime_status: \"implemented_in_storypack_preview\"" in notion_sources
    assert "runtime_status: \"next_implementation_candidate\"" in notion_sources

    assert "told_seoharin_truth" not in encounters_yaml
    assert "wuxia_sado_final_battle_started" not in encounters_yaml
    assert "item_unpriced_wooden_sword" not in encounters_yaml


def test_wuxia_black_serpent_aftermath_runtime_slice_is_docs_synced():
    readme = Path("README.md").read_text(encoding="utf-8")
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    notion_sources = Path("idea_box/notion_sources.yml").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    situations_json = Path("docs/content/storypack_db/encounter_situations.json").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    world_model = Path("docs/design/Storypack_World_Model.md").read_text(encoding="utf-8")
    encounter_model = Path("docs/design/Storypack_Encounter_DB.md").read_text(
        encoding="utf-8"
    )
    encounters_yaml = Path(
        "src/tui_adv/storypack-previews/wuxia_jianghu_pack/encounters.yaml"
    ).read_text(encoding="utf-8")

    assert "runtime은 arrival/first fight/first fragment부터 `wuxia_black_serpent_aftermath`까지 구현됐다" in readme
    assert "## 0.64 2026-06-02 무협 `wuxia_black_serpent_aftermath` preview runtime slice" in plan
    assert "### 0.2cc 2026-06-02 무협 `wuxia_black_serpent_aftermath` preview runtime slice" in checklist
    assert "Black Serpent aftermath implementation" in coverage
    assert "- `wuxia_jianghu_pack`: 이구학지 — 천기록 후보 카드 34개." in storypack_db_readme
    assert "wuxia_black_serpent_aftermath" in storypack_db_json
    assert "wuxia_final_epilogue_renderer_contract" in storypack_db_json
    assert "current_goal: wuxia_final_epilogue_renderer_contract_implementation" in next_goal
    assert "previous_current_goal: wuxia_final_epilogue_renderer_contract_handoff" in next_goal

    assert '"id": "wuxia_black_serpent_aftermath"' in situations_json
    assert '"insert_after": "wuxia_cheongirok_resolution"' in situations_json
    assert '"next_handoff": "wuxia_final_epilogue_renderer_contract_handoff"' in situations_json
    assert '"mark_broken_serpent_without_erasing_scars"' in situations_json
    assert '"fold_the_banner_without_calling_it_gone"' in situations_json
    assert '"send_ledger_to_alliance_and_watch_silence"' in situations_json
    assert '"listen_for_southern_market_debt_rumor"' in situations_json
    assert '"let_true_route_suppress_the_banner"' in situations_json

    assert "34. `wuxia_black_serpent_aftermath` — preview runtime 구현 완료" in wuxia_pack
    assert "## 34. `wuxia_black_serpent_aftermath` — preview runtime 구현 완료" in wuxia_cards
    assert "layout: black_serpent_aftermath_seed" in wuxia_cards
    assert "stable_terms: [흑사방, 장부, 깃발, 남쪽 장터]" in wuxia_cards
    assert "required_flags: [cheongirok_resolution_resolved, seoharin_qingliu_resolution_resolved, mumyeong_resolution_resolved, boss_resolution_resolved, final_result_priority_applied_seeded, final_combat_result_battle_victory_seeded, final_state_routing_seeded]" in wuxia_cards
    assert "final_epilogue_boss_broken_black_serpent_variant_ready_seeded" in wuxia_cards
    assert "final_southern_market_rumor_candidate_reinforced_seeded" in wuxia_cards
    assert "next_handoff: wuxia_final_epilogue_renderer_contract_handoff" in wuxia_cards

    assert "Latest implemented runtime: `wuxia_black_serpent_aftermath`" in runtime_preview
    assert "Next design/runtime gate: `wuxia_final_epilogue_renderer_contract` handoff" in runtime_preview
    assert "`wuxia_black_serpent_aftermath`까지 YAML/generated bundle에 반영" in world_model
    assert "`wuxia_black_serpent_aftermath`도 기존 encounter schema로 구현 완료" in encounter_model
    assert "key: wuxia_black_serpent_aftermath" in notion_sources
    assert "key: wuxia_final_epilogue_renderer_contract" in notion_sources
    assert "runtime_status: \"implemented_in_storypack_preview\"" in notion_sources
    assert "runtime_status: \"next_implementation_candidate\"" in notion_sources

    assert "told_seoharin_truth" not in encounters_yaml
    assert "wuxia_sado_final_battle_started" not in encounters_yaml
    assert "item_unpriced_wooden_sword" not in encounters_yaml


def test_wuxia_final_epilogue_runtime_contract_is_docs_synced():
    readme = Path("README.md").read_text(encoding="utf-8")
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    final_routing = Path("docs/design/Wuxia_Final_State_Routing.md").read_text(
        encoding="utf-8"
    )
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    notion_sources = Path("idea_box/notion_sources.yml").read_text(encoding="utf-8")
    storypack_db_readme = Path("docs/content/storypack_db/README.md").read_text(
        encoding="utf-8"
    )
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    endings_yaml = Path(
        "src/tui_adv/storypack-previews/wuxia_jianghu_pack/endings.yaml"
    ).read_text(encoding="utf-8")
    final_epilogue_rs = Path("crates/escape-core/src/final_epilogue.rs").read_text(
        encoding="utf-8"
    )

    assert "runtime은 arrival/first fight/first fragment부터 `wuxia_battle_loss_epilogue_contract`까지 구현됐다" in readme
    assert "## 0.66 2026-06-02 무협 `wuxia_final_epilogue_renderer_contract` runtime implementation slice" in plan
    assert "### 0.2ce 2026-06-02 무협 `wuxia_final_epilogue_renderer_contract` runtime implementation slice" in checklist
    assert "Final epilogue renderer implementation" in coverage
    assert "`wuxia_final_epilogue_renderer_contract`" in runtime_preview
    assert "`wuxia_final_epilogue_renderer_contract`" in final_routing
    assert "current_goal: wuxia_final_epilogue_ux_playtest_contract_followup" in next_goal
    assert "previous_current_goal: wuxia_final_epilogue_renderer_contract_implementation" in next_goal
    assert "runtime_status: \"implemented_in_storypack_preview\"" in notion_sources
    assert "previous_runtime_status: \"next_implementation_candidate\"" in notion_sources

    assert "runtime final ending/system contract" in storypack_db_readme
    assert "wuxia_final_epilogue_renderer_contract" in storypack_db_json
    assert "35. `wuxia_final_epilogue_renderer_contract` — runtime 구현 완료" in wuxia_pack
    assert "## 35. `wuxia_final_epilogue_renderer_contract` — runtime 구현 완료" in wuxia_cards
    assert "body_block_contract:" in wuxia_cards
    assert "block_kinds: [epilogue_result, epilogue_card, epilogue_suppressed, epilogue_contract_error]" in wuxia_cards

    assert "id: wuxia_final_epilogue_renderer_contract" in endings_yaml
    assert "kind: final_epilogue_contract" in endings_yaml
    assert "boss_resolution_resolved" in endings_yaml
    assert "black_serpent_aftermath_resolved" in endings_yaml

    assert "epilogue_result" in final_epilogue_rs
    assert "epilogue_card" in final_epilogue_rs
    assert "epilogue_suppressed" in final_epilogue_rs
    assert "final_result_key" in final_epilogue_rs


def test_wuxia_return_settlement_epilogue_contract_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(encoding="utf-8")
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    final_routing = Path("docs/design/Wuxia_Final_State_Routing.md").read_text(
        encoding="utf-8"
    )
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    situations_json = Path(
        "docs/content/storypack_db/encounter_situations.json"
    ).read_text(encoding="utf-8")
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    final_epilogue_rs = Path("crates/escape-core/src/final_epilogue.rs").read_text(
        encoding="utf-8"
    )

    assert "## 0.70 2026-06-02 무협 `wuxia_return_settlement_epilogue_contract` runtime slice" in plan
    assert "### 0.2ci 2026-06-02 무협 `wuxia_return_settlement_epilogue_contract` runtime slice" in checklist
    assert "return/settlement epilogue branch runtime implemented" in coverage
    assert "`wuxia_return_settlement_epilogue_contract`" in runtime_preview
    assert "return schema나 archive/save surface를 열지 않는다" in runtime_preview
    assert "| runtime id | `wuxia_return_settlement_epilogue_contract` |" in final_routing
    assert "return_settlement_epilogue_followup_handoff" in next_goal
    assert "wuxia_return_settlement_epilogue_contract_implementation" in next_goal
    assert "wuxia_return_settlement_epilogue_contract" in storypack_db_json
    assert "\"next_handoff\": \"return_settlement_epilogue_followup_handoff\"" in situations_json
    assert "37. `wuxia_return_settlement_epilogue_contract` — runtime 구현 완료" in wuxia_pack
    assert "## 38. `wuxia_return_settlement_epilogue_contract` — runtime 구현 완료" in wuxia_cards

    assert "epilogue_wuxia_returned_commute" in final_epilogue_rs
    assert "epilogue_wuxia_qingliu_settlement" in final_epilogue_rs
    assert "epilogue_wuxia_empty_place_kept_open" in final_epilogue_rs
    assert "epilogue_wuxia_closed_gate_risk" in final_epilogue_rs
    assert "return_settlement_evasion" in final_epilogue_rs
    assert "main_ending_type" not in final_epilogue_rs
    assert "told_seoharin_truth" not in final_epilogue_rs


def test_wuxia_battle_loss_epilogue_contract_handoff_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(
        encoding="utf-8"
    )
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    final_routing = Path("docs/design/Wuxia_Final_State_Routing.md").read_text(
        encoding="utf-8"
    )
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    readme = Path("README.md").read_text(encoding="utf-8")
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    final_epilogue_rs = Path("crates/escape-core/src/final_epilogue.rs").read_text(
        encoding="utf-8"
    )

    assert (
        "## 0.71 2026-06-02 무협 return_settlement_epilogue_followup_handoff docs-only handoff: battle-loss epilogue contract"
        in plan
    )
    assert (
        "### 0.2cj 2026-06-02 무협 return/settlement epilogue follow-up docs-only handoff"
        in checklist
    )
    assert "return_settlement_epilogue_followup_handoff" in coverage
    assert "wuxia_battle_loss_epilogue_contract" in runtime_preview
    assert "Decision from the 2026-06-02 `return_settlement_epilogue_followup_handoff`" in final_routing
    assert "Implementation status: `wuxia_battle_loss_epilogue_contract` is now implemented" in final_routing
    assert "current_goal: wuxia_battle_loss_epilogue_contract_implementation" in next_goal
    assert "previous_current_goal: return_settlement_epilogue_followup_handoff" in next_goal
    assert (
        "Latest handoff selected runtime: return_settlement_epilogue_followup_handoff selected wuxia_battle_loss_epilogue_contract"
        in storypack_db_json
    )
    assert "`wuxia_battle_loss_epilogue_contract`는 explicit battle-loss seed" in readme
    assert "38. `return_settlement_epilogue_followup_handoff` — docs-only handoff 완료" in wuxia_pack
    assert "## 39. `return_settlement_epilogue_followup_handoff` — docs-only handoff 완료" in wuxia_cards

    assert "FinalResult::BattleLoss" in final_epilogue_rs
    assert "final_combat_result_battle_loss_seeded" in final_epilogue_rs
    assert "epilogue_boss_black_serpent_banner" in final_epilogue_rs
    assert "epilogue_wuxia_southern_market_rumor" in final_epilogue_rs
    assert "epilogue_mumyeong_black_serpent_new_scale" in final_epilogue_rs
    assert "epilogue_seoharin_closed_gate" in final_epilogue_rs
    assert "epilogue_tianjilu_last_page" in final_epilogue_rs
    assert "wuxia_sado_final_battle" not in final_epilogue_rs
    assert "battle_loss_hp" not in final_epilogue_rs


def test_wuxia_battle_loss_epilogue_contract_runtime_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(
        encoding="utf-8"
    )
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    final_routing = Path("docs/design/Wuxia_Final_State_Routing.md").read_text(
        encoding="utf-8"
    )
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    readme = Path("README.md").read_text(encoding="utf-8")
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    endings_yaml = Path(
        "src/tui_adv/storypack-previews/wuxia_jianghu_pack/endings.yaml"
    ).read_text(encoding="utf-8")
    final_epilogue_rs = Path("crates/escape-core/src/final_epilogue.rs").read_text(
        encoding="utf-8"
    )
    route_parity_rs = Path("crates/escape-core/tests/route_parity.rs").read_text(
        encoding="utf-8"
    )
    wasm_json_contract_rs = Path(
        "crates/escape-wasm/tests/json_contract.rs"
    ).read_text(encoding="utf-8")

    assert "## 0.72 2026-06-02 무협 `wuxia_battle_loss_epilogue_contract` runtime slice" in plan
    assert "### 0.2ck 2026-06-02 무협 `wuxia_battle_loss_epilogue_contract` runtime slice" in checklist
    assert "battle-loss epilogue runtime implemented" in coverage
    assert "Latest implemented runtime: `wuxia_battle_loss_epilogue_contract`" in runtime_preview
    assert "Latest implemented runtime slice: `wuxia_battle_loss_epilogue_contract`" in final_routing
    assert "current_goal: wuxia_battle_loss_epilogue_followup_handoff" in next_goal
    assert "previous_current_goal: wuxia_battle_loss_epilogue_contract_implementation" in next_goal
    assert "Latest implemented runtime: wuxia_battle_loss_epilogue_contract" in storypack_db_json
    assert "`wuxia_battle_loss_epilogue_contract`는 explicit battle-loss seed" in readme
    assert "39. `wuxia_battle_loss_epilogue_contract` — runtime 구현 완료" in wuxia_pack
    assert "## 40. `wuxia_battle_loss_epilogue_contract` — runtime 구현 완료" in wuxia_cards

    assert (
        "required_flags: [boss_resolution_resolved, mumyeong_resolution_resolved, "
        "seoharin_qingliu_resolution_resolved, cheongirok_resolution_resolved, "
        "black_serpent_aftermath_resolved, final_result_priority_applied_seeded, "
        "final_state_routing_seeded]"
    ) in endings_yaml

    assert "FinalResult::BattleLoss" in final_epilogue_rs
    assert "final_combat_result_battle_loss_seeded" in final_epilogue_rs
    assert "battle_loss_residue" in final_epilogue_rs
    assert "battle_loss_successor_pressure" in final_epilogue_rs
    assert "battle_loss_or_corruption" in final_epilogue_rs
    assert "epilogue_boss_black_serpent_banner" in final_epilogue_rs
    assert "epilogue_wuxia_southern_market_rumor" in final_epilogue_rs
    assert "epilogue_mumyeong_black_serpent_new_scale" in final_epilogue_rs
    assert "epilogue_seoharin_closed_gate" in final_epilogue_rs
    assert "epilogue_tianjilu_last_page" in final_epilogue_rs
    assert "epilogue_boss_broken_black_serpent" in final_epilogue_rs
    assert "epilogue_seoharin_open_gate" in final_epilogue_rs
    assert "epilogue_mumyeong_stolen_forms_stopped" in final_epilogue_rs

    assert (
        "wuxia_final_epilogue_battle_loss_outputs_loss_bundle_and_suppresses_optimistic_cards"
        in route_parity_rs
    )
    assert (
        "json_boundary_outputs_wuxia_battle_loss_epilogue_bundle"
        in wasm_json_contract_rs
    )


def test_wuxia_battle_loss_epilogue_followup_handoff_is_docs_synced():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    coverage = Path("docs/dev/Notion_Design_Coverage.md").read_text(
        encoding="utf-8"
    )
    runtime_preview = Path("docs/dev/Storypack_Runtime_Preview_Mode.md").read_text(
        encoding="utf-8"
    )
    final_routing = Path("docs/design/Wuxia_Final_State_Routing.md").read_text(
        encoding="utf-8"
    )
    next_goal = Path("idea_box/next_goal/README.md").read_text(encoding="utf-8")
    storypack_db_json = Path("docs/content/storypack_db/storypacks.json").read_text(
        encoding="utf-8"
    )
    readme = Path("README.md").read_text(encoding="utf-8")
    wuxia_pack = Path("docs/content/storypacks/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )
    wuxia_cards = Path("docs/content/encounter_db/wuxia_jianghu_pack.md").read_text(
        encoding="utf-8"
    )

    assert (
        "## 0.73 2026-06-02 무협 `wuxia_battle_loss_epilogue_followup_handoff` docs-only handoff: final-state canonical collapse"
        in plan
    )
    assert (
        "### 0.2cl 2026-06-02 무협 `wuxia_battle_loss_epilogue_followup_handoff` docs-only handoff"
        in checklist
    )
    assert "final-state canonical collapse handoff selected" in coverage
    assert "wuxia_final_state_canonical_collapse_contract" in runtime_preview
    assert "## Battle Loss Epilogue Follow-up Handoff" in final_routing
    assert (
        "Next runtime implementation candidate: `wuxia_final_state_canonical_collapse_contract`"
        in final_routing
    )
    assert (
        "current_goal: wuxia_final_state_canonical_collapse_contract_implementation"
        in next_goal
    )
    assert (
        "previous_current_goal: wuxia_battle_loss_epilogue_followup_handoff"
        in next_goal
    )
    assert (
        "Latest handoff: wuxia_battle_loss_epilogue_followup_handoff selected wuxia_final_state_canonical_collapse_contract"
        in storypack_db_json
    )
    assert (
        "Latest handoff: `wuxia_battle_loss_epilogue_followup_handoff` selected `wuxia_final_state_canonical_collapse_contract`"
        in readme
    )
    assert (
        "40. `wuxia_battle_loss_epilogue_followup_handoff` — docs-only handoff 완료"
        in wuxia_pack
    )
    assert (
        "## 41. `wuxia_battle_loss_epilogue_followup_handoff` — docs-only handoff 완료"
        in wuxia_cards
    )

    for doc in (plan, final_routing, next_goal, wuxia_pack, wuxia_cards):
        assert "epilogue_state_audit" in doc
        assert "final_*_seeded" in doc
        assert "combat_result" in doc
        assert "boss_resolution_route" in doc
        assert "cheongirok_state" in doc
        assert "player_method" in doc

    assert "playable defeat-route bridge" in final_routing
    assert "no_combat_resolver" in wuxia_cards

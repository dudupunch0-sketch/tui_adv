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
    assert "Web/Tauri/Electron 패키징 검토" in plan.split("현재 최우선 남은 작업:", 1)[1]
    assert "visual card/GlyphFX/input 안내 polish" in readme
    assert "printer_anomaly stable terms를 terminal visual card 안에 보존" in architecture


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


def test_phase6_textual_style_slice_is_checklisted_and_documented():
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    layout_doc = Path("docs/dev/TUI_Layout.md").read_text(encoding="utf-8")

    assert "- [x] 분리된 Textual 위젯/그리드 패널 스타일링" in checklist
    assert "- [x] Textual CSS 색상 테마 위젯 연결" in checklist
    assert "#game-grid" in layout_doc
    assert "office-panel--wide" in layout_doc


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
    assert "- [ ]" not in checklist


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

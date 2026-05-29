import json
from pathlib import Path


def test_player_distribution_surface_is_web_only_and_documented():
    decision_path = Path("docs/dev/Web_Distribution_Decision.md")
    assert decision_path.exists()

    decision = decision_path.read_text(encoding="utf-8")
    index = Path("docs/00_Index.md").read_text(encoding="utf-8")
    readme = Path("README.md").read_text(encoding="utf-8")
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")

    assert "현재 배포 표면: Web-only" in decision
    assert "Tauri/Electron: deferred" in decision
    assert "Rust/WASM-primary" in decision
    assert "web/dist/" in decision
    assert "npm run build:player" in decision
    assert "npm run preview:player" in decision
    assert "docs/dev/Web_Distribution_Decision.md" in index
    assert "docs/dev/Web_Distribution_Decision.md" in readme
    assert "Web/Tauri/Electron 패키징 결정 완료" in plan
    assert "terminal full-screen app loop/tick/raw-draw GlyphFX 완료" in plan
    assert "현재 구현 우선순위는 `docs/dev/Development_Plan.md`를 따른다" in decision
    assert "Web Storybook transition/audio readiness" not in decision
    assert "- [x] Web/Tauri/Electron 패키징 검토와 Web-only 배포 표면 결정" in checklist


def test_web_package_has_player_build_aliases_without_desktop_runtime_dependencies():
    package = json.loads(Path("web/package.json").read_text(encoding="utf-8"))
    scripts = package["scripts"]

    assert scripts["build:player"] == "npm run build:wasm"
    assert scripts["preview:player"] == "npm run preview:wasm"

    serialized = json.dumps(package, ensure_ascii=False).lower()
    assert "tauri" not in serialized
    assert "electron" not in serialized

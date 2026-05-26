import json
from pathlib import Path


REQUIRED_VIEWPORTS = ["390x844", "414x896", "800x1440", "810x1644", "1440x1000"]


def test_web_package_exposes_storybook_visual_qa_script_and_dependency():
    package = json.loads(Path("web/package.json").read_text(encoding="utf-8"))
    scripts = package["scripts"]
    dev_dependencies = package["devDependencies"]

    assert scripts["qa:storybook:visual"] == "node scripts/storybook-reference-qa.mjs"
    assert "playwright-chromium" in dev_dependencies


def test_storybook_visual_qa_script_contract_is_static_and_scratch_only():
    script_path = Path("web/scripts/storybook-reference-qa.mjs")
    assert script_path.exists()
    script = script_path.read_text(encoding="utf-8")

    assert "playwright-chromium" in script
    assert "--base-url" in script
    assert "--out-dir" in script
    assert "--require-wasm" in script
    assert "visual-qa-report.json" in script
    assert "screenshots" in script
    assert "document.fonts.ready" in script
    assert "reducedMotion" in script
    assert "deviceScaleFactor" in script
    assert "localStorage.clear" in script
    assert "storybook-runtime-warning" in script
    assert "escape_wasm.js" in script
    assert "escape_wasm_bg.wasm" in script
    assert "/home/" not in script
    assert "golden" not in script.lower()

    for viewport in REQUIRED_VIEWPORTS:
        assert viewport in script


def test_storybook_visual_qa_script_names_required_dom_and_interaction_contracts():
    script = Path("web/scripts/storybook-reference-qa.mjs").read_text(encoding="utf-8")

    required_fragments = [
        '[data-player-screen="start"]',
        'data-player-action="new-game"',
        '[data-renderer="web-storybook"]',
        '.storybook-shell',
        '.storybook-hud[data-region="status"]',
        '.story-progress-rail',
        '[data-region="visual"]',
        '[data-region="body"]',
        '[data-region="choices"]',
        '[data-region="history"]',
        '.storybook-dock',
        'button.choice-row[data-action-id]',
        '.choice-bullet',
        '.fake-tui',
        '.storybook-topline',
        'CURRENT ENCOUNTER',
        'LOCAL STATUS',
        'scrollWidth',
        'keyboard.press',
    ]
    for fragment in required_fragments:
        assert fragment in script


def test_storybook_visual_qa_documentation_tracks_tmp_cache_and_report_policy():
    design_doc = Path("docs/design/Mobile_Pixel_Storybook_UI.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")

    for text in (design_doc, plan):
        assert "qa:storybook:visual" in text
        assert "PLAYWRIGHT_BROWSERS_PATH" in text
        assert "/tmp/dudupunch0-tui-adv/storybook-visual-qa" in text
        assert "visual-qa-report.json" in text
        assert "golden" in text.lower()

    assert "Web Storybook visual regression 자동화 완료" in plan
    assert "- [x] `web/scripts/storybook-reference-qa.mjs` Playwright viewport runner 추가" in checklist
    assert "- [x] package script로 visual QA command 노출" in checklist
    assert "- [x] reference viewport DOM/layout/interaction contract 자동 검증" in checklist
    assert "- [x] optional `--require-wasm` Rust/WASM-primary resource load smoke 추가" in checklist
    assert "- [x] screenshots/JSON report를 scratch output에만 남기도록 문서화" in checklist
    assert "- [x] visual QA contract/docs tests 추가" in checklist

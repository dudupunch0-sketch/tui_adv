import json
from pathlib import Path


def test_web_package_exposes_wasm_primary_build_and_preview_scripts():
    package = json.loads(Path("web/package.json").read_text(encoding="utf-8"))
    scripts = package["scripts"]

    assert scripts["wasm:build"] == (
        "wasm-pack build ../crates/escape-wasm --target web "
        "--out-dir ../../web/src/core/wasm-pkg"
    )
    assert scripts["build:wasm"] == "npm run wasm:build && npm run build"
    assert scripts["dev:wasm"] == "npm run wasm:build && vite --host 127.0.0.1"
    assert scripts["preview:wasm"] == "npm run build:wasm && vite preview --host 127.0.0.1"


def test_generated_wasm_package_is_ignored_but_documented():
    gitignore = Path(".gitignore").read_text(encoding="utf-8")
    readme = Path("README.md").read_text(encoding="utf-8")
    architecture = Path("docs/dev/Rust_Core_Dual_Renderer_Architecture.md").read_text(encoding="utf-8")

    assert "web/src/core/wasm-pkg/" in gitignore
    assert "npm run wasm:build" in readme
    assert "npm run build:wasm" in readme
    assert "npm run preview:wasm" in readme
    assert "Rust/WASM-primary" in readme
    assert "legacy TypeScript mirror" in readme
    assert "fallback/parity oracle" in readme
    assert "wasm-pack build ../crates/escape-wasm --target web --out-dir ../../web/src/core/wasm-pkg" in architecture


def test_main_plan_marks_wasm_standardization_complete_and_names_next_slice():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")

    assert "Web WASM build/preview 표준화 완료" in plan
    assert "legacy TypeScript mirror는 fallback/parity oracle로 freeze" in plan
    assert "현재 최우선 남은 작업:" in plan
    assert "SuperLightTUI terminal visual card/GlyphFX/input polish" in plan
    assert "- [x] Web WASM build/preview 절차 표준화" in checklist
    assert "- [x] legacy Python/Textual/TypeScript mirror freeze 범위 결정" in checklist

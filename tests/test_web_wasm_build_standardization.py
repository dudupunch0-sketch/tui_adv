import json
import shutil
import subprocess
from pathlib import Path

import pytest


def test_web_package_exposes_wasm_primary_build_and_preview_scripts():
    package = json.loads(Path("web/package.json").read_text(encoding="utf-8"))
    scripts = package["scripts"]

    assert scripts["wasm:build"] == (
        "wasm-pack build ../crates/escape-wasm --target web "
        "--out-dir ../../web/src/core/wasm-pkg"
    )
    assert scripts["build:wasm"] == "npm run wasm:build && npm run build && npm run wasm:copy"
    assert scripts["wasm:copy"] == "node scripts/copy-wasm-pkg.mjs"
    assert scripts["dev:wasm"] == "npm run wasm:build && vite --host 127.0.0.1"
    assert scripts["preview:wasm"] == "npm run build:wasm && vite preview --host 127.0.0.1"
    assert "wasm:copy" in scripts
    copy_script = Path("web/scripts/copy-wasm-pkg.mjs")
    assert copy_script.exists()
    copy_script_text = copy_script.read_text(encoding="utf-8")
    assert "assets" in copy_script_text
    assert "wasm-pkg" in copy_script_text
    assert "resolveSafeOutDir" in copy_script_text
    assert "assertNoSymlinkComponents" in copy_script_text


def test_wasm_copy_script_copies_generated_package_to_default_dist(tmp_path):
    web_root = make_wasm_copy_fixture(tmp_path)

    result = run_wasm_copy_script(web_root)

    assert result.returncode == 0, result.stderr
    target = web_root / "dist" / "assets" / "wasm-pkg"
    assert (target / "escape_wasm.js").read_text(encoding="utf-8") == "export default function init() {}\n"
    assert (target / "escape_wasm_bg.wasm").read_bytes() == b"\0asm"


def test_wasm_copy_script_rejects_absolute_out_dir(tmp_path):
    web_root = make_wasm_copy_fixture(tmp_path)
    outside = tmp_path / "outside"

    result = run_wasm_copy_script(web_root, "--out-dir", str(outside))

    assert result.returncode != 0
    assert "--out-dir must be relative" in result.stderr
    assert not (outside / "assets").exists()


def test_wasm_copy_script_rejects_parent_traversal_out_dir(tmp_path):
    web_root = make_wasm_copy_fixture(tmp_path)
    outside = tmp_path / "outside"

    result = run_wasm_copy_script(web_root, "--out-dir", "../outside")

    assert result.returncode != 0
    assert "parent-directory traversal" in result.stderr
    assert not outside.exists()


def test_wasm_copy_script_rejects_symlinked_output_parent(tmp_path):
    web_root = make_wasm_copy_fixture(tmp_path)
    outside = tmp_path / "outside"
    outside.mkdir()
    (web_root / "dist").symlink_to(outside, target_is_directory=True)

    result = run_wasm_copy_script(web_root)

    assert result.returncode != 0
    assert "symlink component" in result.stderr
    assert not (outside / "assets" / "wasm-pkg").exists()


def test_wasm_copy_script_rejects_symlinked_final_target(tmp_path):
    web_root = make_wasm_copy_fixture(tmp_path)
    outside = tmp_path / "outside"
    outside.mkdir()
    target_parent = web_root / "dist" / "assets"
    target_parent.mkdir(parents=True)
    (target_parent / "wasm-pkg").symlink_to(outside, target_is_directory=True)

    result = run_wasm_copy_script(web_root)

    assert result.returncode != 0
    assert "symlink component" in result.stderr
    assert not (outside / "escape_wasm.js").exists()


def make_wasm_copy_fixture(tmp_path):
    web_root = tmp_path / "web"
    script_dir = web_root / "scripts"
    script_dir.mkdir(parents=True)
    shutil.copy2(Path("web/scripts/copy-wasm-pkg.mjs"), script_dir / "copy-wasm-pkg.mjs")
    source = web_root / "src" / "core" / "wasm-pkg"
    source.mkdir(parents=True)
    (source / "escape_wasm.js").write_text("export default function init() {}\n", encoding="utf-8")
    (source / "escape_wasm_bg.wasm").write_bytes(b"\0asm")
    return web_root


def run_wasm_copy_script(web_root, *args):
    node = shutil.which("node")
    if node is None:
        pytest.skip("node is required for wasm copy script regression tests")
    return subprocess.run(
        [node, "scripts/copy-wasm-pkg.mjs", *args],
        cwd=web_root,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )


def test_generated_wasm_package_is_ignored_but_documented():
    gitignore = Path(".gitignore").read_text(encoding="utf-8")
    readme = Path("README.md").read_text(encoding="utf-8")
    architecture = Path("docs/dev/Rust_Core_Dual_Renderer_Architecture.md").read_text(encoding="utf-8")

    assert "web/src/core/wasm-pkg/" in gitignore
    assert "npm run wasm:build" in readme
    assert "npm run build:wasm" in readme
    assert "npm run preview:wasm" in readme
    assert "web/dist/assets/wasm-pkg/" in readme
    assert "Rust/WASM-primary" in readme
    # §0.88 Rust core 정본화: TS mirror/Python game 로직은 삭제됨(과거 fallback/parity oracle).
    assert "TypeScript mirror core" in readme
    assert "fallback/parity oracle" in readme
    assert "wasm-pack build ../crates/escape-wasm --target web --out-dir ../../web/src/core/wasm-pkg" in architecture
    assert "web/dist/assets/wasm-pkg/" in architecture


def test_main_plan_marks_wasm_standardization_complete_and_names_next_slice():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")

    assert "Web WASM build/preview 표준화 완료" in plan
    assert "legacy TypeScript mirror는 fallback/parity oracle로 freeze" in plan
    assert "현재 최우선 남은 작업:" in plan
    assert "SuperLightTUI terminal visual card/GlyphFX/input polish" in plan
    assert "- [x] Web WASM build/preview 절차 표준화" in checklist
    assert "- [x] `build:wasm`이 generated wasm package를 `web/dist/assets/wasm-pkg/`로 복사" in checklist
    assert "- [x] legacy Python/Textual/TypeScript mirror freeze 범위 결정" in checklist

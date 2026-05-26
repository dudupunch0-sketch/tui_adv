from pathlib import Path


def test_vite_config_supports_pages_base_path_env():
    vite_config = Path("web/vite.config.ts").read_text(encoding="utf-8")

    assert "base: process.env.VITE_BASE_PATH ?? '/'" in vite_config
    assert "outDir: 'dist'" in vite_config
    assert "sourcemap: true" in vite_config


def test_wasm_runtime_uses_import_meta_relative_module_url():
    runtime = Path("web/src/core/wasmRuntime.ts").read_text(encoding="utf-8")

    assert "new URL(" in runtime
    assert "'./wasm-pkg/escape_wasm.js'" in runtime
    assert "import.meta.url" in runtime
    assert ".toString()" in runtime
    assert "const DEFAULT_WASM_MODULE_PATH = './wasm-pkg/escape_wasm.js'" not in runtime


def test_web_main_has_require_wasm_fatal_player_policy_without_legacy_fallback():
    main_ts = Path("web/src/main.ts").read_text(encoding="utf-8")

    assert "import.meta.env.VITE_REQUIRE_WASM === 'true'" in main_ts
    assert "renderFatalPlayerError" in main_ts
    assert "storybook-runtime-error" in main_ts
    assert "게임 코어를 불러오지 못했습니다" in main_ts
    assert "fatalPlayerError" in main_ts

    catch_block = main_ts.split("} catch (error) {", 1)[1].split("}\n}", 1)[0]
    assert "if (REQUIRE_WASM)" in catch_block
    assert "renderFatalPlayerError" in catch_block
    assert "legacy mirror로 임시 실행 중" in catch_block


def test_github_pages_workflow_installs_python_project_dependencies_before_export_check():
    workflow_path = Path(".github/workflows/pages.yml")
    assert workflow_path.exists()
    workflow = workflow_path.read_text(encoding="utf-8")

    install_step_index = workflow.index("python -m pip install -e .")
    export_check_index = workflow.index("name: Check generated public content bundle")

    assert install_step_index < export_check_index


def test_github_pages_workflow_builds_wasm_required_player_and_runs_visual_qa():
    workflow_path = Path(".github/workflows/pages.yml")
    assert workflow_path.exists()
    workflow = workflow_path.read_text(encoding="utf-8")

    required_fragments = [
        "name: Deploy Web Player",
        "branches: [main]",
        "permissions:",
        "pages: write",
        "id-token: write",
        "actions/setup-node@v4",
        "dtolnay/rust-toolchain@stable",
        "targets: wasm32-unknown-unknown",
        "cargo install wasm-pack --locked",
        "python scripts/export_web_data.py",
        "--bundle crates/escape-core/fixtures/content/content.bundle.json",
        "--bundle web/src/data/generated/content.bundle.json",
        "cargo test --workspace",
        "npm ci",
        "npm test",
        "VITE_BASE_PATH: /tui_adv/",
        'VITE_REQUIRE_WASM: "true"',
        "npm run build:player",
        "--base /tui_adv/",
        "http://127.0.0.1:4173/tui_adv/",
        "PLAYWRIGHT_BROWSERS_PATH: /tmp/tui-adv/ms-playwright",
        "npm run qa:storybook:visual --",
        "--require-wasm",
        "actions/configure-pages@v5",
        "actions/upload-pages-artifact@v3",
        "path: web/dist",
        "actions/deploy-pages@v4",
    ]
    for fragment in required_fragments:
        assert fragment in workflow


def test_docs_and_checklist_track_web_player_deployment_readiness_slice():
    plan = Path("docs/dev/Development_Plan.md").read_text(encoding="utf-8")
    checklist = Path("docs/dev/Checklist.md").read_text(encoding="utf-8")
    web_plan = Path("docs/dev/Web_Player_PokeRogue_Style_Plan.md").read_text(encoding="utf-8")

    assert "Web player deployment readiness 완료" in plan
    assert "VITE_BASE_PATH" in plan
    assert "VITE_REQUIRE_WASM" in plan
    assert "GitHub Pages" in plan
    assert "Web Storybook transition/audio readiness" in plan
    assert "PR A — settings + motion foundation" in plan

    assert "- [x] `VITE_BASE_PATH` 기반 Vite base path 설정" in checklist
    assert "- [x] WASM module path를 `import.meta.url` 기준으로 하드닝" in checklist
    assert "- [x] `VITE_REQUIRE_WASM=true` production fatal policy 추가" in checklist
    assert "- [x] GitHub Pages deploy workflow 추가" in checklist
    assert "- [x] Web player deployment contract/docs tests 추가" in checklist
    assert "- [x] Web player start screen 추가" in checklist
    assert "- [x] 이어하기/새 게임/seed 표시/save timestamp UX 추가" in checklist
    assert "- [x] 새 게임 전 저장 reset confirmation 추가" in checklist
    assert "- [x] start/save UX contract tests 추가" in checklist

    assert "PR 1 — Web player deployment readiness" in web_plan
    assert "Status: implemented" in web_plan

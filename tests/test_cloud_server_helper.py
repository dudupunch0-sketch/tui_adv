import subprocess
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]


def test_cloud_server_launcher_uses_rust_content_play_loop():
    script = (REPO_ROOT / "cloud_server_only.sh").read_text(encoding="utf-8")

    assert "-m tui_adv --play" not in script
    assert 'exec "$BIN_PATH" --scene content' in script
    assert '--content-bundle "$BUNDLE_PATH" --play' in script
    assert "choice:check_message" in script
    assert "선택 실행: 메시지를 확인한다" in script


def test_cloud_server_help_is_focused_usage_text():
    result = subprocess.run(
        ["./cloud_server_only.sh", "help"],
        cwd=REPO_ROOT,
        text=True,
        capture_output=True,
        check=False,
    )

    assert result.returncode == 0
    assert "Usage from project root:" in result.stdout
    assert "launch Rust content-backed direct interactive play" in result.stdout
    assert "#!/usr/bin/env bash" not in result.stdout
    assert "set -euo pipefail" not in result.stdout

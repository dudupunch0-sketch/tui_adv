from __future__ import annotations

import json
import subprocess
import sys
from importlib.util import module_from_spec, spec_from_file_location
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCRIPT_PATH = ROOT / "scripts" / "export_web_data.py"


def _load_export_module():
    spec = spec_from_file_location("export_web_data", SCRIPT_PATH)
    assert spec is not None and spec.loader is not None
    module = module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def test_export_web_data_builds_public_manifest_with_expected_counts():
    exporter = _load_export_module()

    bundle = exporter.build_web_data(ROOT)

    assert bundle["manifest"]["schema_version"] == 1
    assert bundle["manifest"]["counts"] == {
        "locations": 16,
        "items": 13,
        "encounters": 20,
        "endings": 13,
        "achievements": 11,
        "secrets": 3,
    }
    assert bundle["locations"][0]["id"] == "dev_desk"
    assert bundle["encounters"][0]["id"] == "ex_employee_messenger"
    assert bundle["secrets"][0]["id"] == "real_note_001"


def test_export_web_data_writes_generated_json_files(tmp_path):
    exporter = _load_export_module()
    out_dir = tmp_path / "generated"

    written = exporter.write_web_data(ROOT, out_dir)

    assert sorted(path.name for path in written) == [
        "achievements.json",
        "encounters.json",
        "endings.json",
        "items.json",
        "locations.json",
        "manifest.json",
        "secrets.example.json",
    ]
    manifest = json.loads((out_dir / "manifest.json").read_text(encoding="utf-8"))
    assert manifest["counts"]["encounters"] == 20
    secrets = json.loads((out_dir / "secrets.example.json").read_text(encoding="utf-8"))
    assert secrets[0]["final_hint_policy"] == "private_only"


def test_export_web_data_check_detects_stale_generated_files(tmp_path):
    exporter = _load_export_module()
    out_dir = tmp_path / "generated"

    errors = exporter.check_web_data(ROOT, out_dir)

    assert errors
    assert "missing generated file" in errors[0]


def test_export_web_data_cli_write_and_check_roundtrip(tmp_path):
    out_dir = tmp_path / "generated"

    write_result = subprocess.run(
        [
            sys.executable,
            str(SCRIPT_PATH),
            "--root",
            str(ROOT),
            "--out-dir",
            str(out_dir),
            "--write",
        ],
        check=False,
        text=True,
        capture_output=True,
    )
    assert write_result.returncode == 0, write_result.stderr
    assert "wrote 7 web data files" in write_result.stdout

    check_result = subprocess.run(
        [
            sys.executable,
            str(SCRIPT_PATH),
            "--root",
            str(ROOT),
            "--out-dir",
            str(out_dir),
            "--check",
        ],
        check=False,
        text=True,
        capture_output=True,
    )
    assert check_result.returncode == 0, check_result.stdout + check_result.stderr
    assert "web data is up to date" in check_result.stdout


def test_export_web_data_refuses_public_secret_final_hint(tmp_path):
    root = tmp_path / "repo"
    data_dir = root / "src" / "tui_adv" / "data"
    data_dir.mkdir(parents=True)
    for name, key in [
        ("locations", "locations"),
        ("items", "items"),
        ("encounters", "encounters"),
        ("endings", "endings"),
        ("achievements", "achievements"),
    ]:
        (data_dir / f"{name}.yaml").write_text(f"{key}: []\n", encoding="utf-8")
    (data_dir / "secrets.example.yaml").write_text(
        """
secrets:
  - id: unsafe
    title: unsafe
    final_hint: do not publish
""".strip()
        + "\n",
        encoding="utf-8",
    )
    exporter = _load_export_module()

    try:
        exporter.build_web_data(root)
    except ValueError as exc:
        assert "public secret unsafe has private-only field: final_hint" in str(exc)
    else:
        raise AssertionError("expected public final_hint to be rejected")

from __future__ import annotations

import json
import subprocess
import sys
from importlib.util import module_from_spec, spec_from_file_location
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCRIPT_PATH = ROOT / "scripts" / "export_web_data.py"
PRIVATE_SECRET_FIELDS = {"final_hint", "actual_ip_address", "office_location", "treasure_location"}


def _load_export_module():
    spec = spec_from_file_location("export_web_data", SCRIPT_PATH)
    assert spec is not None and spec.loader is not None
    module = module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def _missing_private_secret_fields(payload: object) -> bool:
    if isinstance(payload, dict):
        if PRIVATE_SECRET_FIELDS.intersection(payload):
            return False
        return all(_missing_private_secret_fields(value) for value in payload.values())
    if isinstance(payload, list):
        return all(_missing_private_secret_fields(value) for value in payload)
    return True


def test_export_web_data_builds_public_manifest_with_expected_counts():
    exporter = _load_export_module()

    bundle = exporter.build_web_data(ROOT)

    assert bundle["manifest"]["schema_version"] == 1
    assert bundle["manifest"]["counts"] == {
        "locations": 16,
        "items": 13,
        "encounters": 21,
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
    assert manifest["counts"]["encounters"] == 21
    secrets = json.loads((out_dir / "secrets.example.json").read_text(encoding="utf-8"))
    assert secrets[0]["final_hint_policy"] == "private_only"


def test_export_web_data_builds_renderer_neutral_content_bundle():
    exporter = _load_export_module()

    bundle = exporter.build_content_bundle(ROOT)

    assert bundle["schema_version"] == 1
    assert bundle["kind"] == "tui_adv.content_bundle"
    assert bundle["manifest"]["counts"]["locations"] == 16
    assert bundle["content"]["locations"][0]["id"] == "dev_desk"
    assert bundle["content"]["encounters"][0]["id"] == "ex_employee_messenger"
    assert _missing_private_secret_fields(bundle)


def test_export_web_data_writes_and_checks_content_bundle(tmp_path):
    exporter = _load_export_module()
    bundle_path = tmp_path / "content.bundle.json"

    written = exporter.write_content_bundle(ROOT, bundle_path)

    assert written == bundle_path
    bundle = json.loads(bundle_path.read_text(encoding="utf-8"))
    assert bundle["kind"] == "tui_adv.content_bundle"
    assert bundle["manifest"]["counts"]["secrets"] == 3
    assert exporter.check_content_bundle(ROOT, bundle_path) == []
    bundle_path.write_text("{}\n", encoding="utf-8")
    assert exporter.check_content_bundle(ROOT, bundle_path) == [
        f"stale generated file: {bundle_path}"
    ]


def test_checked_in_content_bundle_is_up_to_date():
    exporter = _load_export_module()
    bundle_path = ROOT / "crates" / "escape-core" / "fixtures" / "content" / "content.bundle.json"

    assert exporter.check_content_bundle(ROOT, bundle_path) == []


def test_export_web_data_check_detects_stale_generated_files(tmp_path):
    exporter = _load_export_module()
    out_dir = tmp_path / "generated"

    errors = exporter.check_web_data(ROOT, out_dir)

    assert errors
    assert "missing generated file" in errors[0]


def test_export_web_data_cli_write_and_check_roundtrip(tmp_path):
    out_dir = tmp_path / "generated"
    rust_bundle_path = tmp_path / "rust" / "content.bundle.json"
    web_bundle_path = tmp_path / "web" / "content.bundle.json"

    write_result = subprocess.run(
        [
            sys.executable,
            str(SCRIPT_PATH),
            "--root",
            str(ROOT),
            "--out-dir",
            str(out_dir),
            "--bundle",
            str(rust_bundle_path),
            "--bundle",
            str(web_bundle_path),
            "--write",
        ],
        check=False,
        text=True,
        capture_output=True,
    )
    assert write_result.returncode == 0, write_result.stderr
    assert "wrote 7 web data files" in write_result.stdout
    assert f"wrote content bundle to {rust_bundle_path}" in write_result.stdout
    assert f"wrote content bundle to {web_bundle_path}" in write_result.stdout
    assert json.loads(rust_bundle_path.read_text(encoding="utf-8")) == json.loads(
        web_bundle_path.read_text(encoding="utf-8")
    )

    check_result = subprocess.run(
        [
            sys.executable,
            str(SCRIPT_PATH),
            "--root",
            str(ROOT),
            "--out-dir",
            str(out_dir),
            "--bundle",
            str(rust_bundle_path),
            "--bundle",
            str(web_bundle_path),
            "--check",
        ],
        check=False,
        text=True,
        capture_output=True,
    )
    assert check_result.returncode == 0, check_result.stdout + check_result.stderr
    assert "web data is up to date" in check_result.stdout
    assert check_result.stdout.count("content bundle is up to date") == 2


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

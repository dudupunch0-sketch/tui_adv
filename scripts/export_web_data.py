#!/usr/bin/env python3
"""Export committed YAML content into browser-friendly JSON files."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

import yaml

DATA_FILES: tuple[tuple[str, str, str], ...] = (
    ("locations", "locations", "locations.json"),
    ("items", "items", "items.json"),
    ("encounters", "encounters", "encounters.json"),
    ("endings", "endings", "endings.json"),
    ("achievements", "achievements", "achievements.json"),
    ("secrets.example", "secrets", "secrets.example.json"),
)
PRIVATE_SECRET_FIELDS = {"final_hint", "actual_ip_address", "office_location", "treasure_location"}


def build_web_data(root: str | Path) -> dict[str, Any]:
    """Return normalized public web data loaded from source YAML files."""

    repo_root = Path(root)
    data_dir = repo_root / "src" / "tui_adv" / "data"
    bundle: dict[str, Any] = {}
    counts: dict[str, int] = {}
    for source_name, root_key, _output_name in DATA_FILES:
        source_path = data_dir / f"{source_name}.yaml"
        data = _read_yaml(source_path)
        entries = data.get(root_key, [])
        if not isinstance(entries, list):
            raise ValueError(f"{source_path} root key {root_key} must be a list")
        if root_key == "secrets":
            _validate_public_secrets(entries)
        bundle[root_key] = entries
        counts[root_key] = len(entries)
    bundle["manifest"] = {
        "schema_version": 1,
        "source": "src/tui_adv/data/*.yaml",
        "counts": counts,
    }
    return bundle


def write_web_data(root: str | Path, out_dir: str | Path) -> list[Path]:
    """Write generated JSON files and return the file paths written."""

    bundle = build_web_data(root)
    output_dir = Path(out_dir)
    output_dir.mkdir(parents=True, exist_ok=True)
    written: list[Path] = []
    for _source_name, root_key, output_name in DATA_FILES:
        output_path = output_dir / output_name
        _write_json(output_path, bundle[root_key])
        written.append(output_path)
    manifest_path = output_dir / "manifest.json"
    _write_json(manifest_path, bundle["manifest"])
    written.append(manifest_path)
    return written


def check_web_data(root: str | Path, out_dir: str | Path) -> list[str]:
    """Return differences between source YAML and generated JSON files."""

    bundle = build_web_data(root)
    output_dir = Path(out_dir)
    errors: list[str] = []
    expected_files: list[tuple[str, Any]] = [
        (output_name, bundle[root_key])
        for _source_name, root_key, output_name in DATA_FILES
    ]
    expected_files.append(("manifest.json", bundle["manifest"]))
    for output_name, payload in expected_files:
        output_path = output_dir / output_name
        expected_text = _json_text(payload)
        if not output_path.exists():
            errors.append(f"missing generated file: {output_path}")
            continue
        actual_text = output_path.read_text(encoding="utf-8")
        if actual_text != expected_text:
            errors.append(f"stale generated file: {output_path}")
    return errors


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="Export tui_adv YAML content for the browser app")
    parser.add_argument("--root", default=Path(__file__).resolve().parents[1], type=Path)
    parser.add_argument(
        "--out-dir",
        default=Path(__file__).resolve().parents[1] / "web" / "src" / "data" / "generated",
        type=Path,
    )
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--write", action="store_true", help="write generated JSON files")
    mode.add_argument("--check", action="store_true", help="verify generated JSON files are up to date")
    args = parser.parse_args(argv)

    if args.write:
        written = write_web_data(args.root, args.out_dir)
        print(f"wrote {len(written)} web data files to {args.out_dir}")
        return 0

    errors = check_web_data(args.root, args.out_dir)
    if errors:
        print("web data is stale:")
        for error in errors:
            print(f"- {error}")
        return 1
    print("web data is up to date")
    return 0


def _read_yaml(path: Path) -> dict[str, Any]:
    loaded = yaml.safe_load(path.read_text(encoding="utf-8"))
    if loaded is None:
        return {}
    if not isinstance(loaded, dict):
        raise ValueError(f"YAML root must be a mapping: {path}")
    return loaded


def _validate_public_secrets(secrets: list[Any]) -> None:
    for secret in secrets:
        if not isinstance(secret, dict):
            raise ValueError("public secret entry must be a mapping")
        secret_id = secret.get("id", "<missing>")
        for field_name in PRIVATE_SECRET_FIELDS:
            if field_name in secret:
                raise ValueError(f"public secret {secret_id} has private-only field: {field_name}")


def _write_json(path: Path, payload: Any) -> None:
    path.write_text(_json_text(payload), encoding="utf-8")


def _json_text(payload: Any) -> str:
    return json.dumps(payload, ensure_ascii=False, indent=2, sort_keys=True) + "\n"


if __name__ == "__main__":
    raise SystemExit(main())

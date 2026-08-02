#!/usr/bin/env python3
"""Loss-preserving Notion Markdown/CSV importer.

The ``source_pages`` output is local-only immutable provenance. Normalized
design records are the Git design SSoT; raw Notion pages are not public output.
"""
from __future__ import annotations
import argparse, csv, json, re, shutil
from collections import Counter
from pathlib import Path
import yaml

HEX = re.compile(r"(?<![0-9a-f])[0-9a-f]{32}(?![0-9a-f])", re.I)
DB = {
    "0c98f6f18d4f4605ad66c9f899286f06": "events",
    "bd93ddd8de224c7da50abdd309bc2880": "afterthoughts",
    "bcdff734c16e4ea4bfd7ae43fb74e3af": "items",
    "9f599836e4344b9cbff9319f62d54dc6": "insights",
    "d04d0e34055c4623b77c8499a6f1d6eb": "skills",
    "da8f254e72404c6f84a5db5738bb7588": "titles",
    "f1e2c76a9cb0474d8e9a07c45c90d26e": "reward_mappings",
}
FIELDS = {
    "events": ["사건 ID", "이벤트 ID", "Event ID", "ID", "id"],
    "items": ["아이템 ID", "보상 ID", "ID", "id"],
    "insights": ["기연 ID", "보상 ID", "ID", "id"],
    "skills": ["스킬 ID", "보상 ID", "ID", "id"],
    "titles": ["칭호 ID", "보상 ID", "ID", "id"],
    "reward_mappings": ["매핑 ID", "보상 매핑 ID", "ID", "id"],
}

def pid(value):
    match = HEX.search(value or "")
    return match.group(0).lower() if match else None

def slug(value, fallback):
    return re.sub(r"[^a-z0-9_-]+", "_", value.strip().lower()).strip("_")[:100] or fallback

def write_yaml(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(yaml.safe_dump(data, allow_unicode=True, sort_keys=False), encoding="utf-8")

def main():
    p = argparse.ArgumentParser()
    p.add_argument("--export-root", type=Path, required=True)
    p.add_argument("--output-root", type=Path, required=True)
    p.add_argument("--zip-hash", required=True)
    p.add_argument("--source-zip", required=True)
    p.add_argument("--extracted-at", required=True)
    a = p.parse_args()
    src, out = a.export_root.resolve(), a.output_root.resolve()
    if not src.is_dir(): raise SystemExit(f"missing export root: {src}")
    if out.exists() and any(out.iterdir()): raise SystemExit(f"refusing non-empty output: {out}")
    out.mkdir(parents=True, exist_ok=True)
    page_count = 0
    csv_count = 0
    arc_count = 0
    for f in sorted(src.rglob("*")):
        if not f.is_file(): continue
        d = out / "source_pages" / f.relative_to(src)
        d.parent.mkdir(parents=True, exist_ok=True); shutil.copy2(f, d)
        if f.suffix == ".md":
            page_count += 1
        elif f.suffix == ".csv":
            csv_count += 1
        if f.suffix == ".md" and len(f.relative_to(src).parts) <= 2:
            name = pid(f.name) or slug(f.stem, f"page_{arc_count:04d}")
            d = out / "arcs" / "imported" / f"{name}.md"
            d.parent.mkdir(parents=True, exist_ok=True); shutil.copy2(f, d); arc_count += 1
    inventory, generated, ids = [], Counter(), {}
    for f in sorted(src.rglob("*.csv")):
        with f.open(encoding="utf-8-sig", newline="") as h:
            reader = csv.DictReader(h); rows = list(reader); headers = list(reader.fieldnames or [])
        match = HEX.search(f.name); dbid = match.group(0).lower() if match else "unknown"; kind = DB.get(dbid, "unknown")
        is_all = f.name.endswith("_all.csv")
        inventory.append({"path": str(f.relative_to(src)), "database_id": dbid, "kind": kind, "is_all_export": is_all, "headers": headers, "row_count": len(rows)})
        if is_all: continue
        if kind == "unknown":
            legacy_path = out / "legacy_rewards" / "imported" / f"legacy_{dbid}.yml"
            write_yaml(legacy_path, {"id": f"legacy_{dbid}", "kind": "legacy_reward_inventory", "status": "imported_unreviewed", "runtime_status": "design", "canonical_candidate": "undecided", "source_refs": [{"export_file": str(f.relative_to(src)), "database_id": dbid}], "row_count": len(rows), "notion_properties": rows})
            continue
        for index, row in enumerate(rows, 1):
            rid = next((slug(str(row[field]), f"notion_{index:04d}") for field in FIELDS.get(kind, []) if row.get(field)), f"notion_{index:04d}")
            refs = [{"export_file": str(f.relative_to(src)), "database_id": dbid}]
            refs.extend({"notion_page_id": x, "notion_url": f"https://app.notion.com/p/{x}"} for x in sorted({pid(str(v)) for v in row.values() if v and pid(str(v))}))
            data = {"id": rid, "status": "imported_unreviewed", "runtime_status": "design", "source_refs": refs, "notion_properties": dict(row)}
            if kind == "events":
                data.update(kind="event", storypack_id=None, arc_id=None, stages=[]); target = out / "events" / "imported" / f"{rid}.yml"; bucket = "events"
            elif kind == "afterthoughts":
                data.update(kind="afterthought", storypack_id=None, arc_id=None, stages=[]); target = out / "afterthoughts" / "imported" / f"{rid}.yml"; bucket = "afterthoughts"
            elif kind == "reward_mappings":
                data.update(event_id=None, choice_id=None, reward_ids=[], condition=None); target = out / "reward_mappings" / "imported" / f"{rid}.yml"; bucket = "reward_mappings"
            else:
                data.update(kind=kind[:-1], effect_policy={"numeric_effects": "deferred"}); target = out / "rewards" / kind / f"{rid}.yml"; bucket = f"rewards/{kind}"
            write_yaml(target, data); generated[bucket] += 1; ids.setdefault(bucket, []).append(rid)
    schema = out / "schema"; schema.mkdir(exist_ok=True)
    for name in ("event", "reward", "reward_mapping", "story_graph", "arc"):
        (schema / f"{name}.schema.json").write_text(json.dumps({"type": "object", "required": ["id", "status", "runtime_status", "source_refs"], "additionalProperties": True}, indent=2) + "\n")
    manifest = {"schema_version": 1, "repository": "tui-adv", "canonical_source": "local_design_source_candidate", "local_canonical_declared": False, "imported_at": a.extracted_at, "source_ledger": [{"key": "notion_extract_zip", "source_type": "notion_export_zip", "source_path": a.source_zip, "sha256": a.zip_hash, "raw_zip_committed": False, "extracted_at": a.extracted_at}], "paths": {"source_pages": "source_pages/**/*", "arcs": "arcs/**/*.md", "events": "events/**/*.yml",
        "afterthoughts": "afterthoughts/**/*.yml",
        "legacy_rewards": "legacy_rewards/**/*.yml", "rewards": "rewards/**/*.yml", "reward_mappings": "reward_mappings/**/*.yml", "graphs": "graphs/*.yml"}, "records": {k: {"count": len(v), "ids": v} for k, v in ids.items()}, "csv_inventory": inventory, "replica": {"markdown_files": page_count, "csv_files": csv_count, "arc_copies": arc_count, "total_files": page_count + csv_count}, "sync": {"direction": "local_to_notion_publish", "notion_mirror_status": "not_started", "base_commit": None}, "ambiguities": ["CSV values are preserved verbatim; semantic IDs and graph links require review.", "Rows without recognizable IDs use notion_NNNN."]}
    write_yaml(out / "manifest.yml", manifest); (out / "reports").mkdir()
    report = {"zip_sha256": a.zip_hash, "source_zip": a.source_zip, "extracted_at": a.extracted_at, "csv_inventory": inventory, "generated": dict(generated), "duplicate_ids": {k: v for k, v in ids.items() if len(v) != len(set(v))}, "markdown_over_100kb": [str(x.relative_to(out)) for x in out.rglob("*.md") if x.stat().st_size > 100 * 1024]}
    (out / "reports" / "import_report.json").write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")

if __name__ == "__main__": main()

#!/usr/bin/env python3
"""Build policy overlays without mutating event afterthought source fields."""
from __future__ import annotations

import argparse
from pathlib import Path

import yaml


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, required=True)
    args = parser.parse_args()
    root = args.root.resolve()
    ledger_path = root / "reports/afterthought_triage.yml"
    ledger = yaml.safe_load(ledger_path.read_text(encoding="utf-8"))
    entries = ledger.get("entries", [])
    conditions = [
        {
            "event_id": entry["event_id"],
            "source_path": entry["source_path"],
            "raw_reference": entry["raw_reference"],
            "condition_summary": entry["condition_summary"],
            "related_keywords": entry.get("related_keywords", []),
            "status": "approved_as_condition",
        }
        for entry in entries
        if entry.get("classification") == "descriptive_condition"
    ]
    graph_dir = root / "graphs"
    graph_dir.mkdir(parents=True, exist_ok=True)
    (graph_dir / "event_afterthought_links.yml").write_text(
        yaml.safe_dump(
            {
                "schema_version": 1,
                "policy": {
                    "eligibility_evaluation": "event_resolution",
                    "reveal_default": "ending_resolution",
                    "exclusive_group_rule": "at_most_one",
                    "runtime_contract": False,
                },
                "links": [],
            },
            allow_unicode=True,
            sort_keys=False,
        ),
        encoding="utf-8",
    )
    (graph_dir / "afterthought_conditions.yml").write_text(
        yaml.safe_dump(
            {
                "schema_version": 1,
                "source": "reports/afterthought_triage.yml",
                "runtime_contract": False,
                "conditions": conditions,
            },
            allow_unicode=True,
            sort_keys=False,
        ),
        encoding="utf-8",
    )
    print(f"approved conditions: {len(conditions)}")
    print("explicit links: 0")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

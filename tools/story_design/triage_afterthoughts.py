#!/usr/bin/env python3
"""Create a conservative, source-preserving triage ledger for unstructured afterthought refs."""
from __future__ import annotations

import argparse
from pathlib import Path

import yaml


EMPTY = {"없음", "해당 없음", "No", "-"}
REVIEW_MARKERS = ("후속", "후보", "가능", "연결", "관련")
KEYWORDS = (
    "청류문", "서하린", "무명", "흑사방", "무림맹", "천기록", "귀환",
    "산문", "무공", "결말", "습격", "후일", "약재", "검", "목검",
    "장터", "풍문", "기록", "편지", "문", "밥", "수련",
)


def properties(record: dict) -> dict:
    value = record.get("notion_properties")
    return value if isinstance(value, dict) else {}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    root = args.root.resolve()

    afterthoughts = {}
    for path in sorted((root / "afterthoughts").rglob("*.yml")):
        record = yaml.safe_load(path.read_text(encoding="utf-8"))
        if isinstance(record, dict):
            props = properties(record)
            afterthoughts[str(record.get("id", ""))] = {
                "id": str(record.get("id", "")),
                "title": str(props.get("카드명", "")),
            }
    by_title = {item["title"]: item["id"] for item in afterthoughts.values() if item["title"]}
    by_id = set(afterthoughts)

    entries = []
    for path in sorted((root / "events").rglob("*.yml")):
        record = yaml.safe_load(path.read_text(encoding="utf-8"))
        if not isinstance(record, dict):
            continue
        raw = str(properties(record).get("후일담 카드 연결", "")).strip()
        if not raw or raw in EMPTY or raw in by_id or raw in by_title:
            continue

        exact_tokens = []
        for token in [part.strip() for part in raw.replace(",", "/").split("/") if part.strip()]:
            if token in by_id:
                exact_tokens.append((token, token))
            elif token in by_title:
                exact_tokens.append((token, by_title[token]))
        candidates = sorted({candidate for _, candidate in exact_tokens})
        reasons = ["후일담 카드 연결 필드의 원문을 보존함", "raw reference is not an exact afterthought ID/title"]
        if exact_tokens:
            reasons.append("exact title/ID token(s): " + ", ".join(token for token, _ in exact_tokens))

        if len(candidates) >= 2 or any(marker in raw for marker in REVIEW_MARKERS):
            classification = "designer_review_required"
            review_status = "needs_designer_review"
            confidence = "high" if len(candidates) >= 2 else "medium"
            reasons.append("둘 이상의 후보 또는 후속/후보/가능/연결 표현이 있어 새 설계 결정이 필요함")
        else:
            classification = "descriptive_condition"
            review_status = "triaged_pending_schema_decision"
            confidence = "medium"
            reasons.append("카드 ID가 아닌 사건 후속 조건/설명으로만 안전하게 해석함")

        keywords = [keyword for keyword in KEYWORDS if keyword in raw]
        entry = {
            "event_id": str(record.get("id", "")),
            "source_path": str(path.relative_to(root)),
            "raw_reference": raw,
            "classification": classification,
            "candidate_afterthought_ids": candidates,
            "evidence": reasons,
            "confidence": confidence,
            "review_status": review_status,
        }
        if classification == "descriptive_condition":
            entry["condition_summary"] = raw
            entry["related_keywords"] = keywords
        else:
            entry["topic_keywords"] = keywords
        entries.append(entry)

    output = args.output
    output.parent.mkdir(parents=True, exist_ok=True)
    document = {
        "schema_version": 1,
        "source": "docs/content/design_source/reports/validation/design_source_validation.json",
        "policy": {
            "exact_match_requires_literal_one_to_one_id_or_title": True,
            "fuzzy_matching": False,
            "event_afterthought_links_modified": False,
        },
        "coverage": {"expected_unstructured_records": len(entries), "entries": len(entries)},
        "entries": entries,
    }
    output.write_text(yaml.safe_dump(document, allow_unicode=True, sort_keys=False), encoding="utf-8")
    print(f"wrote {len(entries)} entries to {output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

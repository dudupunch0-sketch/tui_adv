#!/usr/bin/env python3
from __future__ import annotations
import argparse, json, re
from collections import Counter
from pathlib import Path
import yaml

ID = re.compile(r"^[a-z0-9][a-z0-9_-]*$")
URL = re.compile(r"^https?://")
HEX = re.compile(r"(?<![0-9a-f])[0-9a-f]{32}(?![0-9a-f])", re.I)
FOLDERS = ("events", "afterthoughts", "rewards", "reward_mappings", "legacy_rewards")

def props(d):
    return d.get("notion_properties") if isinstance(d.get("notion_properties"), dict) else {}

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", type=Path, required=True)
    a = ap.parse_args()
    root = a.root.resolve()
    structural_errors, semantic_errors, warnings, unverifiable = [], [], [], []
    page_provenance_records, afterthought_text_records = [], []
    relation_effects, verified_conditions = [], []
    records, by_id = [], {}
    manifest_path = root / "manifest.yml"
    manifest = yaml.safe_load(manifest_path.read_text(encoding="utf-8")) if manifest_path.exists() else {}
    if not manifest: structural_errors.append("missing manifest.yml")
    for folder in FOLDERS:
        base = root / folder
        if not base.exists(): continue
        for f in sorted(base.rglob("*.yml")):
            try:
                d = yaml.safe_load(f.read_text(encoding="utf-8"))
                if not isinstance(d, dict): raise ValueError("not an object")
            except Exception as exc:
                structural_errors.append(f"{f.relative_to(root)} invalid YAML: {exc}"); continue
            rid = str(d.get("id", ""))
            rel = str(f.relative_to(root))
            records.append((f, d, folder))
            by_id.setdefault(rid, []).append((f, folder))
            if not rid: structural_errors.append(f"{rel} missing id")
            elif not ID.fullmatch(rid): structural_errors.append(f"{rel} invalid id format: {rid}")
            for k in ("status", "runtime_status", "source_refs"):
                if k not in d: structural_errors.append(f"{rel} missing {k}")
            if folder == "events" and d.get("kind") != "event": structural_errors.append(f"{rel} wrong kind")
            if folder == "afterthoughts" and d.get("kind") != "afterthought": structural_errors.append(f"{rel} wrong kind")
            if folder == "legacy_rewards" and d.get("canonical_candidate") != "undecided": structural_errors.append(f"{rel} legacy not undecided")
            refs = d.get("source_refs")
            if not isinstance(refs, list) or not refs: unverifiable.append({"record": rid, "reason": "missing source_refs"})
            else:
                for ref in refs:
                    if not ref.get("notion_page_id") or not ref.get("notion_url"):
                        source = str(ref.get("export_file", ""))
                        match = HEX.search(source)
                        page_provenance_records.append({"record": rid, "source_export_id": match.group(0).lower() if match else None, "source_path": source})
    duplicates = {k: [str(x[0].relative_to(root)) for x in v] for k, v in by_id.items() if k and len(v) > 1}
    for rid, paths in duplicates.items(): semantic_errors.append(f"duplicate id {rid}: {paths}")
    event_ids = {d["id"] for f,d,folder in records if folder == "events"}
    after_ids = {d["id"] for f,d,folder in records if folder == "afterthoughts"}
    reward_ids = {d["id"] for f,d,folder in records if folder == "rewards"}
    legacy_ids = {d["id"] for f,d,folder in records if folder == "legacy_rewards"}
    mapping_rows = [(f,d) for f,d,folder in records if folder == "reward_mappings"]
    mapping_issues = []
    choice_unverifiable = []
    text_mismatches = []
    for f,d in mapping_rows:
        p = props(d); rel = str(f.relative_to(root))
        eid = p.get("사건 ID") or d.get("event_id")
        if eid and eid not in event_ids: semantic_errors.append(f"{rel}: mapping event missing {eid}")
        reward_text = str(p.get("보상 ID") or "").strip()
        if reward_text:
            tokens = [x.strip() for x in re.split(r"[,;/]+", reward_text) if x.strip()]
            for token in tokens:
                if token not in reward_ids and token not in legacy_ids:
                    if token.startswith("relationship_") and p.get("보상 종류") in {"인물 호감도", "단체 호감도"}:
                        relation_effects.append({"mapping_id": p.get("매핑 ID", d.get("id")), "reward_id": token, "reward_type": p.get("보상 종류"), "classification": "external/relation_effect"})
                    else:
                        semantic_errors.append(f"{rel}: reward missing {token}")
        else: unverifiable.append({"record": d.get("id"), "reason": "mapping has no parseable reward id"})
        choice = p.get("선택지·조건 키") or d.get("choice_id")
        if choice and eid:
            event = next((x for ff,x,fo in records if fo == "events" and x.get("id") == eid), None)
            summary = str(props(event).get("선택지 요약", "")) if event else ""
            if summary and str(choice) not in summary and "조건" in str(choice):
                verified_conditions.append({"mapping_id": p.get("매핑 ID", d.get("id")), "value": choice, "condition_type": "system_condition", "classification": "verified_by_source_wording"})
            elif summary and str(choice) not in summary: unverifiable.append({"record": d.get("id"), "reason": "condition key is not a literal choice-summary entry"})
            elif not summary: choice_unverifiable.append(d.get("id"))
        else: choice_unverifiable.append(d.get("id"))
    afterthought_names = {str(props(d).get("카드명", "")): d.get("id") for f,d,folder in records if folder == "afterthoughts"}
    afterthought_card_ids = {str(props(d).get("카드 ID", "")): d.get("id") for f,d,folder in records if folder == "afterthoughts"}
    afterthought_classification = {"empty": 0, "exact_card_id": 0, "exact_card_name": 0, "general_description": 0, "malformed": 0}
    verified_afterthought_links = []
    for f,d,folder in records:
        text = str(props(d).get("후일담 카드 연결", ""))
        if not text.strip(): continue
        if text.strip() in {"없음", "해당 없음", "No", "-"}:
            afterthought_classification["empty"] += 1; continue
        if text in after_ids or text in afterthought_card_ids:
            afterthought_classification["exact_card_id"] += 1
            verified_afterthought_links.append({"event_id": d.get("id"), "afterthought_id": text if text in after_ids else afterthought_card_ids[text], "match": "literal_id"})
            continue
        if text in afterthought_names:
            afterthought_classification["exact_card_name"] += 1
            verified_afterthought_links.append({"event_id": d.get("id"), "afterthought_id": afterthought_names[text], "match": "literal_title"})
            continue
        if any(ch in text for ch in "|[]{}"):
            afterthought_classification["malformed"] += 1
        else:
            afterthought_classification["general_description"] += 1
            afterthought_text_records.append({"record": d.get("id"), "source_path": str(f.relative_to(root)), "value": text})
    source_refs = [ref for f,d,folder in records for ref in (d.get("source_refs") or [])]
    with_page = sum(bool(r.get("notion_page_id") and r.get("notion_url") and URL.match(str(r.get("notion_url")))) for r in source_refs)
    triage_path = root / "reports/afterthought_triage.yml"
    triage_entries = []
    triage_counts = {"exact_match": 0, "intentional_no_link": 0, "descriptive_condition": 0, "designer_review_required": 0}
    triage_review_ids = []
    if not triage_path.exists():
        structural_errors.append("missing reports/afterthought_triage.yml")
    else:
        try:
            triage_document = yaml.safe_load(triage_path.read_text(encoding="utf-8"))
            triage_entries = triage_document.get("entries", []) if isinstance(triage_document, dict) else []
            if not isinstance(triage_entries, list):
                structural_errors.append("afterthought_triage.yml entries is not a list")
                triage_entries = []
        except Exception as exc:
            structural_errors.append(f"afterthought_triage.yml invalid YAML: {exc}")
        expected_triage_ids = {x["record"] for x in afterthought_text_records}
        actual_triage_ids = [str(x.get("event_id", "")) for x in triage_entries if isinstance(x, dict)]
        actual_triage_set = set(actual_triage_ids)
        if len(actual_triage_ids) != len(actual_triage_set):
            structural_errors.append("afterthought triage duplicate event_id")
        missing_triage = sorted(expected_triage_ids - actual_triage_set)
        extra_triage = sorted(actual_triage_set - expected_triage_ids)
        if missing_triage: structural_errors.append(f"afterthought triage missing records: {missing_triage}")
        if extra_triage: structural_errors.append(f"afterthought triage unexpected records: {extra_triage}")
        allowed_triage = set(triage_counts)
        required_triage_fields = {"event_id", "source_path", "classification", "candidate_afterthought_ids", "evidence", "confidence", "review_status"}
        for entry in triage_entries:
            if not isinstance(entry, dict):
                structural_errors.append("afterthought triage entry is not an object")
                continue
            missing_fields = sorted(required_triage_fields - set(entry))
            if missing_fields: structural_errors.append(f"afterthought triage {entry.get('event_id')}: missing {missing_fields}")
            classification = entry.get("classification")
            if classification not in allowed_triage:
                structural_errors.append(f"afterthought triage {entry.get('event_id')}: invalid classification {classification}")
                continue
            triage_counts[classification] += 1
            if classification == "designer_review_required": triage_review_ids.append(entry.get("event_id"))
            candidates = entry.get("candidate_afterthought_ids", [])
            if not isinstance(candidates, list) or any(str(x) not in after_ids for x in candidates):
                structural_errors.append(f"afterthought triage {entry.get('event_id')}: invalid candidate ids")
            raw = str(entry.get("raw_reference", ""))
            if classification == "exact_match":
                if len(candidates) != 1 or raw not in {str(candidates[0]), str(props(next((x for ff,x,fo in records if fo == "afterthoughts" and x.get("id") == candidates[0]), {})).get("카드명", ""))}:
                    semantic_errors.append(f"afterthought triage {entry.get('event_id')}: exact_match is not literal one-to-one")
            if classification == "intentional_no_link" and raw not in {"", "없음", "해당 없음", "No", "-"}:
                semantic_errors.append(f"afterthought triage {entry.get('event_id')}: intentional_no_link has non-empty reference")
            if classification == "descriptive_condition" and not entry.get("condition_summary"):
                structural_errors.append(f"afterthought triage {entry.get('event_id')}: descriptive_condition missing condition_summary")
    triage_coverage = {"expected": len(afterthought_text_records), "actual": len(triage_entries), "missing": sorted(set(x["record"] for x in afterthought_text_records) - {str(x.get("event_id")) for x in triage_entries if isinstance(x, dict)}), "extra": sorted({str(x.get("event_id")) for x in triage_entries if isinstance(x, dict)} - {x["record"] for x in afterthought_text_records})}
    condition_overlay_path = root / "graphs/afterthought_conditions.yml"
    link_overlay_path = root / "graphs/event_afterthought_links.yml"
    condition_overlay = {}
    link_overlay = {}
    unresolved_external_conditions = []
    unresolved_runtime_links = []
    if not condition_overlay_path.exists(): structural_errors.append("missing graphs/afterthought_conditions.yml")
    else:
        try: condition_overlay = yaml.safe_load(condition_overlay_path.read_text(encoding="utf-8")) or {}
        except Exception as exc: structural_errors.append(f"afterthought_conditions.yml invalid YAML: {exc}")
    if not link_overlay_path.exists(): structural_errors.append("missing graphs/event_afterthought_links.yml")
    else:
        try: link_overlay = yaml.safe_load(link_overlay_path.read_text(encoding="utf-8")) or {}
        except Exception as exc: structural_errors.append(f"event_afterthought_links.yml invalid YAML: {exc}")
    condition_entries = condition_overlay.get("conditions", []) if isinstance(condition_overlay, dict) else []
    link_entries = link_overlay.get("links", []) if isinstance(link_overlay, dict) else []
    approved_condition_ids = {str(x.get("event_id")) for x in triage_entries if isinstance(x, dict) and x.get("classification") == "descriptive_condition" and x.get("review_status") == "approved_as_condition"}
    condition_overlay_ids = [str(x.get("event_id")) for x in condition_entries if isinstance(x, dict)]
    if len(condition_overlay_ids) != len(set(condition_overlay_ids)): structural_errors.append("afterthought condition overlay duplicate event_id")
    if set(condition_overlay_ids) != approved_condition_ids: structural_errors.append("afterthought condition overlay does not exactly cover approved conditions")
    for entry in condition_entries:
        if not isinstance(entry, dict) or not entry.get("raw_reference") or not entry.get("condition_summary"):
            structural_errors.append(f"afterthought condition missing raw_reference/condition_summary: {entry}")
        elif entry.get("status") != "approved_as_condition":
            semantic_errors.append(f"afterthought condition is not approved: {entry.get('event_id')}")
    link_pairs = set()
    group_priorities = {}
    chain_edges = {}
    for entry in link_entries:
        if not isinstance(entry, dict): structural_errors.append("event afterthought link is not an object"); continue
        pair = (str(entry.get("event_id")), str(entry.get("afterthought_id")))
        if pair in link_pairs: semantic_errors.append(f"duplicate event+afterthought link: {pair}")
        link_pairs.add(pair)
        if pair[0] not in event_ids: semantic_errors.append(f"afterthought link event missing: {pair[0]}")
        if pair[1] not in after_ids: semantic_errors.append(f"afterthought link card missing: {pair[1]}")
        if entry.get("status") != "approved": unresolved_runtime_links.append(pair)
        if entry.get("reveal_policy") not in {"ending_resolution", "run_end"}: semantic_errors.append(f"invalid reveal policy: {pair}")
        group = entry.get("exclusive_group")
        priority = entry.get("priority")
        if group is not None and priority is not None:
            key = (str(group), int(priority))
            if key in group_priorities: semantic_errors.append(f"exclusive-group priority tie: {key}")
            group_priorities[key] = pair
        eligibility = entry.get("eligibility") or {}
        for bucket in ("all_of", "any_of", "none_of"):
            for token in eligibility.get(bucket, []) if isinstance(eligibility, dict) else []:
                if str(token).startswith(("flag:", "ending:", "relation:")):
                    unresolved_external_conditions.append({"link": pair, "token": str(token), "reason": "external registry not present in design source"})
        if entry.get("relation") == "chain" and entry.get("depends_on"):
            chain_edges.setdefault(pair[1], []).append(str(entry["depends_on"]))
    def has_cycle(node, visiting, visited):
        if node in visiting: return True
        if node in visited: return False
        visiting.add(node)
        if any(has_cycle(child, visiting, visited) for child in chain_edges.get(node, [])): return True
        visiting.remove(node); visited.add(node); return False
    if any(has_cycle(node, set(), set()) for node in chain_edges): semantic_errors.append("afterthought chain cycle detected")
    pending_review_ids = sorted(str(x.get("event_id")) for x in triage_entries if isinstance(x, dict) and x.get("classification") == "designer_review_required")
    if len(after_ids) == 18 and sum(afterthought_classification.values()) != 134:
        structural_errors.append(f"afterthought classification total mismatch: {sum(afterthought_classification.values())} != 134")
    capabilities = {"next_fallback": "not_available_in_source", "choice_matching": "partial_text_match", "notion_page_ids_urls": "unavailable_in_export", "afterthought_text_ids": "exact_id_or_name_only"}
    counts = {"events": len(event_ids), "afterthoughts": len(after_ids), "formal_rewards": len(reward_ids), "legacy_reward_records": len(legacy_ids), "reward_mappings": len(mapping_rows), "records": len(records), "source_refs": len(source_refs), "source_refs_with_page_id_url": with_page, "source_refs_with_export_id": sum(bool(x.get("source_export_id")) for x in page_provenance_records), "page_provenance_records": len(page_provenance_records), "afterthought_text_unverifiable": len(afterthought_text_records), "afterthought_original_total": sum(afterthought_classification.values())}
    for f in root.rglob("*.md"):
        if f.stat().st_size > 100*1024: structural_errors.append(f"markdown over 100KB: {f.relative_to(root)}")
    for f in root.rglob("*.json"):
        try: json.loads(f.read_text(encoding="utf-8"))
        except Exception as exc: structural_errors.append(f"{f.relative_to(root)} invalid JSON: {exc}")
    if choice_unverifiable: unverifiable.append({"records": sorted(choice_unverifiable), "reason": "choice key could not be safely matched"})
    result = {"status": "FAIL" if structural_errors or semantic_errors else "PASS", "structural_errors": structural_errors, "semantic_errors": semantic_errors, "warnings": warnings, "unverifiable": unverifiable, "provenance": {"page_id_url_unavailable_in_export": page_provenance_records}, "verified_afterthought_links": verified_afterthought_links, "verified_external_relation_effects": relation_effects, "verified_conditions": verified_conditions, "afterthought_classification": afterthought_classification, "afterthought_triage": {"classification_counts": triage_counts, "review_required_ids": sorted(triage_review_ids), "coverage": triage_coverage, "ledger_path": str(triage_path.relative_to(root)), "pending_review_ids": pending_review_ids, "pending_review_count": len(pending_review_ids), "approved_condition_count": len(approved_condition_ids)}, "afterthought_overlays": {"condition_overlay_path": str(condition_overlay_path.relative_to(root)), "link_overlay_path": str(link_overlay_path.relative_to(root)), "approved_link_count": sum(x.get("status") == "approved" for x in link_entries if isinstance(x, dict)), "unresolved_runtime_links": unresolved_runtime_links, "unresolved_external_conditions": unresolved_external_conditions, "runtime_contract": False}, "capability_limitations": {"notion_page_ids_urls": "export did not include live page IDs/URLs; export-derived IDs are retained in provenance only", "afterthought_text": {"record_ids": [x["record"] for x in afterthought_text_records], "source_paths": [x["source_path"] for x in afterthought_text_records]}}, "capabilities": capabilities, "counts": counts, "duplicate_ids": duplicates, "policy": {"semantic_errors_nonzero": True, "warnings_and_unverifiable_only_do_not_fail": True, "afterthought_unstructured_requires_triage_ledger": True, "approved_condition_requires_raw_and_summary": True, "approved_links_only_graph_input": True, "default_reveal_policy": "ending_resolution", "external_conditions_are_unresolved_not_silent_pass": True}, "manifest_declares_local_canonical": bool(manifest.get("local_canonical_declared", False))}
    out = root / "reports/validation/design_source_validation.json"; out.parent.mkdir(parents=True, exist_ok=True); out.write_text(json.dumps(result, ensure_ascii=False, indent=2)+"\n", encoding="utf-8")
    print(json.dumps(result, ensure_ascii=False, indent=2))
    return 1 if result["status"] == "FAIL" else 0
if __name__ == "__main__": raise SystemExit(main())

#!/usr/bin/env python3
from __future__ import annotations
import argparse
import json
from pathlib import Path
import yaml

EXPECTED = {
    "termination": "combat_termination",
    "simulation_version": "combat_simulation_version",
    "identity": "combat_display_identity",
    "logs": "combat_log_presentation",
    "tactical_zones": "combat_tactical_zones",
}
REQUIRED_TERMINATION = ["forced_stop", "captured", "surrendered", "fled", "objective_completed", "both_sides_defeated", "one_side_defeated", "max_ticks"]
REQUIRED_FACTS = {"forced_stop_requested", "actor_captured", "actor_surrendered", "actor_fled", "objective_progressed", "both_sides_defeated", "one_side_defeated", "tick_limit_reached"}
REQUIRED_RESULTS = {"victory", "defeat", "escape", "surrender", "capture", "objective", "forced_stop"}

def load(path: Path):
    return yaml.safe_load(path.read_text(encoding="utf-8"))

def interval_error(zones, axis):
    ranges = []
    for zone in zones:
        bounds = zone.get("bounds", {}).get(axis)
        if not isinstance(bounds, list) or len(bounds) != 2:
            return f"{axis} bounds must contain [min, max]"
        lo, hi = bounds
        if not (isinstance(lo, (int, float)) and isinstance(hi, (int, float)) and 0 <= lo < hi <= 1):
            return f"{axis} bounds out of range"
        ranges.append((lo, hi, zone.get("id")))
    ranges.sort()
    cursor = 0.0
    for lo, hi, zid in ranges:
        if lo < cursor:
            return f"{axis} overlap at {zid}"
        if lo > cursor:
            return f"{axis} gap before {zid}"
        cursor = hi
    if cursor != 1.0:
        return f"{axis} incomplete coverage at {cursor}"
    return None

def validate(root: Path):
    errors = []
    data = {}
    for key, contract_id in EXPECTED.items():
        path = root / "contracts" / f"{key}.yml"
        if not path.exists():
            errors.append(f"missing contract: {path.relative_to(root)}")
            continue
        try:
            value = load(path)
        except Exception as exc:
            errors.append(f"{path.name}: invalid YAML: {exc}")
            continue
        data[key] = value
        if not isinstance(value, dict) or value.get("contract_id") != contract_id:
            errors.append(f"{path.name}: wrong contract_id")
        if isinstance(value, dict) and value.get("status") != "canonical":
            errors.append(f"{path.name}: status must be canonical")
        if isinstance(value, dict) and value.get("runtime_status") != "handoff_required":
            errors.append(f"{path.name}: runtime_status must remain handoff_required")

    term = data.get("termination", {})
    order = term.get("priority_order", [])
    ids = [x.get("id") for x in order if isinstance(x, dict)]
    priorities = [x.get("priority") for x in order if isinstance(x, dict)]
    if ids != REQUIRED_TERMINATION:
        errors.append("termination: priority order mismatch")
    if len(priorities) != len(set(priorities)) or priorities != sorted(priorities):
        errors.append("termination: priorities must be unique and ascending")
    if term.get("tie_policy") != "validator_error":
        errors.append("termination: tie policy must be validator_error")
    if set(term.get("primitive_facts", [])) != REQUIRED_FACTS:
        errors.append("termination: primitive facts mismatch")
    mapping = term.get("objective_mapping", {})
    if set(mapping.get("allowed_result_kinds", [])) != REQUIRED_RESULTS:
        errors.append("termination: objective result enum mismatch")

    sim = data.get("simulation_version", {})
    versions = sim.get("supported_versions", [])
    if not any(x.get("version") == "v1" and x.get("current_runtime_observed") is True for x in versions if isinstance(x, dict)):
        errors.append("simulation_version: current supported runtime version v1 is missing")
    if sim.get("unsupported_version") != "validator_error" or sim.get("missing_version") != "validator_error":
        errors.append("simulation_version: unsupported/missing versions must fail")
    if sim.get("fallback") != "forbidden":
        errors.append("simulation_version: fallback must be forbidden")

    identity = data.get("identity", {})
    if identity.get("fallback_order") != ["encounter_alias", "canonical_name", "declared_generic_role_label", "unknown_combatant"]:
        errors.append("identity: fallback order mismatch")
    if identity.get("internal_id", {}).get("user_visible") != "forbidden":
        errors.append("identity: internal id must not be user-visible")

    logs = data.get("logs", {})
    if logs.get("group_key") != ["tick", "template_family", "actor_id", "target_id"]:
        errors.append("logs: unstable group key")
    if not set(["terminal", "status", "objective"]).issubset(set(logs.get("non_groupable", []))):
        errors.append("logs: terminal/status/objective must be non-groupable")

    zones = data.get("tactical_zones", {})
    basis = zones.get("coordinate_basis", {})
    if basis.get("normalized_bounds") != [0.0, 1.0] or basis.get("raw_coordinates") != "debug_only":
        errors.append("tactical_zones: normalized/debug coordinate policy mismatch")
    layout = zones.get("default_layout", {})
    if layout.get("depth_boundaries") != [0.0, 0.33, 0.66, 1.0] or layout.get("lane_boundaries") != [0.0, 0.5, 1.0]:
        errors.append("tactical_zones: default boundaries mismatch")
    for axis in ("depth", "lane"):
        if "zones" in zones:
            problem = interval_error(zones["zones"], axis)
            if problem:
                errors.append(f"tactical_zones: {problem}")

    return errors

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, required=True)
    args = parser.parse_args()
    errors = validate(args.root.resolve())
    result = {"status": "PASS" if not errors else "FAIL", "errors": errors, "runtime_status": "handoff_required", "supported_simulation_version_observed": "v1"}
    print(json.dumps(result, ensure_ascii=False, indent=2))
    return 1 if errors else 0

if __name__ == "__main__":
    raise SystemExit(main())

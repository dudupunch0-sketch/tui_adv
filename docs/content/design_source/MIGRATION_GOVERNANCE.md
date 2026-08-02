# Design Source Migration Governance

Effective date: 2026-08-02

## Canonical scope

`docs/content/design_source/` is the canonical source for the current design records only: events, afterthoughts, rewards, reward mappings, provenance metadata, and validation reports. It is not the canonical runtime graph, generated bundle, or game-code contract.

This is a versioned directory inside the Git repository, not a separate database or storage service. It is the planning SSoT for story, events, choices, afterthoughts, rewards, relations, and planning provenance. General engine, UI, build, and combat work should not read the whole directory on every task; for content IDs, rewards, or branch-contract work, read the manifest first and then only the relevant records.

The Notion export remains preserved under the recorded staging/provenance paths. Notion is now a read/review mirror. New design changes flow local → review in Notion. Notion edits must not silently become a competing source of truth.

## Directory use

- `events/`: 154 normalized event records.
- `afterthoughts/`: 18 normalized afterthought records.
- `rewards/`: 51 formal reward records.
- `legacy_rewards/`: deferred legacy inventory; not canonical until separately decided.
- `reward_mappings/`: 29 choice/condition reward mappings.
- `source_pages/`: loss-preserving export pages and CSVs.
- `reports/`: import, validation, and migration evidence.
- `schema/`: design-record schemas; these do not define the runtime graph.

## Editing and review

Edit the local design records and rerun the importer/validator only when the migration workflow explicitly requires it. Preserve source pages and provenance. The flow is local design source → review → optional runtime handoff/implementation → Notion review mirror. Runtime direct contracts remain runtime schemas, preview/generated sources, and `docs/dev/Development_Plan.md`; this directory is not executable input.

Use progressive disclosure: read `manifest.yml` and this governance file first, then open only the related records. Keep AI-facing Markdown under 100KB.

`source_pages/` is local-only immutable provenance and is excluded from the public Git commit candidate. It is tracked by the external source ZIP hash in `manifest.yml` and is not required to publish the normalized design SSoT. The importer may regenerate it locally; do not interpret that behavior as permission to publish raw Notion pages.

Statuses distinguish design readiness from runtime implementation. A record marked approved here is still not runtime-implemented unless a separate runtime slice says so.

## Known backlog

- `next`/`fallback` are `not_available_in_source`; no links may be invented.
- 127 afterthought references remain source-unstructured and require human review.
- Live Notion page URLs/IDs were unavailable in the export; export-derived IDs are provenance only.
- Legacy inventory contains 7 rows whose canonical candidacy remains undecided.

## Prohibited actions

- Do not declare the runtime graph canonical from this directory.
- Do not delete or rewrite the preserved export.
- Do not treat Notion as a competing canonical source.
- Do not modify runtime/generated/game code as part of design-source maintenance.

## Short Notion mirror notice

> 2026-08-02부터 현재 디자인 레코드의 정본 범위는 repo의 `docs/content/design_source/`입니다. Notion은 읽기·검수 미러이며, runtime graph나 generated bundle의 정본을 의미하지 않습니다. 변경은 로컬 검토 후 Notion에 미러링합니다.

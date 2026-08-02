# WP-L5 Git Readiness Deep Audit

Date: 2026-08-02
Scope: design-source migration files only; no staging, commit, push, runtime, or Notion mutation.

## Verdict

`ready_to_commit` for the normalized design records/governance/tools allowlist, with `source_pages/` explicitly excluded by `.gitignore`. The local-only snapshot contains internal Notion design material and remains outside the public commit candidate.

## Evidence

- HEAD at prior audit preflight: `b1afbc51daf37e650e1532294ae8f03cd23f62b0`
- WP-L6 start HEAD: `b3ed0342c678556e80e406449794628b29915e18`
- WP-L6 end HEAD: `b3ed0342c678556e80e406449794628b29915e18` (unchanged during this follow-up)
- Intervening commit: `b3ed034 feat(web): reveal core log lines at their simulation tick`; only Web combat files, no design-source or governance files.
- Branch: `claude/combat-wave3-step1d3`
- Design-source inventory: 606 files, approximately 3.8M.
- `tools/story_design/` size: approximately 72K.
- Markdown over 100KB: none.
- Runtime/game/generated files were outside this audit scope.
- ZIP was not found inside the design-source tree.
- Original ZIP SHA-256 before/after: `c85b646ef73476c00dc72c418687752daca3d07eb81caaa0ed1a290999062e42`.

## Findings

### [RESOLVED] Internal export snapshot needs publication approval

- File: `docs/content/design_source/source_pages/` (exported Markdown/CSV tree)
- Trigger: committing the full loss-preserving Notion export to a public repository.
- Impact: internal planning text, Notion export metadata, and provenance may become public even though no credentials were detected.
- Action: `.gitignore` now excludes `docs/content/design_source/source_pages/`; the normalized records remain the public Git design SSoT and the export hash remains the provenance anchor.

### [MEDIUM] Absolute provenance paths are machine-local

- Files: `docs/content/design_source/manifest.yml`, `docs/content/design_source/reports/migration_readiness.md`
- Trigger: another machine consumes the committed metadata.
- Impact: source provenance is external to the repo and should not expose local layout.
- Action: applied safe fix: manifest now uses `external/notion_extract.zip`; readiness uses `docs/content/design_source/`.

### [INFO] Cache files are ignored

- File: `.gitignore:2`
- Trigger: test/import execution creates `__pycache__/*.pyc`.
- Impact: no commit drift; `git check-ignore` confirmed both cache samples are ignored.
- Action: none.

## Security/privacy pass

Pattern scan found only false-positive domain terms such as badge/bearer and provenance words. No API key, token, password, private key, email, phone, address, resident-number, or financial value was printed or identified. The export snapshot remains an internal-content publication risk, not a detected-secret finding.

## Reproducibility pass

Fresh output was generated outside the repo at `/tmp/design-source-audit-wp-l5` without overwriting the canonical tree. Counts were 154 events, 18 afterthoughts, 51 rewards, 29 reward mappings, and 1 legacy inventory file; the fresh validator returned PASS.

## Commit policy

Allowlist after owner approval: `AGENTS.md`, `idea_box/README.md`, `idea_box/IDEA_INTAKE_GUIDE.md`, `docs/content/design_source/manifest.yml`, `docs/content/design_source/MIGRATION_GOVERNANCE.md`, `docs/content/design_source/reports/`, `docs/content/design_source/schema/`, `docs/content/design_source/events/`, `afterthoughts/`, `rewards/`, `reward_mappings/`, `legacy_rewards/`, `arcs/`, `source_pages/`, and `tools/story_design/` only when intentionally reviewed.

Denylist for this migration: runtime/game/generated code, `.claude/worktrees/`, combat WIP files, unrelated UI changes, ZIP files, `__pycache__/`, and any file containing unapproved private material.

## Verification

- `python3 tools/story_design/validate_design_source.py --root docs/content/design_source` → PASS
- `pytest -q tools/story_design/tests/test_validate_design_source.py` → 3 passed
- `pytest -q tests/test_docs_contract.py` → 60 passed
- `git remote -v` / `gh repo view --json visibility,isPrivate,nameWithOwner` → public `dudupunch0-sketch/tui_adv`

No commit, push, stage, cleanup, or destructive operation was performed.

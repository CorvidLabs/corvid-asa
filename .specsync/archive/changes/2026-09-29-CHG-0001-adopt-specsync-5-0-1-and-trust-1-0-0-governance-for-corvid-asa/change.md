---
id: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-corvid-asa
state: archived
type: migration
base_commit: bda14bf64a68960e8603390b4b2b0bd2dee13465
---

# Adopt SpecSync 5.0.1 and Trust 1.0.0 governance for Corvid ASA

## Intent

Adopt SpecSync 5.0.1 and Trust 1.0.0 governance for Corvid ASA

## Affected Canonical Specs

- None

## Acceptance Criteria

- SpecSync advisory coverage passes; all four agent integrations are installed; Trust doctor passes; deterministic validation confirms every required HTML, CSS, metadata, and favicon artifact is non-empty; hosted Trust passes on pull requests and main pushes.

## No-spec Rationale

This migration adds governance configuration and CI orchestration without changing static-site content or behavior; future meaningful site changes must add or update canonical specifications.

## Migration Note

Migrated by hand to SpecSync 6 per Leif's decision (2026-09-28); the 6.0.0 tool refused to archive this legacy record (`` exact-only delivery input `.github/workflows/trust.yml` changed after acceptance and requires an audited reopen; run `specsync change reopen CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-corvid-asa` to re-verify the accepted change, or supersede it from a later change under a module granted the path by `owns` in `.specsync/config.toml` ``).

- Workflow v1 (SpecSync 5) record, accepted on 2026-07-13 by the closing approval already stored in `approvals.json`. SpecSync 6.0.0 reports its accepted evidence as stale, for the reason quoted above.
- Moved by hand from `.specsync/changes/CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-corvid-asa/` into the layout `specsync change archive` writes: `accepted-state.json` is the unchanged accepted `state.json`, `state.json` is marked `archived`, and this file's front matter says `archived`.
- `approvals.json`, `verification.json` and every other artifact are the original SpecSync 5 evidence, unchanged. `verification.json` verifies commit `780e3aa3dab8092c63a8603d4883f72d55512f20`, not the tree this record was archived from.
- There is no `verification-attempts.json`: SpecSync 5 did not write one for this record, and this migration does not invent attempt history.
- This migration added no verification evidence, test result, attempt history, or approval. It is a manual migration, not a fresh re-verification.
- Closing it through the tool takes `specsync change reopen`, `specsync change verify`, then `specsync change accept`, which writes a new closing approval. Per Leif's decision it was archived by hand instead, so no reopen or new approval is recorded.

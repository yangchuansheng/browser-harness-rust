# Roadmap: Browser Harness Rust Upstream Sync

**Created:** 2026-05-14
**Mode:** mvp

## Overview

Replicate all applicable upstream `browser-use/browser-harness` changes after 2026-04-21 into the Rust implementation through four phases: inventory, runtime migration, knowledge/docs migration, and verification closeout.

## Phases

### Phase 1: Upstream Change Inventory
**Goal:** Build a complete, auditable map of upstream commits and applicability decisions.
**Mode:** mvp
**Requirements:** UPST-01, UPST-02, UPST-03
**Success Criteria**:
- All upstream commits after 2026-04-21 are listed with hash, date, subject, and changed-file summary.
- Commit groups are classified into runtime/helper/browser discovery/IPC/CLI/testing/docs/domain-skill categories.
- No-op decisions are documented for Python-only changes that do not map to Rust.

### Phase 2: Runtime and API Migration
**Goal:** Port applicable runtime, helper, browser-discovery, IPC, and CLI behavior into Rust.
**Mode:** mvp
**Requirements:** RUNT-01, RUNT-02, RUNT-03, RUNT-04
**Success Criteria**:
- `bh-discovery`, `bh-daemon`, `bh-cdp`, `bh-protocol`, `bh-wasm-host`, `bh-guest-sdk`, and CLI binaries reflect applicable upstream behavior.
- Unit tests cover new parsing, discovery, IPC path, endpoint precedence, and helper behavior where feasible.
- Existing CLI surfaces continue to compile and expose migrated capabilities.

### Phase 3: Knowledge and Documentation Migration
**Goal:** Mirror applicable upstream domain/interaction/setup documentation into this repo's canonical layout.
**Mode:** mvp
**Requirements:** KNOW-01, KNOW-02, KNOW-03
**Success Criteria**:
- New and updated upstream domain skills are present under `domains/` with repo-appropriate filenames.
- `README.md`, `install.md`, `SKILL.md`, and `interaction-skills/` reflect applicable upstream guidance without contradicting the Rust runtime.
- Domain-skill opt-in/location semantics are clear.

### Phase 4: Verification and Closeout
**Goal:** Verify behavior, document traceability, and commit the completed sync.
**Mode:** mvp
**Requirements:** VERI-01, VERI-02, VERI-03, VERI-04
**Success Criteria**:
- Formatting and workspace tests pass.
- Targeted CLI smoke checks pass or environment-dependent checks are explicitly documented.
- Secret scan passes for generated docs and migrated files.
- Migration audit and project state are updated with final status.

## Requirement Coverage

| Requirement | Phase |
|-------------|-------|
| UPST-01 | Phase 1 |
| UPST-02 | Phase 1 |
| UPST-03 | Phase 1 |
| RUNT-01 | Phase 2 |
| RUNT-02 | Phase 2 |
| RUNT-03 | Phase 2 |
| RUNT-04 | Phase 2 |
| KNOW-01 | Phase 3 |
| KNOW-02 | Phase 3 |
| KNOW-03 | Phase 3 |
| VERI-01 | Phase 4 |
| VERI-02 | Phase 4 |
| VERI-03 | Phase 4 |
| VERI-04 | Phase 4 |

**Coverage:** 14/14 v1 requirements mapped.

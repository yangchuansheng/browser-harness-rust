# Requirements: Browser Harness Rust Upstream Sync

**Defined:** 2026-05-14
**Core Value:** The Rust implementation must preserve behavior parity with upstream Browser Harness updates while remaining idiomatic, typed, and maintainable in the existing Rust architecture.

## v1 Requirements

### Upstream Analysis

- [ ] **UPST-01**: Maintainer can see a complete inventory of upstream `browser-use/browser-harness` commits after 2026-04-21.
- [ ] **UPST-02**: Each upstream commit group is classified by applicability to the Rust project.
- [ ] **UPST-03**: Non-applicable Python-only changes are recorded with rationale instead of silently ignored.

### Runtime Parity

- [ ] **RUNT-01**: Rust browser discovery includes applicable upstream additions for Flatpak, Brave, Comet, Arc, Dia, Chrome Canary, Helium, Snap Chromium, and related DevTools probing behavior.
- [ ] **RUNT-02**: Rust daemon and control-plane behavior incorporates applicable upstream IPC/runtime-dir hardening, stale CDP detection, restart safety, and explicit endpoint precedence.
- [ ] **RUNT-03**: Rust runner/helper APIs incorporate applicable upstream helper behavior for JS evaluation, tab/session handling, screenshot limits, form helpers, and network idle/event waits.
- [ ] **RUNT-04**: CLI/docs behavior reflects applicable upstream changes such as reload/restart semantics, setup guidance, and removed/deprecated execution paths.

### Knowledge Parity

- [ ] **KNOW-01**: Applicable upstream domain-skill additions and updates after 2026-04-21 are mirrored into this repo's `domains/` layout.
- [ ] **KNOW-02**: Applicable interaction-skill and setup documentation updates are mirrored into `interaction-skills/`, `install.md`, `README.md`, and related docs.
- [ ] **KNOW-03**: Domain-skill opt-in and location semantics are documented consistently for this repo.

### Verification

- [ ] **VERI-01**: Rust code is formatted with `cargo fmt`.
- [ ] **VERI-02**: Rust workspace tests pass with `cargo test --workspace` or failures are fixed before completion.
- [ ] **VERI-03**: Targeted CLI smoke checks confirm updated command surfaces are reachable.
- [ ] **VERI-04**: Generated planning and migration audit files are scanned for secret-like tokens before commit.

## v2 Requirements

### Extended Parity

- **EXT-01**: Add live browser smoke coverage for every migrated runtime behavior that requires local/remote Chrome.
- **EXT-02**: Build automated upstream-drift checks to alert when `browser-use/browser-harness` adds new post-sync commits.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Python package layout migration | This repo is a Rust reimplementation, not a Python package fork. |
| Exact upstream commit replay | Rust adaptation may require grouping multiple upstream changes into idiomatic Rust commits. |
| Unverified live cloud browser behavior without credentials | Requires `BROWSER_USE_API_KEY`; offline tests should cover logic where possible. |
| Blind copy of upstream `agent-workspace/` structure | This repo uses `domains/` as the canonical knowledge tree. |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| UPST-01 | Phase 1 | Pending |
| UPST-02 | Phase 1 | Pending |
| UPST-03 | Phase 1 | Pending |
| RUNT-01 | Phase 2 | Pending |
| RUNT-02 | Phase 2 | Pending |
| RUNT-03 | Phase 2 | Pending |
| RUNT-04 | Phase 2 | Pending |
| KNOW-01 | Phase 3 | Pending |
| KNOW-02 | Phase 3 | Pending |
| KNOW-03 | Phase 3 | Pending |
| VERI-01 | Phase 4 | Pending |
| VERI-02 | Phase 4 | Pending |
| VERI-03 | Phase 4 | Pending |
| VERI-04 | Phase 4 | Pending |

**Coverage:**
- v1 requirements: 14 total
- Mapped to phases: 14
- Unmapped: 0 ✓

---
*Requirements defined: 2026-05-14*
*Last updated: 2026-05-14 after initial definition*

# Browser Harness Rust Upstream Sync

## What This Is

This project is a Rust-native reimplementation of `browser-use/browser-harness`. The current work is a brownfield sync effort: replicate all applicable upstream behavior from `browser-use/browser-harness` commits after April 21, 2026 into the Rust runtime without forcing the Python repository layout onto this codebase.

The audience is agents and developers who need the Browser Harness thesis as a durable Rust runtime: typed CLIs, daemon/control-plane crates, reusable domain knowledge, and optional WASM guest workflows.

## Core Value

The Rust implementation must preserve behavior parity with upstream Browser Harness updates while remaining idiomatic, typed, and maintainable in the existing Rust architecture.

## Requirements

### Validated

- ✓ Rust workspace provides a daemon/runtime split with `bhd`, `bhctl`, `bhrun`, and `browser-harness` — existing
- ✓ CDP-backed browser operations are exposed through typed protocol crates and runner commands — existing
- ✓ Domain knowledge lives under `domains/<site>/skill.md` and interaction mechanics under `interaction-skills/` — existing
- ✓ WASM guest scaffolding exists for packaged workflows — existing

### Active

- [ ] Inventory upstream commits after 2026-04-21 and classify each as runtime, helper, browser discovery, IPC, CLI, testing, documentation, or domain-skill change.
- [ ] Port every applicable runtime/helper/browser-discovery/IPC/CLI behavior into the Rust implementation.
- [ ] Mirror applicable domain-skill and interaction-skill documentation updates into this repo's `domains/` and `interaction-skills/` layout.
- [ ] Preserve explicit traceability from upstream commit groups to Rust changes, including no-op decisions for Python-only changes.
- [ ] Verify the Rust workspace after migration with formatting, tests, and targeted CLI checks.

### Out of Scope

- Replacing the Rust crate/binary architecture with upstream Python `src/browser_harness` layout — this repo intentionally remains Rust-native.
- Implementing Python packaging, Python-only test structure, or Python module relocation directly — migrate behavior only when relevant.
- Blindly copying upstream agent-workspace paths when this repo already uses `domains/` as the canonical knowledge tree.
- Forcing exact commit-by-commit history replay — behavior parity and traceability matter more than identical patch boundaries.

## Context

- Current repo: the current worktree
- Upstream repo: `https://github.com/browser-use/browser-harness`
- Baseline date: 2026-04-21
- Selected strategy: full behavior sync. Review all upstream commits since the baseline and migrate applicable updates.
- Codebase map exists under `.planning/codebase/` and documents the current Rust architecture.
- The upstream Python project moved to a `src/browser_harness` layout, added Windows/runtime-dir IPC hardening, new browser discovery paths, helper APIs, JS evaluation fixes, screenshot/debug features, docs changes, and many domain-skill additions.

## Constraints

- **Architecture**: Preserve the Rust workspace and crate boundaries — the migration must adapt upstream behavior into existing Rust crates and binaries.
- **Traceability**: Maintain a migration audit with upstream commit references and applicability decisions.
- **Safety**: Do not expose secrets from upstream docs or local environment; scan generated docs and changed files before commit.
- **Verification**: Run `cargo fmt --check`, `cargo test --workspace`, and targeted CLI smoke checks where possible.
- **Network dependency**: Upstream commit analysis depends on the fetched `upstream/main` remote.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Full behavior sync | User selected strategy 1: migrate all applicable upstream updates since 2026-04-21. | — Pending |
| Rust-native adaptation over exact file parity | Upstream is Python; this repo is a Rust systems rewrite. Exact layout parity would break the project identity. | — Pending |
| Use `.planning/codebase/` as current-state baseline | Codebase map was generated before project initialization and gives downstream planning agents architecture context. | ✓ Good |
| Treat Python-only packaging/test refactors as traceable no-ops unless behavior maps to Rust | Avoid importing irrelevant Python structure while preserving auditability. | — Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `$gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `$gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-05-14 after initialization*

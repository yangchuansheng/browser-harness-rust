# Codebase Structure

**Analysis Date:** 2026-05-14

## Directory Layout

```text
browser-harness-rust/
├── README.md                         # Project overview, comparison, quick start, docs index
├── SKILL.md                          # Agent-facing harness usage guide and design constraints
├── install.md                        # First-time install/browser bootstrap guide
├── CHANGELOG.md                      # User-facing project change history
├── LICENSE                           # MIT license
├── .env.example                      # Environment variable template; do not read real .env files
├── .github/workflows/ci.yml          # CI: sensitive scan, Rust tests, install, CLI checks
├── .planning/codebase/               # Generated codebase maps for GSD commands
├── docs/                             # Architecture, development, WASM, Python integration docs/assets
├── domains/                          # Site-specific browser workflow knowledge packages
├── interaction-skills/               # Reusable browser interaction mechanics
├── rust/                             # Active Rust workspace: binaries, crates, guests, lockfile
│   ├── Cargo.toml                    # Workspace manifest
│   ├── Cargo.lock                    # Workspace lockfile
│   ├── README.md                     # Rust workspace command reference
│   ├── bins/                         # Executable packages
│   │   ├── browser-harness-cli/      # Public CLI facade package, binary name `browser-harness`
│   │   ├── bhctl/                    # Admin/control-plane CLI
│   │   ├── bhrun/                    # Runner/helper/guest CLI
│   │   ├── bhd/                      # Daemon binary
│   │   └── bhsmoke/                  # Smoke verification runner
│   ├── crates/                       # Shared runtime, protocol, SDK, transport libraries
│   │   ├── bh-cdp/                   # CDP WebSocket client
│   │   ├── bh-daemon/                # Daemon runtime and Unix socket server
│   │   ├── bh-discovery/             # CDP discovery and runtime paths
│   │   ├── bh-guest-sdk/             # Rust guest SDK wrappers
│   │   ├── bh-protocol/              # Daemon protocol and meta constants
│   │   ├── bh-remote/                # Browser Use API client
│   │   └── bh-wasm-host/             # WASM host DTOs, manifest, filters
│   └── guests/                       # WAT examples and Rust-to-WASM guest packages
└── scripts/
    └── scan_sensitive.sh             # Secret/local path leak scan used by CI
```

## Directory Purposes

**`rust/`:**
- Purpose: Active runtime implementation and workspace metadata.
- Contains: `rust/Cargo.toml`, `rust/Cargo.lock`, `rust/bins/`, `rust/crates/`, `rust/guests/`.
- Key files: `rust/Cargo.toml`, `rust/README.md`.

**`rust/bins/browser-harness-cli/`:**
- Purpose: Public installed CLI facade.
- Contains: `Cargo.toml`, `src/main.rs`.
- Key files: `rust/bins/browser-harness-cli/src/main.rs`.
- Use for: command routing, install, verify-install only. Do not put browser operation implementation here.

**`rust/bins/bhctl/`:**
- Purpose: Admin/control plane.
- Contains: Browser Use lifecycle commands, profile-use wrappers, daemon lifecycle commands.
- Key files: `rust/bins/bhctl/src/main.rs`.
- Use for: new browser lifecycle/admin/profile/daemon commands.

**`rust/bins/bhrun/`:**
- Purpose: Typed runner/helper commands and WASM guest host execution.
- Contains: command parser, typed helper wrappers, event wait loops, HTTP utility, raw CDP, `GuestRuntime`, `serve-guest`.
- Key files: `rust/bins/bhrun/src/main.rs`.
- Use for: new user-facing browser helper commands, wait utilities, and guest operation dispatch.

**`rust/bins/bhd/`:**
- Purpose: Daemon process wrapper.
- Contains: environment-to-config setup, runtime file initialization/cleanup, remote browser stop on exit.
- Key files: `rust/bins/bhd/src/main.rs`.
- Use for: process lifecycle only; core daemon behavior belongs in `rust/crates/bh-daemon/src/lib.rs`.

**`rust/bins/bhsmoke/`:**
- Purpose: Smoke verification runner for live/runtime scenarios.
- Contains: local and remote smoke cases, guest builds/runs, CLI command orchestration.
- Key files: `rust/bins/bhsmoke/src/main.rs`.
- Use for: new acceptance scenarios that need a real browser, CLI install surface, or guest workflow validation.

**`rust/crates/bh-protocol/`:**
- Purpose: Shared daemon wire protocol.
- Contains: `DaemonRequest`, `DaemonResponse`, `PROTOCOL_VERSION`, `META_*` constants.
- Key files: `rust/crates/bh-protocol/src/lib.rs`.
- Use for: adding or renaming daemon meta commands and response fields.

**`rust/crates/bh-daemon/`:**
- Purpose: Daemon runtime core.
- Contains: `DaemonConfig`, `DaemonState`, socket server, CDP attach/session logic, event buffer, helper handlers, raw CDP forwarding, shutdown helpers.
- Key files: `rust/crates/bh-daemon/src/lib.rs`.
- Use for: implementing daemon-side behavior for new meta commands and browser state handling.

**`rust/crates/bh-cdp/`:**
- Purpose: Chrome DevTools Protocol WebSocket transport.
- Contains: `CdpClient`, `CdpEvent`, `send_raw`, pending response demux, event parsing.
- Key files: `rust/crates/bh-cdp/src/lib.rs`.
- Use for: CDP transport changes only; helper semantics belong in `bh-daemon` or `bhrun`.

**`rust/crates/bh-discovery/`:**
- Purpose: Local browser endpoint and runtime path discovery.
- Contains: `RuntimePaths`, `runtime_paths`, `default_browser_profiles`, `get_ws_url`, `is_internal_url`.
- Key files: `rust/crates/bh-discovery/src/lib.rs`.
- Use for: adding supported browser profile locations or runtime path behavior.

**`rust/crates/bh-remote/`:**
- Purpose: Browser Use cloud API client.
- Contains: `BrowserUseClient`, browser create/list/stop, profile listing/resolution, CDP URL resolution.
- Key files: `rust/crates/bh-remote/src/lib.rs`.
- Use for: Browser Use API behavior only.

**`rust/crates/bh-wasm-host/`:**
- Purpose: Shared typed operation contracts for runner and guests.
- Contains: `RunnerConfig`, `HostManifest`, operation request/response structs, `GuestServeRequest`, `GuestServeResponse`, `EventFilter`, filter constructors/matchers.
- Key files: `rust/crates/bh-wasm-host/src/lib.rs`.
- Use for: adding typed request/response shapes and manifest operation metadata.

**`rust/crates/bh-guest-sdk/`:**
- Purpose: Rust SDK for WASM guests.
- Contains: `GuestError`, `call_json`, helper wrappers such as `goto`, `js`, `wait_for_response`, `screenshot`.
- Key files: `rust/crates/bh-guest-sdk/src/lib.rs`.
- Use for: exposing host operations to Rust guest authors.

**`rust/guests/`:**
- Purpose: Guest workflow examples and packaged site/workflow automations.
- Contains: small WAT examples (`rust/guests/*.wat`) and Rust guest crates (`rust/guests/rust-*/`).
- Key files: `rust/guests/navigate_and_read.wat`, `rust/guests/persistent_counter.wat`, `rust/guests/rust-navigate-and-read/src/lib.rs`, `rust/guests/rust-github-trending/src/lib.rs`, `rust/guests/rust-reddit-post-scrape/src/lib.rs`.
- Use for: new repeatable workflows that should be compiled to WASM and run through `bhrun`.

**`domains/`:**
- Purpose: Active site-specific Browser Harness knowledge.
- Contains: `domains/<site>/skill.md`, optional side notes such as `domains/github/repo-actions.md`, `domains/facebook/groups.md`, `domains/medium/article-hydration.md`.
- Key files: `domains/README.md`, `domains/github/skill.md`, `domains/reddit/skill.md`, `domains/spotify/skill.md`.
- Use for: selectors, APIs, waits, traps, URL patterns, and durable site workflow notes.

**`interaction-skills/`:**
- Purpose: Reusable browser mechanics that apply across sites.
- Contains: one Markdown guide per mechanic.
- Key files: `interaction-skills/network-requests.md`, `interaction-skills/tabs.md`, `interaction-skills/screenshots.md`, `interaction-skills/uploads.md`, `interaction-skills/viewport.md`.
- Use for: cross-site techniques such as network waits, screenshots, dialogs, downloads, iframes, shadow DOM, scrolling, and tabs.

**`docs/`:**
- Purpose: Project-level technical and integration docs.
- Contains: architecture, development, WASM guest, future WASM, Python integration docs plus images.
- Key files: `docs/architecture.md`, `docs/development.md`, `docs/wasm-guests.md`, `docs/future-wasm.md`, `docs/python-integration.md`.
- Use for: stable design/usage documentation; do not duplicate runtime source truth.

**`scripts/`:**
- Purpose: Repository maintenance helpers.
- Contains: sensitive content scan script.
- Key files: `scripts/scan_sensitive.sh`.
- Use for: checks invoked locally or by CI.

**`.github/workflows/`:**
- Purpose: GitHub Actions CI.
- Contains: `ci.yml`.
- Key files: `.github/workflows/ci.yml`.
- Use for: repository verification pipeline changes.

**`.planning/codebase/`:**
- Purpose: Generated codebase maps consumed by GSD planning/execution commands.
- Contains: `ARCHITECTURE.md`, `STRUCTURE.md`, and peer mapper outputs.
- Key files: `.planning/codebase/ARCHITECTURE.md`, `.planning/codebase/STRUCTURE.md`.
- Use for: generated analysis only; do not put runtime source here.

## Key File Locations

**Entry Points:**
- `rust/bins/browser-harness-cli/src/main.rs`: public `browser-harness` binary, command routing, install/verify-install.
- `rust/bins/bhctl/src/main.rs`: admin CLI command dispatch.
- `rust/bins/bhrun/src/main.rs`: runner CLI command dispatch, helper execution, guest runtime.
- `rust/bins/bhd/src/main.rs`: daemon binary entry point.
- `rust/bins/bhsmoke/src/main.rs`: smoke runner entry point.
- `rust/guests/rust-*/src/lib.rs`: Rust guest `#[no_mangle] extern "C" fn run() -> i32` entry points.

**Configuration:**
- `rust/Cargo.toml`: Rust workspace members, workspace dependencies, edition/license/version.
- `rust/Cargo.lock`: locked dependency graph for the Rust workspace.
- `.env.example`: environment variable template; note existence only for real `.env*` files.
- `.github/workflows/ci.yml`: CI verification workflow.
- `rust/guests/rust-*/Cargo.toml`: standalone guest crate manifests with `[workspace]` to keep guests outside the main workspace.

**Core Logic:**
- `rust/crates/bh-daemon/src/lib.rs`: daemon socket server, state, meta handlers, CDP helper implementation.
- `rust/crates/bh-cdp/src/lib.rs`: CDP WebSocket transport.
- `rust/crates/bh-discovery/src/lib.rs`: CDP endpoint discovery and runtime paths.
- `rust/crates/bh-protocol/src/lib.rs`: daemon protocol constants and request/response DTOs.
- `rust/crates/bh-wasm-host/src/lib.rs`: typed operation DTOs, manifest, guest serve protocol, event filters.
- `rust/crates/bh-guest-sdk/src/lib.rs`: guest SDK wrapper functions.
- `rust/crates/bh-remote/src/lib.rs`: Browser Use API integration.

**Testing and Verification:**
- `rust/bins/bhsmoke/src/main.rs`: live/local/remote smoke scenarios.
- Inline `#[cfg(test)]` modules: unit tests are embedded in `rust/crates/*/src/lib.rs` and `rust/bins/*/src/main.rs`.
- `.github/workflows/ci.yml`: runs `./scripts/scan_sensitive.sh`, `cargo test --workspace --manifest-path rust/Cargo.toml`, install, CLI checks, and `browser-harness verify-install`.
- `scripts/scan_sensitive.sh`: sensitive content and local path leak scanning.

**Documentation:**
- `README.md`: project overview, comparison, quick start, project structure.
- `SKILL.md`: agent-facing usage guide and architecture constraints.
- `docs/architecture.md`: concise runtime architecture summary.
- `docs/development.md`: common commands and verification paths.
- `docs/wasm-guests.md`: guest model and build/run flow.
- `docs/python-integration.md`: supported thin Python subprocess pattern.
- `domains/README.md`: domain package rules and guest-backed site map.

## Naming Conventions

**Files:**
- Rust packages use kebab-case directory/package names: `bh-daemon`, `bh-wasm-host`, `browser-harness-cli`.
- Rust source entry files are conventional `src/main.rs` for binaries and `src/lib.rs` for crates/guests.
- Guest crates use `rust-<site-or-workflow>` directories and package names ending in `-guest`: `rust/guests/rust-github-trending/Cargo.toml`.
- Guest library crate names use snake_case with `_guest`: `rust_github_trending_guest`, `rust_reddit_post_scrape_guest`.
- Domain packages use lowercase kebab-case directories with `skill.md`: `domains/open-library/skill.md`, `domains/package-registries/skill.md`.
- Interaction guides use lowercase kebab-case Markdown: `interaction-skills/network-requests.md`, `interaction-skills/drag-and-drop.md`.
- Generated planning docs use uppercase Markdown names: `.planning/codebase/ARCHITECTURE.md`, `.planning/codebase/STRUCTURE.md`.

**Directories:**
- Put executable crates under `rust/bins/<binary-package>/`.
- Put shared libraries under `rust/crates/<crate-name>/`.
- Put compiled workflow examples under `rust/guests/rust-<workflow>/` or WAT examples directly under `rust/guests/`.
- Put site docs under `domains/<site>/`.
- Put cross-site browser mechanics under `interaction-skills/`.

**Rust identifiers:**
- Public structs/enums use `PascalCase`: `DaemonConfig`, `DaemonRequest`, `RunnerConfig`, `GuestRunResult`.
- Functions use `snake_case`: `send_daemon_request`, `wait_for_event`, `default_runner_config`.
- Protocol constants use uppercase `META_*`: `META_PAGE_INFO`, `META_WAIT_FOR_LOAD`, `PROTOCOL_VERSION`.
- Environment variable names remain uppercase: `BU_NAME`, `BU_CDP_WS`, `BROWSER_USE_API_KEY`.

## Where to Add New Code

**New public runner/browser helper:**
- Protocol constant: `rust/crates/bh-protocol/src/lib.rs`.
- Request/response DTO and defaults: `rust/crates/bh-wasm-host/src/lib.rs`.
- CLI parser and daemon request wrapper: `rust/bins/bhrun/src/main.rs`.
- Daemon-side implementation if it needs CDP/browser state: `rust/crates/bh-daemon/src/lib.rs`.
- Guest SDK wrapper if guests should use it: `rust/crates/bh-guest-sdk/src/lib.rs`.
- Smoke/unit coverage: inline tests near changed code and, for live behavior, `rust/bins/bhsmoke/src/main.rs`.
- Docs/skills: `SKILL.md`, `docs/wasm-guests.md`, and relevant `interaction-skills/*.md` if the user-facing operation changes.

**New admin/browser lifecycle command:**
- Public route table/help: `rust/bins/browser-harness-cli/src/main.rs` if it should route to `bhctl`.
- Command implementation: `rust/bins/bhctl/src/main.rs`.
- Browser Use API wrapper if needed: `rust/crates/bh-remote/src/lib.rs`.
- Tests: inline `#[cfg(test)]` in `rust/bins/bhctl/src/main.rs` or `rust/crates/bh-remote/src/lib.rs`.
- Docs: `SKILL.md`, `docs/development.md`, or `docs/architecture.md` depending on scope.

**New daemon behavior:**
- Runtime config/state: `rust/crates/bh-daemon/src/lib.rs`.
- CDP transport changes only: `rust/crates/bh-cdp/src/lib.rs`.
- Discovery/path changes: `rust/crates/bh-discovery/src/lib.rs`.
- Process wrapper changes only: `rust/bins/bhd/src/main.rs`.

**New WASM guest operation:**
- Add typed contract: `rust/crates/bh-wasm-host/src/lib.rs`.
- Add runner dispatch/capability behavior: `rust/bins/bhrun/src/main.rs`.
- Add SDK helper: `rust/crates/bh-guest-sdk/src/lib.rs`.
- Add or update sample guest: `rust/guests/rust-<workflow>/src/lib.rs`.
- Update manifest/capabilities expectations and guest docs: `docs/wasm-guests.md`.

**New Rust guest workflow:**
- Implementation: `rust/guests/rust-<site-or-workflow>/Cargo.toml` and `rust/guests/rust-<site-or-workflow>/src/lib.rs`.
- Use `bh-guest-sdk` helpers and export `#[no_mangle] pub extern "C" fn run() -> i32`.
- Keep guest manifests standalone with `[workspace]` as current guests do.
- Build/run examples: add to `rust/README.md` or relevant `domains/<site>/skill.md`.
- Smoke if it is acceptance-critical: `rust/bins/bhsmoke/src/main.rs`.

**New site knowledge:**
- Primary documentation: `domains/<site>/skill.md`.
- Extra durable notes: `domains/<site>/<topic>.md` when a single `skill.md` becomes too large.
- Optional executable workflow: `rust/guests/rust-<site-or-workflow>/`.
- Index update: `domains/README.md`.

**New reusable browser mechanic:**
- Primary documentation: `interaction-skills/<mechanic>.md`.
- Runtime support if missing: add helper via the runner/browser helper path above.
- Cross-reference from `SKILL.md` if it becomes a generally available interaction skill.

**New tests:**
- Unit tests: add inline `#[cfg(test)] mod tests` in the same Rust source file as the logic.
- Live/browser smoke: add a scenario in `rust/bins/bhsmoke/src/main.rs`.
- CI command changes: `.github/workflows/ci.yml`.

**Utilities:**
- Shared runtime helpers: prefer the relevant crate under `rust/crates/`.
- CLI-only helpers: keep inside the owning binary under `rust/bins/<name>/src/main.rs` unless shared by multiple binaries.
- Repository maintenance helpers: `scripts/`.

## Special Directories

**`.planning/codebase/`:**
- Purpose: Generated codebase map consumed by GSD commands.
- Generated: Yes.
- Committed: Depends on workflow; treat as generated analysis and edit only assigned docs during mapper runs.

**`rust/target/`:**
- Purpose: Cargo build output.
- Generated: Yes.
- Committed: No; excluded by `rust/.gitignore`.

**`rust/guests/rust-*/target/`:**
- Purpose: Guest-specific WASM build output.
- Generated: Yes.
- Committed: No; guest `.gitignore` files exclude it.

**`domains/`:**
- Purpose: Durable site-specific Browser Harness knowledge.
- Generated: No.
- Committed: Yes.

**`interaction-skills/`:**
- Purpose: Durable reusable browser interaction knowledge.
- Generated: No.
- Committed: Yes.

**`docs/`:**
- Purpose: Stable project documentation and assets.
- Generated: No.
- Committed: Yes.

**`.github/workflows/`:**
- Purpose: CI automation.
- Generated: No.
- Committed: Yes.

---

*Structure analysis: 2026-05-14*

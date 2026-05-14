# Coding Conventions

**Analysis Date:** 2026-05-14

## Naming Patterns

**Files:**
- Rust workspace code lives under `rust/`, with crate and binary package directories in kebab-case: `rust/crates/bh-wasm-host`, `rust/crates/bh-guest-sdk`, `rust/bins/browser-harness-cli`, `rust/guests/rust-github-trending`.
- Each Rust crate currently uses a flat source entry file rather than a multi-module tree: libraries use `src/lib.rs` such as `rust/crates/bh-protocol/src/lib.rs`; binaries use `src/main.rs` such as `rust/bins/bhrun/src/main.rs`.
- Rust package names use hyphens in `Cargo.toml`, while Rust import paths use underscores. Example: package `bh-wasm-host` in `rust/crates/bh-wasm-host/Cargo.toml` is imported as `bh_wasm_host` in `rust/bins/bhrun/src/main.rs`.
- Guest package directories use the `rust-{domain-or-scenario}` prefix and a `-guest` package name, for example `rust/guests/rust-spotify-search/Cargo.toml` and `rust/guests/rust-spotify-search/src/lib.rs`.

**Functions:**
- Use Rust `snake_case` for functions and methods: `runtime_paths` in `rust/crates/bh-discovery/src/lib.rs`, `resolve_profile_name_in_profiles` in `rust/crates/bh-remote/src/lib.rs`, `wait_for_event_with_drain` in `rust/bins/bhrun/src/main.rs`.
- Command implementation functions mirror CLI command names using underscores: `current_tab`, `wait_for_load`, `wait_for_response`, and `handle_dialog` in `rust/bins/bhrun/src/main.rs`.
- Testable command helpers add dependency injection through suffixes such as `_with_sender` and `_with_drain`: `page_info_with_sender`, `goto_with_sender`, and `watch_events_collect_with_drain` in `rust/bins/bhrun/src/main.rs`.
- WASM guest crates export exactly one ABI entry point as `pub extern "C" fn run() -> i32`, then delegate to a private `run_inner()`: `rust/guests/rust-navigate-and-read/src/lib.rs`, `rust/guests/rust-github-trending/src/lib.rs`.

**Variables:**
- Use `snake_case` for local variables and struct fields: `session_id`, `target_id`, `remote_browser_id` in `rust/crates/bh-daemon/src/lib.rs`; `daemon_name`, `guest_module`, `granted_operations` in `rust/crates/bh-wasm-host/src/lib.rs`.
- Use uppercase environment-variable strings and keep their exact names in errors and config paths: `BU_CDP_WS` in `rust/crates/bh-discovery/src/lib.rs`, `BROWSER_USE_API_KEY` in `rust/bins/bhctl/src/main.rs`, `BU_BROWSER_MODE` in `rust/bins/bhsmoke/src/main.rs`.
- Use `Value` for untyped JSON at browser/daemon boundaries, then convert to typed request and response structs as early as practical: `DaemonRequest` in `rust/crates/bh-protocol/src/lib.rs`, `RunnerConfig` in `rust/crates/bh-wasm-host/src/lib.rs`.

**Types:**
- Use `PascalCase` for structs and enums: `DaemonRequest` in `rust/crates/bh-protocol/src/lib.rs`, `BrowserUseClient` in `rust/crates/bh-remote/src/lib.rs`, `GuestServeRequest` in `rust/crates/bh-wasm-host/src/lib.rs`.
- Use semantic suffixes for protocol DTOs:
  - `*Request`: inbound command payloads, for example `GotoRequest`, `WaitForEventRequest`, `SetViewportRequest` in `rust/crates/bh-wasm-host/src/lib.rs`.
  - `*Result`: command outputs, for example `WaitResult`, `CurrentSessionResult`, `SwitchTabResult` in `rust/crates/bh-wasm-host/src/lib.rs`.
  - `*Response`: daemon or serving envelopes, for example `DaemonResponse` in `rust/crates/bh-protocol/src/lib.rs` and `GuestServeResponse` in `rust/crates/bh-wasm-host/src/lib.rs`.
- Use `SCREAMING_SNAKE_CASE` for constants and protocol meta names: `PROTOCOL_VERSION`, `META_PAGE_INFO`, and `META_WAIT_FOR_LOAD` in `rust/crates/bh-protocol/src/lib.rs`; `DEFAULT_EVENT_CAPACITY` in `rust/crates/bh-daemon/src/lib.rs`.

## Code Style

**Formatting:**
- Format Rust code with:
```bash
cargo fmt --all --manifest-path rust/Cargo.toml
```
- The formatting command is documented in `docs/development.md`; no custom `rustfmt.toml` or `.rustfmt.toml` is present, so use standard rustfmt defaults.
- The active Rust workspace uses edition `2021` through `[workspace.package]` in `rust/Cargo.toml`.

**Linting:**
- No Clippy configuration file is detected: `clippy.toml` and `.clippy.toml` are not present.
- CI does not run `cargo clippy`; `.github/workflows/ci.yml` runs `cargo test --workspace --manifest-path rust/Cargo.toml`, CLI entry-point checks, install verification, and `scripts/scan_sensitive.sh`.
- Treat `cargo test --workspace --manifest-path rust/Cargo.toml` plus `./scripts/scan_sensitive.sh` as the enforced quality gate unless a future change adds Clippy to `.github/workflows/ci.yml`.

**Serde and JSON shape:**
- Use `#[serde(default)]` for backward-compatible request fields and optional response fields: `DaemonRequest` in `rust/crates/bh-protocol/src/lib.rs`, `WaitForEventRequest` in `rust/crates/bh-wasm-host/src/lib.rs`.
- Use `skip_serializing_if = "Option::is_none"` for optional protocol fields so JSON output stays compact: `DaemonResponse` in `rust/crates/bh-protocol/src/lib.rs`, `CookieParam` in `rust/crates/bh-wasm-host/src/lib.rs`.
- Use explicit serde renames for external camelCase fields while keeping Rust fields snake_case, for example `#[serde(rename = "targetId")] pub target_id` in `rust/crates/bh-wasm-host/src/lib.rs`.
- Use internally tagged enums for protocol envelopes where the JSON discriminant is part of the wire format: `#[serde(tag = "command", rename_all = "snake_case")]` on `GuestServeRequest` and `#[serde(tag = "kind", rename_all = "snake_case")]` on `GuestServeResponse` in `rust/crates/bh-wasm-host/src/lib.rs`.

## Import Organization

**Order:**
1. Standard-library imports first: `std::fs`, `std::path::PathBuf`, `std::time::Duration` in `rust/crates/bh-discovery/src/lib.rs` and `rust/bins/browser-harness-cli/src/main.rs`.
2. Workspace crate imports next when the file depends on local packages: `bh_protocol`, `bh_wasm_host`, `bh_daemon`, `bh_remote` in `rust/bins/bhrun/src/main.rs` and `rust/bins/bhctl/src/main.rs`.
3. External crate imports after local or standard groups: `serde_json::{json, Value}`, `tokio::runtime::Builder`, `wasmtime::{Engine, Linker, Module}` in `rust/bins/bhrun/src/main.rs`.

**Path Aliases:**
- No custom Rust path aliases are configured.
- Use Cargo package names from `rust/Cargo.toml` and import them with Rust crate identifiers: `bh-protocol` becomes `bh_protocol`, `bh-wasm-host` becomes `bh_wasm_host`, and `bh-guest-sdk` becomes `bh_guest_sdk`.
- Guest crates import the SDK directly from `bh_guest_sdk`, for example `rust/guests/rust-event-waits-sdk/src/lib.rs` and `rust/guests/rust-producthunt-homepage/src/lib.rs`.

## Error Handling

**Patterns:**
- Application and CLI layers generally return `Result<T, String>` with contextual messages from `map_err` and `ok_or_else`: `request_json` in `rust/crates/bh-remote/src/lib.rs`, `send_raw` in `rust/crates/bh-cdp/src/lib.rs`, `read_json_stdin` in `rust/bins/bhctl/src/main.rs`.
- CLI `main` functions print a single user-facing error and exit non-zero: `rust/bins/browser-harness-cli/src/main.rs`, `rust/bins/bhctl/src/main.rs`, `rust/bins/bhrun/src/main.rs`.
- Prefer adding operation-specific context to errors:
```rust
serde_json::to_string(&output).map_err(|err| format!("serialize bhctl output: {err}"))?;
```
  This pattern appears in `rust/bins/bhctl/src/main.rs`.
- Use custom error types where the crate is a public API boundary. `GuestError` in `rust/crates/bh-guest-sdk/src/lib.rs` implements `Display` and `std::error::Error`.
- WASM guests use numeric exit codes at the ABI boundary. Keep detailed branching inside `run_inner() -> Result<(), i32>` and expose only `run() -> i32`, as in `rust/guests/rust-github-trending/src/lib.rs`.
- For non-critical CDP cleanup or best-effort event delivery, the code explicitly discards errors with `let _ = ...`: `mark_session`, `unmark_session`, and event sending paths in `rust/crates/bh-daemon/src/lib.rs` and `rust/crates/bh-cdp/src/lib.rs`.

## Logging

**Framework:** console and file logging

**Patterns:**
- CLI-facing errors use `eprintln!` in `main`: `rust/bins/browser-harness-cli/src/main.rs`, `rust/bins/bhctl/src/main.rs`, `rust/bins/bhrun/src/main.rs`.
- Daemon runtime logging writes to runtime log files derived from `RuntimePaths`: `runtime_paths` in `rust/crates/bh-discovery/src/lib.rs`, `log_line` and `log_tail` usage in `rust/crates/bh-daemon/src/lib.rs`.
- JSON-producing commands print machine-readable output to stdout and keep errors on stderr: `run` in `rust/bins/bhctl/src/main.rs`, `write_json` callers in `rust/bins/bhrun/src/main.rs`.
- Do not introduce a logging framework unless a module has a cross-cutting need; existing crates use plain `eprintln!`, JSON stdout, and daemon log files.

## Comments

**When to Comment:**
- The Rust source contains very few line comments or doc comments. Prefer self-describing type and function names over explanatory comments for straightforward logic.
- Comments are appropriate for complex embedded browser scripts or protocol caveats, but keep them inside the nearest implementation file such as `rust/guests/rust-2048-autoplay/src/lib.rs`.
- User-facing and architectural explanations belong in Markdown docs such as `docs/development.md`, `docs/architecture.md`, and `rust/README.md`, not as long comments in `src/lib.rs` or `src/main.rs`.

**JSDoc/TSDoc:**
- Not applicable; this repo is Rust-first.
- Rustdoc comments are not currently used as a dominant pattern in `rust/crates/*/src/lib.rs` or `rust/bins/*/src/main.rs`. Add Rustdoc only for new public APIs that need stable consumer guidance.

## Function Design

**Size:** Existing implementation files are flat and several are large (`rust/bins/bhrun/src/main.rs`, `rust/bins/bhsmoke/src/main.rs`, `rust/crates/bh-wasm-host/src/lib.rs`). New code should follow the existing helper-function pattern but avoid adding unrelated responsibilities to these files.

**Parameters:** 
- Prefer typed request structs for commands and protocol operations: `CurrentTabRequest`, `ListTabsRequest`, `WaitForResponseRequest` in `rust/crates/bh-wasm-host/src/lib.rs`.
- Prefer borrowing for simple string inputs: `stop_browser(&self, browser_id: &str)` in `rust/crates/bh-remote/src/lib.rs`, `goto(url: &str)` in `rust/crates/bh-guest-sdk/src/lib.rs`.
- Use generic closure parameters to make daemon, drain, and host-call boundaries testable without network or socket access: `current_tab_with_sender` and `wait_for_event_with_drain` in `rust/bins/bhrun/src/main.rs`.

**Return Values:** 
- Use typed results for stable public APIs: `CurrentSessionResult`, `WaitForEventResult`, `WatchEventsLine` in `rust/crates/bh-wasm-host/src/lib.rs`.
- Use `serde_json::Value` for dynamic browser/CDP payloads and protocol pass-throughs: `CdpClient::send_raw` in `rust/crates/bh-cdp/src/lib.rs`, `page_info_result` in `rust/crates/bh-daemon/src/lib.rs`.
- Use `Result<T, String>` for CLI-internal fallible helpers and attach context at the failure site: `build_install_report` in `rust/bins/browser-harness-cli/src/main.rs`, `daemon_launch_command` in `rust/bins/bhctl/src/main.rs`.

## Module Design

**Exports:** 
- Library crates expose their public API from `src/lib.rs`: `rust/crates/bh-protocol/src/lib.rs`, `rust/crates/bh-remote/src/lib.rs`, `rust/crates/bh-wasm-host/src/lib.rs`.
- Binary crates keep helpers private inside `src/main.rs` and expose behavior through CLI subcommands: `rust/bins/bhrun/src/main.rs`, `rust/bins/bhctl/src/main.rs`, `rust/bins/browser-harness-cli/src/main.rs`.
- `bh-guest-sdk` re-exports host protocol types with `pub use bh_wasm_host::{...}` in `rust/crates/bh-guest-sdk/src/lib.rs`; keep SDK consumers on `bh_guest_sdk` rather than importing host internals directly.

**Barrel Files:** 
- No separate barrel modules are present. `src/lib.rs` is the crate API root for each library.
- No `mod`-split source tree is currently used under `rust/crates/*/src/`; adding a new module should be paired with a clear responsibility boundary and a `mod` declaration in that crate’s `src/lib.rs`.

---

*Convention analysis: 2026-05-14*

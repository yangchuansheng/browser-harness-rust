# Technology Stack

**Analysis Date:** 2026-05-14

## Languages

**Primary:**
- Rust 2021 edition - active runtime, CLI, daemon, protocol, CDP client, WASM host, and smoke runner live in `rust/Cargo.toml`, `rust/bins/*/src/main.rs`, and `rust/crates/*/src/lib.rs`.
- Rust package version `0.1.0` - inherited from `[workspace.package]` in `rust/Cargo.toml` by workspace members such as `rust/crates/bh-protocol/Cargo.toml` and `rust/bins/browser-harness-cli/Cargo.toml`.

**Secondary:**
- WebAssembly Text (WAT) - minimal guest samples are stored in `rust/guests/navigate_and_read.wat`, `rust/guests/persistent_counter.wat`, and `rust/guests/persistent_browser_state.wat`.
- Rust-to-WASM guest crates - packaged guest workflows live under `rust/guests/rust-*/Cargo.toml` and compile as `cdylib` for `wasm32-unknown-unknown`.
- Markdown - operator docs and knowledge packages are first-class artifacts in `SKILL.md`, `install.md`, `docs/*.md`, `domains/<site>/skill.md`, and `interaction-skills/*.md`.
- Shell/YAML - repository maintenance and CI are defined in `scripts/scan_sensitive.sh` and `.github/workflows/ci.yml`.
- Python - not an active runtime layer; only optional subprocess wrapper guidance exists in `docs/python-integration.md`.

## Runtime

**Environment:**
- Rust stable toolchain - CI installs stable via `.github/workflows/ci.yml`; no `rust-toolchain` file is detected in the repo.
- Native CLI runtime - `browser-harness` routes commands to `bhctl` or `bhrun` in `rust/bins/browser-harness-cli/src/main.rs`.
- Long-lived daemon runtime - `bhd` owns the browser websocket and session state through `rust/bins/bhd/src/main.rs` and `rust/crates/bh-daemon/src/lib.rs`.
- Unix socket IPC - daemon files are `/tmp/bu-<name>.sock`, `/tmp/bu-<name>.pid`, and `/tmp/bu-<name>.log` from `rust/crates/bh-discovery/src/lib.rs`; `bhrun` connects through `std::os::unix::net::UnixStream` in `rust/bins/bhrun/src/main.rs`.
- Browser runtime - local Chrome, Chromium, and Microsoft Edge profiles are discovered via `DevToolsActivePort` in `rust/crates/bh-discovery/src/lib.rs`; remote Browser Use sessions are supported through `rust/crates/bh-remote/src/lib.rs`.
- WASM guest runtime - `bhrun run-guest` and `bhrun serve-guest` execute `.wat`/`.wasm` guests through Wasmtime in `rust/bins/bhrun/src/main.rs` and `rust/crates/bh-wasm-host/src/lib.rs`.

**Package Manager:**
- Cargo - workspace root is `rust/Cargo.toml`; all runtime crates and binaries are Cargo workspace members.
- Lockfile: present at `rust/Cargo.lock`; use it as the dependency source of truth for resolved crate versions.

## Frameworks

**Core:**
- Tokio `1.52.1` - async runtime and Unix listener support for daemon/control paths in `rust/Cargo.toml`, `rust/crates/bh-daemon/src/lib.rs`, and `rust/bins/bhctl/src/main.rs`.
- Wasmtime `30.0.2` - WASM guest execution engine used by `rust/bins/bhrun/src/main.rs` and declared in `rust/Cargo.toml`.
- tokio-tungstenite `0.24.0` - CDP websocket transport in `rust/crates/bh-cdp/src/lib.rs`.
- reqwest `0.12.28` with `json` and `rustls-tls` - Browser Use API client and runner `http-get` implementation in `rust/crates/bh-remote/src/lib.rs` and `rust/bins/bhrun/src/main.rs`.
- serde `1.0.228` and serde_json `1.0.149` - JSON protocol serialization across `rust/crates/bh-protocol/src/lib.rs`, `rust/crates/bh-wasm-host/src/lib.rs`, and all CLI payload handling.

**Testing:**
- Rust built-in test harness - run all unit tests with `cargo test --workspace --manifest-path rust/Cargo.toml` as documented in `docs/development.md` and `.github/workflows/ci.yml`.
- `bhsmoke` smoke runner - browser/live-site verification scenarios live in `rust/bins/bhsmoke/src/main.rs` and are documented in `rust/README.md`.
- GitHub Actions - CI executes secret scanning, Rust tests, install verification, and CLI entry-point checks in `.github/workflows/ci.yml`.

**Build/Dev:**
- Cargo build/install - `browser-harness install` builds `browser-harness`, `bhctl`, `bhrun`, and `bhd` from `rust/bins/browser-harness-cli/src/main.rs` into `$CARGO_HOME/bin` or a supplied install root.
- rustup WASM target - guest builds require `wasm32-unknown-unknown` as documented in `rust/README.md`, `docs/wasm-guests.md`, and `docs/future-wasm.md`.
- Formatting - use `cargo fmt --all --manifest-path rust/Cargo.toml` from `docs/development.md`.
- Secret hygiene - use `./scripts/scan_sensitive.sh`, invoked by `.github/workflows/ci.yml`.

## Key Dependencies

**Critical:**
- `tokio` `1.52.1` - async networking, daemon listener, timers, and CLI admin flows; declared in `rust/Cargo.toml`.
- `serde` `1.0.228` / `serde_json` `1.0.149` - all daemon, runner, Browser Use API, and guest-host messages are JSON; declared in `rust/Cargo.toml`.
- `tokio-tungstenite` `0.24.0` and `futures-util` `0.3.32` - CDP websocket send/receive loops in `rust/crates/bh-cdp/src/lib.rs`.
- `reqwest` `0.12.28` - Browser Use API calls in `rust/crates/bh-remote/src/lib.rs` and generic HTTP GET operations in `rust/bins/bhrun/src/main.rs`.
- `wasmtime` `30.0.2` - guest module loading and host imports in `rust/bins/bhrun/src/main.rs`.

**Infrastructure:**
- `libc` `0.2.185` - Unix process/socket support for daemon lifecycle paths in `rust/crates/bh-daemon/Cargo.toml`.
- `base64` `0.22.1` - smoke verification utilities in `rust/bins/bhsmoke/Cargo.toml`.
- `rustls` / `webpki-roots` - TLS stack pulled by `reqwest` and `tokio-tungstenite`, resolved in `rust/Cargo.lock`.
- Internal workspace crates - use `bh-protocol`, `bh-discovery`, `bh-cdp`, `bh-daemon`, `bh-remote`, `bh-wasm-host`, and `bh-guest-sdk` from `rust/crates/*/Cargo.toml` instead of adding duplicate protocol or browser layers.

## Configuration

**Environment:**
- `BU_NAME` - daemon namespace for socket, pid, and log paths; used in `rust/bins/bhd/src/main.rs`, `rust/bins/bhctl/src/main.rs`, and `rust/crates/bh-discovery/src/lib.rs`.
- `BU_CDP_WS` - explicit CDP websocket override for remote browsers or pinned local attach; read by `rust/crates/bh-discovery/src/lib.rs`.
- `BU_BROWSER_ID` and `BROWSER_USE_API_KEY` - remote Browser Use lifecycle cleanup inputs; read by `rust/bins/bhd/src/main.rs` and `rust/bins/bhctl/src/main.rs`.
- `BU_BROWSER_MODE`, `BU_DAEMON_IMPL`, `BU_REMOTE_TIMEOUT_MINUTES`, and `BU_LOCAL_DAEMON_WAIT_SECONDS` - smoke-runner scenario controls in `rust/bins/bhsmoke/src/main.rs`.
- `BU_RUST_DAEMON_BIN`, `BU_RUST_ADMIN_BIN`, and `BU_RUST_RUNNER_BIN` - binary override hooks in `rust/bins/bhctl/src/main.rs`, `rust/bins/bhrun/src/main.rs`, and `rust/bins/browser-harness-cli/src/main.rs`.
- `CARGO_HOME`, `CARGO`, `HOME`, `USERPROFILE`, `HOMEDRIVE`, `HOMEPATH`, and `PATH` - installer and binary discovery inputs in `rust/bins/browser-harness-cli/src/main.rs`.
- `.env.example` is present at `.env.example`; contents were not read because environment files are treated as secret-bearing.

**Build:**
- Rust workspace config: `rust/Cargo.toml`.
- Resolved dependencies: `rust/Cargo.lock`.
- CI config: `.github/workflows/ci.yml`.
- Install/bootstrap docs: `install.md`, `README.md`, and `rust/README.md`.
- No Node, Python package, Docker, or Go manifests were detected at repo root beyond the Rust workspace.

## Platform Requirements

**Development:**
- Install a stable Rust toolchain with Cargo; CI uses `dtolnay/rust-toolchain@stable` in `.github/workflows/ci.yml`.
- Add `wasm32-unknown-unknown` when building Rust guest crates under `rust/guests/rust-*`, per `rust/README.md`.
- Use a local Chrome, Chromium, or Microsoft Edge profile with remote debugging enabled for local browser verification, per `install.md` and `rust/crates/bh-discovery/src/lib.rs`.
- Use `ripgrep` for CI secret scanning when missing on Ubuntu runners, per `.github/workflows/ci.yml`.

**Production:**
- Deploy as native CLI binaries installed by `browser-harness install` into `$CARGO_HOME/bin` or a supplied root from `rust/bins/browser-harness-cli/src/main.rs`.
- Runtime connects either to a local Chrome/Edge CDP websocket discovered from `DevToolsActivePort` or to Browser Use cloud through `BU_CDP_WS`, `BU_BROWSER_ID`, and `BROWSER_USE_API_KEY`.
- The daemon IPC implementation is Unix-socket based in `rust/crates/bh-daemon/src/lib.rs` and `rust/bins/bhrun/src/main.rs`; treat Unix-like systems as the supported runtime target unless the daemon layer is ported.

---

*Stack analysis: 2026-05-14*

# External Integrations

**Analysis Date:** 2026-05-14

## APIs & External Services

**Browser control / CDP:**
- Local Chrome, Chromium, and Microsoft Edge - browser automation target discovered from `DevToolsActivePort` in `rust/crates/bh-discovery/src/lib.rs`.
  - SDK/Client: custom CDP websocket client in `rust/crates/bh-cdp/src/lib.rs` using `tokio-tungstenite`.
  - Auth: browser-profile permission and Chrome remote-debugging approval; no API key in repo code.
- Pinned or remote CDP websocket - override local discovery by setting `BU_CDP_WS`, read in `rust/crates/bh-discovery/src/lib.rs`.
  - SDK/Client: `CdpClient::connect` in `rust/crates/bh-cdp/src/lib.rs`.
  - Auth: whatever protects the supplied CDP websocket URL; do not commit the URL.

**Browser Use Cloud:**
- Browser Use API - remote browser provisioning, listing, shutdown, and cloud profile lookup use `https://api.browser-use.com/api/v3` from `rust/crates/bh-remote/src/lib.rs`.
  - SDK/Client: custom `BrowserUseClient` in `rust/crates/bh-remote/src/lib.rs` over `reqwest`.
  - Auth: `BROWSER_USE_API_KEY`, sent as `X-Browser-Use-API-Key` in `rust/crates/bh-remote/src/lib.rs`.
- Browser Use cloud browser sessions - `bhctl create-browser` provisions a browser and resolves `cdpUrl` to `cdpWsUrl` through `/json/version` in `rust/bins/bhctl/src/main.rs` and `rust/crates/bh-remote/src/lib.rs`.
  - SDK/Client: `bhctl` admin command in `rust/bins/bhctl/src/main.rs`.
  - Auth: `BROWSER_USE_API_KEY`; runtime cleanup additionally uses `BU_BROWSER_ID` in `rust/bins/bhd/src/main.rs`.

**Profile synchronization:**
- `profile-use` CLI - local profile listing and cookie/profile sync are delegated to the external `profile-use` binary in `rust/bins/bhctl/src/main.rs`.
  - SDK/Client: shell-out via `std::process::Command` in `rust/bins/bhctl/src/main.rs`.
  - Auth: `BROWSER_USE_API_KEY` for cloud sync; local browser profile access is user-machine state.
- Browser Use profile install script - docs reference `https://browser-use.com/profile.sh` in `interaction-skills/profile-sync.md` and `rust/bins/bhctl/src/main.rs`.
  - SDK/Client: external installer, not vendored.
  - Auth: not handled by this repo.

**Generic HTTP and domain knowledge:**
- Runner HTTP GET - `bhrun http-get` and `bh_guest_sdk::http_get` fetch arbitrary URLs through `reqwest` in `rust/bins/bhrun/src/main.rs`, `rust/crates/bh-wasm-host/src/lib.rs`, and `rust/crates/bh-guest-sdk/src/lib.rs`.
  - SDK/Client: `reqwest` with default `Mozilla/5.0` user agent in `rust/bins/bhrun/src/main.rs`.
  - Auth: caller-supplied headers in `HttpGetRequest`; no secret storage in the repo.
- Domain API notes - optional workflow knowledge under `domains/` includes public APIs such as NASA in `domains/nasa/skill.md`, SEC EDGAR in `domains/sec-edgar/skill.md`, World Bank in `domains/world-bank/skill.md`, OpenStreetMap in `domains/openstreetmap/skill.md`, Archive.org/Wayback in `domains/archive-org/skill.md`, Spotify oEmbed/embed in `domains/spotify/skill.md`, and Quora SSR extraction in `domains/quora/skill.md`.
  - SDK/Client: typically `browser-harness http-get`, `bhrun http-get`, or browser operations from `SKILL.md`.
  - Auth: most documented examples are no-auth; site-specific auth requirements are captured in each `domains/<site>/skill.md`.

## Data Storage

**Databases:**
- Not detected - no database client crate, migration directory, schema file, or DB connection string is present in `rust/Cargo.toml`, `rust/crates/*/Cargo.toml`, or repo-level manifests.

**File Storage:**
- Local runtime files - daemon socket, pid, and log files live at `/tmp/bu-<name>.sock`, `/tmp/bu-<name>.pid`, and `/tmp/bu-<name>.log` from `rust/crates/bh-discovery/src/lib.rs`.
- Local install files - `browser-harness install` copies binaries into `$CARGO_HOME/bin` or a supplied install root in `rust/bins/browser-harness-cli/src/main.rs`.
- Browser downloads - download path is caller-configured through `configure-downloads` in `rust/bins/bhrun/src/main.rs`, `rust/crates/bh-wasm-host/src/lib.rs`, and `interaction-skills/downloads.md`.
- Browser profile state - local cookies/session state remain in Chrome or Edge profile directories discovered by `rust/crates/bh-discovery/src/lib.rs`; cloud profile sync is mediated by `profile-use` in `rust/bins/bhctl/src/main.rs`.

**Caching:**
- No distributed cache is detected in `rust/Cargo.toml` or source imports.
- In-memory event buffer - daemon stores recent CDP events in a `VecDeque` with `DEFAULT_EVENT_CAPACITY` in `rust/crates/bh-daemon/src/lib.rs`.
- WASM guest state - `serve-guest` can preserve guest memory during a runner process when `persistent_guest_state=true`, documented in `docs/wasm-guests.md` and implemented in `rust/bins/bhrun/src/main.rs`.

## Authentication & Identity

**Auth Provider:**
- Browser Use API key - `BROWSER_USE_API_KEY` is required for `create-browser`, `list-browsers`, `stop-browser`, cloud profiles, and remote smoke tests in `rust/bins/bhctl/src/main.rs`, `rust/crates/bh-remote/src/lib.rs`, and `rust/bins/bhsmoke/src/main.rs`.
  - Implementation: API key header `X-Browser-Use-API-Key` in `rust/crates/bh-remote/src/lib.rs`.
- Browser session identity - local auth is inherited from the user's Chrome/Edge profile, and remote auth can be carried through Browser Use cloud profiles described in `interaction-skills/profile-sync.md`.
  - Implementation: browser profile discovery in `rust/crates/bh-discovery/src/lib.rs`; profile sync command wiring in `rust/bins/bhctl/src/main.rs`.
- First-party user auth server - Not detected in `rust/Cargo.toml`, `rust/crates/*`, or `rust/bins/*`.

## Monitoring & Observability

**Error Tracking:**
- None - no Sentry, OpenTelemetry, Datadog, Honeycomb, or tracing dependency is detected in `rust/Cargo.toml` or `rust/Cargo.lock`.

**Logs:**
- CLI errors are printed to stderr by `rust/bins/bhctl/src/main.rs`, `rust/bins/bhrun/src/main.rs`, `rust/bins/bhd/src/main.rs`, and `rust/bins/browser-harness-cli/src/main.rs`.
- Daemon log tailing and runtime log paths are handled by `rust/crates/bh-daemon/src/lib.rs` and `rust/crates/bh-discovery/src/lib.rs`.
- CI observability is GitHub Actions job output from `.github/workflows/ci.yml`.

## CI/CD & Deployment

**Hosting:**
- No hosted application deployment is detected; the deliverable is a set of local native binaries installed by `browser-harness install` from `rust/bins/browser-harness-cli/src/main.rs`.
- Optional browser execution can happen in Browser Use cloud, configured through `rust/crates/bh-remote/src/lib.rs` and documented in `README.md`.

**CI Pipeline:**
- GitHub Actions - `.github/workflows/ci.yml` runs on `push` and `pull_request`.
- CI steps - checkout, install `ripgrep`, run `scripts/scan_sensitive.sh`, set up stable Rust, cache Rust artifacts, run `cargo test --workspace --manifest-path rust/Cargo.toml`, install binaries, and verify CLI entry points in `.github/workflows/ci.yml`.

## Environment Configuration

**Required env vars:**
- `BROWSER_USE_API_KEY` - required only for Browser Use cloud and remote smoke scenarios; read in `rust/bins/bhctl/src/main.rs`, `rust/bins/bhd/src/main.rs`, and `rust/bins/bhsmoke/src/main.rs`.
- `BU_CDP_WS` - optional explicit CDP websocket override; read in `rust/crates/bh-discovery/src/lib.rs`.
- `BU_BROWSER_ID` - optional remote browser id used for cleanup on daemon shutdown; read in `rust/bins/bhd/src/main.rs`.
- `BU_NAME` - optional namespace for daemon socket/pid/log files; read in `rust/bins/bhd/src/main.rs`, `rust/bins/bhctl/src/main.rs`, and `rust/crates/bh-discovery/src/lib.rs`.
- `BU_BROWSER_MODE` and `BU_DAEMON_IMPL` - smoke verification controls in `rust/bins/bhsmoke/src/main.rs` and documented in `docs/development.md`.
- `BU_GUEST_PATH`, `BU_SKIP_GUEST_BUILD`, `BU_2048_TARGET`, `BU_REMOTE_TIMEOUT_MINUTES`, and `BU_LOCAL_DAEMON_WAIT_SECONDS` - smoke-runner scenario options in `rust/bins/bhsmoke/src/main.rs`.
- `BU_RUST_DAEMON_BIN`, `BU_RUST_ADMIN_BIN`, and `BU_RUST_RUNNER_BIN` - binary override hooks in `rust/bins/bhctl/src/main.rs` and `rust/bins/browser-harness-cli/src/main.rs`.

**Secrets location:**
- Secrets are expected in process environment variables, not in committed config; source reads env vars directly in `rust/bins/bhctl/src/main.rs`, `rust/bins/bhd/src/main.rs`, and `rust/crates/bh-discovery/src/lib.rs`.
- `.env.example` exists at `.env.example` and was not read; no actual `.env` contents are inspected or documented.

## Webhooks & Callbacks

**Incoming:**
- Not detected - no HTTP server route, webhook endpoint, or callback listener is present in `rust/Cargo.toml`, `rust/bins/*`, or `rust/crates/*`.
- Daemon IPC is local JSON-line Unix socket traffic handled in `rust/crates/bh-daemon/src/lib.rs`, not an inbound web integration.

**Outgoing:**
- Browser Use API requests - `POST /browsers`, `GET /browsers`, `PATCH /browsers/{id}`, `GET /profiles`, and `GET /profiles/{id}` are implemented in `rust/crates/bh-remote/src/lib.rs`.
- CDP websocket messages - JSON CDP requests/events flow through `rust/crates/bh-cdp/src/lib.rs`.
- Arbitrary HTTP GET requests - caller-controlled via `bhrun http-get` in `rust/bins/bhrun/src/main.rs` and `bh_guest_sdk::http_get` in `rust/crates/bh-guest-sdk/src/lib.rs`.
- Browser Use profile sync - external `profile-use sync` process is invoked by `rust/bins/bhctl/src/main.rs`.

---

*Integration audit: 2026-05-14*

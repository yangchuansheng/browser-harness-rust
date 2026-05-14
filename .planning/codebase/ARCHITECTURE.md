<!-- refreshed: 2026-05-14 -->
# Architecture

**Analysis Date:** 2026-05-14

## System Overview

```text
┌─────────────────────────────────────────────────────────────┐
│                    Agent / User Tooling                      │
│ `SKILL.md`, `domains/<site>/skill.md`, `interaction-skills/` │
└──────────────────────────────┬──────────────────────────────┘
                               │ shell / subprocess JSON stdin
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                 Top-level CLI facade                         │
│                 `rust/bins/browser-harness-cli/src/main.rs`  │
├──────────────────────────────┬──────────────────────────────┤
│ Admin/lifecycle route         │ Runner/helper route          │
│ `rust/bins/bhctl/src/main.rs` │ `rust/bins/bhrun/src/main.rs`│
└──────────────┬───────────────┴──────────────┬───────────────┘
               │ JSON admin calls             │ one JSON line per daemon request
               ▼                              ▼
┌──────────────────────────────┐   ┌──────────────────────────┐
│ Browser Use / profile-use     │   │ Unix daemon socket       │
│ `rust/crates/bh-remote/`      │   │ `/tmp/bu-<name>.sock`    │
└──────────────────────────────┘   └──────────────┬───────────┘
                                                   ▼
┌─────────────────────────────────────────────────────────────┐
│                  Browser daemon/runtime                      │
│                  `rust/bins/bhd/src/main.rs`                 │
│                  `rust/crates/bh-daemon/src/lib.rs`          │
└──────────────┬───────────────────────────────┬──────────────┘
               │                               │
               ▼                               ▼
┌──────────────────────────────┐   ┌──────────────────────────┐
│ CDP websocket client          │   │ Runtime discovery/files  │
│ `rust/crates/bh-cdp/src/lib.rs`│  │ `rust/crates/bh-discovery/src/lib.rs` │
└──────────────┬───────────────┘   └──────────────────────────┘
               ▼
┌─────────────────────────────────────────────────────────────┐
│ Chrome / Edge DevTools or Browser Use cloud CDP endpoint     │
│ Local `DevToolsActivePort` or `BU_CDP_WS`                    │
└─────────────────────────────────────────────────────────────┘

Optional guest path:
`rust/guests/rust-*` / `rust/guests/*.wat`
  -> `rust/crates/bh-guest-sdk/src/lib.rs`
  -> WASM import `bh.call_json`
  -> `rust/bins/bhrun/src/main.rs` `GuestRuntime`
  -> same daemon socket and CDP runtime above
```

## Component Responsibilities

| Component | Responsibility | File |
|-----------|----------------|------|
| Top-level `browser-harness` CLI | Routes public commands to `bhctl` for admin/lifecycle or `bhrun` for helper/runner commands; owns `install` and `verify-install`. | `rust/bins/browser-harness-cli/src/main.rs` |
| `bhctl` admin CLI | Creates/lists/stops Browser Use cloud browsers, resolves/syncs profiles through `profile-use`, starts/stops/restarts the daemon, and reports daemon liveness. | `rust/bins/bhctl/src/main.rs` |
| `bhrun` runner CLI | Parses typed JSON command payloads, sends daemon meta/CDP requests, implements wait/http utilities, executes WASM guests, and serves persistent guest sessions. | `rust/bins/bhrun/src/main.rs` |
| `bhd` daemon binary | Reads daemon environment, initializes runtime files, starts `bh-daemon::serve`, stops remote cloud browsers on shutdown, and cleans pid/socket files. | `rust/bins/bhd/src/main.rs` |
| Daemon runtime | Owns CDP client, active session/target state, dialog/event buffer, Unix socket server, helper meta command handlers, and raw CDP forwarding. | `rust/crates/bh-daemon/src/lib.rs` |
| CDP client | Maintains the WebSocket connection, assigns CDP ids, maps responses to pending callers, emits CDP events on a Tokio channel, and identifies browser-level methods. | `rust/crates/bh-cdp/src/lib.rs` |
| Discovery/runtime paths | Resolves `/tmp/bu-<name>.*` runtime paths, discovers local Chrome/Edge `DevToolsActivePort`, filters internal browser URLs, and honors `BU_CDP_WS`. | `rust/crates/bh-discovery/src/lib.rs` |
| Wire protocol | Defines daemon JSON-line request/response shapes and all daemon meta command names. | `rust/crates/bh-protocol/src/lib.rs` |
| Remote Browser Use client | Wraps Browser Use REST APIs for browser lifecycle, CDP websocket URL resolution, and cloud profile lookup. | `rust/crates/bh-remote/src/lib.rs` |
| WASM host contract | Defines runner config, guest serve protocol, operation request/response DTOs, host manifest, event filters, and operation metadata. | `rust/crates/bh-wasm-host/src/lib.rs` |
| Guest SDK | Provides Rust guest helper functions over the imported `bh.call_json` host function. | `rust/crates/bh-guest-sdk/src/lib.rs` |
| Smoke runner | Exercises CLI, daemon, browser actions, tabs, events, guests, remote mode, and site-specific guest workflows. | `rust/bins/bhsmoke/src/main.rs` |
| Domain knowledge | Stores durable site-specific selectors, APIs, waits, traps, and optional guest mapping. | `domains/<site>/skill.md`, `domains/README.md` |
| Interaction knowledge | Stores reusable browser mechanics such as tabs, screenshots, downloads, network waits, dialogs, uploads, scrolling, and viewport rules. | `interaction-skills/*.md` |

## Pattern Overview

**Overall:** Thin CLI facade + long-lived CDP daemon + typed JSON-line helper protocol + capability-gated WASM guest runtime.

**Key Characteristics:**
- Keep `browser-harness` thin. New browser operation commands belong in `bhrun` and daemon meta handling, while browser lifecycle/admin commands belong in `bhctl`.
- Keep `bhd` as the only owner of the live browser websocket and mutable browser session state.
- Use `bh-protocol` constants for meta command names instead of string literals spread across crates.
- Use typed request/response structs in `bh-wasm-host` for public helper and guest surfaces.
- Use guests for packaged repeatable workflows; guests call host operations and never connect to the daemon socket directly.
- Keep site-specific knowledge in `domains/<site>/skill.md`; Rust guests are optional packaged workflows, not the default artifact for every domain.

## Layers

**Documentation and knowledge layer:**
- Purpose: Teaches agents how to use the harness and site-specific browser workflows.
- Location: `SKILL.md`, `domains/`, `interaction-skills/`, `docs/`.
- Contains: domain `skill.md` files, reusable interaction mechanics, architecture/development notes, install guide.
- Depends on: the public CLI surface exposed by `browser-harness`, `bhctl`, `bhrun`, and guest examples under `rust/guests/`.
- Used by: humans and agents before invoking or extending runtime code.

**Facade CLI layer:**
- Purpose: Provides the stable installed command `browser-harness`.
- Location: `rust/bins/browser-harness-cli/src/main.rs`.
- Contains: route tables (`ADMIN_COMMANDS`, `RUNNER_HELP`), process spawning, install/verify-install implementation.
- Depends on: child binaries `bhctl`, `bhrun`, `bhd`; `cargo` fallback; filesystem checks for install verification.
- Used by: all external shell/subprocess callers and documentation examples.

**Admin/control-plane layer:**
- Purpose: Handles daemon lifecycle, cloud browser lifecycle, and profile sync/lookup.
- Location: `rust/bins/bhctl/src/main.rs`, `rust/crates/bh-remote/src/lib.rs`.
- Contains: `create-browser`, `list-browsers`, `stop-browser`, `ensure-daemon`, `restart-daemon`, `list-cloud-profiles`, profile-use wrappers.
- Depends on: `bh-daemon` runtime file helpers, `bh-remote` Browser Use API client, `profile-use` external command.
- Used by: `browser-harness` admin command route and remote browser setup flows.

**Runner/helper layer:**
- Purpose: Provides typed browser operations, wait utilities, HTTP utility, raw CDP escape hatch, and WASM guest execution.
- Location: `rust/bins/bhrun/src/main.rs`, `rust/crates/bh-wasm-host/src/lib.rs`.
- Contains: command dispatch, request normalization, `send_daemon_request`, event wait loops, `GuestRuntime`, capability checks, manifest/capabilities output.
- Depends on: `bh-protocol` daemon messages, `bh-wasm-host` DTOs, `wasmtime`, `reqwest`, Unix sockets.
- Used by: direct `bhrun` calls, `browser-harness` runner command route, Rust/WAT guests through host calls.

**Daemon/runtime layer:**
- Purpose: Owns the single live CDP connection and browser state for a daemon namespace.
- Location: `rust/bins/bhd/src/main.rs`, `rust/crates/bh-daemon/src/lib.rs`.
- Contains: Unix socket listener, per-connection JSON-line handler, active session attachment, event buffer, dialog tracking, helper implementations, raw CDP forwarding, runtime file lifecycle.
- Depends on: `bh-cdp`, `bh-discovery`, `bh-protocol`, `bh-remote`, Tokio.
- Used by: `bhrun` helper calls and guest host calls.

**CDP transport layer:**
- Purpose: Provides asynchronous WebSocket request/response and event demultiplexing for Chrome DevTools Protocol.
- Location: `rust/crates/bh-cdp/src/lib.rs`.
- Contains: `CdpClient`, `CdpEvent`, pending-response map, reader task, `send_raw`, browser-level method classifier.
- Depends on: `tokio-tungstenite`, `futures-util`, `serde_json`, Tokio channels.
- Used by: `bh-daemon` only.

**Discovery and runtime-file layer:**
- Purpose: Resolves CDP endpoints and daemon runtime file paths.
- Location: `rust/crates/bh-discovery/src/lib.rs`.
- Contains: `RuntimePaths`, `/tmp/bu-<name>.sock|pid|log`, local Chrome/Edge profile search, `BU_CDP_WS` override, internal URL filtering.
- Depends on: filesystem, TCP probing, environment variables.
- Used by: `bh-daemon` and indirectly by `bhctl` daemon lifecycle commands.

**Shared protocol/type layer:**
- Purpose: Prevents drift between daemon, runner, and guest surfaces.
- Location: `rust/crates/bh-protocol/src/lib.rs`, `rust/crates/bh-wasm-host/src/lib.rs`.
- Contains: `DaemonRequest`, `DaemonResponse`, `META_*` constants, `RunnerConfig`, operation DTOs, event filters, host manifest.
- Depends on: `serde`, `serde_json`.
- Used by: `bh-daemon`, `bhrun`, `bh-guest-sdk`, tests, guests.

**Guest workflow layer:**
- Purpose: Packages repeatable workflow logic as WASM modules.
- Location: `rust/guests/*.wat`, `rust/guests/rust-*/src/lib.rs`, `rust/crates/bh-guest-sdk/src/lib.rs`.
- Contains: exported `run()` entry points, site/workflow constants, JS extraction scripts, SDK calls, guest exit-code checks.
- Depends on: `bh-guest-sdk`, `serde`, `serde_json`; host capabilities granted by `RunnerConfig`.
- Used by: `bhrun run-guest`, `bhrun serve-guest`, and `bhsmoke` guest scenarios.

**Verification layer:**
- Purpose: Provides repository acceptance and smoke coverage for local/remote runtime behavior.
- Location: `rust/bins/bhsmoke/src/main.rs`, `.github/workflows/ci.yml`, `scripts/scan_sensitive.sh`.
- Contains: smoke scenario orchestration, cargo test workflow, install verification, sensitive content scan.
- Depends on: installed or repo-local CLI binaries, live local/remote browser settings for smoke paths.
- Used by: local development and CI.

## Data Flow

### Primary Runner Command Path

- User/agent invokes `browser-harness page-info` or another runner command (`rust/bins/browser-harness-cli/src/main.rs`).
- `route_command` classifies non-admin commands as runner commands and `spawn_child` starts sibling `bhrun` or `cargo run --bin bhrun` fallback (`rust/bins/browser-harness-cli/src/main.rs`).
- `bhrun` parses stdin JSON into a typed request such as `PageInfoRequest` and normalizes defaults (`rust/bins/bhrun/src/main.rs`, `rust/crates/bh-wasm-host/src/lib.rs`).
- The runner builds `DaemonRequest { meta: Some(META_PAGE_INFO), params: ... }` and calls `send_daemon_request` over `/tmp/bu-<daemon_name>.sock` (`rust/bins/bhrun/src/main.rs`).
- `bh-daemon::handle_stream` parses one JSON line into `DaemonRequest` and calls `Daemon::handle_request` (`rust/crates/bh-daemon/src/lib.rs`).
- `Daemon::page_info_result` evaluates JavaScript in the current session through `Runtime.evaluate` via `send_with_retry` and `CdpClient::send_raw` (`rust/crates/bh-daemon/src/lib.rs`, `rust/crates/bh-cdp/src/lib.rs`).
- `DaemonResponse { result }` is serialized as one JSON line and returned to `bhrun`, which deserializes/prints pretty JSON to stdout (`rust/crates/bh-protocol/src/lib.rs`, `rust/bins/bhrun/src/main.rs`).

### Daemon Startup Path

- User/agent invokes `browser-harness ensure-daemon` (`rust/bins/browser-harness-cli/src/main.rs`).
- The facade routes `ensure-daemon` to `bhctl` because it is listed in `ADMIN_COMMANDS` (`rust/bins/browser-harness-cli/src/main.rs`).
- `bhctl` parses optional JSON `{name, wait, env}`, checks `already_running`, and launches `bhd` using `BU_RUST_DAEMON_BIN`, sibling binary, or `cargo run --bin bhd` fallback (`rust/bins/bhctl/src/main.rs`).
- `bhd` builds `DaemonConfig` from `BU_NAME`, `BU_BROWSER_ID`, and `BROWSER_USE_API_KEY`, writes pid/log files, and calls `serve` (`rust/bins/bhd/src/main.rs`, `rust/crates/bh-daemon/src/lib.rs`).
- `serve` binds `/tmp/bu-<name>.sock`, sets socket mode `0600`, resolves the CDP endpoint through `get_ws_url`, connects `CdpClient`, attaches the first real page, and starts the CDP event loop (`rust/crates/bh-daemon/src/lib.rs`, `rust/crates/bh-discovery/src/lib.rs`, `rust/crates/bh-cdp/src/lib.rs`).
- `bhctl` polls until the socket accepts connections, then returns a JSON status report (`rust/bins/bhctl/src/main.rs`).

### Remote Browser Control Path

- User/agent invokes `browser-harness create-browser` with JSON payload (`rust/bins/browser-harness-cli/src/main.rs`).
- The facade routes the command to `bhctl`; `bhctl` creates `BrowserUseClient` from `BROWSER_USE_API_KEY` (`rust/bins/bhctl/src/main.rs`).
- `normalize_create_browser_payload` resolves `profileName` into `profileId` when present (`rust/bins/bhctl/src/main.rs`, `rust/crates/bh-remote/src/lib.rs`).
- `BrowserUseClient::create_browser` posts to Browser Use, then `cdp_ws_from_url` reads `/json/version` to derive `cdpWsUrl` (`rust/crates/bh-remote/src/lib.rs`).
- Caller starts a daemon with `BU_CDP_WS` and `BU_BROWSER_ID` in `ensure-daemon` env. On shutdown, `bhd` calls `stop_remote` so Browser Use cloud billing/session ends cleanly (`rust/bins/bhd/src/main.rs`, `rust/crates/bh-daemon/src/lib.rs`).

### WASM Guest Run Path

- User/agent invokes `bhrun run-guest <path>` or `browser-harness run-guest <path>` with `RunnerConfig` JSON (`rust/bins/bhrun/src/main.rs`).
- `GuestRuntime::new` loads `.wat` or `.wasm` with Wasmtime, registers the imported host function `bh.call_json`, instantiates the module, and locates exported `run` (`rust/bins/bhrun/src/main.rs`).
- Rust guests call SDK helpers such as `goto`, `wait_for_load`, `js`, or `wait_for_response`; each helper serializes an operation name and request through `call_json` (`rust/crates/bh-guest-sdk/src/lib.rs`).
- The host `dispatch_guest_operation` checks `granted_operations` plus `allow_http`/`allow_raw_cdp` gates, injects `daemon_name` when missing, dispatches to the same runner helper functions, and records `GuestCallRecord` entries (`rust/bins/bhrun/src/main.rs`, `rust/crates/bh-wasm-host/src/lib.rs`).
- `invoke_run` returns `GuestRunResult { exit_code, success, calls, trap }`; `serve-guest` keeps the same runtime and invocation count across NDJSON `run` commands (`rust/bins/bhrun/src/main.rs`).

### Event Wait Path

- The daemon enables Page, DOM, Runtime, Network, Log, and Console domains for each active session (`rust/crates/bh-daemon/src/lib.rs`).
- `CdpClient` forwards CDP events to a Tokio channel; `run_event_loop` calls `Daemon::handle_event` (`rust/crates/bh-cdp/src/lib.rs`, `rust/crates/bh-daemon/src/lib.rs`).
- `handle_event` appends normalized events `{method, params, session_id}` into a bounded `VecDeque` and tracks dialog open/close state (`rust/crates/bh-daemon/src/lib.rs`).
- Runner waits (`wait-for-event`, `wait-for-response`, `wait-for-console`, `watch-events`) repeatedly drain events via `META_DRAIN_EVENTS` and match them with filters from `bh-wasm-host` (`rust/bins/bhrun/src/main.rs`, `rust/crates/bh-wasm-host/src/lib.rs`).

**State Management:**
- Daemon namespace state is keyed by `BU_NAME`, with socket/pid/log at `/tmp/bu-<name>.sock`, `/tmp/bu-<name>.pid`, and `/tmp/bu-<name>.log` (`rust/crates/bh-discovery/src/lib.rs`).
- Live browser state is process-local in `DaemonState { session_id, target_id, dialog, events }`, protected by `Arc<Mutex<_>>` (`rust/crates/bh-daemon/src/lib.rs`).
- Guest state lives inside Wasmtime `Store<GuestHostState>` for one `run-guest` invocation or across multiple invocations in `serve-guest` when `persistent_guest_state=true` (`rust/bins/bhrun/src/main.rs`).
- Site knowledge state is plain Markdown under `domains/` and `interaction-skills/`; it does not mutate at runtime.

## Key Abstractions

**Daemon JSON-line protocol:**
- Purpose: Stable IPC between `bhrun` and `bhd`.
- Examples: `DaemonRequest`, `DaemonResponse`, `META_PAGE_INFO`, `META_GOTO`, `META_DRAIN_EVENTS` in `rust/crates/bh-protocol/src/lib.rs`.
- Pattern: one JSON line request over Unix socket, one JSON line response.

**Daemon namespace:**
- Purpose: Allows independent browser sessions/daemons for parallel agents or remote browsers.
- Examples: `RuntimePaths` in `rust/crates/bh-discovery/src/lib.rs`, `DaemonConfig` in `rust/crates/bh-daemon/src/lib.rs`.
- Pattern: `BU_NAME` defaults to `default` and maps to `/tmp/bu-<name>.sock|pid|log`.

**CDP client:**
- Purpose: Converts concurrent Rust calls into CDP request ids and asynchronous response/event streams.
- Examples: `CdpClient`, `CdpEvent`, `send_raw` in `rust/crates/bh-cdp/src/lib.rs`.
- Pattern: WebSocket writer protected by a mutex; pending id map resolves oneshot responses; events flow through `mpsc`.

**Daemon meta commands:**
- Purpose: Provide ergonomic browser helpers without exposing every caller to raw CDP.
- Examples: `META_GOTO`, `META_JS`, `META_CLICK`, `META_SCREENSHOT`, `META_WAIT_FOR_LOAD` in `rust/crates/bh-protocol/src/lib.rs`; handlers in `Daemon::handle_request` in `rust/crates/bh-daemon/src/lib.rs`.
- Pattern: `request.meta` selects helper; raw `request.method` path remains available for unsupported CDP calls.

**Typed runner operation DTOs:**
- Purpose: Define public JSON command and guest operation shapes once.
- Examples: `GotoRequest`, `JsRequest`, `WaitForEventRequest`, `SetViewportRequest`, `CookieParam` in `rust/crates/bh-wasm-host/src/lib.rs`.
- Pattern: `serde` structs with `Default` and `normalized()` methods for CLI input defaults.

**Guest capability config:**
- Purpose: Restricts what a guest can do at runtime.
- Examples: `RunnerConfig { granted_operations, allow_http, allow_raw_cdp, persistent_guest_state }` in `rust/crates/bh-wasm-host/src/lib.rs`; checks in `dispatch_guest_operation` in `rust/bins/bhrun/src/main.rs`.
- Pattern: allow-list by operation name plus explicit extra gates for HTTP and raw CDP.

**Guest SDK helper surface:**
- Purpose: Lets Rust guests use browser operations without hand-writing host import serialization.
- Examples: `goto`, `js`, `wait_for_response`, `watch_events`, `screenshot` in `rust/crates/bh-guest-sdk/src/lib.rs`.
- Pattern: each helper calls generic `call_json(operation, request)` and deserializes typed responses.

**Domain skills:**
- Purpose: Holds durable per-site workflow knowledge and maps some sites to guest crates.
- Examples: `domains/github/skill.md`, `domains/reddit/skill.md`, `domains/spotify/skill.md`, `domains/README.md`.
- Pattern: knowledge-first Markdown with optional Rust guest under `rust/guests/rust-<site-or-workflow>/`.

## Entry Points

**Installed public CLI:**
- Location: `rust/bins/browser-harness-cli/src/main.rs`.
- Triggers: `browser-harness <command>` from shell, agent tool, or thin subprocess wrapper.
- Responsibilities: route admin vs runner commands; install/verify installed Rust-only binaries.

**Admin CLI:**
- Location: `rust/bins/bhctl/src/main.rs`.
- Triggers: `bhctl ...` directly or `browser-harness` admin route.
- Responsibilities: Browser Use API calls, profile-use integration, daemon lifecycle.

**Runner CLI:**
- Location: `rust/bins/bhrun/src/main.rs`.
- Triggers: `bhrun ...` directly or `browser-harness` runner route.
- Responsibilities: JSON stdin parsing, typed helper calls, waits, HTTP GET, raw CDP, guest runtime.

**Daemon binary:**
- Location: `rust/bins/bhd/src/main.rs`.
- Triggers: launched by `bhctl ensure-daemon`, manually run, or sibling installed binary.
- Responsibilities: process-level daemon setup and cleanup around `bh-daemon::serve`.

**Daemon library server:**
- Location: `rust/crates/bh-daemon/src/lib.rs`.
- Triggers: `bhd` calls `serve`.
- Responsibilities: bind socket, connect CDP, attach page, accept IPC, handle requests/events.

**Smoke runner:**
- Location: `rust/bins/bhsmoke/src/main.rs`.
- Triggers: local `cargo run --bin bhsmoke -- <scenario>` or CI/manual checks.
- Responsibilities: acceptance checks for runtime, CLI, guest, event, tab, remote, and site-specific workflows.

**Rust guest modules:**
- Location: `rust/guests/rust-*/src/lib.rs`.
- Triggers: `bhrun run-guest` or `bhrun serve-guest` loading compiled `.wasm` artifacts.
- Responsibilities: workflow-specific automation with exported `run() -> i32`.

## Architectural Constraints

- **Threading:** `bhd`, `bhctl`, and CDP transport use Tokio; `bhrun` is mostly synchronous and opens a blocking Unix stream per daemon call, but creates a current-thread Tokio runtime for `http_get`.
- **IPC shape:** Daemon IPC is Unix-socket-only and line-oriented. Windows named pipes are not represented in the current runtime code (`rust/bins/bhrun/src/main.rs`, `rust/crates/bh-daemon/src/lib.rs`).
- **Global state:** Runtime state is daemon-process local in `DaemonState`; runtime files are global `/tmp/bu-<name>.*`; command defaults use environment variables such as `BU_NAME`, `BU_CDP_WS`, `BU_BROWSER_ID`, `BROWSER_USE_API_KEY`, `BU_RUST_*`.
- **Session ownership:** Only `bhd` owns CDP sessions. `bhrun`, guests, Python subprocess wrappers, and domain skills must call the CLI/daemon protocol rather than owning websocket state.
- **Event buffer:** `META_DRAIN_EVENTS` is destructive; wait/watch operations consume the daemon event queue. Start waits before triggering an action for reliable matching (`interaction-skills/network-requests.md`, `rust/bins/bhrun/src/main.rs`).
- **Guest boundary:** Guests cannot bypass host checks. `http_get` requires both `granted_operations` and `allow_http`; `cdp_raw` requires both `granted_operations` and `allow_raw_cdp` (`rust/bins/bhrun/src/main.rs`).
- **Local browser discovery:** Local attach reads `DevToolsActivePort` from known Chrome/Edge profile directories and waits for the port; it does not launch a browser (`rust/crates/bh-discovery/src/lib.rs`).
- **Cloud browser shutdown:** Remote cleanup depends on `BU_BROWSER_ID` and `BROWSER_USE_API_KEY` being present when `bhd` exits (`rust/bins/bhd/src/main.rs`, `rust/crates/bh-daemon/src/lib.rs`).
- **Circular imports:** No Rust crate cycle is present in `rust/Cargo.toml`; dependency flow is one-way: binaries -> crates, `bh-daemon` -> `bh-cdp`/`bh-discovery`/`bh-protocol`/`bh-remote`, `bh-guest-sdk` -> `bh-wasm-host` -> `bh-protocol`.
- **File size concentration:** `rust/bins/bhsmoke/src/main.rs`, `rust/bins/bhrun/src/main.rs`, `rust/crates/bh-wasm-host/src/lib.rs`, `rust/crates/bh-daemon/src/lib.rs`, and `rust/crates/bh-guest-sdk/src/lib.rs` contain many responsibilities in single files; when adding code, prefer extracting by responsibility instead of expanding these files further.

## Anti-Patterns

### Adding a second runtime manager

**What happens:** A new supervisor/session manager wraps `bhd`, retries daemon calls, or owns CDP sessions outside the daemon.
**Why it's wrong:** The project explicitly keeps `browser-harness` thin and makes `bhd` the single owner of browser websocket/session state.
**Do this instead:** Add lifecycle commands to `bhctl` (`rust/bins/bhctl/src/main.rs`) or helper operations to `bhrun` + `bh-daemon` (`rust/bins/bhrun/src/main.rs`, `rust/crates/bh-daemon/src/lib.rs`).

### Putting runner helpers in the top-level CLI

**What happens:** `browser-harness` implements browser operations directly.
**Why it's wrong:** The facade route table should remain thin; direct helpers there duplicate `bhrun` and create drift.
**Do this instead:** Add the command to `RUNNER_HELP` only for help text and implement behavior in `bhrun` (`rust/bins/browser-harness-cli/src/main.rs`, `rust/bins/bhrun/src/main.rs`).

### Letting guests talk to sockets or CDP directly

**What happens:** A guest opens `/tmp/bu-*.sock`, uses HTTP/WebSocket CDP clients, or provisions browsers.
**Why it's wrong:** It bypasses capability gates and splits session ownership.
**Do this instead:** Add a typed host operation in `bh-wasm-host`, dispatch it in `bhrun`, and expose it in `bh-guest-sdk` (`rust/crates/bh-wasm-host/src/lib.rs`, `rust/bins/bhrun/src/main.rs`, `rust/crates/bh-guest-sdk/src/lib.rs`).

### Treating domain docs as executable runtime

**What happens:** Python-like helper snippets in `domains/<site>/skill.md` are interpreted as a required Python runtime layer.
**Why it's wrong:** Domain examples are conceptual operation mappings; the active runtime is Rust and optional Python must remain a thin subprocess wrapper.
**Do this instead:** Map helper-style operations to `browser-harness`/`bhrun` commands or a Rust/WASM guest (`domains/README.md`, `docs/python-integration.md`).

### Reading `.env` or secrets into docs

**What happens:** Mapper or implementation code reads local secret files to document environment values.
**Why it's wrong:** The repository includes `.env.example` only as a template; real `.env*` contents must not be read or committed.
**Do this instead:** Document variable names only, such as `BROWSER_USE_API_KEY`, `BU_CDP_WS`, `BU_BROWSER_ID`, and `BU_NAME` (`.env.example`, `docs/architecture.md`).

## Error Handling

**Strategy:** Most binaries return `Result<_, String>` from internal `run` functions, print errors to stderr, and exit nonzero. Protocol errors are carried in `DaemonResponse.error` and converted back into CLI errors by `bhrun`.

**Patterns:**
- Use `map_err(|err| format!("context: {err}"))` around filesystem, process, JSON, network, and CDP operations (`rust/bins/bhrun/src/main.rs`, `rust/bins/bhctl/src/main.rs`, `rust/crates/bh-daemon/src/lib.rs`).
- Daemon helper handlers return `DaemonResponse { error: Some(err) }` rather than panicking (`rust/crates/bh-daemon/src/lib.rs`).
- `send_with_retry` re-attaches once when the current CDP session is stale (`rust/crates/bh-daemon/src/lib.rs`).
- Guest host-call failures set `GuestHostState.error` and return `-1`; `GuestRunResult.trap` carries the failure (`rust/bins/bhrun/src/main.rs`).
- Live smoke and remote operations return JSON status objects or `String` errors rather than throwing exceptions (`rust/bins/bhsmoke/src/main.rs`, `rust/crates/bh-remote/src/lib.rs`).

## Cross-Cutting Concerns

**Logging:** Daemon logs append plain lines to `/tmp/bu-<name>.log` through `log_line`; startup failure surfaces the last non-empty log line through `log_tail` (`rust/crates/bh-daemon/src/lib.rs`, `rust/bins/bhctl/src/main.rs`). Other binaries primarily use stdout JSON and stderr errors.

**Validation:** CLI input validation is local to each command parser. `bh-wasm-host` request `normalized()` methods enforce defaults; `bhctl` validates positive pagination/wait values and env map types; daemon handlers validate required meta params before calling CDP (`rust/crates/bh-wasm-host/src/lib.rs`, `rust/bins/bhctl/src/main.rs`, `rust/crates/bh-daemon/src/lib.rs`).

**Authentication:** Browser Use API authentication uses only the `BROWSER_USE_API_KEY` environment variable inside `bhctl`/`bh-remote`; profile sync shells out to `profile-use`; local browser attach uses the user's already-running browser profile and CDP permission state (`rust/bins/bhctl/src/main.rs`, `rust/crates/bh-remote/src/lib.rs`, `rust/crates/bh-discovery/src/lib.rs`).

**Security / capabilities:** Daemon socket permissions are set to `0600`; guests are operation allow-listed; raw CDP and HTTP are separately gated; `scripts/scan_sensitive.sh` and CI scan for sensitive content before tests (`rust/crates/bh-daemon/src/lib.rs`, `rust/bins/bhrun/src/main.rs`, `.github/workflows/ci.yml`).

**Browser state:** The daemon marks active pages by prefixing the document title with a green circle during load/switch flows; stale current sessions are recovered by re-attaching to the first real page (`rust/crates/bh-daemon/src/lib.rs`).

---

*Architecture analysis: 2026-05-14*

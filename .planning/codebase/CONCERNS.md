# Codebase Concerns

**Analysis Date:** 2026-05-14

## Tech Debt

**Monolithic Rust implementation files:**
- Issue: Several core files carry multiple architectural responsibilities in one source unit: `rust/bins/bhsmoke/src/main.rs` is 6,372 lines, `rust/bins/bhrun/src/main.rs` is 3,913 lines, `rust/crates/bh-wasm-host/src/lib.rs` is 3,125 lines, `rust/crates/bh-daemon/src/lib.rs` is 2,100 lines, and `rust/crates/bh-guest-sdk/src/lib.rs` is 1,597 lines.
- Files: `rust/bins/bhsmoke/src/main.rs`, `rust/bins/bhrun/src/main.rs`, `rust/crates/bh-wasm-host/src/lib.rs`, `rust/crates/bh-daemon/src/lib.rs`, `rust/crates/bh-guest-sdk/src/lib.rs`
- Impact: Small changes require scanning unrelated routing, protocol, test, guest, and smoke code. Merge conflicts and accidental regressions are likely in `bhrun` and `bhsmoke`.
- Fix approach: Split by responsibility: CLI parsing, daemon transport, typed operations, event waits, guest runtime, smoke scenarios, and fixtures. Add `mod` boundaries while keeping public crate APIs stable.

**Operation surface is duplicated across multiple crates:**
- Issue: Adding one browser operation requires edits across protocol constants, daemon handling, runner command parsing, host DTOs, guest dispatch, SDK helpers, manifests, tests, docs, and smokes.
- Files: `rust/crates/bh-protocol/src/lib.rs`, `rust/crates/bh-daemon/src/lib.rs`, `rust/bins/bhrun/src/main.rs`, `rust/crates/bh-wasm-host/src/lib.rs`, `rust/crates/bh-guest-sdk/src/lib.rs`, `rust/bins/bhsmoke/src/main.rs`
- Impact: Drift between CLI operations, guest capabilities, and daemon meta commands can ship silently. `META_*` constants cover daemon helpers, while host utility operations such as `http_get`, `wait_for_event`, and `cdp_raw` live only in runner/host metadata.
- Fix approach: Introduce a single operation registry or generated table that emits protocol constants, manifest entries, CLI command metadata, and guest dispatch wiring.

**Daemon direct protocol silently defaults missing or malformed parameters:**
- Issue: The daemon uses `unwrap_or` defaults for direct meta requests: missing `goto.url` becomes `about:blank`, missing `js.expression` becomes an empty script, missing click coordinates become `(0, 0)`, and missing scroll deltas become a default wheel action.
- Files: `rust/crates/bh-daemon/src/lib.rs`, `rust/crates/bh-wasm-host/src/lib.rs`, `rust/bins/bhrun/src/main.rs`
- Impact: Direct socket clients can trigger unintended browser actions instead of receiving validation errors. Typed `bhrun` commands catch some missing fields, but the daemon socket remains permissive.
- Fix approach: Move validation to the daemon boundary for every meta command and reject absent required fields. Keep defaults only where the public request type explicitly defines safe defaults.

**Event delivery uses destructive global draining:**
- Issue: `drain_events` empties a single daemon-wide `VecDeque<Value>`; wait and watch loops poll by repeatedly draining all events.
- Files: `rust/crates/bh-daemon/src/lib.rs`, `rust/bins/bhrun/src/main.rs`, `rust/crates/bh-wasm-host/src/lib.rs`
- Impact: Two simultaneous waiters compete for the same events. One `wait_for_event`, `watch_events`, or `drain-events` call can consume events another waiter needs. The fixed `DEFAULT_EVENT_CAPACITY` of 500 also drops older events without per-consumer visibility.
- Fix approach: Replace destructive drain with cursor-based reads, per-client subscriptions, or per-session event queues. Preserve events until all active cursors advance or expire.

**Rust guest package settings are inconsistent:**
- Issue: Most Rust-to-WASM guest crates set `panic = "abort"` for dev and release builds, but `rust-etsy-search`, `rust-letterboxd-popular`, and `rust-spotify-search` do not.
- Files: `rust/guests/rust-etsy-search/Cargo.toml`, `rust/guests/rust-letterboxd-popular/Cargo.toml`, `rust/guests/rust-spotify-search/Cargo.toml`, `rust/guests/rust-github-trending/Cargo.toml`
- Impact: Guest binary size and panic behavior differ between examples. WASM trap behavior becomes inconsistent when guest code panics.
- Fix approach: Standardize guest Cargo templates and enforce the profile block through a small verification script or workspace metadata check.

## Known Bugs

**Workspace tests fail to link in the current macOS arm64 environment:**
- Symptoms: `cargo test --workspace --manifest-path rust/Cargo.toml --quiet` fails while linking `bh-daemon` tests with unresolved `___ubsan_handle_*` symbols from `ring` objects and macOS deployment-version warnings.
- Files: `rust/Cargo.toml`, `rust/Cargo.lock`, `rust/crates/bh-daemon/Cargo.toml`, `rust/crates/bh-cdp/Cargo.toml`
- Trigger: Running the full workspace test command on the current arm64 macOS toolchain.
- Workaround: Use the GitHub Actions Linux CI path as the current reliable gate, or run narrower test packages that avoid the affected TLS/link path. For local macOS, inspect `RUSTFLAGS`, sanitizer flags, Xcode SDK, and `ring` build artifacts.

**`Runtime.evaluate` errors and unserializable values can collapse to `null`:**
- Symptoms: `js_result` extracts only `result.value` from CDP output and returns `Value::Null` when the value is absent; it does not surface `exceptionDetails`, `objectId`, or preview details.
- Files: `rust/crates/bh-daemon/src/lib.rs`, `rust/bins/bhrun/src/main.rs`, `rust/crates/bh-guest-sdk/src/lib.rs`
- Trigger: Evaluating JavaScript that throws, returns an unserializable object, returns a remote object without `value`, or hits a CDP serialization edge case.
- Workaround: Write scripts that explicitly return JSON strings or primitives. Prefer `JSON.stringify(...)` in guest extraction scripts such as `rust/guests/rust-producthunt-homepage/src/lib.rs` and `rust/guests/rust-etsy-search/src/lib.rs`.

**`page_info` changes shape when a dialog is pending:**
- Symptoms: When `DaemonState.dialog` is set, `page_info_result` returns only `{"dialog": ...}` instead of the normal URL/title/viewport payload.
- Files: `rust/crates/bh-daemon/src/lib.rs`, `rust/bins/bhrun/src/main.rs`, `rust/crates/bh-wasm-host/src/lib.rs`
- Trigger: Calling `page-info` while a JavaScript dialog is open.
- Workaround: Call `pending_dialog` or `handle-dialog` first where possible. Consumers that call `page_info` must tolerate the dialog-only response.

**Remote profile listing performs N+1 Browser Use API requests:**
- Symptoms: `list_cloud_profiles` fetches a profile listing and then fetches each profile detail individually.
- Files: `rust/crates/bh-remote/src/lib.rs`, `rust/bins/bhctl/src/main.rs`
- Trigger: `bhctl list-cloud-profiles` or `bhctl resolve-profile-name` with many cloud profiles.
- Workaround: Use exact `profileId` when known. For implementation, cache detail results or use a Browser Use endpoint that returns required fields in the listing if available.

## Security Considerations

**Runtime files in `/tmp` are vulnerable to namespace and file-race problems:**
- Risk: Runtime paths are predictable (`/tmp/bu-<name>.sock`, `.pid`, `.log`), `BU_NAME` is not sanitized, pid/log files are created with default file permissions, and pid-based shutdown sends `SIGTERM` to the pid read from the file.
- Files: `rust/crates/bh-discovery/src/lib.rs`, `rust/crates/bh-daemon/src/lib.rs`, `rust/bins/bhd/src/main.rs`, `rust/bins/bhctl/src/main.rs`
- Current mitigation: The Unix socket is chmodded to `0600` after binding in `rust/crates/bh-daemon/src/lib.rs`.
- Recommendations: Store runtime files under a per-user `0700` runtime directory, sanitize `BU_NAME`, create files with restrictive permissions, avoid following symlinks, and verify pid ownership/executable before signalling.

**Daemon logs can expose sensitive browser connection material:**
- Risk: `serve` logs the full CDP endpoint, and shutdown logs remote browser identifiers. CDP websocket URLs can contain bearer-like connection material depending on provider.
- Files: `rust/crates/bh-daemon/src/lib.rs`, `rust/bins/bhd/src/main.rs`, `rust/crates/bh-discovery/src/lib.rs`
- Current mitigation: Runtime logs are local files and `*.log` is ignored by `.gitignore`.
- Recommendations: Redact websocket query strings, credentials, and browser ids before logging. Set daemon log permissions to `0600` at creation.

**Local socket clients can control the attached browser without application-level authentication:**
- Risk: Any same-user process that can connect to `/tmp/bu-<name>.sock` can issue `js`, `cdp_raw`, `get_cookies`, `set_cookies`, `upload_file`, `configure_downloads`, and other high-impact operations.
- Files: `rust/crates/bh-daemon/src/lib.rs`, `rust/crates/bh-protocol/src/lib.rs`, `rust/bins/bhrun/src/main.rs`
- Current mitigation: Socket permissions are set to `0600`. WASM guests are separately capability-gated by `RunnerConfig` in `rust/crates/bh-wasm-host/src/lib.rs`.
- Recommendations: Document the local same-user trust model, add optional per-daemon auth tokens, and keep dangerous operations behind explicit capability or confirmation layers for non-guest clients.

**Guest `http_get` allows arbitrary outbound requests when enabled:**
- Risk: A guest with `allow_http=true` and `granted_operations=["http_get"]` can request arbitrary URLs, attach custom headers, and read the entire response body into memory.
- Files: `rust/bins/bhrun/src/main.rs`, `rust/crates/bh-wasm-host/src/lib.rs`, `rust/crates/bh-guest-sdk/src/lib.rs`
- Current mitigation: `allow_http` is a separate boolean gate in addition to the operation allow-list.
- Recommendations: Add scheme restrictions, optional host allow-lists, response-size caps, redirect policy controls, and header deny-lists for sensitive names such as authorization and cookie headers.

**Guest call records can leak cookies, headers, and page data:**
- Risk: `GuestRunResult.calls` stores full normalized requests and responses for every guest host call. Cookie operations, HTTP responses, JavaScript extraction results, screenshots, and PDFs can be emitted to stdout or smoke reports.
- Files: `rust/crates/bh-wasm-host/src/lib.rs`, `rust/bins/bhrun/src/main.rs`, `rust/crates/bh-guest-sdk/src/lib.rs`, `rust/bins/bhsmoke/src/main.rs`
- Current mitigation: No redaction layer is present.
- Recommendations: Redact cookie values, headers, raw CDP payloads, screenshots, PDFs, and large text bodies in `GuestCallRecord` by default. Add an explicit debug flag for full traces.

**Environment-driven binary overrides execute arbitrary programs:**
- Risk: `BU_RUST_ADMIN_BIN`, `BU_RUST_RUNNER_BIN`, and `BU_RUST_DAEMON_BIN` override child binary paths; `ensure-daemon` also accepts an `env` map and passes it to the daemon process.
- Files: `rust/bins/browser-harness-cli/src/main.rs`, `rust/bins/bhctl/src/main.rs`, `rust/bins/bhsmoke/src/main.rs`
- Current mitigation: These are local developer escape hatches and are not read from network input by the code itself.
- Recommendations: Treat JSON payloads and environment as trusted local input only. Print the resolved binary path in verbose/debug mode and document that wrappers must not pass untrusted payloads to `ensure-daemon`.

**Hard-coded third-party API key material exists in smoke/domain fixtures:**
- Risk: Metacritic smoke and domain documentation include backend URLs with an `apiKey` query parameter. Even if the key is public or site-owned, the workflow depends on a value that can rotate or be blocked.
- Files: `rust/bins/bhsmoke/src/main.rs`, `domains/metacritic/skill.md`
- Current mitigation: `scripts/scan_sensitive.sh` scans common secret patterns, but it does not classify this site-specific query key.
- Recommendations: Move site-specific public keys into domain documentation with rotation notes, avoid printing the key in failure reports, and add a detector for known sensitive query names where practical.

## Performance Bottlenecks

**Large artifacts are transported as in-memory base64 JSON strings:**
- Problem: Screenshots and PDFs are returned as base64 strings from the daemon, read into a `Vec<u8>` by `bhrun`, then serialized again for CLI or guest output.
- Files: `rust/crates/bh-daemon/src/lib.rs`, `rust/bins/bhrun/src/main.rs`, `rust/crates/bh-guest-sdk/src/lib.rs`
- Cause: `screenshot_result`, `print_pdf_result`, and `send_daemon_request` use whole-payload JSON responses. The guest SDK allocates an 8 MiB buffer for every host call.
- Improvement path: Add file-output modes, streaming/chunked artifact transfer, response-size limits, and operation-specific output caps.

**WASM guest execution has no visible CPU or memory budget:**
- Problem: `GuestRuntime` creates a default Wasmtime engine and store without fuel, epoch interruption, memory limits, or per-run timeout controls.
- Files: `rust/bins/bhrun/src/main.rs`, `rust/crates/bh-wasm-host/src/lib.rs`
- Cause: The current runtime prioritizes a simple preview guest runner.
- Improvement path: Configure Wasmtime fuel or epoch deadlines, memory/table limits, max invocation duration, and clean trap reporting for runaway guests.

**Polling wait loops repeatedly drain daemon events:**
- Problem: Event waits poll by sleeping and draining the daemon event queue, with a default 200 ms interval.
- Files: `rust/bins/bhrun/src/main.rs`, `rust/crates/bh-wasm-host/src/lib.rs`, `rust/crates/bh-daemon/src/lib.rs`
- Cause: Event delivery is pull-based over the existing one-request Unix socket protocol.
- Improvement path: Add blocking event wait support inside the daemon or a streaming subscription socket so waits do not require repeated client-side polling and destructive drains.

**Browser Use profile resolution scales poorly with profile count:**
- Problem: Resolving a profile name can require listing pages and fetching each profile detail.
- Files: `rust/crates/bh-remote/src/lib.rs`, `rust/bins/bhctl/src/main.rs`
- Cause: `resolve_profile_name` calls `list_cloud_profiles`, which performs one detail request per listed profile.
- Improvement path: Cache profile listings during one command, expose `profileId` usage as the preferred path, and avoid detail calls when listing fields are sufficient.

## Fragile Areas

**Live-site guests and smokes depend on changing third-party DOMs and APIs:**
- Files: `rust/bins/bhsmoke/src/main.rs`, `rust/guests/rust-github-trending/src/lib.rs`, `rust/guests/rust-reddit-post-scrape/src/lib.rs`, `rust/guests/rust-producthunt-homepage/src/lib.rs`, `rust/guests/rust-letterboxd-popular/src/lib.rs`, `rust/guests/rust-spotify-search/src/lib.rs`, `rust/guests/rust-etsy-search/src/lib.rs`, `domains/*/skill.md`
- Why fragile: Selectors, hydration timing, anti-bot behavior, API contracts, and live content can change independently of this repository.
- Safe modification: Keep selectors in domain docs, add diagnostic scripts near each guest, and prefer local deterministic fixture pages for CI.
- Test coverage: `bhsmoke` covers these flows manually, but `.github/workflows/ci.yml` does not run live browser smokes.

**Local browser discovery can attach to the wrong profile or stale DevTools endpoint:**
- Files: `rust/crates/bh-discovery/src/lib.rs`, `install.md`, `SKILL.md`
- Why fragile: `get_ws_url` scans a fixed list of Chrome/Edge profile directories and returns the first readable `DevToolsActivePort` that accepts a TCP connection.
- Safe modification: Add explicit profile selection, show the chosen browser/profile path in diagnostics, and prefer `BU_CDP_WS` when the caller needs deterministic attach behavior.
- Test coverage: Unit tests cover `BU_CDP_WS` preference, but not multi-profile discovery order or stale `DevToolsActivePort` files.

**Daemon state assumes one active target/session per daemon namespace:**
- Files: `rust/crates/bh-daemon/src/lib.rs`, `rust/bins/bhrun/src/main.rs`, `interaction-skills/tabs.md`
- Why fragile: `DaemonState` stores one `session_id`, one `target_id`, one pending dialog, and one global event queue. Concurrent `bhrun` clients can switch tabs or drain events under each other.
- Safe modification: Treat each daemon namespace as single-controller unless cursor-based events and per-client active sessions are introduced.
- Test coverage: Unit tests cover pure helpers, not concurrent socket clients or simultaneous tab switching.

**Page-title marking mutates user-visible page state:**
- Files: `rust/crates/bh-daemon/src/lib.rs`, `interaction-skills/tabs.md`
- Why fragile: `mark_session` prepends a marker to `document.title` and `unmark_session` attempts to remove it. This can affect pages, tests, screenshots, and user expectations.
- Safe modification: Prefer DevTools target metadata, overlay-free tab tracking, or a non-mutating marker mechanism.
- Test coverage: No test exercises title mutation against a real page.

**Timeout policy is split and partly hard-coded:**
- Files: `rust/crates/bh-cdp/src/lib.rs`, `rust/bins/bhrun/src/main.rs`, `rust/crates/bh-wasm-host/src/lib.rs`
- Why fragile: CDP calls time out after 30 seconds in `CdpClient::send_raw`; daemon socket read timeout is extended only for `wait_for_load`; event waits use separate millisecond timeouts.
- Safe modification: Add per-operation timeout fields and propagate them through CLI, guest, daemon, and CDP layers.
- Test coverage: Request timeout defaults are tested in `rust/crates/bh-wasm-host/src/lib.rs`, but long-running CDP operations are not covered.

**Runtime is Unix-socket based despite some cross-platform installer logic:**
- Files: `rust/crates/bh-daemon/src/lib.rs`, `rust/bins/bhrun/src/main.rs`, `rust/bins/browser-harness-cli/src/main.rs`
- Why fragile: Daemon and runner import `std::os::unix::*`, while installer verification includes Windows-style `Scripts` and `Lib/site-packages` paths.
- Safe modification: Mark the runtime target explicitly as Unix-only with `cfg(unix)` and documentation, or add Windows named-pipe support before claiming Windows runtime support.
- Test coverage: `.github/workflows/ci.yml` runs only on Ubuntu.

## Scaling Limits

**Single daemon namespace is a single-controller runtime:**
- Current capacity: One active CDP session, one active target, one dialog state, and one bounded event queue per `BU_NAME`.
- Limit: Parallel agents or concurrent commands can switch tabs, consume events, and overwrite state.
- Scaling path: Add per-client sessions, per-target operation routing, non-destructive event cursors, and namespace isolation guidance.
- Files: `rust/crates/bh-daemon/src/lib.rs`, `rust/crates/bh-discovery/src/lib.rs`, `rust/bins/bhrun/src/main.rs`

**Event queue capacity is fixed at 500 events:**
- Current capacity: `DEFAULT_EVENT_CAPACITY` is 500.
- Limit: Network-heavy pages can emit more than 500 CDP events before a watcher drains them.
- Scaling path: Make capacity configurable, add event type filters in the daemon before enqueueing, and support per-wait subscriptions.
- Files: `rust/crates/bh-daemon/src/lib.rs`, `rust/bins/bhrun/src/main.rs`

**Guest SDK output buffer is fixed at 8 MiB per host call:**
- Current capacity: `DEFAULT_OUTPUT_CAPACITY` is 8 MiB.
- Limit: Large screenshots, PDFs, HTTP responses, or event batches can exceed the buffer and fail, while small calls still allocate the full buffer.
- Scaling path: Negotiate output size, support chunked host calls, and allocate based on operation type.
- Files: `rust/crates/bh-guest-sdk/src/lib.rs`, `rust/bins/bhrun/src/main.rs`

**CI verifies build/install but not live browser behavior:**
- Current capacity: CI runs secret scan, Rust unit tests, install, CLI help/summary, and verify-install.
- Limit: Browser automation regressions can pass CI if they only affect live CDP, real Chrome profiles, downloads, dialogs, uploads, or guest site workflows.
- Scaling path: Add deterministic local browser fixture smokes to CI, then keep remote/live-site smokes as scheduled or manual jobs.
- Files: `.github/workflows/ci.yml`, `rust/bins/bhsmoke/src/main.rs`, `docs/development.md`

## Dependencies at Risk

**TLS stack and `ring` link behavior:**
- Risk: Full workspace tests fail in the current macOS arm64 environment during `ring` linking through TLS-dependent crates.
- Impact: Local verification can be blocked even when Linux CI succeeds.
- Migration plan: Pin and document supported local toolchains, add a CI macOS job if macOS support matters, and track `ring`/TLS dependency updates through `rust/Cargo.lock`.
- Files: `rust/Cargo.lock`, `rust/Cargo.toml`, `rust/crates/bh-cdp/Cargo.toml`, `rust/crates/bh-remote/Cargo.toml`, `rust/bins/bhrun/Cargo.toml`

**Browser Use API shape and availability:**
- Risk: Remote browser lifecycle depends on Browser Use endpoints for browser creation, listing, stop, `/json/version`, and profile APIs.
- Impact: `bhctl create-browser`, `stop-browser`, `list-cloud-profiles`, `resolve-profile-name`, and remote smokes break when API shape, auth behavior, quota, or status fields change.
- Migration plan: Add mock-server integration tests for `bh-remote`, keep remote failures classified separately from local release gates, and document expected response shapes.
- Files: `rust/crates/bh-remote/src/lib.rs`, `rust/bins/bhctl/src/main.rs`, `rust/bins/bhsmoke/src/main.rs`

**External `profile-use` command is not managed by Cargo:**
- Risk: Profile sync/list commands shell out to `profile-use`, but the binary is not part of the Rust workspace or CI setup.
- Impact: `list-local-profiles` and `sync-local-profile` fail on machines without a matching `profile-use` install.
- Migration plan: Keep preflight errors clear, document installation/version requirements, and add a mockable adapter around `profile-use` output parsing.
- Files: `rust/bins/bhctl/src/main.rs`, `install.md`, `SKILL.md`

**Third-party live websites and public APIs:**
- Risk: GitHub Trending, Reddit, Product Hunt, Letterboxd, Spotify, Etsy, Metacritic, Walmart, TradingView, and 2048 workflows depend on external DOM/API behavior.
- Impact: Guest smokes and domain skills can fail without code changes in this repo.
- Migration plan: Maintain domain docs, keep diagnostics near guest code, and introduce fixture-backed acceptance tests for core harness mechanics.
- Files: `rust/bins/bhsmoke/src/main.rs`, `rust/guests/rust-*`, `domains/*/skill.md`

## Missing Critical Features

**No explicit runtime threat model:**
- Problem: The repo exposes powerful browser control, cookies, file upload, downloads, raw CDP, and JavaScript execution, but no single document defines the trusted boundaries for local socket clients, guests, or wrapper tools.
- Blocks: Safe embedding in multi-agent, multi-user, or untrusted-workflow environments.
- Files: `docs/architecture.md`, `docs/wasm-guests.md`, `rust/crates/bh-daemon/src/lib.rs`, `rust/bins/bhrun/src/main.rs`

**No formatter or Clippy enforcement in CI:**
- Problem: `docs/development.md` documents `cargo fmt`, but `.github/workflows/ci.yml` does not run `cargo fmt --check` or `cargo clippy`.
- Blocks: Consistent style and lint regression prevention as large files keep growing.
- Files: `.github/workflows/ci.yml`, `docs/development.md`, `rust/Cargo.toml`

**No dedicated integration-test package:**
- Problem: Tests are co-located unit tests; there is no `rust/tests/` integration suite for CLI subprocesses, daemon socket behavior, or Browser Use mock API flows.
- Blocks: Safe refactors of `bhrun`, `bhctl`, `bh-daemon`, and `bh-remote`.
- Files: `rust/bins/bhrun/src/main.rs`, `rust/bins/bhctl/src/main.rs`, `rust/crates/bh-daemon/src/lib.rs`, `rust/crates/bh-remote/src/lib.rs`

**No resource policy for guest execution:**
- Problem: Runner config controls operation capabilities, but not CPU time, memory, response size, outbound domains, or artifact size.
- Blocks: Running untrusted or semi-trusted guests safely.
- Files: `rust/crates/bh-wasm-host/src/lib.rs`, `rust/bins/bhrun/src/main.rs`, `rust/crates/bh-guest-sdk/src/lib.rs`

**No Windows daemon transport:**
- Problem: Runtime IPC uses Unix sockets, and no Windows named-pipe or TCP loopback transport is implemented.
- Blocks: Native Windows runtime support.
- Files: `rust/crates/bh-daemon/src/lib.rs`, `rust/bins/bhrun/src/main.rs`, `rust/bins/browser-harness-cli/src/main.rs`

## Test Coverage Gaps

**Concurrent daemon clients and event races:**
- What's not tested: Two clients waiting for the same event, `watch_events` plus `wait_for_event`, concurrent `switch-tab`, and simultaneous `drain-events`.
- Files: `rust/crates/bh-daemon/src/lib.rs`, `rust/bins/bhrun/src/main.rs`
- Risk: Event loss and session races can go unnoticed.
- Priority: High

**Security-sensitive runtime files:**
- What's not tested: Symlink handling, restrictive permissions on pid/log/socket files, invalid `BU_NAME`, stale pid files pointing at live unrelated processes, and pid ownership checks.
- Files: `rust/crates/bh-discovery/src/lib.rs`, `rust/crates/bh-daemon/src/lib.rs`, `rust/bins/bhctl/src/main.rs`
- Risk: Local privilege boundary assumptions can regress.
- Priority: High

**Guest resource exhaustion:**
- What's not tested: Infinite-loop guests, large memory growth, responses larger than 8 MiB, long-running host calls, and repeated `serve-guest` invocations under load.
- Files: `rust/bins/bhrun/src/main.rs`, `rust/crates/bh-guest-sdk/src/lib.rs`, `rust/crates/bh-wasm-host/src/lib.rs`
- Risk: A guest can hang or exhaust local resources.
- Priority: High

**Remote Browser Use API error shapes:**
- What's not tested: Non-JSON errors, missing fields, pagination edge cases, duplicate profile names from live-like data, `/json/version` failures, and stop-browser cleanup failures.
- Files: `rust/crates/bh-remote/src/lib.rs`, `rust/bins/bhctl/src/main.rs`
- Risk: Remote setup fails with unclear errors or leaves browsers running.
- Priority: Medium

**Large artifact and sensitive output handling:**
- What's not tested: Large screenshots/PDFs, large HTTP responses, cookie redaction, guest call trace redaction, and stdout size behavior.
- Files: `rust/crates/bh-daemon/src/lib.rs`, `rust/bins/bhrun/src/main.rs`, `rust/crates/bh-guest-sdk/src/lib.rs`, `rust/bins/bhsmoke/src/main.rs`
- Risk: Memory spikes, broken guests, and accidental sensitive output.
- Priority: Medium

**Rust-to-WASM guest crates:**
- What's not tested: Guest crate build settings, `panic = "abort"` consistency, wasm32 compilation for every `rust/guests/rust-*` crate, and fixture-backed run behavior.
- Files: `rust/guests/rust-*/Cargo.toml`, `rust/guests/rust-*/src/lib.rs`, `rust/bins/bhsmoke/src/main.rs`
- Risk: Guest examples drift from SDK/runtime expectations.
- Priority: Medium

---

*Concerns audit: 2026-05-14*

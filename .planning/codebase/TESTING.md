# Testing Patterns

**Analysis Date:** 2026-05-14

## Test Framework

**Runner:**
- Rust built-in test harness via `cargo test`.
- Async tests use `#[tokio::test]`, currently observed in `rust/crates/bh-daemon/src/lib.rs`.
- Config: `rust/Cargo.toml`

**Assertion Library:**
- Standard Rust assertions: `assert_eq!`, `assert!`, and `expect_err`.
- JSON fixtures use `serde_json::json`, especially in `rust/crates/bh-protocol/src/lib.rs`, `rust/crates/bh-wasm-host/src/lib.rs`, and `rust/bins/bhrun/src/main.rs`.

**Run Commands:**
```bash
cargo test --workspace --manifest-path rust/Cargo.toml              # Run all workspace tests
cargo test --manifest-path rust/Cargo.toml -p bh-wasm-host          # Run one crate
cargo test --manifest-path rust/Cargo.toml -p bhrun -- --nocapture  # Run one package with test output
```

CI runs the full workspace test command in `.github/workflows/ci.yml`.

## Test File Organization

**Location:**
- Tests are co-located inside the implementation file with `#[cfg(test)] mod tests`.
- No `rust/tests/`, `tests/`, `benches/`, or dedicated integration-test directory is detected.

**Naming:**
- Test functions use descriptive `snake_case`: `parses_cdp_request` in `rust/crates/bh-protocol/src/lib.rs`, `resolve_profile_name_rejects_duplicates` in `rust/crates/bh-remote/src/lib.rs`, `routes_admin_commands_to_bhctl` in `rust/bins/browser-harness-cli/src/main.rs`.
- Tests describe behavior rather than implementation internals: `wait_for_event_matches_after_multiple_polls` and `dispatch_guest_operation_rejects_ungranted_operation` in `rust/bins/bhrun/src/main.rs`.

**Structure:**
```text
rust/crates/{crate}/src/lib.rs      # Library implementation + #[cfg(test)] mod tests
rust/bins/{binary}/src/main.rs      # CLI implementation + #[cfg(test)] mod tests
rust/guests/{guest}/src/lib.rs      # Guest implementation; no co-located tests detected
```

Current co-located test distribution:
- `rust/bins/bhrun/src/main.rs`: 60 tests
- `rust/crates/bh-wasm-host/src/lib.rs`: 48 tests
- `rust/crates/bh-guest-sdk/src/lib.rs`: 18 tests
- `rust/crates/bh-daemon/src/lib.rs`: 7 tests
- `rust/bins/browser-harness-cli/src/main.rs`: 7 tests
- `rust/bins/bhctl/src/main.rs`: 7 tests
- `rust/crates/bh-remote/src/lib.rs`: 5 tests
- `rust/crates/bh-discovery/src/lib.rs`: 3 tests
- `rust/crates/bh-protocol/src/lib.rs`: 2 tests

## Test Structure

**Suite Organization:**
```rust
#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{DaemonRequest, DaemonResponse};

    #[test]
    fn parses_cdp_request() {
        let request = DaemonRequest::from_json_line(
            r#"{"method":"Page.navigate","params":{"url":"https://example.com"},"session_id":"abc"}"#,
        )
        .unwrap();

        assert_eq!(request.method.as_deref(), Some("Page.navigate"));
        assert_eq!(request.session_id.as_deref(), Some("abc"));
        assert_eq!(request.params, Some(json!({"url": "https://example.com"})));
    }
}
```
This pattern appears in `rust/crates/bh-protocol/src/lib.rs`.

**Patterns:**
- Arrange-Act-Assert is the dominant structure: build request/fixture, invoke helper, assert typed result or JSON fields.
- Tests import target functions through `use super::{...}`; avoid black-box module access unless a future integration-test directory is added.
- Use `serde_json::json` to construct protocol input and expected output objects.
- For command routing and JSON command output, call the pure helper instead of spawning a process: `run_cli` in `rust/bins/bhrun/src/main.rs`, `route_command` in `rust/bins/browser-harness-cli/src/main.rs`.
- For side-effect boundaries, test injected helper variants: `*_with_sender` and `*_with_drain` in `rust/bins/bhrun/src/main.rs`.

## Mocking

**Framework:** None

**Patterns:**
```rust
let result = page_info_with_sender(PageInfoRequest::default(), |daemon, request| {
    assert_eq!(daemon, "default");
    assert_eq!(request.meta.as_deref(), Some(META_PAGE_INFO));
    Ok(DaemonResponse {
        result: Some(json!({"url":"about:blank","title":"","w":1280})),
        ..DaemonResponse::default()
    })
})
.expect("page info result");
```
Closure-based sender injection is used in `rust/bins/bhrun/src/main.rs`.

```rust
let result: Value = call_json_with(
    |operation, request, output| {
        assert_eq!(operation, b"goto");
        let request: Value = serde_json::from_slice(request).expect("parse request");
        assert_eq!(request.get("url").and_then(Value::as_str), Some("https://example.com"));
        let response = serde_json::to_vec(&json!({"frameId":"frame-1"})).expect("serialize response");
        output[..response.len()].copy_from_slice(&response);
        response.len() as i32
    },
    "goto",
    &json!({"url":"https://example.com"}),
)
.expect("goto result");
```
Host-call mocking is used in `rust/crates/bh-guest-sdk/src/lib.rs`.

**What to Mock:**
- Daemon request sending through closures in `rust/bins/bhrun/src/main.rs`.
- Event draining through closures for polling behavior in `rust/bins/bhrun/src/main.rs`.
- WASM guest host calls through `call_json_with` in `rust/crates/bh-guest-sdk/src/lib.rs`.
- Environment-variable mutation with a process-wide lock, as in `env_lock` in `rust/crates/bh-discovery/src/lib.rs` and `rust/bins/bhctl/src/main.rs`.
- HTTP interactions with a local `TcpListener` fixture, as in `spawn_http_fixture_server` in `rust/bins/bhrun/src/main.rs`.

**What NOT to Mock:**
- Serde serialization and deserialization for protocol DTOs; tests use real `serde_json` in `rust/crates/bh-protocol/src/lib.rs` and `rust/crates/bh-wasm-host/src/lib.rs`.
- Pure request normalization and defaulting logic; test direct outputs from `Default` and `normalized()` implementations in `rust/crates/bh-wasm-host/src/lib.rs`.
- Simple CLI route classification; call `route_command` directly in `rust/bins/browser-harness-cli/src/main.rs`.

## Fixtures and Factories

**Test Data:**
```rust
fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}
```
This process-wide environment lock pattern appears in `rust/crates/bh-discovery/src/lib.rs` and `rust/bins/bhctl/src/main.rs`.

```rust
fn test_config(label: &str) -> DaemonConfig {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    DaemonConfig::new(format!("test-{label}-{}-{now}", std::process::id()))
}
```
Runtime-file tests use unique daemon names in `rust/crates/bh-daemon/src/lib.rs`.

**Location:**
- Inline JSON fixtures: `rust/crates/bh-wasm-host/src/lib.rs`, `rust/bins/bhrun/src/main.rs`.
- Temporary directory helper: `TestTempDir` in `rust/bins/browser-harness-cli/src/main.rs`.
- Local HTTP fixture server: `spawn_http_fixture_server` in `rust/bins/bhrun/src/main.rs`.
- WASM guest fixture path: `persistent_counter_guest_path()` in `rust/bins/bhrun/src/main.rs`, pointing at `rust/guests/persistent_counter.wat`.

## Coverage

**Requirements:** None enforced

No coverage configuration is detected. There is no `tarpaulin.toml`, no `cargo-llvm-cov` configuration, and `.github/workflows/ci.yml` does not upload coverage.

**View Coverage:**
```bash
Not detected
```

If coverage is added, place the command in `docs/development.md` and wire it into `.github/workflows/ci.yml`.

## Test Types

**Unit Tests:**
- Protocol JSON parsing and serialization: `rust/crates/bh-protocol/src/lib.rs`.
- Request defaults, normalization, event filtering, and manifest metadata: `rust/crates/bh-wasm-host/src/lib.rs`.
- SDK host-call serialization and typed response deserialization: `rust/crates/bh-guest-sdk/src/lib.rs`.
- Browser Use client URL and profile-name resolution helpers: `rust/crates/bh-remote/src/lib.rs`.
- CLI option parsing and command routing: `rust/bins/browser-harness-cli/src/main.rs`, `rust/bins/bhctl/src/main.rs`.

**Integration Tests:**
- No separate integration-test crate or `tests/` directory is present.
- In-process integration-style tests are embedded in `rust/bins/bhrun/src/main.rs`, including:
  - local HTTP server exercise for `http_get`
  - guest runtime persistence through `GuestRuntime`
  - daemon request construction through `*_with_sender`
  - CLI input/output flow through `run_cli`

**E2E Tests:**
- E2E browser smokes are represented by the `bhsmoke` binary in `rust/bins/bhsmoke/src/main.rs`.
- Local and remote smoke commands are documented in `docs/development.md` and `rust/README.md`.
- CI in `.github/workflows/ci.yml` performs lightweight CLI checks (`browser-harness --help`, `browser-harness summary`, `bhctl daemon-alive`, `bhrun summary`) and install verification via `browser-harness verify-install --install-root ...`; live browser smokes are not part of the default CI job.

## Common Patterns

**Async Testing:**
```rust
#[tokio::test]
async fn stop_remote_is_noop_without_remote_config() {
    let config = DaemonConfig::new("default");
    assert!(!stop_remote(&config).await.unwrap());
}
```
Use `#[tokio::test]` for async functions in `rust/crates/bh-daemon/src/lib.rs`.

**Error Testing:**
```rust
let err = dispatch_guest_operation(&mut state, "goto", r#"{"url":"https://example.com"}"#)
    .expect_err("ungranted operation should fail");
assert_eq!(err, "operation denied by runner config: goto");
```
Use `expect_err`, `unwrap_err().contains(...)`, or exact string comparison for failure contracts in `rust/bins/bhrun/src/main.rs` and `rust/crates/bh-remote/src/lib.rs`.

**Environment Testing:**
- Lock and restore environment variables in the same test to avoid cross-test contamination.
- Preserve the previous value, mutate for the assertion, then restore or remove it. Examples: `get_ws_url_prefers_env_override` in `rust/crates/bh-discovery/src/lib.rs`, `resolve_daemon_name_prefers_explicit_value_then_env_then_default` in `rust/bins/bhctl/src/main.rs`.

**CLI Testing:**
- Prefer testable generic functions that accept input and output streams: `run_cli<I, R, W>` in `rust/bins/bhrun/src/main.rs`.
- Assert parsed JSON output or return codes instead of spawning subprocesses when testing command logic: `parse_install_options` and `infer_install_root` in `rust/bins/browser-harness-cli/src/main.rs`.

**Smoke Verification:**
```bash
cargo run --quiet --manifest-path rust/Cargo.toml --bin browser-harness -- install
browser-harness verify-install
BU_BROWSER_MODE=local BU_DAEMON_IMPL=rust rust/target/debug/bhsmoke guest-run
```
Use smoke verification for browser-facing behavior documented in `docs/development.md`; keep default unit tests deterministic and local.

---

*Testing analysis: 2026-05-14*

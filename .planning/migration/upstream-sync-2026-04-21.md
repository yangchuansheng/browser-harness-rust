# Upstream Sync Audit — 2026-04-21+

## Scope

- Upstream repository: `https://github.com/browser-use/browser-harness`
- Baseline commit before requested date: `2d23211d346c7a12bdb2ce03e49b2d955f4769b2`
- Upstream target commit: `2f22ed6709748edc5eab733eae099802640a78e2`
- Commit range: `2d23211d346c7a12bdb2ce03e49b2d955f4769b2..upstream/main`
- Count: 239 commits
- User intent: replicate all upstream updates since Apr 21, 2026 into this Rust fork while preserving the Rust architecture.

## Migrated Runtime Behavior

- Added expanded local browser profile discovery for Chrome Canary, Comet, Arc,
  Dia, Brave, Edge channels, Windows Chrome SxS, and Flatpak profile paths.
- Added `BU_CDP_URL` DevTools HTTP endpoint support alongside `BU_CDP_WS`.
- Added `/json/version` websocket resolution and `DevToolsActivePort` fallback
  for newer Chrome builds returning 404.
- Split runtime and temp paths with `BH_RUNTIME_DIR` for socket/pid files and
  `BH_TMP_DIR` for logs/screenshots.
- Added `BU_NAME` validation to prevent path traversal in runtime file names.
- Added daemon `ping` and `connection_status` metadata.
- Updated controlled-tab marker to 🐴 and fixed marker removal.
- Preserved target attachment status for `current_tab` and `set_session` flows.
- Added remote-specific CDP handshake messaging for cloud websocket failures.

## Migrated Helper/API Surface

- Added `wait_for_element` / `wait-for-element` for SPA late-render waits.
- Added `fill_input` / `fill-input` for framework-managed inputs.
- Added `wait_for_network_idle` / `wait-for-network-idle` for XHR/fetch settle waits.
- Added screenshot `max_dim` support with Rust PNG resize behavior.
- Exposed the new operations through `bhrun`, `browser-harness`, `bh-wasm-host`,
  and `bh_guest_sdk`.

## Migrated Knowledge and Docs

- Imported upstream domain-skill corpus into `domains/` with upstream
  `scraping.md` mapped to this fork's `skill.md` convention.
- Imported upstream issue templates and `VOUCHED.td`.
- Imported `docs/snap-linux-headless.md` and `docs/allow-remote-debugging.png`.
- Updated `SKILL.md`, `install.md`, `README.md`, `domains/README.md`, and
  interaction skills for upstream connection and helper guidance.

## Adapted Instead of Copied

- Python runtime files (`src/browser_harness/*.py`) were not copied verbatim;
  equivalent behavior was ported to Rust crates and binaries.
- Upstream GitHub workflows were not copied so the Rust CI/workspace shape is
  not overwritten by Python packaging assumptions.
- Existing Rust architecture, WASM guest model, and Rust-specific docs were kept.

## Verification Evidence

- `cargo fmt --manifest-path rust/Cargo.toml --all -- --check` passed.
- `cargo check --manifest-path rust/Cargo.toml --workspace` passed.
- `env -u CFLAGS -u CC cargo test --manifest-path rust/Cargo.toml --workspace` passed.
- `env -u CFLAGS -u CC cargo run --quiet --manifest-path rust/Cargo.toml --bin bhrun -- summary` exposed the new helper operations.
- `env -u CFLAGS -u CC cargo run --quiet --manifest-path rust/Cargo.toml --bin browser-harness -- --help` exposed the new runner commands through the facade.
- `git diff --check` passed.
- Secret/local-path scans found no API keys, pinned local websocket, or local home path leaks in tracked/unignored files.

## Re-Audit — 2026-05-14

- Re-fetched `upstream/main`; target remains `2f22ed6709748edc5eab733eae099802640a78e2`.
- Recounted commit range `2d23211d346c7a12bdb2ce03e49b2d955f4769b2..upstream/main`: 239 commits.
- Cross-checked upstream domain-skill entries from both `agent-workspace/domain-skills/` and legacy `domain-skills/` paths against this fork's `domains/` mapping.
- Initial re-audit found two missing legacy Amazon domain-skill files from upstream commit `17e88b4`: `domain-skills/amazon/cart.md` and `domain-skills/amazon/orders.md`.
- Fixed by adding Rust-layout equivalents at `domains/amazon/cart.md` and `domains/amazon/orders.md`; helper examples use text fences and path references follow the local `domains/` convention.
- Post-fix domain mapping result: 109 upstream domain-skill entries / 109 local mapped files present.

## Re-Audit Verification Evidence

- `git fetch upstream main` confirmed target `2f22ed6709748edc5eab733eae099802640a78e2`.
- Domain mapping script reported `upstream domain file entries 109` and `missing mapped files 0`.
- Re-ran Rust formatting, check, tests, CLI smoke, diff whitespace check, and secret/local-path scans after the Amazon fix. The repository `scripts/scan_sensitive.sh` requires Bash 4 `mapfile`; macOS `/bin/bash` is 3.2 in this worktree, so an equivalent Python/rg scan was used for the final secret/local-path pass.

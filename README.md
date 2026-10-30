# Browser Harness Rust ♞

![Browser Harness hero](docs/hero.jpeg)

Browser Harness lets Claude Code, Codex, or Cursor use your real Chrome browser:
open pages, click, type, read, and download, logged in as you. Paste one prompt
into your coding agent to set it up. This Rust fork talks to Chrome over the
DevTools Protocol (CDP) through a thin Rust-native daemon and CLI, and the agent
can extend missing behavior in Rust or as a WASM guest as it works, so the
harness improves with each task. Open source, MIT licensed.

Try browser-harness in [Browser Use Cloud](https://cloud.browser-use.com/v4?utm_campaign=browser-harness-use-in-cloud&utm_source=github) or paste the setup prompt into your coding agent.

It exposes stable browser operations, keeps the control plane small, and still
lets the agent extend missing behavior mid-task when it needs to.

`domains/<site>/skill.md` is the knowledge layer: URL patterns, selectors,
waits, traps, APIs, and site-specific workflow notes. Rust guests are optional
packaged workflows, not the default artifact for every domain. Some domain docs
use Python-like helper syntax as pseudocode for harness operations; that does
not mean Python is required to use this repo.

## Acknowledgements

This project is a Rust reimplementation of the original
[`browser-use/browser-harness`](https://github.com/browser-use/browser-harness)
Python harness.

The core harness idea and direction came from that upstream project. Many
thanks to the `browser-use` maintainers for creating and sharing the original
work that this repo builds on.

```
  ● agent: needs a site-specific browser workflow
  │
  ● browser-harness exposes stable primitives
  │
  ● a helper is missing
  │
  ● the agent adds it in Rust or as a WASM guest
  ✓ task completed
```

## See it work

**Task:** "Open my X profile, find my latest 20 video posts, and download them."

[![Download my latest 20 X videos](docs/download-latest-20-x-videos.gif)](https://browser-use.com/showcase/videos/download-latest-20-x-videos.mp4)

## What This Repo Is

This repo carries the same core thesis as the original
[`browser-use/browser-harness`](https://github.com/browser-use/browser-harness),
but reimplements the runtime in Rust.

The upstream original keeps the harness extremely small with files like
`run.py`, `helpers.py`, `admin.py`, and `daemon.py`. That design proves the
thesis: agents do not need a huge browser framework to be useful.

This repo keeps that same thesis, but moves the control plane into compiled
Rust binaries and crates:

- `browser-harness` as the top-level CLI
- `bhd` as the daemon/runtime
- `bhrun` as the typed runner and guest executor
- `bhctl` as the admin/control-plane binary

So the right comparison is not "framework vs harness". It is:

- original Browser Harness: tiny Python runtime
- this repo: Rust systems rewrite of that runtime
- `browser-harness-js`: protocol-first JS/TS variant with almost no helpers

## Why Rust

We chose Rust because this project is fundamentally a systems boundary:

- It runs a long-lived daemon that owns a live browser websocket and session state.
- It exposes a stable CLI/runtime surface that other tools can depend on.
- It needs typed protocol boundaries between daemon, runner, admin commands, and guest execution.
- It is moving toward a Rust + WASM guest model, where the host and the guest SDK benefit from sharing a strongly typed language and build toolchain.

In practice, Rust gives this project a few concrete advantages over the original
Python harness:

- single native binaries instead of a Python packaging/runtime story
- stronger typing around protocol payloads and internal boundaries
- easier long-lived daemon behavior and runtime encapsulation
- a cleaner path to capability-gated WASM guests
- one implementation language for CLI, daemon, SDK, and host runtime

This is not "Rust because Rust is cool". It is Rust because this repo is no
longer just a handful of editable helper functions; it is now a durable runtime
surface.

## Comparison

Browser Harness should be read against the two adjacent Browser Use projects:

| Project | Main shape | Best when | Tradeoff |
| --- | --- | --- | --- |
| [`browser-use/browser-harness`](https://github.com/browser-use/browser-harness) | Tiny Python harness with editable helper files and a small daemon bridge | You want the original minimal harness exactly as designed | Smaller and simpler, but less suited to a larger typed runtime surface |
| [`browser-use/browser-harness-js`](https://github.com/browser-use/browser-harness-js) | JS/TS CDP surface with generated protocol wrappers and almost no helpers | You want protocol-level freedom in JS/TS and are happy writing close to raw CDP | Maximum flexibility, but more protocol-shaped and less ergonomic for repeated higher-level workflows |
| [`yangchuansheng/browser-harness`](https://github.com/yangchuansheng/browser-harness) (this repo) | Rust daemon + runner + CLI + guest runtime | You want the harness idea, but as a durable systems runtime with stable operations and a WASM path | Heavier than the original Python harness, less raw than the JS variant |

Another way to frame it:

- the original Python repo is the minimal harness proof
- `browser-harness-js` is the raw protocol-first variant
- this repo is the runtime-first Rust variant

If you want the shortest decision rule:

- Choose the original Python harness if you want the smallest possible editable harness.
- Choose `browser-harness-js` if you want the agent to speak almost-direct CDP in JS/TS.
- Choose this repo if you want the harness idea packaged as a typed runtime, daemon, CLI, and guest host.

## Upstream Parity

This Rust fork tracks the upstream
[`browser-use/browser-harness`](https://github.com/browser-use/browser-harness)
behavior while preserving the Rust runtime architecture. The Apr-May 2026 sync
ported the latest discovery, daemon, helper, domain-skill, and setup-doc
changes into equivalent Rust crates and CLIs.

Notable parity points:

- local discovery for Chrome Canary, Comet, Arc, Dia, Brave, Edge channels,
  Windows Chrome SxS, and Flatpak profile directories
- `BU_CDP_WS`, `BU_CDP_URL`, `/json/version` resolution, and
  `DevToolsActivePort` fallback for newer Chrome builds
- split `BH_RUNTIME_DIR` for sockets/pids from `BH_TMP_DIR` for logs/screenshots
- daemon liveness/status metadata and the 🐴 controlled-tab marker
- helper operations: `wait-for-element`, `fill-input`, `wait-for-network-idle`,
  and screenshot `max_dim`
- imported upstream domain skills under `domains/` and issue templates under
  `.github/ISSUE_TEMPLATE/`

## Quick Start

Install once, then use the Rust-native CLI directly:

```bash
cargo run --quiet --manifest-path rust/Cargo.toml --bin browser-harness -- install
export PATH="$HOME/.cargo/bin:$PATH"
browser-harness ensure-daemon
browser-harness page-info <<'JSON'
{"daemon_name":"default"}
JSON
browser-harness new-tab <<'JSON'
{"daemon_name":"default","url":"https://example.com"}
JSON
browser-harness close-tab <<'JSON'
{"daemon_name":"default"}
JSON
```

The installer builds the Rust binaries from this checkout and installs them into
`$CARGO_HOME/bin` or `$HOME/.cargo/bin` by default. Re-run the same install
command after pulling new changes if you want to refresh the global binaries.

If you are working inside the repo and have not installed the global command
yet, use:

```bash
cd rust
cargo run --quiet --bin browser-harness -- --help
```

## Setup prompt

Paste into Claude Code or Codex (Cursor: use the [MCP config](#cursor) below):

```text
Set up https://github.com/yangchuansheng/browser-harness.git for me.

Read `install.md` first to install and connect this repo to my real browser. Then read `SKILL.md` for normal usage. Ask whether I want local browser recordings enabled; default to no and preserve my existing preference on upgrades. Prefer the Rust-native CLI path first. When you open a setup or verification tab, activate it so I can see the active browser tab. After it is installed, open this repository in my browser so I can see the harness has attached, then continue with my requested browser task.
```

When this page appears, tick the checkbox so the agent can connect to your browser:

<img src="docs/setup-remote-debugging.png" alt="Remote debugging setup" width="520" style="border-radius: 12px;" />

On macOS, when Chrome asks for remote-debugging permission, run
`browser-harness mac-approve` in another shell. Continue when it returns
`ready`; otherwise follow its printed instruction.

See [domains/](domains/) for the active site-specific knowledge tree.

## Compared with other browser tools

| | Works with | Browser it drives | What you approve in your own Chrome |
|---|---|---|---|
| **Browser Harness** | Claude Code, Codex, Cursor, any MCP client via `browser-harness-mcp` | Your running Chrome, logged in as you | Whole browser: on Chrome 144+, tick remote debugging in `chrome://inspect`, then click Allow |
| **Playwright MCP** | Any MCP client (Claude Code, Codex, Cursor, Copilot and more) | Its own browser profile by default. Your Chrome with `--extension` (Playwright Extension) or `--cdp-endpoint` | Chosen tabs (one tab group per client) via the extension, approval skippable with a token; or whole browser via `--cdp-endpoint` |
| **Playwright CLI** | Claude Code, GitHub Copilot and other agents that run shell commands | Its own browser by default. Your Chrome with `attach --extension=chrome` or `attach --cdp=chrome` | Chosen tab via the Playwright Extension, or whole browser via `--cdp` |
| **Chrome DevTools MCP** | Any MCP client (Claude Code, Cursor, Copilot, Antigravity and more), plus a CLI | A new Chrome with a dedicated profile by default. Your Chrome 144+ with `--autoConnect` | Whole browser: the same Chrome 144+ toggle and Allow dialog; the server sees every open window of that profile |
| **agent-browser** | Claude Code, Codex, Cursor, Gemini CLI, Copilot and more, as a CLI with a skill, plus an MCP server | Its own Chrome for Testing by default. A copy of your Chrome profile with `--profile`. Your running Chrome with `--auto-connect` or `--cdp` | Whole browser via `--auto-connect` or `--cdp`: the same Chrome 144+ toggle and Allow dialog |
| **Claude in Chrome** | Claude Code (CLI or VS Code) with the Claude extension, signed in with a direct Anthropic plan (Pro, Max, Team, Enterprise), not an API key | Your Chrome, Edge or other Chromium browser, logged in as you | Per site: allow sites in the extension, or per action in Claude Code (auto mode can skip the per-site check). Claude works in a tab group it opens |
| **ChatGPT Browser and browser extension** | Codex and ChatGPT chats in the ChatGPT desktop app, not Codex CLI or the Codex IDE extension | Browser: a separate profile. Extension: your Chrome, Edge, Brave, Opera or Vivaldi profile | Per site: asks before each new website (allow once, for this site, or for all sites) |

Matched eval by Browser Use on 100 of the 200 public [BU Bench V2.1](https://github.com/browser-use/benchmark) tasks (bu2-001 to bu2-100; 101-200 not run). Measured 2026-10-14 against upstream `browser-use/browser-harness`:

- Same agent for every tool: Codex CLI 0.144.6, GPT-6 Luna, medium reasoning, web search off. Same frozen judge.
- Browser Harness was given to the agent as its CLI and `SKILL.md`, not `browser-harness-mcp`. Playwright MCP and Chrome DevTools MCP ran as MCP servers with default settings. `agent-browser` ran as its CLI and skill.
- Every tool drove the same fresh Browser Use cloud browser over CDP. This does not measure each tool's default or local-Chrome mode.
- One run per task. Agent failures were never retried. Runner, browser and judge failures were rerun once (Browser Harness 3 tasks, Playwright MCP 4, Chrome DevTools MCP 6, agent-browser 0).

| Tool | Mean rubric score 0-100 (95% CI) | Fully solved | Median input tokens per task | Median model cost per task (GPT-6 Luna list price) | Median agent time per task |
|---|---|---|---|---|---|
| Browser Harness a6426f84 (main) | 60.4 (55.2-65.4) | 11% | 2.1M | $0.042 | 9.0 min |
| Playwright MCP 0.0.83 | 60.0 (55.1-65.0) | 11% | 4.4M | $0.066 | 8.3 min |
| Chrome DevTools MCP 1.10.1 | 61.0 (55.8-65.9) | 11% | 6.7M | $0.097 | 8.7 min |
| agent-browser 0.39.1 | 56.3 (51.1-61.5) | 9% | 3.3M | $0.056 | 11.2 min |

No tool wins on score: every paired difference against Browser Harness has a 95% CI that includes zero (other tool minus Browser Harness: agent-browser -4.0, CI -10.0 to +1.8; Playwright MCP -0.4, -6.6 to +5.7; Chrome DevTools MCP +0.6, -5.6 to +6.6).

Browser Harness had the lowest median input tokens (2.1M vs 3.3-6.7M) and the lowest median model cost ($0.042 vs $0.056-0.097). Model cost excludes browser, proxy and judge. It was slower than Playwright MCP and Chrome DevTools MCP (median 9.0 vs 8.3 and 8.7 min) and faster than agent-browser (11.2 min). These are medians, with no confidence intervals.

A first `agent-browser` run was discarded: a setup bug let it launch its own local browser instead of the shared cloud browser. The row shows the corrected rerun.

Measured with Codex only. Claude Code, Cursor, Playwright CLI and Claude in Chrome were not measured. Method and per-task scores: [coding-agent-browser-tools.json](https://browser-use.com/benchmarks/coding-agent-browser-tools.json).

### When to use something else

- **Playwright** (Test, MCP or CLI): isolated, repeatable CI tests of your own app, in a clean browser with no personal logins.
- **Chrome DevTools MCP**: performance traces and DevTools debugging (network, console, source-mapped stack traces).
- **Claude in Chrome**: you only use Claude and have a Pro, Max, Team or Enterprise plan.
- **Cursor's built-in Browser**: the agent doesn't need your logged-in Chrome. It runs in its own per-workspace profile.

OpenAI's Codex labels its browser feature "Browser use" in its settings. It is separate from Browser Use, the company behind the upstream project.

## Scale with Browser Use Cloud

Use the local browser for logged-in, personal work. When you want many browsers
in parallel, with live previews, proxies, stealth, CAPTCHA solving, and isolated
sessions, scale with [Browser Use Cloud](https://cloud.browser-use.com/new-api-key).

The Rust CLI can create a browser with `browser-harness create-browser`; pass
its `cdpWsUrl` and browser ID to `browser-harness ensure-daemon` through
`BU_CDP_WS` and `BU_BROWSER_ID`. See [SKILL.md](SKILL.md) for the complete typed
payload flow.

## MCP Server

The installed `browser-harness-mcp` binary exposes the Rust CLI operations as
MCP tools over stdio, so any MCP client (Claude Code, Devin, Cursor, etc.) can
drive the browser without writing a second CDP layer. See [docs/MCP.md](docs/MCP.md)
for setup and client configuration.

### Cursor

Install the Rust binaries with `cargo run --quiet --manifest-path rust/Cargo.toml --bin browser-harness -- install`, then add this to `.cursor/mcp.json` in your project or `~/.cursor/mcp.json` for every project:

```json
{
  "mcpServers": {
    "browser-harness": {
      "type": "stdio",
      "command": "browser-harness-mcp"
    }
  }
}
```

## Contributing

Bug fixes, documentation improvements, focused runtime changes, and domain
skills are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md).

## Docs

- [install.md](install.md) — first-time install and browser bootstrap
- [SKILL.md](SKILL.md) — day-to-day operator/agent guide
- [docs/MCP.md](docs/MCP.md) — MCP stdio server setup
- [CONTRIBUTING.md](CONTRIBUTING.md) — development and domain-skill contributions
- [docs/architecture.md](docs/architecture.md) — runtime layout and core components
- [docs/development.md](docs/development.md) — workspace commands and verification
- [docs/wasm-guests.md](docs/wasm-guests.md) — current guest model, capabilities, and build/run flow
- [docs/future-wasm.md](docs/future-wasm.md) — long-term guest direction
- [docs/python-integration.md](docs/python-integration.md) — optional Python subprocess wrappers
- [docs/snap-linux-headless.md](docs/snap-linux-headless.md) — Snap/Flatpak/headless browser troubleshooting

## Project Structure

- `rust/` — binaries, crates, guest modules, and workspace metadata
- `domains/` — active site-specific packages, centered on `domains/<site>/skill.md`
- `interaction-skills/` — reusable browser mechanics
- `docs/` — architecture, development, and future design notes
- `scripts/` — repo maintenance helpers such as leak scanning

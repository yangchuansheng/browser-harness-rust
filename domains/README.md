# Domains

`domains/` is the active home for site-specific Browser Harness knowledge in
this Rust fork. It mirrors upstream `agent-workspace/domain-skills/` while using
Rust-native runtime operations and optional WASM guests.

## Shape

```text
domains/
  <site>/
    skill.md
    *.md
    guest/
    fixtures/
```

Rules:

- `skill.md` is the main site guide.
- Read every file in a matching `domains/<site>/` directory when
  `BH_DOMAIN_SKILLS=1` and the task is site-specific.
- Upstream `scraping.md` files are mapped to `skill.md` in this fork.
- Upstream `domain-skills/` and `agent-workspace/domain-skills/` entries are
  mapped into this `domains/` tree instead of keeping legacy duplicate roots.
- Extra notes can live beside `skill.md` for task-specific flows.
- Executable Rust guests still live under [`../rust/guests/`](../rust/guests/)
  unless a site explicitly grows its own guest package.
- Python-like helper examples are pseudocode for harness operations; they do
  not require Python at runtime.

Use these commands to discover current coverage instead of relying on a static
site list:

```bash
rg --files domains
find domains -maxdepth 1 -type d | sort
```

When adding or updating a site, prefer `domains/<site>/` and avoid secrets,
cookies, session tokens, task narration, and raw pixel coordinates.

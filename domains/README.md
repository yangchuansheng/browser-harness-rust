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
- Upstream `domain-skills/` and `agent-workspace/domain-skills/` entries map to
  `domains/<site>/skill.md`; synced upstream additions (`aviationweather`,
  `bank-of-canada`, `eurostat`, `gbif`, `oecd`, `ourworldindata`,
  `swiss-transport`, and `uniprot`, plus `census-geocoder`,
  `energi-data-service`, `faa-nas-status`, `gleif`, `icd-10-cm`,
  `npi-registry`, `osv`, and `rxnorm`) also keep verbatim mirrors at
  `agent-workspace/domain-skills/<site>/scraping.md`.
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

## Synced upstream skills

| Site | Capability |
|------|------------|
| `census-geocoder` | Geocode U.S. addresses and coordinates to county, census tract, block, and congressional-district layers. |
| `energi-data-service` | Query Danish DK1/DK2 day-ahead electricity prices and find the cheapest appliance windows. |
| `faa-nas-status` | Check U.S. airports for active ground stops, ground-delay programs, closures, reasons, and delays in the FAA XML feed. |
| `gleif` | Find legal entities by name, inspect LEI status and registration data, and list reported direct subsidiaries. |
| `icd-10-cm` | Resolve condition descriptions to ICD-10-CM codes or look up a code's official name via NLM CTSS. |
| `npi-registry` | Verify U.S. clinicians and organizations by NPI, including taxonomy, practice address, and reported state licence. |
| `osv` | Check pinned dependency versions for known vulnerabilities, aliases, severity, and fixed versions. |
| `rxnorm` | Normalize drug names to RxCUI concepts and list brands, clinical drugs, strengths, and dose forms. |

When adding or updating a site, prefer `domains/<site>/` (plus a verbatim
`agent-workspace/domain-skills/<site>/scraping.md` mirror when syncing upstream
domain skills) and avoid secrets, cookies, session tokens, task narration, and
raw pixel coordinates.

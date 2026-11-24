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
  `npi-registry`, `osv`, and `rxnorm`, plus `cpsc-recalls`, `medlineplus`,
  `openfema`, and `usgs-elevation`, plus `endoflife-date`, `epss`,
  `fdic-bankfind`, and `nyc-open-data`, plus `cisa-kev`,
  `cleveland-museum`, `library-of-congress`, and `noaa-space-weather`, plus
  `crates-io`, `data-gov-sg`, `rubygems`, and `statistics-norway`, plus
  `entur`, `geo-admin-ch`, `smhi`, and `ted-tenders`) also keep verbatim
  mirrors at `agent-workspace/domain-skills/<site>/scraping.md`.
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
| `cisa-kev` | Query the CISA KEV JSON feed for recent or vendor CVEs with due dates, required actions, and ransomware use. |
| `cleveland-museum` | Find CC0 artworks by artist or keyword with metadata, credit lines, and downloadable web/print images. |
| `cpsc-recalls` | Check whether a product has a U.S. CPSC recall, with recall number, date, hazard, units, and remedy. |
| `crates-io` | Look up a Rust crate's latest stable version, downloads, repository and dependents, or search crates by downloads. |
| `data-gov-sg` | Get Singapore town forecasts, nearest temperature and rainfall readings, and regional PSI and PM2.5. |
| `digitraffic` | Query Finnish traffic data, including road weather, cameras, train locations, and maritime notices. |
| `endoflife-date` | Check whether a software version is still supported and when it reaches end of life via the endoflife.date API. |
| `energi-data-service` | Query Danish DK1/DK2 day-ahead electricity prices and find the cheapest appliance windows. |
| `entur` | Get next departures (line, destination, expected time, delay, platform) from a Norwegian stop by name, or plan a trip between two places. |
| `epss` | Get the exploitation probability and percentile for a list of CVEs to decide which to patch first via the FIRST API. |
| `faa-nas-status` | Check U.S. airports for active ground stops, ground-delay programs, closures, reasons, and delays in the FAA XML feed. |
| `fdic-bankfind` | Check whether a bank is open and FDIC-insured, get its CERT number, assets and branches, or list recent bank failures. |
| `gleif` | Find legal entities by name, inspect LEI status and registration data, and list reported direct subsidiaries. |
| `grants-gov` | Search U.S. federal funding opportunities and retrieve opportunity details, eligibility, and deadlines. |
| `geo-admin-ch` | Geocode a Swiss address or place name to WGS84 and LV95 coordinates, get its elevation, commune, and a terrain profile. |
| `icd-10-cm` | Resolve condition descriptions to ICD-10-CM codes or look up a code's official name via NLM CTSS. |
| `library-of-congress` | Search digitized photos and maps by topic with titles, dates, image URLs, and rights notes via the JSON API. |
| `medlineplus` | Get the plain-language MedlinePlus health-topic summary and page link for a condition. |
| `noaa-space-weather` | Check current geomagnetic, solar radiation, and radio blackout levels, Kp forecasts, and active space-weather alerts. |
| `npi-registry` | Verify U.S. clinicians and organizations by NPI, including taxonomy, practice address, and reported state licence. |
| `nyc-open-data` | Get a NYC restaurant's latest health inspection score, grade and violations, or the latest 311 complaints for a ZIP code. |
| `openfema` | List FEMA disaster declarations for a U.S. county or state, with incident type and declared assistance programs. |
| `pegelonline` | Retrieve German federal waterway gauge levels, forecasts, and station metadata. |
| `osv` | Check pinned dependency versions for known vulnerabilities, aliases, severity, and fixed versions. |
| `rubygems` | Look up Ruby gem versions, downloads, licences and source links, list dependents, or search gems. |
| `rxnorm` | Normalize drug names to RxCUI concepts and list brands, clinical drugs, strengths, and dose forms. |
| `smhi` | Get the next 48 hours of hourly temperature, wind and precipitation for a Swedish town by coordinates, plus active SMHI weather warnings. |
| `statistics-norway` | Search SSB tables, inspect their variables, and pull the latest official population, CPI or unemployment figures. |
| `statistics-sweden` | Query Statistics Sweden PXWeb tables for official Swedish statistical series. |
| `ted-tenders` | Find open EU public tenders by keyword, CPV code or buyer country, with title, buyer, country, submission deadline and notice link. |
| `usgs-elevation` | Look up ground elevation for a latitude/longitude point via the USGS Elevation Point Query Service. |

When adding or updating a site, prefer `domains/<site>/` (plus a verbatim
`agent-workspace/domain-skills/<site>/scraping.md` mirror when syncing upstream
domain skills) and avoid secrets, cookies, session tokens, task narration, and
raw pixel coordinates.

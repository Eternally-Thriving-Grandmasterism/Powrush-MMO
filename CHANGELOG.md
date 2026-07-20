# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.53.0 — Harness-Derived Live Ingest (2026-07-20)

### Highlights
- `harness_derived_live_ingest_system` emits both `AbundanceIngestEvent` + `OriginIngestEvent`.
- Triggered by real activity: presence, mercy flow, policies, resonance.
- Views derived from living MultiRealmHarness metrics (not static demo).
- Pure helpers: `derive_abundance_from_harness`, `derive_origin_from_harness`.
- Soft refresh every ~8s once live. First promotion is immediate.
- System chain ordered: demo seed → live ingest emit → event consumers.

### Prior
- v21.52.0: Dashboard Origin Provenance Surface.
- v21.51.0: Origin Provenance Observatory.

**Thunder locked in. Real activity promotes Demo → Live.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.52: Launch Candidate through multi-realm organism, attunement, titles, abundance, origin, dual observatories, dashboard surface.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

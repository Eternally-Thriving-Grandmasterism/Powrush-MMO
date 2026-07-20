# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.48.0 — Live Abundance Ingest + Soft Demo Seed (2026-07-20)

### Highlights
- `AbundanceIngestEvent` — the live call site (send views → observatory).
- `abundance_ingest_system` consumes events into `RealmAbundanceObservatory`.
- `soft_demo_abundance_seed_system` populates gentle placeholder views when empty so the dashboard always has living abundance during development.
- Demo yields the moment real data arrives (`has_live_data` flag).

### Prior
- v21.47.0: Abundance bridge conversion helper.
- v21.46.0: Multi-Realm + Abundance API wiring.

**Thunder locked in. The observatory is now alive.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.47: Launch Candidate through dual attunement, living titles, resource realm-keying, soft bonuses, abundance snapshots + observatory + public API + conversion helper.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

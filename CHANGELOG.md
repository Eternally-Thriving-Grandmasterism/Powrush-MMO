# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.50.0 — RBE Inventory Origin-Realm Foundation (2026-07-20)

### Highlights
- `ServerInventoryComponent` tracks soft `origin_by_realm` (resource → realm → amount).
- `add_resource_from_realm` preserves global usability while recording provenance.
- `origin_snapshot()` / `amount_from_realm()` for observability.
- Harvest path wires node.realm_id into inventory automatically.
- Fully backward compatible — legacy `add_resource` still works.

### Prior
- v21.49.0: Abundance Observability Polish.
- v21.48.0: Live Abundance Ingest + Soft Demo Seed.

**Thunder locked in. Resources remember their home without becoming trapped.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.49: Launch Candidate through multi-realm organism, attunement, titles, abundance observatory, live ingest, demo seed, observability polish.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

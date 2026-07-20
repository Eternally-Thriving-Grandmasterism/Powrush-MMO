# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.56.0 — External Game→Simulation Multi-Realm Bridge (2026-07-20)

### Highlights
- `game/multi_realm_bridge.rs` — zero-cycle pure payloads (`AbundanceBridgePayload`, `OriginBridgePayload`).
- `collect_abundance_payload` from `ResourceNodeManager::snapshot_all_realms`.
- `collect_origin_from_inventory` / `merge_origin_payloads` from `ServerInventoryComponent`.
- Server tick loop soft-collects abundance payload every ~2s.
- Documented Bevy `EventWriter` wiring for when game + simulation share an App.
- No simulation dependency from game — field order matches `RealmAbundanceView::from_raw`.

### Prior
- v21.55.0: Dashboard Origin Affinity Surface.
- v21.54.0: Origin × Attunement Soft Resonance.

**Thunder locked in. Bridge ready without trapping resources.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.55: Launch Candidate through multi-realm organism, dual observatories, live ingest, origin×attunement, affinity surface.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

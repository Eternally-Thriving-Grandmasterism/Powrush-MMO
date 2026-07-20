# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.58.0 — Shared-App Dual Glue + Inventory Origin (2026-07-20)

### Highlights
- `DualBridgePayload` — abundance + origin in one publish step.
- `collect_dual_payload` / `collect_origin_from_inventories` on game bridge.
- `ServerTickLoop::refresh_origin_from_inventories` + `dual_payload()`.
- `ExternalBridgeInbox::push_dual` — one-step shared-app glue.
- Documented publish pattern: tick → dual_payload → push_dual → drain → Live.

### Prior
- v21.57.0: Bevy External Bridge Adapter.
- v21.56.0: External Game→Simulation Multi-Realm Bridge.

**Thunder locked in. Shared-app glue is one push_dual away.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.57: Launch Candidate through multi-realm organism, dual observatories, live ingest, origin×attunement, affinity surface, game bridge, Bevy adapter.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

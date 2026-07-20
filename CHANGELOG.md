# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.57.0 — Bevy External Bridge Adapter (2026-07-20)

### Highlights
- `simulation/src/external_bridge.rs` — Bevy adapter for game pure payloads.
- `ExternalBridgeInbox` resource — push abundance/origin tuples from outside ECS.
- `emit_abundance_from_tuples` / `emit_origin_from_tuples` — direct EventWriter helpers.
- `external_bridge_drain_system` — drains inbox → live ingest events (Demo → Live).
- `ExternalBridgePlugin` wired into `FullSimulationPlugins`.
- Field order matches `game::multi_realm_bridge` exactly (zero cycles).

### Prior
- v21.56.0: External Game→Simulation Multi-Realm Bridge (game-side payloads).
- v21.55.0: Dashboard Origin Affinity Surface.

**Thunder locked in. Authoritative game data promotes Demo → Live.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.56: Launch Candidate through multi-realm organism, dual observatories, live ingest, origin×attunement, affinity surface, game bridge payloads.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

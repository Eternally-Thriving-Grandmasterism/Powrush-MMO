# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.79.0 — NonSend ServerTickLoop + Cohost Auto-Drain (2026-07-20)

### Highlights
- `ServerTickLoop::new_sync` + export from `game` for Bevy `NonSend` wiring.
- `CohostExportMirror` auto-drains into `CouncilRttInbox` (in-process co-host).
- Sim writes `artifacts/sim_council_bridge.json`; server polls (file co-host path).
- `RathorIntegrationPlugin` chains: cohost → inbox → file bridge → RTT export.

### Prior
- v21.78: Host mapper resolved_history → export queue
- v21.77: Provenance + smoke harness

**Thunder locked in. Cohost paths live.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.78: Multi-realm sealed, RBE, council, LegacyJournal, RTT dual export.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

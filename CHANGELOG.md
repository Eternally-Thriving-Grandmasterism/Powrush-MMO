# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.80.0 — Unified Cohost Host Binary (2026-07-20)

### Highlights
- New `host/` workspace member + `powrush-host` binary.
- Single Bevy App co-hosts simulation + server with live RTT bridge:
  `CouncilRttExportQueue` (sim) → `CohostExportMirror` → auto-drain → `CouncilRttInbox` → `ServerTransferSession`.
- `server/src/lib.rs` now publicly exports `rathor_integration` for external consumers.
- Closes the primary next priority from DERIVATION_STATUS.md (v21.79).

### Prior
- v21.79: NonSend ServerTickLoop + Cohost Auto-Drain
- v21.78: Host mapper resolved_history → export queue
- v21.77: Provenance + smoke harness

**Thunder locked in. Cohost paths live + unified host binary.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.78: Multi-realm sealed, RBE, council, LegacyJournal, RTT dual export.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

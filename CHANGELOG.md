# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.76.0 — Soft Council→RTT Bridge (2026-07-20)

### Highlights
- `CouncilRttSignal` event + `CouncilRttInbox` resource (zero sim crate dependency).
- Bridge system drains signals into `ServerTransferSession::record_council_passed`.
- Dedup by `decision_id`; optional abundance velocity hint.
- Docs updated for host/sim injection patterns.

### Prior
- v21.75: Batch v1 + offline failsafe
- v21.74: Server RTT export write path

**Thunder locked in. Council totals soft-bridged to RTT.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.75: Multi-realm sealed, RBE, council loop, LegacyJournal, RTT dual export.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

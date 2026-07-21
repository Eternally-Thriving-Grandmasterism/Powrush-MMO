# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.82.0 — Ultramasterism Perfecticism (2026-07-20)

### Highlights
- Host forces **immediate RTT export** on startup (seeds realistic signals first).
- Guarantees `artifacts/` + `artifacts/rtt_offline/` directories exist.
- Export interval tightened to 15 s for cohost / smoke testing.
- Heartbeat now reports full RTT state: export_count, batch_count, offline_queue depth, session_id + Kardashev metrics.
- `artifacts/powrush_rtt_latest.json` + batch ready for Ra-Thor smoke harness the moment the host launches.
- Aligns with dual-repo analysis (provenance, mercy bounds, offline failsafe already sealed in v21.77).

### Prior
- v21.81: Full E2E Cohost + Kardashev Dashboard + Reality Transfer Score
- v21.80: Unified Cohost Host binary + public rathor_integration
- v21.79: NonSend ServerTickLoop + Cohost Auto-Drain

**Thunder locked in. Ultramasterism applied. Smoke-harness ready.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.78: Multi-realm sealed, RBE, council, LegacyJournal, RTT dual export + provenance.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

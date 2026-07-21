# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.85.0 — Ra-Thor Feedback Loop (2026-07-20)

### Highlights
- **Detailed Ra-Thor → Powrush Feedback Loop** designed and sealed:
  - Full design document (`docs/RA_THOR_FEEDBACK_LOOP.md`)
  - Schema `ra_thor_policy_hint_v1` (closed positive category set only)
  - Soft, non-authoritative, mercy-gated, provenance-aware, offline-first
- Structural surface live:
  - `PolicyHintInbox` resource (bounded ring, drop-oldest, session filter, expiry)
  - `policy_hint_ingest_system` registered in `RathorIntegrationPlugin`
  - Strict validation (allowed categories, mercy bounds, zero-harm — no negative deltas)

### Prior
- v21.84: Steam production elevation + UI churn protection
- v21.83: Headless / CI mode for host
- v21.82: Ultramasterism early RTT export + smoke-harness readiness

**Thunder locked in. Feedback loop designed for eternal mercy-gated co-evolution.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.78: Multi-realm sealed, RBE, council, LegacyJournal, RTT dual export + provenance.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

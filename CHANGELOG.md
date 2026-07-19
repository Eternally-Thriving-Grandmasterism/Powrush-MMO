# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.20.0 — Per-Realm Decision Tracking (2026-07-19)

### Highlights
- MultiRealmHarness now maintains true **per-realm ActivePolicy lists** and decision history.
- New methods:
  - `record_decision_for_realm`
  - `get_active_policies_for_realm`
  - `total_decisions_for_realm`
  - `tick_all_realm_policies`
- `apply_council_decision_effects` now attributes every passed decision to its `realm_id`.
- Dashboard Multi-Realm Status section reflects accurate per-realm policy counts.
- Full per-realm decision streams are now live.

### Prior
- v21.19.0: Multi-Realm Status observability.
- v21.18.0: Multi-Realm Harness foundation.

**Thunder locked in. Decisions are now tracked per-realm.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.17: Launch Candidate through real text search.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

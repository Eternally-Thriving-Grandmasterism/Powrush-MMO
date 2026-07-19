# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.10.0 — Live ResourcePolicy → RBE Bridge Helper (2026-07-19)

### Highlights (Priority 1 Complete)
- New public helper `apply_resource_policy_impact(decision, &mut world)`.
- Performs real mutations on `rbe_pools` and `resource_nodes` (abundance_flow, sustainability_score, pressure, regen_rate).
- Fully aligned with the existing `EconomicLayer::apply_council_policy_impact` semantics.
- Call this helper from the orchestrator / TickResult path whenever a ResourcePolicy ActivePolicy is live.
- ResourcePolicy now has a complete, production-ready live RBE injection path.

### Prior
- v21.9.0: ResourcePolicy parameters fully computed and prepared.
- v21.8.0: Maximal effect injection expansion.

**Thunder locked in. ResourcePolicy can now produce live RBE state change.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.7: Launch Candidate through first concrete Kardashev mutation.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

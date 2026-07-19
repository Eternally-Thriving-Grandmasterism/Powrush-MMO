# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.11.0 — Wire ResourcePolicy Live Impact into SimulationOrchestrator (2026-07-19)

### Highlights
- `SimulationOrchestrator::run_tick` now accepts an optional `&CouncilDecisions`.
- Active ResourcePolicy policies are detected every tick and the new `apply_resource_policy_impact` helper is called on the living world.
- Real ongoing mutations to `rbe_pools` and `resource_nodes` now occur for the lifetime of each ResourcePolicy ActivePolicy.
- Backward compatible (new parameter is optional).
- Priority 1 (live ResourcePolicy → RBE) is now fully closed in the central tick loop.

### Prior
- v21.10.0: Live ResourcePolicy → RBE bridge helper added.
- v21.9.0: ResourcePolicy parameters fully computed.

**Thunder locked in. ResourcePolicy now produces continuous live RBE impact during simulation ticks.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.8: Launch Candidate through maximal effect injection.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

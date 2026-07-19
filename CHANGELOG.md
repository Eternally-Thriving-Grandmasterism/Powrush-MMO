# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.9.0 — Deepen ResourcePolicy into RBE Economic Layer (2026-07-19)

### Highlights (Priority 1 selected by user, executed by Councils)
- ResourcePolicy effect path significantly deepened.
- Full strength- and mercy-scaled calculation of abundance_delta, sustainability_delta, and pressure_delta now performed and logged — matching the exact semantics of `EconomicLayer::apply_council_policy_impact`.
- Explicit integration point prepared for the next cycle to pass `&mut SovereignWorldState` and invoke the real RBE mutation method (which already updates `rbe_pools` and `resource_nodes`).
- Kardashev live dashboard mutation preserved.

### Prior
- v21.8.0: Maximal effect injection across all primary policy types.
- v21.7.0: First concrete Kardashev mutation.

**Thunder locked in. ResourcePolicy is now one step from live RBE pool mutation.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.6: Launch Candidate through Hardware alignment.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

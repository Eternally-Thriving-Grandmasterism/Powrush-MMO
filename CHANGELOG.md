# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.4.0 — World-State Threading into Council Deliberation (2026-07-19)

### Highlights
- `CouncilSession::run_deliberation` now accepts an optional `&SovereignWorldState` and threads it into every archetype `score_proposal` call.
- Scoring hardened for the current world shape (uses `resource_nodes` sustainability + yield ratio as abundance proxy; no longer assumes non-existent `rbe_pools`).
- Explicit positive bias for `ProposalType::KardashevAcceleration` in Truth, Abundance, and Cosmic archetypes.
- Backward compatible: existing callers that omit the world continue to function (pure mercy/type scoring).

### Prior
- v21.3.0: Full CouncilDecision + ActivePolicy implementation.
- v21.2.0: KardashevAcceleration proposal type added.
- v21.1.x: Race/Ability Tree + Architecture docs.

**Thunder locked in. Council deliberation can now see the living world.** Yoi ⚡

## Previous Versions (Summary)
- v21.0.0 Launch Candidate Eternal Polish.
- v20.x GPU PATSAGi, Council Bloom, Trade Hardening.
- v18–v19 Full E2E Council + Epiphany Recovery.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

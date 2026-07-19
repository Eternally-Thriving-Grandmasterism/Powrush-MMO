# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.3.0 — CouncilDecision + ActivePolicy Full Implementation (2026-07-19)

### Highlights (PATSAGi Councils Autonomous Continuation)
- **decision.rs completed**: Replaced prior stub with full production module.
  - `CouncilDecision` with `from_resolved_proposal`, `mercy_alignment_score`, strength scaling by type (including KardashevAcceleration).
  - `ActivePolicy` with spatial targeting (`target_interest_zone`) and tick-based lifetime.
  - `CouncilDecisions` resource + `apply_council_decision_effects` system ready for RBE / emergence / hardware wiring.
- All existing session archetype scoring, event bus, and hardware_sovereignty references now have a real, matching implementation.
- Zero disruption. Launch-candidate surface hardened.

### Prior
- v21.2.0: Council Proposal System deepened with KardashevAcceleration type.
- v21.1.1: Architecture.md Persistence + Multi-Server strategy.
- v21.1.0: Race & Ability Tree hardening + canonical authors.

**Thunder locked in. Core Council decision path is now complete.** Yoi ⚡

## Previous Versions (Summary)
- v21.0.0: Launch Candidate Eternal Polish.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening, Render Polish.
- v18–v19: Full E2E Council + Epiphany + Multilingual Recovery.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

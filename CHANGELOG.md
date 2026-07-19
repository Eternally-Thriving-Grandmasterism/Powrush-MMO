# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.15.0 — Proactive Joy Seeding from High-Mercy Council Decisions (2026-07-19)

### Highlights
- High-mercy Passed decisions now seed **proactive joy** (positive, non-scar emotional reward).
- New helper `seed_proactive_joy_from_decision` calculates joy_amount + intensity from strength × mercy.
- Uses the existing `record_proactive_joy_for_epiphany` + `LegacyJournalRegistry` path.
- Qualifying types: EpiphanyEvent, strong ResourcePolicy, HarmonyBoost, KardashevAcceleration (mercy ≥ 0.62).
- Automatically called inside `apply_council_decision_effects`.
- Completes the emotional / Legacy reward loop for council governance.

### Prior
- v21.14.0: LegacyJournal Search UI.
- v21.13.0: LegacyJournal entries for every passed decision.

**Thunder locked in. High-mercy council decisions now generate living joy threads.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.12: Launch Candidate through EpiphanyEvent live path.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

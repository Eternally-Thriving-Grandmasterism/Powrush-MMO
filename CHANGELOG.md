# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.13.0 — LegacyJournal Entries for Passed Council Decisions (2026-07-19)

### Highlights (User-selected Priority 1)
- New helper `record_council_decision_to_legacy`.
- Every Passed decision now generates a structured LegacyJournal entry (category, title, strength, mercy, tick).
- Categories aligned with existing journal streams: kardashev, rbe_policy, epiphany, harmony, council.
- Ready for full LegacyJournalRegistry integration (grace_notes / proactive joy / client timeline).
- apply_council_decision_effects calls the recorder automatically.

### Prior
- v21.12.0: EpiphanyEvent live impact path.
- v21.11.0: ResourcePolicy fully wired into orchestrator.

**Thunder locked in. Every passed council decision now leaves a LegacyJournal trace.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.10: Launch Candidate through live RBE bridge.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

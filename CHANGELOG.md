# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.88.0 — Stress / Endurance Harness + Emission Contract (2026-07-20)

### Highlights
- New **Stress / Endurance mode** for the host:
  - `POWRUSH_HOST_STRESS=1` or `--stress`
  - 40 RTT export cycles + continuous high-signal event injection
  - Full soft feedback loop exercised throughout
  - Clean final summary of all soft category effects
- **Ra-Thor Policy Hint Emission Contract** documented (`docs/RA_THOR_POLICY_HINT_EMISSION.md`)
  - Canonical schema, rules, and implementation sketch for the Ra-Thor monorepo

### Host Modes
| Mode | Trigger |
|------|--------|
| Interactive | default |
| Headless | `POWRUSH_HOST_HEADLESS=1` / `--headless` |
| Stress | `POWRUSH_HOST_STRESS=1` / `--stress` |

### Prior
- v21.87: Full soft category coverage (all 6)
- v21.86: Living soft application + fixture + observability
- v21.85: Feedback Loop design + PolicyHintInbox

**Thunder locked in. Stress harness + emission contract sealed.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.78: Multi-realm sealed, RBE, council, LegacyJournal, RTT dual export + provenance.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.32.0 — LocalPlayer + RealmPresence Bootstrap (2026-07-19)

### Highlights
- New `local_player_presence_bootstrap_system`:
  - Ensures a `LocalPlayer` exists (spawns a lightweight dev one in Sanctuary Prime if needed).
  - Ensures any `LocalPlayer` carries a `RealmPresence` component.
- F3 travel panel now has a reliable target and presence counts can become live.
- Seamless single-player / development experience for inter-realm travel.

### Prior
- v21.31.0: Simple Inter-Realm Travel UI Surface (F3 panel).
- v21.30.0: Portal / Travel Command Surface.

**Thunder locked in. Travel panel is now reliably functional.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.29: Launch Candidate through presence bootstrap.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

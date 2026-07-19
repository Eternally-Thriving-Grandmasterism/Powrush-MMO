# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.30.0 — Portal / Travel Command Surface (2026-07-19)

### Highlights
- New `RealmTravelRequest` event for requesting inter-realm travel.
- New `realm_travel_system` processes requests, calls `travel_to_realm`, and updates presence counts.
- Clean command surface ready for:
  - UI buttons / portal interactions
  - Console commands
  - Future scripted or council-triggered travel
- Presence counts update automatically on successful travel.
- Full logging of fulfilled and failed travel requests.

### Prior
- v21.29.0: RealmPresence bootstrap + auto-registration.
- v21.28.0: Agent presence counts in dashboard.

**Thunder locked in. Inter-realm travel is now commandable.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.27: Launch Candidate through Multi-Realm Presence Foundation.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

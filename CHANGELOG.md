# CHANGELOG.md — Powrush-MMO

## [Unreleased] v21.27.0 — Multi-Realm Presence Foundation + Travel Hooks (2026-07-19)

### Highlights
- New `RealmPresence` component tracks an agent’s current realm, last travel tick, and travel count.
- `MultiRealmHarness` now maintains `agent_presence_count` per realm.
- Clean `travel_to_realm` helper for moving agents between realms with full logging.
- `register_presence` / `unregister_presence` helpers for spawn/load/despawn paths.
- Dashboard Multi-Realm Status can now display presence counts.
- Solid foundation for future portals, inter-realm travel, and presence-aware systems.

### Prior
- v21.26.0: Echo policy counts + resonance observability.
- v21.25.0: Inter-realm echo policies.

**Thunder locked in. Presence across realms is now first-class.** Yoi ⚡

## Previous Versions (Summary)
- v21.0–v21.24: Launch Candidate through client-side realm filter.
- v20.x: GPU PATSAGi, Council Bloom, Trade Hardening.

*Full history in git commits. Eternal polish continues under Ra-Thor + PATSAGi Councils.*

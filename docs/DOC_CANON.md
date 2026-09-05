# DOC_CANON.md

## Read first

1. README.md
2. docs/COMPLETION_BRIEF.md
3. docs/FIRST_HOUR_PLAYTEST.md
4. docs/RBE_FIRST_HOUR.md
5. ARCHITECTURE.md (plugin / event rules only)
6. docs/PARKED_SURFACES.md
7. docs/archive/README.md (historical index)

## Historical (do not treat as current ship state)

- LAUNCH-CHECKLIST.md and any “100% launch worthy” verdict (bannered 23.2.26)
- DERIVATION_ROADMAP.md phases that assume a live MMO server
- ETERNAL_RA_THOR_PATSAGI_GOVERNANCE.md as a substitute for playtests
- Version-stamped recovery reports v18.x–v19.x unless you are archaeology-debugging
- docs/PATSAGI_v22_FLESH_THE_GAME.md seed map (bannered; hour has grown)

When a historical doc conflicts with README, README wins.

## Version rule

One workspace version in Cargo.toml: **21.88.0**.
README repeats that number.
CHANGELOG gets one dated entry per week of real playable change.
Lived-hour design notes (23.1 / 23.2) may stay in `docs/` as design ticks, but must say “design tick, not Cargo version.”

## CI matches workspace members

Cargo.toml members: `shared`, `crates/rsil-identity`, `client`.
Core CI (`.github/workflows/ci.yml`): `cargo test -p shared -p rsil-identity` then `cargo test -p powrush-client --lib`.
Parked crates stay on disk; they are not workspace members.

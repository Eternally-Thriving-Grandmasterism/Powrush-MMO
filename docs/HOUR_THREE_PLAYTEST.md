# HOUR_THREE_PLAYTEST.md

Run after steward proceed (or a filled Hour-two Report) and Hour three hands are on main. Same binary. No server.

## Build

```bash
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
cargo run -p powrush-client
```

## MACHINE QA

Not Felt minutes. OS, GPU, and steward playtest minutes stay blank.

```bash
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
```

CI job "Core + Q2 one-frame" is already the CI gate. Confirmed by CI on the PR.

Tip `07490761`:

- `cargo test -p shared -p rsil-identity`: PASS — shared 455 passed; rsil-identity 5 passed; 0 failed
- `cargo test -p powrush-client --lib`: PASS — 625 passed; 0 failed

capture: none on tip

No screenshot or tape capture on tip. Headless Q1 script-run (`./scripts/play-offline.sh --script-run`) is cited in `docs/FIRST_HOUR_PLAYTEST.md` MACHINE QA.

MACHINE QA GREEN

## Script

1. Resume with Hour two held. Welcome slab names the yard. Card skips WASD.
2. Card says plant the fabricator. Press Q through MendSpool + LaneCrate until Proof Pack unlocks.
3. Embassy lamp: E Request seat. Book shows MendSpool + LaneCrate.
4. Card / slab: *Hour three held*. Quit. Rerun. Seat + book survive.
5. H still hides. Peace E still harvests.

## Pass / fail

Fail if Hour three unlocks without steward proceed or an Hour-two timing note on record.
Fail if a second HUD appears.
Fail if server is required.
Fail if G / Tab War / Ledger 3 / Crownstone are required to finish this script.
Fail if quit drops Proof Pack or Embassy seat.

## Steward note

Steward approved proceed 2026-09-05; human timing to be filled on next play.

## Report

OS / GPU:
Commit:
Time Proof Pack → seat:
Survived quit: yes/no
Confusion point:

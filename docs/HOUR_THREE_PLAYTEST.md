# HOUR_THREE_PLAYTEST.md

Run only after Hour two timing note exists and Hour three hands are on main. Same binary. No server.

## Build

```bash
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
cargo run -p powrush-client
```

## Script

1. Resume with Hour two held. Welcome slab names the yard. Card skips WASD.
2. Card says plant the fabricator. Press Q through MendSpool + LaneCrate until Proof Pack unlocks.
3. Embassy lamp: E Request seat. Book shows MendSpool + LaneCrate.
4. Card / slab: *Hour three held*. Quit. Rerun. Seat + book survive.
5. H still hides. Peace E still harvests.

## Pass / fail

Fail if Hour three unlocks without an Hour-two timing note on record.
Fail if a second HUD appears.
Fail if server is required.
Fail if G / Tab War / Ledger 3 / Crownstone are required to finish this script.
Fail if quit drops Proof Pack or Embassy seat.

## Report

OS / GPU:
Commit:
Time Proof Pack → seat:
Survived quit: yes/no
Confusion point:

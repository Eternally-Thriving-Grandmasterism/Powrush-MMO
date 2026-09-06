# HOUR_TWO_PLAYTEST.md

Run after the first-hour script. Same binary. No server.

## Build

```bash
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
cargo run -p powrush-client
```

## Script

1. Finish first hour: walk · E · I · R 1 or 2.
2. Card says *Tab the ridge*. Press Tab. Are you a Peace visitor? Does E say *Not your charter*?
3. Card says *Q plant a House stake*. Press Q. Does the spill slab appear (I2 / Offline extractor)?
4. Card says *L opens the Ledger*. Press L. Press E until Settled (Bind, not 3).
5. Card says *Hour two held*. Quit. Rerun. Welcome slab: *Welcome back · Hour two held · the yard remembers*. Card skips WASD. Yard still in `data/powrush_hour_two.json`.
6. H still hides the card. World slabs still speak.

## Pass / fail

Fail if Tab does nothing after allocate.
Fail if Q in Peace founds a House.
Fail if E on the visitor ridge harvests.
Fail if L in Peace opens the Ledger.
Fail if quit drops House / I2 / Settled.
Fail if rerun re-teaches WASD after the pack is held.
Fail if rerun stays silent when the pack is held.
Fail if a second HUD appears.
Fail if Embassy / Crownstone is required to finish this script.

## Report

OS / GPU:
Commit:
Time Tab → Settled:
Yard survived quit: yes/no
Welcome slab named the yard: yes/no
Card skipped WASD: yes/no
Confusion point:

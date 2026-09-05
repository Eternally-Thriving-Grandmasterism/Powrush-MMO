# FIRST_HOUR_PLAYTEST.md

Run this on a clean machine. No Ra-Thor checkout.

## Build

```bash
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
cargo run -p powrush-client
```

## Five-minute script

1. Window opens. One card. Can you tell you are in a climate without reading a doc?
2. WASD walk. Space jump. Shift sprint. Card should step to “Walk to a glow.”
3. Find a glow. Press E. Did you get glow + camera punch + rumble on first take? Slab names Idle / Glowing / Tended / Resting / Stressed.
4. Press I. Is the take in the satchel?
5. Press R, then 1 (flow) and 2 (reserve). Did anything in the world change?
6. Press H. Can you still tend without the guidance card?
7. Quit. Rerun. Are satchel and allocation still there?
8. Teaching claim on Sanctuary: E twice on a glow — slab *extract left it tired*. Then R 1 — *flow restored the well*.

## Pass / fail

Fail if E does nothing visible.
Fail if I is empty after a successful tend.
Fail if R does not change the field or the satchel rights.
Fail if the client requires a server process.
Fail if a second HUD appears before the first tend.
Fail if the card is a manifesto.
Fail if extract-only never tires a well, or flow never restores one.

## Report template

OS / GPU:
Commit:
Time to first tend:
E feedback: yes/no
Satchel correct: yes/no
Allocate visible: yes/no
Save/load: yes/no
Card hid with H: yes/no
Extract tired the well: yes/no
Flow restored the well: yes/no
Confusion point:
Suggested one-line fix:

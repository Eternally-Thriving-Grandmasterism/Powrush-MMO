# FIRST_HOUR_PLAYTEST.md

Run this on a clean machine. No Ra-Thor checkout.

## Build

```bash
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
cargo run -p powrush-client

Five-minute scriptWindow opens. Can you tell you are in a climate without reading a doc?
WASD walk. Space jump. Shift sprint.
Find a glow. Press E. Did you get glow + camera punch + rumble on first take?
Press I. Is the take in the satchel?
Press R, then 1 (flow) and 2 (reserve). Did anything in the world change?
Press H. Can you still tend without the guidance card?
Quit. Rerun. Are satchel and allocation still there?

Pass / failFail if E does nothing visible.
Fail if I is empty after a successful tend.
Fail if R does not change the field or the satchel rights.
Fail if the client requires a server process.
Fail if a second HUD appears before the first tend.

Report template

OS / GPU:
Commit:
Time to first tend:
E feedback: yes/no
Satchel correct: yes/no
Allocate visible: yes/no
Save/load: yes/no
Confusion point:
Suggested one-line fix:


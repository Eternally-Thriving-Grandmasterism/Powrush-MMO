# FIRST10-AUDIT-1 · first ten minutes, frame and source walk

Card FIRST10-AUDIT-1 (ONBOARD), Core CUT on #738 comment 7089813267, at main `f02006acd0a8212417fcb06616ab2cdd5686feb1`.

## Evidence

Frames come from CI artifact `powrush-packaged-frames-f02006ac`, offline-build run `45091445987` (push, head `f02006ac`, success). They are not committed. Its `CAPTURE-ID.txt` reads: `1920x1080 lavapipe yard frames after Return; not a real-GPU or store claim`. Lavapipe frames are not a hardware claim.

| Frame | sha256 |
| --- | --- |
| frame-01.png | `6fa267a9ad94a4f48057d30aa44cdff5112e97054b11e0193f53c8f7dc50c60b` |
| frame-02.png | `3e7c3b823562f59f80092f9a80d7cbbd704232033a34efe2d814b387157b6282` |
| frame-03.png | `54057a1cc6e4c8bf283a2fac2066bead65508393850440863c2eeee45b1dce3d` |
| frame-04.png | `1fda5f29e5eb11626eac6806297ba1c6925caf7e6e4a05fbd18b48d60e5e482a` |
| frame-05.png | `4ee03d29c5cd7384a87260f37039dfa8a277f640772cf0f0e9b0bab26ac989f3` |
| frame-06.png | `e3f4b78510dad244d629cfd2f20e7587510f41707570d5f815f7a87ecc289379` |
| CAPTURE-ID.txt | `9a5fcd8cf03c8b03cdca8f9b373afdfd16c144e32a03d9ae6b7e35875898acc9` |

The capture (`scripts/capture-packaged-frames.sh:39-40`) grabs a pre-Return frame as `.pre.png`, sends one XTEST `Return` (:40), then grabs six idle yard frames about 3 s apart (:41-48). The upload step (`.github/workflows/offline-build.yml:205-207`, `actions/upload-artifact@v4` with no `include-hidden-files`) does not include the dot-named `.pre.png`, so **no Title frame is in the artifact**. The run log only records `pre frame 1920x1080 colours=74797; sending XTEST Return` and `pre/post AE=2073600`. Title rows below are therefore SOURCE-ONLY.

No input is sent after Return, so the frames show an idle yard only: no walk, take, tend, satchel, allocate or Heartwood.

What the frames show:
- All six frames: Sanctuary yard (cobbles, pines, mountains), other figures moving between frames, glowing yellow orbs. The bottom-right card reads `WASD walk · Space jump · Shift sprint  · H hides`.
- frame-01: the top-left slab reads `Sanctuary · Sheltered`.
- frames 02-06: the top-left slab reads `Sanctuary · Sanctuary ember is Glowing · warm-gold yard · the yard holds peace · Human · the yard needs tending or the well goes quiet`, wrapped onto two lines.

Status key: CONFIRMED-IN-FRAME (seen in a frame above), SOURCE-ONLY (read in source, not visible in any frame), NEEDS-REAL-GPU (needs Sherif or a tester he picks on real hardware, usually motion, input feel or rumble).

Minutes are estimates for a first-time player, not measurements.

The pre-Return Title frame is not in the artifact (the hidden `.pre.png` is not uploaded), so Title rows stay SOURCE-ONLY per Core RULING 7089875966.

## A. Scripted run (`docs/FIRST_HOUR_PLAYTEST.md:91`)

| # | Step | Min (est.) | Friction | Evidence | Status |
| --- | --- | --- | --- | --- | --- |
| A1 | Timeline clock starts at launch, before Title | 0 | First row must allow for boot time | `client/src/input.rs:663` | SOURCE-ONLY |
| A2 | A Use rise on Title starts Play | 0-0.5 | The doc cites `title_screen.rs:2383-2429`, which now points at persona menu code; the handler is `scripted_title_play` | `client/src/title_screen.rs:2809` | SOURCE-ONLY |
| A3 | Tend hold outlasts 0.42 s | 0.5 | The doc cites `first_harvest_epiphany.rs:49`, `:162`; the constant now sits at :72 | `client/src/first_harvest_epiphany.rs:72` | SOURCE-ONLY |
| A4 | Passing the last row marks one window close | end | None observed | `client/src/input.rs:516-518`, `:651-653` | SOURCE-ONLY |

## B. Hour 1 verbs (`docs/FIRST_HOUR_PLAYTEST.md:119`)

| # | Verb | Min (est.) | Friction | Evidence | Status |
| --- | --- | --- | --- | --- | --- |
| B1 | Walk card text: WASD · Space jump · Shift sprint | 0-1 | None; the card reads as the doc says | frames 01-06; `client/src/first_session_guidance.rs:390`, `:853` | CONFIRMED-IN-FRAME |
| B2 | Body moves, stops on release; idle camera glance | 0-1 | Not exercised, no input after Return | `client/src/main.rs:2` (key list) | NEEDS-REAL-GPU |
| B3 | Breath: wells pulse; tended hint *let it breathe · R 1 flow* | 1-3 | None; hint lives in `shared/climate_node.rs:34`, rendered at `client/src/climate_visible.rs:444-445` | `shared/climate_node.rs:34`, `client/src/climate_visible.rs:444-445`, `:1898`, `:1901` | SOURCE-ONLY |
| B4 | Take: tap E, prompt *tap E take · hold E tend* | 1-3 | Prompt is source-only; glow, camera punch and rumble are motion or pad | `client/src/first_harvest_epiphany.rs:197`, `client/src/harvest_feel.rs:121` | NEEDS-REAL-GPU |
| B5 | Tend: hold E about 0.42 s, *tended — the node breathes* | 2-4 | None found | `client/src/first_harvest_epiphany.rs:72`, `:220` | SOURCE-ONLY |
| B6 | Satchel: I | 3-5 | None found | `client/src/soft_play_bindings.rs:28`, `client/src/human_inventory.rs:235` | SOURCE-ONLY |
| B7 | Allocate: R then 1 flow · 2 reserve | 4-6 | The card wording is *R share · 1 gives to flow, 2 keeps in reserve*, not the doc's *flow / reserve* | `client/src/soft_play_bindings.rs:30`, `client/src/first_session_guidance.rs:394` | SOURCE-ONLY |
| B8 | Hide: H hides the card | any | Hint `H hides` is visible; hiding itself was not exercised | `client/src/first_session_guidance.rs:1197` | SOURCE-ONLY |

## C. Five-minute script (`docs/FIRST_HOUR_PLAYTEST.md:191`)

| # | Step | Min (est.) | Friction | Evidence | Status |
| --- | --- | --- | --- | --- | --- |
| C0 | Title shows Play / Continue / Settings | 0 | No Title frame in the artifact (see Evidence) | `client/src/title_screen.rs:216`, `:223`, `:2763` | SOURCE-ONLY |
| C1 | Window opens, one card, climate readable without a doc | 0-1 | Place and tier read at once (`Sanctuary · Sheltered`), then a five-clause climate line wraps to two lines at 1080p | frame-01, frames 02-06; `client/src/climate_visible.rs:547`, `:663`; `client/src/first_session_guidance.rs:139` | CONFIRMED-IN-FRAME |
| C2a | Before walking, the card reads the walk prompt | 0-1 | None; holds across all six idle frames | frames 01-06; `client/src/first_session_guidance.rs:390` | CONFIRMED-IN-FRAME |
| C2b | After walking, the card steps to *Walk to a glow* | 1 | Advances after `moved_distance > 4.0`; not exercised | `client/src/first_session_guidance.rs:391`, `:422`, `:679` | SOURCE-ONLY |
| C3 | Tap E on a glow: glow, camera punch, rumble; slab state names | 1-3 | Motion and pad feedback | `client/src/first_harvest_epiphany.rs:197`, `client/src/harvest_feel.rs:121` | NEEDS-REAL-GPU |
| C4 | Hold E: the node breathes; prompt still names tap vs hold | 2-4 | None found | `client/src/first_harvest_epiphany.rs:197`, `:220` | SOURCE-ONLY |
| C5 | I: the take is in the satchel | 3-5 | None found | `client/src/human_inventory.rs:235` | SOURCE-ONLY |
| C6 | R, 1, 2: flow changes a tired well, reserve banks rights | 4-6 | Card wording differs from the doc (see B7) | `client/src/first_session_guidance.rs:394` | SOURCE-ONLY |
| C7 | H: still tend without the card | 5-7 | None found | `client/src/first_session_guidance.rs:1197` | SOURCE-ONLY |
| C8 | Quit, rerun: satchel and allocation persist | 7-9 | None found | `client/src/local_session_persist.rs:229` | SOURCE-ONLY |
| C9 | Teaching claim: E twice, *extract left it tired*; R 1, *flow restored the well* | 8-10 | The source lines carry the place prefix (`Sanctuary Prime · …`) | `client/src/climate_script.rs:40-41` | SOURCE-ONLY |

## Counts

CONFIRMED-IN-FRAME 3 · SOURCE-ONLY 17 · NEEDS-REAL-GPU 3 · total 23.

## Candidate ONBOARD-FIX rows (candidates only; Core cuts)

| # | Candidate | PATH | Rough size |
| --- | --- | --- | --- |
| 1 | Re-pin the drifted cites in the playtest doc (A2, A3) | `docs/FIRST_HOUR_PLAYTEST.md` | about +2/−2 |
| 2 | Upload the pre-Return Title frame with the capture, so Title rows can be judged in frame | `.github/workflows/offline-build.yml` | about +1/−1 |
| 3 | Fit the yard climate line on one row at 1080p (layout only, no new words) | `client/src/climate_visible.rs` | about +10/−5 |

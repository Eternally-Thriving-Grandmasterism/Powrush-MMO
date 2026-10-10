# FIRST_HOUR_PLAYTEST.md

Peak memory this hour must leave (cite [`docs/PEAK_MEMORY_LAW.md`](PEAK_MEMORY_LAW.md)):
I walked to a well. I tended it. The week was the bill. I quit. The yard remembered.
Not a council catalog. Not a LAN fake. Title Online grey.

**CARD PLAYTEST-1** · tip `03a0c6e0` (main · BRIEF-P1 #445) · design tick · not a Cargo bump  
**Seat:** PLAYTEST-1 · **Mode:** routine · **Product:** lived stranger hour  
**Core/human gate.** OS / GPU / commit / minutes / confusion stay **blank**. Agents do not invent a walk. A human fills the Report on a real GPU.  
**Floor** `2163551` · tag `playable-preview` `11c577e` · workspace `21.88.0` · **Title Online grey**  
**Contact:** info@Rathor.ai · **Independent of xAI.** No certification / AGSi warranty / legal-product claims.

AG-SML: personal play free; org use licensed. Do not invent in-world cash the software cannot pay. Purse = **flow + repair-rights**. Violence is opt-in only (Ledger **3** / harm row after Settled + book — **not** this hour).

This is the stranger-hour note. Do not fork it into a second `PLAYTEST.md`. Hour two verify lives here as a pointer; the full Tab → Settled script stays [`HOUR_TWO_PLAYTEST.md`](HOUR_TWO_PLAYTEST.md). Stop before Hour three.

Run on a clean machine. No Ra-Thor checkout. No server.

## Build (one block)

```bash
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
cargo run -p powrush-client
```

Same door, Offline forced: `./scripts/play-offline.sh` (or `./scripts/play-offline.ps1`). CARD Q1 six-landing machine pass (no interactive walk, no WASD): `./scripts/play-offline.sh q1` or `./scripts/play-offline.sh --script-run`. Windows twin stays the interactive Offline door — cite only; no new script. `POWRUSH_NET` unset or `off`. Title Online stays grey. Do not set `POWRUSH_NET=on`.

## MACHINE QA

Not Felt minutes. OS, GPU, and steward playtest minutes stay blank.

**CARD CLERK-MACHINE-CAPTURE.** Tip `bc1a154` (`bc1a1548f0cc961bca5e6777dbee29a565c6dfd6`). Design tick, not a Cargo bump.

The machine gate is these three. OS, GPU, and steward minutes stay blank. `capture: none` stays honest.

```bash
./scripts/play-offline.sh
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
```

`./scripts/play-offline.sh` forces `POWRUSH_NET=off`. With no args it is the Offline door (`cargo run -p powrush-client`). `q1` / `--script-run` is the headless door: those same two `cargo test` lines, then the named `q0_*` landings. No WASD. Title Online stays grey.

```bash
cargo test -p shared -p rsil-identity
cargo test -p powrush-client --lib
```

CI job "Core + Q2 one-frame" is already the CI gate. Confirmed by CI on the PR.

Tip `07490761`:

- `cargo test -p shared -p rsil-identity`: PASS — shared 455 passed; rsil-identity 5 passed; 0 failed
- `cargo test -p powrush-client --lib`: PASS — 625 passed; 0 failed

capture: none on tip

headless Q1 script-run · stdout (no output file; temp log deleted) at tip 07490761

```
CARD Q1 SCRIPT-RUN 2026-09-25
door: ./scripts/play-offline.sh --script-run · POWRUSH_NET=off · no cargo run · no WASD
verified: cargo test -p shared -p rsil-identity && cargo test -p powrush-client --lib (named q0_* landing proofs)
core shared+rsil-identity: PASS
core powrush-client --lib: PASS
Human / Sanctuary yard: PASS
Ambrosian / Sanctuary well-from-above: PASS
Cydruid / Heartwood: PASS
Quellorian / Threshold: PASS
Draek / Depths (teal way-home): PASS
Garden / light path: PASS
```

### ODD-ZONE-NOTE (design note · not Felt minutes)

Cite only `client/src/hex_travel.rs`. Spoken rooms: Sanctuary, Heartwood, Threshold-near, Depths. Disk ids stay `PlaceId::Sanctuary` (`sanctuary`), `PlaceId::Heartwood` (`heartwood`), `PlaceId::Depths` (`depths`). Threshold loads the Heartwood file. The Q1 line `Quellorian / Threshold` is a People landing on that shared disk, not a fifth room and not a PlaceId.

Three odd zones, hush only. Peak memory may be cited: walked · tended · week was the bill · yard remembered.

- Rare chapel — Sanctuary (`PlaceId::Sanctuary`). A seldom chapel in the yard hush. Not a new room.
- Reused unused room — Heartwood · shelf reach (Threshold-near, not a PlaceId). An odd unused room already on that shelf, reused. Not a separate Place.
- Rushed river — Depths (`PlaceId::Depths`). Water already on the teal way-home. Not a new hex.

No fifth Place · no new PlaceId · reuses existing hex_travel Places only · design note, not Felt minutes · capture: none on tip

No screenshot or tape capture on tip. Headless Q1 script-run (`./scripts/play-offline.sh --script-run`) is cited above.

MACHINE QA GREEN

## Scripted run

CARD SCRIPT-RUN-DOC-1 (PG-08). Checked against `4dcbb50e`.

A `--script` timeline is plain text. Blank lines are skipped. A `#` comment is a line whose first character after trim is `#`. Every other line is `seconds move_x move_y use_held`, split on whitespace, commas, or both, empty pieces dropped, exactly four fields. `seconds`, `move_x`, and `move_y` are finite numbers. `use_held` is the token `0` or `1`. Each line holds until the next one. Before the first timestamp the three columns are 0. Equal timestamps: the later line wins. The last row is the greatest timestamp (`client/src/input.rs:509-514`, `:579-614`, `:627-642`).

The timeline clock starts at app launch, not when the Title screen appears, so the first row's seconds must allow for boot/loading time before Title (`Time::elapsed_secs_f64` at `client/src/input.rs:663`).
```
# Title Use rise
0.5 0 0 1
# release before the first yard hold
1.0 0 0 0
# short walk
2.0 1 0 0
# tend hold (stays down until the end row)
3.0 0 0 1
# end
4.0 0 0 0
```

A Use rise on Title starts Play (PR #723, `client/src/title_screen.rs:2383-2429`). That frame clears `interact` and `interact_held`, and both stay clear until Use reads released (`client/src/title_screen.rs:2398-2405`, `:2428-2430`), so the `use_held` 0 row comes before the first yard hold. The 3.0 row stays held until 4.0, longer than the 0.42s tend (`client/src/first_harvest_epiphany.rs:49`, `:162`).

`scripts/play-offline.sh --script-walk <timeline>` keeps `POWRUSH_NET=off` (`scripts/play-offline.sh:12-13`, `:105-117`). The script `cd`s to the repo root, then a relative timeline is prefixed with `$OLDPWD`, the caller's directory (`scripts/play-offline.sh:8-9`, `:108-110`).

`.cursor/run-client-headless.sh --script <timeline>` `exec`s `cargo run -p powrush-client -- --script`, so the script returns the client's exit code, and it does not read `POWRUSH_Q2_FRAME` (`.cursor/run-client-headless.sh:60-72`).

When the clock passes the last row, one window close is sent. In the yard that close saves the house file and the lived hour, then the game exits on its own (PR #724, `client/src/input.rs:516-518`, `:651-653`, `:668-669`, `:693-705`; `client/src/title_screen.rs:1355-1357`, `:2680-2700`). With no `--script`, no timeline resource is inserted, those systems do not run, and nothing changes for players (`client/src/main.rs:35-36`, `:100-101`; `client/src/input.rs:96`, `:99-111`).

## Hour 1 verbs (keys only)

One card. World sentences, not a wiki.

| Hands | Key | Must see |
| --- | --- | --- |
| Walk | WASD · Space jump · Shift sprint | Body moves; stop on release. Idle a breath and the camera may glance at a glow — stick still wins. |
| Breath | (no extra key) | Wells pulse. After tend the world says the node breathes. Tended hint: *let it breathe · R 1 flow*. Sprint spends breath; stand recovers. Not a stamina HUD. |
| Take | **tap E** (release before ~0.4s) | Glow + camera punch + rumble on first take. Slab Idle / Glowing / Tended / Resting / Stressed. Prompt: *tap E take · hold E tend*. |
| Tend | **hold E** (~0.42s in range) | Harmony / vitality returns. World: *tended — the node breathes*. Hold-E is care, not a second harvest. |
| Satchel | **I** | The take is in the satchel. Empty satchel after a take is fail. |
| Allocate | **R** then **1** flow · **2** reserve | **1** flow restores a tired well (*flow restored the well*). **2** reserve banks repair-rights; the field does not spend. Neither verb is gold / sell / Market. |
| Hide | **H** | Card hides. Wells still speak. |

Peace keys stay WASD / E / I / H / R. No F-row. No second HUD. No fake “players online.” No kill-score. No CBDC-style lock. No new currency. No pockets.

## Machine beats (CARD Q0)

`--lib` proves the listed beats without a human. Do not invent minutes, screenshots, or ffmpeg. Report fields below stay **blank**. Named tests in `client/src/lib.rs`:

| Beat | Test |
| --- | --- |
| Title Play / Continue / Settings · Online grey | `q0_title_play_continue_settings_online_grey` |
| new soul = light · sealed Continue = dress | `q0_new_soul_light_sealed_continue_dress` |
| wrong door / decline = still light (S2) | `q0_wrong_door_decline_still_light` |
| each People landing + S1/F7 aftermath line | `q0_each_people_landing_s1_f7_aftermath_line` |
| F1 stance four values · garden = no stance | `q0_f1_stance_four_values_garden_no_stance` |
| F2 window only if Open-trade · ghost lots offline | `q0_f2_window_only_if_open_trade_ghost_lots_offline` |
| F3 Hostile Take allowed · F9 NEVC label · no lockout | `q0_f3_hostile_take_allowed_f9_nevc_label_no_lockout` |
| F4 dress stays · serve other well offline | `q0_f4_dress_stays_serve_other_well_offline` |

## Scripted pass (CARD Q1)

Offline door `q1` / `--script-run` proves the six landings already named in the Q0 table without a human walk, WASD, `cargo run`, screenshot, or ffmpeg. Garden / light is the God-plane door host, **not** a PlaceId. Report fields below stay **blank**.

Door: `./scripts/play-offline.sh q1` or `./scripts/play-offline.sh --script-run`. `POWRUSH_NET=off`. Named proofs stay the Q0 `--lib` tests in `client/src/lib.rs`.

| Landing / beat | Proof |
| --- | --- |
| Human / Sanctuary yard | `q0_each_people_landing_s1_f7_aftermath_line` |
| Ambrosian / Sanctuary well-from-above | `q0_each_people_landing_s1_f7_aftermath_line` |
| Cydruid / Heartwood | `q0_each_people_landing_s1_f7_aftermath_line` |
| Quellorian / Threshold | `q0_each_people_landing_s1_f7_aftermath_line` |
| Draek / Depths (teal way-home) | `q0_each_people_landing_s1_f7_aftermath_line` |
| Garden / light path (not a PlaceId) | `q0_new_soul_light_sealed_continue_dress` · `q0_f1_stance_four_values_garden_no_stance` · garden aftermath in the People test |

PASS/FAIL lines are appended only after that door runs. Agents do not invent OS / GPU / minutes.

After one Tend, a door may name the people and the landing; Human and Ambrosian share Sanctuary; crossing is one way this session.

### CARD Q1 SCRIPT-RUN 2026-09-22

Door: `./scripts/play-offline.sh --script-run` · `POWRUSH_NET=off` · no interactive walk · no WASD  
Verified: `cargo test -p shared -p rsil-identity` (453 + 5 passed) && `cargo test -p powrush-client --lib` (588 passed; named `q0_*` landing proofs)

Human / Sanctuary yard: PASS  
Ambrosian / Sanctuary well-from-above: PASS  
Cydruid / Heartwood: PASS  
Quellorian / Threshold: PASS  
Draek / Depths (teal way-home): PASS  
Garden / light path: PASS

## HUD dual channel (human tick)

Lived NEVC line after VALENCE-HUD-2 (#455). Two fields on the existing helper — not a second overlay. Agents do not invent minutes.

- [ ] class field reads **stewardship / harm gate**
- [ ] second field reads **stewardship quality** (display only)
- [ ] a blocked / grief-gated act does not print as wages or gold
- [ ] tap-E take / hold-E tend / R allocate unchanged
- [ ] OS / GPU / minutes stay **BLANK** for the agent

## Five-minute script

1. Window opens. One card. Can you tell you are in a climate without reading a doc?
2. WASD walk. Space jump. Shift sprint. Card should step to “Walk to a glow.”
3. Find a glow. **Tap E** (take). Did you get glow + camera punch + rumble on first take? Slab names Idle / Glowing / Tended / Resting / Stressed.
4. **Hold E** on the glow (tend). Did the node breathe / harmony return? Prompt still names tap vs hold.
5. Press I. Is the take in the satchel?
6. Press R, then 1 (flow) and 2 (reserve). Did flow change a tired well? Did reserve bank repair-rights without spending the field?
7. Press H. Can you still tend without the guidance card?
8. Quit. Rerun. Are satchel and allocation still there?
9. Teaching claim on Sanctuary: E twice on a glow — slab *extract left it tired*. Then R 1 — *flow restored the well*.

## Hour 2 — yard remembers (already shipped)

Do **not** rebuild Tab / Q / L / Bind / resume. Verify only. Full script + fail verbs: [`HOUR_TWO_PLAYTEST.md`](HOUR_TWO_PLAYTEST.md). Report minutes there stay blank too.

How to verify after Hour 1 allocate:

1. Card *Tab the ridge* → Tab. Peace visitor. E is *Not your charter*.
2. Card *Q plant a House stake* → Q. Spill slab (I2 / Offline extractor). Witness, not attack.
3. Card *L opens the Ledger* → L, then E Bind until Settled (not Ledger 3).
4. Slab *Hour two held*. Quit. Rerun.
5. Welcome slab: *Welcome back · Hour two held · the yard remembers*. Existing card border breathes once, then rests. Card skips WASD.
6. Yard still in `data/powrush_hour_two.json` (House + Settled).

Stop. Embassy / Crownstone / Hybrid / myth / fabricator are **not** this seat.

After a MendSpool is stocked, the fabricator slab names Digit1; an empty spool does not.

## Pass / fail

**Success (PLAYTEST-1):** a stranger can **take → tend → allocate** in **under 20 minutes** without a wiki.

Fail if E does nothing visible.
Fail if tap-E and hold-E feel like the same verb (take vs tend not readable).
Fail if I is empty after a successful take.
Fail if R does not change the field (flow) or the satchel rights (reserve).
Fail if the client requires a server process.
Fail if a second HUD appears before the first tend.
Fail if the card is a manifesto.
Fail if extract-only never tires a well, or flow never restores one.
Fail if Title shows a peer count / “players online.”
Fail if allocate invents currency, pockets, or a cash lock.
Fail if Embassy / Crownstone is required to finish Hour 1 or the Hour-2 verify.

## Steward note

PLAYTEST-1 locks the lived stranger hour in this file. Hour-two minutes / OS / GPU stay **blank** on this stamp. Do not fabricate them. Do not start Hour three systems from this seat.

## Report

OS / GPU:
Commit:
Minutes (take → tend → allocate):
Confusion point:

Time to first tend:
E tap take: yes/no
E hold tend: yes/no
Satchel correct: yes/no
Allocate visible: yes/no
Save/load: yes/no
Card hid with H: yes/no
Extract tired the well: yes/no
Flow restored the well: yes/no
Hour two welcome named the yard (if walked): yes/no
Suggested one-line fix:

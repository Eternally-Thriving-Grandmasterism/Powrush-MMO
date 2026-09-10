# AAA_FEEL_WORKPACK.md — Offline 1.0 feel on practical devices

**Contact:** info@Rathor.ai  
**Base law:** Steam Offline is product 1. Title Online stays grey. `POWRUSH_INGEST` off by default. Ra-Thor may read ticks; never drives keys.
**Tip family:** after P4 House-week Depths ink. Workspace `21.88.0` (design tick). Floor `2163551`. Tag `playable-preview` `11c577e`.

This pack is R&D law for agents. It is **not** Unreal, not a second HUD, not an Online SKU.

## What “AAA on practical devices” means here

AAA is **readable hands + honest juice + 60-ish feel on the machine you have**, not a trailer budget.

Practical devices (priority):

| Device | Bar |
|---|---|
| Windows / Linux laptop (Vulkan or lavapipe) | Title contrast, UI hits plates not world, first E readable |
| Steam Deck / gamepad | South = Use, Start = pause, sticks cull on plates |
| Phone | **not** a ship target this SKU; Cursor phone is for agents, not the binary |

Refuse as “AAA”: ray-traced Sanctuary, cinematic 2nd camera, XP bar, lobby chat, birds, default Grove scatter.

## Already shipped (do not rebuild)

B1 rumble + reduced_motion · B2 well sentences · B3 colorblind tokens · B4/B4.1 remaps + Settings rows · Wave C Wards notice · P2 well-then-Wards slab · P3 Depths slab line · P4 Depths in House week · opaque Title/pause · H hides guidance, well words stay.

Feel already in hands: first-take glow + camera punch + rumble (when rumble on).

**Feel-move (SC2 contract, named door):** FixedUpdate @ 60 Hz locomotion; accel + stop-on-release; one Use ≤120 ms enter-range buffer. Do not rebuild. A2–A5 still steward-named.

## AAA slice queue (one PR each)

| Door | Job | Files |
|---|---|---|
| **A1** | Audio one-shot on first successful E (soft, respects Mute) | client audio + settings mute only |
| **A2** | Frame/feel budget note + `reduced_motion` already skips punch; document Deck 40/60 | docs + existing flag, no new renderer |
| **A3** | Fat-tap Places on pause (Esc already opens Places) | pause / Places plate only |
| **A4** | Ingest overlay fields **opt-in** (`feel_ms`, `last_verb`, `input_kind`) on `LivedTickIngest` — **schema additive, default omit** | `shared/lived_tick_ingest.rs` + tests; flag still off |
| **A5** | Deck/gamepad Settings row copy: South/Use Start/Pause | Settings plate copy only |

Do **not** start A4 and A1 in the same PR. Steward names the next letter.

## Telemetry (Powrush → lattice)

Two different files can share a path name. Do not confuse them.

| Blob | When | Who reads |
|---|---|---|
| Session persist `powrush_lived_tick.json` | Mode B resume — always may exist | Client only |
| Ingest overlay same path | Only if `POWRUSH_INGEST=on\|1\|true` | Ra-Thor **read-only** |

Schema today (`powrush_lived_tick_v1`): house_id, house_name, climate, standing, week, hour_flags, optional hour.

A4 may add **optional** feel keys. Missing keys = old clients. Never require ingest on boot. Never put ingest chrome on the slab.

Lattice policy that comes back is **file side-channel later**, not WASD.

## Dual-repo seats

| Seat | Repo | May |
|---|---|---|
| Grok Bot 1 | Powrush-MMO | A1 or A3 after steward names it |
| BabyBot | Ra-Thor | Read tick schema; R6 or tick-read doc; never client keys |
| Cursor | one repo only | collide if two writers |

## HOLD

Title Online · `0.0.0.0` · Market · Hour-two minutes invented · playable-preview retag · race-at-Title · unparking server/ as default · Simulator JS as the game · Ra-Thor Cargo path dep.

## Related

`OFFLINE_SKU` · `GDD_ADAPTATION` · `PHYSICS_GRAPHICS_CANON` · `LAVAPIPE_CLICK_CLEAN` · `INPUT_CANON` · `PROTOCOL` · `PARKED_SURFACES` · `PLAYTEST_AUDIT_2026-09-09`.

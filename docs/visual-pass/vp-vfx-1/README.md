# VP-VFX-1 — capped Tend / Flow VFX

Visual feedback for the existing **Tend** (hold E at a glow) and **Flow** (allocate `1`)
answers, so they read as part of the painted world. `client/src/tend_flow_vfx.rs` only
*reads* `WorldAnswer`; Tend / Flow gameplay, timing, cooldowns and networking are
unchanged. Small unlit emissive motes (gold for Tend, pale water for Flow) spawn in the
lower third of the view, to either side of centre, rise, shrink and despawn. No particle
crate, no lighting / bloom / grade change, no rig change.

Captures: real release binaries (before = `bc9bf930`, after = this branch) under Xvfb at
1280×800 on lavapipe software rendering, Online off, fresh settings dir per preset. The
effect is triggered with real key presses on the existing bindings (E tap → Take, E hold
→ Tend, `1` → Flow on the allocate panel); one frame grabbed mid-Tend and just after Flow.

## Caps (live motes)

| Preset | Cap | Notes |
|---|---|---|
| Mobile | 0 (`VFX_CAP_MOBILE`) | off |
| Low | 6 (`VFX_CAP_LOW`) | minimal |
| Medium | 14 (`VFX_CAP_MEDIUM`) | |
| High | 24 (`VFX_CAP_HIGH`) | |
| Ultra | 24 (`VFX_CAP_ULTRA` = High) | no Ultra-only stack |

Per fire: Tend ≤ 6 motes / 1.6 s, Flow ≤ 10 motes / 2.2 s, always clamped to the cap.

## Glow clearance

Motes never sit over a Use glow. Every `MercyHarvestNode` is a glow source with a clearance
sphere of `GLOW_BASE_RADIUS` (0.48) × its drawn scale × `GLOW_CLEARANCE_FACTOR` (1.6).
`plan_mote_starts` only accepts a slot whose **whole rising path** is clear of every
glow in screen space (`mote_clear_of`: the angular gap to the glow centre must exceed the
padded glow's angular radius plus the mote's). Blocked slots shift to the next candidate
(wider, lower or nearer in the band) and are skipped if none is clear. The cap is unchanged.

## Before / after — Low and Medium

| Preset · action | Before | After |
|---|---|---|
| Low · Tend | ![](before-low-tend.png) | ![](after-low-tend.png) |
| Low · Flow | ![](before-low-flow.png) | ![](after-low-flow.png) |
| Medium · Tend | ![](before-medium-tend.png) | ![](after-medium-tend.png) |
| Medium · Flow | ![](before-medium-flow.png) | ![](after-medium-flow.png) |

## High / Ultra (cap raised, still capped)

| Preset · action | Before | After |
|---|---|---|
| High · Tend | ![](before-high-tend.png) | ![](after-high-tend.png) |
| High · Flow | ![](before-high-flow.png) | ![](after-high-flow.png) |
| Ultra · Tend | ![](before-ultra-tend.png) | ![](after-ultra-tend.png) |
| Ultra · Flow | ![](before-ultra-flow.png) | ![](after-ultra-flow.png) |

## Side-by-side vs the Quellorian waterfall plate

Plate referenced by URL only (not committed).

| Quellorian waterfall plate (target, by URL) | In-engine after (Medium · Flow) |
|---|---|
| ![plate](https://powrush.com/images/art/home-quellorian-waterfall-city-1500.webp) | ![after medium flow](after-medium-flow.png) |

## Frame time (software — lavapipe under Xvfb, not GPU)

Mean of the last 10 Bevy `frame_time` lines per run (window includes the Tend + Flow fires).

| Preset | Before (median) | After (median) | Budget |
|---|---|---|---|
| Low | 42.8 ms | 46.2 ms | ≤ 50 ms ✅ |
| Medium | 99.1 ms | 89.6 ms | ≤ 100 ms ✅ |
| High | 218.8 ms | 223.9 ms | none (beauty tier, capped; single run) |
| Ultra | 357.8 ms | 446.7 ms | none (beauty tier, capped; single run) |

Low / Medium: 3 + 3 runs, interleaved before, after, before, after, before, after. Each run used a fresh settings dir with Tend + Flow firing, and each started only when the 1-min load average was < 2.5 (8 cores). Before = `bc9bf930`, after = `a4c6333f`.

| Run | Medium before | Medium after | Low before | Low after |
|---|---|---|---|---|
| 1 | 108.0 ms (load 2.22) | 94.2 ms (load 2.39) | 40.8 ms (load 2.20) | 46.2 ms (load 2.48) |
| 2 | 99.1 ms (load 2.32) | 86.1 ms (load 2.21) | 42.9 ms (load 2.29) | 44.6 ms (load 2.42) |
| 3 | 96.6 ms (load 2.35) | 89.6 ms (load 2.23) | 42.8 ms (load 2.14) | 46.8 ms (load 2.30) |


High / Ultra runs used longer settle waits (slower software frames), so their before/after
deltas include run-to-run noise.

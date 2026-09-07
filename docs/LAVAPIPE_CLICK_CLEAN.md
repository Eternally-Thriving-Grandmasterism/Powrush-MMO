# LAVAPIPE_CLICK_CLEAN.md — soft GPU first-class walk proof (docs stamp)

**Contact:** info@Rathor.ai  
Workspace `21.88.0`. Design tick, not a Cargo bump.  
**Floor stays:** `2163551`. Tip reference `b215275`.  
**Law:** Soft GPU (lavapipe) is a **first-class walk proof** — not a unit-test name, not screenshot-only. Docs now; no client / shared / Cargo in this stamp. No Wave P. No `playable-preview` from this doc alone.

**One-liner:** Soft GPU first-class — if Title / pause / Ledger / seals do not **read and click** on lavapipe, the door fails even when a discrete GPU looks fine.

## What “click-clean” means

A human (or steward) boots `cargo run -p powrush-client` on lavapipe and **walks** the lived door: plates readable, pointer hits UI not world, labeled actions fire, overlay cull holds, no second Camera3d. Screenshots may illustrate; they do **not** replace the walk.

Past beats that taught this: UI-above-world soft GPU (`8492a318` / #273), F3 Settled re-bury (`a310cf23` / #275), stranger floors through `932973ef` → `8d91e95` → `2163551`. Reference fix shape: **Camera2d** (order 10 + `IsDefaultUiCamera`) + **TargetCamera**-bound `LivedUiPlate` + **MSAA off**.

## Five checks (all required)

| # | Check | Pass looks like |
|---|---|---|
| 1 | **Readable** | Title / pause / Settings / Ledger / Q / I opaque plates contrast; labels not buried under world fog |
| 2 | **Hits UI not world** | LMB / pointer selects plate buttons; world mesh does not steal the click |
| 3 | **Labeled action fires** | Play / Resume / Settings / Continue / Quit / sheet opens do what the label says |
| 4 | **Overlay culled** | On-screen sticks / scatter / fog-behind-UI stay culled while plates are open |
| 5 | **No 2nd Camera3d** | Exactly one world Camera3d; UI is Camera2d — no order-ambiguity bury |

## Failure modes

| Mode | Symptom | Likely cause |
|---|---|---|
| Camera3d order | World draws over UI; plates look “under” terrain | World / UI camera order wrong; missing per-frame re-stamp |
| TargetCamera | Clicks miss plates or hit wrong layer | `LivedUiPlate` not bound to UI cam via `TargetCamera` |
| MSAA / fog / Z | Soft GPU bury / writeback; fog eats contrast | MSAA on; fog on UI cam; Z-plane fight |
| Extra 3D cam | Intermittent bury after Settled / climate / gen | Second Camera3d spawned (climate / gen path) |
| Uncull sticks | Sticks / scatter steal hits on Title/pause | Overlay not culled on plates |

## Pass / fail / revert law

- **PASS** — all five checks on lavapipe with keyboard (and again with Grove / `POWRUSH_GEN=light` when claiming gen-on click-clean).
- **FAIL** — any check fails on soft GPU even if a discrete GPU passes.
- **REVERT** — if a change breaks lavapipe click-clean, revert or fix before claiming stranger-door / Settled Esc/Q/L / G0 gen-on honesty. Do not paper over with screenshots or “works on my NVIDIA.”

Floor honesty stays `2163551` (G0 #279; Settled Esc/Q/L click-clean on lavapipe with gen on). Online grey. Not a launch candidate.

## Reference fix shape (already beaten in tree)

- UI **Camera2d** order **10** + `IsDefaultUiCamera` above world Camera3d order **0**.
- **TargetCamera**-bind `LivedUiPlate` (Title / pause / Ledger / I / dress).
- **`Msaa::Off`** (writeback bury on soft GPU).
- Re-stamp UI/world orders every frame; strip world `IsDefaultUiCamera`; respawn UI cam if missing.
- Opaque high-contrast plates. Climate fog on **world** Camera3d only.
- Never spawn a second Camera3d for climate / gen / birds.

## Refuse

- Treating lavapipe as optional / CI-only / screenshot-only.
- Unit-test name cosplay without the five-check walk.
- Second Camera3d, birds, Avian, Wave P, lighting Online, GenShare sockets from this proof.
- Cutting `playable-preview` from lavapipe PASS alone.

## Relates

`PREVIEW_CHECKLIST` (tick pointing here) · `PHYSICS_GRAPHICS_CANON` · `INPUT_CANON` · `STRANGER_LOOP` · `GENSHARE` · `SLICE_LOG` (F3 / soft-GPU beats).

**Thunder locked in.** Yoi ⚡

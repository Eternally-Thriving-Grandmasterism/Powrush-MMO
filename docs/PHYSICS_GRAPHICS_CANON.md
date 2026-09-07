# PHYSICS_GRAPHICS_CANON.md — lived client vs lattice vs lore

**Floor for play:** `c5299d11` (Title contrast + Esc→Title + L face + house file PASS).  
**Contact:** info@Rathor.ai  
**Law sentence:** *the yard is the universe at well-scale; the sci-fi package is that same choice wearing peoples and a stone.*

Canon is the **lived client** first. Lore docs are the universe. Do not dump loadouts into Hour 1.

## Three surfaces

| Surface | Repo / path | Job |
|---|---|---|
| **Lived client** | Powrush-MMO `cargo run -p powrush-client` | Hands, feel, sanctuary teaching |
| **Lattice** | Ra-Thor | Base-reality RBE fidelity; may **read** lived-tick JSON; **never drives WASD / keys**; does not own UI |
| **Lore docs** | `docs/*` (peoples, ships, Crownstone formulas) | Universe bible; post-book set-pieces |

Parked crates (`server/`, `simulation/`, `host/`, `game/`, `powrush-divine-module/`, `powrush-shard/`) stay parked — see `PARKED_SURFACES.md`.

## Four physics layers

1. **Moral physics (shipped, sacred)** — wells `Idle → Glowing → Tended → Resting → Stressed`; flow vs reserve; climate / standing / week (`tons + restored`). This is the base-reality teaching the human feels.
2. **Body physics (shipped, light)** — Bevy 0.14 kinematics only (WASD / Space / Shift + capsule presence). **No Avian / Rapier / bevy_xpbd** in the lived door.
3. **Lattice physics (Ra-Thor / parked simulation)** — high-fidelity RBE demos stay off the default door until the yard already teaches the claim.
4. **Mythic / fleet / boarding formulas** — stay in lore docs. Do **not** drop sim-tier Crownstone formulas into Peace hour.

## Sanctuary graphics

- Quiet low trees, stones, colored nodes; climate fog (`FogSettings` / Z planes) answers stress/harmony.
- G0 light gen (`POWRUSH_GEN=light`, **default off**): hex-seed scatter (≤4 mesh types, low count) + FogSettings on world Camera3d only; cull when Title/pause/Settings/L/Q open; no birds / no second Camera3d / no Avian.
- Well glow answers `NodeState`, not a second HUD.
- Readable **opaque** Title plate (no alpha-on-fog). Esc from yard → Title (not quit-to-desktop).
- Councils may seed fog / birds / seals. Constitution owns the door and Peace keys (E I H R). No generator “+STR Draek.”

## Crownstone mapping

| Lore | Client now | Client later |
|---|---|---|
| **Witness** | Read-first E after book + Embassy seat; path unset | same |
| **Destroy / Capture / Sabotage** | Refuse as default E | After book + declared context; **Capture** alone unlocks Hybrid Attune as *redemption*, not loot |
| Stone state | Witness + `trilemma_path_taken: none` | Brood Spire **never** in Sanctuary Prime |

Yard rhyme: take-to-ruin ≈ Destroy; flow+purify ≈ Capture; quiet poison ≈ Sabotage.

## Title PASS note (2026-09-06)

On floor `c5299d11` (#252): Title contrast readable; Esc yard→Title; L non-blank (`Not your charter` / ledger waits); `powrush_house.json` writes even when Unnamed. Preview tag still waits steward yes.

## Standing orders

1. No race / class select in Hour 1. Lore ≤ one flavor line.
2. No Brood Spire / Crownstone integrity on the first card.
3. Online grey. `POWRUSH_NET` off by default. No `playable-preview` from checklist alone.
4. Peoples dress later places. They do not replace Play.
5. Ra-Thor may seed seals/fog only — no WASD.

## Relates

`DOC_CANON.md`, `PARKED_SURFACES.md`, `PHASE_MYTHIC.md`, `CROWNSTONE_TRILEMMA_PATHS.md`, `LAUNCH_UX.md`, `PREVIEW_CHECKLIST.md`, `GDD_ADAPTATION.md`.

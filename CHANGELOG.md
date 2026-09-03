# CHANGELOG.md — Powrush-MMO

## [23.2.4] — 2026-09-02 — Hour sacred (Slice 0)

Peace hour stays one human / one machine:
- `shared::space_law` — HexFlag, WarrantWeight (live 0 in Peace), CharterKind, skin gate
- Tab / G / L / Q no-op without `charter_id` + Frontier. WASD / E / I / H / R untouched
- No hunt UI, no Currency3, no factory

## [23.2.3] — 2026-09-02 — Allocate by hand

R is earn-by-sharing, not a mouse-only panel:
- **1** flow outward · **2** steward reserve while the panel is open (satchel 1/2 yield)
- First share is its own thriving moment (does not steal harvest/inventory firsts)
- Guidance teaches R after the satchel

## [23.2.2] — 2026-09-02 — Harvest juice

First E at a glowing node is a reward, not a counter tick:
- Node emissive + child point light bloom with `pulse`
- Camera punch from `HarvestJuice` (first take stronger; never steals look)
- First-take rumble is longer; epiphany credit only on the first harvest

## [23.2.1] — 2026-09-02 — Lived-hour client bin

- Default `powrush-client` bin is `PowrushClientBundle` (walk, harvest, satchel, guidance), not GPU test spheres.
- Bundle no longer pulls networking, `simulation`, ambisonics, or egui GPU materials.
- Harvest reach uses `SoftPresence` (the human body). `RbeGlobalState` / `RbeUiSync` are local first-hour resources.

## [23.2.0] — 2026-09-02 — Standalone cargo (human game)

Lived first hour is the product. This slice unblocks building *this* repo alone:

- `shared` no longer path-deps sibling `../../Ra-Thor/crates/mercy_tolc_operator_algebra`. NEVC **Mode B** (`nevc_adapter`) is the default. Mode A is a local overlay when the lattice is co-located — not a CI feature.
- Workspace Core members: `shared`, `crates/rsil-identity`. Parked `client`, `server`, `simulation` (unpublished `shamirs-secret-sharing`), `host`, `game`, `powrush-divine-module`. Lived hour stays `client/src`.
- CI Core: `cargo test -p shared -p rsil-identity`. Parked workspace fmt/clippy/`--workspace` test, WASM/Trunk, Docker, and cargo-deny until those surfaces compile.
- README no longer claims 100% launch-worthy.

Next: default `powrush-client` bin is `PowrushClientBundle`, not GPU test spheres.

## [23.1.0] — 2026-08-26 — Lived tick

Launch-worthy was the *sim*. This is the seam into the hour:
- `data/powrush_lived_tick.json` ~1 Hz (lineage, pool, climate, flow, pocket)
- Classic ↔ sim race map documented
- Ra-Thor may read; it may not drive the keys

## [23.0.0] — Lineage (C)
## [22.15.0] — Practice travelers

Contact: **info@Rathor.ai**. Thunder locked in. Yoi ⚡

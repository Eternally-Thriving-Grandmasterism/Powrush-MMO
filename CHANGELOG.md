# CHANGELOG.md — Powrush-MMO

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

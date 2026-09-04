# CHANGELOG.md — Powrush-MMO

## [23.2.21] — 2026-09-03 — Hands hook LivedHourBind

harvest_feel E writes a climate take.
rbe_allocate_choice R 1/2 writes flow/reserve.
Tick file stays data/powrush_lived_tick.json.
Does not replace existing feel or allocate panel.
Dies in Peace. Workspace stays 21.88.0

## [23.2.20] — 2026-09-03 — LivedHour client bind (Slice 0c)

Hands talk to `shared::climate_node` without a new key.

- `client/src/lived_hour_bind.rs`: E tend, I satchel count, R 1/2 allocate
- Writes `data/powrush_lived_tick.json` (Ra-Thor may read; it does not drive keys)
- `LivedHourEconomyPlugin` now also starts `LivedHourBindPlugin`
- Does not replace `harvest_feel` or `rbe_allocate_choice`
- Dies in Peace. Workspace stays `21.88.0`

## [23.2.19] — 2026-09-03 — Climate node first-hour law (Slice 0b)

Hands law for the Peace hour, without a new key.

- `shared/climate_node.rs`: Idle / Glowing / Tended / Resting / Stressed
- **E** tend: first glow takes; second take on a tired node is no-take
- **I** satchel is the take list
- **R** **1** flow restores a Resting/Stressed node; **R** **2** reserve is repair-rights only
- Tick JSON is `LivedHour` — satchel + nodes + allocation. No DID tradecraft
- Dies in Peace. Charter / Frontier / Ledger stay later slices
- Workspace Cargo remains `21.88.0`. This is a lived-hour design tick, not a workspace bump

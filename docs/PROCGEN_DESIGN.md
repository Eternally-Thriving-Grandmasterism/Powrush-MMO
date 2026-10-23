# PROCGEN_DESIGN.md — PGEN-0 procedural level generation (docs only)

Status: **design doc, no code.** Card PGEN-0. Follow-up cards PGEN-1..3 below.
Canon cited: `docs/PLACES_BIBLE.md`, `docs/PLACE_DRESS_SPEC.md`, `docs/ART_BIBLE.md`,
`docs/visual-pass/TARGET_BOARD.md`, `docs/PLAYABLE_RACES.md`. No new lore is
invented here; anything not settled in canon is marked **OPEN** or **UNKNOWN**.

## 1. Sherif's ruling (verbatim)

2026-10-23 3:15 AM ET: *"Both: peaceful regions plus combat zones"*.

2026-10-23 3:15 AM ET: *"Some example games include Conquer Online the Chinese MMORPG and Diablo 2."*

Consequence:
- Every generated zone carries a tag: **Peace** or **Combat**.
- **Peace** zones keep harvest / purify density (existing node model, e.g.
  `client/src/mercy_harvest_nodes.rs`, `client/src/depths_landing.rs`).
- **Combat** zones get hostile spawns. Enemy design is **deferred to later
  cards**. Lore AI docs are sources only, not specs:
  `docs/DRAEK_FLEET_AI_SYSTEMS.md`, `docs/DRAEK_FLEET_AI_BEHAVIOR_PATTERNS.md`,
  `docs/QUELLORIAN_RESONANCE_AI_SYSTEMS.md`,
  `docs/QUELLORIAN_RESONANCE_AI_BEHAVIOR_PATTERNS.md`.
- Hub towns are always Peace.

## 2. Reference games (inspiration only)

No names, assets or mechanics are copied. Only widely known public facts:
- **Diablo 2** — acts each with a distinct biome; outdoor areas and dungeons
  assembled from tile sets per playthrough; fixed hand-authored set-pieces
  and towns; waypoints for fast travel between visited areas.
- **Conquer Online** — MMO hub cities with peaceful town areas and open
  hunting / PK maps around them; monster density and level by map band.
  Exact current rules (e.g. PK specifics) are **UNKNOWN** here.

## 3. Shape of the world

- **Biome per act.** Each act has one biome with its own tile kit and reuses
  the existing Place mood from `client/src/climate_plane.rs` (fog / tint /
  weather beds) per PLACE_DRESS_SPEC. Act list and order: **OPEN**.
- **Hub towns = existing hand-made Places** (Sanctuary, Heartwood, Threshold, Depths; PLACES_BIBLE Heartwood rings L28 for Threshold;
  `shared/hex_travel.rs` `PlaceId` has only Sanctuary / Heartwood / Depths — Threshold has no PlaceId yet and shares Heartwood's save). Hubs are never generated.
  Market is not a Place (PLACES_BIBLE).
- **Generated zones** hang off a hub. Each zone = seeded room graph + tiles.

## 4. Generation

1. **Room graph.** From the zone seed, build a branching graph from the
   entry (hub edge) to the exit; dead-end branches allowed.
2. **Fixed anchors.** Place hand-made set-pieces on graph nodes: **hub-edge
   entry, waypoint, set-piece, exit**. Anchors are authored, never generated.
3. **Filler tiles.** Fill remaining cells with biome tiles whose edge sockets
   match neighbours (open / wall / transition). No unmatched edges.
4. **Density.** Peace: harvest/purify node count. Combat: spawn budget.
5. **Difficulty** = f(act index, graph distance from hub). Formula: **OPEN**
   (tuned on PGEN-2).

Look stays within ART_BIBLE and the visual-pass order on TARGET_BOARD
(AIR-1 > LIGHT-1 > SKY-2 …; TERRAIN-1 pairs with tile kits).

## 5. Determinism and saves

- Generation is **pure** code in `shared/` (no Bevy, no clock, no global RNG):
  `zone_seed = hash(world_seed, act, zone_index, gen_version)`.
- Save = **seed + edits + version** (edits = harvested nodes, opened doors,
  waypoint flags). Never the whole map.
- **`layout_hash` CI test**: fixed seeds must produce fixed hashes on every
  platform; a hash change requires bumping `gen_version`.
- Offline-by-default holds; no server needed to generate.

## 6. OPEN — pending Sherif

- Acts and biome order.
- New Places, e.g. the Cydruid stalactite city (PLAYABLE_RACES; no PlaceId yet).
- Shared world seed vs per-character seed; whether zones re-roll.
- Loot rarity vs crafting-first economy.
- Online scope for generated zones (online stays grey today).
- PK / player-vs-player in Combat zones: **UNKNOWN**, not ruled.

## 7. Follow-up cards

| Card | Scope | Core vote |
|---|---|---|
| PGEN-1 | Pure seeded room-graph + edge-matching tile stitch in `shared/`, `layout_hash` tests | **yes** |
| PGEN-2 | One Depths-biome tile kit, one Peace zone below Depths behind a flag, wired to `climate_plane` | **yes** (runtime) |
| PGEN-3 | Waypoint save flag + Places-plate entry; seed+edits+version save | **yes** (persist schema) |

Combat spawns follow after an enemy-design card (not numbered here).

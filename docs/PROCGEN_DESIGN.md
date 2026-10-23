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

## 2. Reference games (inspiration only)

Only common genre patterns (acts, waypoints, tile-stitched areas) are borrowed; no names or assets are copied. Only widely known public facts:
- **Diablo 2** — acts each with a distinct biome; outdoor areas and dungeons
  assembled from tile sets per playthrough; fixed hand-authored set-pieces
  and towns; waypoints for fast travel between visited areas.
- **Conquer Online** — MMO hub cities with peaceful town areas and open
  hunting / PK maps around them; monster density and level by map band.
  Exact current rules (e.g. PK specifics) are **UNKNOWN** here.

## 3. Shape of the world

> **Status of §3–§5:** council proposals pending council / Core review (one biome per act, waypoints, room-graph / tile generation, hubs always Peace / never generated, seed + edits + version save). Only Peace + Combat zones (§1) and the two reference games (§2) are Sherif's rulings.

- **Biome per act.** Each act has one biome with its own tile kit and reuses
  the existing Place mood from `client/src/climate_plane.rs` (fog / tint /
  weather beds) per PLACE_DRESS_SPEC. Act list and order: **OPEN**.
- **Hub towns = existing hand-made Places** (Sanctuary, Heartwood, Depths). Threshold is not a hub town: in code it is a Heartwood roof shelf, not a PlaceId (`client/src/hex_travel.rs` L115; `shared/threshold_shelf.rs`; PLACES_BIBLE Heartwood rings L28);
  `shared/hex_travel.rs` `PlaceId` has only Sanctuary / Heartwood / Depths. Hubs are never generated.
- Hub towns are always Peace.
  Market is not a Place (PLACES_BIBLE).
- **Generated zones** hang off a hub. Each zone = seeded room graph + tiles.

## 3a. Orbital hub towns (PGEN-0.1)

Sherif's ruling, 2026-10-23 3:44 AM ET (verbatim): *"2 of the hub towns should include the 2 earth orbiting motherships of the Draeks and Quellorians both for deployments and functioning as towns in orbits respectively and appropriately, that's why the acronym is TAUN for the Quellorians side, Lmao, Mate!"*

- **Quellorian hub: The Auroral Unification Nexus (TAUN)**, "Primary capital
  megastructure and mobile command center" and "Massive fleet carrier"
  (`docs/FACTIONS_OVERVIEW.md` §2; `docs/AURORAL_NEXUS.md`). Look follows
  `docs/CODEX_ART_PROMPTS.md` §2 ("a compact pale-gold disc, violet light only
  in the seams, no points, no arms") under the VP-TARGET-0 override.
- **Draek hub: The Brood Spire (TBS)**, "the central megastructure and mobile
  throne of the Draek Dominion" (`docs/BROOD_SPIRE.md` §1;
  `docs/design/LEGACY_GDD_RECONCILIATION.md`).
- Both are **hand-made hubs, never generated**, tagged **Peace** inside the
  hub, with a **deployment launch** to Combat zones (TAUN deploys fleets per
  FACTIONS_OVERVIEW; TBS spawn/production per BROOD_SPIRE).
- Neither has a `PlaceId` yet.
- **Limits lifted (owner ruling).** Sherif, 2026-10-23 3:46 AM ET, verbatim:
  *"Lift!"* — in reply to whether making the motherships walkable lifts the
  "mothership-over-Earth PRESENTATION; no hull mesh" limit
  (`client/src/climate_plane.rs` L448) and "Do not spawn them in the yard"
  (`docs/AGENT_COMPLETION_PACK_v23.2.md` L186). Both limits are lifted for
  **walkable orbital hub interiors (TAUN, TBS)**. Engine / runtime cards that
  build them still need a **Core vote**.
- **Stale docs:** the `climate_plane.rs` comment and AGENT_COMPLETION_PACK L186
  still state the old limits; a later card updates them.
- **Council recommendation (not Sherif's ruling):**
  - TBS interior is **Peace** but keeps its oppressive look (BROOD_SPIRE §1).
  - Faction players home to their own side's ship (TAUN or TBS).
  - The three Earth hub towns (Sanctuary, Heartwood, Depths) stay
    **neutral** hubs.
- **OPEN / UNKNOWN:** exact orbit (canon only says "low orbit" in
  CODEX_ART_PROMPTS §2); hub interior layout; which Combat zones each hub
  launches to; online scope.

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
- Act structure (how many acts, what an act is).
- New Places, e.g. the Cydruid stalactite city (PLAYABLE_RACES; no PlaceId yet).
- Shared world seed vs per-character seed; whether zones re-roll.
- Loot rarity vs crafting-first economy.
- Online scope for generated zones (online stays grey today).
- PK / player-vs-player in Combat zones: **UNKNOWN**, not ruled.

## 7. Follow-up cards

| Card | Scope | Core vote |
|---|---|---|
| PGEN-1 | Pure seeded room-graph + edge-matching tile stitch in `shared/`, `layout_hash` tests | **yes** |
| PGEN-2 | One Depths-biome tile kit, one Peace zone below Depths behind a feature flag, wired to `climate_plane` | **yes** (runtime) |
| PGEN-3 | Waypoint save flag + Places-plate entry; seed+edits+version save | **yes** (persist schema) |

Combat spawns follow after an enemy-design card (not numbered here).

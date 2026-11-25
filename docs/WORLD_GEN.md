# World generation method

WORLD-GEN-1. Docs only. This file is new. Older files stay as they are; this page points at them.

The PATSAGi Councils adopted this page under Sherif's typed delegation t1767u, t1948u and t2247u. Sherif may veto or amend. The seed recipe in §3 is a PROPOSAL.

This page sets the method every WORLD-TRAVEL-1 zone build uses (ruling 7117162436, amended by 7117183388). It owns the method only: the layers, the seed recipe, determinism, fixtures and the runtime-cost rule. It names no zone, place, realm or biome, and coins no name.

The build is offline. Title Online stays grey ([shared/net_mode.rs:5](../shared/net_mode.rs#L5)). Online, server and account work stay planned and are Sherif's.

What this page links instead of repeating:

- Zones, places, realm names and bands: [WORLD_BIBLE.md §6 Zone map](WORLD_BIBLE.md#6-zone-map-proposal) (PROPOSAL).
- Code owners: [SYSTEMS_SPINE.md §6 World](SYSTEMS_SPINE.md#L62) (gen-seed owners at [:64](SYSTEMS_SPINE.md#L64)), and [§11.6](SYSTEMS_SPINE.md#L263), which lists WORLD-GEN-1 as planned at [:269](SYSTEMS_SPINE.md#L269).
- The existing generation sketch: [PROCGEN_DESIGN.md:81-91](PROCGEN_DESIGN.md#L81) (room graph, fixed anchors, filler tiles, density) and [:96-104](PROCGEN_DESIGN.md#L96) (determinism and saves).
- The player-built transport network: [DESIGN_PILLARS.md COMPANY-1](DESIGN_PILLARS.md#L44).

Inspiration only. The ruling names well-known open-world and MMO maps as inspiration for the method. No maps, layouts, names or assets are copied, and nothing here claims to match or outdo any other game ([DESIGN_PILLARS.md:27](DESIGN_PILLARS.md#L27)).

## 1. The six layers

The layers run in this order. Each is seeded and deterministic. Each is marked built, partial or planned at this tip.

| # | Layer | Status | Today |
|---|---|---|---|
| 1 | Macro map | planned | Input is the bible's zone map PROPOSAL ([WORLD_BIBLE.md §6](WORLD_BIBLE.md#6-zone-map-proposal)). No code. |
| 2 | Terrain and biomes | partial | One stepped shelf model ([terrain.rs:120](../client/src/terrain.rs#L120)). Biome is realm dress only ([climate_plane.rs:393](../client/src/climate_plane.rs#L393)). |
| 3 | Sparse starter network | planned | Haul on menu travel only ([shared/hex_travel.rs:568](../shared/hex_travel.rs#L568)). |
| 4 | Settlements and hubs | hand-made only | Three hand-made Places ([shared/hex_travel.rs:52](../shared/hex_travel.rs#L52), [:61](../shared/hex_travel.rs#L61)). |
| 5 | Points of interest, quest hubs, creature spawns by level band | planned | Spawns are fixed constants ([combat_core.rs:56](../client/src/combat_core.rs#L56), [:58](../client/src/combat_core.rs#L58)). Bands are placeholders ([LEVELS_CANON.md:18](design/LEVELS_CANON.md#L18), [:75](design/LEVELS_CANON.md#L75)). |
| 6 | Prop scatter | built, Sanctuary only | §4 below. |

### Layer 1: macro map

The input is [WORLD_BIBLE.md §6](WORLD_BIBLE.md#6-zone-map-proposal). This page copies no zone name, band value or realm table from it. Any new name goes through the bible, the councils and Sherif.

### Layer 2: terrain and biomes

Today the ground is stepped shelves with no seed: `terrace_shelf` ([terrain.rs:106](../client/src/terrain.rs#L106)), `terrace_height` ([:120](../client/src/terrain.rs#L120), documented deterministic at [:118](../client/src/terrain.rs#L118)), `shelf_offset` ([:127](../client/src/terrain.rs#L127)), `terrace_mesh` ([:133](../client/src/terrain.rs#L133)) and `rim_wobble` ([:186](../client/src/terrain.rs#L186)), pinned by the test at [:263](../client/src/terrain.rs#L263). A zone build adds a seeded height and biome pass on top of this; that is planned.

### Layer 3: sparse starter network

Per ruling 7117183388, the generator makes only a sparse starter network: footpaths and a few NPC-run trade roads.

Players and their guild companies build everything else: rail, sea lanes, mounted-courier lines, depots, stations and portals. That is COMPANY-1 ([DESIGN_PILLARS.md:44](DESIGN_PILLARS.md#L44)). The generator never lays them.

None of this implies online, server or account work. That stays planned and Sherif's.

### Layer 4: settlements and hubs

Settlements sit along the starter network. Today the only settlements are the hand-made Places. Hand-placed set pieces always sit on top of the generated base; anchors are authored, never generated ([PROCGEN_DESIGN.md:85-86](PROCGEN_DESIGN.md#L85)).

### Layer 5: points of interest, quest hubs and spawns

Placed by level band from the zone seed. Today every spawn is a fixed constant. Band values are placeholders until LEVELS-LADDER-MAP-1 ([LEVELS_CANON.md:18](design/LEVELS_CANON.md#L18)).

### Layer 6: prop scatter

Built today, for Sanctuary only (§4).

## 2. Readable and playable

- Landmarks guide the player.
- Paths have sightlines.
- There are no empty filler fields.
- Hand-placed set pieces sit on top of the generated base.
- How a generated zone looks is Sherif's call. The first zone build needs a side-by-side screenshot and his typed look OK. LOOK-TARGET-1 owns the look target.

## 3. Seed recipe (PROPOSAL)

> **PROPOSAL for the councils; Sherif may veto.** It reuses the existing line at [PROCGEN_DESIGN.md:98-99](PROCGEN_DESIGN.md#L98).

- `zone_seed = hash(world_seed, act, zone_index, gen_version)`.
- Save is seed plus edits plus version, never the mesh ([PROCGEN_DESIGN.md:100-101](PROCGEN_DESIGN.md#L100)). The schema is SAVE-SCHEMA-1's ([SYSTEMS_SPINE.md §14.1](SYSTEMS_SPINE.md#L397)).

**Legacy seeds, kept as they are.** Each current source has its own constant. Moving them onto one recipe is a later code card.

| Source | Seed | Cite |
|---|---|---|
| Vegetation field | `VEG_SEED` | [vegetation.rs:55](../client/src/vegetation.rs#L55) |
| Backdrop conifers | `CONIFER_SEED` | [sky_backdrop.rs:61](../client/src/sky_backdrop.rs#L61) |
| Backdrop ridges | fixed literal | [sky_backdrop.rs:456](../client/src/sky_backdrop.rs#L456) |
| Light-gen grove (opt-in) | house, hex and climate band | [shared/powrush_gen.rs:95](../shared/powrush_gen.rs#L95), [:107](../shared/powrush_gen.rs#L107) |

## 4. What is generated today

| Item | Cite | Seed |
|---|---|---|
| Vegetation scatter | `vegetation_plants` ([vegetation.rs:368](../client/src/vegetation.rs#L368), "Same preset, same plants" at [:367](../client/src/vegetation.rs#L367)), `scatter` ([:484](../client/src/vegetation.rs#L484)), `ring_point` ([:538](../client/src/vegetation.rs#L538)), local `Rng` ([:611](../client/src/vegetation.rs#L611)) | `VEG_SEED` |
| Backdrop | `ring_peaks` ([sky_backdrop.rs:325](../client/src/sky_backdrop.rs#L325)), `conifer_spots` ([:350](../client/src/sky_backdrop.rs#L350)), `faceted_mountain` ([:398](../client/src/sky_backdrop.rs#L398)), `ridge_meshes` ([:455](../client/src/sky_backdrop.rs#L455)), `silhouette_band_mesh` ([:220](../client/src/sky_backdrop.rs#L220)) | `CONIFER_SEED`, ridge literal |
| Light-gen grove (opt-in, off by default) | `scatter_from_seed` ([shared/powrush_gen.rs:146](../shared/powrush_gen.rs#L146)); client `LightGenPlugin` ([light_gen.rs:112](../client/src/light_gen.rs#L112)); seed kept on disk ([light_gen.rs:356-377](../client/src/light_gen.rs#L356)) | grove triple |

**Light gen is opt-in and off by default.** `POWRUSH_GEN` defaults to off ([shared/powrush_gen.rs:3](../shared/powrush_gen.rs#L3), [:48](../shared/powrush_gen.rs#L48)), pinned by tests at [shared/powrush_gen.rs:230](../shared/powrush_gen.rs#L230) and [light_gen.rs:660](../client/src/light_gen.rs#L660). Opt-in light gen turns on only when the Settings grove is light or `POWRUSH_GEN=light` is set ([light_gen.rs:76](../client/src/light_gen.rs#L76)).

**Sanctuary only.** Scatter, backdrop and water show in realm 0 only ([vegetation.rs:250-252](../client/src/vegetation.rs#L250), [sky_backdrop.rs:168-171](../client/src/sky_backdrop.rs#L168), [water.rs:180-182](../client/src/water.rs#L180)).

**Authored, not generated.** Solids and gameplay objects are a fixed table ([PLACES_BIBLE.md:69](PLACES_BIBLE.md#L69)): `FIELD_TREES` ([vegetation.rs:41](../client/src/vegetation.rs#L41)), the yard tree, rock and crystal spots ([living_ecology.rs:212](../client/src/living_ecology.rs#L212), [:226](../client/src/living_ecology.rs#L226), [:230](../client/src/living_ecology.rs#L230)), the ward posts ([shared/heartwood_wards.rs:20](../shared/heartwood_wards.rs#L20)) and the fight homes ([combat_core.rs:56](../client/src/combat_core.rs#L56), [:58](../client/src/combat_core.rs#L58)).

**Ground seat.** Generated and authored props sit on `shelf_offset` (vegetation [vegetation.rs:467](../client/src/vegetation.rs#L467), conifers [sky_backdrop.rs:513](../client/src/sky_backdrop.rs#L513), deer [living_ecology.rs:535](../client/src/living_ecology.rs#L535)). GROUND-SNAP cards own this.

## 5. Determinism rule for later code cards

- Generation is pure code in `shared/`, with no Bevy, no clock and no global RNG ([PROCGEN_DESIGN.md:98](PROCGEN_DESIGN.md#L98)).
- No wall clock as game truth, no unseeded randomness and no `HashMap` iteration order as gameplay ([SIM_AND_HAND_CANON.md:75-79](SIM_AND_HAND_CANON.md#L75)).
- Any later authority runs the same function on the same seed.

## 6. Fixture rule (binding on WORLD-TRAVEL-1 zone builds; not built)

From ruling 7117162436:

1. The same seed produces identical output on every run.
2. Every hostile spawn sits inside its zone's level band.

The `layout_hash` test is planned ([PROCGEN_DESIGN.md:102-103](PROCGEN_DESIGN.md#L102)). A hash change bumps `gen_version`. When a code card adds it, it falls under the Build / CI row ([SYSTEMS_SPINE.md:450](SYSTEMS_SPINE.md#L450)). Bands stay placeholders until LEVELS-LADDER-MAP-1.

## 7. Runtime cost

- Output is baked at build time or generated once per world seed.
- No live AI calls and no network calls.
- Cost stays inside the 1080p/60 floor and the WORLD-DIRECTOR-1 budgets. That figure is a target, not a measurement ([SYSTEMS_SPINE.md §13.1](SYSTEMS_SPINE.md#L377)). WORLD-DIRECTOR budgets are planned ([SYSTEMS_SPINE.md:66](SYSTEMS_SPINE.md#L66)).
- No hardware claim is made here.

## 8. SPINE §12 row 6 (World), for the record

This records the row for [SYSTEMS_SPINE.md:365](SYSTEMS_SPINE.md#L365) (status planned). SPINE is not edited.

| Column | Value |
|---|---|
| Authority owner | Client, offline, as built. Planned: generation is pure `shared/` code, so a later authority runs the same function. |
| State that would replicate | The seed recipe (world seed, zone, `gen_version`) plus edits, never the mesh ([PROCGEN_DESIGN.md:100-101](PROCGEN_DESIGN.md#L100), [KEEPSAKE_SEED.md:12](KEEPSAKE_SEED.md#L12), [GENSHARE.md:9](GENSHARE.md#L9)). |
| Serializable | Yes for the grove only today ([shared/genshare.rs:1](../shared/genshare.rs#L1), `seed_u64` [shared/shard_climate.rs:31](../shared/shard_climate.rs#L31)). Zone seed: planned. |
| Deterministic input | Fixed seeds today (§3 legacy table). No clock and no global RNG. |

## 9. Open, pending Sherif

These stay open ([PROCGEN_DESIGN.md:106-114](PROCGEN_DESIGN.md#L106)). This page decides none of them:

- Acts and biome order ([:108](PROCGEN_DESIGN.md#L108)).
- New Places ([:110](PROCGEN_DESIGN.md#L110)).
- Shared world seed vs per-character seed, and whether zones re-roll ([:111](PROCGEN_DESIGN.md#L111)).
- Online scope for generated zones; online stays grey ([:113](PROCGEN_DESIGN.md#L113)).
- The rest of that list.

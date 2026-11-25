# Look target

LOOK-TARGET-1. Docs only. This file is new. Older files stay as they are; this page points at them.

The PATSAGi Councils adopted this page under Sherif's typed delegation t1767u, t1948u and t2247u. Sherif may veto or amend. Everything here is a **target, not a final look ruling**: every per-zone and hero look OK is Sherif's typed call (#738 comment 7117117730).

What this page is: the per-zone and hero roll-up and index for the look. It links the three docs that already own the look and adds no knob value, no hex, no new swatch and no precedence layer above them.

What it links instead of repeating:

- Visual law and Place accents: [ART_BIBLE.md:40-45](ART_BIBLE.md#L40). One shader family: [:33-38](ART_BIBLE.md#L33). Direction override: [:20-24](ART_BIBLE.md#L20).
- Procedural knobs per tier: [VISUAL_TARGET.md](VISUAL_TARGET.md), Palette [:74](VISUAL_TARGET.md#L74), Sky and atmosphere [:132](VISUAL_TARGET.md#L132), Fog [:158](VISUAL_TARGET.md#L158), Light and bloom [:182](VISUAL_TARGET.md#L182), Terrain and material [:228](VISUAL_TARGET.md#L228), Silhouettes [:254](VISUAL_TARGET.md#L254).
- Reference plates and the ordered visual-pass card plan: [visual-pass/TARGET_BOARD.md:7-12](visual-pass/TARGET_BOARD.md#L7), [:28-47](visual-pass/TARGET_BOARD.md#L28).
- The world generation method lives in [WORLD_GEN.md](WORLD_GEN.md); this page does not restate it.

## 1. Precedence (stated by cite, not changed)

The existing order stands: [VISUAL_TARGET.md:37-47](VISUAL_TARGET.md#L37). Where a frame conflicts with canon, ART_BIBLE wins ([VISUAL_TARGET.md:43](VISUAL_TARGET.md#L43)). A conflict is a gap, not a new look. This page sits below both and decides nothing between them.

Colours are named by constant and line only, never as new swatches. The knob values stay owned by VISUAL_TARGET and the `climate_plane.rs` constants.

## 2. Code owners of the look

| Layer | Owner (verified at this page's base) |
|---|---|
| Dress struct | `struct ClimateLook` [climate_plane.rs:377](../client/src/climate_plane.rs#L377) |
| Realm to look | `fn look_for` [climate_plane.rs:393](../client/src/climate_plane.rs#L393); arms Heartwood [:400](../client/src/climate_plane.rs#L400), Crystal Spires / Voidfarer Horizon [:421](../client/src/climate_plane.rs#L421), Abyssal Depths [:446](../client/src/climate_plane.rs#L446), Sanctuary Prime [:465](../client/src/climate_plane.rs#L465) |
| Realm to Place mood | `fn place_mood_for` [climate_plane.rs:538](../client/src/climate_plane.rs#L538) |
| Travel to dress | `fn sync_place_dress_from_travel` [climate_plane.rs:1661](../client/src/climate_plane.rs#L1661) |
| Apply dress | `fn apply_climate_look` [climate_plane.rs:1854](../client/src/climate_plane.rs#L1854) |
| One painterly shader | header [painterly.wgsl:1-4](../client/assets/shaders/painterly.wgsl#L1); `struct PainterlyExtension` [painterly_shader.rs:54](../client/src/painterly_shader.rs#L54), `type PainterlyMaterial` [:67](../client/src/painterly_shader.rs#L67), shader include [:76](../client/src/painterly_shader.rs#L76) |
| Painterly roles | `enum PainterlyRole` [painterly_mat.rs:28](../client/src/painterly_mat.rs#L28), `fn painterly_shade` [:75](../client/src/painterly_mat.rs#L75), `struct PainterlyPlugin` [:213](../client/src/painterly_mat.rs#L213) (registered [lib.rs:258](../client/src/lib.rs#L258)), `fn paint_height_ramp` [:259](../client/src/painterly_mat.rs#L259) |
| Generated layers | `LightGenPlugin` [light_gen.rs:112](../client/src/light_gen.rs#L112), `fn vegetation_plants` [vegetation.rs:368](../client/src/vegetation.rs#L368), `fn ring_peaks` [sky_backdrop.rs:325](../client/src/sky_backdrop.rs#L325), `fn terrace_height` [terrain.rs:120](../client/src/terrain.rs#L120) |
| Quality tier | `enum GraphicsPreset` [local_settings.rs:164](../shared/local_settings.rs#L164) (`ALL` [:174](../shared/local_settings.rs#L174)); `enum MeshLod` [:209](../shared/local_settings.rs#L209) (`ALL` [:220](../shared/local_settings.rs#L220)) |
| Engine pin | client Bevy 0.20 [client/Cargo.toml:26](../client/Cargo.toml#L26) |

Spine: this card plugs into SYSTEMS_SPINE §11.6 World ([SYSTEMS_SPINE.md:263-277](SYSTEMS_SPINE.md#L263)), whose contract is "dress per place" ([:265](SYSTEMS_SPINE.md#L265)). The budget table in §5 cites §13 PERF ([:375](SYSTEMS_SPINE.md#L375), §13.1 [:377](SYSTEMS_SPINE.md#L377)). It replicates no state, so the World row in §12 NET-READY stays planned ([:365](SYSTEMS_SPINE.md#L365)). It adds no player-facing copy.

## 3. Per zone (keyed on what exists)

Blocks key on the four ART_BIBLE Places ([ART_BIBLE.md:40-45](ART_BIBLE.md#L40)) and the five realm ids ([WORLD_BIBLE.md:22-28](WORLD_BIBLE.md#L22); labels in code at [human_soft_panels.rs:33-39](../client/src/human_soft_panels.rs#L33)). Each block is a target for a side-by-side and a typed look OK, not a claim that the game shows it today.

### Sanctuary (realm 0, Sanctuary Prime)

- Accent and must / must not: [ART_BIBLE.md:42](ART_BIBLE.md#L42).
- Dress arm: [climate_plane.rs:465](../client/src/climate_plane.rs#L465). Constants: `SANCTUARY_WELL_GOLD` [:191](../client/src/climate_plane.rs#L191), `SANCTUARY_VALLEY_EARTH` [:198](../client/src/climate_plane.rs#L198), `SANCTUARY_VALLEY_SKY` [:200](../client/src/climate_plane.rs#L200), `SANCTUARY_VALLEY_FOG` [:201](../client/src/climate_plane.rs#L201).
- Place identity bar: [PLACE_DRESS_SPEC.md:80](PLACE_DRESS_SPEC.md#L80).

### Heartwood (realm 2, Verdant Heartwood)

- Accent and must / must not: [ART_BIBLE.md:43](ART_BIBLE.md#L43).
- Dress arm: [climate_plane.rs:400](../client/src/climate_plane.rs#L400). Constants: `HEARTWOOD_AMBER_LAMP` [:224](../client/src/climate_plane.rs#L224), `HEARTWOOD_WOOD_SKY` [:235](../client/src/climate_plane.rs#L235), `HEARTWOOD_WOOD_FOG` [:236](../client/src/climate_plane.rs#L236).
- Place identity bar: [PLACE_DRESS_SPEC.md:81](PLACE_DRESS_SPEC.md#L81).

### Threshold (realms 1 and 4, Crystal Spires / Voidfarer Horizon)

- Accent and must / must not: [ART_BIBLE.md:44](ART_BIBLE.md#L44).
- Dress arm: [climate_plane.rs:421-438](../client/src/climate_plane.rs#L421). Constants: `THRESHOLD_TEND_SEAM` [:244](../client/src/climate_plane.rs#L244), `THRESHOLD_PIPE_SKY` [:256](../client/src/climate_plane.rs#L256), `THRESHOLD_PIPE_FOG` [:257](../client/src/climate_plane.rs#L257).
- Place identity bar: [PLACE_DRESS_SPEC.md:82](PLACE_DRESS_SPEC.md#L82).
- Realm 1 and realm 4 share one dress ([climate_plane.rs:421-426](../client/src/climate_plane.rs#L421); [WORLD_BIBLE.md:83](WORLD_BIBLE.md#L83)). This page records that and does not resolve it; realm naming and V4 belong to other cards and to Sherif.

### Depths (realm 3, Abyssal Depths)

- Accent and must / must not: [ART_BIBLE.md:45](ART_BIBLE.md#L45).
- Dress arm: [climate_plane.rs:446](../client/src/climate_plane.rs#L446). Constants: `DEPTHS_TEAL_PEACE` [:265](../client/src/climate_plane.rs#L265), `DEPTHS_WET_SKY` [:277](../client/src/climate_plane.rs#L277), `DEPTHS_WET_FOG` [:278](../client/src/climate_plane.rs#L278).
- Place identity bar: [PLACE_DRESS_SPEC.md:83](PLACE_DRESS_SPEC.md#L83).

### Proposed zones (PROPOSAL only)

WORLD_BIBLE §6 Z1-Z5 and Orbit are cited only as a PROPOSAL ([WORLD_BIBLE.md:134-149](WORLD_BIBLE.md#L134)). For any zone with no code dress today (Orbit, the gated deep, any new zone) the look is **planned, needs Sherif's typed look OK**. This page gives such zones no look and no name.

## 4. Hero (direction only)

- Authored body gate: the private body path at [human_presence.rs:1695](../client/src/human_presence.rs#L1695), loaded by `fn spawn_authored_ual1_presence` [:1711](../client/src/human_presence.rs#L1711). Low stays on primitives via `MeshLod::primitives_only` ([local_settings.rs:232](../shared/local_settings.rs#L232)).
- Practice dress: [ART_BIBLE.md:51-57](ART_BIBLE.md#L51). Cydruid dress rule: [ART_BIBLE.md:61](ART_BIBLE.md#L61). Not restated here.
- Draek: see the row at [ART_BIBLE.md:54](ART_BIBLE.md#L54). Draeks carry a secret that is found through play; this page does not state it.
- The build belongs to the planned card HERO-LOOK-1 and needs Sherif's typed look OK (#738 comment 7117117730).

## 5. Asset budget (ASSET-PIPE-1 table, docs only)

Moved here by QUEUE RULING 7118811899 (#738). Only the table moves.

**Tiers.** Rows are the five `GraphicsPreset` tiers mapped to the three `MeshLod` tiers by `fn mesh_lod` ([local_settings.rs:250](../shared/local_settings.rs#L250); Mobile to Low, Ultra to High per [:248-249](../shared/local_settings.rs#L248)). The mesh feel per tier stays ruled by [MESH_QUALITY_BUDGET.md:17-23](MESH_QUALITY_BUDGET.md#L17); this page does not re-rule it.

| GraphicsPreset | MeshLod | Mesh rule (cite) |
|---|---|---|
| Mobile | Low | primitives only ([local_settings.rs:232](../shared/local_settings.rs#L232)) |
| Low | Low | primitives only ([local_settings.rs:232](../shared/local_settings.rs#L232)) |
| Medium | Medium | optional glb ([local_settings.rs:242](../shared/local_settings.rs#L242)) |
| High | High | optional glb ([local_settings.rs:242](../shared/local_settings.rs#L242)) |
| Ultra | High | optional glb ([local_settings.rs:242](../shared/local_settings.rs#L242)) |

**Measured baseline.** Read from each tracked glb's JSON chunk (triangle-list primitives: index count / 3; counts of `materials`, `images`, `skins`, `animations`) and from the PNG IHDR header, at this page's base.

| Asset | Triangles | Materials | Images | Skins / animations |
|---|---|---|---|---|
| kenney-fantasy-town-kit/pillar-stone | 124 | 1 | 1 | none |
| kenney-fantasy-town-kit/pillar-wood | 44 | 1 | 1 | none |
| kenney-nature-kit/log_large | 96 | 2 | 0 | none |
| kenney-nature-kit/stone_smallFlatA | 20 | 1 | 0 | none |
| kenney-nature-kit/tree_default | 114 | 2 | 0 | none |
| quaternius-modular-men/ko-traveler | 6206 | 7 | 0 | 8 skins / 1 animation |
| quaternius-modular-men/mira-traveler | 6634 | 8 | 0 | 8 skins / 1 animation |
| quaternius-modular-men/ren-traveler | 5240 | 11 | 0 | 8 skins / 1 animation |

The five ambientCG colour maps (`assets/textures/ambientcg/*/color.png`) are 512 x 512.

**Targets.** Any budget number that is not in the measured table is a **target, not a measurement** ([SYSTEMS_SPINE.md:377](SYSTEMS_SPINE.md#L377)). The 1080p/60 floor is a target only. No hardware claim is made.

**Licence row.** The public asset guard passes models and textures only as CC0 or self-made AG-SML-1.1 ([check_asset_licenses.sh:4](../scripts/check_asset_licenses.sh#L4), `licence_ok` [:17](../scripts/check_asset_licenses.sh#L17)), run in CI ([ci.yml:37](../.github/workflows/ci.yml#L37), [:39](../.github/workflows/ci.yml#L39)). Current kits are recorded in [THIRD_PARTY_ASSETS.md:12-15](../THIRD_PARTY_ASSETS.md#L12) and [asset-licenses.csv](../assets/asset-licenses.csv). New assets are $0 only. CC-BY is **not** accepted today; any widening of the allow-list is an open item for Sherif.

**Import validator: planned, unowned.** No card owns it yet; it needs its own Clerk PRE-PATH and Core shape ruling.

## 6. Concept sources (named by location, never copied)

No concept art file is in this repository. Each piece is named by where it lives, and nothing is copied, downloaded or embedded ([DRIVE_POWRUSH_BANK.md:7](DRIVE_POWRUSH_BANK.md#L7), [visual-pass/TARGET_BOARD.md:5](visual-pass/TARGET_BOARD.md#L5)).

- Drive pieces: by file name and Drive id exactly as listed in [VISUAL_TARGET.md:297-338](VISUAL_TARGET.md#L297) (Corrected Powrush Art, the stills, and the viewed trailer frames).
- powrush.com: the plate URL at [visual-pass/TARGET_BOARD.md:10](visual-pass/TARGET_BOARD.md#L10).
- Excluded pieces ([VISUAL_TARGET.md:28-35](VISUAL_TARGET.md#L28)) and unviewed pieces ([:43](VISUAL_TARGET.md#L43), [:338](VISUAL_TARGET.md#L338)) stay uncited.
- In-game captures under `docs/visual-pass/` are in-game captures, never concept.
- Moving any concept art into this repository is Sherif's call.

All pieces are reference only. This page makes no claim that the game looks like them.

## 7. Standing rules (by cite only)

From #738 comment 7117117730: a side-by-side per zone and for the hero, Sherif's typed look OK per zone and for the hero, hybrid art direction ($0 free-licensed or self-made, licence recorded, one painterly shader), the 1080p/60 floor as a target, and reference only.

## 8. Known stale cites elsewhere (recorded, not fixed here)

These files are out of this card's path, so they stay as they are:

- Code comments citing `VISUAL_TARGET L149` ("flat desaturated sky"): that text is now at [VISUAL_TARGET.md:152](VISUAL_TARGET.md#L152). Nearby L-number cites drift by a similar amount.
- VISUAL_TARGET's ART_BIBLE cites "L39", "L41", "L19" now sit at [ART_BIBLE.md:55](ART_BIBLE.md#L55), [:57](ART_BIBLE.md#L57), [:35](ART_BIBLE.md#L35).
- [VISUAL_TARGET.md:51](VISUAL_TARGET.md#L51) cites `GraphicsPreset` at L160-L169; it is now at [local_settings.rs:164](../shared/local_settings.rs#L164).
- [PLACE_DRESS_SPEC.md:9](PLACE_DRESS_SPEC.md#L9) cites the Place constants at `:141-223`; they are now at [climate_plane.rs:191-278](../client/src/climate_plane.rs#L191).
- [VISUAL_TARGET.md:340-342](VISUAL_TARGET.md#L340) checks Bevy 0.14.2 names; the client pins Bevy 0.20 ([client/Cargo.toml:26](../client/Cargo.toml#L26)).

## 9. Open look questions (listed, not decided)

These are Sherif's look and feel calls:

- ART_BIBLE open council questions Q1-Q5: [ART_BIBLE.md:26-31](ART_BIBLE.md#L26).
- GAP-AMBROSIAN-ACCENT [VISUAL_TARGET.md:110](VISUAL_TARGET.md#L110), GAP-TEAL-OVERLAP [:112](VISUAL_TARGET.md#L112), GAP-COLORBLIND-MAP [:120](VISUAL_TARGET.md#L120), and the gap list at [:278](VISUAL_TARGET.md#L278).
- The earth-tone question: [visual-pass/TARGET_BOARD.md:15](visual-pass/TARGET_BOARD.md#L15).
- Licence widening (for example CC-BY), per §5.

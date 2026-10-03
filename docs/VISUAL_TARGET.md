# VISUAL_TARGET.md — procedural knobs

**CARD:** ART-TARGET-1
**Base:** `6b6fe4d19ba96bec26e93928dcc7e31f859038c6` (main tip). Bevy pin 0.14; lockfile crate `bevy` 0.14.2.
**Contact:** info@Rathor.ai
**Independent of xAI.** No certification / AGSi warranty / legal-product claims.

This file turns the intended look into procedural knobs. It does not ship meshes, images, hex targets, or a new preset enum. Cite Drive files by name and file id only. No image is embedded.

No setting looks perfect on every device; one procedural look scales by adding detail per tier.

No pixel goldens. Lavapipe / headless makes no GPU claim ([`PHYSICS_GRAPHICS_CANON.md`](PHYSICS_GRAPHICS_CANON.md) L48–L50). Online stays grey.

## Exclusion — no tree people

No body built from bark, roots or trunk; no leaves or ferns as hair; no green plant skin, on any race, NPC or golem. Moss, vines, or a leaf emblem on armor or props is fine. A body made of plant is not.

That rule covers Sherif’s avoid-references: a bark-and-root golem in pale green armor with shield and staff; the same golem in a sunlit overgrown hall; a green-skinned merchant with leafy hair. Those looks are not sources for any knob.

Canon backing, quoted from [`ART_BIBLE.md`](ART_BIBLE.md) L39:

```
| Cydruid | teal / leaf-metal **trim / metal accents** (not bark-skin) | Depths |
```

Cydruids are not tree people. The brief names that ruling ACITY-ART-FIX-1. The line in this repo is L39 above. [`ART_BIBLE.md`](ART_BIBLE.md) L45 is the dress paragraph; this card does not restate it. Nothing in this file describes a Cydruid body. Cydruid sheets were not viewed and are not cited, including `Cydruid-Characters.png` (`1BZB54AS9rFa-h1jG480aBokd-w6kUC4y`). **GAP-CYDRUID-STILL.**

Excluded stills — do not cite them for any knob, including palette or light:

| File | Drive id | Why excluded |
|---|---|---|
| `08_four_paths.jpg` | `1nRUCPo6NEm48S3EX9s2ZniqBf1TgjmUk` | Bark / root body and a twig / leaf crown. Treant frame is out. |
| `06_betrayal.jpg` | `1Vj-h-6RuJYCyLFGGsdi0RsITLl27gcZY` | Lower limbs read as gnarled roots. Excluded on the conservative screen. |

The treant trailer frame is out. It was not among the five viewed trailer frames, and this card does not name an unviewed file as that frame. **GAP-TREANT-FRAME.** Unviewed trailer frames stay uncited.

## Precedence

1. Corrected Powrush Art (`Ambrosian-*.png` in Drive folder `1ABL1iwwnVkTeZWKxkpghhdqMbiZxmoOC`).
2. The 12 stills (Drive folder `1ob1lr90URaoRbDYw4YGA7Q90-TaKQ3vz`).
3. PowRush MMO Trailer frames (Drive folder `1KF6ggG6NKuTibrZLsYO1TjbP_THy3I5L`).

Where a frame conflicts with canon, [`ART_BIBLE.md`](ART_BIBLE.md) wins. A conflict is a **GAP-...**, not a new look. Only files the Hands seat viewed and passed are citable. Unviewed `Cydruid-` / `Draek-` / `Human-` / `Quellorian-` sheets, and unviewed Ambrosian sheets, are not citable.

Not used: Powrush Art Collection Draft (`1z9XZxPvBUOFkqq2GJBgjoNosn30Peqnj`), any `ChatGPT Image Sep 13, 2026` png, and the folder `1CUvlmCFmIeKbMNMxNfz19Zpzf2sPSZBg`.

[`DRIVE_POWRUSH_BANK.md`](DRIVE_POWRUSH_BANK.md) L7: references only — do not paste binaries. [`TAPE_TRUTH.md`](TAPE_TRUTH.md) L24: do not treat `acitygames.com` copy as shipped law. The brief’s earth-battle hero is the viewed still `07_earth_battle.jpg` (`1kV_yfwYKkj7Lpj0nOQYI4jsy4DjbIjYd`), Codex lane only.

## Tier ladder

The five-tier ladder exists in [`GraphicsPreset`](../shared/local_settings.rs) (`shared/local_settings.rs` L160–L169, impl L171–L201), landed in #613. It does not invent a second preset enum.

| Doc tier | In code today | Mesh | Weather |
|---|---|---|---|
| Mobile | Present · `GraphicsPreset::Mobile` (L163) | Uses Low's `MeshLod` (`shared/local_settings.rs` L250) | Uses Low's `WeatherFidelity` (L260) |
| Low | `GraphicsPreset::Low` | `MeshLod::Low` — primitives / capsule (`shared/local_settings.rs` L205–L243) | `WeatherFidelity::Low` — gentler fog / soft breath (L269–L313) |
| Medium (default) | `GraphicsPreset::Medium`, device-safe default | `MeshLod::Medium` — balanced Place props; optional glb if already on disk (`shared/local_settings.rs` L205–L243) | `WeatherFidelity::Medium` — Place weather, Peace default (L269–L313) |
| High | `GraphicsPreset::High` | `MeshLod::High` — fuller PersonaCommit dress | `WeatherFidelity::High` — richer beds, fuller band |
| Ultra | Present · `GraphicsPreset::Ultra` (L168) | Uses High's `MeshLod` (`shared/local_settings.rs` L252) | Uses High's `WeatherFidelity` (L262) |

`GraphicsPreset::mesh_lod` and `weather_fidelity` (`shared/local_settings.rs` L245–L265) take five tiers and map onto three. This card does not copy `WeatherFidelity::intensity` into a look target. Stills have no density, and Mobile / Ultra have no value.
On Mobile, `sky_for` flattens/desaturates the sky (#614, `client/src/climate_plane.rs` L300).

Dead `QualityPreset` in `client/src/settings.rs` L45–L50 (`Seedling` / `FlowGuardian` / `Eternal`) is slated for removal in a later card, GFX-TIER-LADDER-1. This card does not remove it.

Each knob table states the Mobile look and the Ultra look. A look that only works at the top is marked **Ultra-only** and is never the baseline. Rows say what that tier adds over the tier above. Mute still kills Tend / Flow dust at every tier, including Ultra ([`ART_BIBLE.md`](ART_BIBLE.md) L21). No tier adds a second `Camera3d` (L22).

## Sample method

Hex strings below were sampled from the named file via PIL by the Hands seat on 2026-10-02. This run did not re-sample and did not open Drive. Method: `quantize(5, MEDIANCUT)` on a 200 px-wide resize for five dominant colours and share; box-filter means of the top 10 percent, the middle band (45–55 percent), and the bottom 10 percent of rows; hue-masked medians on a 300×200 resize (HSV hue in degrees, S and V in 0–1): teal = hue 165–205, S>0.35, 0.15<V<0.6; gold = hue 30–55, S>0.45, V>0.55; bright-sat = V>0.85, S>0.3; violet = hue 260–310, S>0.3, V>0.3. Fewer than 50 matched pixels is n/a. These hexes describe JPEG/PNG pixels, not scene radiance. They are not adopted swatches. No lux, fog distance, exposure, or bloom intensity was read from a still. **GAP-LUX.** **GAP-FOG-DISTANCE.** **GAP-EXPOSURE.** **GAP-BLOOM-INTENSITY.**

## Knob — Palette

### What the stills show

Corrected Ambrosian paintings are warm sandstone and cream stone, with teal-green panels, gold trim, and a small cyan focal. That is the conflict with canon, recorded below, not the working palette.

Samples (sampled from the named file via PIL, method above):

| File | Drive id | Sample |
|---|---|---|
| `Ambrosian-Settlement-v2.png` | `1IV7xzZd71xx-gC-b-q4SzgbDbK4lrC2a` | pal5 `#c3ae98` 26 %, `#3d392a` 20 %, `#948470` 20 %, `#5c584a` 16 %, `#76644d` 16 %; teal panel median `#213b40`; gold trim median `#a6824d` |
| `Ambrosian-Buildings-v2.png` | `1qqyWjwy9EDwh6trrCmv2wXIUTYjrWIKu` | sheet background `#272728` 52 % (not scene); stone `#948674` / `#5e5448`; teal `#1d3237`; gold `#a07c4f`; gem glow `#92d8e1` |
| `Ambrosian-Characters-v2.png` | `1oLgqjcJ2gx_LH1hrKw8JtyZnxw79M-un` | skin pink-mauve; teal `#223b43`; gold `#ab8137`; cyan glow `#6febf7` |
| `Ambrosian-Market-Square.png` | `1rMy4EESGQkKXJX18u5B-rAFWeyemNOfo` | `#6a5b49` 25 %, `#89776b` 24 %, `#393429` 23 %, `#aa9180` 16 %, `#d8bca3` 10 %; teal `#19383f`; gold `#a07c44` |
| `Ambrosian-Energy-District.png` | `1xn5RfoSPerUjqITtKP8zME2aFuk7-pOA` | `#6b5d51`, `#8a7a75`, `#3a352d`, `#b39a8f`, `#ddbda3`; bright-sat `#eac09b` |
| `Ambrosian-Coastal-Lighthouse.png` | `16AoRiDKbEzkX5iit9TAB7JNlFdzvlgEJ` | `#463a37` 26 %, `#8e97b7` 23 %, `#716a6d` 21 %, `#daab9f` 16 %, `#f5ce9c` 12 %; teal `#265b69`; bright-sat `#fbc590` 13 % |
| `Ambrosian-Underwater-Habitat-Exterior.png` | `1g88lbnvsHQ2MRJ5rP6Yvo1wiJa3QJ2g4` | `#44575d` 26 %, `#56767f` 24 %, `#31343e` 23 %, `#b1b0a5` 13 %, `#42a9cb` 13 %; teal `#275a6b` 17 %; bright cyan `#3bbfe6` |

Node stills sit closer to desaturated earth. `01_glow_field.jpg` (`1PCI6iuZkMZc_z6RnP02PMaVUWgWBLa2a`) pal5 `#2c2415`, `#17130b`, `#503d20`, `#7e653d`, `#b99c67`; gold `#b18748`; glow `#eac376`. `02_tired_node.jpg` (`1yo60HhE3d0dWtrthhj8ertazqJpnfVVk`) pal5 `#2f332d`, `#141614`, `#49514b`, `#5d6a65`, `#7f908d` — grey-green, no gold glow. Trailer frames `15_Quellorian_Waterfall_City.png` (`1iPNUWOaoVQ67S8Y_jAbTYNOoJC6l9Zx9`), `19_Earth_Citadel_Game_UI.png` (`15iTQWgyfr0diwLIZl1ckNlTHkCQx7aUq`), `20_Explorers_Alien_Landscape.png` (`1_gzuz9zanfJq6Ri3laL7vG9nnCYd6vyf`), and `30_Team_Overlooks_Waterfall_World.png` (`1AdeT6deOip55o3LrVOGmYUcFp49pjMFk`) are more saturated than the law line. **GAP-TRAILER-SATURATION.** Those samples are not the working palette.

### Canon

Working target until Sherif rules: [`ART_BIBLE.md`](ART_BIBLE.md) L19, quoted:

```
Gritty low-poly volume, high-detail light. Desaturated earth + one biome accent.
```

Ambrosian accent, [`ART_BIBLE.md`](ART_BIBLE.md) L41, quoted:

```
| Ambrosian | prism cool | mythic / portal ring |
```

Place accents (L26–L29): Sanctuary warm gold well; Heartwood amber lamp; Threshold iron + tend seam; Depths teal Peace. Practice accents (L37–L40): Human warm grey-gold; Draek dry red / bronze; Quellorian iridescent violet / pink and pale gold. Cydruid accent is the L39 quote in the exclusion. One accent. No sixth people (L43).

**GAP-AMBROSIAN-ACCENT.** For Sherif. The corrected Ambrosian stills show warm stone / cream / teal / gold / a cyan focal. L41 says `prism cool`. This card does not pick a side. Until he rules, this knob follows [`ART_BIBLE.md`](ART_BIBLE.md): desaturated earth, one accent, Ambrosian word `prism cool`. The sampled hexes are not adopted. L41 gives no hex for `prism cool`. This card invents none.

**GAP-TEAL-OVERLAP.** L39’s Cydruid accent is teal trim. The same Ambrosian stills also lean teal. Two practices would share teal if the stills were adopted. They are not. Depths may stay teal Peace (L29) because that line is canon, not because an Ambrosian painting used teal.

Plates are not world paint. L61–L62, L62 quoted:

```
Mute, invert-Y, remaps, colorblind tokens, grey Online — if Title is in frame, Online is grey.
```

**GAP-COLORBLIND-MAP.** L62 names colorblind tokens. No mapping is in the cited docs. No tier adds one.

### Tiers

| Tier | What this tier shows |
|---|---|
| Mobile | Base look. Desaturated earth and one accent. Ambrosian accent word is `prism cool`. No second accent. No sampled hex. No trailer saturation. |
| Low | Adds a clearer split of that one accent on large surfaces (node, door, trim). Still one accent. |
| Medium (default) | Adds the Place accent from L26–L29 when the player is in that Place. Ambrosian practice word stays `prism cool`. |
| High | Adds the same one accent on fuller dress trim (`MeshLod::High`). No second hue family. |
| Ultra | **Ultra-only:** a saturated scenic grade from trailer frames 15, 20, and 30 may sit as a grade plate. It is not the palette. Sampled Ambrosian teal and gold are not added. |

## Knob — Sky and atmosphere

### What the stills show

`Ambrosian-Settlement-v2.png`: hazy pale mountains in a thin top strip. `Ambrosian-Market-Square.png`: pale warm sky, low sun; top 10 percent mean `#a59382` (sampled from this file via PIL). `Ambrosian-Energy-District.png`: dusky pink-lavender; top 10 percent mean `#aa9595`. `Ambrosian-Coastal-Lighthouse.png`: sunset, orange sun on the horizon, pink-lilac clouds, blue-lilac upper sky; top 10 percent mean `#9297c0`. Strongest warm/cool split in the corrected set. `Ambrosian-Buildings-v2.png` and `Ambrosian-Characters-v2.png`: no sky (studio sheets). `Ambrosian-Underwater-Habitat-Exterior.png`: no sky; the water column is brighter toward the top (top 10 percent mean `#3891b0`).

`04_allocate.jpg` (`1_s_ItJnkshrXRnVvMgh-XRZLMZhXe_ey`): overcast grey with an orange sunset band; top 10 percent `#393b3f`. `05_prefall_hive.jpg` (`1c2y2LKR9p7H_WY_jUjE68nk781IGcXBG`): pink-lilac sunset clouds; sky and light only (organic dress stays uncited). `12_end_lattice.jpg` (`1VCjirlv--lCGXDNaXrn4c2Cmgc-opGuL`): navy night and a starfield with the Milky Way. `07_earth_battle.jpg`: starfield, deep navy, warm glow along the limb — Codex only (fleet / mothership; [`ART_BIBLE.md`](ART_BIBLE.md) L15 and L26).

Trailer, lowest precedence: frame 15 pink-lilac sunset and a huge planet and moon; frame 20 a planet, a small moon, and a low sun; frame 30 a bright sun. Frame 19 has a blue cloudy sky; cite the sky only, never the HUD (**GAP-UI** under Light).

### Canon

Sky is weather and Place, not a second palette. L19 still holds: desaturated earth, one accent. L22: no second `Camera3d`. A cubemap is not named in the cited docs. **GAP-SKYBOX-ASSET.**

Bevy 0.14.2 has `Skybox` in `bevy_core_pipeline` (verified in the 0.14.2 crate). This card does not assign an image to it.

### Tiers

| Tier | What this tier shows |
|---|---|
| Mobile | Base look. A flat desaturated sky. No clouds, planet, moon, or starfield. Underwater Places have no sky; the column is a flat depth color (fog knob). |
| Low | Adds a two-stop horizon-to-zenith shift, still desaturated. |
| Medium (default) | Adds one soft cloud band or a dusk band when the Place still shows that hour (Market Square, Energy District, `04_allocate.jpg`). |
| High | Adds the coastal warm/cool split as that Place’s sky, and a sparse navy night where `12_end_lattice.jpg` is the night plate. No trailer saturation. |
| Ultra | **Ultra-only:** planet, moon, dense Milky Way, and the saturated pink-lilac trailer sky. A `Skybox` image, if a later card supplies one, is Ultra-only. None is adopted here. |

## Knob — Fog

### What the stills show

Haze sits on far masses. Foregrounds stay crisp. `Ambrosian-Settlement-v2.png`: light aerial haze on far mountains and the lake. `Ambrosian-Market-Square.png`: aerial haze on red-rock mesas and distant domes. `Ambrosian-Energy-District.png`: haze over far mountains and the water at the right. `Ambrosian-Coastal-Lighthouse.png`: thin haze on far sea stacks. `Ambrosian-Underwater-Habitat-Exterior.png`: distant ruins and fish fade to blue. `04_allocate.jpg`: layered valley mist between ridges. `02_tired_node.jpg`: heavy grey forest fog, no direct sun. `01_glow_field.jpg` and `03_tend_bloom.jpg` (`1BR-On5LhukAYRJ2ggyfmHDzsUyind9BY`): back-lit sun haze through trees. `10_wanderer.jpg` (`1VgKPlp7aWwCz8cWyLzS6NtjRyWtrPQyB`): back-lit haze at the upper right; dark foreground. `12_end_lattice.jpg`: low cloud bank on the horizon. `20_Explorers_Alien_Landscape.png`: far layers fade to lilac-grey; mist in the valley. `23_Ancient_Temple_Ruins_Expedition.png` (`1MIHerfwUS-CkT6hCi7v9Tbd57OfD-5kp`): dense blue haze between cavern layers. Frame 15: waterfall spray. No still states a start, end, or density. **GAP-FOG-DISTANCE.**

### Canon

[`ART_BIBLE.md`](ART_BIBLE.md) L5: glow is the Use target; fog is weather. L20, quoted in Light below: climate fog behind plates, never through type. [`PHYSICS_GRAPHICS_CANON.md`](PHYSICS_GRAPHICS_CANON.md) L28: climate fog (`FogSettings` / Z planes) answers stress and harmony, on the world `Camera3d` only (L29).

Bevy 0.14.2 names the distance-fog component `FogSettings` with `FogFalloff` (`Linear`, `Exponential`, `ExponentialSquared`, `Atmospheric`) in `bevy_pbr`. The type name `DistanceFog` is absent from the 0.14.2 `bevy_pbr` and `bevy_core_pipeline` sources checked. The lived client already inserts `FogSettings`. This card sets no falloff parameter.

Shafts that fill the air are a different hook: `VolumetricFogSettings` exists in `bevy_pbr` 0.14.2. It is not the weather fog, and it is not wired by this card.

### Tiers

| Tier | What this tier shows |
|---|---|
| Mobile | Base look. One weather-colored wash behind plates. Type stays clear. Foreground stays readable. No shafts. |
| Low | Adds today’s Low feel: gentler fog, soft breath (`WeatherFidelity::Low`). |
| Medium (default) | Adds Place weather. Far mesas, mountains, and domes haze; the plaza stays crisp. A tired node may use the heavy grey wash from `02_tired_node.jpg`. |
| High | Adds layered valley mist (`04_allocate.jpg`) and a stronger depth fade in the underwater Place and in the cavern of frame 23. Still behind plates. |
| Ultra | **Ultra-only:** light shafts through `VolumetricFogSettings` (glow field, tend bloom, underwater column, trailer spray). Baseline fog does not depend on them. |

## Knob — Light and bloom

### What the stills show

Corrected Places use soft daylight or a low sun, plus one small cyan focal (a gem or an orb). `Ambrosian-Energy-District.png`: one large cyan orb on the central hall is the focal. `Ambrosian-Coastal-Lighthouse.png`: low sun, glints on water, a warm lantern in the lighthouse, warm windows. `Ambrosian-Market-Square.png`: long soft shadows, warm stall lanterns, cyan diamond emblems, a polished stone floor. `Ambrosian-Characters-v2.png`: cyan forehead marks, cyan eyes, a cyan chest gem, a glowing staff; no sky. `01_glow_field.jpg`: one gold-white point and a filament network — the warm gold well. `02_tired_node.jpg`: a faint red ember in a cracked mound, no gold glow. `03_tend_bloom.jpg`: golden dust motes over the same mound. `09_crownstone.jpg` (`1SWODoTeZYRqzSL4oTRD0DeA52SJ43sVE`): near-black hall, emissive gold and violet, lightning beams, wet floor — Codex / lore, not the yard. `11_workshop.jpg` (`1lGjzwdsjp_SlXReDAccVJ2RKPEc75CA8`): one warm desk lamp in a dark room; meta still, not a Place. `07_earth_battle.jpg`: gold city lights and a sun rim on the limb — Codex only.

`Ambrosian-Buildings-v2.png` shows worn stone, dark teal insets, gold edge lines, one cyan diamond gem, and warm light through doors.

Frame 19: crowded plaza, polished reflective floor, cyan-blue emissive. Ships on that frame are Codex only (L15, L26). The frame’s HUD, including Social and Galaxy Map, is not a source for any knob. **GAP-UI.** Trailer frames 15 and 30 show strong sun bloom. That bloom is past L19. **GAP-TRAILER-SATURATION.** **Ultra-only.**

### Canon

[`ART_BIBLE.md`](ART_BIBLE.md) L20, quoted:

```
Climate fog behind plates, never through type. Node glow = Use. Lamp disk empty.
```

L21: particles are Tend / Flow dust only, capped, off with Mute. L26: Sanctuary must have one glow and a readable plate, and must not show a fleet. Well glow answers node state ([`PHYSICS_GRAPHICS_CANON.md`](PHYSICS_GRAPHICS_CANON.md) L30), not a second HUD.

**GAP-LAMP-DISK.** For the record, not a new lamp. L20 says the lamp disk is empty, and node glow is Use. L27’s Heartwood accent word is “amber lamp.” Corrected stills and `11_workshop.jpg` also show practical lanterns. This knob does not fill a lamp disk and does not treat a lantern as the Use glow. The Use glow stays the node (the gold point in `01_glow_field.jpg`, the faint ember in `02_tired_node.jpg`).

**GAP-UI.** Frame 19’s HUD conflicts with L62 (quoted under Palette) and with the L66 list below. Cite that frame for plaza reflections and blue emissive only.

**GAP-TELEPORT.** For Sherif. Whether a teleport pad on a building is allowed is open. Until he rules, do not cite `Ambrosian-Buildings-v2.png` for the pad either way: not as allowed, and not as forbidden. `Ambrosian-Settlement-v2.png` shows a cyan-glowing ring on the plaza. This card does not call that ring a teleport and does not adopt a beam. L41’s rhyme words “mythic / portal ring” stay the canon words. They are not a citation of the buildings sheet.

L66, quoted word for word. This line vetoes the words “teleport-beam in HANDS”. It is not a pad ruling:

```
NFT · crypto ticker · race lobby · live Online · XP/kills · Brood Spire in Sanctuary · eat-on-screen · teleport-beam in HANDS · Unreal as cargo-run · “gameplay loop complete” while Online is grey.
```

[`DRIVE_LORE_ADAPTATION.md`](DRIVE_LORE_ADAPTATION.md) L13: “Portal edition beats teleport-beam edition (metal ring + blue vortex; Ambrosian-tech hint).” The vortex stays out of HANDS with the L66 beam words. L13 does not close the pad question, and it is not a citation of `Ambrosian-Buildings-v2.png`.

Bevy 0.14.2, verified in the locked crates, and not given numbers here: `BloomSettings` (`bevy_core_pipeline`); `Tonemapping` (`bevy_core_pipeline`, crate default variant `TonyMcMapface` — this card does not set a variant and does not claim the lived camera overrides it); `CascadeShadowConfig` and `CascadeShadowConfigBuilder` (`bevy_pbr`); `ScreenSpaceAmbientOcclusionSettings` (`bevy_pbr`); `TemporalAntiAliasSettings` (`bevy_core_pipeline`). **GAP-EXPOSURE.** **GAP-BLOOM-INTENSITY.** **GAP-LUX.**

### Tiers

| Tier | What this tier shows |
|---|---|
| Mobile | Base look. One node glow, and that glow means Use. A tired node is a faint ember, not a second accent. No bloom. Empty lamp disk. No extra shadow map. Tend / Flow dust off. |
| Low | Adds one directional light and a simple contact shadow. Dust stays off. |
| Medium (default) | Adds soft daylight or a low sun when the Place still shows it, and keeps a single focal emissive. Capped Tend / Flow dust may appear (`03_tend_bloom.jpg`) and goes off with Mute. Bloom stays off. |
| High | Adds `BloomSettings` on the Use glow only, and cascaded shadows (`CascadeShadowConfig`) so near feet and far domes both keep a shadow. No intensity is set. |
| Ultra | **Ultra-only:** heavy sun bloom and mirror-bright floors from trailer frames 15, 19, and 30; `ScreenSpaceAmbientOcclusionSettings`; `TemporalAntiAliasSettings`. Lightning in `09_crownstone.jpg` stays Codex and is not added to the yard. |

## Knob — Terrain and material rules

### What the stills show

`Ambrosian-Settlement-v2.png`: terraced plaza, stairs, small water channels, cypress-like conifers, domed stone halls, teal ribbed roofs. `Ambrosian-Buildings-v2.png`: worn pale stone shells, dark teal inset panels, gold edge-trim lines, one cyan diamond gem as the focal, warm interior light through doors. Silhouette of those shells is in the next knob. `Ambrosian-Market-Square.png`: silver-steel ribbed canopies, teal awnings, stone paving with inlaid curves, fountain water, orange and red flowering beds, teal banners with a gold emblem. `Ambrosian-Energy-District.png`: ribbed metal-and-stone cylinders, teal bands, tanks with cyan liquid, gold ring inlays in the floor, succulents. `Ambrosian-Coastal-Lighthouse.png`: rock outcrop, lilac and purple ground cover, turquoise shallows, a sea arch, a white-cream lighthouse with teal panels and gold trim, a boat. `Ambrosian-Underwater-Habitat-Exterior.png`: silver and steel domes with teal ribs, glass bubbles, sand, purple and blue coral and kelp, sunken arches far back, a small submersible.

`01_glow_field.jpg`: soil, roots, and a huge trunk — terrain, not a body. `02_tired_node.jpg`: cracked dry earth, dead leaves, moss. `03_tend_bloom.jpg`: moss on the mound and a fern in the ground. `10_wanderer.jpg`: a root tunnel as terrain; the figure is a human in a rag cloak. Frame 15’s crystalline city is a rhyme, not a mesh. Frame 23: arched aqueduct ruins, hooded stone statues, violet crystals. Frame 20: rocky ridge, violet crystal clusters, valley spires, lakes, an arched bridge. Those trailer landforms are below the corrected sheets.

### Canon

L19: gritty low-poly volume, high-detail light, desaturated earth, one accent. L21: water is a bath. L27: Heartwood ribs stay outside the water. L39: leaf-metal is trim, not bark-skin. Moss on stone, a vine on a prop, and a leaf emblem on armor or a banner are props. They are not a body.

[`ART_BIBLE.md`](ART_BIBLE.md) L57, the C-16 line: Heartwood / Wards borrow only the restrained colour rhyme from the waterfall crystalline city, never the city mesh.

L15: do not fuse lanes. A mothership bay in a HANDS still is a defect. `07_earth_battle.jpg` and the ships in frames 15, 19, and 30 stay Codex.

### Tiers

| Tier | What this tier shows |
|---|---|
| Mobile | Base look. A ground plane, water as a flat bath, and large stone masses. Trees, roots, moss, and a ground fern are terrain or props. No plant body. |
| Low | Adds `MeshLod::Low` primitives. People read as capsules. Stone and one inset panel are a flat color split. |
| Medium (default) | Adds Place props: stairs, channels, a dome shell, stall canopies. Optional on-disk glb only if it is already present (`MeshLod::Medium`). |
| High | Adds ribbed roofs, trim lines, conifers, flower beds, and the lighthouse mass for that Place. Underwater domes and tubes may appear for that Place. Fuller dress stays `MeshLod::High`. |
| Ultra | **Ultra-only:** trailer valley dressing (broken rings, snow peaks) and kelp thickets. The crystalline city mesh is not added at any tier (L57). |

## Knob — Silhouettes

### What the stills show

`Ambrosian-Settlement-v2.png`: rounded domes, ribbed barrel vaults, a pointed central arch with a gem. Readable at distance. `Ambrosian-Buildings-v2.png`: dome, ribs, twin horn-pylons; the gem crest reads at thumbnail size. `Ambrosian-Characters-v2.png`: a broad bulky armored figure against a tall caped caster with flared gold shoulder fins. Skin on that sheet is pink-mauve, not plant. `Ambrosian-Market-Square.png`: ribbed arched pavilions; robed figures with mauve skin. `Ambrosian-Energy-District.png`: barrel-vault halls and tanks; a violet-skinned armored figure. `Ambrosian-Coastal-Lighthouse.png`: a tall lighthouse with a bulb top against the bright sky, plus a rock arch. `Ambrosian-Underwater-Habitat-Exterior.png`: segmented dome modules, tube connectors, a small submersible.

`10_wanderer.jpg`: a cloaked human against a dark tunnel. `01_glow_field.jpg`: the trunk and roots read as a terrain mass. Frame 20: jagged spires and ring arcs — trailer, lowest precedence. Frame 15: a vertical crystalline city — colour rhyme only (L57).

`05_prefall_hive.jpg` is not a silhouette source (sky and light only). No Cydruid sheet is a silhouette source. **GAP-CYDRUID-STILL.**

### Canon

L19 plus L26: one readable mass, one glow. Mobile must carry the dome and the two character masses, because the buildings sheet reads at thumbnail size. Thumbnail read is the floor, not an Ultra reward. L22: no second camera. L61: opaque plates; this knob does not restyle type.

### Tiers

| Tier | What this tier shows |
|---|---|
| Mobile | Base look. A dome cluster and a pointed arch, plus two character masses: wide plate and tall caster. The gem crest is a notch in the dome so it still reads small. |
| Low | Adds capsule proportions that keep wide versus tall (`MeshLod::Low`). |
| Medium (default) | Adds the pavilion arch, the lighthouse bulb, and the rock arch when that Place is loaded. |
| High | Adds horn-pylons, the cape, and the staff so the caster separates from the brute (`MeshLod::High`). |
| Ultra | **Ultra-only:** jagged spire fields and ring arcs from frame 20. The crystalline city skyline is not added (L57). The Ambrosian read does not depend on either. |

## Gaps

| Id | For | What is open |
|---|---|---|
| **GAP-AMBROSIAN-ACCENT** | Sherif | Warm stone / cream / teal in the corrected Ambrosian stills versus L41 `prism cool`. No side is picked. The Palette knob follows [`ART_BIBLE.md`](ART_BIBLE.md) until he rules. No hex is invented for `prism cool`. |
| **GAP-TEAL-OVERLAP** | Record | L39 teal trim and the Ambrosian stills’ teal panels. Stills are not adopted, so the knob does not give both practices the still teal. |
| **GAP-TELEPORT** | Sherif | Whether a teleport pad on a building is allowed. L66 vetoes only “teleport-beam in HANDS” (full line quoted under Light). `Ambrosian-Buildings-v2.png` is not cited for the pad either way. The settlement ring is not adopted as a teleport or a beam. |
| **GAP-UI** | Record | Frame 19’s HUD, including Social and Galaxy Map, conflicts with L62 and L66. Not a knob source. |
| **GAP-CYDRUID-STILL** | Record | Cydruid sheets were not viewed. They stay uncited. This file does not describe a Cydruid body. |
| **GAP-LAMP-DISK** | Record | L20 “Lamp disk empty” and node glow = Use, against L27 “amber lamp” and practical lanterns in the stills. Lanterns are not the Use glow. |
| **GAP-TRAILER-SATURATION** | Record | Frames 15, 19, 20, and 30 are more saturated and bloom-heavy than L19. Ultra-only flavour, never baseline. |
| **GAP-TREANT-FRAME** | Record | The ruled-out treant trailer frame was not in the five viewed frames. No unviewed file is named for it. |
| **GAP-FOG-DISTANCE** | Record | No start, end, or density in the stills or in the cited canon lines. |
| **GAP-LUX** | Record | No illuminance in the stills or the cited docs. |
| **GAP-EXPOSURE** | Record | No exposure value. `Tonemapping` is named, not tuned. |
| **GAP-BLOOM-INTENSITY** | Record | `BloomSettings` exists. No intensity is set. |
| **GAP-SKYBOX-ASSET** | Record | `Skybox` exists in 0.14.2. No cubemap is named or shipped. |
| **GAP-COLORBLIND-MAP** | Record | L62 names colorblind tokens. No mapping is specified. |

## Citable files

Viewed and allowed. Hexes are sampled from that file via PIL, method above.

### Corrected Powrush Art

| File | Drive id |
|---|---|
| `Ambrosian-Settlement-v2.png` | `1IV7xzZd71xx-gC-b-q4SzgbDbK4lrC2a` |
| `Ambrosian-Buildings-v2.png` | `1qqyWjwy9EDwh6trrCmv2wXIUTYjrWIKu` |
| `Ambrosian-Characters-v2.png` | `1oLgqjcJ2gx_LH1hrKw8JtyZnxw79M-un` |
| `Ambrosian-Market-Square.png` | `1rMy4EESGQkKXJX18u5B-rAFWeyemNOfo` |
| `Ambrosian-Energy-District.png` | `1xn5RfoSPerUjqITtKP8zME2aFuk7-pOA` |
| `Ambrosian-Coastal-Lighthouse.png` | `16AoRiDKbEzkX5iit9TAB7JNlFdzvlgEJ` |
| `Ambrosian-Underwater-Habitat-Exterior.png` | `1g88lbnvsHQ2MRJ5rP6Yvo1wiJa3QJ2g4` |

### Stills

| File | Drive id | Lane note |
|---|---|---|
| `01_glow_field.jpg` | `1PCI6iuZkMZc_z6RnP02PMaVUWgWBLa2a` | HANDS node. Trunk and roots are terrain. |
| `02_tired_node.jpg` | `1yo60HhE3d0dWtrthhj8ertazqJpnfVVk` | Tired node. Grey fog, faint ember. |
| `03_tend_bloom.jpg` | `1BR-On5LhukAYRJ2ggyfmHDzsUyind9BY` | Tend dust. Fern is in the ground. |
| `04_allocate.jpg` | `1_s_ItJnkshrXRnVvMgh-XRZLMZhXe_ey` | Valley mist, dusk band, emissive lines. |
| `05_prefall_hive.jpg` | `1c2y2LKR9p7H_WY_jUjE68nk781IGcXBG` | Sky and light only. |
| `07_earth_battle.jpg` | `1kV_yfwYKkj7Lpj0nOQYI4jsy4DjbIjYd` | Codex only. Earth-battle hero. |
| `09_crownstone.jpg` | `1SWODoTeZYRqzSL4oTRD0DeA52SJ43sVE` | Codex / lore hall. Not the yard. |
| `10_wanderer.jpg` | `1VgKPlp7aWwCz8cWyLzS6NtjRyWtrPQyB` | Human cloak. Root tunnel is terrain. |
| `11_workshop.jpg` | `1lGjzwdsjp_SlXReDAccVJ2RKPEc75CA8` | Meta still. Not a Place. |
| `12_end_lattice.jpg` | `1VCjirlv--lCGXDNaXrn4c2Cmgc-opGuL` | Night sky and emissive lines. |

### Trailer frames viewed

| File | Drive id | Use |
|---|---|---|
| `15_Quellorian_Waterfall_City.png` | `1iPNUWOaoVQ67S8Y_jAbTYNOoJC6l9Zx9` | C-16 colour rhyme only. Never the city mesh. |
| `19_Earth_Citadel_Game_UI.png` | `15iTQWgyfr0diwLIZl1ckNlTHkCQx7aUq` | Plaza reflections and blue emissive only. Not the HUD. |
| `20_Explorers_Alien_Landscape.png` | `1_gzuz9zanfJq6Ri3laL7vG9nnCYd6vyf` | Ultra-only valley and sky flavour. |
| `23_Ancient_Temple_Ruins_Expedition.png` | `1MIHerfwUS-CkT6hCi7v9Tbd57OfD-5kp` | Cavern haze and cool emissive. |
| `30_Team_Overlooks_Waterfall_World.png` | `1AdeT6deOip55o3LrVOGmYUcFp49pjMFk` | Ultra-only sun bloom. Ships are Codex. |

Remaining trailer frames in folder `1KF6ggG6NKuTibrZLsYO1TjbP_THy3I5L` were not viewed and are not citable.

## Bevy 0.14.2 names checked

Checked against crate sources for `bevy` 0.14.2 in `Cargo.lock` (`bevy_pbr` 0.14.2 and `bevy_core_pipeline` 0.14.2). Present: `FogSettings`, `FogFalloff`, `VolumetricFogSettings`, `BloomSettings`, `Tonemapping`, `Skybox`, `CascadeShadowConfig`, `CascadeShadowConfigBuilder`, `ScreenSpaceAmbientOcclusionSettings`, `TemporalAntiAliasSettings`. Absent: a public type named `DistanceFog`. No other renderer feature is named, because it was not checked.

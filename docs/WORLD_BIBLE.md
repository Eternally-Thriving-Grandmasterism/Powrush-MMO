# World bible

WORLD-BIBLE-1. Docs only. This file is new. Older files stay as they are; this page points at them.

The PATSAGi Councils adopted this page under Sherif's typed delegation t1767u, t1948u and t2247u. Sherif may veto or amend. The realm names in §2 are owner-ruled by Sherif. The zone map in §6 is a PROPOSAL.

The build this page describes is offline. Title Online stays grey, and server work stays planned ([shared/net_mode.rs:5](../shared/net_mode.rs#L5), [:18](../shared/net_mode.rs#L18)).

What this page owns: the five realms as dress, the places, travel today, canon mapped to code, the realm names, and the zone map PROPOSAL.

What it links instead of repeating:

- The weighting and the one Sherif quotation about the five levels live in [DESIGN_PILLARS.md:13](DESIGN_PILLARS.md#L13) and [:17](DESIGN_PILLARS.md#L17).
- The Zones row is [DESIGN_PILLARS.md:37](DESIGN_PILLARS.md#L37).
- The four-Places cap is under a PROPOSAL supersede in [DESIGN_PILLARS.md:87-89](DESIGN_PILLARS.md#L87), with the restatements at [:96](DESIGN_PILLARS.md#L96). This page adds no second supersede table.
- Code owners and the Places-vs-realm-switch overlap live in [SYSTEMS_SPINE.md §6 World](SYSTEMS_SPINE.md#L62). This page does not copy the owner table.

## 1. The five realms, as named in code

The realm id is `SoftPlayerRealm { current: Option<u8> }` ([living_practice_loop.rs:29-31](../client/src/living_practice_loop.rs#L29)). It is switched by Z or F3 ([soft_play_bindings.rs:27](../client/src/soft_play_bindings.rs#L27), [:46](../client/src/soft_play_bindings.rs#L46)), then a digit 1–5 ([human_soft_panels.rs:168](../client/src/human_soft_panels.rs#L168)).

| id | Panel / journal label ([human_soft_panels.rs:33-39](../client/src/human_soft_panels.rs#L33)) | Look name (`look_for`, [climate_plane.rs:393-480](../client/src/climate_plane.rs#L393)) | Place mood ([:537-544](../client/src/climate_plane.rs#L537)) | Schedule place ([living_ecology.rs:128-135](../client/src/living_ecology.rs#L128)) |
|---|---|---|---|---|
| 0 | Sanctuary Prime (:34) | Sanctuary Prime (:466) | SanctuarySkyYard | Sanctuary |
| 1 | Synthetic Lattice (:35) | Crystal Spires (:423) | ThresholdPipeAir | Threshold |
| 2 | Verdant Bloom (:36) | Verdant Heartwood (:401) | HeartwoodCanopy | Heartwood |
| 3 | Harmonic Chorus (:37) | Abyssal Depths (:447) | DepthsWetStone | Depths |
| 4 | Voidfarer Horizon (:38) | Voidfarer Horizon (:425) | ThresholdPipeAir | Threshold |

The five realms are not places. Today they are dress on one plate (§3).

## 2. Realm names

### Owner-ruled

Sherif answered "1a 2a" (Lead position 8412, recorded in comment 7118691808):

- **V1, owner-ruled.** Players see **Crystal Spires**, **Verdant Heartwood** and **Abyssal Depths** for realm ids 1, 2 and 3. These are the look names at [climate_plane.rs:423](../client/src/climate_plane.rs#L423), [:401](../client/src/climate_plane.rs#L401) and [:447](../client/src/climate_plane.rs#L447).
- **V3, owner-ruled.** Realm id 4 keeps the name **Voidfarer Horizon** ([climate_plane.rs:425](../client/src/climate_plane.rs#L425), [human_soft_panels.rs:38](../client/src/human_soft_panels.rs#L38)).
- **V2 is moot.** It was the other way round (keep the panel labels and rename the look names), and V1 rules it out.
- **V4 stays open for Sherif.** Any future zone name drawn from LORE_BIBLE biomes is a new on-screen name and his call.

This began as a councils' PROPOSAL in ruling 7118332320 and was amended by the owner ruling above.

### Legacy on-screen labels

Until REALM-NAMES-1 ships, the screen still shows the legacy panel labels Synthetic Lattice, Verdant Bloom and Harmonic Chorus ([human_soft_panels.rs:35-37](../client/src/human_soft_panels.rs#L35)):

- The Z / F3 panel lists them under the head "`{place} · 1–5 choose a climate`" ([human_soft_panels.rs:228-233](../client/src/human_soft_panels.rs#L228), [:260-268](../client/src/human_soft_panels.rs#L260)).
- The journal line "`Traveled to {name}`" uses them ([human_soft_panels.rs:197-201](../client/src/human_soft_panels.rs#L197)).
- The plugin is always added ([lib.rs:290](../client/src/lib.rs#L290)).

The look names do not reach on-screen text in the live build. The climate chip shows `PlaceId::chip_name` whenever `HexTravelState` exists ([climate_plane.rs:2086-2089](../client/src/climate_plane.rs#L2086), [shared/hex_travel.rs:81-87](../shared/hex_travel.rs#L81)), and `HexTravelPlugin` is always added ([lib.rs:239](../client/src/lib.rs#L239)).

The place-chip names (Sanctuary Prime, Heartwood, Depths) name places, not realm ids. They are context only.

This page renames nothing in code. REALM-NAMES-1 is the later code card that changes the three panel strings.

### Open note: Azure vs Abyssal

[LORE_BIBLE.md:30](LORE_BIBLE.md#L30) heads a biome "Azure Depths & Living Reefs", marked Future Expansion. The look name for realm id 3 is "Abyssal Depths" ([climate_plane.rs:447](../client/src/climate_plane.rs#L447); module header [:10](../client/src/climate_plane.rs#L10)). LORE_BIBLE is not edited by this page. Whether that line names realm id 3 is left to REALM-NAMES-1's PRE-PATH.

Doc support for the look names: Verdant Heartwood heads a biome at [LORE_BIBLE.md:18](LORE_BIBLE.md#L18), and Crystal Spires at [:25](LORE_BIBLE.md#L25).

## 3. How each realm differs today

A realm switch re-dresses the same 56 × 56 m plate (`PLAY_HALF = 28.0`, [terrain.rs:35](../client/src/terrain.rs#L35)), with one stepped shelf model ([terrain.rs:42-46](../client/src/terrain.rs#L42), [:120-122](../client/src/terrain.rs#L120)).

`apply_climate_look` ([climate_plane.rs:1854](../client/src/climate_plane.rs#L1854)) changes the clear colour from `sky_for` ([:1902](../client/src/climate_plane.rs#L1902)), the ground material and texture set, stone and node colours, fog and the colour grade. The per-realm values are the `look_for` arms ([:400-413](../client/src/climate_plane.rs#L400), [:421-438](../client/src/climate_plane.rs#L421), [:446-459](../client/src/climate_plane.rs#L446), [:465-478](../client/src/climate_plane.rs#L465)).

Realm-gated props, all on the same plate at the same spots:

| Gate | Realms shown | Cite |
|---|---|---|
| Sky backdrop (dome, ring peaks, conifer belt, silhouette bands, terrace ground) | 0 only | [sky_backdrop.rs:168-171](../client/src/sky_backdrop.rs#L168) |
| Dense vegetation field | 0 only | [vegetation.rs:250-252](../client/src/vegetation.rs#L250) |
| Water, falls and bath | 0 only | [water.rs:180-182](../client/src/water.rs#L180) |
| Yard trees (5 fixed spots) | 0, 2 | [living_ecology.rs:211-223](../client/src/living_ecology.rs#L211) |
| Yard rocks (3 fixed spots) | 1, 4 | [living_ecology.rs:225-228](../client/src/living_ecology.rs#L225) |
| Crystal spires (4 fixed spots) | 1, 4 | [living_ecology.rs:229-231](../client/src/living_ecology.rs#L229) |
| Follow-camera look height | 0 differs from the rest | [human_presence.rs:2449-2454](../client/src/human_presence.rs#L2449) |

- Realm 1 and realm 4 differ only in the name string ([climate_plane.rs:422-426](../client/src/climate_plane.rs#L422)).
- Realm 3 has no realm-gated solids.

Distinct systems are keyed by `PlaceId`, not by realm id:

| Place | Distinct systems in code |
|---|---|
| Sanctuary | Stavehold gate plaque and quest sentence ([wellsworn.rs:20-21](../client/src/wellsworn.rs#L20)). Practice fight at `STAVEHOLD_FIGHT_HOME` ([combat_core.rs:58](../client/src/combat_core.rs#L58)). Embassy restore on arrival ([hex_travel.rs:266-268](../client/src/hex_travel.rs#L266)). |
| Heartwood | Lip dress and bath ([heartwood_lip.rs:1](../client/src/heartwood_lip.rs#L1)). Heartwood Prowler ([combat_core.rs:46](../client/src/combat_core.rs#L46)). Heartwood Grazer ([grazer.rs:1-2](../client/src/grazer.rs#L1)). |
| Threshold (a Heartwood sub-area, not a `PlaceId`) | Shelf look and tend ([shared/threshold_shelf.rs:1](../shared/threshold_shelf.rs#L1)). Threshold Sentry, the one hostile droid ([droid.rs:1-2](../client/src/droid.rs#L1), name at [:20](../client/src/droid.rs#L20)). |
| Depths | Landing disk and one Peace node ([depths_landing.rs:1-2](../client/src/depths_landing.rs#L1)). Fresh hex file with zeroed tons on first entry ([shared/hex_travel.rs:505-511](../shared/hex_travel.rs#L505)). |

Bottom line: the five realms are reskins. Sanctuary, Heartwood and Depths are thin distinct places on the same footprint. The councils' reading of the five levels is linked at [DESIGN_PILLARS.md:13](DESIGN_PILLARS.md#L13).

## 4. Travel today

1. **Places plate (the real travel).** It sits on the Esc pause row, shown only when Settled; Heartwood and Depths need the book ([hex_travel.rs:536-540](../client/src/hex_travel.rs#L536)). The buttons are Sanctuary, Heartwood, Threshold and Depths ([:497-500](../client/src/hex_travel.rs#L497)). The confirm is "Leave this hex?" ([shared/hex_travel.rs:43](../shared/hex_travel.rs#L43)), then `apply_travel_named` ([shared/hex_travel.rs:541](../shared/hex_travel.rs#L541)), which writes `powrush_hex_current.json` ([:40](../shared/hex_travel.rs#L40)). Without the book, Heartwood and Depths bounce to Sanctuary ([hex_travel.rs:258-262](../client/src/hex_travel.rs#L258)). A haul can be delivered on arrival ([shared/hex_travel.rs:568](../shared/hex_travel.rs#L568)). The dress follows via `sync_place_dress_from_travel` ([climate_plane.rs:1661](../client/src/climate_plane.rs#L1661)) and `dress_realm_for_place` ([:550-556](../client/src/climate_plane.rs#L550)).
2. **People-door landing.** `apply_people_landing` ([hex_travel.rs:281-296](../client/src/hex_travel.rs#L281)) routes the five peoples onto the three PlaceIds ([hour_sacred.rs:146-185](../client/src/hour_sacred.rs#L146); [PLAYABLE_RACES.md:36-40](PLAYABLE_RACES.md#L36)).
3. **Boot.** Play always boots Sanctuary. Continue with the book reloads the last hex ([hex_travel.rs:234-236](../client/src/hex_travel.rs#L234); [shared/hex_travel.rs:442-450](../shared/hex_travel.rs#L442)).
4. **Z / F3 realm panel.** A dress switch only (§1), though the journal calls it travel. It overrides the Place dress until the next Places travel.
5. **Not travel.** Mercy Transporters is a toggle on key T with no movement ([mercy_transporters.rs:1-5](../client/src/mercy_transporters.rs#L1)). Portals are design only ([COMBAT_AND_PVP.md:360-372](design/COMBAT_AND_PVP.md#L360)); there is no portal code.

There are no roads, distances, seas, mounts, rail or world map in code.

## 5. Canon mapped to code

Peoples: `MechanicalRace { Human, Quellorian, Draek, Cydruid, Ambrosian }` ([shared/persona.rs:331-337](../shared/persona.rs#L331)).

| Canon item | Source | In code? |
|---|---|---|
| One Crownstone, which enables the hive mind's ultimate controller | owner canon (Sherif) | A witnessed set-piece, "Path stays Unset" ([shared/crownstone.rs:1-3](../shared/crownstone.rs#L1)). No hive-control mechanic. |
| A corrupt player can wield the Crownstone and be overtaken, only in the fight against the original NPC hive leader | owner canon (Sherif) | planned |
| A post-Crownstone after-credits world | owner canon (Sherif) | planned |
| Tasteful PK outside protected newbie zones | owner canon (Sherif); [COMBAT_AND_PVP.md:83-92](design/COMBAT_AND_PVP.md#L83) | planned, online-only. "No PvP" offline ([combat_core.rs:1-3](../client/src/combat_core.rs#L1)). Online stays grey. |
| Ranks follow abilities and logical naming | owner canon (Sherif) | no rank code |
| Five faction leaders | [FACTION_LEADERS.md:15-21](design/FACTION_LEADERS.md#L15) | not in code |
| The Draek name | owner canon (Sherif) | `MechanicalRace::Draek` ([shared/persona.rs:334](../shared/persona.rs#L334)) |
| TAUN and The Brood Spire (TBS) as orbital hub towns | [PROCGEN_DESIGN.md:49](PROCGEN_DESIGN.md#L49) | presentation only, no hull mesh ([climate_plane.rs:594-597](../client/src/climate_plane.rs#L594)); no PlaceId |
| Earth seats: surviving cities, the Cydruid stalactite city in Earth's hollows, the Unbound camp on the fractured Earth fringe | [FACTION_LEADERS.md:17-21](design/FACTION_LEADERS.md#L17) | not in code |
| Heartwood rings: Lamp, Lip, Wards, Teeth, Threshold, Spiral | [PLACES_BIBLE.md:20-31](PLACES_BIBLE.md#L20) | Lamp, Lip, Wards, Threshold shelf and Depths landing are built. Teeth is the predator band ([combat_core.rs:1](../client/src/combat_core.rs#L1)). Spiral is lore only. |

Draeks: a secret exists and is found through play.

Gaps:

- G1. No Draek, Quellorian, cyborg-faction or alien creatures are in play code.
- G2. Humans have no city or hub place. Sanctuary is a yard ([PLACES_BIBLE.md:71-76](PLACES_BIBLE.md#L71)).
- G3. No seas, mounts, roads or rail exist in code.
- G4. [LORE_BIBLE.md:8-10](LORE_BIBLE.md#L8) frames Powrush as a parallel carbon copy of Earth's biomes, while [FACTION_LEADERS.md:17-21](design/FACTION_LEADERS.md#L17) places seats on Earth. Earth or a parallel copy is an open lore call for Sherif.
- G5. The LORE_BIBLE biomes ([LORE_BIBLE.md:18-38](LORE_BIBLE.md#L18)) are candidate zone names only. The live build reads none of them.

## 6. Zone map (PROPOSAL)

> Everything in this section is a **PROPOSAL for the councils; Sherif may veto**. Names reuse code or canon names only, and no new name is coined. Level bands are placeholders: levels are per skill, profession and class, and "Powrush thresholds are unnamed placeholders" ([LEVELS_CANON.md:15-17](design/LEVELS_CANON.md#L15), [:69-75](design/LEVELS_CANON.md#L69)). Whether a band reads class level or total level is OPEN (LEVELS-LADDER-MAP-1).

| # | Zone (existing name) | Region on one Earth map | Band | PvP tag ([COMBAT_AND_PVP.md:83-92](design/COMBAT_AND_PVP.md#L83)) | Teaches | Links |
|---|---|---|---|---|---|---|
| Z1 | Sanctuary Prime + Stavehold gate | temperate valley, Human and Ambrosian home | A (newbie) | Sanctuary dome: no player damage | the Use verb, wells, practice fight, House | road to Z2 and Z3; coast later |
| Z2 | Contested ground / Heartwood Teeth | forest belt beside the valley | B | Newcomer Aegis, then contested wilds | first hunting, telegraphs, loot finds | road from Z1; path to Z3 |
| Z3 | Heartwood (Lamp / Lip / Wards) | old forest, Cydruid approach | B | Peace inside the rings | House posts, craft, wood haul | from Z1 or Z2; path up to Z4 |
| Z4 | Threshold / Crystal Spires highlands | mountain shelf above Heartwood | C | contested wilds; hostile droids start here ([COMBAT_AND_PVP.md:93](design/COMBAT_AND_PVP.md#L93)) | ranged threats, scouting, ore | path from Z3; rail later |
| Z5 | Depths landing, then the gated deep | hollows below, the Cydruid stalactite city and the Draek fringe | D+ | landing Peace; past the gate is the lawless deep (online-only, planned) | way-home law, risk and reward | descent from Z3 or Z4; earned portal later |
| Orbit | TAUN, TBS | low orbit ([PROCGEN_DESIGN.md:77](PROCGEN_DESIGN.md#L77)) | hubs | Peace hub proposal ([PROCGEN_DESIGN.md:70-73](PROCGEN_DESIGN.md#L70)) | faction home and deployment | earned tech travel only |

- Voidfarer Horizon (realm id 4) has no distinct place. The councils may fold it into Z4 or retire it.
- The legacy panel labels name no place (§2).
- Travel ladder: on foot, then mount, road, sea, and earned rail or portal ([COMBAT_AND_PVP.md:360-372](design/COMBAT_AND_PVP.md#L360) portals feed this).

## 7. Open decisions

- V4: future zone names from LORE_BIBLE biomes (Sherif).
- Whether the five realms map to zones or retire.
- Earth or parallel-copy framing (G4, Sherif).
- Band units (LEVELS-LADDER-MAP-1).
- Whether TAUN and TBS are in the first map.

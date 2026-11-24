# Systems spine

SYSTEMS-SPINE-1. Docs only. This file is new. No code, data, crate or format change.

The PATSAGi Councils adopted this page under Sherif's typed delegation t1767u, t1948u and t2247u. Sherif may veto or amend. The ten sections are the queue ruling. Every owner line below was re-read at main `6ba72cf2`.

Enforcement starts once this file is on main. From then on, each new card's PRE-PATH cites the spine section it plugs into. Cards already ruled are not held back: COMBAT-FAIR-1, STRIKE-HINT-1, GROUND-SNAP-1, GROUND-SNAP-2, GROUND-SNAP-3, GROUND-SNAP-4 and LOD-LIMBS-1.

Each overlap below is one later migration card. No big-bang rewrite. That card proves no behaviour change unless it names one.

Title Online stays grey, and server work stays planned (`shared/net_mode.rs:5`, `Offline` `:15`, `:18`, `:27`; `client/src/net_mode.rs:1`, `:3`). The `steam` feature is optional and off by default (optional-off) (`client/Cargo.toml:13`, `:14`, `:52`).

## 1. Entities

**Owner:** `client/src/entity_policy.rs:2` (ENTITY-CONTRACT), `#![cfg(test)]` at `:1`, declared at `client/src/lib.rs:108`. The contract table starts at `client/src/entity_policy.rs:37`.

**Status:** partial. A test-side contract exists. The runtime entity registry is planned (ENTITY-PURPOSE-1).

**Overlap:** spawn positions sit in per-module consts, with no single table. One later card. Sites today: `client/src/living_ecology.rs:62` `ResonantDeer`, `:211` `YARD_TREE_SPOTS`, `:225` `YARD_ROCK_SPOTS`, `:229` `YARD_CRYSTAL_SPOTS`; `client/src/combat_core.rs:104` `Predator`; `client/src/droid.rs:33` `Droid`; `client/src/grazer.rs:23` `Grazer`, `:50` `GrazerPlugin`; `client/src/local_human_sim.rs:46` `LocalHumanSim`; `client/src/companion_bond.rs:43` `CompanionBond`, `:59` `CompanionBondPlugin`; `client/src/collide.rs:129` `CreatureBodyPlugin`.

Draeks: a secret exists and is found through play.

## 2. Combat (damage, telegraph, block, recovery, targeting and soft-aim)

COMBAT-FAIR-1's numbers are this section's first rows. That card is already ruled and is not held back.

**Owner:** `client/src/combat_core.rs`. `CombatState` `:72`. Damage: `land_hit` `:327`, `land_hit_paid` `:351`, `combat_fire` `:547`. Telegraph: `tell_visible` `:124` (0.35 s at `:106`). Block: `plate_arm` `:97`. Recovery: `respawn_step` `:391`. Targeting: `in_reach` `:282`, `facing_ok` `:288`. `Predator` `:104`. `PREDATOR_RESPAWN_SECS` `:68`. Also `client/src/difficulty.rs:39` `enemy_hit`, `client/src/droid.rs:1`, and `client/src/combat_text.rs:221` `CombatTextPlugin`.

**Status:** built for one predator and one droid. Soft-aim is planned (no soft-aim function in this tree).

**Overlap:** `client/src/droid.rs:40` `droid_in_reach` and `:59` `shot_step` run a reach and shot path beside `combat_core`. The reach test is `:248`. One later card.

## 3. Stats, items, gear and recipes

**Owner:** `shared/temper.rs:1` (Temper, Lumen, Ward; `WardKind` `:26`). Finds: `client/src/loot_drops.rs:43` (names only, no stats) and `FoundFinds` `:45`, file `client/src/loot_drops.rs:37`. Satchel: `client/src/human_inventory.rs:99` `HumanInventoryPlugin`. Recipes: `shared/fabricator.rs:22` `Recipe` through `:26`, client plugin `client/src/fabricator.rs:66`. The same recipe row is `docs/EA_BAR.md:28`.

**Status:** built, thin. Finds carry no stats (`docs/EA_BAR.md:33`).

**Overlap:** finds (`loot_drops`) and the satchel (`human_inventory`) are separate stores. One later card.

## 4. Progression and the unlock ladder

**Owner:** four tracks, already in code.

1. Skill XP and levels: `client/src/skill_xp.rs:1`, `SkillXp` `:31`, `SkillXpPlugin` `:149`.
2. Temper, Lumen and Ward: `shared/temper.rs:1`.
3. Area difficulty gate: `client/src/difficulty.rs:1`, `DifficultyPlugin` `:216`.
4. Hour and book gate: `client/src/hour_sacred.rs:796` `HourSacred` (`hour_three_complete` `:799`), and the book check in `client/src/hex_travel.rs:258` with `book_flags` `:536`.

**Status:** partial. UNLOCK-LADDER-1 has no doc in this tree: planned.

**Overlap:** the four tracks above. `docs/design/LEVELS_CANON.md:17` and `:85` already leave them unmapped. One later card.

## 5. Quests, journal and captions

**Owner:** no quest log. What exists: `client/src/first_session_guidance.rs:794` `FirstSessionGuidancePlugin` (signposts and the objective strip). `client/src/abundance_journey_echo.rs:146` `AbundanceJourneyEchoPlugin` (journey feed, key `client/src/soft_play_bindings.rs:21`). `client/src/wellsworn.rs:21` `QUEST_TEXT`. Shape tokens sit beside word captions at `client/src/local_settings.rs:8`.

**Status:** mostly planned. There is no quest log (`docs/LOOP_MAP.md:155`).

**Overlap:** three surfaces write player-facing progress lines: guidance (`first_session_guidance`), the journey echo, and combat text (`client/src/combat_text.rs:31` `CombatTextKind`). One later card.

## 6. World (bible, gen seeds, zones, travel, WORLD-DIRECTOR L0–L3 budgets)

**Owner:** travel is `shared/hex_travel.rs:52` `PlaceId` (`LOCAL_HEXES` `:61`, `apply_travel_named` `:541`) and `client/src/hex_travel.rs:251` `apply_place` (`places_plate_clicks` `:625`). Dress is `client/src/climate_plane.rs:393` `look_for`, `:1661` `sync_place_dress_from_travel`, `:1854` `apply_climate_look`. Gen seeds: `shared/powrush_gen.rs:95` `grove_seed`, `client/src/light_gen.rs:112` `LightGenPlugin`, `client/src/vegetation.rs:368` `vegetation_plants`, `client/src/sky_backdrop.rs:325` `ring_peaks`, `client/src/terrain.rs:120` `terrace_height`.

**Status:** partial. The world bible is planned (WORLD-BIBLE-1). WORLD-DIRECTOR L0–L3 budgets are planned (no file and no function in this tree).

**Overlap:** Places travel (`PlaceId`) versus the Z/F3 realm switch. Z is `client/src/soft_play_bindings.rs:27`, F3 is `:46`, and the switch is `client/src/human_soft_panels.rs:168` (travel note `:200`). Two realm-name tables disagree. `client/src/human_soft_panels.rs:34-38` names Sanctuary Prime, Synthetic Lattice, Verdant Bloom, Harmonic Chorus, Voidfarer Horizon. `look_for` names Sanctuary Prime (`client/src/climate_plane.rs:466`), Crystal Spires (`:423`), Verdant Heartwood (`:401`), Abyssal Depths (`:447`) and Voidfarer Horizon (`:425`). One later card.

Draeks: a secret exists and is found through play.

## 7. Economy and transport

**Owner:** `client/src/rbe_allocate_choice.rs:206` `RbeAllocateChoicePlugin`. Manufacture: `shared/fabricator.rs:22` and `client/src/fabricator.rs:66`. Haul: `shared/hex_travel.rs:568` `deliver_haul`. `client/src/wood_expedition.rs:587` `WoodExpeditionPlugin`. `client/src/vertical_factory.rs:63` `VerticalFactoryPlugin`. Week score: `shared/week_audit.rs:3`, struct `:10`. `client/src/mercy_transporters.rs:60` `MercyTransportersPlugin` (`toggle_transporters` `:76`) is a local toggle.

**Status:** partial.

Local only: no network calls, no server, no data off the device, no live AI. Server stays planned, and online stays grey.

**Overlap:** `shared/contribution_ledger.rs:64` `ContributionLedger` (named in the client test at `client/src/lib.rs:996`) sits beside the week audit. One later card.

## 8. Morality and consequence ledger

**Owner:** none is canonical. The canonical owner is planned: chosen by the §8 migration card's PRE-PATH and ruling.

**Status:** partial. The four stores today:

1. `ledger_bind` — `shared/ledger_bind.rs:1`, client `client/src/ledger_bind.rs:78` `LedgerBindPlugin`.
2. `contribution_ledger` — `shared/contribution_ledger.rs:64`.
3. `run_ledger` — `client/src/run_ledger.rs:1`, plugin `:74`.
4. `week_audit` — `shared/week_audit.rs:1`, struct `:10`.

Also in this section today: local faction standing `shared/shard_standing.rs:1` (`ShardStanding` `:11`) and `shared/crownstone.rs:31` `CrownstoneState`. The morality design doc is planned, and online stays grey (`docs/design/SERVER_MORALITY.md`).

**Overlap:** the four ledgers above. Consequence gaps are ranked at `docs/LOOP_MAP.md:65`. One later card.

Draeks: a secret exists and is found through play.

## 9. Input and UI (one action map; the merged pad routes become its table)

**Owner:** `client/src/input.rs:29` `PlayerInput`, `:75` `InputPlugin`. Also `client/src/touch_controls.rs:46` `TouchControlsPlugin`, the key consts in `client/src/soft_play_bindings.rs:12` through `:46`, HUD `client/src/hud_anchor_registry.rs:19` `HudSlab` and `client/src/hud_presets.rs:198` `HudPresetId`. Canon doc: `docs/INPUT_CANON.md`.

**Status:** built, not unified. There is no single action map yet. The merged pad routes become this table when one later card builds it.

**Overlap:** raw `GamepadButton::` and `KeyCode` reads. At `6ba72cf2`, `GamepadButton::` occurs 344 times in 19 client files:

| File | Count |
|---|---|
| `client/src/title_screen.rs` | 77 |
| `client/src/input.rs` | 53 |
| `client/src/hex_travel.rs` | 35 |
| `client/src/mercy_harvest_nodes.rs` | 28 |
| `client/src/human_inventory.rs` | 26 |
| `client/src/rbe_allocate_choice.rs` | 22 |
| `client/src/embassy.rs` | 20 |
| `client/src/hour_sacred.rs` | 16 |
| `client/src/launch_menu.rs` | 14 |
| `client/src/licences_screen.rs` | 13 |
| `client/src/age_gate.rs` | 12 |
| `client/src/ledger_bind.rs` | 11 |
| `client/src/ui_frame.rs` | 7 |
| `client/src/hud_edit_mode.rs` | 3 |
| `client/src/heartwood_plaque.rs` | 3 |
| `client/src/thriving_moments.rs` | 1 |
| `client/src/soft_play_bindings.rs` | 1 |
| `client/src/first_session_guidance.rs` | 1 |
| `client/src/first_harvest_epiphany.rs` | 1 |

`client/src/soft_play_bindings.rs:104` and `:108` take `ButtonInput<KeyCode>`. One later card.

## 10. Save and telemetry hooks

**Owner:** none is canonical. The canonical save is planned: chosen by the §10 migration card, which carries the load-old-save fixture.

**Status:** built, fragmented. The local writer for `powrush_*` JSON is `shared/user_persist.rs:3`. Session blob: `client/src/local_session_persist.rs:25`, `SessionBlob` `:28`, plugin `:147`.

An existing save must load after any schema change. That load-old-save rule is a requirement on future migration cards. It is not built. The §10 card carries the fixture.

Local only: no network calls, no server, no data off the device, no live AI. Server stays planned, and online stays grey. Bot-soak hooks stay on the device. Lived-tick ingest is off by default (`shared/lived_tick_ingest.rs:3`, path `:20`).

`client/src/steam_abundance_mirror.rs:25` writes local stage directories only (`:57` is the fallback directory). The resource flag at `:76` starts enabled for those local directories. Steamworks stays optional and off by default (optional-off) (`client/Cargo.toml:14`, `:52`).

**Overlap:** 23 distinct persist filenames, listed as they are today. 21 are json saves and 2 are jsonl logs. None is canonical. One later card. `shared/f_book_fixture.rs:34` through `:38` copies five of these names into a fixture tree. It is not another schema. `run_ledger` and skill XP ride `powrush_local_session.json` (`client/src/run_ledger.rs:2`, `client/src/skill_xp.rs:2`).

| # | File | Declared at |
|---|---|---|
| 1 | `data/powrush_local_session.json` | `client/src/local_session_persist.rs:25` |
| 2 | `powrush_finds.json` | `client/src/loot_drops.rs:37` |
| 3 | `powrush_hex_current.json` | `shared/hex_travel.rs:40` |
| 4 | `powrush_hex_sanctuary.json` | `shared/hex_travel.rs:195` (`:745`) |
| 5 | `powrush_hex_heartwood.json` | `shared/hex_travel.rs:195` (`:746`) |
| 6 | `powrush_hex_depths.json` | `shared/hex_travel.rs:195` (`:861`) |
| 7 | `data/powrush_hour_two.json` | `client/src/hour_sacred.rs:115` |
| 8 | `data/powrush_house.json` | `shared/house_name.rs:21` |
| 9 | `data/powrush_hud_layout.json` | `shared/hud_layout.rs:19` |
| 10 | `data/powrush_settings.json` | `shared/local_settings.rs:19` |
| 11 | `data/powrush_persona.json` | `shared/persona.rs:292` |
| 12 | `data/powrush_shard_climate.json` | `client/src/lived_hour_bind.rs:34` |
| 13 | `data/powrush_shard_standing.json` | `client/src/lived_hour_bind.rs:35` |
| 14 | `data/powrush_week_audit.json` | `client/src/lived_hour_bind.rs:36` |
| 15 | `data/powrush_lived_tick.json` | `client/src/lived_hour_bind.rs:33` |
| 16 | `data/powrush_lived_events.jsonl` | `client/src/lived_sim_bridge.rs:20` |
| 17 | `data/powrush_lattice_flow_share.json` | `client/src/lattice_flow_share.rs:26` |
| 18 | `data/powrush_lattice_flow_share_peer.json` | `client/src/lattice_flow_share.rs:27` |
| 19 | `data/powrush_abundance_journey.json` | `client/src/abundance_journey_echo.rs:26` |
| 20 | `powrush_wellsworn.json` | `client/src/wellsworn.rs:22` |
| 21 | `powrush_wood_expedition.json` | `client/src/wood_expedition.rs:20` |
| 22 | `powrush_signpost.json` | `client/src/first_session_guidance.rs:862` |
| 23 | `data/powrush_genshare.jsonl` | `shared/genshare.rs:23` |

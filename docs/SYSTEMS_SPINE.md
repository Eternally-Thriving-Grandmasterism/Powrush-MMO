# Systems spine

SYSTEMS-SPINE-1. Docs only. This file is new. No code, data, crate or format change.

The PATSAGi Councils adopted the rulings this page records, under Sherif's typed delegation t1767u, t1948u and t2247u. Sherif may veto or amend. The ten sections are the queue ruling. Owner lines in §1–§10 were first read at main `6ba72cf2`. §11 and §18 name their own re-read tips, and later cards re-pin only the lines they touch.

Enforcement starts once this file is on main. From then on, each new card's PRE-PATH cites the spine section it plugs into. Cards already ruled are not held back: COMBAT-FAIR-1, STRIKE-HINT-1, GROUND-SNAP-1, GROUND-SNAP-2, GROUND-SNAP-3, GROUND-SNAP-4 and LOD-LIMBS-1.

Each overlap below is one later migration card. No big-bang rewrite. That card proves no behaviour change unless it names one.

Title Online stays grey, and server work stays planned (`shared/net_mode.rs:5`, `Offline` `:15`, `:18`, `:27`; `client/src/net_mode.rs:1`, `:3`). The `steam` feature is optional and off by default (optional-off) (`client/Cargo.toml:13`, `:14`, `:52`).

## 1. Entities

**Owner:** `client/src/entity_policy.rs:2` (ENTITY-CONTRACT), `#![cfg(test)]` at `:1`, declared at `client/src/lib.rs:108`. The contract table starts at `client/src/entity_policy.rs:37`.

**Status:** partial. A test-side contract exists. The runtime entity registry is planned (ENTITY-PURPOSE-1).

**Overlap:** spawn positions sit in per-module consts, with no single table. One later card. Sites today: `client/src/living_ecology.rs:62` `ResonantDeer`, `:211` `YARD_TREE_SPOTS`, `:225` `YARD_ROCK_SPOTS`, `:229` `YARD_CRYSTAL_SPOTS`; `client/src/combat_core.rs:105` `Predator`; `client/src/droid.rs:33` `Droid`; `client/src/grazer.rs:23` `Grazer`, `:50` `GrazerPlugin`; `client/src/local_human_sim.rs:46` `LocalHumanSim`; `client/src/companion_bond.rs:43` `CompanionBond`, `:59` `CompanionBondPlugin`; `client/src/collide.rs:129` `CreatureBodyPlugin`.

Draeks: a secret exists and is found through play.

## 2. Combat (damage, telegraph, block, recovery, targeting and soft-aim)

COMBAT-FAIR-1's numbers are this section's first rows. That card is already ruled and is not held back.

**Owner:** `client/src/combat_core.rs`. `CombatState` `:72`. Damage: `land_hit` `:332`, `land_hit_paid` `:356`, `combat_fire` `:552`. Telegraph: `tell_visible` `:125` (0.35 s at `:107`). Block: `plate_arm` `:98`. Recovery: `respawn_step` `:396`. Targeting: `in_reach` `:287`, `facing_ok` `:293`. `Predator` `:105`. `PREDATOR_RESPAWN_SECS` `:68`. Also `client/src/difficulty.rs:39` `enemy_hit`, `client/src/droid.rs:1`, and `client/src/combat_text.rs:221` `CombatTextPlugin`.

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

**Status:** partial. The world bible is on main as `docs/WORLD_BIBLE.md` (WORLD-BIBLE-1, #1171, docs only). WORLD-DIRECTOR L0–L3 budgets are planned (no file and no function in this tree).

**Overlap:** Places travel (`PlaceId`) versus the Z/F3 realm switch. Z is `client/src/soft_play_bindings.rs:27`, F3 is `:46`, and the switch is `client/src/human_soft_panels.rs:168` (travel note `:200`). The two realm-name tables match since REALM-NAMES-1 (#1174). `client/src/human_soft_panels.rs:34-38` names Sanctuary Prime, Crystal Spires, Verdant Heartwood, Abyssal Depths, Voidfarer Horizon, the same as `look_for`: Sanctuary Prime (`client/src/climate_plane.rs:466`), Crystal Spires (`:423`), Verdant Heartwood (`:401`), Abyssal Depths (`:447`) and Voidfarer Horizon (`:425`). Test `realm_names_1_panel_names_match_look_for` (`client/src/climate_plane.rs:2271`) pins them equal.

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

Also in this section today: local faction standing `shared/shard_standing.rs:1` (`ShardStanding` `:11`) and `shared/crownstone.rs:31` `CrownstoneState`. Server morality is proposed, not built, and online stays grey (`docs/design/SERVER_MORALITY.md:9`).

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
| 11 | `data/powrush_persona.json` | `shared/persona.rs:290` |
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

## 11. FOUNDATIONS

One block for each system in §1 through §10. The seven fields follow queue ruling 7118722588. Known-today lines in this section were re-read at main `44dca531`. A missing fact says planned or **GAP**.

A perf figure here is a target, not a measurement (§13.1). Multiplayer is planned only. This page names no server and no account.

Shared work that is already written stays planned: the COMPANY-1 layer (`docs/DESIGN_PILLARS.md:44`), NPC rival companies (`docs/DESIGN_PILLARS.md:56`), BOT-SOAK-1 (`docs/DESIGN_PILLARS.md:66`), and the EA minimum of one refine leg plus one haul leg (`docs/EA_BAR.md:47`). WORLD-BIBLE-1 (#1171) and WORLD-GEN-1 (#1173) are on main, docs only. WORLD-TRAVEL-1 is planned.

Dev proof is `cargo test -p shared -p rsil-identity` and `cargo test -p powrush-client --lib` (`.github/workflows/ci.yml:141`, `.github/workflows/ci.yml:144`). The seven required checks stay: validate (`.github/workflows/validate-full-files.yml:8`), PATSAGi merge court (`.github/workflows/patsagi-auto-merge.yml:20`), Core (`.github/workflows/ci.yml:18`), Core + Q2 one-frame (`.github/workflows/agent-headless-qa.yml:25`), offline-build on ubuntu-24.04 and on windows-latest (`.github/workflows/offline-build.yml:17`, matrix `.github/workflows/offline-build.yml:27`), and clean-boot (`.github/workflows/offline-build.yml:221`). Release proof is that offline-build, that clean-boot, and Sherif's real-PC test named at `docs/DESIGN_PILLARS.md:72`. There is no perf step in CI. Q2 does not compare pixels (`.github/workflows/agent-headless-qa.yml:6`).

The Draek sentences already in §1, §6 and §8 stay as they are. This section adds none.

### 11.1 Entities

**Contract:** A mesh in the world has a policy row, and a spawn for that place sits on the yard.

**Known today:** Test-side ENTITY-CONTRACT at `client/src/entity_policy.rs:2`, `#![cfg(test)]` at `client/src/entity_policy.rs:1`, declared at `client/src/lib.rs:108`. The table starts at `client/src/entity_policy.rs:37`. Tests `client/src/entity_policy.rs:287` and `client/src/entity_policy.rs:289` (7 `#[test]` fns). The runtime registry is planned (ENTITY-PURPOSE-1). Spawn sites are the consts named in §1.

**Expected:** ENTITY-PURPOSE-1 is the planned runtime registry. Multiplayer is planned only.

**Failure modes and recovery cost:** A spawn const that misses the contract table is a medium recovery. §1 already records those sites as per-module consts, and this page does not build the registry.

**Tests:** The two tests above, under the dev command in this section. Release proof is the release list in this section.

**Performance budget:** Target, not a measurement. See §13.1.

**Save and version migration:** No §10 row is an entity registry. An existing save must load (§10). The load-old-save fixture is a requirement on the migration card. It is not built.

### 11.2 Combat (damage, telegraph, block, recovery, targeting and soft-aim)

**Contract:** One predator and one droid, PvE only, with a telegraph, a block, a recovery and a facing check. Soft-aim is planned.

**Known today:** Re-read at `44dca531`: `CombatState` `client/src/combat_core.rs:72`, `land_hit` `client/src/combat_core.rs:332`, `land_hit_paid` `client/src/combat_core.rs:356`, `combat_fire` `client/src/combat_core.rs:552`, `tell_visible` `client/src/combat_core.rs:125`, the telegraph comment `client/src/combat_core.rs:107`, `plate_arm` `client/src/combat_core.rs:98`, `respawn_step` `client/src/combat_core.rs:396`, `in_reach` `client/src/combat_core.rs:287`, `facing_ok` `client/src/combat_core.rs:293`, `Predator` `client/src/combat_core.rs:105`, `PREDATOR_RESPAWN_SECS` `client/src/combat_core.rs:68`. PvE only `client/src/combat_core.rs:3`. The basic-strike comment is `client/src/combat_core.rs:48`. Droid module `client/src/droid.rs:1`, reach assert `client/src/droid.rs:248`. Example test `client/src/combat_core.rs:1030` (165 `#[test]` fns in that file). No soft-aim function in `client/src`. COMBAT-FAIR-1 landed at `44dca531`. The §2 anchors on L19 and L27 were refreshed to that tip.

**Expected:** COMBAT-FAIR-1 has landed at `44dca531`. Soft-aim stays planned. Multiplayer is planned only.

**Failure modes and recovery cost:** The droid reach and shot path sits beside `combat_core` (§2 Overlap). Two paths that disagree are a medium recovery: one later card, and this page does not merge them.

**Tests:** `client/src/combat_core.rs:1030` and `client/src/droid.rs:248`, under the dev command in this section. Release proof is the release list in this section.

**Performance budget:** Target, not a measurement. See §13.1.

**Save and version migration:** Combat XP rides the session blob (§10 row 1, `client/src/skill_xp.rs:2`). An existing save must load (§10). The load-old-save fixture is a requirement on the migration card. It is not built.

### 11.3 Stats, items, gear and recipes

**Contract:** Named finds, a satchel, and the recipe enum. An owned find that is equipped adds its standing step to one outgoing strike.

**Known today:** `shared/temper.rs:1`, `WardKind` `shared/temper.rs:26`. `FoundFinds` `client/src/loot_drops.rs:45`, file `client/src/loot_drops.rs:37`, schema `client/src/loot_drops.rs:107`. Six named finds `shared/temper.rs:236`. An equipped owned find's standing step reaches combat in `land_hit_paid` (`client/src/combat_core.rs:356`). Tested at `client/src/combat_core.rs:2303`. The Shell ward is `client/src/combat_core.rs:96`, tested at `client/src/combat_core.rs:2592`. Satchel `client/src/human_inventory.rs:99`. Recipes `shared/fabricator.rs:22` through `shared/fabricator.rs:26`, client plugin `client/src/fabricator.rs:66`, same row `docs/EA_BAR.md:28`. Example test `client/src/loot_drops.rs:148` (14 `#[test]` fns).

**Expected:** Finds and the satchel stay separate stores until one later card (§3 Overlap). That card is not named here. Multiplayer is planned only.

**Failure modes and recovery cost:** The two stores diverging is a medium recovery. §3 names one later card, and this page does not merge them.

**Tests:** `client/src/loot_drops.rs:148`, `client/src/loot_drops.rs:304` and `client/src/loot_drops.rs:328`, under the dev command in this section. Release proof is the release list in this section.

**Performance budget:** Target, not a measurement. See §13.1.

**Save and version migration:** §10 row 2. An existing save must load (§10). The load-old-save fixture is a requirement on the migration card. It is not built. See §14.

### 11.4 Progression and the unlock ladder

**Contract:** Four tracks are already in code: skill XP, Temper, Lumen and Ward, an area difficulty gate, and the hour and book gate. Temper, Lumen and Ward stay the one power ladder.

**Known today:** `client/src/skill_xp.rs:1`, `SkillXp` `client/src/skill_xp.rs:31`, `SkillXpPlugin` `client/src/skill_xp.rs:149`, `client/src/skill_xp.rs:243` (16 `#[test]` fns). `shared/temper.rs:1`. `client/src/difficulty.rs:1`, `enemy_hit` `client/src/difficulty.rs:39`, `DifficultyPlugin` `client/src/difficulty.rs:216`, test `client/src/difficulty.rs:228` (17 `#[test]` fns). `HourSacred` `client/src/hour_sacred.rs:796`, `hour_three_complete` `client/src/hour_sacred.rs:799`. Book check `client/src/hex_travel.rs:258`, `book_flags` `client/src/hex_travel.rs:536`. XP from verbs, and kills never pay, `docs/design/LEVELS_CANON.md:15`. The tracks stay unmapped at `docs/design/LEVELS_CANON.md:17` and `docs/design/LEVELS_CANON.md:85`.

**Expected:** UNLOCK-LADDER-1 is planned. §4 records that it has no doc in this tree.

**Failure modes and recovery cost:** Four unmapped tracks are a medium recovery. `docs/design/LEVELS_CANON.md:85` says skill levels do not feed the power ladder until a card maps them.

**Tests:** `client/src/skill_xp.rs:243` and `client/src/difficulty.rs:228`, under the dev command in this section. Release proof is the release list in this section.

**Performance budget:** Target, not a measurement. See §13.1.

**Save and version migration:** Skill XP rides §10 row 1 (`client/src/skill_xp.rs:2`). Hour two is §10 row 7. An existing save must load (§10). The load-old-save fixture is a requirement on the migration card. It is not built.

### 11.5 Quests, journal and captions

**Contract:** There is no quest log. Signposts, a journey feed, and one quest const exist.

**Known today:** No quest log, `docs/LOOP_MAP.md:155`. `FirstSessionGuidancePlugin` `client/src/first_session_guidance.rs:794`, test `client/src/first_session_guidance.rs:1438` (110 `#[test]` fns). `AbundanceJourneyEchoPlugin` `client/src/abundance_journey_echo.rs:146`, key `client/src/soft_play_bindings.rs:21`. `QUEST_TEXT` is the const at `client/src/wellsworn.rs:21`. This page does not quote it. Shape tokens sit at `client/src/local_settings.rs:8`.

**Expected:** QUEST-DATA-1 is planned. Multiplayer is planned only.

**Failure modes and recovery cost:** Three surfaces write player-facing progress lines (§5 Overlap). Lines that disagree are a medium recovery: one later card, and this page writes no new line.

**Tests:** `client/src/first_session_guidance.rs:1438`, under the dev command in this section. Release proof is the release list in this section.

**Performance budget:** Target, not a measurement. See §13.1.

**Save and version migration:** §10 rows 19, 20 and 22. An existing save must load (§10). The load-old-save fixture is a requirement on the migration card. It is not built.

### 11.6 World (bible, gen seeds, zones, travel, WORLD-DIRECTOR L0–L3 budgets)

**Contract:** Three PlaceIds, travel between them, and dress per place. The world bible is on main (§6). The WORLD-DIRECTOR budgets and distinct traveled places are planned.

**Known today:** `PlaceId` `shared/hex_travel.rs:52`, `LOCAL_HEXES` `shared/hex_travel.rs:61`, `apply_travel_named` `shared/hex_travel.rs:541`, test `shared/hex_travel.rs:635` (22 `#[test]` fns). Client `apply_place` `client/src/hex_travel.rs:251`, `places_plate_clicks` `client/src/hex_travel.rs:625` (77 `#[test]` fns in that file). `look_for` `client/src/climate_plane.rs:393`, `sync_place_dress_from_travel` `client/src/climate_plane.rs:1661`, `apply_climate_look` `client/src/climate_plane.rs:1854`. Look-name pins `client/src/climate_plane.rs:2612`. Rooms note `docs/PLACES_BIBLE.md:61`. Zones row `docs/DESIGN_PILLARS.md:37`. Grove seed `shared/powrush_gen.rs:95`. The two realm-name tables match (§6). Those names are owner-ruled (#738 7118691808). REALM-NAMES-1 shipped as #1174.

**Expected:** WORLD-BIBLE-1 (#1171, `docs/WORLD_BIBLE.md`) and WORLD-GEN-1 (#1173, `docs/WORLD_GEN.md`) are on main, docs only. WORLD-TRAVEL-1 is planned. REALM-NAMES-1 is on main (#1174). WORLD-DIRECTOR L0–L3 budgets stay planned, as §6 says: no file and no function in this tree. Multiplayer is planned only.

**Failure modes and recovery cost:** Realm-name drift between the two tables (§6) fails `realm_names_1_panel_names_match_look_for` (`client/src/climate_plane.rs:2271`). Missing WORLD-DIRECTOR budgets are a medium recovery: the card is planned, not built.

**Tests:** `shared/hex_travel.rs:635` and `client/src/hex_travel.rs:625`, under the dev command in this section. Release proof is the release list in this section.

**Performance budget:** Target, not a measurement. See §13.1.

**Save and version migration:** §10 rows 3, 4, 5, 6, 12 and 23. An existing save must load (§10). The load-old-save fixture is a requirement on the migration card. It is not built. See §14.

### 11.7 Economy and transport

**Contract:** The local week score is tons plus restored. Manufacture and haul exist. No network calls, and no data off the device.

**Known today:** `shared/week_audit.rs:3`, struct `shared/week_audit.rs:10`, tests `shared/week_audit.rs:63` and `shared/week_audit.rs:80` (2 `#[test]` fns). EA minimum `docs/EA_BAR.md:47`. Manufacture `shared/fabricator.rs:22` and `client/src/fabricator.rs:66`. Haul `shared/hex_travel.rs:568`. The plugins are the ones named in §7. COMPANY-1 `docs/DESIGN_PILLARS.md:44`. NPC rival companies `docs/DESIGN_PILLARS.md:56`. BOT-SOAK `docs/DESIGN_PILLARS.md:66`.

**Expected:** COMPANY-1, NPC rival companies and BOT-SOAK-1 are planned. Multiplayer is planned only. No server or account design.

**Failure modes and recovery cost:** A network call, or data leaving the device, breaks the local-only rule in §7. That is a high recovery: server work stays planned, and this page adds no endpoint.

**Tests:** `shared/week_audit.rs:63` and `shared/week_audit.rs:80`, under the dev command in this section. Release proof is the release list in this section.

**Performance budget:** Target, not a measurement. See §13.1.

**Save and version migration:** §10 rows 14, 17, 18 and 21. An existing save must load (§10). The load-old-save fixture is a requirement on the migration card. It is not built.

### 11.8 Morality and consequence ledger

**Contract:** No ledger is canonical. Four stores exist. Server morality is proposed, not built.

**Known today:** `shared/ledger_bind.rs:1`, test `shared/ledger_bind.rs:633` (37 `#[test]` fns), client plugin `client/src/ledger_bind.rs:78`. `ContributionLedger` `shared/contribution_ledger.rs:64`. `client/src/run_ledger.rs:1`, plugin `client/src/run_ledger.rs:74`. `shared/week_audit.rs:1`. `shared/shard_standing.rs:1`, `ShardStanding` `shared/shard_standing.rs:11`. `CrownstoneState` `shared/crownstone.rs:31`. Server morality `docs/design/SERVER_MORALITY.md:9`. Consequence gaps `docs/LOOP_MAP.md:65`.

**Expected:** The canonical owner is chosen by the §8 migration card's PRE-PATH. Server morality stays proposed, not built. Online stays grey. Multiplayer is planned only.

**Failure modes and recovery cost:** Four stores and no canonical owner (§8) are a medium recovery: that migration card, not this page. Citing the morality doc as built would be a high mis-cite. `docs/design/SERVER_MORALITY.md:9` says proposed, not built.

**Tests:** `shared/ledger_bind.rs:633`, under the dev command in this section. Release proof is the release list in this section.

**Performance budget:** Target, not a measurement. See §13.1.

**Save and version migration:** `run_ledger` rides §10 row 1 (`client/src/run_ledger.rs:2`). Standing is row 13. Week audit is row 14. An existing save must load (§10). The load-old-save fixture is a requirement on the migration card. It is not built.

### 11.9 Input and UI (one action map; the merged pad routes become its table)

**Contract:** One Use verb is in code. There is no single action map yet.

**Known today:** `PlayerInput` `client/src/input.rs:29`, `InputPlugin` `client/src/input.rs:75`, tests `client/src/input.rs:744` and `client/src/input.rs:753` (31 `#[test]` fns). `TouchControlsPlugin` `client/src/touch_controls.rs:46`. Key consts `client/src/soft_play_bindings.rs:12` through `client/src/soft_play_bindings.rs:46`. `HudSlab` `client/src/hud_anchor_registry.rs:19`. `HudPresetId` `client/src/hud_presets.rs:198`. Canon `docs/INPUT_CANON.md:1`. Fixed sim rate `client/src/feel_move.rs:15`, inserted at `client/src/feel_move.rs:35`. `GamepadButton::` is still 53 in `client/src/input.rs` and 344 under `client/src`, the same totals as the §9 table. This page quotes no button hint.

**Expected:** The merged pad routes become this table when one later card builds it (§9). That card is not named here. HUD-IA-1 copy is Sherif's (§15.2). Multiplayer is planned only.

**Failure modes and recovery cost:** Raw `GamepadButton::` and `KeyCode` reads sit outside one map (§9). That is a medium recovery: one later card.

**Tests:** `client/src/input.rs:744` and `client/src/input.rs:753`, under the dev command in this section. Release proof is the release list in this section.

**Performance budget:** Target, not a measurement. See §13.1.

**Save and version migration:** §10 rows 9 and 10. An existing save must load (§10). The load-old-save fixture is a requirement on the migration card. It is not built.

### 11.10 Save and telemetry hooks

**Contract:** `powrush_*` JSON writes go through the user-dir writer. An existing save must load. Telemetry stays on the device. No save is canonical.

**Known today:** User-dir rule `shared/user_persist.rs:1`, including no Steamworks account or cloud API at `shared/user_persist.rs:20`. `write_named` `shared/user_persist.rs:94`, the `.bak` steps `shared/user_persist.rs:91`, read fallback `shared/user_persist.rs:295`, and `replace_synced` (`shared/user_persist.rs:100`) with no `.bak` rule (`shared/user_persist.rs:99`). Tests `shared/user_persist.rs:687` and `shared/user_persist.rs:711` (27 `#[test]` fns). Session path `client/src/local_session_persist.rs:25`, `SessionBlob` `client/src/local_session_persist.rs:28`, schema field `client/src/local_session_persist.rs:29`, plugin `client/src/local_session_persist.rs:147`, load check `client/src/local_session_persist.rs:176`, tests `client/src/local_session_persist.rs:274` and `client/src/local_session_persist.rs:280` (17 `#[test]` fns). Lived-tick ingest is off by default (`shared/lived_tick_ingest.rs:3`, path `shared/lived_tick_ingest.rs:20`). Crash log name `client/src/main.rs:19`, hook `client/src/main.rs:80`, tests `client/src/main.rs:193`. Perf log opt-in `client/src/frame_pace.rs:14`.

**Expected:** SAVE-SCHEMA-1 plans one version style and a migration, with a load-old-save fixture (§14). CRASH-REPORT-1 is planned and local only: no upload, and no data off the device. Multiplayer is planned only.

**Failure modes and recovery cost:** A torn write that drops the last good bytes is a high recovery. The built recovery is the `.bak` read (`shared/user_persist.rs:295`), tested at `shared/user_persist.rs:687` and `shared/user_persist.rs:711`. A breaking schema change with no fixture is a high recovery: §10 requires that fixture, and it is not built.

**Tests:** The tests named above, under the dev command in this section. Release proof is the release list in this section.

**Performance budget:** Target, not a measurement. See §13.1.

**Save and version migration:** All 23 rows in §10. None is canonical. The marker styles are §14. The migration chain is planned (SAVE-SCHEMA-1).

## 12. NET-READY

Design only. Multiplayer is planned. There is no server, no account, and no networking commitment. Online stays grey. LoopbackDev is a 127.0.0.1 lab permit and never a public bind.

Built today, and not a per-system replication design:

- `shared/net_mode.rs:12` `NetMode`: Offline default `shared/net_mode.rs:15`, LoopbackDev `shared/net_mode.rs:17`, Online `shared/net_mode.rs:19`. Title Online stays grey `shared/net_mode.rs:5`. Tests `shared/net_mode.rs:59` and `shared/net_mode.rs:68`.
- `client/src/net_mode.rs:3` through `client/src/net_mode.rs:7`: outbound WS only by intent, 127.0.0.1 only. Plugin `client/src/net_mode.rs:93`, registered `client/src/lib.rs:254`. Tests `client/src/net_mode.rs:172` and `client/src/net_mode.rs:229`.
- `shared/hex_join.rs:18` `AuthorityMode`: Offline `shared/hex_join.rs:20`, Online `shared/hex_join.rs:22`. `shared/hex_join.rs:3` says no listen socket and no WS. `NetMode` and `AuthorityMode` both say offline versus online. This page adds no third mode.
- `shared/hex_protocol.rs:4` says no listen socket and no WS client. `shared/hex_listen.rs:1` is pure loopback bind rules, and `shared/hex_listen.rs:19` is `127.0.0.1:7788`. The only `TcpListener` hit under `client/src` and `shared` is the negative assert at `client/src/hud_edit_mode.rs:1979`.
- Fixed sim rate `client/src/feel_move.rs:15`, inserted at `client/src/feel_move.rs:35`. Grove seed `shared/powrush_gen.rs:95`.
- `docs/REPLICATION_PREDICTION_ARCHITECTURE.md:3` is an older v17.92 design. It is not a built system.

Each later card's PRE-PATH fills its own row. The columns that card fills are authority owner, state that would replicate, serializable, and deterministic input. Until then the row says planned. The built authority for every row today is the client, offline, as the bullets above.

| § | System | NET-READY row |
|---|---|---|
| 1 | Entities | planned |
| 2 | Combat | planned |
| 3 | Items, gear and recipes | planned |
| 4 | Progression | planned |
| 5 | Quests and journal | planned |
| 6 | World | planned |
| 7 | Economy and transport | planned |
| 8 | Morality ledger | planned |
| 9 | Input and UI | planned |
| 10 | Save and telemetry | planned |

Owner of the built rules: `shared/net_mode.rs` and `shared/hex_join.rs`, with the client resource in `client/src/net_mode.rs`.

## 13. PERF

Owner: `client/src/frame_pace.rs`. `GraphicsPreset` (`shared/local_settings.rs:164`) is the quality tier. It stays in this one contract. There is no second perf module.

**§13.1 Target.** Target, not a measurement: 1080p/60 on a mid-range PC first, then Steam Deck. The public record is #738 comment 6517222964 (GFX-PACE-1: the council's 1080p/60-first target; smooth, even frames at 60 on a mid-range PC; Steam Deck tuning comes after 1080p/60) and scope vote 6517254928, which banks the real 1080p/60 frame-time read on Sherif's PC and the playtester's PC. That read is not in this file. No hardware claim is made.

**§13.2 Hitch log.** Built: hitch counts over `FRAME_BUDGET_SECS` (`client/src/frame_pace.rs:26`) and `HITCH_SECS` (`client/src/frame_pace.rs:29`), logged once per 1 s roll (`client/src/frame_pace.rs:1`). Tests `client/src/frame_pace.rs:529` and `client/src/frame_pace.rs:539` (22 `#[test]` fns). There is no CI number.

**§13.3 Present mode.** `PresentMode::Fifo` at `client/src/main.rs:96`. The plugin is registered at `client/src/main.rs:113`.

**§13.4 Release spacing.** Focused spacing is `client/src/frame_pace.rs:32`. Inactive or unfocused spacing is `client/src/frame_pace.rs:36`. The waiter is `release_waiter` at `client/src/frame_pace.rs:201`. `client/src/frame_pace.rs:2` and `client/src/frame_pace.rs:3` say that waiter is release spacing only: not a frame, present, or render cap, and not a safety, thermal, or leak claim.

**§13.5 PERF-GATE-1.** Planned: a seeded bench walk, a committed baseline, a warn-only CI step, and frame-rate and background-running limits. It does not become a required check. The seven required checks in §11 stay as they are. Nothing is added as required without a separate ruling. `benches/harvest_flow_bench.rs` has no `[[bench]]` entry in any `Cargo.toml`.

**§13.6 Baseline.** A later perf number is a delta against our own committed baseline. Real-GPU numbers come only from Sherif's PC and the playtester's PC, the read banked in 6517254928. That read is not in this file.

**§13.7 Opt-in log.** `POWRUSH_PERF_LOG` turns the log on (`client/src/frame_pace.rs:14`). The line is appended at `data/perf.log` (`client/src/frame_pace.rs:20`). The call site is `client/src/main.rs:25`. It stays off unless that value is set (`client/src/frame_pace.rs:15`).

**§13.8 Image diff.** VIS-GUARD-1's warn-only image diff is planned. It is not built. Q2 does not compare pixels (`.github/workflows/agent-headless-qa.yml:6`).

## 14. SAVE

This section summarizes §10. It does not replace the 23 rows. No canonical save is named. SAVE-SCHEMA-1 is the enforcing card: a save version, a migration, and a load-old-save fixture, citing the §10 rule. This page picks no style.

**§14.1 Rule.** An existing save must load after any schema change (§10). Where a file already grows by fields, the pattern is `#[serde(default)]` (`client/src/local_session_persist.rs:38`) or a lenient field (`client/src/local_session_persist.rs:59`, reader `client/src/run_ledger.rs:26`). A breaking change needs a version stamp, a forward migration, and the fixture. The fixture is not built.

**§14.2 Writer.** `shared/user_persist.rs:94` `write_named` keeps a good live file as `.bak` and stages new bytes through `.tmp` (`shared/user_persist.rs:91`). A read falls back to `.bak` (`shared/user_persist.rs:295`). Tests `shared/user_persist.rs:687` and `shared/user_persist.rs:711`. `replace_synced` (`shared/user_persist.rs:100`) has no `.bak` rule (`shared/user_persist.rs:99`). The user-dir rule is `shared/user_persist.rs:9`, and `shared/user_persist.rs:20` says no Steamworks account or cloud API.

**§14.3 Styles.** The markers that exist on the §10 files are a schema string, a `v: u16`, and a `schema: u32`. Counted at `44dca531`, 11 of the 23 files have no version marker on their default bytes.

Schema string:

- Row 1, field `client/src/local_session_persist.rs:29`, written `client/src/local_session_persist.rs:65`, checked `client/src/local_session_persist.rs:176`.
- Row 2, `client/src/loot_drops.rs:107`.
- Row 8, `shared/house_name.rs:22`, field `shared/house_name.rs:47`.
- Row 9, `shared/hud_layout.rs:22`, field `shared/hud_layout.rs:59`.
- Row 10, `shared/local_settings.rs:20`, field `shared/local_settings.rs:324`.
- Row 11, `shared/persona.rs:293`, field `shared/persona.rs:844`.
- Row 16, `client/src/lived_sim_bridge.rs:99`.
- Rows 17 and 18, field `client/src/lattice_flow_share.rs:46`, written `client/src/lattice_flow_share.rs:135`. The peer file is read at `client/src/lattice_flow_share.rs:178` and the prefix check is `client/src/lattice_flow_share.rs:164`.
- Row 19, field `client/src/abundance_journey_echo.rs:45`, written `client/src/abundance_journey_echo.rs:86`.

`v: u16`: row 23, `shared/genshare.rs:26`, field `shared/genshare.rs:31`.

`schema: u32`: row 21, `client/src/wood_expedition.rs:21`, field `client/src/wood_expedition.rs:39`.

No version field on the default bytes of rows 3, 4, 5 and 6 (`shared/hex_travel.rs:130` `HexClimateFile`, `shared/hex_travel.rs:165` `CurrentHex`), row 7 (`shared/hour_two.rs:18`), rows 12, 13 and 14 (`shared/shard_climate.rs:14`, `shared/shard_standing.rs:11`, `shared/week_audit.rs:10`), row 15's default blob (`shared/climate_node.rs:217`, written `client/src/lived_hour_bind.rs:231`), row 20 (`client/src/wellsworn.rs:80`), and row 22 (`client/src/first_session_guidance.rs:898`). Row 15's ingest overlay is off by default (`shared/lived_tick_ingest.rs:3`) and, when on, writes a schema string (`shared/lived_tick_ingest.rs:22`) onto that same path.

**§14.4 Chain.** There is no migration chain in `client/src` or `shared`. The hit at `client/src/_parked/ui_utils.rs:7` is a comment about a label, not a save migration. Old-save tests that exist today are `client/src/local_session_persist.rs:274`, `client/src/local_session_persist.rs:280`, `client/src/loot_drops.rs:304` and `client/src/loot_drops.rs:328`.

## 15. STRINGS

This section writes no player-facing string and no button-hint text.

**§15.1 Table.** Planned (STRINGS-1): every player-facing string lives in one English table the client reads, plus one literal check on new code. The owner is planned, and that card's PRE-PATH sets it. `content/locales/en.json`, `content/locales/es.json`, `content/locales/fr.json`, `content/locales/de.json`, `content/locales/ar.json` and `content/locales/glossary.json` are in the tree. No module line in `client/src/lib.rs` or `client/src/main.rs` reads them.

**§15.2 Copy rule.** New or changed player-facing copy and button-hint text are Sherif's. HUD-IA-1 is bound by that rule.

**§15.3 Language.** English-only for now. Sherif's "1a" is #738 comment 7118839473. `.github/workflows/automated-translations.yml:4` records that workflow as manual-only, and `.github/workflows/automated-translations.yml:5` is `workflow_dispatch`. The workflow file and the locale files stay. This card deletes nothing.

**§15.4 Honesty fns.** These fold into the one later check. This page adds no second check.

- `persona_copy_is_honest` at `shared/persona.rs:788`.
- `embassy_copy_is_honest` at `shared/embassy.rs:53`.
- `temper_copy_is_honest` at `shared/temper.rs:130`.
- `manufacture_copy_is_honest` at `shared/fabricator.rs:105`.
- `schedule_copy_is_honest` at `client/src/living_day.rs:324`.
- `ecology_copy_is_honest` at `client/src/living_ecology.rs:184`.
- `climate_dress_copy_is_honest` at `client/src/climate_plane.rs:2103`.
- `presence_copy_is_honest` at `client/src/human_presence.rs:2519`.

## 16. SERVICES

A service that is not in the tree is **GAP**. The Steam row is description only and reserved to Sherif. This page changes no depot and no config. Telemetry is local only.

| Service | Dev expectation | Prod expectation | Health check today | Owner path |
|---|---|---|---|---|
| Build / CI | `cargo test -p shared -p rsil-identity` and `cargo test -p powrush-client --lib` on a PR | the same on main | Core `.github/workflows/ci.yml:18` and `.github/workflows/ci.yml:141`; Core + Q2 one-frame `.github/workflows/agent-headless-qa.yml:25`; validate `.github/workflows/validate-full-files.yml:8`; PATSAGi merge court `.github/workflows/patsagi-auto-merge.yml:20` | `.github/workflows/ci.yml`, `.github/workflows/agent-headless-qa.yml`, `.github/workflows/validate-full-files.yml`, `.github/workflows/patsagi-auto-merge.yml` |
| Offline-build | `workflow_dispatch` (`.github/workflows/offline-build.yml:7`). A pull request runs it only when that workflow file changes (`.github/workflows/offline-build.yml:8`, `.github/workflows/offline-build.yml:10`) | push to main (`.github/workflows/offline-build.yml:5`) and tags `v*` (`.github/workflows/offline-build.yml:6`) | offline-build `.github/workflows/offline-build.yml:17` on the matrix `.github/workflows/offline-build.yml:27`; clean-boot `.github/workflows/offline-build.yml:221`; the steamworks assert `.github/workflows/offline-build.yml:46` | `.github/workflows/offline-build.yml` |
| Pages | GAP | GAP | GAP. No Pages workflow under `.github/workflows/`. The Pages API for this repo returns 404. `website/index.html` is in the tree, and no workflow here deploys it | GAP |
| Steam depot | description only | description only. Reserved to Sherif | GAP. No depot check. The `steam` feature is optional and off by default (`client/Cargo.toml:13`, `client/Cargo.toml:14`, `client/Cargo.toml:52`), asserted at `.github/workflows/offline-build.yml:46`. Files present, unchanged: `publishing/steam/PARTNER_CHECKLIST.md`, `publishing/steam/steam_cloud_config.json`, `publishing/steam/sync_steam_appid.py`, `deployment/steam/steam_build_v18.10.sh`. DEPOT-DRYRUN-1 is planned, local and dry-run only | `publishing/steam/` |
| Telemetry | local files only, off unless a flag is set | the same. No data off the device | panic hook `client/src/main.rs:19` and `client/src/main.rs:80`, tests `client/src/main.rs:193`. Opt-in perf log `client/src/frame_pace.rs:14` and `client/src/frame_pace.rs:20`. Ingest off by default `shared/lived_tick_ingest.rs:3` and `shared/lived_tick_ingest.rs:26`. `tools/export_powrush_telemetry.py:119` writes JSON to a file and `tools/export_powrush_telemetry.py:122` writes it to stdout. The file has no upload call | `client/src/main.rs`, `client/src/frame_pace.rs`, `shared/lived_tick_ingest.rs`. CRASH-REPORT-1 is planned and local only |
| Saves | user-dir rule and `POWRUSH_USER_DIR` (`shared/user_persist.rs:9`) | OS user-data dir, atomic write plus `.bak` (`shared/user_persist.rs:12`, `shared/user_persist.rs:94`) | 27 `#[test]` fns in `shared/user_persist.rs`; old-save tests `client/src/local_session_persist.rs:274` and `client/src/local_session_persist.rs:280`; the 23 files are §10 | `shared/user_persist.rs` |

`client/src/steam_abundance_mirror.rs:25` stays what §10 says: local stage directories only. It is not a Steam service row. `shared/user_persist.rs:20` says no Steamworks account or cloud API.

## 17. Enforcement

Once this file is on main, every new CUT cites the spine contract it satisfies: a §11 block, or a row in §12, §13, §14, §15 or §18. The PR names the test that proves that contract. Clerk's PRE-PATH checks both, and fills that card's §12 row.

Cards already ruled are not held back: COMBAT-FAIR-1, STRIKE-HINT-1, GROUND-SNAP-1, GROUND-SNAP-2, GROUND-SNAP-3, GROUND-SNAP-4, LOD-LIMBS-1 and REALM-NAMES-1.

Rows a later test may cite: §13.1 through §13.8, §14.1 through §14.4, §15.1 through §15.4, §18.1 through §18.7, and the ten §12 rows.

Soft-aim stays planned. No soft-aim function is in `client/src` at `44dca531`. COMBAT-FAIR-1 landed at `44dca53197ccd437a1931a1f66b54ed165f3f400`. The `combat_core` anchors on L19 and L27 were refreshed to that tip.

L35 and L37 are left as they stand (D6, ruling 7119432575). An equipped owned find's standing step reaches combat through `land_hit_paid` (`client/src/combat_core.rs:356`; the add is `client/src/combat_core.rs:360`). Tested at `client/src/combat_core.rs:2303`.

Steam, spend, deletes, new or changed copy, look, and ASSET-CI-PRIVATE-1 stay reserved to Sherif. Copy and button-hint text follow §15.2.

## 18. PLAYER RIG

Owner: `client/src/human_presence.rs`. Known-today lines in this section were re-read at the SOFT-PANEL-PIN-1 tip, where `client/src/human_presence.rs` is byte-identical to main `54986f38`. This section records what the code does. It adds no rule and holds back no card already ruled (§17). How the body looks stays under Sherif's veto (RULING 7117797312(b)). This page states structure only.

**§18.1 Spawn.** `HumanPresencePlugin` (`client/src/human_presence.rs:817`) is added at `client/src/lib.rs:296`. `spawn_human_presence` (`client/src/human_presence.rs:869`) runs at Startup (`client/src/human_presence.rs:841`). The root carries `HumanPresence` (`client/src/human_presence.rs:924`) and the hips capsule (`client/src/human_presence.rs:909`, `client/src/human_presence.rs:920`).

**§18.2 Parts and joints.** The pose systems drive `PersonPart` joints (`client/src/human_presence.rs:266`): the waist, the head, and one shoulder and one hip per side. The torso, arms and legs are capsules (`client/src/human_presence.rs:910`, `client/src/human_presence.rs:913`, `client/src/human_presence.rs:914`). The head and the hand accent are spheres (`client/src/human_presence.rs:911`, `client/src/human_presence.rs:915`; `PersonAccent` `client/src/human_presence.rs:275`). Joints are never scaled. Only child meshes carry a LOD law (`PersonLodMesh` `client/src/human_presence.rs:280`).

**§18.3 LOD law (LOD-LIMBS-1).** The detail scale comes from `GraphicsPreset` (`shared/local_settings.rs:164`, §13) through `lived_presence_detail_scale` (`client/src/gltf_integration.rs:181`): Low 0.65, Medium 1.0, High 1.15 (`client/src/gltf_integration.rs:173`, `client/src/gltf_integration.rs:174`, `client/src/gltf_integration.rs:175`). Girth parts keep their length and scale only x and z (`girth_scale` `client/src/human_presence.rs:475`): the torso (`client/src/human_presence.rs:943`), arms (`client/src/human_presence.rs:983`) and legs (`client/src/human_presence.rs:1016`). Whole parts scale evenly (`lod_scale` `client/src/human_presence.rs:484`): the head (`client/src/human_presence.rs:953`) and the hand accent (`client/src/human_presence.rs:993`). The shoulder pivot follows torso girth (`shoulder_x` `client/src/human_presence.rs:480`). `apply_presence_mesh_lod` (`client/src/human_presence.rs:1028`) re-applies the law only when the setting changes (`client/src/human_presence.rs:1036`). It runs in the Update chain (`client/src/human_presence.rs:846`).

**§18.4 Readable body.** `presence_still_capsule_readable` (`client/src/human_presence.rs:443`) requires every rest-pose joint overlap from `presence_joint_overlaps` (`client/src/human_presence.rs:500`) to be at least `-JOINT_EPS` (`client/src/human_presence.rs:496`, 5 mm). `presence_low_stays_capsule_readable` (`client/src/human_presence.rs:433`) requires the Low plan to be primitives only and never to dress. Tests: `lod_limbs_1_joint_overlaps_hold_every_tier` (`client/src/human_presence.rs:7986`) and `lod_limbs_1_real_lod_switch_keeps_every_joint` (`client/src/human_presence.rs:8080`).

**§18.5 Pose.** `pose_person` (`client/src/human_presence.rs:1690`) runs in the same chain (`client/src/human_presence.rs:851`), after `sync_body` (`client/src/human_presence.rs:850`). It sets the waist from `waist_pitch` (`client/src/human_presence.rs:536`) and `crouch_drop` (`client/src/human_presence.rs:546`), both read through `bow_tell` (`client/src/human_presence.rs:551`) at `client/src/human_presence.rs:1719` and `client/src/human_presence.rs:1720`. It sets the head from `head_pitch` (`client/src/human_presence.rs:541`) at `client/src/human_presence.rs:1723`, and each shoulder and hip from `shoulder_pitch` (`client/src/human_presence.rs:556`) at `client/src/human_presence.rs:1727` and `hip_pitch` (`client/src/human_presence.rs:570`) at `client/src/human_presence.rs:1735`. `stance_read` (`client/src/human_presence.rs:523`) names the `Stance` (`client/src/human_presence.rs:287`). Tests: `ground_snap_3_winded_after_sprint_walks_without_crouch` (`client/src/human_presence.rs:3351`) and `ground_snap_3_primitives_winded_bow_only_when_still` (`client/src/human_presence.rs:3418`).

**§18.6 Locomotion.** `apply_locomotion` (`client/src/human_presence.rs:1336`) runs in FixedUpdate (`client/src/human_presence.rs:865`). Riser lips go through `push_out_of_riser_lips` (`client/src/human_presence.rs:1575`). Tests: `ground_snap_2_walk_height_matches_drawn_tread_every_preset` (`client/src/human_presence.rs:7706`), `ground_snap_4_walk_height_follows_drawn_ground_per_realm` (`client/src/human_presence.rs:7809`), `ground_snap_5_still_near_rim_stays_on_tread` (`client/src/human_presence.rs:7912`) and `ground_snap_5_airborne_lip_reads_feet_before_fall` (`client/src/human_presence.rs:7937`).

**§18.7 Authored body lane.** At Startup, `spawn_human_presence` takes an authored body lane when `use_optional_glb` (`client/src/gltf_integration.rs:151`) is true (`client/src/human_presence.rs:889`), and then skips the primitives of §18.1 to §18.5 (`client/src/human_presence.rs:891`). That lane spawns `AuthoredBodyRoot` (`client/src/human_presence.rs:232`) in `spawn_authored_ual1_presence` (`client/src/human_presence.rs:1765`). Its animation systems are registered at `client/src/human_presence.rs:859` and `client/src/human_presence.rs:860`. This page does not name the asset or its source. Those stay with Sherif (§17).

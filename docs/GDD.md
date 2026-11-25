# Game design docs index

GDD-INDEX-1. Docs only. This file is new. It links the docs on main and replaces none of them.

This page is adopted by the PATSAGi Councils under Sherif's typed delegation t1767u, t1948u and t2247u. Sherif may veto or amend, and keeps the sound, look, copy and feel vetoes.

Every path and line cite below was read at main `9248857a`. Planned items are plain text, not links. No path is invented for them.

## 1. Status values

- `foundation`: written by a ruled docs-lane card. The row names the card and the line that says so.
- `index`: an index that already exists on main.
- `historical`: only where the file itself or an existing index says so. The row cites that line.
- `listed`: on main; status not reviewed by this card. This is the default.
- `planned`: named in a ruling but not on main.

A status records cited evidence only. It is not a judgment of quality or currency. Changing a doc's status is its own card.

## 2. Existing indexes and the spine

- [DOC_CANON.md](DOC_CANON.md) stays the read-first order (`docs/DOC_CANON.md:3-12`). README stays canonical (`docs/DOC_CANON.md:21`, `docs/archive/README.md:39`).
- This page links [DOC_CANON.md](DOC_CANON.md), [DESIGN-INDEX.md](DESIGN-INDEX.md) and [archive/README.md](archive/README.md) and copies none of their lists. It inherits nothing from DESIGN-INDEX.md.
- System contracts are owned by [SYSTEMS_SPINE.md](SYSTEMS_SPINE.md) (header `docs/SYSTEMS_SPINE.md:1-9`, §17 Enforcement `docs/SYSTEMS_SPINE.md:459-465`). This page adds no contract, no §12 row and no player-facing copy.

## 3. Foundations and indexes

| Doc | Status | What it is |
|---|---|---|
| [DESIGN_PILLARS.md](DESIGN_PILLARS.md) | foundation (DESIGN-PILLARS-1, `docs/DESIGN_PILLARS.md:3`) | Design pillars and their weights |
| [SYSTEMS_SPINE.md](SYSTEMS_SPINE.md) | foundation (SYSTEMS-SPINE-1, `docs/SYSTEMS_SPINE.md:3`) | System contracts, owners and enforcement |
| [WORLD_BIBLE.md](WORLD_BIBLE.md) | foundation (WORLD-BIBLE-1, `docs/WORLD_BIBLE.md:3`) | World bible: realms as named in code, travel, zone map |
| [WORLD_GEN.md](WORLD_GEN.md) | foundation (WORLD-GEN-1, `docs/WORLD_GEN.md:3`) | World generation method: layers, seed, determinism, fixtures |
| [LOOP_MAP.md](LOOP_MAP.md) | foundation (LOOP-MAP-1, `docs/LOOP_MAP.md:1`) | The real loop on main, including quest-log surfaces |
| [LOOK_TARGET.md](LOOK_TARGET.md) | foundation (LOOK-TARGET-1, `docs/LOOK_TARGET.md:3`) | Per-zone and hero look roll-up; targets, not look rulings |
| [DOC_CANON.md](DOC_CANON.md) | index | Read-first order, historical list and version rule |
| [DESIGN-INDEX.md](DESIGN-INDEX.md) | index | Older design documents index (v14.5) |
| [archive/README.md](archive/README.md) | index | Index for stubbed and bannered root files |

## 4. Planned (not on main)

| Item | Status | Note |
|---|---|---|
| WORLD-DIRECTOR-1 | planned | Ruled path `docs/design/WORLD_DIRECTOR.md`, not on main (7111195757; `docs/SYSTEMS_SPINE.md:66`) |
| UNLOCK-LADDER-1 | planned | Ruled path `docs/design/UNLOCK_LADDER.md`, not on main (7111363137; `docs/SYSTEMS_SPINE.md:50`) |
| ENTITY-PURPOSE-1 | planned | Card name only; no path ruled (7117081816; `docs/SYSTEMS_SPINE.md:17`) |
| WORLD-TRAVEL-1 | planned | Card name only; no path ruled (7117081816; `docs/WORLD_GEN.md:7`) |
| Quest-log tiers | planned | planned, no card yet (7111363137 rule 2; DESIGN_PILLARS.md:41) |
| SLICE-ZONE-1 | planned | planned, no card yet (7118811899) |

## 5. Open items (linked, not decided)

Nothing on this page settles these. Each stays open where it lives:

- Realm and zone open decisions: [WORLD_BIBLE.md](WORLD_BIBLE.md) §7 (`docs/WORLD_BIBLE.md:151`).
- World generation items pending Sherif: [WORLD_GEN.md](WORLD_GEN.md) §9 (`docs/WORLD_GEN.md:134`).
- Open look questions: [LOOK_TARGET.md](LOOK_TARGET.md) §9 (`docs/LOOK_TARGET.md:140`).
- Loop open questions: [LOOP_MAP.md](LOOP_MAP.md) (`docs/LOOP_MAP.md:169`).

## 6. Inventory

One row per tracked `.md` under `docs/` at `9248857a`, outside `docs/visual-pass/`, except the foundation and index rows in §3. Files guarded by the spoiler rule (7117141494) are not indexed. `legal/` is out of scope.

| Doc | Status | What it is |
|---|---|---|
| [AAA_FEEL_WORKPACK.md](AAA_FEEL_WORKPACK.md) | listed | Offline 1.0 feel workpack for practical devices |
| [AGENT_AUTONOMY_COURT_2026-09-22.md](AGENT_AUTONOMY_COURT_2026-09-22.md) | listed | Agent autonomy court record |
| [AGENT_COMPLETION_PACK_v23.2.md](AGENT_COMPLETION_PACK_v23.2.md) | listed | Agent completion pack (v23.2) |
| [AGENT_QA_ORACLE.md](AGENT_QA_ORACLE.md) | listed | Fail-closed CARD schema for agent QA |
| [AGENT_QA_TAPE.md](AGENT_QA_TAPE.md) | listed | Machine PASS/FAIL roll-up for agent QA |
| [AGENT_RUN_BRIEF.md](AGENT_RUN_BRIEF.md) | listed | Agent run brief |
| [AION2_READ.md](AION2_READ.md) | listed | Sky is a place, not a product |
| [AIR-FOUNDATION-INTEGRATION.md](AIR-FOUNDATION-INTEGRATION.md) | listed | AlphaProMega Air Foundation integration model |
| [AMBROSIAN_ASCENSION_MERCY_ASCENT.md](AMBROSIAN_ASCENSION_MERCY_ASCENT.md) | listed | Ambrosian Ascension: the Mercy Ascent race design |
| [AMBROSIAN_ATTUNEMENT_MECHANICS.md](AMBROSIAN_ATTUNEMENT_MECHANICS.md) | listed | Ambrosian attunement mechanics |
| [AMBROSIAN_CRYSTALLINE_SHIP_DESIGNS.md](AMBROSIAN_CRYSTALLINE_SHIP_DESIGNS.md) | listed | Ambrosian crystalline ship designs |
| [AMBROSIAN_RACE_MECHANICS.md](AMBROSIAN_RACE_MECHANICS.md) | listed | Ambrosian race mechanics |
| [AMBROSIAN_SELF_REDEMPTION_PATH.md](AMBROSIAN_SELF_REDEMPTION_PATH.md) | listed | Ambrosian self-redemption path (the Lattice Revolution) |
| [ARCHITECTURE.md](ARCHITECTURE.md) | listed | Architecture overview |
| [ARCHITECTURE_LAYERS_AND_DATAFLOW.md](ARCHITECTURE_LAYERS_AND_DATAFLOW.md) | listed | Architecture layers and data flow (v19.20) |
| [ART_BIBLE.md](ART_BIBLE.md) | listed | Visual constitution (art bible) |
| [ASSET_BUDGET_COURT.md](ASSET_BUDGET_COURT.md) | listed | Offline 1.0 asset budget court |
| [AUDIO_DIRECTION.md](AUDIO_DIRECTION.md) | listed | Audio direction |
| [AURORAL_NEXUS.md](AURORAL_NEXUS.md) | listed | Auroral Unification Nexus: technical, lore and design notes |
| [Ambrosian_Ascension_Mercy_Ascent_Design_v1.1.md](Ambrosian_Ascension_Mercy_Ascent_Design_v1.1.md) | listed | Ambrosian Ascension and the Mercy Ascent Trial (v1.1) |
| [BEVY_CLIMB_PLAN.md](BEVY_CLIMB_PLAN.md) | listed | Bevy climb plan |
| [BLIZZARD_CRAFT_UX.md](BLIZZARD_CRAFT_UX.md) | listed | Menu, intro and chrome craft notes |
| [BOARDING_MECHANICS.md](BOARDING_MECHANICS.md) | listed | Boarding mechanics |
| [BOT_RESET_QUEUE.md](BOT_RESET_QUEUE.md) | listed | Bot reset queue (steward) |
| [BROOD_SPIRE.md](BROOD_SPIRE.md) | listed | The Brood Spire and the Hivelord's Suit: technical and lore notes |
| [CHARACTER_IDENTITY.md](CHARACTER_IDENTITY.md) | listed | Character creator, backstory, loyalties and cosmetics |
| [CODEX_ART_PROMPTS.md](CODEX_ART_PROMPTS.md) | listed | Codex art prompts: site paintings, one world |
| [CODE_MODIFICATION_WORKFLOW.md](CODE_MODIFICATION_WORKFLOW.md) | listed | Code modification workflow |
| [COMPLETION_BRIEF.md](COMPLETION_BRIEF.md) | listed | Completion brief |
| [CONSTELLATION_LIVING_AUTHORITY_2026-08-19.md](CONSTELLATION_LIVING_AUTHORITY_2026-08-19.md) | listed | Constellation living authority note |
| [CONTROLLER_NAV_AUDIT.md](CONTROLLER_NAV_AUDIT.md) | listed | Controller navigation audit (CONTROLLER-NAV-AUDIT-1) |
| [COPYWRITING_LISTS_PUBLIC_NOTE.md](COPYWRITING_LISTS_PUBLIC_NOTE.md) | listed | Copywriting lists: public note |
| [COUNCIL_MERCY_TRIAL.md](COUNCIL_MERCY_TRIAL.md) | listed | Council Mercy Trial: multiplayer receptor bloom amplification (v18.9) |
| [COUNCIL_SYSTEM.md](COUNCIL_SYSTEM.md) | listed | Council system architecture |
| [COUNCIL_VERTICAL_SLICE_TEST_PLAN.md](COUNCIL_VERTICAL_SLICE_TEST_PLAN.md) | listed | Council Mercy Trial vertical slice test plan |
| [CREDIT_RESERVE_LOGISTICS.md](CREDIT_RESERVE_LOGISTICS.md) | listed | Logistics bible for immersion ladder rung D |
| [CROWNSTONE_QUESTLINE.md](CROWNSTONE_QUESTLINE.md) | listed | Crownstone questline |
| [CROWNSTONE_TRILEMMA_PATHS.md](CROWNSTONE_TRILEMMA_PATHS.md) | listed | Crownstone trilemma paths |
| [CURSOR_GROK_MODEL_SEAT.md](CURSOR_GROK_MODEL_SEAT.md) | listed | Cursor model seat note |
| [CYDRUID_ECOLOGICAL_DEFENSE_ROLES.md](CYDRUID_ECOLOGICAL_DEFENSE_ROLES.md) | listed | Cydruid ecological defense roles |
| [CYDRUID_ORGANIC_SHIP_DESIGNS.md](CYDRUID_ORGANIC_SHIP_DESIGNS.md) | listed | Cydruid organic ship designs |
| [DEMO_PREALPHA_BANK.md](DEMO_PREALPHA_BANK.md) | listed | Pre-alpha demo court inventory |
| [DIPLOMACY_AND_WORLD_SIMULATION.md](DIPLOMACY_AND_WORLD_SIMULATION.md) | listed | Diplomacy and world simulation |
| [DISCORDANT_AMBROSIAN_CORRUPTION.md](DISCORDANT_AMBROSIAN_CORRUPTION.md) | listed | Discordant Ambrosian corruption |
| [DISCORDANT_REDEMPTION_QUESTLINES.md](DISCORDANT_REDEMPTION_QUESTLINES.md) | listed | Discordant redemption questlines |
| [DOGFIGHT_MECHANICS.md](DOGFIGHT_MECHANICS.md) | listed | Dogfight mechanics |
| [DRAEK_COUNTER_PURIFICATION_STRATEGIES.md](DRAEK_COUNTER_PURIFICATION_STRATEGIES.md) | listed | Draek counter-purification strategies |
| [DRAEK_FLEET_AI_BEHAVIOR_PATTERNS.md](DRAEK_FLEET_AI_BEHAVIOR_PATTERNS.md) | listed | Draek fleet AI behavior patterns |
| [DRAEK_FLEET_AI_SYSTEMS.md](DRAEK_FLEET_AI_SYSTEMS.md) | listed | Draek fleet AI systems |
| [DRIVE_GAP_COURT.md](DRIVE_GAP_COURT.md) | listed | Drive gap court |
| [DRIVE_LORE_ADAPTATION.md](DRIVE_LORE_ADAPTATION.md) | listed | Drive lore adaptation |
| [DRIVE_PLACE_CITE.md](DRIVE_PLACE_CITE.md) | listed | Drive place cite court (DRIVE-PLACE-CITE) |
| [DRIVE_POWRUSH_BANK.md](DRIVE_POWRUSH_BANK.md) | listed | Powrush Drive court inventory |
| [DYNAMIC_ARCHETYPE_CREATION_SYSTEM.md](DYNAMIC_ARCHETYPE_CREATION_SYSTEM.md) | listed | Dynamic archetype creation system |
| [EA_BAR.md](EA_BAR.md) | listed | Early Access loop as one verb chain |
| [ENDUSER_EXPERIENCE_SIMULATION_POLISH_PROTOCOL.md](ENDUSER_EXPERIENCE_SIMULATION_POLISH_PROTOCOL.md) | listed | End-user experience simulation and polish protocol |
| [ENSLAVED_MINION_SPECIES.md](ENSLAVED_MINION_SPECIES.md) | listed | Enslaved minion species |
| [EPIPHANY_CATALYST_SYSTEMS.md](EPIPHANY_CATALYST_SYSTEMS.md) | listed | Epiphany catalyst systems |
| [F9_TWO_CLIENT_LOCALHOST.md](F9_TWO_CLIENT_LOCALHOST.md) | listed | Two-client same-hex localhost recipe (dev only) |
| [FACTIONS_OVERVIEW.md](FACTIONS_OVERVIEW.md) | listed | Factions overview |
| [FACTION_HERALDRY.md](FACTION_HERALDRY.md) | listed | Workshop finishes and house heraldry, without transmog |
| [FIRST10_AUDIT.md](FIRST10_AUDIT.md) | listed | First ten minutes: frame and source walk (FIRST10-AUDIT-1) |
| [FIRST_EPIPHANY_VALIDATION_CHECKLIST.md](FIRST_EPIPHANY_VALIDATION_CHECKLIST.md) | listed | First epiphany validation checklist |
| [FIRST_HOUR_CLIENT_BIND.md](FIRST_HOUR_CLIENT_BIND.md) | listed | First hour client bind |
| [FIRST_HOUR_PLAYTEST.md](FIRST_HOUR_PLAYTEST.md) | listed | First hour playtest |
| [FIRST_LAUNCH_UI_SCALE.md](FIRST_LAUNCH_UI_SCALE.md) | listed | Named chrome large enough on first launch (UI scale) |
| [FLEET_CLASSES.md](FLEET_CLASSES.md) | listed | Fleet classes |
| [FLOW_STATE_FORGE.md](FLOW_STATE_FORGE.md) | listed | Flow State Forge module design (v18.9) |
| [FREE_HOSTING_TEST_DEPLOYMENT.md](FREE_HOSTING_TEST_DEPLOYMENT.md) | listed | Test deployment guide for a Docker server |
| [FULL_RBE_STATUS.md](FULL_RBE_STATUS.md) | listed | full_rbe feature status |
| [FUN_WITHOUT_WOW.md](FUN_WITHOUT_WOW.md) | listed | The Offline teacher is not a raid calendar |
| [GDD_ADAPTATION.md](GDD_ADAPTATION.md) | listed | GDD 1.5-2.0 adapted to lived Powrush |
| [GDD_IMMERSION_REVISION.md](GDD_IMMERSION_REVISION.md) | listed | Immersion references folded into existing teachers |
| [GENSHARE.md](GENSHARE.md) | listed | Recipe, not triangles (docs stamp) |
| [GODSPEED_PREP.md](GODSPEED_PREP.md) | listed | Godspeed prep |
| [GROK_BOT_PLAYER_EXPERIENCE_BRIEF.md](GROK_BOT_PLAYER_EXPERIENCE_BRIEF.md) | listed | Grok Bot brief on the lived player experience |
| [GROK_BOT_SEATS_POWRUSH_GROUP_2026-09-22.md](GROK_BOT_SEATS_POWRUSH_GROUP_2026-09-22.md) | listed | Grok Bot seats for the Powrush group |
| [GROK_BOT_TODO.md](GROK_BOT_TODO.md) | listed | Grok Bot to-do list |
| [GROVE_COMMUNION_RITUAL.md](GROVE_COMMUNION_RITUAL.md) | listed | Grove Communion Ritual (Sylvaris redemption phase 3) |
| [HIVELORD_BIOMECHANICAL_SUIT.md](HIVELORD_BIOMECHANICAL_SUIT.md) | listed | The Hivelord's biomechanical suit: technical and lore notes |
| [HIVELORD_COUNTER_STRATEGIES.md](HIVELORD_COUNTER_STRATEGIES.md) | listed | Hivelord counter-strategies (Draek Dominion response doctrine) |
| [HOUR_THREE.md](HOUR_THREE.md) | listed | Proof Pack and Embassy door (v23.2.35) |
| [HOUR_THREE_PLAYTEST.md](HOUR_THREE_PLAYTEST.md) | listed | Hour three playtest |
| [HOUR_TWO.md](HOUR_TWO.md) | listed | Charter door (v23.2.32) |
| [HOUR_TWO_PLAYTEST.md](HOUR_TWO_PLAYTEST.md) | listed | Hour two playtest |
| [HUMAN_EXPERIENCE_GAP_ANALYSIS_MULTI_SERVER_WAR_SIM_v20.8.md](HUMAN_EXPERIENCE_GAP_ANALYSIS_MULTI_SERVER_WAR_SIM_v20.8.md) | listed | Human experience gap analysis: multi-realm war simulation (v20.8) |
| [HUMAN_HYBRID_PROTOCOL_CODE.md](HUMAN_HYBRID_PROTOCOL_CODE.md) | listed | Human Hybrid Protocol Code |
| [HUMAN_PATCHWORK_SHIP_DESIGNS.md](HUMAN_PATCHWORK_SHIP_DESIGNS.md) | listed | Human patchwork ship designs |
| [HUMAN_PLAYABILITY_v21.92.md](HUMAN_PLAYABILITY_v21.92.md) | listed | Human playability: living practice loop (v21.92) |
| [HUMAN_PLAYABILITY_v21.94.md](HUMAN_PLAYABILITY_v21.94.md) | listed | Human playability: local first hour (v21.99.4) |
| [HUMAN_PLAYABILITY_v23.2_ARCHIVE_STUBS.md](HUMAN_PLAYABILITY_v23.2_ARCHIVE_STUBS.md) | listed | Slice 20: archive stubs (v23.2.27) |
| [HUMAN_PLAYABILITY_v23.2_CLIMATE_VISIBLE.md](HUMAN_PLAYABILITY_v23.2_CLIMATE_VISIBLE.md) | listed | Slice 16: climate visible (v23.2.23) |
| [HUMAN_PLAYABILITY_v23.2_COMPASS.md](HUMAN_PLAYABILITY_v23.2_COMPASS.md) | listed | Slice 14: compass tells (v23.2.18) |
| [HUMAN_PLAYABILITY_v23.2_COOP_VOICE.md](HUMAN_PLAYABILITY_v23.2_COOP_VOICE.md) | listed | Slice 4: co-op voice (v23.2.8) |
| [HUMAN_PLAYABILITY_v23.2_CROWNSTONE.md](HUMAN_PLAYABILITY_v23.2_CROWNSTONE.md) | listed | Slice 11: Crownstone set-piece (v23.2.15) |
| [HUMAN_PLAYABILITY_v23.2_DECLARED_LETHAL.md](HUMAN_PLAYABILITY_v23.2_DECLARED_LETHAL.md) | listed | Slice 10: DeclaredLethal and blood tariff (v23.2.14) |
| [HUMAN_PLAYABILITY_v23.2_EMBASSY.md](HUMAN_PLAYABILITY_v23.2_EMBASSY.md) | listed | Slice 8: Embassy and blueprints (v23.2.12) |
| [HUMAN_PLAYABILITY_v23.2_FABRICATOR.md](HUMAN_PLAYABILITY_v23.2_FABRICATOR.md) | listed | Slice 7: Fabricator and Proof Pack (v23.2.11) |
| [HUMAN_PLAYABILITY_v23.2_HOUR_SACRED.md](HUMAN_PLAYABILITY_v23.2_HOUR_SACRED.md) | listed | Slice 0: hour sacred (v23.2.4) |
| [HUMAN_PLAYABILITY_v23.2_HOUR_TWO_DOOR.md](HUMAN_PLAYABILITY_v23.2_HOUR_TWO_DOOR.md) | listed | Slice 21: hour-two Charter door (v23.2.28) |
| [HUMAN_PLAYABILITY_v23.2_HYBRID_MATRIX.md](HUMAN_PLAYABILITY_v23.2_HYBRID_MATRIX.md) | listed | Slice 13: Hybrid Matrix v0 (v23.2.17) |
| [HUMAN_PLAYABILITY_v23.2_HYGIENE.md](HUMAN_PLAYABILITY_v23.2_HYGIENE.md) | listed | Slice 19: repo hygiene (v23.2.26) |
| [HUMAN_PLAYABILITY_v23.2_INFRA_SPILL.md](HUMAN_PLAYABILITY_v23.2_INFRA_SPILL.md) | listed | Slice 5: infra offline and spill (v23.2.9) |
| [HUMAN_PLAYABILITY_v23.2_LEDGER_BIND.md](HUMAN_PLAYABILITY_v23.2_LEDGER_BIND.md) | listed | Slice 6: Ledger and Bind/Escort (v23.2.10) |
| [HUMAN_PLAYABILITY_v23.2_ONBOARDING_CARD.md](HUMAN_PLAYABILITY_v23.2_ONBOARDING_CARD.md) | listed | Slice 17: onboarding card (v23.2.24) |
| [HUMAN_PLAYABILITY_v23.2_SKIRMISH_WELL.md](HUMAN_PLAYABILITY_v23.2_SKIRMISH_WELL.md) | listed | Slice 15 / Wave 2: skirmish well (v23.2.22) |
| [HUMAN_PLAYABILITY_v23.2_SPECIES_REDEMPTION.md](HUMAN_PLAYABILITY_v23.2_SPECIES_REDEMPTION.md) | listed | Slice 12: species redemption v0 (v23.2.16) |
| [HUMAN_PLAYABILITY_v23.2_TEACHING_CLIMATES.md](HUMAN_PLAYABILITY_v23.2_TEACHING_CLIMATES.md) | listed | Slice 18: teaching climates (v23.2.25) |
| [HUMAN_PLAYABILITY_v23.2_WAR_WEEK.md](HUMAN_PLAYABILITY_v23.2_WAR_WEEK.md) | listed | Slice 9: war week (v23.2.13) |
| [HUMAN_RACE_MECHANICS.md](HUMAN_RACE_MECHANICS.md) | listed | Human race mechanics |
| [HYBRID_INSTABILITY_MITIGATION_SYSTEMS.md](HYBRID_INSTABILITY_MITIGATION_SYSTEMS.md) | listed | Hybrid instability mitigation systems |
| [IMAGINE_TRAILER_PACK.md](IMAGINE_TRAILER_PACK.md) | listed | Stills to trailer cut pack |
| [IN-BROWSER-ASSET-LOADING.md](IN-BROWSER-ASSET-LOADING.md) | listed | Empty file (0 lines) |
| [INPUT_CANON.md](INPUT_CANON.md) | listed | One Use verb, all hands (input canon) |
| [ITCH_EARLY_ACCESS_KIT.md](ITCH_EARLY_ACCESS_KIT.md) | listed | Early access kit card (ITCH-EARLY-ACCESS-KIT-1) |
| [JOY_WITHOUT_MALL.md](JOY_WITHOUT_MALL.md) | listed | Rarity and social joy without the mall |
| [KARDASHEV_ACCELERATION_ABSOLUTE_PURE_TRUTH_DISTILLATION_FROM_X_THREAD.md](KARDASHEV_ACCELERATION_ABSOLUTE_PURE_TRUTH_DISTILLATION_FROM_X_THREAD.md) | listed | Kardashev acceleration notes from an X thread |
| [KEEPSAKE_SEED.md](KEEPSAKE_SEED.md) | listed | Mark one generation, keep the seed |
| [L2_HOUSE_PERSONA_DRESS_REPLAY.md](L2_HOUSE_PERSONA_DRESS_REPLAY.md) | listed | L2 house persona dress replay |
| [L3_SPAWN_RESEARCH.md](L3_SPAWN_RESEARCH.md) | listed | CARD R3 PATHS hunt (docs only) |
| [LAPTOP_OFFLINE_PLAY.md](LAPTOP_OFFLINE_PLAY.md) | listed | Human laptop review of offline play |
| [LAUNCH-CHECKLIST.md](LAUNCH-CHECKLIST.md) | listed | Launch checklist (v19.2.9), client rows re-checked under LR-05 |
| [LAUNCH_READINESS_AUDIT.md](LAUNCH_READINESS_AUDIT.md) | listed | Readiness gap audit |
| [LAUNCH_UX.md](LAUNCH_UX.md) | listed | Steward House launch UX (offline first) |
| [LAVAPIPE_CLICK_CLEAN.md](LAVAPIPE_CLICK_CLEAN.md) | listed | Soft GPU walk proof (docs stamp) |
| [LAYERED_GAMES_AND_SERVER.md](LAYERED_GAMES_AND_SERVER.md) | listed | Layer stack, economy spectrum, offline NPCs, server plan |
| [LIGHT_ESSENCE_EXPERIENCE_TRAVEL_LORE.md](LIGHT_ESSENCE_EXPERIENCE_TRAVEL_LORE.md) | listed | Light essence and experience travel lore |
| [LIVE_MOUTH_AUDIT.md](LIVE_MOUTH_AUDIT.md) | listed | Live mouth audit |
| [LIVING_THUNDER_SERIES_LORE_SEED.md](LIVING_THUNDER_SERIES_LORE_SEED.md) | listed | Living Thunder series lore seed |
| [LOCAL_COURT.md](LOCAL_COURT.md) | listed | Bot 1 yard court |
| [LORE_BIBLE.md](LORE_BIBLE.md) | listed | Lore bible |
| [LORE_RESEARCH_LAW.md](LORE_RESEARCH_LAW.md) | listed | Lore research law: cite, do not ship |
| [LUMINARI_EXILE_SHIP_DESIGNS.md](LUMINARI_EXILE_SHIP_DESIGNS.md) | listed | Luminari Exile ship designs |
| [LUMINARI_REDEMPTION_QUEST_BIBLE.md](LUMINARI_REDEMPTION_QUEST_BIBLE.md) | listed | Luminari Exile redemption quest bible |
| [MECHANICS_INTERACTION_BIBLE.md](MECHANICS_INTERACTION_BIBLE.md) | listed | One hand, four Places, one bill |
| [MERCY_PERSONA_CREATION.md](MERCY_PERSONA_CREATION.md) | listed | Mercy persona creation |
| [MERCY_TEMPER_PROGRESSION.md](MERCY_TEMPER_PROGRESSION.md) | listed | Mercy temper progression |
| [MESH_PERSONA_COURT.md](MESH_PERSONA_COURT.md) | listed | Comfort L/M/H persona dress bank |
| [MESH_QUALITY_BUDGET.md](MESH_QUALITY_BUDGET.md) | listed | Mesh quality budget law |
| [MONTAGE_CHANNEL_LAW.md](MONTAGE_CHANNEL_LAW.md) | listed | Montage channel law |
| [MOVEMENT_STANDARD.md](MOVEMENT_STANDARD.md) | listed | Movement standard |
| [MYCORRHIZAL_NETWORK_SYNCHRONIZATION.md](MYCORRHIZAL_NETWORK_SYNCHRONIZATION.md) | listed | Mycorrhizal network synchronization forge |
| [NAME_DOOR_LAW.md](NAME_DOOR_LAW.md) | listed | Name door law |
| [NAME_RITE.md](NAME_RITE.md) | listed | Name rite: offline, after the hour |
| [NET_OFFLINE_CONTRACT.md](NET_OFFLINE_CONTRACT.md) | listed | Online later; Offline is the door |
| [NEXT_NAMED_CARDS.md](NEXT_NAMED_CARDS.md) | listed | Next named cards (steward) |
| [NPC_SCHEDULE_SPEC.md](NPC_SCHEDULE_SPEC.md) | listed | Schedule bible for immersion ladder rung E |
| [OBFUSCATION.md](OBFUSCATION.md) | listed | Client-side obfuscation guide (v18.9) |
| [OFFLINE_ECONOMY_COURT.md](OFFLINE_ECONOMY_COURT.md) | listed | Offline 1.0 economy teacher, banked |
| [OFFLINE_SKU.md](OFFLINE_SKU.md) | listed | Offline product scope (U0) |
| [ONLINE_LADDER.md](ONLINE_LADDER.md) | listed | Online ladder R0-R7 |
| [PAIN_TRANSMUTATION_PATHS.md](PAIN_TRANSMUTATION_PATHS.md) | listed | Pain transmutation paths (Sylvaris redemption phase 4) |
| [PARKED_SURFACES.md](PARKED_SURFACES.md) | listed | Parked surfaces |
| [PATSAGI_MERGE_COURT.md](PATSAGI_MERGE_COURT.md) | listed | PATSAGi merge court |
| [PATSAGI_v21.99_HUMAN_IMMERSION.md](PATSAGI_v21.99_HUMAN_IMMERSION.md) | listed | Human immersion rulings (v21.99) |
| [PATSAGI_v22.13_FLOW_WEATHER.md](PATSAGI_v22.13_FLOW_WEATHER.md) | listed | Flow weather (v22.13) |
| [PATSAGI_v22.15_LOCAL_SIM.md](PATSAGI_v22.15_LOCAL_SIM.md) | listed | Local human sim (v22.15) |
| [PATSAGI_v22.4_BELIEVABLE_REALITY.md](PATSAGI_v22.4_BELIEVABLE_REALITY.md) | listed | Believable temporary reality (v22.4) |
| [PATSAGI_v22.9_SURVIVAL_TRANSFER.md](PATSAGI_v22.9_SURVIVAL_TRANSFER.md) | listed | Survival transfer (v22.9) |
| [PATSAGI_v22_FLESH_THE_GAME.md](PATSAGI_v22_FLESH_THE_GAME.md) | historical (`docs/PATSAGI_v22_FLESH_THE_GAME.md:1`, `docs/DOC_CANON.md:19`) | Seed map rulings (v22) |
| [PATSAGI_v23.1_LIVED_INTEGRATION.md](PATSAGI_v23.1_LIVED_INTEGRATION.md) | listed | Lived integration (v23.1) |
| [PATSAGI_v23.2_ARCHITECTURE.md](PATSAGI_v23.2_ARCHITECTURE.md) | listed | Powrush organism architecture (v23.2) |
| [PATSAGI_v23_PLAYGROUND.md](PATSAGI_v23_PLAYGROUND.md) | listed | Playground plan (v23) |
| [PEAK_MEMORY_LAW.md](PEAK_MEMORY_LAW.md) | listed | Peak memory law |
| [PERFECTION_PHILOSOPHY.md](PERFECTION_PHILOSOPHY.md) | listed | Repository workflow handover summary |
| [PERSISTENCE_ORIGINAL_OWNERSHIP.md](PERSISTENCE_ORIGINAL_OWNERSHIP.md) | listed | Persistence layer: original ownership constraint |
| [PERSON_READ_SPEC.md](PERSON_READ_SPEC.md) | listed | Presentation bible for immersion ladder rung B |
| [PHASE_A_IMPLEMENTATION_STEPS.md](PHASE_A_IMPLEMENTATION_STEPS.md) | listed | Phase A implementation steps |
| [PHASE_FEEL_P2.md](PHASE_FEEL_P2.md) | listed | Stranger-path feel survey (v23.2.49) |
| [PHASE_LADDER.md](PHASE_LADDER.md) | listed | Phase ladder: refuse as calendar |
| [PHASE_LETHAL.md](PHASE_LETHAL.md) | listed | Ledger 3 DeclaredLethal and L1 hex sign (v23.2.44) |
| [PHASE_MYTHIC.md](PHASE_MYTHIC.md) | listed | Read-first mythic verbs (v23.2.45) |
| [PHASE_PLAYABLE_LOOP.md](PHASE_PLAYABLE_LOOP.md) | listed | Stranger offline playable loop (v23.2.46) |
| [PHASE_Q.md](PHASE_Q.md) | listed | Solo shard climate (v23.2.36) |
| [PHASE_R.md](PHASE_R.md) | listed | Local hex standing (v23.2.37) |
| [PHASE_S.md](PHASE_S.md) | listed | Week audit preview (v23.2.39) |
| [PHASE_T_NET.md](PHASE_T_NET.md) | listed | Honest net label (v23.2.42) |
| [PHASE_T_OFFLINE.md](PHASE_T_OFFLINE.md) | listed | Shard sim harness (v23.2.41) |
| [PHASE_U.md](PHASE_U.md) | listed | Divergent shards (v23.2.43) |
| [PHYSICS_GRAPHICS_CANON.md](PHYSICS_GRAPHICS_CANON.md) | listed | Physics and graphics canon: lived client, lattice, lore |
| [PLACES_BIBLE.md](PLACES_BIBLE.md) | listed | Heartwood rings and yard law (docs stamp) |
| [PLACE_CLARITY_HOUR.md](PLACE_CLARITY_HOUR.md) | listed | Four rooms read as four bodies |
| [PLACE_CLARITY_WALK.md](PLACE_CLARITY_WALK.md) | listed | Human shot checklist for place clarity |
| [PLACE_DRESS_SPEC.md](PLACE_DRESS_SPEC.md) | listed | Presentation bible for immersion ladder rung F |
| [PLAYABLE_GAP_AUDIT.md](PLAYABLE_GAP_AUDIT.md) | listed | Playable gap audit |
| [PLAYABLE_RACES.md](PLAYABLE_RACES.md) | listed | Playable races |
| [PLAYER_FEEL_METRICS.md](PLAYER_FEEL_METRICS.md) | listed | Measure the stranger, not the feature list |
| [PLAYTEST_AUDIT_2026-09-09.md](PLAYTEST_AUDIT_2026-09-09.md) | listed | Docs-only playtest code check (B7 tip) |
| [POWRUSH_ENDGAME_HUMAN_EXPERIENCE_UPGRADES_v21_PATSAGi.md](POWRUSH_ENDGAME_HUMAN_EXPERIENCE_UPGRADES_v21_PATSAGi.md) | listed | Endgame human experience upgrades (v21) |
| [POWRUSH_LOOP_NOTES.md](POWRUSH_LOOP_NOTES.md) | listed | Loop depth ideas (POWRUSH-LOOP-NOTES-1) |
| [POWRUSH_MMO_ABSOLUTE_PURE_TRUTH_DISTILLATION_CODEX.md](POWRUSH_MMO_ABSOLUTE_PURE_TRUTH_DISTILLATION_CODEX.md) | listed | Empty file (0 lines) |
| [POWRUSH_MMO_ARCHITECTURE.md](POWRUSH_MMO_ARCHITECTURE.md) | listed | Architecture and strategic roadmap |
| [POWRUSH_MMO_INTEGRATED_DESIGN_v14.5.md](POWRUSH_MMO_INTEGRATED_DESIGN_v14.5.md) | listed | Empty file (0 lines) |
| [PRACTICE_NOT_VERB.md](PRACTICE_NOT_VERB.md) | listed | Practice is not a verb |
| [PRESS_KIT_v18.10.md](PRESS_KIT_v18.10.md) | listed | Press kit (v18.10) |
| [PREVIEW_CHECKLIST.md](PREVIEW_CHECKLIST.md) | listed | Human tick boxes for preview (Offline 1.0) |
| [PRE_RELEASE_LAW.md](PRE_RELEASE_LAW.md) | listed | Pre-release law |
| [PROCGEN_DESIGN.md](PROCGEN_DESIGN.md) | listed | PGEN-0 procedural level generation (docs only) |
| [PROTOCOL.md](PROTOCOL.md) | listed | Hex shard protocol (v1) |
| [Q2_CAPTURE_IF_PATH_CARD.md](Q2_CAPTURE_IF_PATH_CARD.md) | listed | CARD Q2 CAPTURE-IF-PATH |
| [QUELLORIAN_KEY_FIGURES.md](QUELLORIAN_KEY_FIGURES.md) | listed | Quellorian key figures |
| [QUELLORIAN_RESONANCE_AI_BEHAVIOR_PATTERNS.md](QUELLORIAN_RESONANCE_AI_BEHAVIOR_PATTERNS.md) | listed | Quellorian resonance AI behavior patterns |
| [QUELLORIAN_RESONANCE_AI_SYSTEMS.md](QUELLORIAN_RESONANCE_AI_SYSTEMS.md) | listed | Quellorian resonance AI systems |
| [RA_THOR_FEEDBACK_LOOP.md](RA_THOR_FEEDBACK_LOOP.md) | listed | Ra-Thor to Powrush feedback loop |
| [RA_THOR_POLICY_HINT_EMISSION.md](RA_THOR_POLICY_HINT_EMISSION.md) | listed | Ra-Thor policy hint emission contract |
| [RA_THOR_SOFT_POLICY_SNR.md](RA_THOR_SOFT_POLICY_SNR.md) | listed | Ra-Thor soft policy: high-SNR doctrine |
| [RA_THOR_TELEMETRY_EXPORT.md](RA_THOR_TELEMETRY_EXPORT.md) | listed | Ra-Thor telemetry export |
| [RBE_FIRST_HOUR.md](RBE_FIRST_HOUR.md) | listed | RBE first hour |
| [RECOVERY_INTEGRITY_REPORT_v19.20.md](RECOVERY_INTEGRITY_REPORT_v19.20.md) | listed | Recovery integrity report (v19.20) |
| [REDEMPTION_MECHANICS_PER_SPECIES.md](REDEMPTION_MECHANICS_PER_SPECIES.md) | listed | Redemption mechanics per enslaved species |
| [REPLICATION_PREDICTION_ARCHITECTURE.md](REPLICATION_PREDICTION_ARCHITECTURE.md) | listed | Replication and client prediction architecture |
| [RESEARCH-CONTRACT-TEMPLATE.md](RESEARCH-CONTRACT-TEMPLATE.md) | listed | Research and development service agreement template |
| [RESONANCE_BURST_MECHANICS.md](RESONANCE_BURST_MECHANICS.md) | listed | Resonance burst mechanics |
| [RESTORATION_AND_MERGE_PROTOCOL.md](RESTORATION_AND_MERGE_PROTOCOL.md) | listed | Restoration and merge protocol |
| [RITE_DOS_BANK.md](RITE_DOS_BANK.md) | listed | Rite and DOS bank |
| [ROADMAP.md](ROADMAP.md) | listed | Roadmap (written at v19.20) |
| [ROOM_LAW.md](ROOM_LAW.md) | listed | Room law for Core, Hands and Clerk |
| [RSIL-SPEC.md](RSIL-SPEC.md) | listed | Ra-Thor Sovereign Identity Lattice (RSIL) v1.0 |
| [SEAT_WINDOW_2026-10-04.md](SEAT_WINDOW_2026-10-04.md) | listed | Seat window note |
| [SEAT_WINDOW_CURSOR.md](SEAT_WINDOW_CURSOR.md) | listed | Seat window: Cursor |
| [SECOND_HOUR_CHARTER_FRONTIER.md](SECOND_HOUR_CHARTER_FRONTIER.md) | listed | Second hour Charter frontier |
| [SENTIENCE_THRIVE_MANIFESTO.md](SENTIENCE_THRIVE_MANIFESTO.md) | listed | Sentience Thrive Manifesto |
| [SETTLED_DOOR_CLARITY.md](SETTLED_DOOR_CLARITY.md) | listed | Door bible for immersion ladder rung C |
| [SHARD_JOIN.md](SHARD_JOIN.md) | listed | Join, drop and presence (F2-F4) |
| [SHIP_VISUAL_GUIDELINES.md](SHIP_VISUAL_GUIDELINES.md) | listed | Ship visual guidelines |
| [SIM_AND_HAND_CANON.md](SIM_AND_HAND_CANON.md) | listed | Sim and hand canon: hand ack and universe F |
| [SLICE_LOG.md](SLICE_LOG.md) | listed | Slice log |
| [SOVEREIGN_SIMULATION_HARNESS_ARCHITECTURE.md](SOVEREIGN_SIMULATION_HARNESS_ARCHITECTURE.md) | listed | Sovereign simulation harness architecture |
| [SOVEREIGN_SIMULATION_HARNESS_LEPTOS_UI_ARCHITECTURE.md](SOVEREIGN_SIMULATION_HARNESS_LEPTOS_UI_ARCHITECTURE.md) | listed | Sovereign simulation harness Leptos UI architecture |
| [SPACE_GRANDMASTERISM.md](SPACE_GRANDMASTERISM.md) | listed | Space Grandmasterism: cosmic exploration and coexistence |
| [SPATIAL_INTEREST_ARCHITECTURE.md](SPATIAL_INTEREST_ARCHITECTURE.md) | listed | Spatial interest architecture |
| [SPECIFIC_REDEMPTION_QUESTS.md](SPECIFIC_REDEMPTION_QUESTS.md) | listed | Specific redemption quests |
| [STABILITY_BATCH_SPENT.md](STABILITY_BATCH_SPENT.md) | listed | Stability batch, spent on main |
| [STEAM-CLOUD-INTEGRATION.md](STEAM-CLOUD-INTEGRATION.md) | listed | Cloud save integration (client RemoteStorage) |
| [STEAM_CHECKLIST.md](STEAM_CHECKLIST.md) | listed | Human tick-box checklist for the offline build |
| [STEAM_CLOUD_AUDIO.md](STEAM_CLOUD_AUDIO.md) | listed | Cloud audio moment catalog |
| [STEAM_DECK.md](STEAM_DECK.md) | listed | Handheld device notes |
| [STEAM_INTEGRATION.md](STEAM_INTEGRATION.md) | listed | Platform integration notes |
| [STEAM_WISHLIST_PATSAGI_BRIEF_2026-09-22.md](STEAM_WISHLIST_PATSAGI_BRIEF_2026-09-22.md) | listed | Wishlist signal brief |
| [STRANGER_LOOP.md](STRANGER_LOOP.md) | listed | One cold stranger, keys only (Offline 1.0) |
| [STUDIO_ARCHITECTURE_ORDER.md](STUDIO_ARCHITECTURE_ORDER.md) | listed | Layer cake and order of operations |
| [SURGICAL_PURIFICATION_MECHANICS.md](SURGICAL_PURIFICATION_MECHANICS.md) | listed | Surgical purification mechanics |
| [SWARM_LESSON.md](SWARM_LESSON.md) | listed | Swarm lesson |
| [SYLvaris_REDEMPTION_MECHANICS.md](SYLvaris_REDEMPTION_MECHANICS.md) | listed | Sylvaris redemption mechanics |
| [SYNC-ARCHITECTURE.md](SYNC-ARCHITECTURE.md) | listed | Empty file (0 lines) |
| [SYSTEM_INTEGRATION_MAP.md](SYSTEM_INTEGRATION_MAP.md) | listed | System integration map (v18.96) |
| [TAPE_TRUTH.md](TAPE_TRUTH.md) | listed | Video is pitch memory; the binary is the well |
| [TECH_TREE_HARDWARE_SOVEREIGNTY_OBSIDIAN_AETHER_STUBS.md](TECH_TREE_HARDWARE_SOVEREIGNTY_OBSIDIAN_AETHER_STUBS.md) | listed | Tech tree: hardware sovereignty layer stubs |
| [THE_HIVELORD.md](THE_HIVELORD.md) | listed | The Hivelord |
| [THE_MERCY_ASCENT_UI_SPEC.md](THE_MERCY_ASCENT_UI_SPEC.md) | listed | Mercy Ascent UI: player experience and Leptos spec |
| [UI_AND_VFX_DIRECTION.md](UI_AND_VFX_DIRECTION.md) | listed | UI and VFX direction |
| [UI_CHURN_PROTECTION.md](UI_CHURN_PROTECTION.md) | listed | UI churn protection note |
| [VERTICAL_SLICE_TESTING_PROTOCOL_v18.49.md](VERTICAL_SLICE_TESTING_PROTOCOL_v18.49.md) | listed | Structured vertical slice testing protocol (v18.58) |
| [VISUAL_TARGET.md](VISUAL_TARGET.md) | listed | Procedural visual knobs |
| [VOICE_ACTING_DIRECTION.md](VOICE_ACTING_DIRECTION.md) | listed | Voice acting direction |
| [VOICE_ACTING_LINES.md](VOICE_ACTING_LINES.md) | listed | Voice acting lines |
| [VOICE_LINE.md](VOICE_LINE.md) | listed | A line when the hour earns it |
| [WEB-PORTAL-AUTH-LAUNCHER.md](WEB-PORTAL-AUTH-LAUNCHER.md) | listed | Empty file (0 lines) |
| [WORK_PACK_NATIVE.md](WORK_PACK_NATIVE.md) | listed | Grok Bot and X-Grok collaboration work pack |
| [audio/3D3A_Adoption_Phase1_Evaluation.md](audio/3D3A_Adoption_Phase1_Evaluation.md) | listed | 3D3A HRTF database, phase 1 evaluation |
| [audio/HRTF_Database_Comparison.md](audio/HRTF_Database_Comparison.md) | listed | HRTF database comparison: 3D3A and MIT KEMAR |
| [audio/Spatial_Audio_Architecture_LongTerm.md](audio/Spatial_Audio_Architecture_LongTerm.md) | listed | Long-term spatial audio architecture |
| [councils.md](councils.md) | listed | APAGI Councils |
| [design/AUDIO_RADIO_DESIGN.md](design/AUDIO_RADIO_DESIGN.md) | listed | In-game radio: stations, licence manifest, station slots (design) |
| [design/AUDIO_RADIO_QUEST_DESIGN.md](design/AUDIO_RADIO_QUEST_DESIGN.md) | listed | In-game radio unlock quest: combat and Engineering paths |
| [design/COMBAT_AND_PVP.md](design/COMBAT_AND_PVP.md) | listed | Guild wars, battlegrounds, world PK, shields, consumables |
| [design/FACTION_LEADERS.md](design/FACTION_LEADERS.md) | listed | Faction leaders |
| [design/GDD_LOGISTICS_SOCIETY.md](design/GDD_LOGISTICS_SOCIETY.md) | listed | Logistics and society pillar (GDD-LOGISTICS-SOCIETY-1) |
| [design/LEVELS_CANON.md](design/LEVELS_CANON.md) | listed | Levels and XP canon (LEVELS-CANON-1) |
| [design/PROFESSIONS_BENCHMARK.md](design/PROFESSIONS_BENCHMARK.md) | listed | Professions and societal teamwork notes (PROFESSIONS-BENCH-1) |
| [design/SERVER_MORALITY.md](design/SERVER_MORALITY.md) | listed | Server morality: good, evil and double agents |
| [design/UI_LAYOUT_SYSTEM.md](design/UI_LAYOUT_SYSTEM.md) | listed | UI layout system (design) |
| [faction-diplomacy-v14.5.md](faction-diplomacy-v14.5.md) | listed | Faction dynamics and diplomacy (v14.5) |
| [mercy-gates-powrush-integration.md](mercy-gates-powrush-integration.md) | listed | Mercy-Checks integration notes |
| [movement-reconciliation-v14.5.md](movement-reconciliation-v14.5.md) | listed | Movement, network prediction and server reconciliation (v14.5) |
| [rbe-implementation-v14.5.md](rbe-implementation-v14.5.md) | listed | RBE implementation and abundance mechanics (v14.5) |
| [rbe.md](rbe.md) | listed | Resource-based economy |
| [weekly-war-unlock-v14.5.md](weekly-war-unlock-v14.5.md) | listed | Weekly war unlock and dynamic content mechanics (v14.5) |
| `docs/visual-pass/` (12 `.md` files) | listed | Visual pass board and HUD map: [visual-pass/TARGET_BOARD.md](visual-pass/TARGET_BOARD.md), [visual-pass/HUD_SLAB_MAP.md](visual-pass/HUD_SLAB_MAP.md) |
| Repo root `*.md` (38 files) | listed | Not indexed one by one; see [DOC_CANON.md](DOC_CANON.md) and [archive/README.md](archive/README.md) |

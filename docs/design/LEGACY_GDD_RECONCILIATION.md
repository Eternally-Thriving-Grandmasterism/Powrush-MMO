# Legacy GDD reconciliation (v1.5–v2.1)

**Contact:** info@Rathor.ai

Design tick, not a Cargo bump. Title Online stays grey.

Docs only: a proposal, not a work order; nothing lands until Core names a slice.

Read against `main` at `4fc13bad`.

## 0. Purpose

This file extends [GDD_ADAPTATION.md](../GDD_ADAPTATION.md) (the 1.5–2.0 name map) through v2.1 and two story docs, section by section. It agrees with that map: no class tree at Title, no upgrade gambling, no crypto, no XP-from-kills, no race select, no Peace PK.

Canon order: [README](../../README.md), then the lived client (`cargo run -p powrush-client`), then current docs, then this file, then the legacy docs. Current canon wins every conflict in §6.

Every legacy section has one row. Verdicts are KEEP, MODERNISE or RETIRE. Where one old sentence mixes a fit and a miss, the verdict is the reshape or the drop, and the reason says which clause survives.

## 1. Sources and how the versions differ

The five legacy docs are not in this repo. This file summarises them in our own words. Short quotes appear only where the wording is the point.

- **GDD v1.5** (oldest). A working draft of about 45 PDF pages. Year 2048. Same section list as the later drafts. The guidelines block has no heading.
- **GDD v2.0.** Same length. Adds the guidelines heading, a server-database part and client timeouts. Drops the infinite-bank note and the battle-gambling note.
- **GDD v2.1** (newest, a docx). Year moves to 2037. Cell phones become player work. No section is added or removed.
- **The *Powrush Story* outline** (~680 words). Story beats in order. Not committed.
- **The *Galactic Reckoning* cinematic script** (~2.75k words, about five and a half minutes). A shot list. Not committed. See §5.

Each GDD carried a removed / added / suggestions colour legend, so all three were working drafts. About 95% of the wording is shared.

| Topic | v1.5 | v2.0 | v2.1 |
|---|---|---|---|
| Guidelines heading | None | Added | Kept |
| Year | 2048 | 2048 | 2037 |
| Cell phones | Surviving engineers already restored them | Same as v1.5 | Players salvage the net over low-frequency radio |
| Game servers | Capacity, a monthly champions copy, optional AFK drop | Adds a server database and client timeouts | Same as v2.0 |
| Loose end notes | Infinite bank; gamble on battles and a stream hook | Both dropped | Stay dropped |
| Classes and tables | Monk labels loose; footnotes | Weapon and armour lists reordered | Monk labels placed; footnotes removed; tables reformatted |
| Wording polish | — | — | "Take back control of Earth"; "stave off the alien invasion" |

v2.0 is the servers edit: the stack grew, and the gambling note left. v2.1 is a lore polish: an earlier year, and the comms net turned into player work.

## 2. Naming canon

The retired spelling in the first row is the only place this file uses it. Everywhere else, the old enemy people are Draeks.

| Retired / legacy name | Canon name | Source of canon |
|---|---|---|
| Draexx / Drax | **Draek** (the Draek Dominion; plural Draeks) | [GDD_ADAPTATION](../GDD_ADAPTATION.md), [PLAYABLE_RACES](../PLAYABLE_RACES.md) §3, [FACTIONS_OVERVIEW](../FACTIONS_OVERVIEW.md) |
| Drax mothership / "the Draek Mothership" | **The Brood Spire (TBS)** | [FACTIONS_OVERVIEW](../FACTIONS_OVERVIEW.md) |
| "The Council" (righteous faction) | **Quellorian / Aetherion Luminari Alliance** | [FACTIONS_OVERVIEW](../FACTIONS_OVERVIEW.md). "Council" stays the governance word: Local, Regional, Global in [COUNCIL_SYSTEM](../COUNCIL_SYSTEM.md), and guild leaders and/or their councils in [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §7.3 |
| Druid | **Cydruid** | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §5, [GDD_ADAPTATION](../GDD_ADAPTATION.md) |
| Quelorian | **Quellorian** | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §2 |
| Quelorian mothership | **The Auroral Unification Nexus (TAUN)** | [FACTIONS_OVERVIEW](../FACTIONS_OVERVIEW.md) |
| Drenadon | **Drenadore** | [DRIVE_LORE_ADAPTATION](../DRIVE_LORE_ADAPTATION.md). Also the Steam continuity reel in [STEAM_WISHLIST_PATSAGI_BRIEF_2026-09-22](../STEAM_WISHLIST_PATSAGI_BRIEF_2026-09-22.md). `client/src/first_session_guidance.rs` names Draek Depths and does not spell the home world. Stale [web-portal/index.html](../../web-portal/index.html) still says Drenadon (not edited here) |
| "Benevolent aliens" | **Quellorians**, with Ambrosians as the distant observers | [FACTIONS_OVERVIEW](../FACTIONS_OVERVIEW.md), [DRAEK_ORIGIN](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §7 |
| "Captain Veyra" (script) | **Veyra of the Crystal Choir**, High Resonance Keeper. Fleet command belongs to **Kaelith Starweaver, Grand Fleet Warden** | [QUELLORIAN_KEY_FIGURES](../QUELLORIAN_KEY_FIGURES.md) |
| Vylurian / Toruian | Unassigned name candidates. No sixth People | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §1.1 |

## 3. Verdict key and rulings applied

- **KEEP** — fits current canon as it stands.
- **MODERNISE** — keep the intent, reshape it to canon.
- **RETIRE** — drop it. The reason says why.
- **(Online · grey)** — designed now, built only after Sherif says "online yes".
- **→ COMBAT_AND_PVP.md** — combat detail lives in that file (same folder). It merged in #685 (`1316ddad`). This file adds no combat design.

Rulings applied:

- Current repo canon wins every conflict in §6, except the Cydruid home. Sherif's ruling of 2026-10-08, 12:34 AM ET, is the canon for that city. The older Place pages are flagged in §5 and are not rewritten here.
- Strong lore hooks stay where they fit: low gravity and big jumps; the four old peoples map onto five Peoples. The upside-down stalactite city is the Cydruid home in Earth's hollows, not scenery for Sanctuary, Heartwood, Threshold, or Depths. Draek and Alliance allegiance stay, with redemption open. Benevolent and evil aliens stay. Server timelines, time-travel resets, a war effort that opens servers, and per-server stories stay. Those server ideas are online-only and later.
- Pay-to-win and real-world-risk rows retire. That includes buying righteousness, buying NPC shops with real money, real-company shops and items, per-server crypto or player earnings, and paid server transfers that carry power. Real-money unbinding, random-skin subscriptions and battle gambling retire with them.
- Revenue follows no pay-to-win. Sherif's companies use Stripe and PayPal. Stripe is unused on itch.io; itch.io runs its own checkout. This file sets no price, SKU, storefront or tax treatment. Revenue never sells power. Revenue never sells looks (no transmog, no skins).
- Gear customisation is physical on that one item: paint, grips and wraps, engraving, a faction or House tabard, metal finish ([FACTION_HERALDRY](../FACTION_HERALDRY.md)). Cosmetic-skin ideas are flagged on M2, M4, J3, E9 and S2, with the physical alternative in the row.
- Combat rows defer: righteousness and criminality, PK modes, bounties, tournaments, guild wars, duels, teleport control, skill-shots, class combat kits.
- Redemption stays hopeful. Draeks are a playable People, with an open-lore hope (the fled sect). Enslaved species are victims to free. A whole people is not a burn list.
- The retired enemy spelling is used once, in §2, and then only as Draek.

## 4. Section-by-section

Columns: ID, old idea in our words (15 words or fewer), verdict, one-line reason, canon home.

### G — Overarching guidelines

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| G1 | Impactful quests on a base story the players extend | MODERNISE | Story is verbs, Places and server events, not a quest wall. | [LORE_BIBLE](../LORE_BIBLE.md) tone, [GDD_ADAPTATION](../GDD_ADAPTATION.md) |
| G2 | Competition, trade and diplomacy among players and factions | KEEP | Trade and diplomacy are the layer stack. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3c–§3d, [DIPLOMACY](../DIPLOMACY_AND_WORLD_SIMULATION.md) |
| G3 | Freedom to choose the experience, allies and enemies | KEEP | Faction is a guild allegiance a House can change, at a cost. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3d |
| G4 | Recurring PvP that pays loot and kill experience | MODERNISE (Online · grey) | Recurring contests can stay. Kill XP and loot pressure drop. → COMBAT_AND_PVP.md | [GDD_IMMERSION](../GDD_IMMERSION_REVISION.md) §4, [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md) |
| G5 | Fair play first, because the game centres on PvP | KEEP | Fairness is the anti-pay-to-win law. | [GDD_IMMERSION](../GDD_IMMERSION_REVISION.md) §4 |
| G6 | Safe trade zones, an AFK training area, and player trade | MODERNISE | Sanctuary stays protected. Trade is barter now; Market is HOLD. AFK training drops (no fake play). | [PLACES_BIBLE](../PLACES_BIBLE.md), [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §4 |
| G7 | A cryptocurrency that pays players for taking part | RETIRE | Player crypto earnings are pay-to-win. README refuses chain mint and NFT. | [README](../../README.md), [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3e |
| G8 | A criminality and righteousness score for player choices | MODERNISE | Standing moves by verbs. The fight rules defer. → COMBAT_AND_PVP.md | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3d, [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md) |

### P — Plot and Point Zero

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| P1 | Year 2037 (2048 in v1.5 and v2.0), after the fall | MODERNISE | The repo states no calendar year. 2037 stays a legacy note for a steward call. | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md) |
| P2 | A nuclear war shrank Earth, split it, then the halves rejoined | MODERNISE | Nukes were disabled first. Fracture came from kinetic, sun-lance and earthquake bombs. | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md) |
| P3 | Lower gravity after the war lets people jump far | KEEP | Low gravity and big jumps match the Space jump. | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md), [PATSAGI_v23_PLAYGROUND](../PATSAGI_v23_PLAYGROUND.md) |
| P4 | Evil aliens staged false flags that cascaded into world war | KEEP | WWIII is false flags. | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md) |
| P5 | Weaken humans, take Earth's resources, enslave the survivors | KEEP | Consumption and enslavement are the ethical collapse. Victims keep a way out. | [DRAEK_ORIGIN](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §4, [ENSLAVED_MINION_SPECIES](../ENSLAVED_MINION_SPECIES.md) |
| P6 | Players rush to form factions and take Earth back | KEEP | Houses and the Proof of Work Rush reading are that rush. Wordplay only; nothing mines. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §7.8 |
| P7 | Vigilantes and paid bounty hunters police criminals | MODERNISE (Online · grey) | Bounties defer. → COMBAT_AND_PVP.md | [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md) |
| P8 | Benevolent aliens offer aid to the Earthlings | KEEP | Those aliens are the Quellorian Alliance. | [FACTIONS_OVERVIEW](../FACTIONS_OVERVIEW.md) |
| P9 | Phones return after players salvage low-frequency radio | MODERNISE (Online · grey) | A salvaged comms net is a guild infrastructure project. v2.1's player-work version wins over v1.5. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3b |
| P10 | A rejoin canyon reveals an advanced hollow-Earth society | KEEP | The reveal stands. Their home is the hollows city (§5), not Depths dress. | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md) |
| P11 | Diplomacy or pillage, while holding off the invasion | KEEP | That split is the economy spectrum's success and failure paths. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §4.2–§4.3 |
| P12 | At level 1 a future person is sent back; mission still open | MODERNISE (Online · grey) | "From the future" can colour server timelines. It is never a Title wall. Heritage comes after House. | [GDD_ADAPTATION](../GDD_ADAPTATION.md), [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §7.6 |

### C — Camera and UI

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| C1 | Wide 3D isometric camera, cut at the horizon | MODERNISE | The Hour-1 camera is a readability contract. This file does not change the projection. | [PRE_RELEASE_LAW](../PRE_RELEASE_LAW.md) 7–9 |
| C2 | Curved health and resource, party frames, and an XP bar | MODERNISE (Online · grey) | One HUD, and no XP bar. Party frames wait. | [UI_LAYOUT_SYSTEM](UI_LAYOUT_SYSTEM.md), [GDD_IMMERSION](../GDD_IMMERSION_REVISION.md) §4 |
| C3 | Hotkeys for attacks, buffs, spells, items and emotes | MODERNISE | One Use verb. Combat hotkeys defer. → COMBAT_AND_PVP.md | [INPUT_CANON](../INPUT_CANON.md), [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md) |
| C4 | Bag, friends, PvP, guild, bounties, stats, broadcasts, trade desk | MODERNISE (Online · grey) | Menus follow the layout presets. Broadcasts and bounties wait. → COMBAT_AND_PVP.md | [UI_LAYOUT_SYSTEM](UI_LAYOUT_SYSTEM.md), [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md) |

### R — Peoples (old "races")

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| R1 | Hollow-Earth cyborgs whose "sorcery" is nanotech | KEEP | A Cydruid is a human in a cyborg frame. The age number is conflict 18. | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §5 |
| R2 | Xenki, eldest clairvoyant, foresaw the splitting of Earth | RETIRE | Not canon (Sherif, 2026-10-08). Xenki is not a named Cydruid elder. | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §5 |
| R3 | They share technology; extraction triggers natural disasters | KEEP | Cydruids share tech and harvest with care (Sherif 2026-10-08). Draek extraction stresses the land. | [DRAEK_ORIGIN](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §6 |
| R4 | Hooded robes; more machine parts as levels rise | KEEP | More machine is the Body range. The hood is dress. Level tiers are not the ladder. | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §5 |
| R5 | A black-market shop sells upgradeable body parts | MODERNISE | Upgrades come from Cydruid standing. "No shop and no second currency." | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §5 |
| R6 | A power source that requires giving up body parts | MODERNISE | The hook becomes a chosen point on the Body range, not a forced sacrifice. Sherif's "chose to become cyborgs" matches that. | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §5 |
| R7 | Upside-down stalactite city, with a dangerous spiral below | KEEP | Sherif 2026-10-08: this is the Cydruid home in the hollows, not dress on the four Places. No shop and no second currency. The Place question was resolved by Sherif 2026-10-08: no four-Place cap, the city may become a later Place, no PlaceId yet. | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §5 |
| R8 | A glowing healing lake and bobbing stepping-stones | KEEP | The glow-lake stays with that Cydruid home. The Heartwood pond bath is a separate lived teach. | [ART_BIBLE](../ART_BIBLE.md), [PLACES_BIBLE](../PLACES_BIBLE.md) |
| R9 | Dark minimal shop dress, holograms, neon glass | MODERNISE | That dress moves to a fabricator or bench. There is no shop. | [FACTION_HERALDRY](../FACTION_HERALDRY.md), [PLACES_BIBLE](../PLACES_BIBLE.md) |
| R10 | PvP hideout where rares cost the gang leader's credit | MODERNISE | Quest-credit black market is Market HOLD. Criminals in the abandoned stone can stay as flavour. The PvP zone defers. → COMBAT_AND_PVP.md | [OFFLINE_SKU](../OFFLINE_SKU.md), [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md) |
| R11 | Low zone raw-to-tech; high zone is a glow-worm sanctuary | KEEP | Those looks belong to the Cydruid hollows home (§5), not as Depths or Heartwood dress. | [ART_BIBLE](../ART_BIBLE.md) |
| R12 | Humans look like people now: determined, and they persist | KEEP | Humans stay the adaptable middle People. | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §4 |
| R13 | False-flag massacres, then footage of strange lizard creatures | MODERNISE | The false-flag war stands. Draeks read as biomechanical chitin, not lizard footage. | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md), [PLAYABLE_RACES](../PLAYABLE_RACES.md) §3 |
| R14 | Utilitarian, honest, inquisitive, strong, intelligent | KEEP | Honest and inquisitive fits the Quellorians. | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §2 |
| R15 | A two-sun homeworld, lost after they harnessed light | KEEP | Light-as-energy fits. The lost two-sun homeworld is canon (Sherif, 2026-10-08). | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §2, [DRAEK_ORIGIN](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §7 |
| R16 | Another alien people gave them a poorer new home | MODERNISE | Open-lore candidate: the Ambrosians gave that home. | [DRAEK_ORIGIN](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §7 |
| R17 | High-gravity bulk and strength; tools that serve many jobs | MODERNISE | Quellorians are tall, slender and graceful. The multi-purpose craft aesthetic stays. | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §2 |
| R18 | A two-tier caste of fertile people and the "unsexed" | RETIRE | The caste adds nothing canon needs, and it crowds the Draek female-loss story. A steward could reopen it. | [DRAEK_ORIGIN](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §3 |
| R19 | Strongly social: rescued people should stand together | KEEP | "Better together" is the unity theme. | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §2 |
| R20 | Faster-than-light merchants hauling goods across the galaxy | MODERNISE | Threshold throughput and logistics flavour. No server-minted gold. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3d, [OFFLINE_SKU](../OFFLINE_SKU.md) |
| R21 | Merchants who already know the enemy and send early aid | KEEP | The Quellorians are old foes of the Draeks. | [DRAEK_ORIGIN](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §5–§6 |
| R22 | The enemy people infiltrated Earth decades before play | KEEP | The long infiltration stands. They are also a playable People (conflict 2). | [DRAEK_ORIGIN](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §6, [PLAYABLE_RACES](../PLAYABLE_RACES.md) §3 |
| R23 | Max criminality opens their mothership and freezes decay | MODERNISE | The ship is the Brood Spire, never in Sanctuary. Access and decay defer. → COMBAT_AND_PVP.md | [FACTIONS_OVERVIEW](../FACTIONS_OVERVIEW.md), [PLACES_BIBLE](../PLACES_BIBLE.md), [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md) |

### T — Classification tree and systems appendix

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| T1 | Three playable races; the enemy people are not playable | MODERNISE | Five Peoples, Draek and Ambrosian included, chosen after House. | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §1.1 |
| T2 | Five starter classes with outside-game touchstones | RETIRE | Class trees at Title are refused. Practices replace classes. Kits defer. → COMBAT_AND_PVP.md | [GDD_ADAPTATION](../GDD_ADAPTATION.md), [GDD_IMMERSION](../GDD_IMMERSION_REVISION.md) §4 |
| T3 | Second-tier classes, including a disease-spreading kit | RETIRE | Same class-tree refuse. The disease kit is a taste flag for the combat paper, not adopted here. → COMBAT_AND_PVP.md | [GDD_IMMERSION](../GDD_IMMERSION_REVISION.md) §4, [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md) |
| T4 | Suggested fixes: cleric, xeno hunter, gambler, brawler | RETIRE | Suggested classes are the same refused tree, including the luck-based gambler. | [GDD_ADAPTATION](../GDD_ADAPTATION.md) |
| T5 | Four play paradigms, plus an engineer and a rideable colossus | MODERNISE (Online · grey) | The tool-user idea fits guild automation droids. Rideable power is not adopted. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3b, §6 |
| T6 | Fighters and warriors should have a self-heal | MODERNISE | Sustain may be a practice. It is not a class feature. → COMBAT_AND_PVP.md | [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md), [GDD_IMMERSION](../GDD_IMMERSION_REVISION.md) §4 |
| T7 | Five stats, weapon families, sockets, sets, a player-set name | MODERNISE | Stats defer. A player-set name and materials stay, on Lineage. → COMBAT_AND_PVP.md | [FACTION_HERALDRY](../FACTION_HERALDRY.md), [MERCY_TEMPER](../MERCY_TEMPER_PROGRESSION.md) |
| T8 | Weight changes speed, jump, dodge, damage and defence | MODERNISE | Jump height is the low-gravity world. Weight, dodge and damage defer. → COMBAT_AND_PVP.md | [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md), [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md) |
| T9 | Helm, chest, boots, gloves, pants, and weapon hands | MODERNISE | Slot list defers. No second paper-doll HUD. → COMBAT_AND_PVP.md | [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md), [GDD_IMMERSION](../GDD_IMMERSION_REVISION.md) §4 |
| T10 | Buy durable ammo and repair it, instead of counting rounds | MODERNISE | A repairable kit fits wear-and-maintain. Items are never deleted. | [FACTION_HERALDRY](../FACTION_HERALDRY.md), [MERCY_TEMPER](../MERCY_TEMPER_PROGRESSION.md) |
| T11 | Any people may take any craft; a salvager turns scrap into inputs | KEEP | Any People, any profession. Salvager fits tend and salvage. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3a |

### A — Art and technical requirements

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| A1 | Fully 3D, readable silhouettes, a simplified painted look | KEEP | Readable silhouettes and a slight exaggeration fit the readability law. The lived lane is gritty low-poly with high-detail light. Touchstone names stay off player-facing copy. | [ART_BIBLE](../ART_BIBLE.md), [PERSON_READ_SPEC](../PERSON_READ_SPEC.md) |
| A2 | Player characters around ten thousand triangles | MODERNISE | Triangle budgets are the Low / Medium / High mesh tiers. No flat ten-thousand figure lives there. | [MESH_QUALITY_BUDGET](../MESH_QUALITY_BUDGET.md) |

### S — Ascension

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| S1 | At max level, reclass or drop back to level 10 | MODERNISE | There is no level ladder and no reclass. The ascent idea lives in Ambrosian Mercy Ascent. | [AMBROSIAN_ASCENSION_MERCY_ASCENT](../AMBROSIAN_ASCENSION_MERCY_ASCENT.md) |
| S2 | A visible special effect marks ascended players | MODERNISE | Cosmetic flag: no projected glow-skin. A physical earned mark (engraving or tabard thread) joins Lineage. | [FACTION_HERALDRY](../FACTION_HERALDRY.md) |
| S3 | Gear scaled back to level 10, plus an auto-attack guardian | RETIRE | De-levelling gear and an auto-attack summon are a power scalar. | [GDD_IMMERSION](../GDD_IMMERSION_REVISION.md) §4 |
| S4 | Ascended players gain extra item-find, health and damage | RETIRE | Extra stats are a power scalar. | [GDD_IMMERSION](../GDD_IMMERSION_REVISION.md) §4, [MERCY_TEMPER](../MERCY_TEMPER_PROGRESSION.md) |

### E — Gear system and loot

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| E1 | Gear is not bind-on-equip; anything can be traded | KEEP | Person-to-person trade stands. Market stays HOLD. | [OFFLINE_SKU](../OFFLINE_SKU.md), [FACTION_HERALDRY](../FACTION_HERALDRY.md) |
| E2 | Bound rares can be unbound by a real-money item | RETIRE | Real-money unbinding sells power. | [GDD_IMMERSION](../GDD_IMMERSION_REVISION.md) §4, [FACTION_HERALDRY](../FACTION_HERALDRY.md) |
| E3 | A new gear version every ten levels, up to level 100 | RETIRE | The item-level treadmill drops. Temper +1…+9 is the ladder. | [MERCY_TEMPER](../MERCY_TEMPER_PROGRESSION.md) |
| E4 | At zero max durability the item is permanently destroyed | MODERNISE | Wear and repair stay. The item rests and is never destroyed. Combat wear defers. → COMBAT_AND_PVP.md | [MERCY_TEMPER](../MERCY_TEMPER_PROGRESSION.md), [FACTION_HERALDRY](../FACTION_HERALDRY.md) |
| E5 | Upgrade resource spent on a gamble, a sure small gain, or a coin-flip | RETIRE | Upgrade gambling is already refused. Temper costs care artifacts. | [GDD_ADAPTATION](../GDD_ADAPTATION.md), [MERCY_TEMPER](../MERCY_TEMPER_PROGRESSION.md) |
| E6 | A tiny per-try chance to add a socket, max five | MODERNISE | Lumen opens on its own at +3, +6 and +9. | [MERCY_TEMPER](../MERCY_TEMPER_PROGRESSION.md) |
| E7 | Blueprints from a mothership terminal or a PvP-city gamble | MODERNISE | Blueprints come through Embassy and standing. No combine fee. No gamble NPC. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3d, [MERCY_TEMPER](../MERCY_TEMPER_PROGRESSION.md) |
| E8 | A monk spell protects upgrade slots when an attempt fails | RETIRE | There is no destroy-on-fail, so there is nothing for an omen to protect. | [MERCY_TEMPER](../MERCY_TEMPER_PROGRESSION.md) |
| E9 | A second quality track; sets grant bonuses and show-off effects | MODERNISE | Cosmetic flag: no stat set bonuses. Set identity is physical finish on those items. | [FACTION_HERALDRY](../FACTION_HERALDRY.md) |
| E10 | A rare portable crafter with combine recipes | KEEP | Bench and fabricator recipes already teach this. | [MERCY_TEMPER](../MERCY_TEMPER_PROGRESSION.md) |
| E11 | Real-life brand items, with a tooltip about real-world use | RETIRE | Real-world marks and brands are refused. | [FACTION_HERALDRY](../FACTION_HERALDRY.md) |
| E12 | Named weapons earn kill reputation, then merge into a visible hybrid | MODERNISE | Keep the custom name and the visible merged part (a hilt on a bow). Retire kill-count soul stats. | [FACTION_HERALDRY](../FACTION_HERALDRY.md), [MERCY_TEMPER](../MERCY_TEMPER_PROGRESSION.md) |
| E13 | A gear piece looks the same on every race | KEEP | The look belongs to the item, not to a wardrobe swap. | [FACTION_HERALDRY](../FACTION_HERALDRY.md) |

### V — Game servers

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| V1 | One PC mega-server, a split mobile realm, sales in a database | MODERNISE (Online · grey) | The later shape is an event log and an integer double-entry ledger. Numbers there are targets. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §8 P2–P3 |
| V2 | A reconnect grace on special maps, waiting in a city | KEEP (Online · grey) | A grace period on a war map fits a later server. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §8 |
| V3 | Five hundred to two thousand concurrent players per server | MODERNISE (Online · grey) | Population figures are targets in the server budget, not a promise. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §8 |
| V4 | Monthly cross-server champions on copied characters | MODERNISE (Online · grey) | Champions can exist. Histories never merge. Rewards are titles and history lines, not power. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §7.6 |
| V5 | Maybe disconnect a player after thirty minutes AFK | KEEP (Online · grey) | An AFK timeout is a later server policy. | [ONLINE_LADDER](../ONLINE_LADDER.md) |
| V6 | Infinite bank space and a very large inventory | MODERNISE | v2.0 already dropped this note. Storage stays the satchel and House stores. The sketch's blanket online tag does not fit that offline form. | [OFFLINE_SKU](../OFFLINE_SKU.md), [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3a |
| V7 | Gamble on battles, and hook a live-stream | RETIRE | Battle gambling left in v2.0. It stays retired. Spectating is a later idea only, with no stake. | [GDD_ADAPTATION](../GDD_ADAPTATION.md) |

### J — Righteousness and criminality

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| J1 | Four attack stances: hostile, friendly, apprehend, passive | MODERNISE | Peace is the default and the hour. Lethal is opt-in after the book, on Ledger 3 DeclaredLethal. → COMBAT_AND_PVP.md | [GDD_IMMERSION](../GDD_IMMERSION_REVISION.md) §2.1, [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md) |
| J2 | Crime from kills, stolen XP, and gear drops on a criminal death | MODERNISE | Standing replaces the crime score. XP theft and drop-on-death pressure are out. → COMBAT_AND_PVP.md | [GDD_IMMERSION](../GDD_IMMERSION_REVISION.md) §4, [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md) |
| J3 | High crime invites a one-way join, plus a skin-title and a private channel | MODERNISE | Draek allegiance is standing, changeable at a cost, with redemption open. Cosmetic flag: faction gear is a physical tabard, not a skin. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3d, [FACTION_HERALDRY](../FACTION_HERALDRY.md), [REDEMPTION_MECHANICS_PER_SPECIES](../REDEMPTION_MECHANICS_PER_SPECIES.md) |
| J4 | High righteousness invites "The Council" to raid enemy holdings | MODERNISE | The faction path is the Quellorian / Aetherion Luminari Alliance, earned by standing. The name Council stays governance. | [FACTIONS_OVERVIEW](../FACTIONS_OVERVIEW.md), [COUNCIL_SYSTEM](../COUNCIL_SYSTEM.md) |
| J5 | Does the score survive ascension; can agents jump to earlier servers | MODERNISE (Online · grey) | Cross-timeline allegiance is a later timeline question. Ascension retention is moot without levels. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §7.6 |
| J6 | Full devotees stop decaying until good deeds lock the door | MODERNISE | Devotion does not freeze a one-way lock. Redemption can leave. → COMBAT_AND_PVP.md | [REDEMPTION_MECHANICS_PER_SPECIES](../REDEMPTION_MECHANICS_PER_SPECIES.md), [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md) |

### K — Skill-shots, guild wars, tournaments, duels

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| K1 | Jump-aware skill shots, meant to be fairer than the touchstone | MODERNISE | Jump-aware expression can stay. Balance defers. → COMBAT_AND_PVP.md | [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md) |
| K2 | Three daily free-for-alls in each ten-level bracket | MODERNISE (Online · grey) | Brackets can be a later server event. Item and gem prizes that add power drop. The offline door has no arena. → COMBAT_AND_PVP.md | [PRE_RELEASE_LAW](../PRE_RELEASE_LAW.md), [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md) |
| K3 | A half-week guild war; winners take every teleport fee | MODERNISE (Online · grey) | Weekly war contests stewardship (tons, restored, hold time). Fee income to a guild bank drops. → COMBAT_AND_PVP.md | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3c |
| K4 | Maze entry, a victory statue, and a holding room for the dead | MODERNISE (Online · grey) | Spectacle defers, including the no-loss waiting room. → COMBAT_AND_PVP.md | [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md) |
| K5 | Duels need a bought item; a public log; points buy gear | MODERNISE (Online · grey) | A public log can become a server-history line. A bought entry and a point shop drop. → COMBAT_AND_PVP.md | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §7.6, [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md) |
| K6 | Arena, field, or safe-zone duels where the loser drops rares | MODERNISE (Online · grey) | Sanctuary stays protected in every mode, so the safe-zone gear drop is out. Other venues defer. → COMBAT_AND_PVP.md | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3g, [PLACES_BIBLE](../PLACES_BIBLE.md) |

### L — Teleportation

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| L1 | Twelve guild-moved teleport portals, fee rising with level | MODERNISE (Online · grey) | Control of a portal grid defers. Travel that survives is the portal edition (metal ring, blue vortex) and Places. Waypoint networks are refused. → COMBAT_AND_PVP.md | [PRE_RELEASE_LAW](../PRE_RELEASE_LAW.md), [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md) |

### N — Talent point systems

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| N1 | One talent point per level across three trees and two specs | RETIRE | Talent trees and a talent panel are refused. | [GDD_IMMERSION](../GDD_IMMERSION_REVISION.md) §4 |

### M — Revenue without pay-to-win

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| M1 | Free, or a small box price plus a monthly fee | MODERNISE | Steam Offline is the first product. The full MMO is a second product. No subscription gate is designed here. Checkout constraint: Stripe and PayPal; itch.io uses its own checkout. No price is set. | [OFFLINE_SKU](../OFFLINE_SKU.md) |
| M2 | Optional purchases of appearance-changing skins | RETIRE | Cosmetic flag: skins and transmog for sale are out. Looks are physical workshop work, and they are not sold. | [FACTION_HERALDRY](../FACTION_HERALDRY.md) |
| M3 | Items modelled on real products under brand deals | RETIRE | Real-brand items are the same refuse as real-world marks, and a branded stat boost is pay-to-win. | [FACTION_HERALDRY](../FACTION_HERALDRY.md) |
| M4 | A subscription that grants random skins on a schedule | RETIRE | Cosmetic flag: a random-skin subscription is a loot-box shape and a transmog shop. | [FACTION_HERALDRY](../FACTION_HERALDRY.md) |

### X — Expansion plans

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| X1 | More explorable planets later, with character import | KEEP (Online · grey) | Further worlds are Sky and fleet content after Online. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3f |
| X2 | A deeper bridge story that folds in lessons from the first | KEEP | Later story can learn from the hour. It does not reopen walked slices. | [LORE_BIBLE](../LORE_BIBLE.md) |
| X3 | A benevolent people helps humanity become interstellar | KEEP | Ambrosians already fill that role, on the late branch called the Ambrosian Awakening. | [DRAEK_ORIGIN](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §7 |

### D — Game development notes

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| D1 | NPCs guide, and the hour can finish without them | KEEP | Persons never gate the hour. | [NPC_SCHEDULE_SPEC](../NPC_SCHEDULE_SPEC.md) |
| D2 | Enemy aliens are placed with a purpose, never as filler | KEEP | Placement stays purposeful. | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md) |
| D3 | Enemies tall and skinny; benevolent aliens buff and bulky | MODERNISE | Draeks are tall, imposing and biomechanical. Quellorians are tall, slender and graceful. | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §2–§3 |
| D4 | Wide customisation, with female, male and neutral | KEEP | Heritage after House, the Cydruid Body range, and inclusive genders. | [GDD_ADAPTATION](../GDD_ADAPTATION.md), [PLAYABLE_RACES](../PLAYABLE_RACES.md) §5 |
| D5 | Many outcomes; preferably server events instead of quests | KEEP | The Crownstone trilemma is the many-outcome beat. Shared projects are the server-wide events. | [CROWNSTONE_TRILEMMA_PATHS](../CROWNSTONE_TRILEMMA_PATHS.md), [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §7.4 |
| D6 | A short list of unused alien names beside the retired spelling | MODERNISE | The retired spelling maps to Draek (see §2). Vylurian and Toruian stay unassigned. No sixth People. | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §1.1 |
| D7 | Town shops, plus a free market aboard the mothership | MODERNISE | Stores become bench, fabricator and Embassy. Market stays HOLD. A mothership is not the hub. | [OFFLINE_SKU](../OFFLINE_SKU.md), [PLACES_BIBLE](../PLACES_BIBLE.md) |
| D8 | Take over unnamed NPC stores with real money or PK; they are real companies | RETIRE | Real-money or PK shop ownership, and real-company storefronts, are out. | [FACTION_HERALDRY](../FACTION_HERALDRY.md), [GDD_IMMERSION](../GDD_IMMERSION_REVISION.md) §4 |
| D9 | Many town gates and massive guards, the strongest monsters | MODERNISE | Sanctuary protection is law. Guards, if they return, are scheduled persons, not a damage wall. | [PLACES_BIBLE](../PLACES_BIBLE.md), [NPC_SCHEDULE_SPEC](../NPC_SCHEDULE_SPEC.md) |
| D10 | Benevolent time-travel resets grant server-wide buffs | KEEP (Online · grey) | A reset may colour a season or a mood. It does not grant a power scalar. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §7 |
| D11 | A second-chance name, marriage for anyone, almost no filler quests | KEEP (Online · grey) | The name's second chance is the RUSH reading. Filler quests stay out. Marriage is later social. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §7.8 |

### O — Other servers (time travel, war effort)

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| O1 | New servers restart at launch and replay past events | KEEP (Online · grey) | Timeline-reset servers replaying history fit a later online. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §7.6 |
| O2 | Pay for the right to move servers, with caps on what comes | RETIRE | A paid transfer that carries power is pay-to-win. | [GDD_IMMERSION](../GDD_IMMERSION_REVISION.md) §4 |
| O3 | A radioactive nuke blocks travel into one's own future | MODERNISE (Online · grey) | One-way travel into the past can stay. The nuke cause conflicts with nukes-disabled. The cause is a steward lore call. | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md) |
| O4 | Everyone's war effort opens a server; early contributors get a trial | KEEP (Online · grey) | Contributions are in-game work (tons and restored), never money. The reward is entry and a history line, not power. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3c, §7.6 |
| O5 | A transfer is one-way, decided at the portal | KEEP (Online · grey) | One-way entry at a portal fits timeline resets. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §7.6 |
| O6 | Each server can grow a completely different story | KEEP (Online · grey) | Two servers' histories never merge. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §7.6 |

### Y — Donating to a city

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| Y1 | Donate money to a city to raise your righteousness | RETIRE | Paying for righteousness sells standing. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3d |
| Y2 | The city with the most donations evolves, or donations fund a new one | MODERNISE | A city changes by community vote and shared projects, funded by in-game contribution only. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §7.3–§7.5 |

### Z — Crypto note

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| Z1 | Each server needs its own cryptocurrency so outside money cannot flood it | RETIRE | The old note's own worries (inflation, power transfer, pay-to-win, transfer shocks) are the reason. One ledger, no token. | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3e, [README](../../README.md) |

## 5. Story and lore

The *Galactic Reckoning* script is source material for future trailers and cutscenes. Homes: [IMAGINE_TRAILER_PACK.md](../IMAGINE_TRAILER_PACK.md), [ART_BIBLE.md](../ART_BIBLE.md), and the steward's Imagine pack. It is not a design doc, and it is not committed. The *Powrush Story* outline is the same kind of source. `GRv` is the look the script assumes, not a fourteenth scene.

HANDS stills stay free of eat-flesh, teleport-beam shots and a mothership bay ([IMAGINE_TRAILER_PACK](../IMAGINE_TRAILER_PACK.md), [ART_BIBLE](../ART_BIBLE.md) C-12).

### Cydruid home (Sherif, 2026-10-08, 12:34 AM ET)

This ruling overrides the earlier row that parked the upside-down stalactite city as dress on Heartwood or Depths.

The city is the home of the Cydruids. They live deep underground in the hollows of the Earth. They quietly advance beyond all humans by harvesting resources thoughtfully and carefully. They are an endlessly burrowing culture of humans and robots that eventually merged into one: druids who chose to become cyborgs, the Cydruids.

Still held, and not reopened: no Cydruid shop, and no second currency ([PLAYABLE_RACES](../PLAYABLE_RACES.md) §5). Frame upgrades come through standing, along the Body range.

Checked against Cydruid canon already on main. Fits:

- A Cydruid is a human housed in a cyborg frame, nature as practice, not a treant ([PLAYABLE_RACES](../PLAYABLE_RACES.md) §5, steward C0 and the 2026-10-02 Body range). "Chose to become cyborgs" matches a chosen point on that range, including Fully cyborg.
- Hidden Cydruids already "quietly advance their technology in Earth's hollows" ([DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md)). This ruling confirms the hollows life and the careful harvest.
- Thoughtful harvest matches tend and mercy. It is not Draek extraction.

Clashes, flagged here and not edited in those files:

- [PLAYABLE_RACES](../PLAYABLE_RACES.md) §1.1 (2026-09-19) lands the Cydruid door in Heartwood and says four Places only, no fifth Place. That Place question was resolved by Sherif 2026-10-08: no four-Place cap, the city may become a later Place, no PlaceId yet. [PLACE_DRESS_SPEC](../PLACE_DRESS_SPEC.md) says the same four. This ruling says the city is not scenery for Sanctuary, Heartwood, Threshold, or Depths. This file does not add a Place, a door, or a mesh.
- [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md) sends the rise to Depths dress. Depths remains the Draek door in [PLAYABLE_RACES](../PLAYABLE_RACES.md) §1.1. The burrow city is not that landing. The hollows sentence in Drive lore is still marked LORE-ORIGINS, unconfirmed (2026-10-02). This ruling confirms it. That file is unchanged.
- "Humans and robots that eventually merged" is taken with Sherif's own gloss: druids who chose cyborg frames, one People. A separate robot People would be a sixth People and is not taken.
- "Beyond all humans" is a head start from careful harvest. It does not close the human path: humans can still earn the Fully cyborg end by Cydruid standing, and the two Peoples can converge ([PLAYABLE_RACES](../PLAYABLE_RACES.md) §4–§5). It is not a power scalar, a shop, or a second currency.

### SL — *Powrush Story* outline

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| SL1 | Mind-controlled scouts abduct humans as a clone-harvest species | MODERNISE | Minion scouts can find Earth. Abduction stays clinical: incapacitate, then a ship or a portal. | [ENSLAVED_MINION_SPECIES](../ENSLAVED_MINION_SPECIES.md), [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md) |
| SL2 | Drenadore died of wars caused by an immature, deceptive nature | MODERNISE | Drenadore is a red tomb-world and does not explode on screen. The "immature by nature" cause fights the peaceful matriarchal past. | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md), [DRAEK_ORIGIN](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §2 |
| SL3 | Eugenics extinguished Draek women; cloning yields only males | KEEP | Keep as hypothesis beside "experiments gone wrong" and the open-lore genetics origin. Do not harden it. The fled sect stays hope. | [DRAEK_ORIGIN](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §3 |
| SL4 | Through a window, the ruined world, and people scrambling aboard | KEEP | The dying-world exodus is a trailer image. | [IMAGINE_TRAILER_PACK](../IMAGINE_TRAILER_PACK.md) |
| SL5 | The most corrupt fraction takes the escape seats to the mothership | KEEP | That elite fits the ethical collapse. The ship is the Brood Spire. | [DRAEK_ORIGIN](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §4, [FACTIONS_OVERVIEW](../FACTIONS_OVERVIEW.md) |
| SL6 | Mind control of named great powers starts a war of missiles and nukes | MODERNISE | Crownstone mind control stands. Named real states leave game content. The nuke beat is conflict 1. | [FACTIONS_OVERVIEW](../FACTIONS_OVERVIEW.md), [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md) |
| SL7 | Magma islands, debris, a moon scrape, chunks spinning off | KEEP | Fracture spectacle stays. New land can be frontier hex flavour. | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md), [ART_BIBLE](../ART_BIBLE.md) |
| SL8 | Weaker gravity, so people jump higher and further | KEEP | Same hook as P3. | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md) |
| SL9 | Cydruids rise from the hollows after more than five thousand years | MODERNISE | The hollows home is Sherif 2026-10-08 (§5). The year count is still unset. Steward call on the number only. | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md) |
| SL10 | Wrist light is weapon and gathering tool; no biological body, fuel not food | MODERNISE | Wrist light as weapon and gathering tool fits the trailer. "No biological body" is only the Fully cyborg end of the Body range. | [IMAGINE_TRAILER_PACK](../IMAGINE_TRAILER_PACK.md), [PLAYABLE_RACES](../PLAYABLE_RACES.md) §5 |
| SL11 | A scout relay calls help only after nuclear-scale destruction, and spots the mothership late | MODERNISE | Quellorians already monitored and made covert contact. The relay is the shift to open aid. The trigger is fracture weapons. | [DRAEK_ORIGIN](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §6 |
| SL12 | The Quellorian mothership arrives by wormhole and deploys aid | KEEP | TAUN arrives through a wormhole. | [FACTIONS_OVERVIEW](../FACTIONS_OVERVIEW.md) |
| SL13 | They eat flesh, shapeshift into leaders, or spend captives as clone fuel | RETIRE | Eat-flesh and shapeshift-into-leaders are refused. The puppet-leader beat survives as Crownstone mind control (SL6). | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md), [FACTIONS_OVERVIEW](../FACTIONS_OVERVIEW.md) |
| SL14 | Shields up, negotiate first, and only then arm interceptors | KEEP | Negotiate first is a mercy-first beat. | [QUELLORIAN_KEY_FIGURES](../QUELLORIAN_KEY_FIGURES.md), [IMAGINE_TRAILER_PACK](../IMAGINE_TRAILER_PACK.md) |

### GR — *Galactic Reckoning* script

| ID | Old idea | Verdict | One-line reason | Canon home |
|---|---|---|---|---|
| GR1 | Fall of the home world; a male-only hologram; cloning hungers; "claim Earth" | MODERNISE | Trailer source. Spelling is Drenadore. No on-screen explosion. Hunger for flesh stays off screen. | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md), [IMAGINE_TRAILER_PACK](../IMAGINE_TRAILER_PACK.md) |
| GR2 | Exodus under plasma whips; "only the strong"; let the home world rot | MODERNISE | The scramble stays. Home-world spelling is Drenadore. The line is despair, not a species sentence. | [DRAEK_ORIGIN](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §3–§4 |
| GR3 | Aboard ship, cloning vats fail and someone demands flesh | MODERNISE | Codex or trailer only. The bay stays out of HANDS stills. The shortage is clinical biomass, not an eat shot. | [ART_BIBLE](../ART_BIBLE.md), [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md) |
| GR4 | A jagged red-scarred mothership, swarm in tow, leader on a throne | MODERNISE | The ship is the Brood Spire: tall, biomechanical, oppressive. | [FACTIONS_OVERVIEW](../FACTIONS_OVERVIEW.md) |
| GR5 | Consumes an official in a real capital, shapeshifts, enters the government seat | MODERNISE | Consumption, shapeshift and the real capital drop. Infiltration survives as Crownstone puppetry of unnamed leaders. | [FACTIONS_OVERVIEW](../FACTIONS_OVERVIEW.md), [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md) |
| GR6 | Nuclear blasts, fracture, magma islands, a moon scrape, a mother jumps rubble | MODERNISE | Fracture, low gravity and the jump stay. Nuclear blasts do not. | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md) |
| GR7 | Polished-metal Cydruids rise; "for five millennia"; "the invaders will burn" | MODERNISE | Cydruids are humans in frames, not an all-metal species. Soften the burn line. Redemption stays open. Age is conflict 18. | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §5, [REDEMPTION_MECHANICS_PER_SPECIES](../REDEMPTION_MECHANICS_PER_SPECIES.md) |
| GR8 | A scout sends the relay while the enemy mothership looms unseen | KEEP | Usable as the moment covert watch becomes an open call (see SL11). | [DRAEK_ORIGIN](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §6 |
| GR9 | "Captain Veyra" on the bridge: shield, sword and salvation | MODERNISE | Bridge command is Kaelith Starweaver, Grand Fleet Warden. Veyra stays High Resonance Keeper. | [QUELLORIAN_KEY_FIGURES](../QUELLORIAN_KEY_FIGURES.md) |
| GR10 | Split-screen parley, then Veyra's "Then burn." | MODERNISE | Keep the parley. "Then burn." clashes with mercy; Kaelith's doctrine is restraint and boarding over pure destruction. | [QUELLORIAN_KEY_FIGURES](../QUELLORIAN_KEY_FIGURES.md) |
| GR11 | Humans, Cydruids and Quellorians fight as one; syringe harvest; an energy blade | KEEP | Three Peoples stand as one. Harvest on screen stays clinical, not feeding gore. | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md), [ART_BIBLE](../ART_BIBLE.md) |
| GR12 | Enemy ships flee; "Earth is free. Let them run." | MODERNISE | Use it as a first-wave repulse. The war continues (Brood Spire, Crownstone trilemma). "Let them run" can stay as restraint. | [CROWNSTONE_TRILEMMA_PATHS](../CROWNSTONE_TRILEMMA_PATHS.md), [FACTIONS_OVERVIEW](../FACTIONS_OVERVIEW.md) |
| GR13 | Sunrise rebuilding, aid drones, laser tools, "a new era" | KEEP | Rebuilding and unity fit the tone of a shared new era. | [LORE_BIBLE](../LORE_BIBLE.md), [ART_BIBLE](../ART_BIBLE.md) |
| GRv | Towering male reptilian Draeks with scales, claws and tails; ethereal glowing scouts | MODERNISE | Draeks are biomechanical chitin, not scaled reptiles with tails. Ethereal, glowing-eyed Quellorian scouts can stay. | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §2–§3 |

## 6. Conflicts with current canon

Canon wins each row. Row 6 is Sherif's 2026-10-08 ruling. The older four-Places pages stay as written and are flagged in §5.

| # | Old claim | Current canon | Winner |
|---|---|---|---|
| 1 | A nuclear war broke Earth (P2, O3, SL6, GR6) | Earth nukes were disabled first. WWIII is false flags plus kinetic, sun-lance and earthquake bombs. Nuke-on-Earth is refused | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md) |
| 2 | Three playable races; Draeks are non-playable villains (T1, and R22 read as villains only) | Five playable Peoples, Draek and Ambrosian included, chosen after House | [PLAYABLE_RACES](../PLAYABLE_RACES.md) |
| 3 | One-way Draek allegiance with no exit (J3) | Allegiance can change, at a cost. Redemption stays open | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3d, [REDEMPTION_MECHANICS_PER_SPECIES](../REDEMPTION_MECHANICS_PER_SPECIES.md) |
| 4 | "The Council" as the righteous faction (J4) | Council means governance tiers. The faction is the Quellorian / Aetherion Luminari Alliance | [COUNCIL_SYSTEM](../COUNCIL_SYSTEM.md), [FACTIONS_OVERVIEW](../FACTIONS_OVERVIEW.md) |
| 5 | A black-market body-part shop (R5) | Cydruid upgrades come through standing. "No shop and no second currency." | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §5 |
| 6 | A Druid capital the size of a new region (R7), earlier read here as dress on Heartwood or Depths | Written Place law still says four Places, Cydruid door to Heartwood, rise as Depths dress | Sherif 2026-10-08: the stalactite city is the Cydruid home, not scenery for those four Places. The Place question was resolved by Sherif 2026-10-08: no four-Place cap, the city may become a later Place, no PlaceId yet. Those docs are not edited |
| 7 | Draeks as lizards or scaled reptiles with tails (R13, GRv, and the public site) | Draeks are tall biomechanical humanoids with chitin. Vesh'kar are a different enslaved people and are the reptile minions | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §3 and §7 |
| 8 | Allies buff and bulky; enemies tall and skinny; high-gravity bulk (D3, R17) | Quellorians are tall, slender and graceful. Draeks are tall, imposing and biomechanical | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §2–§3 |
| 9 | Draeks were deceptive warmongers by nature (SL2) | They were a peaceful matriarchal people who fell | [DRAEK_ORIGIN](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §2–§4 |
| 10 | Eat flesh and shapeshift into leaders (SL13, GR5) | Clinical harvest only. Never eat on screen. Mind control is the Crownstone | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md), [FACTIONS_OVERVIEW](../FACTIONS_OVERVIEW.md) |
| 11 | Quellorians discover the invasion late (SL11, GR8) | They monitored the Draeks for a long time and made covert contact first | [DRAEK_ORIGIN](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §6 |
| 12 | Veyra commands the mothership (GR9–GR12) | Veyra is High Resonance Keeper. Fleet command is Kaelith Starweaver | [QUELLORIAN_KEY_FIGURES](../QUELLORIAN_KEY_FIGURES.md) |
| 13 | "Earth is free" ends the war (GR12) | The war continues: Brood Spire, Crownstone trilemma | [FACTIONS_OVERVIEW](../FACTIONS_OVERVIEW.md), [CROWNSTONE_TRILEMMA_PATHS](../CROWNSTONE_TRILEMMA_PATHS.md) |
| 14 | Home world spelled Drenadon (script, and the public site) | Drenadore, a red tomb-world that does not explode on screen. The lived guidance file does not spell it | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md) |
| 15 | Items are destroyed at zero durability (E4) | Items are never deleted. No destroy-on-fail. A failed temper rests | [MERCY_TEMPER](../MERCY_TEMPER_PROGRESSION.md), [FACTION_HERALDRY](../FACTION_HERALDRY.md) |
| 16 | Twelve movable teleport portals (L1) | Waypoint networks are refused. Travel that stands is the portal edition | [PRE_RELEASE_LAW](../PRE_RELEASE_LAW.md), [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md) |
| 17 | Cosmetic skins for sale, and a skin subscription (M2, M4) | Physical finishes only. No cosmetic shop. No transmog | [FACTION_HERALDRY](../FACTION_HERALDRY.md) |
| 18 | Cydruid age "hundreds of years" (R1) versus "over 5,000 years" (SL9, GR7, public site) | Repo docs do not fix a number | Neither legacy count. Steward call |

### Stale spots left in place

Not edited in this PR:

- [web-portal/index.html](../../web-portal/index.html) still says male-only reptilian invaders from dying Drenadon, Cydruids underground for over 5,000 years, and cloning vats that hunger for biomass.
- `client/src/first_session_guidance.rs` names Draek and Depths. It does not spell Drenadore. The spelling is in [DRIVE_LORE_ADAPTATION](../DRIVE_LORE_ADAPTATION.md) and the Steam brief.
- [GDD_ADAPTATION](../GDD_ADAPTATION.md) still says "Post-book skins" on the cosmetics transform row. [FACTION_HERALDRY](../FACTION_HERALDRY.md) is the later law: physical finishes, no transmog, no cosmetic shop.
- [PLAYABLE_RACES](../PLAYABLE_RACES.md) §2–§3 still speaks of unity bonuses, converting the defeated, and "massive power", beside the no-power-scalar law in [GDD_IMMERSION](../GDD_IMMERSION_REVISION.md) §4.
- The same file's §5 says a starting Cydruid picks one free cyborg upgrade at character creation. §1.1 of that file puts People after House, with no Title race lobby.
- The same §1.1 lands Cydruids in Heartwood and forbids a fifth Place. That Place question was resolved by Sherif 2026-10-08: no four-Place cap, the city may become a later Place, no PlaceId yet. [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md) still sends their rise to Depths dress. Sherif 2026-10-08 names the stalactite city as the Cydruid home instead (§5). Those files were not edited.

## 7. Retired, and why

**Pay-to-win.** G7 and Z1 (crypto earnings, a per-server coin). E2 (real-money unbinding). E5 (upgrade gambling). Y1 (buy righteousness). O2 (paid transfers that carry power). D8 (buy a shop). M3 where a brand grants stats. V7 (battle gambling).

**Legal / real-world risk.** E11 and M3 (real brands and real-world marks). D8 (real-company stores). SL6 and GR5 (named real states, a real capital, a real official). V7 (staking on fights, plus a stream hook).

**Breaks a canon law.** E4 and E8 (destroy-on-fail). G4, J2 and the soul half of E12 (XP-from-kills and kill-count stats). M2, M4, S2's glow, J3's skin-title and E9's show-off set effects (transmog; the physical alternative is paint, wrap, engraving, tabard, metal finish). T2, T3, T4 and N1 (class or talent tree at Title). L1 (waypoint network). SL13 and GR5 (eat-flesh and shapeshift). P2, O3, SL6 and GR6 (nuke-on-Earth). S3 and S4 (a power scalar).

## 8. Online · grey register

Designed now, built only after Sherif says "online yes". See [ONLINE_LADDER](../ONLINE_LADDER.md) R0–R7 and [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §8–§9. Title Online stays grey. Default NetMode stays Offline.

- G4 — recurring player contests, without kill XP or loot pressure
- P7 — bounties
- P9 — player-salvaged comms as guild infrastructure
- P12 — a future-person note on server timelines, never a Title wall
- C2 — party frames (the XP bar is an offline refuse, not a later feature)
- C4 — public cell broadcasts and bounties
- T5 — guild automation droids
- V1 — server store and ledger shape
- V2 — reconnect grace on special maps
- V3 — population budgets (targets)
- V4 — cross-server champions, titles and history lines, histories never merge
- V5 — AFK disconnect policy
- J5 — allegiance questions across timelines
- K2 — daily brackets
- K3 — weekly stewardship war (fee-to-bank income stays retired)
- K4 — war spectacle and the waiting room
- K5 — duel log as a history line
- K6 — duel venues other than a Sanctuary gear drop (that drop stays retired)
- L1 — who may move a portal, if anyone; local travel stays the portal edition
- X1 — further worlds, Sky and fleet
- D10 — timeline-reset mood, not a power buff
- D11 — marriage and later social (the name and the no-filler rule are offline)
- O1 — timeline-reset servers
- O3 — one-way travel into the past (cause is a steward lore call)
- O4 — war effort opens a server by in-game work
- O5 — one-way portal entry
- O6 — divergent per-server stories

V6 is not in this list. v2.0 dropped the infinite bank, and the kept form is the offline satchel. V7, O2 and the other retired pay-to-win rows are not designed forward.

## 9. Combat pointer

These old sections defer to [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md) (merged in #685): righteousness and criminality (J1, J2, J6, G8, R23), bounties (P7, C4), skill-shots (K1), tournaments (K2), guild wars (K3, K4), duels (K5, K6), teleport control (L1), class kits (T2, T3, T6), weight and dodge (T8), armour slots (T9), and durability in a fight (E4). Verdicts above are what canon already implies. This file does not design the fights.

## 10. Revive next

Ideas only. Core names any later work. This table is not a queue.

Ranked by fit, then by lower effort.

| Rank | Idea | Fit | Effort | Canon home | Gate |
|---|---|---|---|---|---|
| 1 | One lore line that ties low gravity and the big jump to the existing Space jump | High | Tiny | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md), [PATSAGI_v23_PLAYGROUND](../PATSAGI_v23_PLAYGROUND.md) | Offline now |
| 2 | Fracture aftermath as dress: magma-born islands, a scarred moon, debris as frontier hex flavour | High | Low | [DRIVE_LORE](../DRIVE_LORE_ADAPTATION.md), [ART_BIBLE](../ART_BIBLE.md) | Offline now |
| 3 | Negotiate-first beat (shields up, open a channel) for a trailer and a diplomacy moment | High | Low | [IMAGINE_TRAILER_PACK](../IMAGINE_TRAILER_PACK.md), [QUELLORIAN_KEY_FIGURES](../QUELLORIAN_KEY_FIGURES.md) | Offline now |
| 4 | Crafted-item names plus a visible merged part (a hilt on a bow), physical, no stats | High | Medium | [FACTION_HERALDRY](../FACTION_HERALDRY.md), [MERCY_TEMPER](../MERCY_TEMPER_PROGRESSION.md) | Offline now |
| 5 | Upside-down stalactite city as the Cydruid home in the hollows, with the glow-lake; not dress on the four Places. No shop, no second currency. Place count resolved by Sherif 2026-10-08: no four-Place cap, the city may become a later Place, no PlaceId yet | High | Medium-high | This file §5, [PLAYABLE_RACES](../PLAYABLE_RACES.md) §5 | Offline now |
| 6 | Draek and Alliance allegiance as faction standing, redemption open | High | High | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3d, [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md) | Online · grey |
| 7 | Timeline-reset servers, a work-only war effort, divergent stories, histories never merge | High | Very high | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §7.6 | Online · grey |
| 8 | Quellorian origin: the lost two-sun home is canon (Sherif, 2026-10-08), Ambrosians gave a new one, Crystal Choir bond | Medium-high | Low | [DRAEK_ORIGIN](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §7, [QUELLORIAN_KEY_FIGURES](../QUELLORIAN_KEY_FIGURES.md) | Canon (Sherif, 2026-10-08) |
| 9 | Xenki, clairvoyant Cydruid elder, as a named figure | Medium | Low | [PLAYABLE_RACES](../PLAYABLE_RACES.md) §5 | Not canon (Sherif, 2026-10-08) |
| 10 | Salvager gathering, and ammo as a repairable kit | Medium | Medium | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3a, [FACTION_HERALDRY](../FACTION_HERALDRY.md) | Offline now |
| 11 | Player-salvaged comms net as a guild infrastructure project | Medium | High | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §3b | Online · grey |
| 12 | Cross-server champions with titles and history lines, no power | Medium | High | [LAYERED](../LAYERED_GAMES_AND_SERVER.md) §7.6 | Online · grey |

## 11. Relates

Parent: [GDD_ADAPTATION](../GDD_ADAPTATION.md).

Names and peoples: [PLAYABLE_RACES](../PLAYABLE_RACES.md), [FACTIONS_OVERVIEW](../FACTIONS_OVERVIEW.md), [QUELLORIAN_KEY_FIGURES](../QUELLORIAN_KEY_FIGURES.md), [ENSLAVED_MINION_SPECIES](../ENSLAVED_MINION_SPECIES.md), [REDEMPTION_MECHANICS_PER_SPECIES](../REDEMPTION_MECHANICS_PER_SPECIES.md), [SPECIFIC_REDEMPTION_QUESTS](../SPECIFIC_REDEMPTION_QUESTS.md), [DISCORDANT_REDEMPTION_QUESTLINES](../DISCORDANT_REDEMPTION_QUESTLINES.md).

Lore and picture: [DRIVE_LORE_ADAPTATION](../DRIVE_LORE_ADAPTATION.md), [DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md), [LORE_BIBLE](../LORE_BIBLE.md), [PLACES_BIBLE](../PLACES_BIBLE.md), [PLACE_DRESS_SPEC](../PLACE_DRESS_SPEC.md), [ART_BIBLE](../ART_BIBLE.md), [IMAGINE_TRAILER_PACK](../IMAGINE_TRAILER_PACK.md), [PHYSICS_GRAPHICS_CANON](../PHYSICS_GRAPHICS_CANON.md).

Hour and net: [GDD_IMMERSION_REVISION](../GDD_IMMERSION_REVISION.md), [PRE_RELEASE_LAW](../PRE_RELEASE_LAW.md), [ONLINE_LADDER](../ONLINE_LADDER.md), [LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md), [OFFLINE_SKU](../OFFLINE_SKU.md), [INPUT_CANON](../INPUT_CANON.md), [UI_LAYOUT_SYSTEM](UI_LAYOUT_SYSTEM.md), [NPC_SCHEDULE_SPEC](../NPC_SCHEDULE_SPEC.md), [COUNCIL_SYSTEM](../COUNCIL_SYSTEM.md).

Gear: [FACTION_HERALDRY](../FACTION_HERALDRY.md), [MERCY_TEMPER_PROGRESSION](../MERCY_TEMPER_PROGRESSION.md), [MESH_QUALITY_BUDGET](../MESH_QUALITY_BUDGET.md), [PERSON_READ_SPEC](../PERSON_READ_SPEC.md), [AMBROSIAN_ASCENSION_MERCY_ASCENT](../AMBROSIAN_ASCENSION_MERCY_ASCENT.md).

Combat, merged in #685: [COMBAT_AND_PVP.md](COMBAT_AND_PVP.md).

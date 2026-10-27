# PROFESSIONS-BENCH-1: professions benchmark (social + societal teamwork)

Docs only. No code, assets or economy code. Online stays grey. No certification, legal or health claims.

Base: main `5dba1723` (after #782 AUDIO-RADIO-QUEST-DESIGN-1). Bounds: issue #738, comment 6431512884 (card named by Core). Feeds the Engineering path in [`AUDIO_RADIO_QUEST_DESIGN.md`](AUDIO_RADIO_QUEST_DESIGN.md) §4–§5. Style follows [`docs/visual-pass/playtest-ui-bench-1/BENCHMARK.md`](../visual-pass/playtest-ui-bench-1/BENCHMARK.md).

## 0. Ground rules and source/bounds map

- Every claim about a reference game links a page that was fetched on 2026-10-26 (America/Toronto) and loaded, unless marked **UNVERIFIED**. Gaps were not filled from memory.
- Patterns are written in our own words. Quotes are short. No art, assets, names or text copied.
- Some official wikis refused our fetcher (HTTP 403 or "maintenance"): lotro-wiki.com, wiki.albiononline.com, wiki.guildwars2.com (maintenance page), swg.fandom.com. Where that happened we used another real page that did load (official news, Wikipedia, a community wiki), or marked the claim UNVERIFIED. §8 lists them.
- **"Free levelling anywhere"** is Sherif's rule as relayed on Core's card. It is **not written verbatim** in the repo. The closest written rule is [`COMBAT_AND_PVP.md`](COMBAT_AND_PVP.md): Proficiency "is not XP, not a damage bonus, and not a Place gate. The world does not scale to it." This doc maps to that, and says so.
- Binding rules come from the addendum in the same #738 comment, already written in [`AUDIO_RADIO_QUEST_DESIGN.md`](AUDIO_RADIO_QUEST_DESIGN.md) §5.2: learned recipes account-bound; blueprint copies tradeable with limited uses; master blueprints tradeable until learned, then bind; radio unlock account-bound; spare parts tradeable. Nothing here changes them.

| Bounds item (verbatim theme) | Where in this doc |
|---|---|
| WoW TBC, LOTRO (named by Sherif) | §1.1, §1.2 |
| FFXIV, EVE, SWG, Albion, GW2, ARK | §1.3–§1.8 |
| Nine focus themes | §2 pattern table |
| Map to locked rules, adopt / adapt-Powrush-style / skip | §2, §3 |
| Real cited sources, unverified marked | §7, §8 |
| Implications for the Engineering path (proposal) | §4 |

Verdict key: **adopt** = take the pattern as is. **adapt-Powrush-style** = keep the idea, fit it to Powrush (care/abundance loop, no mall, no place gate, war + PvP + double agents intact). **skip** = do not take it.

## 1. Per-game notes

### 1.1 World of Warcraft: The Burning Crusade (named by Sherif)

- TBC is the first WoW expansion, released 16 January 2007 ([Wikipedia: The Burning Crusade](https://en.wikipedia.org/wiki/World_of_Warcraft:_The_Burning_Crusade)).
- Professions split into gathering, production and service; two primary professions per character, and any profession is open to any faction, race or class ([Warcraft Wiki: Profession](https://warcraft.wiki.gg/wiki/Profession)).
- Gatherers feed crafters. The wiki names "companion" pairings (e.g. Engineering ↔ Mining, Leatherworking ↔ Skinning) and frames the choice as gold saved by gathering yourself versus time saved by buying from other gatherers (same page). That trade-off is the interdependence engine.
- Skill rises by practice; recipe colour shows how likely a craft is to raise skill (same page).
- **TBC specialisation example.** In Shattrath City a tailor at 350 skill picks Mooncloth, Shadoweave or Spellfire. Mooncloth tailors make two Primal Mooncloth from one set of mats and get exclusive patterns whose results are Bind on Pickup, so "only Mooncloth Tailors could ever have the crafted items". Primal Mooncloth must be crafted at a Moonwell; the old 4-day cooldown was later removed; re-specialising became possible in patch 2.1.0 for a fee ([Warcraft Wiki: Mooncloth Tailoring](https://warcraft.wiki.gg/wiki/Mooncloth_Tailoring)).
- High-end recipes often come from faction reputation or instance drops ([Profession](https://warcraft.wiki.gg/wiki/Profession)).
- Later WoW (not TBC): Crafting Orders (Dragonflight, patch 10.0.2) let players post public, guild or personal orders with a commission; crafters fill them at a crafting station ([Warcraft Wiki: Crafting Orders](https://warcraft.wiki.gg/wiki/Crafting_order)). Crafting stations are fixed city anchors ([Profession](https://warcraft.wiki.gg/wiki/Profession)).
- TBC-era crafter name on items ("Made by"), and TBC Engineering Goblin/Gnomish detail beyond the one-line mention on the Profession page: **UNVERIFIED** this pass.

### 1.2 The Lord of the Rings Online (named by Sherif)

- Each character can master up to four professions, split into gathering and crafting ([Wikipedia: LOTRO](https://en.wikipedia.org/wiki/The_Lord_of_the_Rings_Online)).
- **Vocations**: a fixed bundle of three professions (a gatherer, a producer, a third). The design is described as meant "to encourage community inter-dependence by making some professions dependent on others for subcomponents" (lotro-wiki.com Vocations, https://lotro-wiki.com/index.php/vocations — **page refused our fetch (403); text seen only in a search-index extract, so UNVERIFIED by fetch**).
- **Crafting guilds** are NPC reputation factions joined at Expert tier, with improved recipes unlocked by rank and advanced by crafting reputation items; guild halls sit in fixed towns (lotro-wiki.com Crafting, https://lotro-wiki.com/index.php/crafting — **same 403; UNVERIFIED by fetch**).
- **Kinships** are LOTRO's player guilds, used for grouping, socialising and shared interests "such as crafting"; a kinship needs a charter and at least six characters ([LOTRO Life: Kinship Guide](https://www.lotrolife.com/kinship_guide/)).
- LOTRO crafter signatures, kinship crafting halls, and critical-success "masterwork" details: **UNVERIFIED**.

### 1.3 Final Fantasy XIV

- Eight crafting classes (Disciples of the Hand), each specialising in crafting, repairing, melding and desynthesising certain item types; HQ quality is a probability raised by crafting actions and HQ ingredients ([FFXIV Wiki: Crafting](https://ffxiv.consolegameswiki.com/wiki/Disciples_of_the_Hand)). The crafter "soul crystal" equivalent comes from a Specialist System (same page).
- Free Company houses can hold a **Company Workshop** to build items for the company such as airship parts and housing fixtures ([FFXIV Wiki: Company Workshop](https://ffxiv.consolegameswiki.com/wiki/Company_Workshop)).
- Crafted, materia-melded gear is described as the best gear for crafters and gatherers until the next crafted set, which drives steady materia demand ([FFXIV Wiki: Materia](https://ffxiv.consolegameswiki.com/wiki/Materia)).
- FFXIV crafter signatures on items and master-recipe books: **UNVERIFIED** this pass.

### 1.4 EVE Online

- Economy is open and largely player-driven; players gather raw materials to build almost all ships and modules; NPCs sell skill books and blueprints ([Wikipedia: Eve Online](https://en.wikipedia.org/wiki/Eve_Online)).
- **Blueprint originals vs copies.** Originals (BPO) sell on the market; copies (BPC) have a limited number of runs, cannot go on the market but can be sold by contract; copies are made from BPOs, bought from loyalty stores, or dropped by NPCs ([EVE University: Blueprints](https://wiki.eveuniversity.org/Blueprints)). BPOs can be researched for material and time efficiency (same page).
- **Invention** turns a Tech I copy into a Tech II copy using datacores, with success chance raised by skills; datacores are consumed whether or not the job succeeds ([EVE University: Invention](https://wiki.eveuniversity.org/Invention)).
- **Sinks**: every manufacturing job pays a cost made of a system cost index, a facility tax (owner-set in player structures, capped) and a surcharge ([EVE University: Manufacturing](https://wiki.eveuniversity.org/Manufacturing)).
- Ship destruction as the main item sink: commonly said, **UNVERIFIED** on fetched pages.

### 1.5 Star Wars Galaxies

- Launched with 34 professions; later reduced to nine with crafting as one XP group; Trader was the crafting profession after the change ([Wikipedia: Star Wars Galaxies](https://en.wikipedia.org/wiki/Star_Wars_Galaxies)).
- Design aimed at a dynamic economy and "a complicated division of labor"; players made blasters, starships, clothing, armour, food, housing, furniture and droids (same page).
- Restoration server wiki (community emulator, not the original live game): crafting depends on surveying for resources, resource quality capping the best possible result, and an **experimentation** step that spends points to push stats; a Merchant line builds vendors and advertising ([SWG Restoration wiki: Crafting](https://swgr.org/wiki/crafting/)). Treat details as emulator-era.
- SWG maker's-mark serials and resource spawns that rotate across planets: **UNVERIFIED** on fetched pages.

### 1.6 Albion Online

- "All equipment and items are made by other players"; red and black zones are full-loot on death ([Wikipedia: Albion Online](https://en.wikipedia.org/wiki/Albion_Online)).
- **Gathering → refining → crafting.** Refining sits between gathering and crafting; each tier needs the refined material of the tier below plus raw material; demand stays high because of the Black Market, item study and PvP where equipment may break; cities give local production bonuses by resource; stations in cities, hideouts and islands charge taxes ([Albion official: Guide: Refining](https://albiononline.com/news/guide-refining)).
- Focus points raise resource return when crafting/refining ([same guide](https://albiononline.com/news/guide-refining)). Daily focus amounts and Destiny Board specialisation numbers: on wiki.albiononline.com, which refused our fetch — **UNVERIFIED**.

### 1.7 Guild Wars 2

- Eight crafting disciplines, two active at a time with a fee to switch; players can experiment with ingredient combinations to **discover** new recipes ([Wikipedia: Guild Wars 2](https://en.wikipedia.org/wiki/Guild_Wars_2)).
- **Guild halls**: six buildings (tavern, mine, workshop, war room, market, arena); the workshop hosts a guild-only crafting profession, scribing ([GW2 official: Building Your Guild Hall](https://www.guildwars2.com/en/news/building-your-guild-hall/)).
- **Scribes craft for the guild, not themselves**: schematics go into a shared assembly queue; output lands in the guild panel; a shared "resonance" pool, filled by shards found while grouping with guildmates, can rush an item; scribes make guild consumables, WvW improvements and hall decorations ([GW2 official: Guild Halls: From Pen to Production to Placement](https://www.guildwars2.com/en/news/guild-halls-from-pen-to-production-to-placement/)).
- wiki.guildwars2.com showed a maintenance page on fetch; per-level costs and caps: **UNVERIFIED**.

### 1.8 ARK: Survival Evolved / Ascended

- Blueprints are found items, separate from learned engrams; usable unlimited times except Fishing Rod and Tek blueprints (3 uses); since patch 184.0 they do not stack; quality blueprints cost more materials and give better items ([ARK wiki: Blueprints](https://ark.wiki.gg/wiki/Blueprints)). Already used in `AUDIO_RADIO_QUEST_DESIGN.md` §5.1.
- Crafted quality items "able to be given to allies" is mentioned on the same page; general player-to-player trade of blueprints is **UNVERIFIED** (as already noted in §5.1 of the quest doc).

## 2. Pattern table (focus themes)

Locked rules column uses: **FL** = free levelling anywhere (closest written: `COMBAT_AND_PVP.md` "not a Place gate"); **DP** = dopamine/progress balance (`JOY_WITHOUT_MALL.md`, `PEAK_MEMORY_LAW.md`); **WP** = war + PvP + double agents intact (`COMBAT_AND_PVP.md`, `POWRUSH_LOOP_NOTES.md` §0, `SERVER_MORALITY.md` §2.2).

| Theme | Seen in | Verdict | Powrush shape | Locked-rule mapping |
|---|---|---|---|---|
| **Interdependent professions** | LOTRO vocations (unverified by fetch), WoW companions, SWG division of labour, Albion all-player-made | **adopt** | Each profession needs at least one input another profession makes (Engineering needs refined metal; harvest crews need repairs). | FL: anyone can take any profession anywhere, no place gate. DP: needing a neighbour is social joy, not a mall. WP: supply lines become real targets. |
| **Crafter reputation / signatures** | Claimed for WoW/FFXIV/SWG but **UNVERIFIED** on fetched pages | **adapt-Powrush-style** (held) | A maker's name on crafted goods and a visible "trusted maker" record. Proposal only. | DP: recognition, not stat power. WP: a double agent can forge trust and then betray, so signatures must show maker, not loyalty. |
| **Guild halls + crafting stations** | GW2 workshop/scribe, FFXIV Company Workshop, WoW crafting stations, LOTRO crafting guild halls (unverified by fetch), Albion stations | **adapt-Powrush-style** | Benches anywhere (FL); faction/guild workshops add shared queues and shared projects, never exclusive power. | FL: a basic bench in every settlement, so no place gate. DP: shared projects with visible progress. WP: workshops are raidable ground in PvP zones. |
| **Commissions / work orders** | WoW Crafting Orders (post-TBC), EVE contracts | **adapt-Powrush-style** | Player-to-player or faction-board orders with a fee; no global auction mall. | DP: `JOY_WITHOUT_MALL.md` refuses "AH", so orders stay local boards / direct. WP: an order can be a lure; that is allowed. |
| **Gathering → crafting supply chains** | Albion refining tiers, WoW companions, SWG surveying, EVE raw materials | **adopt** | Gather → refine → craft, with transport between. | FL: gather anywhere. WP: haulers are interceptable (as spare radio parts, quest doc §3). |
| **Specialisations** | WoW TBC tailoring specs, FFXIV Specialist, GW2 two active disciplines | **adapt-Powrush-style** | Late-ladder specialisation with re-spec allowed for a fee (as TBC 2.1.0), never locking a player out of an area or the radio. | FL: spec is not a place gate. DP: identity without permanent regret. |
| **Recipe discovery** | GW2 discovery, EVE invention (chance), SWG experimentation | **adapt-Powrush-style** | Higher level → higher discovery chance, with a pity floor (quest doc §4). | DP: floor stops slot-machine feel. No paid rerolls. |
| **Blueprint / item trade** | EVE BPO/BPC (copies limited runs, contract-only), ARK blueprints (limited-use Tek), WoW BoP specialist items | **adopt (already locked)** | Exactly the addendum: learned recipes account-bound; copies tradeable with limited uses; master tradeable until learned. | DP: no permanent farmable token. WP: copies can be sold to both sides by a double agent. |
| **Economy sinks** | EVE job cost + taxes, Albion station taxes + gear breaking in PvP, ARK limited uses, EVE datacores consumed on failure | **adapt-Powrush-style** | Copies wear out; materials consumed on each scribe; bench fees small. Avoid "lose all on failure" punishment for new players. | DP: sinks visible and fair. WP: PvP loss stays a real sink per existing zone rules. |
| Full-loot as main sink | Albion red/black zones | **skip** (beyond existing zone rules) | Powrush keeps its own zone/shield rules in `COMBAT_AND_PVP.md`; this card adds no new loot rule. | WP intact as written; nothing softened, nothing added. |
| One global player-run auction house | WoW AH, EVE market | **skip** | `JOY_WITHOUT_MALL.md` refuses AH. | DP. |
| Exclusive BoP specialist gear | WoW TBC Mooncloth | **skip** | Exclusive power for one spec conflicts with "neither path strictly better" (quest doc §5.3). | DP, FL. |

## 3. Summary of verdicts

- **adopt (3):** interdependent professions; gathering → crafting supply chains; blueprint/item trade (as already locked by the addendum).
- **adapt-Powrush-style (6):** crafter reputation/signatures (held, source claims unverified); guild halls + stations; commissions/work orders; specialisations; recipe discovery; economy sinks.
- **skip (3):** full-loot beyond existing rules; global auction house; exclusive spec-only BoP gear.

## 4. Implications for the Engineering path (PROPOSAL only)

Refines `AUDIO_RADIO_QUEST_DESIGN.md` §4–§5. Nothing here is locked.

1. **Inputs from others.** Engineering's radio parts need at least one refined input from a gathering/refining profession (WoW companion / Albion refining pattern). Solo players can still do it by taking a gathering profession too, so no gate.
2. **Benches everywhere, workshops for groups.** Any Engineering bench assembles the radio (already in quest doc §2). A faction or guild workshop adds a shared queue for group projects (GW2 scribe pattern), not a better radio.
3. **Field-repair work orders.** The quest doc's "field repairs" mission maps onto local work orders: a harvest crew posts, an engineer fills for a fee. Local boards, no AH.
4. **Copy scribing as the trade role.** Matches EVE BPC (limited runs) and ARK limited-use blueprints; keeps the addendum binding.
5. **Discovery with floor.** GW2 discovery + EVE invention chance, with the existing pity floor; failed attempts cost a little material, not everything.
6. **Specialisation late, re-spec allowed.** Optional Engineering sub-focus (held name, needs Sherif's OK) with a fee to switch; no exclusive power items.
7. **Maker mark (held).** Optional maker name on radio parts and copies, to grow crafter reputation. Held for Sherif's OK; sources for this pattern were not verified this pass.

## 5. Coherence with the locked rules (repo refs)

| Rule | Repo file | This benchmark |
|---|---|---|
| Free levelling anywhere / not a place gate | `docs/design/COMBAT_AND_PVP.md` (Proficiency row) — phrase "free levelling anywhere" not in repo verbatim | No profession, spec or workshop gates an area; basic benches everywhere. |
| No XP from kills | `docs/design/COMBAT_AND_PVP.md` | Profession XP from crafting/gathering only. |
| Dopamine / progress, no mall | `docs/JOY_WITHOUT_MALL.md`, `docs/PEAK_MEMORY_LAW.md` | No AH, discovery has a floor, no paid shortcut. |
| War + PvP + double agents | `docs/POWRUSH_LOOP_NOTES.md` §0, `docs/design/SERVER_MORALITY.md` §2.2 | Supply lines, workshops and orders are real PvP stakes; double agents may trade with both sides. |
| Binding addendum | `docs/design/AUDIO_RADIO_QUEST_DESIGN.md` §5.2 | Unchanged. |

## 6. Open questions

1. Does Engineering require a second profession input, or only make it cheaper? (§4.1)
2. Work orders: faction boards only, or also direct player-to-player? (§4.3; quest doc Q5)
3. Is a maker mark wanted at all, and is it account name or character name? (held)
4. Specialisation names and count (held for Sherif's OK).
5. Bench fee size as a sink (numbers open; no invented numbers here).

## 7. Sources (fetched 2026-10-26, loaded)

1. https://en.wikipedia.org/wiki/World_of_Warcraft:_The_Burning_Crusade
2. https://warcraft.wiki.gg/wiki/Profession
3. https://warcraft.wiki.gg/wiki/Mooncloth_Tailoring
4. https://warcraft.wiki.gg/wiki/Crafting_order
5. https://en.wikipedia.org/wiki/The_Lord_of_the_Rings_Online
6. https://www.lotrolife.com/kinship_guide/
7. https://ffxiv.consolegameswiki.com/wiki/Disciples_of_the_Hand
8. https://ffxiv.consolegameswiki.com/wiki/Company_Workshop
9. https://ffxiv.consolegameswiki.com/wiki/Materia
10. https://en.wikipedia.org/wiki/Eve_Online
11. https://wiki.eveuniversity.org/Blueprints
12. https://wiki.eveuniversity.org/Invention
13. https://wiki.eveuniversity.org/Manufacturing
14. https://en.wikipedia.org/wiki/Star_Wars_Galaxies
15. https://swgr.org/wiki/crafting/
16. https://en.wikipedia.org/wiki/Albion_Online
17. https://albiononline.com/news/guide-refining
18. https://en.wikipedia.org/wiki/Guild_Wars_2
19. https://www.guildwars2.com/en/news/building-your-guild-hall/
20. https://www.guildwars2.com/en/news/guild-halls-from-pen-to-production-to-placement/
21. https://ark.wiki.gg/wiki/Blueprints

Cited but **not loaded by our fetcher** (403): https://lotro-wiki.com/index.php/vocations, https://lotro-wiki.com/index.php/crafting.

## 8. Unverified list

- LOTRO vocation interdependence wording and crafting-guild details (lotro-wiki 403; search extract only).
- Crafter signatures / maker marks in WoW, LOTRO, FFXIV, SWG (no fetched page confirms).
- TBC-era Engineering Goblin/Gnomish specifics.
- FFXIV master-recipe books.
- EVE ship destruction as the main sink.
- SWG maker's-mark serials and rotating resource spawns; SWG Restoration details are emulator-era, not proof of the original live game.
- Albion focus daily amounts and Destiny Board numbers (wiki.albiononline.com 403).
- GW2 per-level crafting costs/caps (wiki.guildwars2.com maintenance page).
- ARK player-to-player blueprint trade (as in quest doc §5.1).

# In-game radio — the unlock quest: combat path, Engineering path, blueprints (design)

**Contact:** [info@Rathor.ai](mailto:info@Rathor.ai)

CARD AUDIO-RADIO-QUEST-DESIGN-1 — the second AUDIO card. Design only. Base: `main` at `64b9cc6a`.

Docs only: a proposal, not a work order. This file adds no client, shared, host or Cargo code, no audio assets and no economy code. It **states** guards; it implements none. Nothing here lands in the game until Core names a slice. Title Online stays grey.

What the radio *is* (stations, tune-in panel, credits screen, licence manifest) is set by AUDIO-RADIO-DESIGN-1 in [`AUDIO_RADIO_DESIGN.md`](AUDIO_RADIO_DESIGN.md). This file designs only how a player earns it. It does not change any station, track, licence or manifest rule there.

Labels used below:

- **FACT** — read from the repo at `64b9cc6a` (file cited) or from a public page read on 2026-10-26 (URL cited).
- **PROPOSAL** — a design choice, not a fact.
- **UNVERIFIED** — could not be confirmed on a source's own page or in the repo on 2026-10-26. Treat as unknown.
- **PROPOSED · HELD** — player-facing copy (mission, boss, profession-tier names, beat text). Held for Sherif's OK; nothing ships before it.
- **Qn** — an open question (§9).

No certification, legal or health claims are made here.

---

## 0. Source

Core's card on COUNCIL BOARD issue #738 (comment 6431481983) sets the bounds, and the addendum (comment 6431512884, Sherif "I Love it", starting rule; final copy still needs his OK) sets blueprint binding. Summary of what they require, with where this file covers it:

1. Two equal-reward paths to the same unlock, neither strictly better: a **Combat** path (parts / a partly-working radio from a high-difficulty boss job, real skill, fail-possible) and an **Engineering** profession path (levelling profession that unlocks recipes; higher level raises the chance of discovering more recipes; crafts the same parts). → §2, §3, §4.
2. Tradeable / shareable blueprints, ARK-style. → §5.
3. Coherence with the locked progression rules (free levelling anywhere, dopamine/progress balance, war + PvP + double agents intact); blueprint trading anti-exploit and economy-fair (no infinite dupes, no pay-to-skip of both paths, no RMT vector), stated not implemented. → §6, §7.
4. Mission-chain beats, mission names and profession-tier names ranked with the creative-stuck approach, held for Sherif's OK. → §3, §8.
- Addendum binding rules → §5.2.

Context relayed from Sherif (2026-10-26), used only to keep this quest consistent, not designed here: Transport-Tycoon-style transport/harvesting and Factorio-style production chains run by guilds, gangs and corporations; every advanced feature is unlocked through a rational, in-world prerequisite; war and PvP are real; double agents can be good or evil.

---

## 1. Inputs

| Input | Where | Status |
|---|---|---|
| What the radio is | `docs/design/AUDIO_RADIO_DESIGN.md` — §2.3 says the panel is "reachable from the pause menu and from the radio device once unlocked (unlock is AUDIO-RADIO-QUEST-DESIGN-1)"; the header says the unlock is "a quest with combat or Engineering-profession paths, account-bound" | FACT |
| Lead Mate's licence research | Not in the repo. `AUDIO_RADIO_DESIGN.md` §1.1 names it as a box file, `/workspace/lead-mate/audio-rnd/royalty-free-sources-2026-10-26.md`. This card adds nothing from it: the unlock quest touches no track or licence. Every licence rule stays where AUDIO-RADIO-DESIGN-1 put it. | FACT (location); content not restated |
| Locked progression rules | See §6 table — the files that actually exist | FACT |
| ARK blueprint pattern | [ARK Official Community Wiki — Blueprints](https://ark.wiki.gg/wiki/Blueprints), read 2026-10-26 | FACT, quoted in §5.1 |
| PROFESSIONS-BENCH-1 | Sibling card (same addendum comment) that benchmarks professions across MMOs. Not yet written. The Engineering path here is a placeholder shape it may refine. | FACT (card exists on #738); no content yet |

---

## 2. The unlock in one picture (PROPOSAL)

The radio is built from **three radio parts** plus one assembly step. Both paths end at the same assembly step and the same unlock.

| Part (working names, PROPOSED · HELD) | What it is in-world |
|---|---|
| Tuner coil | Picks the frequency. |
| Signal board | Turns signal into sound. |
| Power cell housing | Keeps it running off-grid. |

**Assembly** at any Engineering bench (or a faction workshop that has one) turns the three parts into a **Field Radio**. Assembling it grants the account-bound **radio unlock** (§5.2) and the radio device that opens the panel from AUDIO-RADIO-DESIGN-1 §2.3.

Why parts and not one drop: the rule "every advanced feature unlocked through a rational in-world prerequisite" reads naturally — a radio is made of components; you either take them from someone who had a working one, or you learn to make them. Parts also give the two paths a shared currency so neither one is a dead end: a player can mix (two parts from the boss, one crafted).

---

## 3. Combat path (PROPOSAL)

**Shape.** A high-difficulty job against a powerful boss that carries a partly-working radio. Real skill and practice, fail-possible, retryable. Typical combat-game challenge: learnable telegraphs, phases, a wipe if the group ignores mechanics.

**Beats** (PROPOSED · HELD; names in §8):

1. **Hook — the broken broadcast.** A garbled signal is heard near a faction outpost. A technician NPC explains it is coming from a machine carried by a raiding boss.
2. **Trace.** Track the signal with a handheld detector to the boss's ground. Teaches the area and the boss's tells; no kill required.
3. **The fight.** The boss jams the area (screen static on its telegraphs as the readable cue — a visual, not an audio-asset ask). Fail-possible. On a kill: a **partly-working radio** that salvages into **two of the three parts**, plus a chance at a **master blueprint** for one part (§5).
4. **Missing piece.** The third part is not on the boss. The player either crafts it (Engineering, any player), buys or is gifted one (parts are tradeable, §5.2), or repeats the fight for the missing salvage on a later run.
5. **Assembly.** At an Engineering bench → Field Radio → unlock.

**Why fail-possible is still fair.** A failed attempt costs time and consumables, never the parts already earned, never XP or proficiency (consistent with `COMBAT_AND_PVP.md`, where kills never pay XP and death never transfers XP). Retries are allowed.

**PvP and war.** The boss ground sits in contested space where the zone rules from `COMBAT_AND_PVP.md` already apply; this card adds no new PvP rule. Equipped gear never drops; *carried* salvaged parts follow the existing carried-goods rules for that zone, so a rival gang or a double agent can intercept a courier carrying spare parts. The radio unlock itself cannot be stolen (§5.2).

---

## 4. Engineering profession path (PROPOSAL)

**Shape.** Engineering is a levelling profession. Each level unlocks recipes; higher level also raises the chance of **discovering** extra recipes while crafting. The three radio parts sit mid-ladder.

**Beats** (PROPOSED · HELD; names in §8):

1. **Apprentice job.** A workshop NPC hands out scrap to sort and simple fixes (lamps, pumps). Teaches the bench.
2. **Field repairs.** Repair broken equipment for harvest and transport crews. Ties the profession to the transport / harvesting and production-chain pillars without designing them: the crew needs working machines, the engineer supplies them.
3. **First discovery.** At the tier that unlocks circuits, crafting has a chance to reveal a new recipe. The tuner coil is guaranteed by the tier; the signal board and power cell housing can be learned by tier **or** by discovery **or** from a blueprint (§5).
4. **Parts run.** Gather or buy the materials (supply chains, guild/corp stock), craft the three parts.
5. **Assembly.** Same bench step → Field Radio → unlock.

**Discovery odds.** The rule is "higher level → higher chance". Exact numbers are open (Q2). A floor keeps it from becoming a slot machine: after a set number of qualifying crafts without a discovery, the next one is guaranteed (PROPOSAL; dopamine balance, §6).

---

## 5. Blueprints (PROPOSAL, addendum rules binding)

### 5.1 The real ARK pattern (FACT, read 2026-10-26)

From the [ARK Official Community Wiki, Blueprints page](https://ark.wiki.gg/wiki/Blueprints):

- Blueprints "enable the crafting of specific items, but are not learned through spending Engram Points".
- They "can be found in Supply Crates, Loot Crates, Deep Sea Loot Crates, by killing Mini-bosses and Orbital Supply Drops".
- They "can be used an unlimited number of times at any level, except for the Fishing Rod and all Tek blueprints, which are only usable 3 times."
- Changelog patch 184.0: "Blueprints can no longer stack." Patch 245.95: "Fishing Rod blueprint can only be used 3 times."
- "You can stock blueprints on a Bookshelf."

That ARK blueprints are tradeable between players as inventory items is the common understanding but is **not stated** on that page — **UNVERIFIED** by this card. What Powrush takes from ARK: blueprints are physical inventory items, found from bosses and loot, separate from learned knowledge, and some carry limited uses. What Powrush changes: limited uses are the default for copies, not the exception (the economy sink the addendum asks for).

### 5.2 Binding rules (addendum — starting rule; final copy needs Sherif's OK)

| Thing | Rule |
|---|---|
| **Learned recipe** (profession knowledge) | Account-bound. Not sellable, not tradeable. |
| **Blueprint item — copy** | Physical. Tradeable, giftable, sellable. Carries **limited uses / durability**; each craft spends a use, and the copy is consumed at zero. Economy sink: the market cannot flood. |
| **Master blueprint** (rare; e.g. the boss drop in §3) | Tradeable **until used to learn**. Using it teaches the recipe and the item binds to the learner (it cannot then be resold). |
| **Radio unlock** | Account-bound via either path. Never tradeable, giftable or sellable. |
| **Spare radio parts** | Tradeable (and therefore interceptable as carried goods in contested zones, §3). |

How a copy is made (PROPOSAL): a player who has **learned** a recipe can scribe a limited-use copy at a bench, paying materials each time. That makes Engineering players the source of copies, gives the profession a trade role, and keeps every copy tied to real materials.

### 5.3 Anti-exploit and economy-fair guards (stated, not implemented)

| Risk | Guard (stated) |
|---|---|
| **Infinite dupes** | Copies have finite uses and are consumed. Scribing a copy costs materials every time. Master blueprints are unique drops, server-authoritative, each with an item id logged on drop, trade and use (the event log already referenced in `COMBAT_AND_PVP.md` lets a review roll an exploit back). Copies do not stack (as ARK patch 184.0). |
| **Pay-to-skip both paths** | No real-money purchase of the unlock, of parts, of blueprints, or of profession levels. A bought (in-game currency) blueprint still needs the materials and a bench, so trade shortens the path, it never removes it. The unlock is account-bound and can only be granted by the assembly step. |
| **RMT vector** | Real-money trade is already a reportable conduct item (`COMBAT_AND_PVP.md`, Reports). Master blueprints bind on learn, the unlock is never tradeable, and copies wear out, so there is no permanent high-value token to farm and sell. Item-id logs (above) make a farmed chain visible to review. No auction-house mall: `JOY_WITHOUT_MALL.md` refuses "AH" and "RMT bounty". Trade stays player-to-player / faction stores (Q5). |
| **One path strictly better** | Both paths cost comparable time for a typical player (target in Q1), give the same parts and the same unlock. Combat gives a chance at a master blueprint; Engineering gives a permanent learned recipe and the ability to scribe copies. Different flavours, equal reward. |
| **Alt-account funnelling** | Master blueprints bind on learn, unlocks are per account; a group can help one player, but cannot mint unlocks for alts without each alt doing the assembly with real parts. |

---

## 6. Coherence with the locked progression rules

The phrase "free levelling anywhere" was **not found verbatim** in the repo at `64b9cc6a` (grep for "free level", "level anywhere", "levelling anywhere"). It is cited here as Sherif's rule relayed on Core's card (#738, comment 6431481983). The closest written rule is in `COMBAT_AND_PVP.md`. Files that do exist:

| Rule | Real repo file | How this quest keeps it |
|---|---|---|
| Free levelling anywhere / no place gate | `docs/design/COMBAT_AND_PVP.md` — Proficiency "is not XP, not a damage bonus, and not a Place gate. The world does not scale to it." | The radio is an optional comfort feature, never a gate on levelling or on any area. Engineering XP is earned at any bench, anywhere; the combat path is one zone among many and never required. |
| No XP from kills | `docs/design/COMBAT_AND_PVP.md` — "Kills never pay proficiency and never pay XP"; "There is no XP transfer on a kill." | The boss pays parts and a blueprint chance, not XP. Player kills over a courier pay nothing new. |
| Dopamine / progress balance, no mall | `docs/JOY_WITHOUT_MALL.md` (refuses AH, RMT bounty, perfect-or-trash trading, target-farm tables); `docs/PEAK_MEMORY_LAW.md` ("`JOY_WITHOUT_MALL` still forbids mall dopamine") | Discovery has a pity floor (§4) so it is not a slot machine. No loot box, no paid shortcut. Progress is visible: each part is a step you can see in the satchel. |
| War + PvP intact | `docs/design/COMBAT_AND_PVP.md`; `docs/POWRUSH_LOOP_NOTES.md` §0 (Sherif verbatim: "war and PvP are real parts of the game. Double agents can be evil or good.") | Boss ground and parts couriers sit inside existing PvP zone rules; spare parts are real loot worth guarding. Nothing here removes or softens PvP. |
| Double agents | `docs/design/SERVER_MORALITY.md` §2.2 (double agents, proposed, not built) | A double agent can sell parts or blueprint copies to both sides, or tip off a courier route. The unlock itself cannot be stolen, so betrayal costs goods, never a player's earned feature. |

---

## 7. Fit with the wider pillars (context only, PROPOSAL)

- **Rational prerequisite**: a radio is assembled from parts that either came from a working set or were engineered. Nothing is granted by a menu.
- **Production chains / guilds, gangs, corporations**: parts and copies are ordinary goods; a corp can run a parts line, a gang can raid one. This card does not design those systems.
- **Transport**: spare parts are carryable goods; moving them is a job.

---

## 8. Names — creative-stuck ranking (PROPOSED · HELD for Sherif's OK)

Method: the `creative-stuck-idea-recycling` skill. Choice: player-facing names for the quest chain, the two path missions, the boss, and Engineering tiers. Constraints: no clash with existing Powrush canon, no major-IP clash, short, radio/engineering pun welcome (matching the station-name tone in `AUDIO_RADIO_DESIGN.md` §8), no health claims.

**Bases (real sources):** station-name style and puns in `AUDIO_RADIO_DESIGN.md` §8 (e.g. "Static Cling", "Cool Fusion … a joke for the Engineering crowd"); "MendSpool" blueprint copy in `docs/HOUR_THREE.md`; "Embassy blueprint page chance (no mall)" in `docs/MERCY_TEMPER_PROGRESSION.md`; ARK's quality ladder names on the wiki page (Primitive → Ascendant — pattern only, not reused); Core's card wording ("partly-working radio", "powerful boss").

**Repo clash search** (`rg -i -F` across the repo at `64b9cc6a`): 0 hits for every candidate below except as noted. "Fabricator" (52 hits, e.g. `docs/HUMAN_PLAYABILITY_v23.2_FABRICATOR.md`) and "Patcher" (6 hits) were dropped for that reason.

**Screen limits:** not trademark clearance. Only English was checked; other languages need a native-speaker check. Web/IP clash checks were by knowledge only, not a search — **UNVERIFIED**.

| Slot | Rank | Name (PROPOSED · HELD) | Pitch / bases | Tier | Open flags | Next test that could kill it |
|---|---|---|---|---|---|---|
| Whole chain | 1 | **Tuning In** | Plain, inviting; the player literally tunes in. From the station tone. | idea | Common phrase, not distinctive | Council read-aloud |
| | 2 | **Back on the Air** | Restoring a dead radio. | idea | Common phrase | Same |
| | 3 | **Signal & Noise** | Skill vs chaos. | idea | Close to a well-known book title (UNVERIFIED) | IP check |
| Combat mission | 1 | **Dead Air Raid** | "Dead air" (radio silence) + "air raid"; the boss jams the airwaves. | idea | "Dead Air" is used as a film/book title (UNVERIFIED) | IP check |
| | 2 | **Jam Session** | Jammer boss + music pun. | idea | Very common phrase | Read-aloud |
| | 3 | **Static Shock** | Static on telegraphs. | idea | Same name as a DC Comics character/series — likely fail on clash | IP check (expected fail) |
| Engineering mission | 1 | **Solder On** | "Soldier on" with solder; perseverance in a world at war. | idea | Pun may read as typo | Read-aloud |
| | 2 | **Circuit Training** | Training + circuits. | idea | Fitness-term echo; keep copy free of health claims | Copy review |
| | 3 | **Ohm Sweet Ohm** | Home-made radio. | idea | Widely used pun (shops, shirts) | Distinctiveness check |
| Boss | 1 | **The Jammer** | Says what it does. | idea | Generic | Lore fit (Draek or faction-neutral?) Q6 |
| Engineering tiers (5) | 1 | **Tinker → Fixer → Wright → Engineer → Master Engineer** | Craft words rising in scope; reads in-world. | idea | "Tinker" is a hero/class name in other games (UNVERIFIED which); "Wright" substring hits "copyright" only | IP check |
| | 2 | **Scrapper → Wirer → Builder → Designer → Visionary** | Scrap-to-vision arc. | idea | "Scrapper" also means fighter | Read-aloud |
| | 3 | **Spark → Coil → Circuit → Grid → Lattice** | Electrical scale; "Lattice" echoes repo tone. | idea | Bare nouns, weak as rank names | Council vote |

Default is HOLD. A seed becomes copy only after a council vote and Sherif's OK.

---

## 9. Open questions

- **Q1** Target time for each path for a typical player (equal-reward check).
- **Q2** Discovery odds curve and pity floor value.
- **Q3** Uses per blueprint copy and material cost to scribe one.
- **Q4** Is the boss group content, solo-scalable, or both?
- **Q5** Where trades happen (player-to-player, faction stores) given the no-AH rule.
- **Q6** Boss lore: Draek, raider, or faction-neutral.
- **Q7** Does a second radio (spare device) need a new assembly, or does the account unlock spawn the device anywhere?

## 10. Next

Core may vote this as a docs card. Any slice (quest data, profession ladder, blueprint items, logs) needs its own card. PROFESSIONS-BENCH-1 may refine §4. All names stay held for Sherif's OK. Online grey.

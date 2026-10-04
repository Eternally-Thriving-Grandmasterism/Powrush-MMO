# LAYERED GAMES AND SERVER — layer stack, economy spectrum, offline NPCs, authoritative server plan

**CARD DESIGN-LAYERS-1.** Base `86fb4786` (`86fb4786b01ff9fb9ac1018b7f8a6f8a2b44f90b`) after #638.
**CARD DESIGN-LAYERS-2.** Base `fdd2928a` (`fdd2928afe689eec6db0a3ca0a61bb4f0c5ece81`) after #639: RUSH Easter egg (§7.8) and epiphanies through play (§11).

**Contact:** [info@Rathor.ai](mailto:info@Rathor.ai). Design tick, not a Cargo bump. Docs only.

Online grey. This file is a **plan**. It opens no socket, adds no listen, unparks no crate, edits no `client/**`, `shared/**`, `server/**`, `.github/**` or Cargo file, and lights nothing on the Title. `NetMode::default()` stays `Offline`. `title_online_enabled` stays false. Bevy stays pinned at `0.14`.

**Every number in this file is a target, not a measurement.** Nothing here has been built, profiled or load-tested. No performance, scale or certification claim is made.

Peak memory still holds: walked · tended · week was the bill · yard remembered.

---

## 0. Sherif's asks (verbatim)

> "All right, but let's do better, and remember eventually Powrush-MMO will sort of have games layered on games, like the guilds formed by players will have to harvest with AI/AGI robots and drones like a modern factorio mixed with Transport Tycoon Deluxe. Also, players can manually harvest and do things in a simple and somewhat solo way like in ARK:Survival games and of course it needs to all be cohesive like the weekly guild wars of Conquer Online, and the faction, reputation and racial systems of World of Warcraft packaged altogether to offer the paths to failure and success for experimenting towards a truly player structured and managed economy potentially leading to an RBE or other variants like an advanced AGI-RBE or more primitive economic systems which can play out both naturally to some extent in the offline experience with the in-game NPCs which are like super advanced oblivion NPCs from ES:4 and in the online experience of course the real players make it truly happen altogether with an authoritative server which we should architect and design for maximal optimization from top and moste critical priorities all the way down appropriate and respectively for online gameplay of Powrush-MMO to thrive, and perform even better than this modernized remix of the genius behind Chris Sawyer's TTDX to inspire our work even more, Mates"
>
> — Sherif, 2026-10-04 2:43 PM ET

> "The oblivion style NPCs may exist online also I suppose, but accordingly and respectively to avoid replacing humans, and instead be more like only for those choosing to engage them or need them for quests where players lack and a group quest can help or simply those who can't or won't communicate yet want the group experience online during parts and perhaps with real players or a mixture of both like when stuck at a boss for example, but maybe not for end game like raid bosses?"
>
> — Sherif, 2026-10-04 2:46 PM ET

> "Perhaps the online version has to be hired and paid for by 1 or more pf the players like if they own their own droid, or the guild offers the privilege of summoning a droid for example then they can do it online, but offline its easier to access in a rational and logical way, as well as simply fill in gaps where real players pr important NPCs may, should, could or perhaps occasionally or even rarely be available for Easter eggs and seasonal things, my Dear Brilliant Legendary Mates!"
>
> — Sherif, 2026-10-04 2:49 PM ET

> "Let's have a holiday schedule also with tastefulness and make it based on the in game universe, and have more in game holidays exist based on each online server community's decisions like a global vote by the guild leaders and/or their Councils to give players a sense of true ownership of their decisions, diplomacy amongst other real players, and beyond by expanding and extrapolating from all this properly and effectively, Mates!"
>
> — Sherif, 2026-10-04 2:52 PM ET

> "Like the AGi play on words, Powrush has also been a Proof of Work Rush, and maybe Rush is its own secret acronym Easter egg, but hilariously, that was a happy accident, or subconsciously intentionally, maybe straight from channeling TOLC, eh, Mate?"
>
> — Sherif, 2026-10-04 3:29 PM ET

> "Yeah, Mate!"
>
> — Sherif, 2026-10-04 3:30 PM ET

> "Agreed, and 2nd place goes to Real Unity Shapes History, both Councils choice and 2nd place are fitting, but theirs is truly #1 and we can include more philosophical ideas to offer humans true epiphanies from playing POWRUSH, my Dear Brilliant Legendary Mates!"
>
> — Sherif, 2026-10-04 3:32 PM ET

> "Perhaps more rewards are in order, wisely and thoughtfully altogether respectively and appropriately, Mates!"
>
> — Sherif, 2026-10-04 4:20 PM ET

> "The rarer and more difficult a challenge, the higher the tier of rewards, with some realism simular to real life fluctuations with some ordered abd structured reliability when continuously maintaining the infrastructure and requirements like in Chris Sawyer games and similar such as ARK:SE games and others such as Medieval Dynasty with economics, resource harvesting, auromated NPCs like humans and even animals, and so on for Powrush-MMO which includes more too like space aliens, advanced tech, and droids to name examples which all may be bolstered and supported or even maliciously or naively destroyed by other players when online, but never offline unless enabled as a server setting mode, Mate!"
>
> — Sherif, 2026-10-04 4:24 PM ET

A quote is a direction, not a lifted HOLD. Where this plan meets an existing refuse line (§9), the line stays until Sherif names the lift on its own ticket.

In this doc, 'the Steward' means Sherif's own word as relayed. Only Sherif can lift a refuse line, open Online or name PATHS (§6.7).

---

## 1. Borrow law

We borrow **technique only** from seven references: ARK (solo gather, build, upkeep, spoilage, taming), Factorio (production chains, throughput simulation), Chris Sawyer's Transport Tycoon Deluxe and OpenTTD (routes, depots, cargo flow, upkeep, vehicle and station reliability and servicing), Medieval Dynasty (villagers with needs and jobs, husbandry, seasons), Conquer Online (weekly guild war cadence), World of Warcraft (factions, reputation, races) and Oblivion (scheduled NPCs with needs and goals). We never borrow their characters, names, places, art, text or UI. Player-facing words stay Powrush words (`docs/PRE_RELEASE_LAW.md` refuses franchise strings). The keep / transform / refuse tables in `docs/GDD_IMMERSION_REVISION.md` §2 still apply; this file extends them to Online and does not restate them.

---

## 2. What already exists (ground truth at base)

The plan grows from what is on disk, not from a blank page.

| Area | On disk today | Status |
|---|---|---|
| Deterministic sim law | `docs/SIM_AND_HAND_CANON.md` §4: \(S_{n+1} = F(S_n, C_n)\); no wall clock, no unseeded RNG, no `HashMap` order as gameplay | Law |
| Layer cake | `docs/STUDIO_ARCHITECTURE_ORDER.md`: F / Hand / Presentation / Sky; rungs A–F then Sky | Law |
| Verbs | Sacred five: Tend / Take / Flow / Reserve / Mend; wire `Op` in `shared/hex_protocol.rs` (also Lane, Bind, DeclareLethal, NameHouse, RequestSeat …) | Shipped offline |
| Node life | `Idle / Glowing / Tended / Resting / Stressed` (`docs/RBE_FIRST_HOUR.md`, `shared/climate_node.rs`) | Shipped offline |
| Chain | `shared/vertical_factory.rs` extractor → depot → hauler → two stops → arrival; `shared/fabricator.rs` MendSpool + LaneCrate → Proof Pack; `shared/embassy.rs` blueprints | Shipped offline |
| Credit | Flow = field restore; Reserve = repair-rights (`docs/CREDIT_RESERVE_LOGISTICS.md`) | Shipped offline |
| Score | Week bill = tons \(T_w\) + restored \(U_w\) (`shared/week_audit.rs`); `shared/war_week.rs` score = tons + restored | Shipped offline |
| Standing | `shared/shard_standing.rs` peace · harmony · consumption · steward per hex | Shipped offline |
| Contribution class | `shared/nevc_adapter.rs` Contributor / Zombie; `shared/contribution_ledger.rs` | Shipped (teaching signal, not wages) |
| Offline time-lapse | `shared/shard_sim.rs` ledgers tick without a second human | Harness only |
| Divergent slots | `shared/shard_slots.rs` thrive vs poor hex files | Shipped offline |
| Day | `client/src/living_day.rs` 240 s day, `DayPeriod` | Shipped offline |
| NPC law | `docs/NPC_SCHEDULE_SPEC.md` rung E: schedules on the existing day, same verbs, hour finishes with zero persons, no theft / fence / person combat | Law, unbuilt |
| Peoples | `docs/PLAYABLE_RACES.md` Human · Cydruid · Quellorian · Draek · Ambrosian, each with a landing Place; `docs/FACTIONS_OVERVIEW.md` Luminari Alliance vs Draek Dominion | Lore |
| Councils | `docs/COUNCIL_SYSTEM.md` Local → Regional → Global tiers, proposals, sessions, decisions | Design |
| Join law | `docs/SHARD_JOIN.md`: offline authority until honest `hello_ok`; join is copy-with-consent; drop resumes last certified snapshot; presence = real houses or silence; merge veto | Law |
| Shard lab | `powrush-shard/` loopback-only WS, JSON envelopes, `ledger_snapshot.json`, `--dry-apply events.jsonl`, soft cap 32 Houses per hex | Parked, loopback only |
| Server experiments | `server/` (Bevy 0.14 `default-features = false`, tokio): AOI, hierarchical grid, interest management, validator, war system, trade, persistence, telemetry | Parked; depends on `simulation/`, which does not resolve on crates.io, so it does not build today |
| Observability configs | `observability/` Prometheus, OTel collector, Grafana JSON | On disk, unused |
| Online ladder | `docs/ONLINE_LADDER.md` R0–R7 | Law |

**Decision:** `shared` stays engine-free (today it depends only on serde). Game truth is pure functions in `shared`; the client and any future server are shells that call them. Parked `server/` is a mine to copy ideas from, not a crate to revive wholesale.

---

## 3. The layer stack

Four layers, one economy. Each layer is a game you can play on its own for an evening, and each layer's output is another layer's input. Nothing is a mini-game bolted on: if a layer stopped producing, the others would feel it within a week.

```
          (d) PEOPLES · FACTIONS · REPUTATION
           who you are, who trusts you, which routes and blueprints open
                 ▲ standing                       │ access, sides
                 │                                ▼
 (a) SOLO HANDS ──raw stock, rare care──▶ (b) GUILD AUTOMATION ──throughput──▶ economy
     ARK-like gather, build, mend          drones, robots, chains,            (§4 spectrum)
     ▲                                     routes, depots, upkeep                  │
     │ tired fields, needs                       ▲ territory, node rights          │
     │                                           │                                  │
     └──────────── (c) WEEKLY GUILD WAR ◀────────┘◀─── what is worth contesting ────┘
                   contest stewardship of frontier nodes and corridors
```

### 3a. Solo hands (ARK-like, manual)

**Technique borrowed:** gather by hand, carry what you can, build a small shelter, keep it up or it decays.

- One human, one satchel, the sacred five. Take at a Glowing node, Mend and Tend tired ones, Flow or Reserve the take. This is the Offline 1.0 hour, unchanged.
- Online adds a **homestead**: a small claim the solo player keeps (a hearth, a bench, a store). It has **upkeep** in Reserve and decays to Resting, never deleted, if left (`docs/GODSPEED_PREP.md`: failure teaches, no wipe).
- **Why automation needs it:** some nodes are **care-only** — a Stressed or fragile node can only be brought back by a hand Mend, never by a drone. Rare inputs (seeds, cuttings, first samples of a new biome) come only from hands. Guild chains therefore buy, barter or ask for solo work. A solo player is a supplier, a scout and a repair crew, not a lesser tier.
- **Why it needs the others:** solo players need blueprints (Embassy, faction-gated), safe corridors (held by guilds or ceasefires), and a place to sell, barter or Flow their surplus.

### 3b. Guild automation (Factorio chains, TTD logistics)

**Technique borrowed:** production chains with ratios and bottlenecks (Factorio); routes, depots, vehicle capacity, station rating, running cost (TTD / OpenTTD).

- A guild (a House of Houses) charters **extractors** on nodes it stewards, places **depots**, and runs **routes** with **haulers**: drones, crawlers, barges. AI/AGI robots are guild workers that execute standing orders the guild writes: "keep depot B above 40 tons of LaneCrate", "Mend any node under harmony 0.3".
- Chains extend the one already on main: extractor → depot → hauler → stops → arrival (`shared/vertical_factory.rs`), and MendSpool + LaneCrate → Proof Pack (`shared/fabricator.rs`). New tiers add refine and assemble steps. Every tier is a ratio problem, so a bottleneck is visible and fixable.
- **Upkeep is the teacher.** Each extractor, depot and route has a running cost in Reserve and a wear rate. An over-extracting chain makes nodes go Stressed, throughput falls, upkeep keeps charging, and the guild learns the RBE lesson at scale: "if you only take, the glow fades."
- **Throughput is the score that matters:** tons moved per week plus nodes restored, the same \(T_w\) + \(U_w\) family as the House week.
- **Why it needs the others:** raw care-only inputs from 3a; node rights and corridors from 3c; route passage and blueprints from 3d.

### 3c. Weekly guild war (Conquer Online cadence)

**Technique borrowed:** a fixed weekly window when guilds that show up contest a held objective, and the result shapes the next week.

- What is contested is **stewardship**: the right to charter extractors on a frontier node cluster or to run a corridor for the next week. Not loot, not kill counts.
- Score carries the law already on main (`shared/war_week.rs`): **tons delivered + nodes restored**, plus **hold time** on contested posts. A guild that wins by stripping the frontier inherits a Stressed frontier.
- Lethal is **opt-in per declared war instance only**, never the default and never on Sanctuary (DeclaredLethal path, `docs/GDD_IMMERSION_REVISION.md` §2.1). A war can be fought entirely on logistics and restore score.
- Losing does not delete. Losers keep homesteads, Reserve and standing; they lose next week's rights to the contested cluster.
- **Why it needs the others:** it only matters because 3b makes nodes valuable; factions (3d) give it sides and treaties; solo players (3a) can join as escorts, scouts and menders.

### 3d. Peoples, factions and reputation (WoW technique)

**Technique borrowed:** faction sides that shape access and conflict; reputation that opens doors; races with distinct homes and leanings.

- **Peoples** are the five on disk, each with a landing Place: Human (Sanctuary), Cydruid (Heartwood), Quellorian (Threshold), Draek (Depths), Ambrosian (Sanctuary from above). A people gives **affinities**, not a power scalar: a Cydruid tends Heartwood growth with less wear; a Draek reads Depths routes; a Quellorian runs Threshold pipe throughput cleaner. Differences create comparative advantage, and comparative advantage creates trade. Peoples are chosen **after House**, never at a Title race lobby.
- **Factions**: the Luminari Alliance, the Draek Dominion, and unaligned Houses. Faction is a guild-level allegiance that can change, with a cost.
- **Reputation** is **standing**, extended per faction from `shared/shard_standing.rs` (peace · harmony · consumption · steward). It moves only by verbs: restoring a faction's nodes, honouring treaties, hauling on its corridors. It opens faction depots, blueprints at the Embassy, safe passage on faction routes, and war alliances. It is a moral read with consequences, not a grind bar with a vendor at the end.
- **Redemption** stays open: Draek and others have redemption paths (`docs/REDEMPTION_MECHANICS_PER_SPECIES.md`); standing can recover.
- **Why it needs the others:** reputation has nothing to open without 3b's blueprints and routes and 3c's alliances; it is earned in 3a and 3b verbs.

### 3e. Cohesion rule

A layer may not have its own currency, its own score or its own world. One ledger, one week bill family, one map of Places and hexes. A design that needs a separate token for a layer is a bolt-on and is refused.

### 3f. The wider cast

The world is more than Houses and wells. Every entity below is a **world entity** under the same rules: it has an owner or steward, upkeep, reliability (§4.5) and standing effects, and it can be supported, bolstered or harmed under §3g.

| Kind | Examples from canon | Role in the layers |
|---|---|---|
| **Automated NPC humans** | Villagers, haulers, menders, guards (§5) | Work posts and chains; have needs (food, rest, shelter, belonging) and morale that shift output, the Medieval Dynasty way |
| **Animals** | Herd animals, pack animals, wildlife (`client/src/living_ecology.rs` deer and ecology props are the seed) | **Husbandry:** feed, shelter and breed; output (wool, milk, pull) follows care. **Herding:** animals move on routes like haulers, at a slower rate. **Wildlife ecology:** a population per hex with regrowth and predator/prey balance; overhunting collapses it, rest restores it |
| **Space aliens** | The playable alien peoples (Quellorian, Draek, Ambrosian, `docs/PLAYABLE_RACES.md`) and non-playable alien fauna and fleets (`docs/FLEET_CLASSES.md`, `docs/DRAEK_FLEET_AI_SYSTEMS.md`) | Peoples per §3d; fauna as wildlife; fleets as later Sky content, after Online |
| **Advanced tech** | Resonance tech, crystalline and organic ship designs, Cydruid machine parts (`docs/AMBROSIAN_CRYSTALLINE_SHIP_DESIGNS.md`, `docs/CYDRUID_ORGANIC_SHIP_DESIGNS.md`) | High-tier chain outputs and infrastructure with high upkeep and high reliability when maintained |
| **Droids** | Companions (§6), guild automation droids (§3b) | Workers and companions with upkeep; offline droids follow §6.7 |

Supporting or bolstering an entity means repairing it, paying its upkeep, guarding it, or reinforcing it (raising its reliability ceiling or its defence for a week). Support from another House earns standing with the owner and the owner's faction.

### 3g. Support, bolster and destruction

Online, other players can **support** or **bolster** what someone else built, or **destroy** it, maliciously or by accident. Destruction is real enough to matter, bounded enough that griefing doesn't ruin play.

**Server setting: `world_harm_mode`.** One named setting per server, and a matching local setting offline. **Default `off` everywhere.**

| Mode | Who can harm others' infrastructure, NPCs, animals and droids | Where |
|---|---|---|
| **`off`** (default) | Nobody. Accidents bounce off ("this belongs to House X"). | Everywhere |
| **`declared`** | Players inside a weekly war instance (§3c) or on a hex with DeclaredLethal set (the existing opt-in after the book) | War instances and declared hexes only |
| **`frontier`** | As `declared`, plus frontier hexes outside homestead cores | Frontier; never homestead cores |
| **`open`** | Anyone, under the consequences below | Everywhere except protected areas |

Always protected, in every mode: **Sanctuary Prime**, a homestead's core (hearth, bench, store), new players for their first 2 real weeks (target), and every Place during a holiday ceasefire (§7.4).

**Offline:** nothing is ever destroyed by anyone else. The local `world_harm_mode` is `off` by default. A player may turn it on to let NPC factions raid their frontier, as a harder offline mode; Sanctuary stays protected, and turning it back off stops raids at the next sim day.

**Consequences (online):**

| Act | Inside war rules (`declared` instance or hex) | Outside war rules |
|---|---|---|
| **Malicious destruction** | Allowed and expected; capped by war rules (siege damage drops a target to ruin, never deletes it) | Large standing loss with the victim's House and faction; own faction censure; the victim's allies may declare against the attacker's House; event log entry for review |
| **Naive destruction** (bad drone order, route through a field, stray fire) | Normal war cost | Small standing loss; automatic repair claim against the actor; a confirm prompt before any act that would harm another's property |
| **Support / bolster** | Allies may repair and guard mid-war | Standing gain with the owner and their faction |

**Mitigation, so griefing doesn't ruin play:**

- **Ruins, not deletion.** A destroyed thing becomes a ruin that keeps salvage (target 50% of its build cost) and can be rebuilt with Mend. Nothing is wiped (`docs/GODSPEED_PREP.md`).
- **Insurance.** Houses and guilds may pay a premium in Reserve into an insurance pool, in the double-entry ledger (§8 P3). A claim pays repair materials less a deductible. Malicious damage outside war rules is charged back to the attacker's House first.
- **Repair.** Mend restores; allies can help; repair speed rises with reliability record (§4.5).
- **Guards.** Hired or owned guards (NPC humans or droids) defend depots and routes; they count against the zone budget (§6.3).
- **Caps.** Damage per target per day is capped (target: at most one tier of damage per day outside war); repeated attacks on the same victim escalate consequences.
- **Review.** Every harm act is in the event log (§8 P2), so exploits can be reviewed and rolled back.

---

## 4. Economy spectrum

The server **does not pick the economy.** It enforces physics (nodes tire, upkeep charges, haulers have capacity), keeps an honest ledger, and **measures**. The regime is whatever the players' choices add up to, read from indicators.

### 4.1 Regimes

| Regime | What players do | What it looks like in the ledger | Existing tie |
|---|---|---|---|
| **R0 Subsistence** | Each takes and keeps for self | Almost no transfers; high Take, low Flow | Offline hour before allocate |
| **R1 Barter** | Swap goods directly | Paired item-for-item transfers | Satchel I; Lane crates |
| **R2 Credit** | Promise repair-rights; carry debts | Reserve notes moving between Houses | `CREDIT_RESERVE_LOGISTICS` Reserve |
| **R3 Exchange** | Players quote ratios and set terms in a player-run hall | Many transfers at varying ratios in Reserve units | Market **HOLD** (§9); no server-minted gold |
| **R4 Guild commons** | Guild pools stock and plans allocation | Transfers into guild stores, allocation by guild order | House week across hexes |
| **R5 RBE** | Needs met from commons; contribution earns standing, not stock | High Flow share; needs-met ratio high; holdings even | Flow = field restore; Contributor class |
| **R6 AGI-RBE** | Guild AGI managers allocate by player-set policy, players audit and vote | Allocation orders from managers; policy votes logged | Council tiers; §7 votes |

Regimes can sit side by side on one server (a barter frontier, a commons guild, an exchange hall) and can regress.

### 4.2 Paths to failure (all reachable, none forced)

- **Extraction collapse:** chains over-take, nodes go Stressed, throughput falls, upkeep keeps charging. Indicator: restore ratio under 0.5 for two weeks.
- **Hoard and cartel:** one alliance holds the corridors and starves the rest. Indicator: holdings Gini and corridor share.
- **Credit default:** Reserve promises outrun real repair capacity. Indicator: outstanding Reserve vs capacity.
- **Tragedy of the commons:** a commons with no rules is stripped. Indicator: commons stock trend vs draw.
- **War attrition:** guilds spend more on war than the cluster yields.
- **AGI misallocation:** a manager follows a bad policy faithfully. Indicator: idle capacity and unmet needs while stock sits in depots.

Failure is recoverable. Stressed nodes recover with Mend; regimes can change by vote; nothing is wiped.

### 4.3 Paths to success

- Restore keeps pace with take; frontier stays green.
- Needs are met for most players from commons or fair exchange.
- Holdings stay broad; new players can start.
- Automation frees hands for care, scouting, diplomacy and building.

### 4.4 Indicators (computed by the server, shown on a server ledger face)

| Indicator | Formula sketch | Source |
|---|---|---|
| Climate health | mean harmony − mean stress across hexes | `ShardClimate`, `ShardStanding` |
| Restore ratio | \(U_w / \max(T_w, 1)\) | `WeekAudit` |
| Needs-met ratio | players whose weekly need basket was met ÷ active players | new need basket (§5) |
| Flow share | Flow tons ÷ all allocated tons | allocate log |
| Transfer mix | share of transfers that are barter / credit / exchange / commons / managed | ledger tx kinds |
| Holdings Gini | Gini over House holdings in Reserve-equivalent | ledger |
| Corridor concentration | top-3 guild share of route throughput | logistics graph |
| Automation share | tons moved by haulers ÷ all tons | logistics graph |
| Idle capacity | depot stock unused while needs unmet | logistics + needs |
| Contributor share | Contributor ÷ (Contributor + Zombie) | `nevc_adapter` |

**Regime label** is derived, never set: e.g. transfer mix dominated by commons and managed allocation, needs-met ≥ 0.8, restore ratio ≥ 1.0 and Gini ≤ 0.35 reads as R5. Thresholds are design targets to tune, not facts.

The NEVC Contributor / Zombie class stays a teaching signal, not wages, not an abundance score (README).

### 4.5 Realistic fluctuation with structured reliability

The world varies like real life, and care makes it dependable.

**Fluctuation (bounded, seeded, deterministic):**

- Yield = base × season × weather × event × (1 + ε). Seasons come from the calendar cycle (§7.2), weather from the existing flow-weather moods, events from the server calendar.
- ε is bounded (target ±15%) and seeded by (world seed, hex, sim day), so a replay gives the same harvest (§8 P1).
- **Spoilage** (ARK technique): perishables lose value in store each sim day unless kept in a cold store or processed. Spoilage is a sink, and it rewards logistics that move goods on time.
- **Ratios and prices:** NPC barter ratios drift within bounds with supply and season. Player prices, if exchange (R3) is ever lifted, are set by players, never by the server.

**Structured reliability (Chris Sawyer technique):**

- Every vehicle, drone, depot, extractor and station has **reliability** from 0 to 1. It decays with use and time.
- **Breakdowns** are a seeded roll against reliability, so a neglected hauler stalls on its route and its flow drops to zero until it is repaired.
- **Servicing** at a depot restores reliability. A route can set a service interval (TTD-style), at an upkeep cost.
- **Maintenance record:** infrastructure that is serviced on time week after week raises its **reliability ceiling** step by step (target +1 point per maintained week, up to 0.99), and its output variance narrows. Neglect drops the ceiling back down. This is the "ordered and structured reliability" the steer asks for: steady care earns a steadily more dependable chain.
- **Station rating** (TTD technique): a node or depot that is served often gives a larger share of its output to that route; one served rarely gives less and lets some output go to waste.
- **Villager needs and work** (Medieval Dynasty technique): NPC workers with met needs work steadily; unmet needs lower morale and output, and in time they leave for a better-run House.
- **Animals:** husbandry output follows feed, shelter and care; neglected herds thin out; wildlife regrows when left to rest (§3f).

All of it runs on the logistics and ledger ticks (§8 P7, P3), as rates, not per-item entities.

### 4.6 Challenge reward tiers

The rarer and harder a challenge, the higher its reward tier.

**Tier score** = rarity + difficulty + cooperation, each scored 0 to 3:

| Axis | 0 | 1 | 2 | 3 |
|---|---|---|---|---|
| **Rarity** | Always available | Weekly | Seasonal (once per cycle season) | Once per cycle, or a rare spawn |
| **Difficulty** (target success rate for a prepared group) | > 90% | 60–90% | 30–60% | < 30% |
| **Cooperation** | Solo | Party (2–5 players) | Guild (6+ players) | Cross-guild and cross-faction |

| Tier | Score | Example |
|---|---|---|
| **T1 Common** | 0–2 | Mend a Stressed node cluster alone |
| **T2 Uncommon** | 3–4 | Weekly party escort through a frontier corridor |
| **T3 Rare** | 5–6 | Seasonal guild restore of a collapsed wildlife hex |
| **T4 Epic** | 7–8 | Seasonal cross-guild rebuild of a ruined depot network |
| **T5 Legendary** | 9 | Once-per-cycle, cross-faction server project (§7.4) |

**Two kinds of reward, kept apart:**

- **Discovery and epiphany rewards** keep the §11.3 law: cosmetic only, never the ledger. A higher tier only makes the cosmetic rarer (a finer thread, a rarer title).
- **Challenge rewards in the economy layers** may give **real output** (stock, blueprints, Reserve), scaled by tier and posted through the double-entry ledger from a named source, `challenge_reward` (§8 P3).

**Anti-farm limits for challenge output:**

- Once per character per challenge per cycle; repeats give story and cosmetics only.
- A per-cycle **emission cap** per server for `challenge_reward`, sized against the §4.4 indicators, so tiers can't flood the economy.
- Shares split by server-measured contribution, never by who was in the party list.
- Companion-assisted runs follow §6.4: one-time completion only.
- No reward for destroying your own or an alt's property, and no reward for harm acts outside war rules (§3g).

---

## 5. Offline mode: scheduled NPCs that play the economy

**Technique borrowed (Oblivion):** people with schedules, needs and goals who do the job of the place, remember what you did, and react to your standing.

Offline, the other Houses, guilds and factions are NPCs. They run the same verbs on the same tick, so the economy spectrum plays out without a second human. `docs/NPC_SCHEDULE_SPEC.md` stays law: **the hour finishes with zero persons**, persons never write the house book, never gate a Place, never steal, fence or fight.

### 5.1 Agent model

| Part | Shape |
|---|---|
| **Schedule** | Blocks on `DayPeriod` from the existing 240 s day: rest, work post, meal, gather, home. No second clock. |
| **Needs** | Rest, supply, belonging, purpose; each 0..1, decays per sim hour, refilled by verbs (Tend fills purpose, a shared meal fills belonging). |
| **Goals** | Utility pick from small packages: tend my post, haul to depot, mend a tired node, visit the Embassy, join the war-week score. Highest score wins; ties broken by entity id, never by hash order. |
| **Faction** | One allegiance plus standing toward the player's House (same fields as `ShardStanding`). |
| **Memory** | Bounded ring of facts, 16 slots per agent (target): "House X restored my well", "House X took from a Stressed node". Facts decay over in-game weeks. |
| **Reactions** | Standing and memory pick greeting lines (one line, H hides), barter willingness, ratio offers, and whether an NPC House joins a commons. A cooler greeting and a tired well, never a crime meter. |
| **Economy role** | NPC Houses run small chains through the same `vertical_factory` and `fabricator` sims, choose Flow or Reserve by a temperament, and form commons or hoard. That is how R0–R5 appear offline. |

NPC guilds also play an **offline war week**: a peaceful contest scored on tons + restored, resolved by the same `war_week` math.

### 5.2 Determinism

NPC decisions are part of \(F\): seeded RNG keyed by (save seed, sim tick, agent id); ordered iteration; no wall clock. Same save + same player commands → same NPC world. This is also the test oracle for the server later.

### 5.3 Level of detail

| Tier | Who | Update rate (target) | What runs |
|---|---|---|---|
| **L0 Near** | Within the player's Place and view | every sim tick that the agent is due, at most 10 Hz | Full schedule, goal pick, movement, greeting |
| **L1 Place** | Same Place, out of view | 1 Hz | Schedule + goal pick; position snaps along a path |
| **L2 Hex** | Other written hexes | once per in-game hour (10 s real at a 240 s day) | Schedule block outcome only, verbs as counts |
| **L3 Aggregate** | Whole NPC Houses far away | once per in-game day | Statistical flows into the hex ledgers, as `shard_sim` does now |

Promotion and demotion are deterministic: an agent promoted from L2 to L1 is placed where its schedule says it should be, so the player never sees a jump.

### 5.4 CPU budget (targets)

Frame budget assumes the 60 Hz client and the "readable hour on a small machine" law.

| Item | Medium target | Low target |
|---|---|---|
| All NPC sim per frame | ≤ 1.5 ms | ≤ 0.75 ms |
| L0 agents | ≤ 32 at ≤ 20 µs each | ≤ 16 |
| L1 agents | ≤ 256 at ≤ 2 µs amortized | ≤ 128 |
| L2 agents | ≤ 2,000, batched once per in-game hour, spread over frames | ≤ 1,000 |
| L3 Houses | ≤ 64, once per in-game day | ≤ 32 |
| Memory per agent | ≤ 1 KiB | same |

If the budget is exceeded, demote the furthest agents a tier; never drop a frame for NPCs, and never let NPC work touch the hand (`SIM_AND_HAND_CANON` §3).

---

## 6. NPC companions online

Sherif's 2:46 PM and 2:49 PM steers. Online, scheduled NPCs may also be **companions**, but they **never replace humans** and are **opt-in only**. Online they are **hired, owned or summoned and paid for in in-game resources**; offline they are easier to reach.

### 6.1 When companions are allowed

| Allowed | Not allowed |
|---|---|
| Filling group-quest slots when there are not enough humans | End-game raid bosses |
| A group experience for solo players, or players who can't or won't talk, in chosen parts | Weekly guild wars (any role: fighter, hauler, holder, scout) |
| Mixed parties with real players, e.g. when a group is stuck at a boss | Holding or capturing territory, ranked or scored contests |
| Story moments where a canon NPC walks with you | Council votes, holiday votes, any governance (§7) |

### 6.2 Rules

1. **Opt-in, default off.** A player turns companions on per party and pays for them (§6.5–§6.6). Nobody is placed beside a companion without choosing it.
2. **Humans first.** Group finder offers human players first. A companion slot **yields** to a human who joins; the companion leaves at the next safe moment.
3. **At least one human leads.** Companion parties need a human in range. If the leader is idle past a timeout (target 3 min), companions stand down.
4. **Labelled honestly.** Companions show as companions. They never count in presence (`SHARD_JOIN` F4: presence is real houses or silence).
5. **Server brains only.** Companions run on the server under the same authority. Players give stances (follow, hold, mend first), never scripts.
6. **Cap per party.** Party size − 1 at most; target max 3 companions per party.

### 6.3 Server cost

Companions are server entities in the zone's tick and count against its budget.

| Item | Target |
|---|---|
| Companion brain rate | 5 Hz (movement interpolated by the client) |
| Brain cost | ≤ 50 µs per companion per brain tick |
| Zone share | companions ≤ 15% of a zone's entity budget; past that, new companion requests queue |
| Bandwidth | same snapshot path as players; no extra channel |

### 6.4 Abuse and limits

| Abuse | Limit |
|---|---|
| Companion farming of stock or credit | Companions cannot Take, carry, hold a satchel, trade or allocate. Kills and drops in companion-assisted content give **no** ledger output beyond the quest's own one-time completion reward. |
| Repeat-farming a quest with companions | Quest completion reward is once per character; repeats give story only. |
| Companions as harvest bots | They cannot touch nodes or chains; guild automation is the only automation and it pays upkeep. |
| Companions as war or territory proxies | Excluded from war instances and from holding posts. |
| Reputation farming | Companion-assisted completions grant standing at a reduced rate (target 50%) and only once per quest. |
| AFK parties | Leader idle timeout; companions stand down. |
| Displacing humans | Human-first group finder; slot yield on join. |
| Load abuse (spawning many) | Per-party cap and per-zone share; queue past the share. |

### 6.5 Hiring, owning and summoning (online)

Online, a companion is never free and never a menu default. There are three ways to get one, all paid in **in-game resources**:

| Way | Who pays | What it is |
|---|---|---|
| **Hired** | One player, or several players splitting the fee | A companion for one quest or one session, paid in Reserve or goods at a hall or Embassy post |
| **Owned droid** | One player | A personal droid built or bought in game, kept at the player's homestead |
| **Guild summon** | The guild, as a privilege its leaders grant | A droid the guild keeps, summoned by members the guild allows, charged to the guild store |

### 6.6 Costs, upkeep and the logistics tie

- **In-game resources only, never real money.** No cash purchase, no paid tier, no premium summon. This holds for every way above.
- Companions are **economy sinks**. Hiring burns a fee. Owned and guild droids carry **upkeep**: power cells and MendSpool for repair, charged on the ledger tick like any other machine (§8 P3).
- Droid parts and power cells come from the **logistics layer** (§3b). A guild that runs a droid bay needs the chain that feeds it, so companions draw on the economy and produce nothing back into it (§6.4).
- An unpaid droid goes Resting, the same as a tired node or an unkept homestead. It is never deleted.
- Fee and upkeep sizes are tuning targets, set so a droid costs about as much as the help it gives in a group quest, and recorded as `companion_fee` and `companion_upkeep` transaction kinds so the §4 indicators can see the sink.

### 6.7 Offline access

Offline, companions are **easier to reach**, and how you get one still makes sense in the world. A neighbour from a nearby NPC House walks with you because your standing with that House is good, or your House keeps a droid at its bench after the fabricator makes one. Nothing is paid in real money, and the hour still finishes with zero companions (`NPC_SCHEDULE_SPEC`).

**Reconciling with the offline drone refuse.** `docs/OFFLINE_ECONOMY_COURT.md` refuses "drones / robots as product" in Offline 1.0, and §9 keeps that line. The plan handles it this way:

- Offline companions that are **people** (NPC House neighbours, §5) fit inside today's law, because they are scheduled persons who use the same verbs and never harvest for you.
- Offline **droids**, whether as companions or as guild automation, stay refused offline until **Sherif lifts that line** on a named ticket. Lifting it is Sherif's call alone. This doc records the proposal and how to lift it, and does not lift the line itself.
- Even after the lift, an offline droid companion follows §6.4: it does not Take, carry or trade, so it can't become a harvest bot.

### 6.8 Gap-fillers, rare visitors and seasonal NPCs

- **Filling gaps.** When there aren't enough real players for a group, or a key NPC (a guide, an Embassy steward, a post keeper) isn't around, a companion or stand-in can fill the part. A stand-in yields as soon as the real player or key NPC returns.
- **Rare and seasonal.** Some notable NPCs show up only rarely, as **Easter eggs** (an unannounced visit at one Place, at a low chance per in-game week, target ≤ 5%) or as **seasonal visitors** tied to the holiday calendar (§7.2), such as a Luminari figure on Auroral Night or a Draek guide on Depths Return.
- Rare and seasonal NPCs carry story, a greeting and maybe a cosmetic keepsake. They carry **no** power items and **no** ledger output, and you can't hire them.
- Offline, they appear on the House-week calendar; online, on the server calendar. Both are deterministic from the seed and calendar, so they can be tested.

### 6.9 Card

See §10, CARD COMPANION-RULES-1 (shared rules and tests, no server) and the HOLD server card that follows.

---

## 7. Holidays and community governance

Sherif's 2:52 PM steer. A tasteful canon calendar from Powrush's own world, plus holidays each server community adds by vote, with the same vote engine reused for other server decisions.

### 7.1 Taste law

- Holidays come from the **in-game universe**: harvest, sky, Places, peoples, faction history. Never real-world religious holidays, never real-world national days, never a sale.
- A holiday changes **mood, gathering and story**, not power. No holiday power buffs that decide wars, no paid items, no login streaks, no loot boxes (`docs/GODSPEED_PREP.md` refuses daily login, loot box and season pass).
- Tone follows `docs/LORE_BIBLE.md`: grounded, warm, quiet. Remembrance days are dignified, not festive.

### 7.2 Canon calendar (proposal)

Online, one in-game year is a **cycle** of eight real weeks (target), four seasons of two weeks each, so the weekly war cadence lines up with seasons. Offline, the calendar keys to the House week count, so a solo player meets each holiday in order at their own pace.

| Season | Holiday | Lore root | What happens |
|---|---|---|---|
| Thaw | **First Tend** | The first harvest a House ever makes (the epiphany beat) | Wells glow longer; new players are paired with a mentor House if both opt in |
| Thaw | **Settling Day** | Houses founded and books written (Settled + book) | House seals shown at Heartwood; the server ledger face shows the year's founded Houses |
| Bloom | **Lamp Night** | Heartwood lamp and Wards | Night walk between Wards; shared Mend goals on Heartwood nodes |
| Bloom | **Auroral Night** | The aurora of the Luminari mothership | Sky event; rare sighting of a canon Luminari figure at Threshold |
| Harvest | **Week of the Bill** | The House week: tons + restored | Server-wide restore goal; if met, the next season starts with greener frontier |
| Harvest | **Depths Return** | The one way down, one way home | Depths landing lit; Draek and Human guides together |
| Dark | **Day of the Broken Light** | The Great Betrayal, remembered | Remembrance, not celebration: no war that day, Draek redemption quests open, quiet beds |
| Dark | **Crownstone Witness** | The Crownstone is seen | Witness gathering; council year-review read aloud from the server's history |

Rare canon NPC appearances happen only on their holiday, at one Place, for a short window (target 20 min real, twice per holiday), and never carry power items.

### 7.3 Community holidays and the vote engine

Each server can add its own holidays. Eligible voters are **guild leaders and/or their councils** (the council tiers in `docs/COUNCIL_SYSTEM.md`), so the voice is the organised community, not alt accounts.

**Steps**

1. **Proposal.** A guild leader or council files: name, date in the cycle, lore root, what happens, which Place. A proposal needs **co-sponsors** from at least 3 guilds of at least 2 factions (target).
2. **Taste review.** Automatic checks (name filter, no real-world religious or national day, no power effect, no paid item) and then a council review window. Failing proposals return with reasons.
3. **Debate.** 7 real days (target). Threaded comments on the proposal, one post per guild per day, amendments by the sponsor only.
4. **Vote.** 3 real days. Ballot per guild weighted by **active members**, with diminishing weight (square root of active members, target) so one giant guild cannot decide alone. Councils that vote as a body cast their member guilds' ballots only if those guilds delegate.
5. **Quorum and threshold.** Quorum: guilds holding at least 40% of active players voted (target). Pass: at least 60% yes by weight **and** yes from guilds in at least 2 factions.
6. **Enactment.** A passed holiday enters the server calendar at the next season boundary, never mid-season. It stays one cycle on probation; if repeal is not filed, it becomes permanent.

**Protections**

| Risk | Protection |
|---|---|
| Spam proposals | One active proposal per guild; 2 proposals per season per council; failed proposals cooled for one cycle |
| Alt or shell guilds | Voting guilds need minimum age (target 4 weeks) and minimum active members (target 5 humans) |
| Vote buying | Ballots are secret until close; ledger transfers to voting guilds during a vote window are flagged for review |
| Brigading by one faction | Two-faction yes rule; square-root weighting |
| Calendar crowding | Cap of 1 community holiday per season, 4 per cycle; canon holidays cannot be removed by vote |
| Offensive content | Taste review, Steward/admin veto with a public reason in the server history |
| Companions or NPCs voting | Never; ballots are human guild leaders and councils only |

### 7.4 Holidays as diplomacy

- **Holiday truces:** an enacted holiday may carry a **ceasefire**: no war instance opens and declared-lethal hexes stand down for its window.
- **Treaties on the day:** alliances and treaties signed during a holiday are recorded with the holiday in the server history.
- **Shared projects:** a holiday can name a server-wide project (restore the frontier river, rebuild a bridge) with a shared progress bar fed by tons + restored from every guild that joins.
- **Co-sponsorship** across factions is itself a diplomacy act and shows on each guild's record.

### 7.5 The same engine for other server decisions

The vote engine is generic. Other ballot types, each with its own quorum and cooldown:

| Ballot type | Example | Bound |
|---|---|---|
| Event theme | Next season's sky event | From a canon list plus approved proposals |
| Economy policy experiment | Raise route upkeep 10% for one season; open a commons granary | Only parameters on an allowed list, with min/max, one season, auto-revert unless renewed |
| War calendar | Shift the war window by two hours for a region | Within allowed windows |
| Holiday repeal | Remove a community holiday | Same quorum as enactment |

Economy experiments are where the spectrum (§4) is tested on purpose: the vote, the parameter change and the indicator before and after are all recorded, so a server can see what its choice did.

### 7.6 Server record

Each server keeps its own **history**: calendar, every proposal, debate summary, vote tally, enactments, repeals, treaties, ceasefires and project outcomes. It is readable in game at the Embassy and exportable as JSON. Two servers' histories never merge (`SHARD_JOIN` merge veto, carried to servers).

### 7.7 Data and server needs

| Need | Shape |
|---|---|
| **Vote ledger** | Append-only events: `proposal_filed`, `cosponsored`, `review_passed/failed`, `comment`, `ballot_cast` (sealed), `vote_closed`, `enacted`, `repealed`. Same event log as §8 P2. |
| **Ballot secrecy** | Ballots stored sealed until close, then tallied and published as totals per guild. |
| **Per-server calendar** | `calendar.json` derived from the canon list + enacted events; versioned with the snapshot. |
| **Eligibility** | Guild age, active member count, faction, council delegation; computed at proposal time and frozen for the vote. |
| **Scheduler** | Holiday windows, ceasefire flags and rare NPC windows as timed events on the world tick (sim time, not wall clock inside \(F\); the real-time schedule maps to sim ticks at the edge). |
| **Cost** | Votes and calendar are low rate (minutes, not ticks); they run on the ledger tick, not the zone tick. |

### 7.8 RUSH Easter egg

Powrush also reads as **Proof of Work Rush**: the work of tending and restoring is the proof a House leaves behind. It's wordplay only. Nothing here mines, mints or touches a chain (README refuses NFT and chain mint).

**RUSH is its own secret backronym**, with two canon readings:

| Rank | Reading | Where and when | Why it fits |
|---|---|---|---|
| **#1 (Councils' choice)** | **RUSH = Restore, Uplift, Share, Harvest** | Faintly carved in gold on the old stone of the first Sanctuary well, visible **only during Week of the Bill** (§7.2), online and offline | Week of the Bill is the holiday about the House week, tons + restored, and the server-wide restore goal. The four words are that bill read aloud, on the first well a player ever tends. |
| **#2** | **RUSH = Real Unity Shapes History** | Carved on the base of the Embassy lamp at Heartwood, where the server history is read (§7.6), visible **only during a community-voted holiday** (§7.3), so online only | A community holiday exists only because players' unity, through proposal, debate and vote, shaped that server's history. |

Rules:

- **A quiet wink, a quiet reward.** No announcement, no toast, no pop-up, no quest marker, no patch note pointing at it. A player who reads a carving on its holiday earns the quiet rewards below, and nothing louder.
- **What finding a carving earns** (see §11.3 for the shared reward law):
  - **Codex entry.** A short journal or codex page appears the next time the player opens the book, not at the moment of finding: "Restore, Uplift, Share, Harvest" for #1, "Real Unity Shapes History" for #2, each with one line of the well's or the lamp's lore.
  - **Crest flourish or tabard thread.** A thin gold thread the player may stitch into their own House tabard or crest, on that one item, following `docs/FACTION_HERALDRY.md` (physical, not projected; no transmog, no wardrobe). #1 gives a well-stone gold thread; #2 gives a lamp-light thread.
  - **Rare title.** "Reader of the Stone" (#1) or "Witness of the Lamp" (#2), shown only if the player chooses to wear it.
  - **Small sound reveal.** The well's or the lamp's bed carries a faint extra phrase for that player on later visits during the same holiday.
  - **First finder (online, opt-in).** The first player on each server to read each carving may choose to have one line added to that server's history (§7.6), e.g. "Week of the Bill, cycle 3: the stone was first read by House Vysholt". Off by default; declining costs nothing.
- **Rare.** Each carving shows only on its holiday window, and the carving is faint enough that only a player standing close at the stone, in daylight, reads it.
- **Purely cosmetic.** No ledger output, no standing, no Reserve, no stat, no power. The thread and the title are cosmetic only. It doesn't change the well's life or the lamp.
- **Not a verb.** E on the stone tends the well as always; the carving has no Use of its own (`docs/PRE_RELEASE_LAW.md`: a verb that does nothing is a lie, so the carving is dress, not a verb).
- **Deterministic.** Shown from the calendar and the sim day, never from wall clock inside \(F\), so it can be tested.
- **Never a sale or a hint system.** Not on the store page, not in a loading tip, not in a holiday banner.

---
## 8. Online authoritative server (ordered by priority)

Ordered from **most critical** down: the earlier items, if wrong, cannot be fixed later without a rewrite. All of it stays HOLD until the Steward writes `online yes` and names the PATHS (`docs/ONLINE_LADDER.md` R7).

### 8.0 Shape

Start as **one process with module boundaries**, split later only when a measurement says so.

```
 clients ──▶ gateway module ──▶ zone sim (per hex cluster) ──▶ event log + snapshots
                                   │   ▲
                                   ▼   │ flows, node rights
                              logistics sim (per region)
                                   │
                                   ▼
                              ledger (single writer per account shard)
                              votes · calendar · war scheduler
```

Each module talks through typed messages, so moving one to its own process later changes transport, not logic.

### P0. Server authority and anti-cheat

- Clients send **intents** (the `Op` set in `shared/hex_protocol.rs` plus movement input), never results. The server applies \(F\) and replies `apply` or `reject` with the existing codes (`NO_TAKE`, `STALE_SEQ`, `PROTO`, …).
- Every intent is validated against server state: range to node, node life, satchel space, rate, sequence number.
- Movement: the client predicts its own body (local body law); the server checks a speed and collision envelope and corrects only outside it.
- Every ledger change is server-side. The client never writes stock, Reserve, standing or score.
- Rate limits per op per player (target: Use ≤ 10/s, chat ≤ 2/s). Behaviour flags feed review, never auto-bans.
- Harm intents (§3g) are checked against `world_harm_mode`, protected areas, ceasefires and damage caps before apply; the server, not the client, decides whether a hit lands.
- Sanctioned automation (guild drones) removes most of the reason for external bots, and nodes tiring (Resting / Stressed) caps what any macro can take.
- Presence is server-authored only (`SHARD_JOIN` F4).

### P1. Deterministic fixed-tick simulation

- \(S_{n+1} = F(S_n, C_n)\) on a fixed tick, the same law as `SIM_AND_HAND_CANON` §4.
- **Ledger quantities are integers** (milli-units), never floats. Positions may stay `f32` because the server is the single authority, but anything that is replayed for audit is integer.
- Seeded RNG per (world seed, tick, entity id); `BTreeMap` or sorted ids for any iteration that affects state.
- Commands are ordered by (tick, sequence, player id) before apply.
- Payoff: replay from the event log reproduces state, which gives audit, crash recovery, bug repro and load-test oracles.

### P2. Persistence and event log

- Every applied command and every system event (war result, vote close, upkeep charge) is appended to an **event log** with tick and sequence. Batched flush (target every 100 ms).
- **Snapshots** per zone at a fixed cadence (target 5 min) and at shutdown. Recovery = last snapshot + replay of the log tail (target ≤ 30 s).
- `powrush-shard` already has the seed of this: `ledger_snapshot.json` plus `--dry-apply events.jsonl`.
- Player Offline saves stay theirs. Join is copy-with-consent; drop resumes last certified snapshot (`SHARD_JOIN`).

### P3. Economy ledger and transactions

- **Double entry:** every transfer debits one account and credits another; sum over all accounts is invariant, except named sources (Take from a node) and sinks (upkeep, wear).
- **Idempotent** transactions with client-chosen ids, so retries never double-pay.
- Cross-zone or cross-guild transfers go through the ledger module as one atomic step; zones hold no balances of their own.
- Transaction kinds are tagged (barter, credit, exchange, commons, managed, upkeep, war) so §4 indicators fall out of the ledger.
- An invariant checker runs every ledger tick and halts that shard's writes on a mismatch.
- Named sources and sinks include `challenge_reward` (capped per cycle, §4.6), `spoilage` and `wear` (§4.5), and `insurance_premium` / `insurance_claim` through a pool account (§3g).

### P4. ECS on the server

- `bevy_ecs` headless at **0.14** (`MinimalPlugins` style, fixed schedule, no renderer) or a plain Rust loop. Either way the systems call the same pure `shared` functions the client calls.
- Recommendation: plain Rust first for the ledger, logistics and votes (low entity counts, heavy logic); `bevy_ecs` for zone sims (many entities, queries). Both share `shared`.
- No Bevy climb from this plan (`docs/BEVY_CLIMB_PLAN.md`).

### P5. Spatial partitioning and interest management

- Coarse: hex cells (the game is already hex-addressed) for ownership, persistence and handoff.
- Fine: a uniform spatial hash inside each zone for range queries (ideas on disk in `server/src/spatial/`).
- Interest per client: radius by entity class (players and companions near, haulers mid, depots and nodes far as low-rate summaries), plus a **priority accumulator** so the most important updates win when the bandwidth cap is hit.
- Distant crowds collapse to counts per cell.

### P6. Snapshots, prediction and reconciliation

- Per-client **delta snapshots** against the last acked baseline; keyframe on join and on baseline loss.
- Quantize: position to 1 cm relative to the cell, yaw to 8 bits, stock counts as varints; changed-field bitmasks.
- Client: own body predicted locally; verbs show an immediate local ack ("finger ack this tick"), then server `apply` or `reject` settles the universe. Other entities interpolated about two snapshots behind.
- Corrections blend, never snap, unless far out of envelope.
- Transport for the lab stays JSON over loopback WS (`powrush-shard`). A binary envelope (postcard, already named in `NET_OFFLINE_CONTRACT` §7) is a later card.

### P7. Logistics and factory sim at its own tick

The TTD / Factorio lesson: **simulate flows, not items.**

- Routes, depots and chains are a **graph**: nodes (extractors, depots, assemblers) with rates and buffers; edges (routes) with capacity, length and upkeep.
- The graph solves on a **logistics tick** (target 1 Hz), separate from the zone tick. Stock moves as rate × dt; buffers fill and drain; a full buffer stalls its producer, and stalled machines are skipped until something changes (Factorio-style sleep).
- Haulers are **not** simulated per item. A route carries an aggregate flow; the client draws drones and barges as presentation tokens spaced from the server rate (OpenTTD keeps cargo as counted packets, not per-item entities).
- A heavier route-planning pass (who sends how much where, OpenTTD cargo-distribution style) runs rarely (target every 60 s) and its result is a set of rates the 1 Hz tick follows.
- Upkeep and wear are charged on the ledger tick (target 0.2 Hz) from the graph's running state.

### P8. Zone sharding and handoff

- A zone = a cluster of hexes owned by one sim. Players hand off at hex borders: the source zone freezes the player's last state, the target zone accepts it, the client swaps baselines.
- Handoff target ≤ 250 ms with no lost intents (intents during handoff are queued and replayed in the target).
- Border interest: zones exchange a thin border-band of entities so players see across.
- Soft cap per hex stays 32 Houses (`shared/hex_shard_apply::SOFT_CAP_HOUSES`) until a load test says otherwise.

### P9. Weekly war instancing and scale

- War maps are **instances** scheduled in advance, so capacity is allocated before the window opens.
- Interest LOD inside the war: full rate within 60 m, reduced beyond, counts per cell beyond 200 m (targets).
- Companions excluded (§6). Lethal only inside the declared instance.
- Results (holds, tons delivered, restored) write node rights for the next week through the ledger.
- Holiday ceasefires (§7.4) block instance creation for their window.

### P10. NPCs and companions on the server

- Scheduled NPCs online use the same agent model as §5 with the same LOD tiers; companions use §6 rules.
- They live in the zone tick and its budget; L2/L3 NPC Houses run on the logistics tick.

### P11. Observability

- Per-system tick time histograms, tick overrun counter, entity counts per zone, bandwidth per client, intents per second, rejects by code, handoff time, ledger invariant status, logistics graph solve time, §4 indicators.
- Reuse the configs already on disk under `observability/` (Prometheus, OTel collector, Grafana). No new hosted service.

### P12. Load testing

- Headless bot clients that replay recorded intent streams over loopback against the shard.
- Scenarios: quiet town; harvest crowd at one well cluster; logistics-heavy guild (10,000 route edges); war instance at 2 × 100; holiday crowd at one Place.
- Pass = every budget in the Budgets table below holds at p99 for 30 minutes. The harness is built early (card 10 in §10) even though it is listed last here, because every other budget depends on it.

### Budgets (all targets, none measured)

| Budget | Target |
|---|---|
| Zone sim tick | 20 Hz (50 ms); p99 tick work ≤ 25 ms (50% headroom) |
| Logistics tick | 1 Hz; graph solve ≤ 100 ms for 10,000 edges |
| Route planning pass | every 60 s; ≤ 2 s, spread over ticks |
| Ledger / upkeep tick | 0.2 Hz |
| Votes, calendar | event-driven, minute scale |
| Bandwidth down per player | ≤ 8 KB/s average in a normal zone; ≤ 24 KB/s peak in a war instance |
| Bandwidth up per player | ≤ 2 KB/s |
| Players per zone | 200 soft cap |
| Houses per hex | 32 soft cap (existing) |
| War instance | 2 × 100 first target; 2 × 250 stretch |
| Companion share | ≤ 15% of zone entities |
| Zone handoff | ≤ 250 ms, zero lost intents |
| Event log flush | every 100 ms |
| Snapshot cadence | 5 min per zone |
| Crash recovery | ≤ 30 s replay |
| Client prediction | verb local ack same frame; server settle within RTT + one tick |

**The aim versus TTD.** TTD and OpenTTD are the inspiration for how much simulation a small budget can carry. Our target is to carry a TTD-scale logistics network **per guild**, shared by many players, inside the logistics budget above. That is an aim to test, not a claim.

---

## 9. Where this plan meets existing law

These lines stand until the Steward lifts them on a named ticket. The plan is designed to fit inside them where it can.

| Existing line | Where | What this plan does |
|---|---|---|
| Title Online grey; no listen; `server/` parked | `ONLINE_LADDER`, `AGENTS.md` | Everything in §6–§8 server-side is HOLD |
| Drones / robots as product refused (Offline 1.0); automated harvest bots refused Online until named law | `OFFLINE_ECONOMY_COURT` | Guild automation (§3b) and online droid companions (§6.5) are proposed for Online 2.0. Offline droids stay refused until Sherif lifts the line (§6.7). Offline NPC Houses stay hands-and-House scale |
| Market HOLD; no gold, ticker, sell | `OFFLINE_SKU`, `CREDIT_RESERVE_LOGISTICS` | R3 is designed but HOLD; no server-minted gold; ratios in Reserve units |
| No race lobby at Title | `GDD_IMMERSION_REVISION` §4 | Peoples chosen after House |
| Reputation grind refused as belonging | `GDD_IMMERSION_REVISION` §2.2 | Reputation = standing, moved only by verbs |
| Weekly wars parked Sky weather | `GDD_IMMERSION_REVISION` §2.1 | §3c is Online only, score stays tons + restored |
| No daily login, loot box, season pass | `GODSPEED_PREP` | Holidays are calendar mood, never streaks or paid |
| Peace is the default; lethal is opt-in after the book (DeclaredLethal) | `GDD_IMMERSION_REVISION` §2.1, §4 | `world_harm_mode` defaults to `off`; `declared` reuses the existing DeclaredLethal path; Sanctuary Prime is always protected (§3g) |
| Hour finishes with zero persons; no person combat | `NPC_SCHEDULE_SPEC` | Kept for offline NPCs and companions |
| New `shared` files need a named PATH | `AGENTS.md`, `NEXT_NAMED_CARDS` | Cards below that add a file are JUNCTION until the Steward names them |

---

## 10. Roadmap (cards)

One CARD, one PR, PATHS listed. Cheapest high-leverage first. **These are proposals, not a queue.** Standing Next stays `docs/GODSPEED_PREP.md`; Dual and the Steward name any card before it is cooked. Core gate for every `shared` card: `cargo test -p shared -p rsil-identity` and `cargo test -p powrush-client --lib`. Online stays grey throughout.

### 1. CARD SIM-REPLAY-1 — determinism proof on what exists

**PATHS:** `shared/shard_sim.rs`

Add a pure `snapshot_digest` over `ShardClimate` + `ShardStanding` + `WeekAudit` and tests: same start + same tick count → same digest; a different command order → a different digest. No new file, no client. Proves the P1 law on today's sim and gives every later card an oracle.

### 2. CARD ECON-INDICATORS-1 — measure the spectrum

**PATHS:** `shared/economy_indicators.rs` (new) · `shared/lib.rs` (+1 `pub mod`)

Pure functions for restore ratio, climate health, Flow share, holdings Gini and the derived regime label, fed from existing `WeekAudit` / `ShardStanding` / allocate counts. Tests with fixed fixtures. No UI. JUNCTION until the Steward names the new file.

### 3. CARD FLOW-GRAPH-1 — aggregate logistics core

**PATHS:** `shared/flow_graph.rs` (new) · `shared/lib.rs` (+1 `pub mod`)

Nodes with rate and buffer, edges with capacity and upkeep, a deterministic `step(dt)` that moves integer milli-tons and stalls full producers. Tests: bottleneck equals min capacity; stalled producer sleeps; upkeep charged per step; the existing `vertical_factory` chain expressed as a graph gives the same arrival. No client, no drones on screen. JUNCTION (new file).

### 4. CARD NPC-AGENT-1 — offline agent model

**PATHS:** `shared/npc_agent.rs` (new) · `shared/lib.rs` (+1 `pub mod`)

Needs, schedule blocks keyed by a day-period index, utility goal pick with id tie-break, 16-slot memory ring, standing-driven greeting choice. Deterministic tests. Hour still finishes with zero agents (no client wiring). JUNCTION (new file).

### 5. CARD NPC-LOD-1 — tiers and budget accounting

**PATHS:** `shared/npc_agent.rs`

L0–L3 tiers, deterministic promote/demote, a per-tick work counter tests can assert against the §5.4 counts. No timing claims.

### 6. CARD COMPANION-RULES-1 — companion policy as pure rules

**PATHS:** `shared/companion_policy.rs` (new) · `shared/lib.rs` (+1 `pub mod`)

Pure checks: opt-in default off; allowed vs excluded content (raid boss, war, territory, governance); human-first slot yield; per-party cap; zero ledger output except one-time quest completion; reduced standing rate; hire / own / guild-summon paths paid in in-game resources only (no real-money path exists); upkeep and Resting when unpaid; rare and seasonal visitor windows from seed + calendar. Tests for each abuse row in §6.4 and each cost rule in §6.6. JUNCTION (new file). Offline droids stay refused until Sherif lifts the `OFFLINE_ECONOMY_COURT` line (§6.7).

### 7. CARD SHARD-EVENTLOG-1 — append-only log + replay in the lab

**PATHS:** `shared/hex_shard_apply.rs` · `powrush-shard/src/main.rs`

Append every applied envelope with tick and sequence to `events.jsonl` under `--data`; a replay of the log rebuilds `ledger_snapshot.json` byte-identical. Loopback only, parked crate, no Cargo change. HOLD until the Steward names `powrush-shard` PATHS.

### 8. CARD LEDGER-TX-1 — double-entry integer ledger

**PATHS:** `shared/ledger_tx.rs` (new) · `shared/lib.rs` (+1 `pub mod`)

Accounts, tagged transaction kinds, idempotency ids, named sources and sinks (including `challenge_reward` with a per-cycle emission cap, `spoilage`, `wear` and an insurance pool), invariant check. Tests. JUNCTION (new file).

### 9. CARD VOTE-ENGINE-1 — proposals, ballots, calendar as pure rules

**PATHS:** `shared/vote_engine.rs` (new) · `shared/lib.rs` (+1 `pub mod`)

Proposal → review → debate → vote → enact state machine; eligibility (guild age, active members, faction); square-root weighting; quorum and two-faction pass; spam limits; canon holidays cannot be removed; ballot types from §7.5 with bounded parameters and auto-revert. Calendar derivation from canon + enacted events. Tests. Offline it can drive a NPC-council calendar later. JUNCTION (new file).

### 10. CARD SHARD-LOADTEST-1 — loopback bot harness

**PATHS:** `powrush-shard/src/main.rs`

A `--load-bots N --script file` mode that replays intents against the loopback listen and prints tick and bandwidth stats. Loopback only. HOLD with card 7.

### 11. CARD EPIPHANY-PLAY-1 — epiphany map, docs first

**PATHS:** `docs/LAYERED_GAMES_AND_SERVER.md`

Docs only. For each epiphany in §11, write the **observation bar** a human playtester checks: the still frame or moment that shows the system teaching (e.g. restore ratio visibly greening a hex on **L**), the honest-fail frame (a Stressed frontier after an extract-only week), and the indicator from §4.4 that would show it in numbers later. Map each one to the existing `content/epiphany_scenarios/*.json` where one already fits, and name its two codex entries (conclusion and opposite) and its §11.3 reward trigger. No client, no shared code, no new text plate, no invented playtest minutes. Online grey. Later Hands cards (indicator reads, NPC memory lines) wait on Sherif naming PATHS.

### 12. CARD RELIABILITY-1 — fluctuation and reliability on the flow graph

**PATHS:** `shared/flow_graph.rs` (after FLOW-GRAPH-1 lands)

Add reliability per node and edge, seeded breakdown rolls, service intervals, the maintenance-record ceiling, station rating, spoilage per sim day, and bounded seeded yield ε (§4.5). Tests: same seed gives the same breakdowns; a serviced route's ceiling rises and its variance narrows; a neglected one stalls; ε never leaves its bound. No client, no wall clock.

### 13. CARD HARM-MODE-1 — `world_harm_mode` as pure rules

**PATHS:** `shared/harm_mode.rs` (new) · `shared/lib.rs` (+1 `pub mod`)

`WorldHarmMode { Off, Declared, Frontier, Open }`, default `Off`, plus a pure `may_harm(actor, target, hex, mode, war, ceasefire, protection)` check, the consequence table for malicious vs naive acts, ruin-not-delete, and damage caps (§3g). Tests: `Off` never allows; offline default is `Off`; Sanctuary Prime, homestead cores, new players and ceasefires are always protected. No client, no socket. JUNCTION (new file). Online grey.

### HOLD until `online yes` and named PATHS

- **SERVER-SHELL-1** — headless zone sim shell (plain Rust or `bevy_ecs` 0.14) calling `shared`; needs a crate decision and Cargo, so Steward names it.
- **INTEREST-1**, **DELTA-SNAP-1**, **PREDICT-RECON-1** — P5–P6.
- **ZONE-HANDOFF-1** — P8.
- **GUILD-AUTOMATION-1** — needs the `OFFLINE_ECONOMY_COURT` lift for drones.
- **WAR-INSTANCE-1** — P9.
- **COMPANION-SERVER-1** — §6 rules on the server tick and budget.
- **HOLIDAY-CALENDAR-1** — canon calendar offline mood (client) and server calendar online.
- **OBS-1** — wire `observability/` configs to the shell.
- **MARKET-R3-1** — needs the Market HOLD lift.

---

## 11. Epiphanies: philosophy through play

An epiphany here is a realisation the **systems** let a player reach on their own. The game never states it. The player notices it because the land, the ledger or the people around them change. This extends what is already on main: `docs/RBE_FIRST_HOUR.md` acceptance ("If I only take, the glow fades. If I allocate, the climate comes back."), `docs/EPIPHANY_CATALYST_SYSTEMS.md` and `content/epiphany_scenarios/`.

**Technique borrowed, nothing else.** TTD and Factorio teach by legible chains where a bottleneck shows itself. ARK teaches by upkeep and decay you can see. Conquer Online teaches by a weekly result everyone remembers. WoW teaches by reputation that opens or closes doors. Oblivion teaches by people who remember what you did. We borrow those teaching methods, not their content.

### 11.1 The epiphanies

| Epiphany | System that teaches it | The moment the player feels it | Honest failure when it's absent |
|---|---|---|---|
| **Restoring outlasts taking** | Node life (Idle → Stressed), restore ratio \(U_w / T_w\), week bill on **L** | A hex they restored is still green weeks later, while a stripped neighbour stays tired. The restore ratio visibly heals the land. | A take-only House sees its wells go Resting and Stressed, yields fall, and the bill shows tons with nothing restored. Nothing is wiped; it can be mended. |
| **Sharing outperforms hoarding** | Flow vs Reserve, guild commons (R4–R5), needs-met ratio, holdings Gini (§4.4) | A commons guild's members all meet their needs in a week when a hoarding guild with more stock still has idle capacity and unmet members | Hoarding works for a while. Then depots fill, producers stall, upkeep charges on idle stock, and neighbours stop trading. The hoard is real, and so is its cost. |
| **Scarcity is often a logistics problem** | The flow graph (§3b, §8 P7): routes, depots, capacity, upkeep | A shortage disappears after the player adds one route or depot, with no new extraction at all | A House that answers every shortage by taking more strips its nodes while the real bottleneck, a full depot or a missing route, stays. The chain view shows the stalled edge. |
| **Unity beats domination** | Weekly war score (tons + restored + holds), treaties, holiday ceasefires, vote engine (§3c, §7) | An alliance of smaller guilds outscores a dominant one because they restored and hauled together, or a ceasefire week lets a shared project finish | A dominating guild can win a week by force, inherit a stripped frontier, and lose co-sponsors for votes. The win stands; so does the cost. |
| **Stewardship matters more than ownership** | Node rights are stewardship for a week, not deeds (§3c); standing (§3d); NPC memory (§5.1) | A node they tended keeps producing under its next steward, and NPCs greet them for it, even after they lost the war for it | An owner who treats a node as property and strips it before the rights pass on sees cooler greetings, lower standing, and a node that won't recover fast for anyone. |

### 11.2 Design rules

1. **Never preach.** No lecture, no text wall, no narrator stating the lesson, no "you have learned" toast. One line at most, and only as a person's greeting (§5.1) or a well's mood word. H hides it.
2. **Let players reach the opposite conclusion.** Hoarding, domination and extraction must also be able to win, for a while and at a real cost. If only one strategy can succeed, the game is preaching through its rules.
3. **Discovery is optional.** No epiphany gates a Place, a quest, a war or the hour. A player who never notices any of them still finishes and has fun.
4. **Show, then measure.** The land, the ledger and the people show it first. Numbers (§4.4) are there for players who look, never pushed.
5. **Reward the noticing, never a "right" answer.** An epiphany earns a quiet codex entry for having noticed, whichever way the player went. Reaching the opposite conclusion (hoarding paid off, domination won the week) earns **its own** codex entry, written just as respectfully. Nothing about the reward says one path was correct.
6. **Failure is honest and recoverable.** Every "absent" path in §11.1 can be mended. Nothing is wiped (`docs/GODSPEED_PREP.md`: failure teaches).
7. **Same rules offline and online.** Offline, NPC Houses (§5) make each epiphany reachable alone. Online, real players make it real.

### 11.3 Thoughtful rewards for discovery

Discovery, whether an epiphany (§11.1) or an Easter egg (§7.8), earns rewards that are **meaningful but give no power and no economy edge**.

| Reward | What it is | Where it lives |
|---|---|---|
| **Codex entry** | A short journal page in the House book, in the world's voice, never a lesson summary. Each epiphany has two: one for the conclusion, one for its opposite. | Offline and online |
| **Crest flourish or tabard thread** | A small stitch, edge or charge the player may add to their own crest or tabard, on that one item (`docs/FACTION_HERALDRY.md`: physical, no transmog, no wardrobe). It fits the realistic faction-gear customization and fades and restitches like any finish. | Offline and online |
| **Rare title** | A worn-if-chosen title, e.g. "Mender of Long Weeks" for restoring outlasts taking, or "Keeper of the Full Depot" for its opposite | Offline and online |
| **Lore or sound reveal** | A small extra: one more line from an NPC who remembers, or a faint phrase in a Place's bed | Offline and online |
| **First finder mention** | One line in the server history (§7.6) for the first player on each server to reach a given discovery | Online, **opt-in**, off by default |

**Reward law:**

- **Tiers for discovery are prestige only.** A rarer or harder discovery (§4.6 tier score) earns a rarer cosmetic, never more power. Challenge rewards with real output are a separate kind and live in §4.6.
- **Never the ledger.** No Reserve, stock, standing, currency, stat, drop rate or yield. Nothing a reward gives can be traded, sold or spent (§4, §8 P3).
- **Never sold.** No discovery reward, thread or title is ever on a store, in a bundle or behind real money.
- **Never loud.** No server-wide broadcast, no fanfare, no pop-up at the moment of discovery. The codex entry is waiting the next time the book is opened. The first-finder line sits quietly in the server history.
- **Reward the noticing.** Detection uses the same indicators as §4.4 (for example, a restore ratio held at or above 1.0 for three House weeks, or a guild's needs-met ratio rising after it opened a commons). Each epiphany has a matching opposite trigger, so both conclusions are recognised.
- **Once per character.** Each reward is earned once. Repeats give nothing extra, so there is nothing to farm.
- **Deterministic and testable.** Triggers read sim state and the calendar, never wall clock inside \(F\).

### 11.4 Card

See §10, CARD EPIPHANY-PLAY-1 (docs first, Online grey).

## 12. Refuse

- Lighting Title Online, adding a listen or public bind, `0.0.0.0`, unparking `server/`, enabling steamworks, bumping Cargo or Bevy, from this file or any card it names.
- Gold, price ticker, auction house, NFT, paid power, loot boxes, daily login, season pass.
- A separate currency or score per layer.
- Companions that replace humans, appear in presence counts, fight in wars, face raid bosses, vote, or produce stock.
- Companions, droids or summons sold for real money.
- Holidays rooted in real-world religious or national days, or holidays that sell anything.
- Borrowing characters, names, art, text or UI from any reference game.
- Performance, scale or certification claims. Every number here is a target.
- Epiphanies delivered as lectures, text walls or quizzes, or rewards that favour a "right" answer over its opposite.
- Discovery or RUSH rewards that touch the ledger, pay currency or stats, are sold, or are announced loudly.
- Destruction that deletes: harm leaves a ruin with salvage, never a wipe. Harm to anyone's work while `world_harm_mode` is `off`, on Sanctuary Prime, on homestead cores, on new players, or during a ceasefire.
- Offline harm by anyone other than opted-in NPC raids under a local `world_harm_mode` the player turned on.
- Unbounded or wall-clock randomness in yields, breakdowns or spoilage; challenge output beyond its per-cycle emission cap.
- Transmog or wardrobe as a discovery reward; threads and flourishes stay on the one item (`docs/FACTION_HERALDRY.md`).

## 13. Cite

`README.md` · `AGENTS.md` · `docs/ONLINE_LADDER.md` · `docs/NET_OFFLINE_CONTRACT.md` · `docs/SHARD_JOIN.md` · `docs/SIM_AND_HAND_CANON.md` · `docs/STUDIO_ARCHITECTURE_ORDER.md` · `docs/GDD_IMMERSION_REVISION.md` · `docs/CREDIT_RESERVE_LOGISTICS.md` · `docs/NPC_SCHEDULE_SPEC.md` · `docs/OFFLINE_ECONOMY_COURT.md` · `docs/OFFLINE_SKU.md` · `docs/RBE_FIRST_HOUR.md` · `docs/FUN_WITHOUT_WOW.md` · `docs/PRE_RELEASE_LAW.md` · `docs/GODSPEED_PREP.md` · `docs/PLAYABLE_RACES.md` · `docs/FACTIONS_OVERVIEW.md` · `docs/COUNCIL_SYSTEM.md` · `docs/LORE_BIBLE.md` · `docs/REPLICATION_PREDICTION_ARCHITECTURE.md` · `docs/SPATIAL_INTEREST_ARCHITECTURE.md` · `docs/BEVY_CLIMB_PLAN.md` · `docs/EPIPHANY_CATALYST_SYSTEMS.md` · `docs/FACTION_HERALDRY.md` · `docs/FLEET_CLASSES.md` · `docs/DRAEK_FLEET_AI_SYSTEMS.md` · `docs/AMBROSIAN_CRYSTALLINE_SHIP_DESIGNS.md` · `docs/CYDRUID_ORGANIC_SHIP_DESIGNS.md` · `client/src/living_ecology.rs` (cite only) · `content/epiphany_scenarios/` · `shared/hex_protocol.rs` · `shared/hex_shard_apply.rs` · `shared/vertical_factory.rs` · `shared/fabricator.rs` · `shared/war_week.rs` · `shared/week_audit.rs` · `shared/shard_standing.rs` · `shared/shard_sim.rs` · `shared/nevc_adapter.rs` · `client/src/living_day.rs` (cite only) · `powrush-shard/README.md` · `server/src/spatial/` (cite only)

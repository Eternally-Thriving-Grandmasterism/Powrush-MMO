# GDD-LOGISTICS-SOCIETY-1: logistics and society pillar

Docs only. No code, assets or economy code. Online stays grey. No certification, legal or health claims.

Base: main `fe6bdd79` (after #783 PROFESSIONS-BENCH-1). Bounds: issue #738, Core's card comment 6431587875 and addenda 6431607614, 6431632915, 6431647758, 6431704396 (card named by Core). This is a new pillar, so it sits in one focused doc and links the three live GDD files instead of editing them:

- [`docs/GDD_ADAPTATION.md`](../GDD_ADAPTATION.md) — GDD 1.5–2.0 name map and keep / transform / refuse.
- [`docs/GDD_IMMERSION_REVISION.md`](../GDD_IMMERSION_REVISION.md) — §2.3 Transport Tycoon Deluxe and §3 Rung D (Guild / House logistics on credit).
- [`docs/design/LEGACY_GDD_RECONCILIATION.md`](LEGACY_GDD_RECONCILIATION.md) — §5 "Cydruid home" and §6 conflicts with current canon.

Everything below that is not quoted canon is a **PROPOSAL**. Nothing here is law until Sherif confirms it. Any new name is **HELD for Sherif's OK**.

---

## 0. Sherif's words (verbatim)

All five are Sherif's own chat messages to Lead Mate on 2026-10-26 (America/Toronto, ET), copied character for character, typos kept. Source: Lead Mate's session transcript; Core relayed each one into #738 (comment ids beside each).

**9:36 PM ET — the pillar (card 6431587875, item 1):**

> "Remember, Powrush-MMO GDD also should include Transport Tycoon Deluxe style systems for transporting and harvesting plus more like Factorio and other similar games which will typically be controlled by guilds or similar setups such as gangs, corporations, etc., and players will get to role play like real-life with differing rules and laws to engage in business and war to discovery naturally and practice ultimately real diplomacy with consequences, rewards and risks all with server-wide impact for each decision made and action taken by every entity, my Dear Brilliant Legendary Mate!"

**9:39 PM ET — automation, sabotage, alarms, radar (addendum 6431607614):**

> "These transport systems can eventually become fully autonomously run like Stardew Valley farms, and even more much beyond, plus can be sabotaged by players or accidentally damaged by those who don't know any better, with reports and alarms for affected guilds/etc. Like SC2 minimap and so on; which will be a developed and unlocked feature for each guild/similar/etc., just like in Dune2 and Dune2000 you need a Radar to unlock the advanced minimap in a rational and logical way for each aspect of Powrush-MMO, Mate!"

**9:41 PM ET — group registration and stance (addendum 6431632915):**

> "Guilds and similar entities like gangs, corps, NFP, etc., once registered can then opt to be Nuetral, or join either of the 2 primary factions of Powrush-MMO, Mate!"

**9:42 PM ET — home bases (addendum 6431647758):**

> "Based on their decisions they get one of 4 primary bases on Earth for Nuetral, one in the cyborg hollows of the Cydruids deep in the Earth. The other main bases are the respective motherships with cool names like TAUN."

**9:47 PM ET — Cydruids and the split base (addendum 6431704396):**

> "Cydruids are originally Nuetral, but their leadership quickly decided to join against the Draeks once they saw the source of destruction root cause; however, perhaps a half Nuetral half against Draeks setup for the cyborg underground base makes sense in the stalactite city or cities, Mate!"

---

## 1. Pillar content (card items 2, 6)

The pillar in plain words: groups own and run real supply chains, and every group lives under its own rules inside one shared world where business, war and diplomacy have server-wide results.

### 1.1 What is already on main (do not duplicate)

- [`docs/LAYERED_GAMES_AND_SERVER.md`](../LAYERED_GAMES_AND_SERVER.md) §3b "Guild automation (Factorio chains, TTD logistics)" already writes extractors, depots, routes, haulers (drones, crawlers, barges), standing orders for robot workers, upkeep as the teacher, and throughput (tons moved + nodes restored) as the score. Chains extend `shared/vertical_factory.rs` and `shared/fabricator.rs` (cited there; this card touches no code).
- §3d of the same file already says faction is "a guild-level allegiance that can change, with a cost".
- §3g already writes `world_harm_mode` (default `off`), malicious vs naive destruction, ruins not deletion, insurance, caps and review.
- [`docs/GDD_IMMERSION_REVISION.md`](../GDD_IMMERSION_REVISION.md) §2.3 keeps TTD's harvest → refine → ship → manufacture chain and tonnage score, and **refuses** money as the verb and **transforms** vehicle fleets / timetables into hand-and-satchel carrying. Sherif's 9:36 words ask for TTD-style transport systems. That tension is recorded as open question Q6, not resolved here.

This doc adds the society layer on top: registration, stance, home bases, laws, reports, the radar unlock, and the automation ladder.

### 1.2 The four strands (PROPOSAL)

| Strand | What it means in Powrush | Nearest canon |
|---|---|---|
| **Logistics (TTD-style)** | Routes between stations; carriers with capacity; cargo with a source and a destination; harvesting supply that feeds the route | LAYERED §3b; IMMERSION §2.3, Rung D |
| **Production chains (Factorio-style)** | Ratio chains where a bottleneck is visible and fixable; more tiers = more refine/assemble steps | LAYERED §3b; [`PROFESSIONS_BENCHMARK.md`](PROFESSIONS_BENCHMARK.md) (gathering/crafting feed the chain) |
| **Group ownership with own rules and laws** | Guilds, gangs, corporations, not-for-profits and similar groups each write a charter: who may use their routes, tolls, whom they shoot, whom they trade with | §2 below |
| **Role play of business, war, diplomacy** | Every entity's act (a contract, a raid, a treaty, a sabotage) lands in the ledger and event log and moves standing and the server meter | [`SERVER_MORALITY.md`](SERVER_MORALITY.md) §3; COMBAT_AND_PVP §16 |

### 1.3 Supply feeding logistics

[`PROFESSIONS_BENCHMARK.md`](PROFESSIONS_BENCHMARK.md) §2 and §4 already map gathering → refining → crafting and work orders. Its output is this pillar's cargo. Blueprints follow [`AUDIO_RADIO_QUEST_DESIGN.md`](AUDIO_RADIO_QUEST_DESIGN.md) §5.2 unchanged: learned recipes account-bound; limited-use copies tradeable; so copies are real cargo a convoy can carry and an enemy can intercept.

---

## 2. Groups, registration and stance (card items 12–14)

### 2.1 Registration (PROPOSAL)

- Any player-led group registers as one **group kind**: guild, gang, corporation, not-for-profit, or "other" (Sherif's "etc."). Kind is a label for role play and for the charter template; it gives **no stat bonus** and no exclusive item.
- Registration is an in-world act at a registry desk in the group's home base (§3), signed by founders. The founder count and any fee are open numbers (Q9). Fees, if any, post through the one ledger (COMBAT_AND_PVP §1 pillar 5, "One economy").
- A registered group gets a **charter**: rules of engagement, toll and trade policy, membership rules, and who holds which role. Patterns from EVE (corp roles, standings) and SWG (elected mayor, city rank) are in §7.

### 2.2 The three stances

Canon on main names exactly two primary powers. [`docs/FACTIONS_OVERVIEW.md`](../FACTIONS_OVERVIEW.md) §1: "the conflict between two vastly different civilizations" — the **Quellorian / Aetherion Luminari Alliance** and the **Draek Dominion**. [`docs/design/FACTION_LEADERS.md`](FACTION_LEADERS.md) §3: "§1 names two powers" and "This file adds no third mothership faction." [`docs/design/COMBAT_AND_PVP.md`](COMBAT_AND_PVP.md) §8 invitation table names the same two.

After registering, a group picks one stance: **Neutral**, **Alliance**, or **Dominion**.

| Area | Neutral | Alliance | Dominion |
|---|---|---|---|
| **Laws** | Group charter + the server's common rules (marks, infamy, domes per COMBAT_AND_PVP §5–§7) | Charter must sit inside Alliance law (no Clear kills, mirrors the §8 righteous path) | Charter must sit inside Dominion law (§8 criminal path); harsher internal discipline is role play, not a power scalar |
| **War** | May not declare faction war; may defend; may hire out as escorts/mercenaries (PROPOSAL) | Joins faction war fronts and the weekly guild war (COMBAT_AND_PVP §10) on the Alliance side | Same, Dominion side |
| **PvP** | Unchanged: PvP stays opt-in by place (COMBAT_AND_PVP §1 pillar 3); stance never forces PvP in peace zones | Same, plus faction-war instances | Same, plus faction-war instances |
| **Trade** | Trades with both sides; neutral hubs | Trades freely inside the Alliance; trade with Dominion groups is possible but logged and costs standing (PROPOSAL) | Mirror of Alliance |
| **Diplomacy** | Treaties with anyone; may broker | Treaties inside faction policy | Treaties inside faction policy |
| **Server-wide consequences** | Acts move the server meter like anyone's (SERVER_MORALITY §3.2) | Acts also move faction standing (DIPLOMACY_AND_WORLD_SIMULATION §2.2) | Same |
| **Home base** | Earth surface base, or the Neutral half of the Cydruid hollows (§3) | TAUN, or the anti-Draek half of the Cydruid hollows (§3) | The Brood Spire |

### 2.3 Switching stance (PROPOSAL — costs and cooldowns are open numbers)

- A stance change is a group act (charter vote). LAYERED §3d already says allegiance "can change, with a cost".
- **Cooldown:** a minimum hold after any change, so a group cannot flip before a battle. EVE has the same idea for its alliance capital ("takes seven days to take effect") and for vulnerability windows (a change takes effect after the next window). Powrush's length is open (Q9).
- **Cost:** standing loss with the side left; home base moves (§3); contracts with the side left are wound down. No real-money cost, ever.
- **No mid-war flip:** a group in an active war instance cannot change stance until the war ends plus a cool-down (EVE pattern: wars end with a 24-hour cool-down "to avoid a particular engagement").
- Members who disagree may leave the group; individual leaving is free.

### 2.4 Double agents (permanent rule)

War, PvP and double agents stay permanent ([`docs/POWRUSH_LOOP_NOTES.md`](../POWRUSH_LOOP_NOTES.md) §0; [`SERVER_MORALITY.md`](SERVER_MORALITY.md) §2.2). This doc keeps SERVER_MORALITY §2.2 as written: a covert tie rides on a person, "A personal covert tie never flips a guild", no tell on the nameplate, exposure from the event-log trace. New for groups (PROPOSAL): a group's charter may set its own vetting and its own internal penalties for an exposed member; a group may run counter-intelligence via its reports (§6). A whole group cannot be secretly registered to two stances.

### 2.5 Reconciling with the individual invitation (recorded, not resolved — Q2)

COMBAT_AND_PVP §8 today makes faction membership an **individual**, non-permanent invitation gated by infamy at Outlaw (300) or righteousness at 200, each "held 14 real days". Sherif's 9:41 rule is **group-level** opt-in at registration. How the two coexist is open question Q2.

---

## 3. Home bases (addenda 3 and 4)

Four primary home bases, assigned by stance:

| # | Base | Stance | Canon |
|---|---|---|---|
| 1 | **An Earth surface base** | Neutral | FACTION_LEADERS §1 Roster: Ronan Ashcroft, Warden, seat "Earth's surviving cities". Specific base name not set — see Q8 |
| 2 | **The Cydruid cyborg hollows**, deep in the Earth | Split (PROPOSED, §3.1) | FACTION_LEADERS §1: Orin Hollowell, "Archdruid, Keeper of the stalactite city", seat "The upside-down stalactite city in Earth's hollows"; LEGACY_GDD_RECONCILIATION §5 "Cydruid home" |
| 3 | **The Auroral Unification Nexus (TAUN)** | Alliance mothership | FACTIONS_OVERVIEW §2 and §5 "Finalized"; FACTION_LEADERS §1 (Solmaris Orynthe's seat) |
| 4 | **The Brood Spire (TBS)** | Dominion mothership | FACTIONS_OVERVIEW §3 and §5 "Finalized"; [`docs/BROOD_SPIRE.md`](../BROOD_SPIRE.md) §1 ("mobile production, domination, and command hub") |

No new mothership names are proposed; both are canon and finalized.

### 3.1 Cydruid lore and the split base (PROPOSED per Sherif's "perhaps")

- **Lore (Sherif, 9:47):** Cydruids start Neutral. Their leadership (Archdruid Orin Hollowell) soon sides against the Draek Dominion once they see the root cause of the destruction. This sits with FACTION_LEADERS §3, which already calls Humans and Cydruids "independent Peoples, allied by need".
- **Base (PROPOSED):** the stalactite city or cities in the hollows are a **split home**: half Neutral, half aligned against the Draeks (Alliance-side). Kept as Sherif's own wording; written as a proposal because he said "perhaps".
- No Cydruid shop, no second currency (LEGACY_GDD_RECONCILIATION §5, PLAYABLE_RACES §5) — unchanged. No Place, door or mesh is added here.

Sub-details recorded for Sherif (not resolved): Q3 one city with two districts, or separate cities; Q4 how the two halves relate to the Earth Neutral base and to TAUN; Q5 border, trade and PvP rules between the halves.

---

## 4. Automation ladder (card item 6)

From manual to fully autonomous, per group (PROPOSAL). Each rung is unlocked by an in-world prerequisite (§8 rule).

| Rung | What runs | Prerequisite (in-world) | Pattern source |
|---|---|---|---|
| A. Manual | Player carries and crafts by hand | None | Stardew (hand watering before sprinklers) |
| B. Assisted | A single device does one chore each morning in a small radius | Crafted device from gathered materials + profession skill | Stardew Sprinkler / Auto-Grabber |
| C. Scheduled routes | Carriers run a fixed route with a timetable and slack | A depot/station pair and a registered route | OpenTTD timetables, shared orders |
| D. Networked logistics | Requests are filled automatically inside a covered area | A coverage hub that needs power and upkeep | Factorio logistic network / roboports |
| E. Standing orders | Robot workers run written rules ("keep depot B above 40 tons") | Already in LAYERED §3b | LAYERED §3b |
| F. Fully autonomous | A whole chain runs while the group is offline, within caps | All of the above + insurance + guards | Albion hideouts ("autonomous 'home base'") |

**Dopamine/progress guard:** automation trades hands-on proficiency for scale. Stardew does exactly this: the Auto-Grabber gives no Farming XP and costs animal friendship unless paired with an Auto-Petter. Powrush proposal: automated output earns no personal proficiency; it earns group throughput score only, so hands-on play keeps its reward and nobody levels while asleep. No real-money shortcut to any rung.

### 4a. Automation law, AUTOMATION-LAW-1 (council decision, #738 6443008117 under A16-A19)

Sherif, 2026-10-26 9:39 PM ET (full text in §0): "These transport systems can eventually become fully autonomously run like Stardew Valley farms, and even more much beyond, plus can be sabotaged by players or accidentally damaged by those who don't know any better, with reports and alarms for affected guilds/etc."

Online 2.0 only. PvP/Online HOLD is unchanged.

- **AL1 (council decision, A17):** this law lifts `OFFLINE_ECONOMY_COURT` :24 Online only, and only for guild-chartered automation in LAYERED §3b (:412, :603). Personal bots, companions and outside macros stay refused. Public AH, NFT and gold sink stay refused. Every automated unit is a world object owned by a registered group and shows in the event log.
- **AL2 (council decision):** care-only stays hands-only (LAYERED :121). Automation cannot Mend a Stressed node, take rare or first samples, or Take at a Glowing node meant for hand care.
- **AL3 (council decision):** node fatigue limits automation on its own (LAYERED :124-131, :603). Automated take obeys Resting/Stressed and upkeep keeps charging; no separate production cap.
- **AL4 (council decision, A16):** rung F (:145) runs while owners are offline only with insurance, guards and coverage in place. "Offline" here means the owners are logged off on an Online 2.0 server, never the Offline 1.0 SKU. While they are away the chain stays open to sabotage and accidents under §5 (:151-160) and LAYERED §3g (ruin, not delete, under world_harm_mode), and the group gets reports and alarms (§6).
- **AL5 (council decision, A18/A19):** automated output earns group throughput only: no personal Temper/Lumen/Ward, proficiency or ledger standing (:147). Nobody progresses while asleep; no real-money shortcut.
- **AL6 (PROPOSAL, A18):** each group's charter law governs its own automation: who may write standing orders and who answers for them, building on group-ordered sabotage co-attribution (:160).
- **AL7 (council decision):** rung F's server-wide unlock stays tied to the after-credits world shift (`CROWNSTONE_TRILEMMA_PATHS` :63, S5). Rungs A-E follow the in-world prerequisites in §4.
- **AL8 (untouched, A16):** offline droids and drones stay refused (LAYERED :444) until Sherif lifts that line himself.

**OPEN (no numbers):** how long offline chains may run; cap and upkeep amounts; insurance terms; alarm delivery. Every name is [NAME TBD].

---

## 5. Sabotage and accidents, and who is to blame (card item 7)

LAYERED §3g is the base law and stays as written (`world_harm_mode` default `off`; malicious vs naive; ruins not deletion; insurance; caps; review). This doc adds attribution and the stance layer (PROPOSAL):

| Case | Attribution | Consequence (outside war rules) | Inside war rules |
|---|---|---|---|
| **Deliberate sabotage** | Event-log row names actor, group, stance | LAYERED §3g malicious row; plus the victim group's charter may list the actor's group as hostile; diplomatic incident between stances | Allowed, capped (ruin not delete) |
| **Accident by an uninformed player** | Same row, flagged naive | LAYERED §3g naive row (small standing loss, automatic repair claim, confirm prompt before harming another's property); first-time naive acts open a short "how this works" hint instead of a penalty (PROPOSAL) | Normal war cost |
| **Double agent sabotage** | Logged to the trace; exposure rolled per SERVER_MORALITY §2.2 | "Cover gives no immunity" (SERVER_MORALITY §2.2) | Same |
| **Group-ordered sabotage** | Charter officer who gave the order is co-attributed (PROPOSAL) | Group standing loss, not only the hand that did it | — |

"Practice base reality without encouraging harm" ([`docs/POWRUSH_LOOP_NOTES.md`](../POWRUSH_LOOP_NOTES.md) §0): harm is possible and has real cost; there is no reward for harm outside war rules (LAYERED §3g, and its no-reward line), repair and redemption stay open, and every act is reviewable.

---

## 6. Reports and alarms (card item 8)

For the affected guild/gang/corp (PROPOSAL; online later, grey):

- **Alarm:** a one-line toast when a group asset takes damage, a route stalls, or a depot runs dry. Uses existing HUD rules: COMBAT_AND_PVP §16 puts toasts on `TOP_TOAST`, one at a time; no new anchor.
- **Jump to last alarm** and cycle recent ones (SC2 pattern: Space centres on "the location of the last transmission", cycling "up to 8").
- **Group report:** a ledger page (not a new window, per COMBAT_AND_PVP §16 "Existing ledger page") listing incidents with attribution (§5), losses, insurance claims and repairs. EVE's public War Report (kills per side, value, loss records) is the pattern; Powrush keeps reports to the group's own records, matching SERVER_MORALITY §2.2 "never a server-wide broadcast".
- **Accessibility:** every alarm has shape + word, never colour alone (COMBAT_AND_PVP §16; UI-BENCH-1 §3 colour-alone row).

---

## 7. Advanced minimap / radar, unlocked in-world (card items 9, 11)

### 7.1 The pattern

Dune II / Dune 2000: the radar minimap needs an Outpost and enough power; without it the radar is restricted (Dune II: friendly units only) or closed (Dune 2000). SC2: minimap alerts plus jump-to-last-alert. UI-BENCH-1 (`docs/visual-pass/playtest-ui-bench-1/BENCHMARK.md`):

- §4 "Minimap and compass" verdicts: full minimap with enemy/ally dots = **skip (for now)** (slab budget, combat framing); goal compass, audio ping, compass snap, one-gesture hide = adopt/adapt.
- §3 "Quest / objective tracker": one-line next step, cycle tracked goal, compass auto-pin.
- §10: PLAYTEST-TRACKER-1 proposed; "Deliberately not proposed: a minimap slab".

### 7.2 Proposal (fits UI-BENCH-1's skip)

- Every player keeps the compass (UI-BENCH-1 adopt). There is **no** free full minimap.
- A group that builds and powers a **radar structure** in its territory unlocks a group minimap for members inside that coverage. Tiers (PROPOSAL): (1) own assets only, (2) + terrain and resources, (3) + alerts on attacked assets, (4) + hostile contacts inside coverage. Lose power or the structure → minimap restricted, as in Dune.
- The radar structure is real infrastructure under LAYERED §3g: it can be sabotaged, so it is a war target (war + PvP stay real).
- Any future HUD minimap card (none exists yet; UI-BENCH-1 §10 lists none) must follow this unlock and the UI_LAYOUT_SYSTEM slab budget. Structure name is **HELD for Sherif's OK** (working label only: "radar structure").

---

## 8. General design rule: every advanced feature has an in-world prerequisite (card item 9)

Proposed rule for all Powrush GDD work: **every advanced feature is unlocked by a rational in-world prerequisite** — something built, powered, learned or earned in the world — never by a menu toggle or a purchase. Precedents on main: the radio unlock quest ([`AUDIO_RADIO_QUEST_DESIGN.md`](AUDIO_RADIO_QUEST_DESIGN.md) §2–§4, equal combat and Engineering paths); COMBAT_AND_PVP §1 pillar 2 "Realism with a sci-fi reason. Every rule has an in-world cause." Pattern precedents: Dune 2000 Outpost (radar + tech access), Stardew sprinklers by Farming level, EVE strategic upgrades needing an unbroken claim. Limits: the rule never gates levelling by place (below) and never sells the prerequisite.

---

## 9. Locked rules — how this doc keeps them

| Rule | Repo source | This doc |
|---|---|---|
| War + PvP + double agents permanent | POWRUSH_LOOP_NOTES §0; SERVER_MORALITY §2.2 | Stance never removes war or PvP; double agents kept as written (§2.4) |
| Practice base reality without encouraging harm | POWRUSH_LOOP_NOTES §0 (quote: "practice base reality without encouraging harm") | §5: harm costs, no reward outside war, repair and redemption open |
| Free levelling anywhere | **Not written verbatim in the repo** (same finding as PROFESSIONS_BENCHMARK §0 and AUDIO_RADIO_QUEST_DESIGN §6). Closest: COMBAT_AND_PVP §3 Proficiency "is not XP, not a damage bonus, and not a Place gate." | Stance, base and radar never gate where anyone may level; Neutral and faction players level in the same world |
| Progress / dopamine balance | JOY_WITHOUT_MALL, PEAK_MEMORY_LAW (as cited in PROFESSIONS_BENCHMARK §5) | §4 guard: automation gives group score, not personal XP; no paid rung |

---

## 10. Benchmark: adopt / adapt / skip

All sources fetched 2026-10-26 (ET) and loaded unless marked UNVERIFIED (§13). Patterns in our own words.

| Game | Pattern | Verdict | Powrush use |
|---|---|---|---|
| OpenTTD | Shared orders + timetables with slack keep carriers spaced; autofill measures a route | **adapt** | Rung C routes; slack teaches planning. IMMERSION §2.3 refuses timetable plates — see Q6 |
| OpenTTD | Station rating falls with long waits; local authority can obstruct | **adapt** | Route reliability as a standing read, not a money score |
| Factorio | Inserters move items between belts/machines; circuit conditions enable/disable | **adopt** (pattern) | Device-level automation tiers |
| Factorio | Logistic network: provider/requester chests inside roboport coverage; bots need power and recharge | **adapt** | Rung D coverage hubs with upkeep |
| EVE Online | Sovereignty hubs; upgrades need power/workforce; strategic index grows with an unbroken claim; Advanced Logistics Network needs strategic index 3 | **adapt** | In-world prerequisite rule (§8); tiers earned by holding |
| EVE Online | Corp standings (one-way), NRDS / NBSI rules of engagement | **adopt** | Group charter rules of engagement |
| EVE Online | War declarations: fee, 24h startup, allies join defender only, surrender, 2-week no-redeclare, 24h end cool-down, public War Report | **adapt** | Stance switching cooldowns, no mid-war flip, group reports (kept private) |
| EVE Online | Alliances led by an executor corporation; maintenance bill; capital change takes seven days | **adapt** | Group-of-groups; change delays |
| Albion Online | Guild alliances: allied guilds cannot attack each other; scheduled battles cancelled on joining; alliance tax | **adapt** | Treaties end scheduled conflicts; upkeep in Reserve |
| Albion Online | Hideouts: underground guild bases, attackable during construction, then "autonomous 'home base'"; limits per cluster | **adapt** | Rung F; home-base outposts |
| Star Wars Galaxies (Restoration) | Player cities: elected Master Politician mayor, rank by declared citizens, one residence at a time with 24h re-declare wait, specialization changeable once a week | **adapt** | Registration, citizenship, weekly change cooldowns |
| Stardew Valley | Sprinklers by Farming level; Auto-Grabber gives no Farming XP and costs friendship | **adopt** (balance lesson) | §4 dopamine guard |
| StarCraft II | Space jumps to last transmission, cycles up to 8; idle-worker icon above minimap; ally minimap ping | **adopt** | §6 alarms |
| StarCraft II | Attacked units flash white on the minimap (observers, HotS patch 2.0) | **adapt** | Alert tier 3 — UNVERIFIED (search snippet only) |
| Dune II / Dune 2000 | Radar needs Outpost + power; restricted/closed otherwise; Outpost also gates higher tech | **adopt** | §7 radar unlock |
| Dune 2000 | Outpost requires Barracks | **adapt** | Prerequisite chains |
| — | Full free minimap with enemy dots for everyone | **skip** | UI-BENCH-1 §4 skip stands |
| — | Money-as-verb tycoon balance sheet | **skip** | IMMERSION §2.3 refuse stands |

---

## 11. Cross-links

- [`docs/GDD_ADAPTATION.md`](../GDD_ADAPTATION.md), [`docs/GDD_IMMERSION_REVISION.md`](../GDD_IMMERSION_REVISION.md) §2.3 / Rung D, [`LEGACY_GDD_RECONCILIATION.md`](LEGACY_GDD_RECONCILIATION.md) §5–§6
- [`PROFESSIONS_BENCHMARK.md`](PROFESSIONS_BENCHMARK.md) (PROFESSIONS-BENCH-1) — gathering/crafting supply chains feed logistics
- [`AUDIO_RADIO_QUEST_DESIGN.md`](AUDIO_RADIO_QUEST_DESIGN.md) (AUDIO-RADIO-QUEST-DESIGN-1) — §5.2 blueprint trade; in-world unlock pattern
- [`../visual-pass/playtest-ui-bench-1/BENCHMARK.md`](../visual-pass/playtest-ui-bench-1/BENCHMARK.md) (UI-BENCH-1) — §3 Quest / objective tracker, §4 Minimap and compass, §10 follow-ups
- [`../LAYERED_GAMES_AND_SERVER.md`](../LAYERED_GAMES_AND_SERVER.md) §3b, §3d, §3g
- [`COMBAT_AND_PVP.md`](COMBAT_AND_PVP.md) §1, §3, §8, §10, §16; [`SERVER_MORALITY.md`](SERVER_MORALITY.md) §2.2, §3
- [`../FACTIONS_OVERVIEW.md`](../FACTIONS_OVERVIEW.md), [`FACTION_LEADERS.md`](FACTION_LEADERS.md), [`../BROOD_SPIRE.md`](../BROOD_SPIRE.md), [`../faction-diplomacy-v14.5.md`](../faction-diplomacy-v14.5.md)

---

## 12. Open questions for Sherif (recorded, NOT resolved)

- **Q1 (a)** [`docs/faction-diplomacy-v14.5.md`](../faction-diplomacy-v14.5.md) §"Factions (Current Canon)" lists five factions (Sovereign, Harvesters, Guardians, Innovators, Nomads), which conflicts with the two-power canon (FACTIONS_OVERVIEW §1, FACTION_LEADERS §3, COMBAT_AND_PVP §8). Is the v14.5 five-faction model superseded, or how do they relate?
- **Q2 (b)** COMBAT_AND_PVP §8 individual, non-permanent invitation (infamy/righteousness held 14 days) vs Sherif's group-level opt-in at registration. How do they coexist, or does one replace the other?
- **Q3** Cydruid hollows: one city with two districts, or separate cities?
- **Q4** How the Neutral and anti-Draek halves relate to the Earth-surface Neutral base and to TAUN.
- **Q5** Border, trade and PvP rules between the two halves.
- **Q6** Sherif's TTD-style transport (vehicles, stations) vs GDD_IMMERSION_REVISION §2.3, which transforms vehicle fleets/timetables into hand-and-satchel carrying and refuses money as the verb. Which wins for the logistics layer?
- **Q7** LAYERED §3d lists "unaligned Houses" and Peoples' landing Places (Cydruid → Heartwood) — confirm "Neutral" = "unaligned", and how home bases relate to landing Places.
- **Q8** Name of the Earth-surface Neutral base (none in canon; Ronan Ashcroft holds "Earth's surviving cities").
- **Q9** Numbers: founder count, registration fee, stance-switch cooldown and cost, radar tiers. None invented here.

---

## 13. Sources (fetched 2026-10-26 ET, loaded)

1. OpenTTD wiki — Timetable: https://wiki.openttd.org/en/Manual/Timetable
2. Factorio wiki — Inserters: https://wiki.factorio.com/Inserters
3. Factorio wiki — Logistic network: https://wiki.factorio.com/Logistic_network
4. EVE University wiki — Sovereignty: https://wiki.eveuniversity.org/Sovereignty
5. EVE University wiki — Corporation diplomacy (standings, wars, allies, War Report): https://wiki.eveuniversity.org/Corporation_diplomacy
6. EVE University wiki — Corporations: https://wiki.eveuniversity.org/Corporations (loaded via curl)
7. EVE University wiki — Alliances (executor corporation, maintenance): https://wiki.eveuniversity.org/Alliances (loaded via curl)
8. Albion Online — Guild Alliances (official news): https://albiononline.com/news/guild-alliances
9. Albion Online — Queen Update changelog (Hideouts, territory battles): https://albiononline.com/changelog/QUEEN%20UPDATE
10. SWG Restoration wiki — Player Cities: https://swgr.org/wiki/player_cities/
11. Stardew Valley wiki — Auto-Grabber: https://stardewvalleywiki.com/Auto-Grabber
12. Stardew Valley wiki — Crafting (Sprinklers): https://stardewvalleywiki.com/Sprinklers (redirects to Crafting; loaded via curl)
13. Blizzard News — StarCraft II Game Guide: Special Control: https://news.blizzard.com/en-us/article/4552955/game-guide-special-control
14. Dune RTS wiki — Radar: https://dunerts.wiki.gg/wiki/Radar
15. Wikipedia — Dune II: https://en.wikipedia.org/wiki/Dune_II

Search-snippet only (not fetched; see §14): Blizzard Simplified Controls (minimap ping), JudgeHype HotS UI article, FED2k Dune 2000 structures, Dune RTS wiki Outpost (Dune 2000), Albion wiki Territory/Hideout, SWG Restoration Structures & Cities.

## 14. Unverified list

- SC2 attacked units flash white on the minimap (HotS patch 2.0, observers) — search snippet of judgehype.com only; page not fetched.
- SC2 ally ping via Alt+G on the minimap — search snippet of Blizzard "Simplified Controls" only.
- Dune 2000 Outpost requires Barracks and gates High-Tech Factory / Starport / Ix — search snippets (FED2k, Dune RTS wiki Outpost page); the fetched Radar page confirms only the Outpost + power rule.
- Albion territory benefits (gathering bonus, watchtower) — wiki.albiononline.com refused our fetcher (403); not used beyond the official changelog.
- SWG original (2003–2011) live-game rules — only the Restoration emulator wiki was read; original rules may differ.
- Star Wars Galaxies fandom, liquipedia SC2, dune.fandom refused our fetcher (403/challenge).
- Original Transport Tycoon Deluxe (vs OpenTTD) mechanics — only OpenTTD read.
- "Free levelling anywhere" — not in repo verbatim (see §9).

## 15. Held names (need Sherif's OK)

- "radar structure" (working label for §7; final name held).
- Earth-surface Neutral base name (none proposed; Q8).
- No other new names proposed. TAUN and The Brood Spire are canon.

# Server morality — good, evil, double agents, and the heaven↔hell meter

**Contact:** [info@Rathor.ai](mailto:info@Rathor.ai)

Design tick, not a Cargo bump. Title Online stays grey. Docs only: a proposal, not a work order. Nothing here lands in the client until Core names a slice.

Read against `main` at `2334af48`. Card SERVER-MORALITY-1.

Every struct, field and system in this file is **proposed, not built**. This file names no CARD and no PATHS. It adds no People, no Place, no faction, no item, no title, no currency, and no second realm or map. Anything that needs a shared server is marked **online later** ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §2, [ONLINE_LADDER](../ONLINE_LADDER.md)).

---

## 0. Source

> "Players have real free will to be good, evil or double agents for either side. Each server can drift toward 'heaven' or 'hell' through collective play, entropy, grief and sabotage. A hell server can be recovered, and a heaven server can be ruined. Friction should teach players that wholesome, thriving servers are where they prosper most. Councils decide the details."
>
> — Sherif, 2026-10-08 2:14 AM ET, relayed by Lead Mate

A quote is a direction. This file is the councils' proposal. The details Sherif leaves to the councils are the named blanks in §9. This file fills none of them.

---

## 1. Shape

Two things are designed here.

1. **A player's path.** Good, evil, unaligned, or a double agent for either side. This rides on the marks, infamy, righteousness, and lore-faction invitations already written ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §7–§8) and on Faction Standing & Reputation ([DIPLOMACY_AND_WORLD_SIMULATION](../DIPLOMACY_AND_WORLD_SIMULATION.md) §2.2).
2. **A server's state.** One reading per server, from heaven to hell, fed by what its players do together. It is a state of the same world, proposed as one more field on `WorldSimulationState` ([DIPLOMACY_AND_WORLD_SIMULATION](../DIPLOMACY_AND_WORLD_SIMULATION.md) §2.1).

The two are linked but not the same. A player's faction is not the server's state. A Dominion member can live well on a heaven server. An Alliance member can help drag a server toward hell. The meter reads what happens to the shared world, not which banner a player carries.

Heaven is not a place you travel to. [RITE_DOS_BANK](../RITE_DOS_BANK.md) L15: "The world you build is the world you enter. The yard after tend is the heaven. Do not ship a second realm to prove it." Hell is the same yard, untended and wrecked.

---

## 2. Good, evil, and double agents

### 2.1 Open paths (already written)

Free will is already in the rules. This file keeps every line below as written.

| Path | How it is walked | Where it is written |
|---|---|---|
| **Good** | Good deeds at infamy 0 grant righteousness. Righteousness does not decay. Held at 200 for 14 real days with no Clear kill, it opens the Quellorian / Aetherion Luminari Alliance invitation | [COMBAT_AND_PVP](COMBAT_AND_PVP.md) §7.3, §8 |
| **Evil** | Killing Clear players adds infamy. Held at Outlaw (300) for 14 real days, it opens the Draek Dominion invitation | [COMBAT_AND_PVP](COMBAT_AND_PVP.md) §7.2, §8 |
| **Unaligned** | Decline both invitations, or let them lapse | [COMBAT_AND_PVP](COMBAT_AND_PVP.md) §8 |
| **Standing** | Moves only by verbs, per faction. Opens depots, blueprints, safe passage, and alliances | [DIPLOMACY_AND_WORLD_SIMULATION](../DIPLOMACY_AND_WORLD_SIMULATION.md) §2.2; [LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §3d |

No path is forced and no path is locked. The Dominion exit is a redemption quest, paid in verbs, not in Reserve ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §8). Invitations stay **online later**, as §8 already marks them.

### 2.2 Double agents (proposed, not built)

A double agent openly holds one of the two §8 memberships and covertly works for the other side. "Either side" means both directions: an Alliance member feeding the Dominion, or a Dominion member quietly working for the Alliance.

This is not a third invitation. [FACTION_LEADERS](FACTION_LEADERS.md) §11 clash 10 leaves the two doors in [COMBAT_AND_PVP](COMBAT_AND_PVP.md) §8 as the only two. A covert tie rides on those two doors.

| Proposed piece | Meaning | Status |
|---|---|---|
| `covert_allegiance` on the character record | Which side, if any, the player covertly serves. Server-side only | Proposed, not built |
| Covert offer | The other side offers covert work to a player who already holds an open §8 membership. The qualifying line is an open number (§9). Declining costs nothing | Proposed, not built |
| Covert work | The same verbs the game already has (haul, tend, escort, scout), pointed at the covert side's goals. No new verb, no new item | Proposed, not built |
| Exposure trace | Every covert act is already an event-log row ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §16; [LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §8 P2). The trace is what exposure is rolled from | Proposed, not built |

**The name stays clean.** A hidden allegiance is never shown on a player's name. There is no new name colour, no glyph, and no tell on the nameplate. The sentry mark comes from infamy alone ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §7.3, L214). The house glyph beside the mark stays the guild or party glyph ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §7.1). A covert tie is not a mark and not a crime against a player, so it adds no infamy by itself.

**Cover gives no immunity.** A double agent who kills a Clear player earns infamy like anyone else (§7.2). One who wrecks another House's work outside war rules takes the standing loss, censure, and event-log entry in [LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §3g. The same act also moves the server meter (§3.2). Covert work done through tending, hauling, and scouting moves the meter the way those verbs always do.

**Risk of exposure.** Each covert act carries a chance of exposure, drawn from its trace. The chance per act, and how long a trace stays live, are open numbers (§9). A player may also unmask on their own and declare for the covert side. That ends the cover with the same consequences as exposure, and it is the honest way out.

**Consequences of exposure.**

- The open membership ends. For the Alliance this is expulsion, the same end §8 writes for a second Clear kill inside a censure week. For the Dominion the invitation lapses. The player returns to unaligned access: domes by the mark rules, motherships closed until they qualify again ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §8).
- Standing with the betrayed side falls ([DIPLOMACY_AND_WORLD_SIMULATION](../DIPLOMACY_AND_WORLD_SIMULATION.md) §2.2; [LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §3d: allegiance can change, with a cost). The size of the fall is an open number (§9).
- The covert side may then offer open membership, if the player meets the §8 line for that side. Otherwise the player stays unaligned.
- Infamy and righteousness are untouched by exposure itself. The mark on the name does not change.
- The exposure is told to the exposed player and written into the betrayed faction's own records. It is never a server-wide broadcast, never on the nameplate, never on the server meter or the server list, and never outside the game.
- Redemption stays open. The paths in §6 apply to a burned double agent like anyone else.

A personal covert tie never flips a guild. Guild allegiance stays the guild-level choice in [LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §3d.

**Offline.** There is no other player to deceive. Double agents are **online later** and HOLD until "online yes" and named PATHS.

---

## 3. The server heaven↔hell meter (proposed, not built)

### 3.1 Where it lives

One proposed field on `WorldSimulationState` ([DIPLOMACY_AND_WORLD_SIMULATION](../DIPLOMACY_AND_WORLD_SIMULATION.md) §2.1). Every existing field there stays as written.

| Proposed field | Type sketch | Meaning | Status |
|---|---|---|---|
| `server_morality` | `ServerMoralityState` | The server's heaven↔hell reading | Proposed, not built |
| `ServerMoralityState.meter` | number | Where the server sits between the heaven end and the hell end. The range is an open number (§9) | Proposed, not built |
| `ServerMoralityState.trend` | number | Recent drift, smoothed, so a recovering or sliding server is visible | Proposed, not built |
| `ServerMoralityState.band` | derived | The band the meter falls in. Derived from `meter`, never set by hand. Band count and edges are open numbers (§9) | Proposed, not built |

It is a state of the same world. It is not a second realm, not a second map, and not a separate shard ([RITE_DOS_BANK](../RITE_DOS_BANK.md) L15). The Places stay the Places. The hexes stay the hexes.

It is not a layer score. [LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §3e refuses a layer with its own currency, score, or world. The meter pays no one directly. Its effects run through paths that already exist: RBE regeneration ([DIPLOMACY_AND_WORLD_SIMULATION](../DIPLOMACY_AND_WORLD_SIMULATION.md) §4) and reliability ([LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §4.5).

It sits beside `total_harmony` and `total_corruption`. Those stay the lore-scale totals the Crownstone paths move ([DIPLOMACY_AND_WORLD_SIMULATION](../DIPLOMACY_AND_WORLD_SIMULATION.md) §3). This file cites those paths and does not extend them. The meter neither reads nor writes Crownstone state.

### 3.2 What moves it

The meter reads the sum of what the server's players do, in aggregate. Each input's weight is an open number (§9).

| Input | Direction | Read from | Where it is written |
|---|---|---|---|
| **Tending** | Toward heaven | Nodes brought from Stressed to Idle; ruins mended; support and bolster of another House's work; good deeds | [LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §3g, §4.5; [COMBAT_AND_PVP](COMBAT_AND_PVP.md) §7.2 |
| **Collective play** | Toward heaven when healthy, toward hell when failing | The server indicators already proposed: climate health, restore ratio, needs-met ratio, holdings spread, commons stock trend | [LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §4.2–§4.4 |
| **Entropy** | Toward hell, always, slowly | Nodes tire, reliability decays, upkeep charges, perishables spoil. The bill comes whether anyone plays or not | [LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §4.5; [RITE_DOS_BANK](../RITE_DOS_BANK.md) DOS catalogue 3 |
| **Grief** | Toward hell | Clear kills, weighted harder on low-proficiency victims as infamy already is; repeated attacks on the same victim; conduct reports upheld after review | [COMBAT_AND_PVP](COMBAT_AND_PVP.md) §7.2, §16; [LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §3g |
| **Sabotage** | Toward hell | Malicious destruction outside war rules; a commons stripped; corridors hoarded to starve the rest | [LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §3g, §4.2 |

What does **not** move it:

- Which faction a player belongs to, open or covert.
- Kills inside consented frames: duels, battlegrounds, declared war instances ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §7.1). Legal Apprehend hits on Aggressor, Marked, or Outlaw players ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §6).
- Crownstone state ([DIPLOMACY_AND_WORLD_SIMULATION](../DIPLOMACY_AND_WORLD_SIMULATION.md) §3).
- Head count. A small server that tends well reads as well as a large one.
- Any purchase. Nothing about the meter is for sale (§4).

### 3.3 How each state feels

The rules stay the same at every reading. What changes is how generous the world is, how it looks, and what happens in it. Friction teaches the lesson. The game does not lecture.

| | Heaven end | Middle | Hell end |
|---|---|---|---|
| **Economy** | Regeneration runs high through the harmony path ([DIPLOMACY_AND_WORLD_SIMULATION](../DIPLOMACY_AND_WORLD_SIMULATION.md) §4). Reliability ceilings hold ([LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §4.5). Abundance nodes find footing ([DIPLOMACY_AND_WORLD_SIMULATION](../DIPLOMACY_AND_WORLD_SIMULATION.md) §4) | Base rates | Regeneration runs low. Stressed spreads faster. Starvation zones ([DIPLOMACY_AND_WORLD_SIMULATION](../DIPLOMACY_AND_WORLD_SIMULATION.md) §4) appear. Upkeep bites harder against thinner yields |
| **Safety** | Player-damage rules unchanged. In practice: more players tending and escorting, corridors kept, fewer Marked and Outlaw players about | Unchanged | Player-damage rules unchanged. Domes, the Newcomer Aegis, homestead cores, and stances hold exactly as written (§4). In practice: fewer helpers, more marks in the wilds, more bounties open |
| **Visuals** | Tended yards, clear wells, light, kept banners | The world as dressed today | Haze and ash, Stressed wells, ruins left standing. A Stressed well stays a well ([RITE_DOS_BANK](../RITE_DOS_BANK.md) Reel 2). Dress changes, geometry does not. Reduced-motion play gets the same dress without moving effects |
| **Events** | Redemption Wave chain reactions ([DIPLOMACY_AND_WORLD_SIMULATION](../DIPLOMACY_AND_WORLD_SIMULATION.md) §5) and abundance nodes find more footing | As scheduled | Scarcity in the starvation zones. The Mirror Reckoning echo in [DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §9 is cited here only, with damage unchanged (§7) |

The meter never touches ledger balances. It never deletes an item, never fines a player, and never wipes anything ([LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §3g, §4.2: nothing is wiped).

### 3.4 The ways out of hell

A hell server can be recovered. There is no floor it cannot climb from.

- **Tend.** Every Stressed node brought back to Idle, every ruin mended with Mend, every corridor kept moves the meter up. Ruins keep salvage so rebuilding is real work, not a restart ([LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §3g).
- **Help.** Good deeds for low-proficiency players ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §7.2) count toward the meter as well as toward the player's own infamy or righteousness.
- **Change the rules together.** An economy policy ballot, such as opening a commons granary, is already a server lever ([LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §7.5). Its effect on the indicators feeds the meter.
- **Redeem.** Players leaving the criminal path through the exit quest ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §8) spend 28 real days on restore-and-help verbs. That work lifts the server too.
- **Show the climb.** A rising trend is visible on the server list (§5), so players who want to help heal a server can find it.

There is no admin wash, no paid reset, and no server wipe as the remedy. Whether the climb out runs slower than the slide in is an open number (§9).

### 3.5 How heaven decays

A heaven server can be ruined.

- **Entropy never stops.** Nodes tire and reliability decays every week ([LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §4.5). A server that stops tending drifts toward hell with no villain needed. The week was the bill.
- **Sabotage and grief still land.** Free will includes ruining things. The bounds are the existing ones: damage caps per target per day, escalating consequences for repeat attacks, ruins not deletion, review and rollback ([LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §3g).
- **Concentration.** A heaven server where one alliance quietly hoards the corridors drifts down through the collective-play indicators ([LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §4.2: hoard and cartel).

Heaven has no lock. It is held by upkeep or it is lost.

### 3.6 Offline

Offline there is one world and one player. If a later card builds the meter offline, it reads only tending and entropy on the local world. There is no one to grief or sabotage. Offline harm to another's work stays the local `world_harm_mode` rule, which is off by default ([LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §3g). Hour 1 stays the hour. The meter is not on the Hour 1 door.

---

## 4. Guardrails

These hold at every reading of the meter and on every path in §2.

- **Shields hold.** The Newcomer Aegis and faction Sanctuary Domes still block all damage between players ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §5.1, §5.2). Homestead cores and the Depths landing stay Peace ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §4). A hell server does not thin a dome, shorten the Aegis, or raise dome upkeep until it fails. Holiday ceasefires still stand player damage down ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §4).
- **PvE is flat.** Nothing here changes monster, wildlife, droid, alien, or boss damage. That damage lands everywhere, under every shield, at every reading, as [COMBAT_AND_PVP](COMBAT_AND_PVP.md) §5.2 already says. Enemy toughness belongs to the Place ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §3).
- **Nothing is for sale.** No purchase moves the meter, washes it, hides an allegiance, buys exposure protection, or buys a way around any rule here. Cosmetics never change stats ([FACTION_HERALDRY](../FACTION_HERALDRY.md)). See the refuse table (§8).
- **Grief does not travel.** A player's contribution to one server's meter stays on that server. There is no cross-server hunt list, no carried grudge target, and no meter debt that follows a character. Server histories never merge ([LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §7.6). Infamy, righteousness, and allegiance still carry on a server move exactly as [COMBAT_AND_PVP](COMBAT_AND_PVP.md) §8 writes. That is the player's own record, not griefing.
- **Grief does not leave the game.** No meter reading, exposure, or covert record is ever tied to a real identity, posted outside the game, or used for anything outside the game.
- **No player is named or ranked.** The meter is an aggregate. It never names, lists, or ranks individual players publicly. There is no "top tender" board and no "worst griefer" board. The event log keeps acts for review, as it already does ([LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §8 P2), and review is not public.
- **No new UI anchor.** Any reading joins existing surfaces ([design/UI_LAYOUT_SYSTEM](UI_LAYOUT_SYSTEM.md) §1.6; [COMBAT_AND_PVP](COMBAT_AND_PVP.md) §16).

---

## 5. Showing server state

Players see a server's state before they choose it, so they can pick a wholesome server.

| Surface | What it shows | Status |
|---|---|---|
| **Server list** | The band, the trend (rising, steady, falling), and the server's `world_harm_mode` ([LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §3g). No player names | **Online later.** Online is grey ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §2) |
| **Server ledger face** | The band and trend beside the indicators already proposed there ([LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §4.4) | **Online later** |
| **Server history** | A line when the band changes, with the date and no names ([LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §7.6) | **Online later** |
| **The world itself** | The dress in §3.3. A player reads the server by walking it | Proposed, not built |

Offline there is no list and no choice of server. If a later card shows the local reading, Core names the surface. This file adds none.

---

## 6. Redemption

The model redemption arc is the Unbound's turn from Drainer to freed.

- **The fall.** The Draeks were a peaceful, cooperative people who chose survival at any cost, then theft, then domination ([DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §1, §2, §4, §5). Under the Crownstone, they drain other Peoples. That fall is why redemption has a past to face ([FACTION_LEADERS](FACTION_LEADERS.md) §8).
- **The turn.** The Unbound is the first Draek to break the Crownstone's hold and stay outside it, and others followed ([FACTION_LEADERS](FACTION_LEADERS.md) §3, §8). No one bought that freedom. It was walked.
- **The proof.** Other Peoples distrust the Unbound and those who followed. They earn a place through Faction Standing & Reputation, by verbs, with nothing for sale ([FACTION_LEADERS](FACTION_LEADERS.md) §3; [DIPLOMACY_AND_WORLD_SIMULATION](../DIPLOMACY_AND_WORLD_SIMULATION.md) §2.2). The cities withhold until standing is real ([FACTION_LEADERS](FACTION_LEADERS.md) §8).

The same shape holds at every scale in this file.

| Scale | Fall | Turn | Proof |
|---|---|---|---|
| **A People** | The Draek fall | The Unbound breaks the Crownstone's hold | Standing earned by verbs |
| **A player** | Infamy, the Dominion, or a burned double agent | Decline, lapse, unmask, or start the exit quest ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §8) | Restore-and-help verbs under the existing redemption framework ([REDEMPTION_MECHANICS_PER_SPECIES](../REDEMPTION_MECHANICS_PER_SPECIES.md); [DISCORDANT_REDEMPTION_QUESTLINES](../DISCORDANT_REDEMPTION_QUESTLINES.md); [SPECIFIC_REDEMPTION_QUESTS](../SPECIFIC_REDEMPTION_QUESTS.md)) |
| **A server** | Entropy, grief, and sabotage toward hell | Players start tending together | The meter climbs (§3.4) |

No scale has a skip. No scale has a permanent brand ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §18 retires the criminal faction as a door that can never open again, and the redemption exit replaces it).

---

## 7. Where this meets existing text

These files were not edited.

1. [DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md) §9 and §10 say extractive, low-mercy servers manifest stronger Mirrors. §4 here keeps all PvE damage flat. A later card should say how a Mirror on a hell-leaning server differs without hitting harder. Until then, damage stays as the Place sets it.
2. [COMBAT_AND_PVP](COMBAT_AND_PVP.md) §8 carries infamy, righteousness, and allegiance on a server move. §4 here keeps that and refuses only that grief, meter state, and grudges travel. A covert allegiance carries the same way, still hidden.
3. [FACTION_LEADERS](FACTION_LEADERS.md) §11 clash 10 asks how Unbound standing sits beside the two §8 doors. This file does not answer it and adds no third door. Double agents ride on the existing two.
4. [DIPLOMACY_AND_WORLD_SIMULATION](../DIPLOMACY_AND_WORLD_SIMULATION.md) §2.2 marks Draek standing as an inverted scale. The meter is not a faction scale. A server can be heaven while the Dominion is strong on it.

---

## 8. Refuse

| Rejected row | Why |
|---|---|
| A heaven realm or hell realm as a second map, shard, or instance | [RITE_DOS_BANK](../RITE_DOS_BANK.md) L15. The world you build is the world you enter |
| Real-money purchase that moves the meter, washes a server, or skips the climb out of hell | Pay-to-win. Recovery is walked |
| Real-money purchase that hides an allegiance or protects against exposure | Pay-to-win. Cover is risk, not a product |
| A pay-to-win recovery kit, server reset, or paid admin wash | Pay-to-win, and nothing is wiped ([LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §4.2) |
| Any real-money effect at all from the meter or from a player path | Nothing here is for sale |
| A new name colour, glyph, or nameplate tell for allegiance | The sentry mark comes from infamy alone ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §7.3) |
| Showing a hidden allegiance on a name, even after exposure | A name carries the mark and the house glyph only |
| A third lore-faction invitation for double agents | Two doors only ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §8; [FACTION_LEADERS](FACTION_LEADERS.md) §11 clash 10) |
| Hell scaling monster, wildlife, droid, alien, or boss damage | PvE damage is the Place's, and lands the same everywhere (§4) |
| Hell thinning domes, shortening the Aegis, or lifting a ceasefire | Shields hold at every reading ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §5) |
| Heaven granting a stat, health, or damage bonus | One power ladder ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §1) |
| A public board that names or ranks players by meter contribution | The meter is an aggregate. No player is named or ranked |
| Broadcasting an exposure server-wide or outside the game | Exposure stays with the player and the betrayed faction's records |
| Grief, grudges, or meter debt that follow a character to another server | Server histories never merge ([LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §7.6) |
| Anything tying meter, exposure, or covert records to a real identity | Grief does not leave the game |
| Reading a faction as the meter, so that Draek or Dominion means hell | The meter reads acts on the shared world, not banners |
| Counting consented-frame kills as grief | Duels, battlegrounds, and declared wars are consented ([COMBAT_AND_PVP](COMBAT_AND_PVP.md) §7.1) |
| A permanent hell with no way out, or a permanent heaven with no upkeep | Sherif's ask: hell can be recovered, heaven can be ruined |
| Wiping or deleting anything as the meter falls | Ruins, not deletion ([LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md) §3g) |
| A popup lecture about morality | Friction teaches. The world shows it |
| Extending Crownstone, treaty, or war-week rules to feed the meter | Cited only. Those rules stay as written |
| A new HUD anchor or panel for the meter | [design/UI_LAYOUT_SYSTEM](UI_LAYOUT_SYSTEM.md) §1.6; [COMBAT_AND_PVP](COMBAT_AND_PVP.md) §16 |
| Lighting Title Online, adding a listen or a public bind, or a live server list now | Online is grey ([ONLINE_LADDER](../ONLINE_LADDER.md)) |
| New People, Place, faction, item, or title names | Out of bounds for this card |
| Invented CARD names or PATHS | Core names those |

---

## 9. Open numbers

Every value is a named blank for the councils. None is set here.

| Tunable | What it sets | Value |
|---|---|---|
| `meter_range` | The span from the heaven end to the hell end | Blank (councils) |
| `band_count` | How many bands the meter splits into | Blank (councils) |
| `band_edges` | Where each band starts and ends | Blank (councils) |
| `weight_tending` | How much tending moves the meter | Blank (councils) |
| `weight_collective` | How much the §4.4 indicators move the meter | Blank (councils) |
| `weight_entropy` | How fast an untended server drifts toward hell | Blank (councils) |
| `weight_grief` | How much grief moves the meter | Blank (councils) |
| `weight_sabotage` | How much sabotage moves the meter | Blank (councils) |
| `climb_vs_slide` | Whether recovery runs slower, the same, or faster than decline | Blank (councils) |
| `trend_window` | The smoothing window behind the trend arrow | Blank (councils) |
| `regen_by_band` | How far each band moves RBE regeneration | Blank (councils) |
| `stress_spread_by_band` | How fast Stressed spreads in each band | Blank (councils) |
| `dress_by_band` | Which band edges change the world's dress | Blank (councils) |
| `covert_offer_line` | What a player must hold before the other side offers covert work | Blank (councils) |
| `exposure_chance` | Chance of exposure per covert act | Blank (councils) |
| `trace_lifetime` | How long a covert act's trace can still expose | Blank (councils) |
| `exposure_standing_loss` | Standing lost with the betrayed side on exposure | Blank (councils) |
| `covert_reoffer_wait` | How long before a burned agent can be offered covert work again | Blank (councils) |
| `list_refresh` | How often the server list refreshes a server's reading | Blank (councils) |

---

## 10. Cite

- [COMBAT_AND_PVP](COMBAT_AND_PVP.md): §1, §2, §3, §4, §5.1, §5.2, §6, §7.1, §7.2, §7.3 (L214), §8, §16, §18
- [DIPLOMACY_AND_WORLD_SIMULATION](../DIPLOMACY_AND_WORLD_SIMULATION.md): §2.1, §2.2, §3, §4, §5
- [RITE_DOS_BANK](../RITE_DOS_BANK.md): L15 (Reel 1), Reel 2, DOS catalogue 3
- [FACTION_LEADERS](FACTION_LEADERS.md): §3, §8, §11 clash 10
- [DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL](../DRAEK_ORIGIN_AND_THE_GREAT_BETRAYAL.md): §1, §2, §4, §5, §9, §10
- [LAYERED_GAMES_AND_SERVER](../LAYERED_GAMES_AND_SERVER.md): §3d, §3e, §3g, §4.2, §4.3, §4.4, §4.5, §7.5, §7.6, §8 P2
- [design/UI_LAYOUT_SYSTEM](UI_LAYOUT_SYSTEM.md): §1.6
- [FACTION_HERALDRY](../FACTION_HERALDRY.md)
- [ONLINE_LADDER](../ONLINE_LADDER.md)
- [REDEMPTION_MECHANICS_PER_SPECIES](../REDEMPTION_MECHANICS_PER_SPECIES.md)
- [DISCORDANT_REDEMPTION_QUESTLINES](../DISCORDANT_REDEMPTION_QUESTLINES.md)
- [SPECIFIC_REDEMPTION_QUESTS](../SPECIFIC_REDEMPTION_QUESTS.md)

Title Online stays grey. Default play stays `cargo run -p powrush-client`. This file adds no listen and no public bind.

Draek is the only spelling of that People used here.

# Design pillars

DESIGN-PILLARS-1. Docs only. This file is new. Older files stay as they are; this page points at them.

The PATSAGi Councils adopted this page under Sherif's typed delegation t1767u, t1948u and t2247u. Sherif may veto or amend. Every row in the table below is a PROPOSAL until Sherif's veto window passes.

The build this page describes is the offline EA. Title Online stays grey, and server work stays planned ([shared/net_mode.rs:5](../shared/net_mode.rs#L5), [:18](../shared/net_mode.rs#L18)).

## The weight

Core verified these words in comment 7117081816. They are the only quotation of Sherif in this doc:

> Consult Ra-Thor and the PATSAGi regarding the game design documents and all the rest. Also the 5 levels were reskins instead of traveled places. Game should mostly be Conquer Online × World of Warcraft with the properly layered Transport Tycoon Deluxe.

The mechanic lists under that quotation are the councils' reading, from comments 7117081816, 7117183388 and 7117212330. Comment 7117051674 named this card.

Conquer Online Classic and World of Warcraft: The Burning Crusade are the weighted core. Transport Tycoon Deluxe is a layer on that core, never the core.

Older maps this page builds on:

- Conquer Online classic: [GDD_IMMERSION_REVISION.md:48](GDD_IMMERSION_REVISION.md#L48)
- World of Warcraft, Vanilla and The Burning Crusade: [GDD_IMMERSION_REVISION.md:73](GDD_IMMERSION_REVISION.md#L73)
- Transport Tycoon Deluxe: [GDD_IMMERSION_REVISION.md:90](GDD_IMMERSION_REVISION.md#L90)

Transport Tycoon Deluxe is spelled the same way at [CREDIT_RESERVE_LOGISTICS.md:19](CREDIT_RESERVE_LOGISTICS.md#L19).

Inspiration only: no copied names, art or code, and no claim that this game rivals or beats those games. Borrow technique only, and Powrush words for players, are the combat pillar at [COMBAT_AND_PVP.md:30](design/COMBAT_AND_PVP.md#L30). Names of other games here are an English-only repo screen, not trademark clearance ([LEVELS_CANON.md:65](design/LEVELS_CANON.md#L65)).

## Where the councils' reading sits

| Weight | Councils' reading | Home |
|---|---|---|
| Core | Fast click combat | A strike of 8 from left click, right trigger, or key 1 ([combat_core.rs:49](../client/src/combat_core.rs#L49), [:262](../client/src/combat_core.rs#L262)), with a 0.35 s tell ([:107](../client/src/combat_core.rs#L107)). Offline PvE ([:3](../client/src/combat_core.rs#L3)). |
| Core | PK and loot | Loot is named finds on defeat ([loot_drops.rs:1](../client/src/loot_drops.rs#L1)). PK is online-only, planned ([COMBAT_AND_PVP.md:44](design/COMBAT_AND_PVP.md#L44)). |
| Core | XP grind feel | Per-skill XP and levels ([skill_xp.rs:1](../client/src/skill_xp.rs#L1)). Combat XP is from landed hits ([:27](../client/src/skill_xp.rs#L27)). A kill pays no XP ([LEVELS_CANON.md:15](design/LEVELS_CANON.md#L15)). |
| Core | City hub | Planned. Sanctuary today is a yard ([PLACES_BIBLE.md:73](PLACES_BIBLE.md#L73)). |
| Core | Zones | Three PlaceIds today ([hex_travel.rs:52](../shared/hex_travel.rs#L52)). Four named disk rooms ([client hex_travel.rs:4](../client/src/hex_travel.rs#L4)). Distinct traveled places are planned, under later cards WORLD-BIBLE-1 and WORLD-TRAVEL-1. This page names no new place. |
| Core | Quests | Planned. The in-build quest sentence is [wellsworn.rs:21](../client/src/wellsworn.rs#L21). The prod client has no quest log ([LOOP_MAP.md:155](LOOP_MAP.md#L155)). |
| Core | Classes and talents | Six classes, chosen in play after House, plus Confluence ([LEVELS_CANON.md:66](design/LEVELS_CANON.md#L66), [:67](design/LEVELS_CANON.md#L67)). A talent tree is named at [GDD_IMMERSION_REVISION.md:62](GDD_IMMERSION_REVISION.md#L62). A class select in the running game is planned. |
| Core | Dungeons | Planned ([PROCGEN_DESIGN.md:83](PROCGEN_DESIGN.md#L83)). The offline teacher stays [FUN_WITHOUT_WOW.md:1](FUN_WITHOUT_WOW.md#L1). |
| Core | Quest log | Planned. The old refusal is in the table below. Tiers are planned for their own later CUT. |
| Layer | Routes, logistics, a growing economy between places | The chain is mapped at [GDD_IMMERSION_REVISION.md:94](GDD_IMMERSION_REVISION.md#L94). Week score is tons plus restored, local only ([week_audit.rs:3](../shared/week_audit.rs#L3), [:44](../shared/week_audit.rs#L44)). Manufacture leg: [fabricator.rs:1](../shared/fabricator.rs#L1). Player-built routes are planned in COMPANY-1. The EA minimum is one refine leg and one haul leg; carts and vehicles come after that minimum ([EA_BAR.md:47](EA_BAR.md#L47)). |

## COMPANY-1

Councils' reading from comment 7117183388. This is a layer on the Conquer Online and World of Warcraft core, never the core. The code is planned, through later LOOP-CLOSE and UNLOCK-LADDER cards. Account work stays reserved.

**Running a company.** Planned: founding one, its members and its roles. The councils' name for a player company is a guild company.

**Routes and income.** Planned: building routes and vehicles, carrying cargo and passengers between settlements, and earning income from deliveries against upkeep. That income does not lift the money refusal in the table below. Credit, never gold, stays the currency sentence at [GDD_IMMERSION_REVISION.md:42](GDD_IMMERSION_REVISION.md#L42).

**Rivalry and growth.** Planned: competition and cooperation over routes. Settlements grow when they are served. A tie to valence and thriving scores is planned. Valence is named at [LOOP_MAP.md:125](LOOP_MAP.md#L125). A settlement thriving score is planned.

**Shared routes.** Planned, while online stays grey: shared or contested routes, cargo that is safe inside safe zones and at PK risk outside them (PK stays online-only, planned), and corrupt or double-agent sabotage that is possible and has consequences.

## NPC rival companies

Councils' reading from comment 7117212330.

They are optional. In the offline EA they are on by default, so a solo player has competition. A per-world setting turns them off, or sets low, medium or high.

They build inside budgets a later WORLD-DIRECTOR card sets (planned). They never block or snipe an active player route. They yield or sell out to player companies. Yielding does not open a gold market; the money row below covers that.

Player-driven companies stay the vision the councils recorded. Server work stays planned, and online stays grey.

## BOT-SOAK

Councils' reading from comment 7117212330. The run is the planned card BOT-SOAK-1. That card is cut after this section is on main and the systems it drives exist. It sits ahead of the planned card RC-PLAYTEST-1.

Planned sessions: NPC companies and scripted NPC players play long, seeded, headless sessions. They build routes, trade, fight, save and quit. They report crashes, stuck pathing, an economy that runs away or deflates, and performance numbers.

The sessions feed the 60-minute crash-free gate named for the planned RC-PLAYTEST-1 card. BOT-SOAK is not a required check, and it is not a replacement for Sherif's real-PC test. The run is planned as a scheduled or manual workflow. The seven required checks stay as they are.

Server work stays planned, and online stays grey.

## Supersede or still binds

Each row is a PROPOSAL until Sherif's veto window passes. Realm ids stay for their own later CUT, as do quest-log tiers.

| Old line | Proposal | Rests on |
|---|---|---|
| [GDD_IMMERSION_REVISION.md:80](GDD_IMMERSION_REVISION.md#L80) no quest log | PROPOSAL: supersede those words. The one card, one sentence, hideable by H, stays until a later CUT names the tiers. | 7117081816 |
| [GDD_IMMERSION_REVISION.md:83](GDD_IMMERSION_REVISION.md#L83) race or class lobby at select | PROPOSAL: still binds as a Title-door refusal. Classes and talents stay in-play growth after House. | 7117081816 |
| [GDD_IMMERSION_REVISION.md:85](GDD_IMMERSION_REVISION.md#L85) talent panel | PROPOSAL: still binds as a panel refusal, with no second HUD at [:86](GDD_IMMERSION_REVISION.md#L86). Talents in play stay the classes row above. | 7117081816 |
| [GDD_IMMERSION_REVISION.md:96](GDD_IMMERSION_REVISION.md#L96) no combat | PROPOSAL: supersede that keep. Fast click combat is in the weighted core. | 7117051674, 7117081816 |
| [GDD_IMMERSION_REVISION.md:88](GDD_IMMERSION_REVISION.md#L88) combat never scores the week | PROPOSAL: still binds. Week score stays tons plus restored. | 7117081816 |
| [GDD_IMMERSION_REVISION.md:98](GDD_IMMERSION_REVISION.md#L98) stays four Places | PROPOSAL: supersede the cap. The councils' reading is that the reskinned levels become distinct, connected places the player travels between. This page names no new place. | 7117081816 |
| [OFFLINE_SKU.md:50](OFFLINE_SKU.md#L50) no fifth place | PROPOSAL: supersede the no-fifth-place clause, on the same reading as :98. The other sentences on that line stay. | 7117081816 |
| [COMBAT_AND_PVP.md:81](design/COMBAT_AND_PVP.md#L81) combat adds no fifth Place | PROPOSAL: supersede the cap. Combat is not what names a place. | 7117081816 |
| [GDD_IMMERSION_REVISION.md:58](GDD_IMMERSION_REVISION.md#L58) Battle Power | PROPOSAL: still binds. | 7117081816 |
| [GDD_IMMERSION_REVISION.md:59](GDD_IMMERSION_REVISION.md#L59) cash-shop power | PROPOSAL: still binds. | 7117081816 |
| [GDD_IMMERSION_REVISION.md:60](GDD_IMMERSION_REVISION.md#L60) XP from kills | PROPOSAL: still binds. The grind above is per-skill XP, not kill XP. | 7117081816 |
| [GDD_IMMERSION_REVISION.md:82](GDD_IMMERSION_REVISION.md#L82) gold auction house | PROPOSAL: still binds. | 7117081816 |
| [GDD_IMMERSION_REVISION.md:100](GDD_IMMERSION_REVISION.md#L100) money as the verb | PROPOSAL: still binds. Company income and an NPC company yielding do not lift it. | 7117081816 |

The same no-fifth-place refusal is restated at [EA_BAR.md:71](EA_BAR.md#L71) and [CREDIT_RESERVE_LOGISTICS.md:9](CREDIT_RESERVE_LOGISTICS.md#L9). The :98 proposal covers those lines too.

Later docs-lane cards, each its own CUT: SYSTEMS-SPINE-1 (#1162), WORLD-BIBLE-1 (#1171), WORLD-GEN-1 (#1173), LOOK-TARGET-1 (#1175), GDD-INDEX-1 (#1177). All five are on main, docs only. This page invents none.

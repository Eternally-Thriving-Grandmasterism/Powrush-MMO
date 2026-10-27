# POWRUSH LOOP NOTES — loop depth ideas (POWRUSH-LOOP-NOTES-1)

**Design note only — no code; nothing here is built.**

**Base** `e387644b` (`e387644bb8b1cde6355836aa9e0a54ff4b07f9bc`). Docs only, one file. Online stays grey. Every idea below is optional and needs its own Core card before any code.

---

## 0. Sherif's principle (verbatim)

> "Keep in mind, war and PvP are real parts of the game. Double agents can be evil or good. Powrush-MMO is a universe for humans to practice base reality without encouraging harm, even if it's part of the game."

Nothing in these notes removes war, PvP or double agents. They add depth beside them.

---

## 1. Reframes (optional ideas)

Canon for war, PvP and moral standing stays where it is. These notes link to it rather than restating it:

- [`docs/design/COMBAT_AND_PVP.md`](design/COMBAT_AND_PVP.md)
- [`docs/design/SERVER_MORALITY.md`](design/SERVER_MORALITY.md)

Four ideas, none built:

1. **House-scale restoration event.** A whole House works one tired region back to thriving together. The score is the same as it is today: tons plus restored (see `shared/war_week.rs`, `shared/week_audit.rs`).
2. **Mastery shown in the world.** Today mastery is only `hands_memory` (`client/src/hands_memory.rs`, `HANDS_CAP = 8` tends per realm) plus the `{place} · the hands remember` journey-feed line. There is no yearbook code. The only yearbook mention is [`docs/JOY_WITHOUT_MALL.md`](JOY_WITHOUT_MALL.md) L18 ("Show the find on House / yearbook. Never trade."). The idea is to let mastery show as marks in the world (worn paths, tended wells) rather than as a UI number.
3. **Dailies become a dismissable offer card.** There is no daily-quest system today. If a daily-style prompt is ever proposed, it shows as one card the player can dismiss, with no streak, no timer and no loss for skipping it.
4. **Storm boss (optional idea).** Players could call a storm and face it together as a shared pressure on the land. It would need its own card and must follow §4 if it involves any fighting.

---

## 2. "Surge" side mode (working name)

Short, offline, client-only. Nothing goes over a network.

- **Waves are pressures on the land:** blight, seam strain, tremors, water surges and heat surges.
- **Players shore, drain and restore** to answer each wave.
- **Failure is a tired climate, never death.** A lost wave leaves the hex tired. It does not kill anyone.

**Existing rules it could reuse** (all on main today):

| Rule | Where |
|------|-------|
| `ClimateNode` fatigue and its five `NodeState`s (Idle, Glowing, Tended, Resting, Stressed), `tend`, `tick`, `restore_from_flow` | `shared/climate_node.rs` |
| Hex tired signal (`ShardClimate.stress`) and thriving signal (`harmony`) | `shared/shard_climate.rs` |
| Temper `ToolTier` ladder (Hands → TendHook → LaneCrateMk1 → ClimatePick → MendSpindle → HarmonyLoom) | `shared/temper.rs` |
| `WardKind` wards (Shell, Current, Reserve, Dawn, Book) | `shared/temper.rs` |
| Score = tons plus restored | `shared/war_week.rs`, `shared/week_audit.rs` |

"Careful drilling" in Surge would lean on fatigue: drilling too hard tires the node, the same way an over-tend does today.

**Net-new (honest gaps):** there is no drill, ore or mining system on main. "ore" shows up only as a resource-type string in a few older tests and setup files (for example `server/tests/persistence_tests.rs`, `tests/harvest_flow_test.rs`); no mining loop exists. There is also no placed tool that acts on its own each tick. Drills, seams, ore yields and placed tools would all be new work.

---

## 3. Horde-fighting option beside mining (design note only)

Combat stays parked. It is lifted only by Sherif himself, through a Steward ticket plus its own Core card (see the parked-combat row in [`docs/design/COMBAT_AND_PVP.md`](design/COMBAT_AND_PVP.md) and [`docs/MERCY_TEMPER_PROGRESSION.md`](MERCY_TEMPER_PROGRESSION.md)).

No combat code exists today. `client/src/war_week.rs` says "No combat key", and `shared/skirmish_well.rs` is an E contest, not a fight.

If combat is ever lifted, Core's rulings for a Surge horde option are:

- **Clearing a horde never counts as restored.** Restored stays a land verb.
- **A combat run pays no more than a mining run** of equal difficulty and time.
- **Both share one surge cap.** Fighting does not open a second reward lane.

**Sherif's single-player horde picture (CYDRUID-DRAEK-LORE-1).** Sherif (https://x.com/AlphaProMega/status/1998596460703780956):

> Imagine being a lone Cydruid or a lone Human fighting against hordes of Draeks and their strange, enslaved, alien minions in the single-player experience with Draeks Mindlords eventually coming out to battle after losing all their mind-controlled minions!

**Draek Mindlords (CYDRUID-DRAEK-LORE-1).** A boss tier that comes out to battle only after a Draek force has lost all its mind-controlled minions. Name and trigger are Sherif's; nothing else about Mindlords is set.

**Crownstone ruling (Sherif, 2026-10-27 12:35 AM ET, verbatim):** "There's 1 Crownstone which is what enables the ultimate controller of the hive mind and all below." Recorded only as said: exactly one Crownstone, and it enables the ultimate hive-mind controller over all below.

**Mindlord placement: decided by the councils (DRAEK-HIERARCHY), council decision under Sherif's 12:36 AM rule.** The record (P1-P4) is in [`ENSLAVED_MINION_SPECIES.md` §3.1](ENSLAVED_MINION_SPECIES.md#31-draek-mindlords-boss-tier-after-the-minions-fall). Mindlord rank versus the Hivelord's chosen Brood Lords stays open there.
Hierarchy design rule: see [`ENSLAVED_MINION_SPECIES.md` §3.1](ENSLAVED_MINION_SPECIES.md#31-draek-mindlords-boss-tier-after-the-minions-fall).
Mindlord detail lives in [`ENSLAVED_MINION_SPECIES.md` §3.1](ENSLAVED_MINION_SPECIES.md#31-draek-mindlords-boss-tier-after-the-minions-fall). Combat stays parked as above; this is a design note only.

---

## 4. Gear and build depth

Only within the existing rarity rules in [`docs/JOY_WITHOUT_MALL.md`](JOY_WITHOUT_MALL.md) ("Rarity (keep)" and "Refuse rarity poison"):

- Two numbers: Temper `+N` × Lumen seats × Ward kind.
- Trophy is not the strongest item. Shell stays mitigation.
- Lineage is the hook's `+N` plus first-well-restored.
- Show the find; never trade it.

Build depth comes from choosing tool tier, temper, Lumen seats and ward kind for the job, not from rarer drops. The older `src/inventory.rs` has a random-roll `Rarity` enum; these notes do not build on that roll.

---

## 5. Mercy-Checks guardrails

Any card that grows out of these notes must keep all of these:

- No loot boxes.
- No paid randomness.
- No pay-to-win.
- No FOMO timers and no streak punishment.
- Rewards are earned by play.

---

Online grey · no OFFER NEXT. Contact: [info@Rathor.ai](mailto:info@Rathor.ai).

- Crownstone Destroy path: Sherif's canon (cross-faction Draek + Alliance endgame raid, server-wide achievement) is recorded in CROWNSTONE_TRILEMMA_PATHS.md §2 (CROWNSTONE-QUEST-1).

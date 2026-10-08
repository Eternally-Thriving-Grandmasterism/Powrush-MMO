# COMBAT AND PVP — guild wars, battlegrounds, world PK, shields, consumables

**Base** `4fc13bad` (`4fc13bad8c829dab812fad155144d516302585ef`) after #684. Docs only. Online grey. Every number is a target.

**Contact:** [info@Rathor.ai](mailto:info@Rathor.ai). Design tick, not a Cargo bump. This file opens no socket, adds no listen, unparks no crate, and lights nothing on the Title. `NetMode::default()` stays `Offline`. `title_online_enabled` stays false.

**Every number in this file is a target, not a measurement.** Nothing here has been built, profiled, or load-tested. No performance, scale, or certification claim is made.

Peak memory still holds: walked · tended · week was the bill · yard remembered.

---

## 0. Sherif's ask (verbatim)

> "Combat gameplay should be the most optimal mix of Conquer Online Guild Wars, Heroes of Newerth, World of Warcraft Battlegrounds and for the open PVP something like the world PK system of Conquer Online; ultimately, in the Powrush-MMO universe when in permitted areas by realistic logic like a shielded area by a faction for lowbies/newbies to safely get started without combat risks and other smaller ones which can be setup as a protection shield in an area for temporary use by a group or a raid respectively and appropriately similar to World of Warcraft and their respective experiences strategically, tactically, even for all other worthwhile aspects like consumables to buff up and so on, because we can do even better with realism as well as sci-fi similar to the energy drinks and pills in PUBG and so on, but ultra better, till it's the Best"
>
> — Sherif, 2026-10-07 night ET

On 2026-10-08 Sherif asked for Conquer-style world PK in permitted areas. That request is his direction. §19 records it against the existing refuse lines.

A quote is a direction. This file is the councils' decision. It does not send the questions back. Where a ruling meets an existing refuse line, the offline game keeps that line until Sherif says "online yes" and Core names the cards.

---

## 1. Pillars

1. **Skill first, and readable.** A fight is decided by aim, timing, positioning, and supply. No published power scalar pre-decides it (`docs/GDD_IMMERSION_REVISION.md` §2.1 refuses Battle Power / Potency).
2. **Realism with a sci-fi reason.** Every rule has an in-world cause. Sanctuary shields are faction energy domes. Temporary shields are crafted generators. Bounties are records kept by faction sentries. Name colours are the sentry mark, painted on the person so a stranger can read it.
3. **PvP is opt-in by place.** Newcomers are not dropped into player damage. Peace is the default everywhere outside the contested wilds and the lawless deep, and those two are online-only.
4. **Borrow technique only.** Conquer Online, Heroes of Newerth, World of Warcraft, and PUBG contribute cadence, flags, objectives, lanes, and consumable timing. They contribute no names, art, text, or UI. Player-facing words stay Powrush words (`docs/PRE_RELEASE_LAW.md`).
5. **One economy.** Bounties, portal fees, and match rewards post through the one ledger. There is no PvP currency (`docs/LAYERED_GAMES_AND_SERVER.md` §3e).
6. **The yard ladder stays the yard ladder.** Temper, Lumen, and Ward are the only power ladder (`docs/MERCY_TEMPER_PROGRESSION.md`, `docs/FACTION_HERALDRY.md`). Combat does not add a second one. Shell Ward reduces stress on wells. It does not reduce player damage.

---

## 2. Offline now, online later

The game ships offline today. Online stays grey until Sherif says "online yes" (`docs/ONLINE_LADDER.md` R0–R7).

Offline combat, when a later card builds it, is PvE only: wildlife, hostile droids, aliens, and raid bosses. There is no player to flag, no bounty, no guild war, and no battleground.

Every PvP system in this file is online-only. It is designed now so it can slot in later. It is HOLD until "online yes" and until Core names PATHS. This doc names no CARD and no PATH.

This matches the merged ruling in `docs/LAYERED_GAMES_AND_SERVER.md` (CARD DESIGN-LAYERS-2, #640): player destruction is online-only. Offline, harm to another House's work happens only if that player turns on a local `world_harm_mode`, and Sanctuary stays protected.

Parked `server/` holds old combat scaffolding from merged PRs #39–#41 (interest management, a basic combat tick, lag compensation, health). It does not build today (`docs/LAYERED_GAMES_AND_SERVER.md` §2). It is a mine for ideas. It is not law.

Person-scale combat is this file. `docs/DOGFIGHT_MECHANICS.md`, `docs/BOARDING_MECHANICS.md`, `docs/FLEET_CLASSES.md`, and `docs/DRAEK_FLEET_AI_SYSTEMS.md` stay the fleet docs. This file does not retune them.

---

## 3. Terms

| Term | Meaning |
|---|---|
| **Proficiency** | A bracket key. It rises from practised verbs. It is not XP, not a damage bonus, and not a Place gate. The world does not scale to it. |
| **Newcomer Aegis** | The personal bubble that blocks all damage between players for a newcomer outside a dome. PvE damage always goes through. It is not the item Ward, and it is not a Heartwood Ward. |
| **Heartwood Wards** | Well · Grove · Ember seal posts (`docs/PLACES_BIBLE.md`). Dress and tend. They are not shields. |
| **Faction Sanctuary Dome** | A permanent faction energy dome over a starter area. No damage between players inside. PvE damage always goes through. |
| **Canopy** | A crafted, temporary shield generator. Pocket, Squad, or Hold. |
| **Surge** | The boost meter. Drinks and stims fill it. It drains and pays gradual regen and speed. |
| **Infamy** | The criminality axis. Sentries record it. It decays. |
| **Righteousness** | The other axis. A surplus does not decay. |
| **Clear / Aggressor / Marked / Outlaw** | The four name marks. Colour plus a shape plus the word. |
| **Muster Vault** | The war respawn chamber under the Holdfast. |
| **Charter Pole** | The held objective in a guild-war instance. Holding it accrues hold time. |
| **Charge** | A match-only resource inside the Lane Trial. It dies with the match. It is not on the ledger. |

**Proficiency gain (targets).** One mark per completed craft, one mark when a node you tended reaches Idle, one mark per PvE skillshot that lands. PvE shot marks cap at 20 per real day. A player kill grants 0 marks. Ten marks raise proficiency by 1. Cap 60. Kills never pay proficiency and never pay XP.

**Health in every PvP mode is 200.** Proficiency, Temper, Lumen, and Ward do not add PvP health or PvP damage. PvE uses the same 200 for the player. Enemy toughness belongs to the Place.

---

## 4. Zones

Places already on disk (`docs/PLACES_BIBLE.md`, `docs/PLAYABLE_RACES.md`): Sanctuary, Heartwood, Threshold, Depths. Peoples land there. Combat does not add a fifth Place.

| Zone | Where | Player damage | Stance | Shield you may raise | Death of a player |
|---|---|---|---|---|---|
| **Sanctuary dome** | Sanctuary Prime and the starter yards under a faction dome | Impossible | Toggle unavailable. Peace holds | None. The dome is already up | No player death |
| **Homestead core** | Hearth, bench, store, and an 8 m work radius | Impossible | Peace inside the radius | None needed | No player death |
| **Newcomer Aegis** | Contested wilds and the lawless deep, while the Aegis holds | Incoming damage from players blocked. PvE damage always goes through | May stay Passive in the wilds. The deep still forces Apprehend or Hostile for outgoing damage | Canopies, if the canopy rules allow | No player death from players |
| **Contested wilds** | Heartwood Teeth past the dome, Threshold, frontier hexes outside cores | World PK rules | Toggle matters. Default Passive | Canopies outside combat and outside 40 m of an objective | §9 |
| **War instance** | Scheduled Holdfast map | War rules, lethal because you entered | World toggle ignored | Canopies refused | Muster Vault if nobody revives you |
| **Battleground** | Banner Run, Node Claim, Convoy, Lane Trial | Match rules | World toggle ignored | Canopies refused | Match respawn. Own gear untouched |
| **Lawless deep** | Past the signed gate beyond the Depths landing | Anyone may strike anyone, except a player whose Newcomer Aegis still holds | Passive and Friendly refused. Apprehend or Hostile | Canopies allowed only out of combat and outside 40 m of an objective. Shorter duration (§5) | §9. Release is the landing, not the deep |
| **Depths landing** | The camp before the gate | Impossible. It is the way home | Peace | Pocket Canopy for rest | No player death |

Offline, the same ground is PvE or Peace. Sanctuary Prime stays a yard: no combat, no lethal UI (`docs/PLACES_BIBLE.md`). The Heartwood pond stays a bath. The lamp, Lip, and Heartwood Wards stay teaching ground. PvE wildlife starts at the Teeth. Hostile droids start at Threshold. Alien fauna and the raid seat start in the Depths. The lawless gate does not exist offline.

Leaving a dome, a homestead core, or the Depths landing plays one edge warning (§16): a banded shimmer for 15 m and a toast. The toast reads "Dome edge. Contested wilds ahead." or "Gate ahead. Lawless past this line." Entering the lawless deep asks for a confirm. With the Aegis up it reads "Lawless. Your Aegis still holds. Equipped gear stays on you." With the Aegis down it reads "Lawless. You can be struck. Equipped gear stays on you. Marked and Outlaw can lose carried goods."

Holiday ceasefires (`docs/LAYERED_GAMES_AND_SERVER.md` §7.4) and the Day of the Broken Light (§7.2, no war that day) close war instances and stand player damage down everywhere for the window. Domes keep working.

---

## 5. Shields

### 5.1 Faction Sanctuary Domes

A dome is faction infrastructure, maintained by upkeep the way a depot is. Inside it, damage between players does not land, including friendly fire, duel damage, and skillshots. PvE damage always goes through. Wildlife in the yard flees. It does not fight.

Guards at the edge are sentry posts, online only. They are not scheduled NPC persons and they do not break the Peace hour (`docs/NPC_SCHEDULE_SPEC.md`). They refuse Marked and Outlaw entry. They do not chase past 20 m into the wilds.

Redemption and rite traffic can still use the gate path the Place already has. A refused player is stopped at the shimmer. They are not pulled inside.

### 5.2 Newcomer Aegis

The Aegis is a personal tether the starter dome extends onto a newcomer who walks out. In-world, the dome's sentry still has them on the list.

It blocks incoming player damage. It does not block PvE. It ends at the first of these:

- Proficiency 12.
- 8 real hours spent in the contested wilds or the lawless deep. Time under a dome, on a homestead core, or at the Depths landing does not count.
- The newcomer lands a hit on another player.
- 14 real days from first step outside. This is the existing always-protected window for new players (`docs/LAYERED_GAMES_AND_SERVER.md` §3g). The Aegis never outlasts it.

When it ends early because they attacked, they can be struck back under ordinary wilds rules. When it ends because of proficiency, hours, or the 14th day, the homestead-core and dome rules still protect those places for everyone. Infrastructure protection for a new House through the first 2 real weeks stays the §3g rule even after the Aegis drops.

The Aegis also holds in the lawless deep until one of those endings. The gate confirm still fires, and it names that the Aegis is up.

The Aegis cannot be refreshed, bought, or reapplied.

PvE combat (wildlife, hostile droids, aliens, raid and quest bosses) is never blocked by any shield.

### 5.3 Canopies (temporary shields)

Crafted generators, harvested power cells, a visible footprint everyone can see. Sizes:

| Canopy | Who fits | Radius | Duration | Cooldown | Cells | Siege integrity | Raise time |
|---|---|---|---|---|---|---|---|
| **Pocket** | 1 | 4 m | 10 min | 20 min | 1 | 200 | 5 s |
| **Squad** | 5 | 12 m | 20 min | 40 min | 3 | 800 | 5 s |
| **Hold** | 20 to 40 | 28 m | 30 min | 90 min | 8 | 2400 | 8 s |

In the lawless deep, durations are half (5 / 10 / 15 min) and integrity is half.

Rules:

- Raising takes the whole channel. Damage interrupts it. The cells are spent on an interrupt.
- You cannot start a raise while you, or the ground under the footprint, is in combat (player damage or PvE damage in the last 8 s).
- War instances and battlegrounds refuse canopies.
- No canopy within 40 m of a contested objective: Charter Pole, war statue, battleground node, banner, convoy, Lane Trial beacon, or Relay.
- The footprint is a banded ring on the ground plus a soft dome. Pattern first, colour second.
- Enemies siege it. Sapper shots hurt it. When integrity hits 0, the dome drops and the generator becomes a ruin with 50% of its cell cost as salvage (`docs/LAYERED_GAMES_AND_SERVER.md` §3g ruins, not deletion).
- A living canopy blocks all damage between players across its skin. PvE damage always goes through. Siege shots damage the generator. People inside stay safe until integrity hits 0. They may Tend, craft, and revive.
- Uses: a safe rest, a crafting stop, a regroup, a raid camp before a boss. A canopy is not a capturable objective and scores nothing.

Offline, Pocket and Squad canopies are PvE camp tools once that card exists. Hold waits until group play exists. They never appear in the Hour 1 yard.

---

## 6. Stances

The old PvP Mode toggle becomes this stance control. It is live only in the contested wilds.

| Stance | You can damage | Default where |
|---|---|---|
| **Passive** | Wildlife, hostile droids, aliens, raid bosses | Contested wilds. This is the default |
| **Friendly** | As Hostile, except guild mates, party members, and the friends list | Chosen |
| **Apprehend** | Aggressor, Marked, and Outlaw players, plus PvE | Chosen. A legal hit on those marks adds 0 infamy |
| **Hostile** | Any player, plus PvE | Chosen. Hitting a Clear player marks you Aggressor |

Sanctuary domes, homestead cores, and the Depths landing ignore the toggle. The control is present and inert, and the toast says "Peace holds."

War instances and battlegrounds ignore it and use their own sides.

The lawless deep forces Hostile-capable entry. Passive and Friendly cannot be selected. On entry they become Apprehend. The zone does not flip anyone to Hostile on its own. A player may switch to Hostile. Apprehend still lets them hunt marks without hitting Clear people. Anyone without an Aegis can still be hit.

Hitting a guild mate, a party member, or a friend while Friendly does nothing. The shot fizzles on them. A Friendly player who hits a Clear stranger still becomes Aggressor.

---

## 7. World PK, infamy, and righteousness

World PK runs in the contested wilds and, wider, in the lawless deep. Both are online-only. Peace is the default stance in the wilds: a Passive player harms no one.

### 7.1 Marks

Colour is never the only signal. Each mark has a shape and the word on the nameplate, in the world, so a minimal HUD cannot hide the only cue.

| Mark | Colour | Shape | How it starts | What follows |
|---|---|---|---|---|
| **Clear** | White | Open circle | Default | A hit on you by a Hostile player flags that player |
| **Aggressor** | Yellow | Triangle | You hit a Clear player in the wilds or the deep | Lasts 90 s from the last such hit. Killing you costs the killer 0 infamy |
| **Marked** | Red | Diamond | Infamy reaches 100 | Domes and guards refuse you. A public bounty opens. On death in the wilds or the deep, 30% chance to drop one carried stack |
| **Outlaw** | Black, white outline | Filled square | Infamy reaches 300 | The same, with a 60% drop chance. The Draek Dominion invitation can open |

A house glyph for guild or party sits beside the mark. It never replaces the mark.

Killing an Aggressor, a Marked, or an Outlaw adds 0 infamy. Killing inside a duel, a battleground, or a declared war instance adds 0 infamy. Those fights are consented frames.

### 7.2 Infamy

Infamy is criminality, stored as an integer 0 to 500.

- Killing a Clear player: **+25**.
- If your proficiency is 8 or more above theirs, multiply that gain by 3.
- If your highest Temper is 2 or more steps above theirs, multiply by 3.
- If both gaps apply, multiply by 4. Cap the one kill at +100.
- Three Clear kills on players 8 or more proficiency below you, inside one real hour: an extra **+50**, and a review-log mark (§16).
- Decay: **2 per real hour** while you are out of combat and not standing in the lawless deep. Decay pauses for 10 minutes after you hit a Clear player.
- Floor 0. Cap 500.

A good deed lowers infamy when infamy is above 0. The deed is a real verb for a player below proficiency 20 who is not in your guild, not in an allied guild, and not on your friends list: tend a Stressed node of theirs through to Idle, escort them along a corridor they did not start, or hand them a crafted consumable they then use. They confirm with E. You must be within 30 m.

Anti-farm: −8 infamy per deed, once per pair per 7 real days, at most 3 deeds per real day (−24). No credit for a character on your account, a character that shared your guild or friends list in the last 30 days, or a deed with no confirm. Standing nearby pays nothing.

### 7.3 Righteousness

The same good deed, performed while infamy is 0, grants **+8 righteousness** instead. Same pair window, same daily cap, same anti-farm rules. Righteousness does not decay. Cap 500. An innocent kill does not subtract righteousness. It adds infamy, and it resets any open Alliance invitation timer (§8).

The two axes stay separate. A player can hold leftover righteousness from an older life and still earn infamy now. Sentries show the mark from infamy alone. Righteousness is a quiet total on the status chip, not a second name colour.

---

## 8. Lore faction invitations

Sustained extremes invite a player into a lore faction. The invitation is an offer. Declining leaves the numbers as they are.

| Qualification | Path | Faction | Mothership | What membership gives |
|---|---|---|---|---|
| Infamy at Outlaw (300) held 14 real days | Criminal | **Draek Dominion** | **The Brood Spire** | Dominion missions, a Dominion channel, faction gear that is paint and a title (`docs/FACTION_HERALDRY.md`). No stat bonus |
| Righteousness at 200, held 14 real days with no Clear kill in that window | Righteous | **Quellorian / Aetherion Luminari Alliance** | **The Auroral Unification Nexus** | Alliance missions of the same shape. Paint and a title. No stat bonus |

Names follow `docs/FACTIONS_OVERVIEW.md`. The old GDD's "The Council" is this Alliance. `docs/COUNCIL_SYSTEM.md` Councils are Local → Regional → Global governance. They are not a faction, and this file does not call the righteous path a council.

Joining is a long commitment. It is not permanent. The 14-day hold resets if the qualifying number drops under its line. The Alliance hold also resets on a Clear kill.

- While you stay in the Dominion and keep the invitation, infamy decay stops. Good deeds still lower infamy. Below 300, Brood Spire access closes. Below 100, the invitation lapses and you are unaligned.
- Leaving cleanly is a redemption quest: 28 real days of restore-and-help verbs under the existing redemption framework (`docs/REDEMPTION_MECHANICS_PER_SPECIES.md`, `docs/DISCORDANT_REDEMPTION_QUESTLINES.md`, `docs/SPECIFIC_REDEMPTION_QUESTS.md`). The shipped slice is the Sylvaris grove, "One tend. Not a war." (`docs/HUMAN_PLAYABILITY_v23.2_SPECIES_REDEMPTION.md`). You pay nothing. You redeem by restoring and helping.
- Alliance membership: one Clear kill opens a 7-day censure. A redemption verb inside the week clears it. A second Clear kill inside the week expels you. Righteousness stays on the character either way, because it does not decay.
- City access follows the faction you currently have. Expulsion or a finished exit quest returns you to unaligned access: domes by the mark rules, motherships closed until you qualify again.
- On a later Ascension rite, infamy is halved toward 0 and righteousness is kept. Faction membership must be re-confirmed within 7 real days or it lapses to unaligned. The rite is not a free wash and it is not a permanent stain.
- A character who moves servers carries infamy, righteousness, and allegiance as they are. The destination's domes and guards apply their own entry rules on arrival. Server histories still never merge (`docs/LAYERED_GAMES_AND_SERVER.md` §7.6).

Faction gear from these paths is physical finish: paint, a grip, a tabard. Cosmetics never change stats.

---

## 9. Death, gear, and bounties

Equipped gear stays on the body in every zone. Ordinary victims do not drop it. There is no XP transfer on a kill.

| Zone | Equipped | Carried goods | Durability and charges | Proficiency / XP | Where you wake |
|---|---|---|---|---|---|
| Dome, core, landing, Aegis | Stays | Stays | Untouched by players | Untouched | You do not die to a player |
| Contested wilds, victim Clear or Aggressor | Stays | Stays | 2% of a mend cycle nicked, repaired at a bench | Untouched | Your homestead core, else the nearest dome edge, outside the dome |
| Contested wilds, victim Marked | Stays | 30% chance, one carried stack | Same nick | Untouched | Outside any dome |
| Contested wilds, victim Outlaw | Stays | 60% chance, one carried stack | Same nick | Untouched | Outside any dome |
| Lawless deep, Clear or Aggressor | Stays | Stays | Same nick | Untouched | Depths landing |
| Lawless deep, Marked or Outlaw | Stays | 30% or 60%, one stack | Same nick | Untouched | Depths landing |
| War instance | Stays | Stays | Charges and durability spend on the map. They do not spend in the Muster Vault | Untouched | Muster Vault, then the next wave |
| Battleground or Lane Trial | Own gear locked and returned | Own satchel locked and returned | Match kit only | Untouched | Match respawn |
| Duel | Restored to the stored state | Restored | Restored | Untouched | The ring's edge, state restored |
| Offline PvE | Stays | Stays. A bait you chose to spend is spent | Tool rest follows Temper law. Items are never deleted | Shot marks only, under the daily cap | The entrance of that Place |

The dropped stack is the highest satchel stack that is not a quest item, not mid-craft on a bench, and not a heraldry piece in progress. It lies for 2 minutes. The owner or the killer may Take it. Then it becomes ordinary world salvage.

A disconnect while in player combat leaves the body for 30 s under the same drop rules, then releases it. This stops a logout from erasing a loss. It is separate from the war-map grace in §17.

**Spawn calm.** At a wilds or landing release you have 8 s and a 6 m radius where you cannot deal or take player damage. Attacking or walking out ends it early.

**Bounties.** Sentries publish a bounty on every Marked or Outlaw player. Claim it by bringing that player to 0 health in the contested wilds or the lawless deep. Duel, war, and battleground kills do not claim it.

Payout posts on the one ledger from source `bounty_claim`: 40 Reserve, plus 10 per 100 infamy, capped at 120. The server emission cap is 2,000 Reserve per week from this source. Past the cap, the claim still logs and pays 0. Once per hunter per outlaw per 24 real hours. No claim on your own account's characters. No claim on a player who was on your friends list or in your guild in the last 30 days.

---

## 10. Weekly guild war

Technique borrowed: a weekly siege of a held place, with a real prize for the week. The prize is stewardship, tied to the player economy and the RBE path (`docs/LAYERED_GAMES_AND_SERVER.md` §3c). It is not pay-to-win, not a kill board, and not a cash shop.

**Window.** One declared window per cycle week, 48 real hours. A cycle is eight real weeks (`docs/LAYERED_GAMES_AND_SERVER.md` §7.2). The Day of the Broken Light closes the instance if it falls inside the window. Hold time pauses. The window reopens after that day. A holiday ceasefire does the same (§7.4). A War calendar ballot (§7.5) may shift the window by two hours. It may not add a second war.

**Score.** The shipped helper is tons plus restored (`shared/war_week.rs`). This design adds hold time, which §3c already describes:

`score = tons + restored + hold_minutes`

There is no kill term. A kill matters only when it changes who holds the pole, the corridor, the nodes, or the statue. A guild may win the week without entering the instance, on tons and restored alone. Entering the instance is the lethal opt-in. Losers keep homesteads, Reserve, and standing. They lose next week's charter on the contested cluster. A winner who strips the frontier inherits a Stressed frontier.

**Who may enter.** Guild members, plus a solo player who signs on as escort, scout, or mender for one guild for that window. Companions are barred from every war role (`docs/LAYERED_GAMES_AND_SERVER.md` §6.1): fighter, hauler, holder, scout.

**Scale target,** already stated in §8 P9: 2×100 first, 2×250 stretch. Those are targets, not measurements.

### 10.1 Maps

At least five maps rotate, one per war week, in order, then repeat. The approach is **the Way In**: a short signed route, one branch, the same all week. It is readable on purpose. The old weekly maze is retired (§18).

| Map | Objective | Who takes the week's privileges |
|---|---|---|
| **Pole Hold** | Charter Pole hold time | Highest score |
| **Corridor Cut** | A logistics corridor. Tons weigh double inside the score for this map only | Highest score |
| **Restore Race** | Nodes start Stressed. Restored weighs double for this map only | Highest score |
| **Statue Break** | A destructible Week Figure. Damage to the figure is tallied | The guild with the most figure damage |
| **Twin Gate** | Two gates. Hold time splits across them | Highest score |

On Corridor Cut the tons term is doubled. On Restore Race the restored term is doubled. Hold minutes stay 1 per minute. The doubled term is still the same score family. It is not a kill count.

The Week Figure is chosen by the winning guild from pre-made designs. It stands at the Holdfast entrance for the week. It is paint and form, no stats, under `docs/FACTION_HERALDRY.md` (original library, no real marks). On Statue Break the figure is the objective and can be ruined. Ruin, not deletion. Next week's figure is a fresh pre-made piece.

### 10.2 Muster Vault

A player killed in the instance, and not revived by another player, wakes in the Muster Vault under the Holdfast. People there may spar. Sparring spends no durability and no charges. Every 12 minutes the vault releases a wave onto the Way In. The route does not change during the week.

### 10.3 Duties of the holder

The holder guild, for that week:

- Keeps portal coverage (§11).
- Mends the cluster it won. If the cluster is Stressed at the next window, the fee cap for the following week is 0.
- Does not raise fees above the cap, does not switch portals off, and does not refuse a people or a newcomer.

Perks are the capped portal fees into the guild bank, the Week Figure, and the charter rights §3c already names. No real-money perk exists.

---

## 11. Portals

The holder repositions the server's portal network. There are 12 portals.

- Fee is 0 Reserve below proficiency 12. At proficiency 12 and above, the fee is 1 Reserve per 10 proficiency, capped at 8. A newcomer pays nothing. The cap keeps the veteran price from running away.
- Fees post to the holder guild bank through the one ledger, source `portal_fee`.
- At least 4 of the 12 sit at canon landings: Sanctuary gate, Heartwood approach, Threshold, Depths landing. The other 8 may move.
- A portal cannot be switched off. Moving one starts a 6-hour cooldown. The old site keeps a beacon for 30 minutes.
- Portals sit in the open or at the entrance of a key-gated or credit-gated area. They do not sit inside those areas. They do not sit inside the Sanctuary Prime yard. The gate is allowed.
- Marked and Outlaw players may use a wilds portal. An exit that would land inside a dome drops them at the shimmer instead.

If a ceasefire covers the whole window, the current holder keeps the duties, including the fee cap. If there is no holder, the 4 landing portals stay, and fees are 0.

---

## 12. Battlegrounds and the Lane Trial

Instanced, matchmade, online-only. A terminal opens the queue. The terminal is a machine, not a person, so scheduled persons still never fight. Companions are barred. Humans first if a group finder is ever added (`docs/LAYERED_GAMES_AND_SERVER.md` §6.2).

Gear is normalised in rated play: the same 200 health, the same role kit, the standard consumable kit (§15). Own Temper, Lumen, Ward, and satchel are locked for the match and returned after. Unrated matches use the same lock. Crafted quality does not decide a match. World PK and guild wars are where real satchels matter.

| Mode | Player-facing name | Win | Length | Team |
|---|---|---|---|---|
| Capture the flag | **Banner Run** | Banner captures. Kills are not the score | 15 min | Two sides, queue fills |
| Resource control | **Node Claim** | Hold time on three nodes | 20 min | Two sides |
| Escort or siege | **Convoy** | The hauler arrives, or it is stopped | 12 min | Two sides |
| Lanes | **Lane Trial** | The Relay falls, or greater structure damage at the cap | 25 min | 5v5 |

Player-facing text says Banner Run and Node Claim. Node Claim borrows resource-control technique. The reference map name (Arathi) stays out of player-facing text.

Ally damage is off in Banner Run, Node Claim, and Convoy. In the Lane Trial, a skillshot can clip an ally. Heals still prefer allies. That clip is the lanes tax, and it is part of the skill.

**Lane Trial depth.** Three lanes. Droid waves every 30 s. Three beacons per lane, then a Relay. The droids are match entities. They are not the product drones `docs/OFFLINE_ECONOMY_COURT.md` refuses, and they are gone when the match ends.

Last hit grants 20 charge. A deny (last hit on your own droid) grants the enemy 0 and grants you 2. A beacon last hit grants 40. A player kill grants 30 charge and 0 infamy and 0 proficiency. Charge buys match consumables and a 30 s speed step that matches Surge's +8% and does not stack with it. The shop sells no damage multiplier and no persistent gear. Match proficiency inside the Trial runs from 1 to 12 and is deleted at the end. It does not touch the character's proficiency.

Rated placement pays a title and a tabard thread. Both are cosmetic (`docs/LAYERED_GAMES_AND_SERVER.md` §11.3). They never touch the ledger.

---

## 13. Duels, scrimmages, and the Cycle Final

### 13.1 Duels

A duel is consensual and happens outside domes, cores, and the landing.

- The challenger offers a crafted **Duel Token** (a small metal piece, not a real-money item). One outgoing offer at a time. The token sits in escrow. It returns on a decline or after 60 s with no answer. It is spent when the duel starts.
- Same map. Proficiency within 5.
- The ring lasts at most 3 minutes. Other players' damage does not land inside it. Leaving the ring forfeits.
- The log line is public, on the character's ledger page: "A (proficiency, role) challenged B (proficiency, role) and won." A draw logs "drew."
- At the accept, the server stores health, charges, consumable counts, and durability. At the end it restores that state. The duel is not a free heal.
- Reward: the first decisive result between that pair in 30 real days may pay one cosmetic (a heraldry thread or a title). Further duels in the window still log and still restore, and pay nothing. Draws pay nothing. The cosmetic never grants PvP power. There is no duel-point currency and no shop of stat gear.

Safe zones do not host duels. There is no item that forces a fight inside a dome.

A 1v1 queue at the battleground terminal uses these restore rules and the standard kit. Queue pairing stays within 5 proficiency. A 5v5 queue stays within a spread of 10.

### 13.2 Bracket Scrimmage

Each proficiency bracket (1–10, 11–20, 21–30, 31–40, 41–50, 51–60) has a free-for-all scrimmage with three start times in the real day, 15 minutes each, so a player in any waking region can reach one. Missing it costs nothing. There is no streak and no login reward (`docs/GODSPEED_PREP.md`).

One cosmetic or one T1 `challenge_reward` per character per day, across every bracket. You cannot hop brackets for a second payout. The scrimmage is practice toward the guild war. It writes a readiness line on the guild. That line adds 0 to the war score.

### 13.3 Cycle Final

Once a month, online, a later phase. A championship server takes copies of characters. Titles and heraldry write back to the home server. Ledger wealth does not. Losses on the copy do not touch the home character's gear, infamy, or satchel. Server histories do not merge. This waits on "online yes" and on cards Core names after the earlier PvP phases.

---

## 14. Skillshots, jump, roles, and PvE

### 14.1 Aimed shots

Skillshots are aimed volumes with travel time. Every damaging ability except the basic strike telegraphs for at least 0.35 s. The telegraph is a ground shape: line, circle, or cone, drawn as stripes or dots. Colour is an extra. Reduced-motion play pops the shape in at full size, still on the same timer.

The basic strike is short range, 0.2 s, 8 damage, costs no charge, and is aimed at a body in front of you.

Jump is part of the dodge. Air time 0.4 s. Landing recovery 0.15 s. Height 1.2 m. A ground-aimed shot misses an airborne body. Shots marked air-true can hit. Air-true is rare: the Striker's line lance is the only air-true shot in the starting kits.

Hard crowd control diminishes. The second hard control inside 8 s lasts half as long. The third grants immunity for 8 s.

### 14.2 Role kits

Kits are chosen at a match terminal, or carried as a crafted tool in the world. They are not a Title class lobby and not a race lobby (`docs/GDD_IMMERSION_REVISION.md` §4). Five kits, four shots each. Damage numbers are against 200 health.

| Kit | Shots (tell, damage or effect) |
|---|---|
| **Striker** | Line lance, air-true, 0.4 s, 20. Arc mortar, ground, 0.55 s, 24. Step, 6 m, no damage. Riposte, 0.3 s window, reflects the next skillshot |
| **Bulwark** | Plate, 3 s, incoming damage reduced by 30%. Wall, 4 m, 4 s, blocks shots. Pull, one target, 3 m. Brace, the next hit on an ally within 8 m lands on you |
| **Mender** | Beam, heal 30 over 2 s, breaks if the mender takes damage. Cleanse, one slow or root. Calm field, allies regen 8, enemies slowed. Draught toss, an ally gains 20 Surge |
| **Runner** | Kick, +20% speed for 3 s. Smoke, 4 s, breaks target locks, body and name mark stay visible. Haul, faster hands on a banner, convoy, or crate. Drop, one medkit charge; in the world this spends a real medkit from the satchel |
| **Sapper** | Breach, heavy on canopies, beacons, and the Week Figure, 8 on players. Mine, 30 on structures, 12 on players, arms in 1 s. Strip, a canopy loses 2 s of duration. Patch, slow repair of an allied canopy or beacon, breaks if you take damage |

Smoke is not invisibility. There is no stealth that hides a mark.

Tool quality does not change these numbers. `docs/PLACES_BIBLE.md` keeps combat stats off recipes. Quality changes consumable timing (§15), not hit size. Faction recipes change who can craft the item and how it is painted. Both factions share the same combat numbers, so faction choice is not a damage side.

### 14.3 Keys, when combat is eventually built

The cold-start door stays: WASD · Space · Shift · E take/tend · I satchel · H hide · R allocate. Do not bind W. E keeps take and tend. R keeps allocate. There is no F-row.

Until Sherif lifts the parked-combat line on a ticket, no combat key exists. When a card does bind one, it is a single fire key Core names, plus aim. It is not W and it is not E. While a Care prompt owns `ACTION_BAR`, the fire key does nothing (§16).

### 14.4 Offline PvE

Enemies are of the Place. They are never scheduled NPC persons, never House NPCs, and never the people of the hour.

| Enemy | Place | Tell | Hit (target) |
|---|---|---|---|
| Grazer | Heartwood Teeth | Flees. No fight | 0 |
| Predator | Heartwood Teeth | Lunge, 0.6 s | 15 |
| Hostile droid | Threshold | Shot, 0.5 s | 25 |
| Alien fauna | Depths | Strike, 0.45 s | 35 |
| Raid construct | Depths pocket | A new tell at 75%, 50%, and 25% of its pool | Pool 4,000. Each slam is telegraphed |

Hostile droids are enemies. They are not harvest drones and not a product (`docs/OFFLINE_ECONOMY_COURT.md`).

The raid can be attempted alone offline. Failure wakes you at the entrance. The yard is not wiped (`docs/GODSPEED_PREP.md`). Companions stay barred from end-game raid bosses (`docs/LAYERED_GAMES_AND_SERVER.md` §6.1). A group-sized raid is a later online seat, still with tells on every slam.

---

## 15. Consumables

Player-crafted from harvested stock. Families:

| Item | Inputs | Effect | Use time | Surge fill | Tolerance and side effect |
|---|---|---|---|---|---|
| **Hydration pack** | Water, mineral salt | Clears thirst. Small stamina return | 1.5 s | 0 | Extra doses do nothing harmful |
| **Electrolyte pack** | Hydration inputs, extra salt | Stamina regen for 30 s | 2.0 s | +10 | Third dose inside 10 min: a cramp, −20% speed for 20 s |
| **Ration bar** | Grain or protein harvest | Clears hunger. Slow health regen, 20 over 20 s | 3.0 s | 0 | A fourth bar inside 10 min slows movement 15% for 30 s |
| **Nano-medkit** | Fabricated mesh, reagent | Heals 60 | 4.0 s | 0 | 60 s cooldown. A second inside 3 min heals 30. Interrupt heals 0 and still spends it |
| **Adrenaline stim** | Gland-analogue, reagent | +15% speed and steady hands for 8 s | 1.0 s | +25 | Then a tremor: wider aim for 6 s. Later doses inside 10 min add less and deepen the crash |
| **Focus stim** | Sibling reagent | Telegraph timer drawn 0.1 s earlier for you, for 10 s | 1.0 s | +15 | Tunnel: the place-name chip hides for 8 s. Your own mark stays |
| **Catalyst Draught** | Water, sugar harvest, catalyst | The energy-drink role | 1.2 s | +40 | If Surge crosses 60 and later hits 0, crash: −8% speed for 20 s |

Any damage during the use time cancels the channel. The item is spent. Panic use is a real cost.

**Surge.** Range 0 to 100. While above 0 it drains 8 per 10 s and grants gradual regen (2 health per 10 s) and +8% move speed. Regen and speed from Surge do not stack with a second source of the same buff. The higher one wins.

**Tolerance.** Per item family, inside a 10-minute window with no gap: the first dose is full, the second fills 70%, the third fills 40%, the fourth fills 0 and applies the side effect once more. The window clears after 10 minutes without that family.

**Quality.** Rough, Sound, Fine, Faction. Crafter skill gates them: Rough from the first craft, Sound after 20 successful crafts of that family, Fine after 60 with a tool that is not resting, Faction from Fine plus standing with that faction. A failed craft rests the tool and does not destroy the inputs' container item. Quality changes use time, not damage: Rough +20% use time, Sound as the table, Fine −20% use time, Faction the same use time as Fine plus 5 extra Surge fill on drinks and stims. Alliance and Dominion recipes share those numbers.

**Where the satchel counts.** Battlegrounds, the Lane Trial, rated play, and the 1v1 queue issue a standard kit and lock the satchel: 2 hydration, 2 ration, 1 nano-medkit, 1 Catalyst Draught, 1 adrenaline stim, 1 focus stim, all at Sound timing. World PK, the lawless deep, guild wars, and offline PvE use what you crafted and carried. You can run out. That is the tactical part.

No real-brand drinks, no sponsored marks, no franchise names (`docs/PRE_RELEASE_LAW.md`, `docs/FACTION_HERALDRY.md` §5).

---

## 16. HUD, reports, and accessibility

Combat UI adds no anchors and no new overlapping panels. `war_week` and `faction*_ui` stay parked with no anchors (`docs/design/UI_LAYOUT_SYSTEM.md` §1.6). Reads join the registry that file already defines.

| Read | Class | Anchor | Sharing |
|---|---|---|---|
| Dome edge, lawless confirm result, ceasefire | 4 toast | `TOP_TOAST` | Rank 1 for its 8 s, then it yields. One toast at a time (R2) |
| Own mark word, infamy or righteousness number, Surge pips | 5 status | `CORNER_WATCH` | One line inside the existing budget. In a combat zone it takes rank 1 and Watch yields (R2). Outside combat, Watch returns |
| War or match objective, one line | 5 status | `TRACKER_1` | Yields to Embassy while Embassy shows (R2) |
| Up to four kit shots | 2, folded into the bar | `ACTION_BAR` | CareStrip and CarePrompt outrank it. While they show, the fire key does nothing. The tend prompt owns that moment |
| Duel log, bounty list | Existing ledger page | Ledger fixed rect | Not a new window |
| A full fight summary | 1 panel | `WINDOW` | R3: opening it closes the other window occupant through that panel's own close |

Presets classic, minimal, and management keep these reads on the anchors those presets already move (`docs/design/UI_LAYOUT_SYSTEM.md` §3.2–§3.4). Combat adds no R1 pair. Classes 2 and 3 are never covered (R4). Pause, Places, and the launch door still yield the HUD (R5).

The world nameplate carries the mark's shape and word even when the chip is yielded. That is the colour-blind guarantee: pattern, shape, and text. Colour is a fourth channel.

**Reports.** Online, after "online yes": report conduct (hate, harassment, repeated killing of low-proficiency players, real-money trade) or a banner. Hide the reported chat and the reported banner locally at once. The body stays visible, so a report is not a combat vanish. The report enters the review queue. This extends `docs/FACTION_HERALDRY.md` §5, which is already report → hide locally → review queue, online only. Every flag, kill, bounty, canopy drop, and report is an event-log row (`docs/LAYERED_GAMES_AND_SERVER.md` §8 P2 and §3g) so a review can roll an exploit back.

Offline, banner hide stays local, matching the heraldry phase ladder. There is no player-conduct queue while Online is grey.

---

## 17. Disconnect and AFK

- **30 minutes** with no input disconnects an online session. An AFK body is not left in the world to be farmed.
- On a war map or another special map, a drop that is not a proper logout starts a **3-minute** grace. The body is moved to a holding room beside the Depths landing rules: present on a peace map, not on the live war map, so it is not a free kill. Reconnect inside the grace returns you to the instance. Miss it and you join the next Muster Vault wave with your gear intact.
- Combat logout in the wilds or the deep uses the 30 s body rule in §9, not the war grace.
- The grace is a reconnect courtesy. It is not an exploit window: you deal no damage from the holding room.

---

## 18. Lineage from the v2.x GDD

Sherif's older GDD (v1.5, v2.0, v2.1) already wanted a PvP toggle, karma, weekly wars, portals, duels, and skillshots. This file keeps the bones, modernises the unfair parts, and retires the rest. The kept rulings are the ones in §4–§17. This section is the map.

### Kept and modernised

| Old idea | Ruling now |
|---|---|
| PvP Mode: Hostile, Friendly, Apprehend, Passive | §6. The toggle is real only in the contested wilds. Domes forbid it. War and battlegrounds have their own sides. The lawless deep forces Hostile-capable play and still allows Apprehend |
| Righteousness and criminality, two axes | §7. Criminality is infamy. It decays. Good deeds lower it, with caps, a once-per-pair window, and no alt credit. Righteousness does not decay |
| Invitations at sustained extremes | §8. Dominion and Alliance, repo names, motherships included. A slow quest is the way out. It ties to species redemption. Not a permanent brand |
| Guild-war winners run the teleport network, about 12 portals, nominal fees | §11. Fee ceilings, a free band for low proficiency, minimum landings, no switch-off, entrances only |
| Weekly rotation of 5 or more maps | §10.1. Five named maps |
| A destructible statue whose damage tally takes the privileges | Statue Break. Damage to the Week Figure, not a kill tally |
| Winner statues from pre-made designs | The Week Figure. Original presets only |
| A respawn room where the dead spar for free and leave in waves (old name "Happy Fun Chamber") | The Muster Vault. Wave every 12 minutes. That old name is not used |
| Daily bracket free-for-alls that feed the war | Bracket Scrimmage. Three start times, one payout a day, 0 war-score |
| Consensual duels, a public log, a level window, a 30-day reward limit, state restore | §13.1. Token is crafted. Reward is cosmetic |
| A monthly cross-server championship on character copies | Cycle Final. §13.3. Online, later |
| A 30-minute AFK disconnect, plus a reconnect grace on special maps | §17 |
| Skillshots, and jumping as a dodge, balanced on purpose | §14. Tells, air-true versus ground shots, diminishing control |

The old open question about Ascension and karma is decided in §8: infamy halves, righteousness stays, allegiance is re-confirmed or it lapses.

### Retired

| Old idea | Why |
|---|---|
| Bribing righteousness with a donation | Pay-to-win. Righteousness comes from deeds |
| Taking over NPC shops with real money | Pay-to-win, and it fights the no-mall rule |
| Per-server cryptocurrency payouts | Retired. Out of scope. It clashes with the no-crypto direction. This doc promises no earnings |
| The killer steals level EXP from the victim | Too punishing, and XP-from-kills is already refused |
| Ordinary victims drop equipped gear | Breaks the promise that a fair death keeps what you wear. Only Marked and Outlaw risk one carried stack |
| A special dagger that forces a safe-zone duel and makes the loser drop rare gear | Breaks the dome. Domes do not host forced fights |
| The approach maze, regenerated every week | Unreadable, and it punished anyone who died or reconnected. The Way In is short and stable |
| Real-brand consumables and brand deals | Refused by `docs/PRE_RELEASE_LAW.md` and by heraldry IP rules |
| Duel points that buy PvP-power gear | A second currency and a power mall. Rewards stay cosmetic, or the standard kit inside a match |
| Criminal faction as a door that can never open again | Replaced by the redemption exit. A long stay is enough commitment |

---

## 19. Where this meets existing law

These lines stay in force for the offline game, and they stay in force until Core names cards and Sherif says "online yes". This file does not pretend to lift them.

| Existing line | Where | What this design does |
|---|---|---|
| Conquer-style world PK in permitted areas. Sherif, 2026-10-08 | This file, his direction | World PK is specified for the contested wilds and the lawless deep, online only. Peace stays the default outside those two zones. Inside the wilds the personal default is Passive, so a player is not flagged until they choose to strike |
| Combat is parked. "No F-key combat." Hour 1 door has no combat key | `docs/MERCY_TEMPER_PROGRESSION.md`; `docs/HUMAN_PLAYABILITY_v23.2_DECLARED_LETHAL.md`; `docs/HUMAN_PLAYABILITY_v23.2_WAR_WEEK.md` | Phase 1 PvE is the direction. Building it needs a Steward ticket that lifts this line, and cards Core names. The cold-start door stays. W and E keep walk and tend |
| Persons never break Peace, never fight | `docs/NPC_SCHEDULE_SPEC.md` §6 | Offline enemies are wildlife, hostile droids, aliens, and raid bosses. Never scheduled persons or Houses |
| Open PK on every map, drop-on-death loot pressure, XP from kills | `docs/GDD_IMMERSION_REVISION.md` §2.1 | PK stays in two online zones. No XP from kills. Equipped gear never drops in ordinary PvP. Only Marked and Outlaw risk part of what they carry. Sanctuary stays Peace. New players keep the Aegis rules in §5.2 |
| Peace is the default. Lethal is opt-in after the book, on DeclaredLethal | `docs/GDD_IMMERSION_REVISION.md` §2.1 and §4; the DeclaredLethal slice | Domes, cores, and the landing stay Peace. The wilds default to Passive. War lethal exists only inside a declared instance you entered. The book path is unchanged |
| War score is tons + restored. "Not loot. Not lethal." | `shared/war_week.rs`; the war-week slice | Hold time joins that score, as §3c of the layered plan already says. Kills are not a term. They can only change hold or objective control |
| `world_harm_mode` governs harm to infrastructure, NPCs, animals, and droids | `docs/LAYERED_GAMES_AND_SERVER.md` §3g | Player-versus-player flags are a separate online rule set. Both respect Sanctuary Prime, homestead cores, the new-player window, and holiday ceasefires |
| Online grey. No listen. `server/` parked | `docs/ONLINE_LADDER.md`, `AGENTS.md` | Every PvP system here is HOLD until "online yes" and named PATHS |
| Companions barred from weekly guild wars, raid bosses, ranked or scored contests, and holding territory | `docs/LAYERED_GAMES_AND_SERVER.md` §6.1 | Barred from guild wars, battlegrounds, the Lane Trial, rated play, and end-game raids |
| No combat stats on recipes. Cosmetics never change stats | `docs/PLACES_BIBLE.md`, `docs/FACTION_HERALDRY.md` | Hit size is the kit. Quality changes consumable timing only. Heraldry stays physical |
| Drones and robots as product are refused offline | `docs/OFFLINE_ECONOMY_COURT.md` | Hostile droids are PvE enemies. They are not harvest bots and not a sold companion |
| No daily login, loot box, or season pass | `docs/GODSPEED_PREP.md` | Scrimmages do not streak. Missing a window costs nothing |
| New anchors for parked `war_week` and `faction*_ui` | `docs/design/UI_LAYOUT_SYSTEM.md` | This file adds none. Combat reads share the existing registry |
| Seat notes in `AGENTS.md` | `AGENTS.md` | Not this file's job. This file does not edit them |

---

## 20. Build order

Docs only. Core names the cards. This list is the order, not a queue and not a set of PATHS.

1. **PvE combat core, offline.** Wildlife, hostile droids, aliens, one raid construct. Tells, jump, the five kits, no player damage. Needs the parked-combat line lifted on a ticket. Hour 1 Peace stays.
2. **Consumables and crafting.** The seven items, quality, tolerance, Surge. They work on PvE first.
3. **Canopies.** Pocket and Squad as camp tools. Hold when groups exist. Siege rules exist in the data even if the first attackers are PvE.
4. **Online-only, after "online yes" and named PATHS.** World PK and the Aegis, then battlegrounds and the Lane Trial, then the weekly war, portals, scrimmages, and last the Cycle Final.

Each step is one later slice. None of them is this PR.

---

## 21. Refuse

- Lighting Title Online, adding a listen or a public bind, unparking `server/`, or treating this file as code.
- A power scalar, Battle Power, or any number that pre-decides a fight.
- XP from kills. Proficiency from kills. A kill term in the war score.
- Open PK on Sanctuary, on homestead cores, on the Depths landing, or on any offline map.
- Equipped-gear drops for ordinary players. Forced duels inside a dome. Safe-zone loot daggers.
- A PvP currency, a duel-point mall, or stat gear bought with match points.
- Pay-to-win: donations for righteousness, real-money shops, crypto payouts, paid shields, paid kits.
- Real-brand consumables, franchise names in player-facing strings, and uploaded statue meshes.
- Companions in wars, battlegrounds, rated matches, or end-game raids.
- Invisibility that hides a name mark. A report that hides a body in combat.
- New HUD anchors, a second combat panel, or an F-row on the Hour 1 door.
- Binding combat to W or to E's tend.
- NPC persons who fight.
- Invented CARD names or PATHS. Core names those.
- Performance, scale, certification, or earnings claims. Every number here is a target.

---

## 22. Open tuning numbers

Numbers only. The rulings above stay if these move.

- Proficiency: 10 marks per point, cap 60, 20 PvE shot marks per real day.
- PvP and PvE player health: 200.
- Aegis: proficiency 12, 8 hours in the wilds or the deep, 14 real days, ends on the first hit dealt.
- Dome shimmer 15 m. Guard leash 20 m. Homestead work radius 8 m. Spawn calm 8 s and 6 m.
- Aggressor mark 90 s. Infamy +25, ×3 or ×4 on the gaps, cap +100 per kill, extra +50 on the third low-proficiency kill in an hour. Thresholds 100 and 300. Cap 500. Decay 2 per real hour.
- Gap tests: 8 proficiency, 2 Temper steps.
- Good deed ±8, pair window 7 days, 3 deeds a day, 30 m, 30-day alt exclusion.
- Invitations: 14 days at 300 infamy or at 200 righteousness. Exit quest 28 days. Censure 7 days. Ascension infamy halved. Re-confirm allegiance in 7 days.
- Canopies: radii 4 / 12 / 28 m. Durations 10 / 20 / 30 min (half in the deep). Cooldowns 20 / 40 / 90 min. Cells 1 / 3 / 8. Integrity 200 / 800 / 2400. Raise 5 s, Hold 8 s. Combat memory 8 s. Objective keep-out 40 m. Ruin salvage 50%.
- Drop chances 30% and 60%. Dropped stack stays 2 min. Combat-logout body 30 s. Durability nick 2% of a mend cycle.
- Bounty 40 + 10 per 100 infamy, cap 120. Emission cap 2,000 Reserve per server per week. Claim lockout 24 h.
- War window 48 h. Score hold term: 1 per minute. Vault wave 12 min. Way In: one branch. Map count 5. Instance size targets 2×100 and 2×250.
- Portals 12, minimum 4 landings, fee 0 under proficiency 12, then 1 per 10 proficiency, cap 8, move cooldown 6 h, old beacon 30 min.
- Battlegrounds 15 / 20 / 12 min. Lane Trial 25 min, wave 30 s, match proficiency 1–12, last hit 20, deny 2, beacon 40, player-kill charge 30.
- Duel: within 5 proficiency, 3 min, offer escrow 60 s, reward lockout 30 days. 5v5 spread 10.
- Scrimmage: 15 min, 3 starts, 1 payout per day. Brackets of 10 from 1 to 60.
- Tells 0.35 s minimum. Basic strike 8 damage and 0.2 s. Jump 0.4 s air, 0.15 s land, 1.2 m. Control diminish 8 s.
- Kit damage figures in §14.2. Raid pool 4,000. Place hits 15 / 25 / 35.
- Surge drain 8 per 10 s, regen 2 per 10 s, speed +8%, crash 20 s at −8% after a peak of 60. Tolerance 100% / 70% / 40% / 0 inside 10 min. Quality use-time ±20%. Faction Surge +5.
- Use times: 1.5, 2.0, 3.0, 4.0, 1.0, 1.0, 1.2 seconds. Medkit heal 60, half on the early repeat, cooldown 60 s.
- AFK disconnect 30 min. War reconnect grace 3 min. Toast rank-1 hold 8 s.
- Standard kit counts: 2, 2, 1, 1, 1, 1.

---

## 23. Cite

`docs/LAYERED_GAMES_AND_SERVER.md` · `docs/design/UI_LAYOUT_SYSTEM.md` · `docs/FACTIONS_OVERVIEW.md` · `docs/FACTION_HERALDRY.md` · `docs/MERCY_TEMPER_PROGRESSION.md` · `docs/REDEMPTION_MECHANICS_PER_SPECIES.md` · `docs/HUMAN_PLAYABILITY_v23.2_SPECIES_REDEMPTION.md` · `docs/DISCORDANT_REDEMPTION_QUESTLINES.md` · `docs/SPECIFIC_REDEMPTION_QUESTS.md` · `docs/GDD_IMMERSION_REVISION.md` · `docs/HUMAN_PLAYABILITY_v23.2_DECLARED_LETHAL.md` · `docs/HUMAN_PLAYABILITY_v23.2_WAR_WEEK.md` · `docs/NPC_SCHEDULE_SPEC.md` · `docs/PRE_RELEASE_LAW.md` · `docs/COUNCIL_SYSTEM.md` · `docs/ONLINE_LADDER.md` · `docs/OFFLINE_ECONOMY_COURT.md` · `docs/GODSPEED_PREP.md` · `docs/PLAYABLE_RACES.md` · `docs/PLACES_BIBLE.md` · `docs/DOGFIGHT_MECHANICS.md` · `docs/BOARDING_MECHANICS.md` · `docs/FLEET_CLASSES.md` · `docs/DRAEK_FLEET_AI_SYSTEMS.md` · `shared/war_week.rs` (cite only)

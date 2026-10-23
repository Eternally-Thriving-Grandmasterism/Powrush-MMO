# R&D: Classic Gameplay Lessons for Powrush-MMO

Status: **APPROVED BY SHERIF.** The 10 ranked items and F1–F6 are approved. Sherif Botros, 2026-10-23 3:55 AM ET, verbatim: "Approved."
Each build card still has to pass the council gates, including a Core vote wherever one is flagged. Items marked "considered and set aside" stay dropped.
Scope: docs only. No code, assets or schema changes.

## The ask (Sherif Botros, 2026-10-23 3:50 AM ET, verbatim)

> "Tons of ancient games from Accolade and similar had amazing gameplay which most people never got to experience. Let's conduct R&D to enhance the Powrush-MMO gaming overall experience, because it'll offer something most people never had or may be nostalgic for if they did, and also, is so different than current games which seem like slop, Mate, all thoughtfully, wisely and creatively with Rathor.ai full TOLCing, properly and respectively!"

## Design principle (Sherif's ruling)

Sherif Botros, 2026-10-23 3:53 AM ET, verbatim:

> "When designing after research, aim for maximal fun, and transferable skills as well as useful ideas to enhance end-users overall better problem solving skills and useful techniques to properly apply on reality with finesse to thrive with positive emotions maximally overall, Mates!"

How we apply it: every shortlisted item has a **Fun** line and a **Real-world skill** line. The skill lines are **design intent only**. We make no claim of proven learning outcomes; any such claim would need real playtesting and evidence first.

Sherif Botros, 2026-10-23 3:54 AM ET, verbatim:

> "Also include fun without education, Lmao, Mates!"

So the real-world skill lines are **optional, never required**. A feature can exist purely because it is fun. See "Just for fun (no lesson)" below.

## Ground rules

- **Inspiration only.** We take design ideas, never names, assets, code, music, UI layouts or trademarks. Every adopted mechanic gets a Powrush-native name and look (the painted powrush.com style).
- **TOLC filter** (truth, mercy/peace-compatible, respect for players, no slop): no loot boxes, no FOMO timers, no pay-to-win, no endless grind, no manipulative streaks. Mechanics must work **offline-by-default** in Bevy 0.20.
- **Canon fit:** the calm first hour (Sanctuary, Heartwood, Threshold, Depths), the harvest/craft/ledger economy, the D2-style procgen with Peace and Combat zones (`docs/PROCGEN_DESIGN.md`, PR #757), the Draek Brood Spire (TBS) and Quellorian TAUN orbital hub towns, and `docs/CHARACTER_IDENTITY.md`. This doc also follows the rulings on no real-world flags and no AI-generated crests.
- **Facts:** we only list facts we confirmed, and each one has a cited source. We mark anything we didn't confirm as **UNKNOWN**.

## 1. Research: what made these games special

### Accolade (San Jose, 1984–2000)

Source for everything in this table: https://en.wikipedia.org/wiki/Accolade,_Inc. (it cites Retro Gamer #59, *High Score!*, and CGW #88).

| Game | Accolade's role | The mechanic that made it special |
|---|---|---|
| Law of the West (1985) | Developed (Alan Miller) | It mixed gunfights with adventure play and helped pioneer **choosing your dialogue lines**. |
| HardBall! (1985) | Developed (Bob Whitehead) | It used the TV-style "behind the pitcher" camera, and it added player data and a **coach mode** (management alongside play). |
| Test Drive (1987) | Published (developed by Distinctive Software) | You drive **exotic cars with a risk of being chased by police**: tension from freedom plus consequence. |
| Mean 18 / Jack Nicklaus series (1986–) | Published (Rex Bradford) | It pioneered the **"three-click" timing swing** (backswing, downswing, follow-through), which is easy to learn and has a deep skill ceiling. |
| Star Control (1990) / Star Control II (1992) | Published (Paul Reiche III and Fred Ford) | Award-winning and repeatedly ranked among the best games of all time. Design details are in the next table. |
| Bubsy (1993) | Published (Mike Berlyn) | Mascot platformer. It sold well, but we found no standout mechanic worth adopting. |
| Ace of Aces (1986) | Published (Artech) | A $80k budget that sold 500k copies, a reminder that **scope discipline** beats spectacle. |
| Elvira: Mistress of the Dark | **UNKNOWN**: we did not confirm Accolade's role. | — |
| Grand Prix Circuit | **UNKNOWN**: we did not confirm this source. | — |

Aside: Accolade dropped a planned MMO to focus on consoles (same source).

### Star Control II, in more depth

Source: https://en.wikipedia.org/wiki/Star_Control_II

- It has a **non-linear open star map** where you learn what matters by **talking to alien races**, each with its own voice and its own politics.
- It runs on a **world clock**: events advance whether or not the player acts. We list this so the councils can weigh it; we do **not** recommend copying the pressure.
- You **gather resources to upgrade your flagship**, and the ship is your home base.
- The **melee mode** offers short, readable ship-vs-ship duels with asymmetric ships.

### Similar studios of the era

| Studio / game | Mechanic | Source |
|---|---|---|
| Origin: Ultima IV (1985) | The goal is **becoming an Avatar of eight virtues**, not killing a final boss. The game watches how you behave (honesty, compassion, and so on), and character creation is a **moral questionnaire** made of dilemmas. | https://en.wikipedia.org/wiki/Ultima_IV:_Quest_of_the_Avatar |
| Origin: Ultima Online (1997) | **Player-run economy and housing**, with skills that improve through use. | https://en.wikipedia.org/wiki/Ultima_Online |
| Blizzard North: Diablo (1996) / Diablo II (2000) | **Procedurally generated dungeons** around a **safe town hub**, and readable item tiers. | https://en.wikipedia.org/wiki/Diablo_(video_game) |
| MicroProse: Sid Meier's Pirates! (1987) | An **open sandbox career** made of linked minigames (sailing, fencing, trading), and your **reputation with several nations** changes how the world treats you. | https://en.wikipedia.org/wiki/Sid_Meier%27s_Pirates! |
| MicroProse: Civilization (1991) | You win through **multiple paths** (conquest or the space race), and the tech tree is the "one more turn" engine. | https://en.wikipedia.org/wiki/Civilization_(video_game) |
| Bullfrog: Populous (1989) | You play a "god game": you **shape the terrain** to help your followers rather than steering units directly. | https://en.wikipedia.org/wiki/Populous_(video_game) |
| Bullfrog: Theme Park (1994) | A **management sim** with visible satisfaction for visitors. | https://en.wikipedia.org/wiki/Theme_Park_(video_game) |
| Westwood: Dune II (1992) | It set the RTS template: **harvest a resource, then build**. | https://en.wikipedia.org/wiki/Dune_II |
| Sierra: King's Quest (1984–) | Illustrated adventure with **parser and point-and-click interaction with the world**. Details of individual mechanics are **UNKNOWN** here because we didn't check them. | https://en.wikipedia.org/wiki/King%27s_Quest |
| SSI: Gold Box series (1988–) | **Faithful tabletop rules** turned into turn-based tactical combat. | https://en.wikipedia.org/wiki/Gold_Box |

## 2. Council deliberation (PATSAGi, TOLC lens)

- **Hands (build feasibility, Bevy 0.20, offline):** favours mechanics that are data-driven (RON/JSON tables, ECS components) and need no server. It rejects anything that needs live ops or an online economy before networking exists.
- **Clerk (canon fit):** favours mechanics that deepen what we already have (ledger economy, Places, identity, Peace/Combat zones, orbital hubs). It flags anything that would add combat into the calm first hour.
- **Core (runtime cost):** "Core vote needed" means the item touches the save format, the simulation tick, or the asset pipeline budget.
- **TOLC exclusions:** We dropped Star Control II's doom clock as a pressure mechanic (FOMO-adjacent). We also dropped loot-box-style randomized rewards, timed exclusives and grind-gated progress. The world clock only survives as an *optional, gentle* living-world idea.

## 3. Ranked shortlist: approved by Sherif ("Approved.", 2026-10-23 3:55 AM ET)

| # | Mechanic (Powrush-native working name) | Inspiration | Maps to Powrush | Effort | Core vote? | Risk |
|---|---|---|---|---|---|---|
| 1 | **Virtue Path**: the world notices how you act | Ultima IV virtues and moral questionnaire | The ledger records mercy, honesty and generosity acts (sharing harvest, helping NPCs). NPCs and Places respond. `CHARACTER_IDENTITY.md` gets an optional set of dilemma questions in the creator that seed your backstory and starting loyalties. No karma meter shaming. | M | Yes (save schema) | Preachiness; it must stay subtle. |
| 2 | **Voices of the Spire and TAUN**: dialogue-driven discovery | Star Control II alien conversations; Law of the West dialogue choices | Draek and Quellorian hubs become places you **learn** from. Each faction has a distinct voice, and its lore unlocks routes and recipes. Branching dialogue is written as data. | M | No | Writing volume; needs a consistent tone. |
| 3 | **Hearth hub and wild descent** | Diablo/D2 town and procgen dungeon | Already ruled (PR #757). Sanctuary and the orbital hubs are Peace zones, and procgen Depths are optional Combat zones. This doc just reinforces the pattern: always a safe return. | L (already scoped) | Yes (already in #757) | Scope creep. |
| 4 | **Three-beat craft timing**: optional skill input for crafting and harvesting | Mean 18 / Jack Nicklaus three-click swing | A tap-hold-release rhythm can improve quality, and it can always be skipped for a fair baseline result, so it stays accessible. | S | No | Accessibility; must be optional. |
| 5 | **Standing with Houses**: multi-faction reputation | Pirates! reputation with nations | Draek, Quellorian and Place communities each track standing from trade and deeds. Prices and doors change. No real-world flags; heraldry is hand-painted only. | M | Yes (save) | Balancing. |
| 6 | **Shape the land, not the people** | Populous | In Heartwood you can tend terrain (water, groves) to help NPC settlers flourish. It's peaceful god-game play with no combat. | M | Yes (terrain/sim tick) | Simulation cost. |
| 7 | **Steward mode**: watch-and-manage | HardBall! coach mode; Theme Park satisfaction | You can take over a stall or workshop in a hub town and set prices or production, with visible visitor satisfaction. Offline NPC customers. | M | No | UI complexity. |
| 8 | **Your vessel is your home**: an upgradable personal craft | Star Control II flagship upgrades | A personal skiff or chamber docked at the Spire or TAUN, upgraded with crafted parts. It's a cosmetic and utility home, not a power ladder. | M | Yes (save/assets) | Becoming a grind treadmill; cap the upgrades. |
| 9 | **Many ways to thrive**: plural endgame goals | Civilization's multiple victory paths | Recognise mastery in craft, cartography, diplomacy, stewardship or (later) combat. No single "best" path. | S (docs/design) | No | It needs the other systems first. |
| 10 | **Gentle living world**: the world turns while you rest | Star Control II world clock (softened) | Seasons and NPC routines advance in game time, never punishing absence and never using missed-event FOMO. | M | Yes (sim tick) | Drifting toward FOMO; a TOLC review is required. |

### Fun and real-world skill per item (design intent, not proven outcomes)

1. **Virtue Path.** *Fun:* the world noticing your kindness feels personal and surprising. *Real-world skill:* empathy and ethical reasoning, from weighing dilemmas where every choice has a cost.
2. **Voices of the Spire and TAUN.** *Fun:* the detective joy of piecing together hints from strange, funny characters. *Real-world skill:* active listening and inference, from reading between the lines and checking a claim against a second source.
3. **Hearth hub and wild descent.** *Fun:* the risk-and-reward thrill of venturing out and coming home safe. *Real-world skill:* planning and risk assessment, from packing, choosing when to turn back and judging what you can handle.
4. **Three-beat craft timing.** *Fun:* the satisfying "click" of nailing a rhythm. *Real-world skill:* rhythm, focus and calm under light pressure, plus the habit of practising for steady improvement.
5. **Standing with Houses.** *Fun:* doors opening because people trust you. *Real-world skill:* negotiation and reputation management, from fair dealing, keeping promises and seeing other parties' interests.
6. **Shape the land, not the people.** *Fun:* the cozy pleasure of watching a grove and its settlers flourish. *Real-world skill:* systems thinking, from seeing how small changes ripple outward (water, growth, community).
7. **Steward mode.** *Fun:* running your own little shop and watching it hum. *Real-world skill:* basic budgeting, pricing and reading customer feedback.
8. **Your vessel is your home.** *Fun:* making a space that is truly yours. *Real-world skill:* goal-setting and resource planning, from saving and choosing what to build first.
9. **Many ways to thrive.** *Fun:* the freedom to master what you love. *Real-world skill:* self-knowledge, from finding the strengths you want to grow and seeing that there are many valid ways to succeed.
10. **Gentle living world.** *Fun:* coming back to a world that moved on in small, charming ways. *Real-world skill:* patience and noticing change over time, with no pressure to be always on.

Considered and set aside for now: Test Drive's police chase (it needs vehicles and combat-adjacent pressure) and Ace of Aces/Star Control melee duels (deferred until combat design exists, then strong candidates for readable 1v1 duels in Combat zones).

## Just for fun (no lesson): approved by Sherif ("Approved.", 2026-10-23 3:55 AM ET)

These are pure play, with no skill goal attached. TOLC still applies: everything is earned in play or free, with no loot boxes, no FOMO timers, no pay-to-win and no copied names or assets.

| # | Idea | Inspiration (verified above) | Powrush mapping | Effort | Core vote? |
|---|---|---|---|---|---|
| F1 | **Skiff rally**: time-trial races, with ghost replays of your own best run | Test Drive (Accolade) racing | Hover-skiff circuits around the Brood Spire and TAUN hull, with offline ghosts saved locally. | M | Yes (save, physics) |
| F2 | **Silly-physics stunts**: bouncy mushrooms, gust vents, ragdoll tumbles | Accolade-era arcade play (general inspiration) | Heartwood bounce-caps and Threshold wind-vents in Peace zones. You can't take damage, only laugh. | S | No |
| F3 | **Secret rooms and easter eggs**: hidden nooks with jokes and painted vignettes | Era tradition of hidden content (general inspiration; no specific game claimed) | Hand-placed secrets in all 4 Places and both hubs, logged in a "Curiosities" ledger page. | S | No |
| F4 | **Emotes and dances**: free gestures, group dances, sit by the fire | MMO social play, e.g. Ultima Online (inspiration only) | Sanctuary campfire circle, plus Draek and Quellorian dance styles. All are free to unlock in play. | S | Yes (animation assets) |
| F5 | **Little companions**: a harmless critter that follows you and reacts | Original Powrush idea (no direct classic source; flagged honestly) | Befriend a Depths glow-moth or a Heartwood burrower with food. It does nothing useful and is just cute. Expanded in [`COMPANIONS_DESIGN.md`](COMPANIONS_DESIGN.md) (COMP-0, pending Sherif's OK). | M | Yes (save, AI) |
| F6 | **Goofy hats and Hub minigames**: crafted silly hats, plus a timing-swing "skystone toss" golf-like minigame | Mean 18 / Jack Nicklaus three-click swing (Accolade) | Crafted hats with painted art (no AI crests, no flags). Skystone toss courses on the orbital hubs' outer decks. | M | No |

## Related designs

- Companions: [`COMPANIONS_DESIGN.md`](COMPANIONS_DESIGN.md) (COMP-0, pending Sherif's OK).
- Collectible duel card game: [`CARD_GAME_DESIGN.md`](CARD_GAME_DESIGN.md) (CARD-0, pending Sherif's OK).

## 4. Next steps

1. Sherif approved the lists ("Approved.", 2026-10-23 3:55 AM ET). Each build card still has to pass the council gates.
2. Clerk writes per-item design stubs in `docs/`.
3. Core votes on the save, tick and asset items (1, 3, 5, 6, 8, 10).
4. Hands prototypes item 4 (S effort) first as a low-risk test.

# CARD-0: In-World Collectible Duel Card Game

Status: **DRAFT. Council recommendation, pending Sherif's OK.** Nothing here has been voted on.
Inspiration only: no names, card text, art, rules text, code or trademarks are copied from any game.

## The ask (Sherif Botros, 2026-10-23 3:58 AM ET, verbatim)

> "Is it practical to add trading cards as loot in the game with a payable hearthstone like card game in the Powrush-MMO universe similar to the poker and dice games in conquer online?"

How we read it: we took **"payable" as "playable"**. Whether there should be a *paid* version is an **open question for Sherif**. Payments are parked, and the council's default is that nothing is paid.

## Council answer: yes, it's practical

We recommend an in-world collectible duel card game, played at tables in the hub towns (the Draek Brood Spire and the Quellorian TAUN) and in Sanctuary.

## Card cap and skill-first rule (Sherif's ruling)

Sherif Botros, 2026-10-23 4:00 AM ET, verbatim:

> "Let's make a card limit so we don't have feature creep like hearthstone too many cards and league of legends too many heroes to have a real meta to allow skilled players near 100% win like Starcraft 2 fairplay instead of praying to RNGeezus for luck, Lmao, Mates!"

Council design. The 64 cap and the 8×8 structure are Sherif's rulings. The family split is a **council proposal** pending Sherif's OK.

### Hard pool cap

- **Hard cap: 64 cards in total, and the game can never hold more.** This is Sherif's ruling, 2026-10-23 4:04 AM ET, verbatim: "64 (8 families of 8)".
- **Structure (ruled): 8 families of 8 cards each.**
- **Family list (council proposal, pending Sherif's OK). Chosen option, 2 + 2 + 4:**
  - **Draek:** 2 families (16 cards), the Brood and the Spire-forge.
  - **Quellorian:** 2 families (16 cards), the TAUN Stewards and the Lightwardens.
  - **Places:** 4 families (32 cards), one each for Sanctuary, Heartwood, Threshold and Depths.
  - Neutral cards are folded into the Place families. For example, companion allies live where they belong: the glow-moth in Depths and the burrower in Heartwood.
  - *Why this option:* it maps one-to-one onto the 4 hand-made Places players already know, gives both peoples equal weight, and needs no separate "neutral" category to learn.
- **Alternative (not chosen), 2 + 2 + 3 + 1:** 2 Draek, 2 Quellorian, 3 Place families, and 1 Neutral family for companions and wanderers. Its downside is that one Place would have to share a family or be left out.
- **Why 64 and families of 8:**
  - Working memory is usually estimated at about 4 chunks (Cowan 2001, https://doi.org/10.1017/S0140525X01003922) up to 7±2 (Miller 1956, https://doi.org/10.1037/h0043158). Families of 8 cards, in 8 families, keep the whole pool learnable as chunks.
  - By contrast, Hearthstone launched with about 378 collectible cards (133 Basic + 245 Expert), per https://liquipedia.net/hearthstone/Card_Set (we didn't re-check the figure this pass), and it has grown much larger since. League of Legends has well over 150 champions (**UNKNOWN**: we didn't verify the exact current count).
- **Replace, never add.** A new card enters only by replacing an existing card in the same slot. The total is fixed at 64 (each family stays at 8), and the replaced card goes into an archive so it can still be played casually.
- **No power creep.** A replacement must pass the same balance tests as the card it replaces.

### Skill-first, low-RNG rules

- **Open information:** both players' decks are public lists, and there's a shared visible card pool.
- **Draws are fixed or chosen by the player.** Each player arranges their deck order before the match (or chooses each draw from their next three cards). There is no random shuffle in ranked play.
- **No random effects:** no random damage, no random targets and no "discover a random card".
- **Seeded randomness only where it can't be avoided:** for example, who goes first. Even then it is shared, visible and announced ahead (a pre-match seed both players see), and the second player gets a small compensation.
- **Mirror-match fairness:**
  - Ranked play offers a mode where both players use the same faction or deck.
  - Automated balance tests run simulated AI matches, and every faction's win rate must stay within a set band.
  - Faction-vs-faction matchups are tracked in CI.

### Ranked skill, offline first

- Ranked skill is measured first against AI tiers, offline (for example Novice, Adept, Master and Grandmaster). Each tier is a fixed, documented AI.
- A skill rating is kept locally. Player-vs-player ladders come after networking.

### Honesty note

"Skilled players near 100% win" is a **design goal, not a guarantee**. Fixed draws and open information shrink luck a lot, but in any game against a thinking opponent the outcome stays uncertain. StarCraft II is cited **only as inspiration** for the feeling that skill decides the match. We copy nothing from it.

## 1. Research (cited; **UNKNOWN** where we couldn't confirm)

- **Hearthstone** (Blizzard Entertainment, 2014), https://en.wikipedia.org/wiki/Hearthstone:
  - Free-to-play digital collectible card game with **30-card decks** and a **mana crystal** resource that grows by one each turn, up to 10.
  - Cards come from **packs bought with in-game gold or real money**.
  - You can **disenchant** cards into "dust" and **craft** specific cards from it.
  - Common player criticisms are cost to compete and a rotating meta. **UNKNOWN**: we didn't gather specific sources this pass.
  - Belgium and Netherlands loot-box rulings: whether Hearthstone itself was affected is **UNKNOWN**, so check before citing. General background: https://en.wikipedia.org/wiki/Loot_box.
- **Conquer Online** (TQ Digital, 2003):
  - Players widely report in-game **gambling-style minigames (poker, dice) played with in-game currency**.
  - We couldn't confirm specifics this pass, because the Wikipedia page fetch returned 404. Rules, stakes and dates are **UNKNOWN**.
- **Loot-box and gambling context:** several jurisdictions treat paid random packs, and wagering where the winnings can be traded for value, as gambling or near-gambling. The details vary and are **UNKNOWN** per jurisdiction, so we need a legal check before any paid feature. Source: https://en.wikipedia.org/wiki/Loot_box.

## 2. Council deliberation (PATSAGi, TOLC)

- **Clerk:** It fits the canon. Cards are painted lore objects: you learn the world by collecting it. There are three faction suits:
  - **Draek** (brood, pride, fire-forged).
  - **Quellorian** (TAUN stewards, light, order).
  - **Places** (Sanctuary, Heartwood, Threshold and Depths as neutral terrain and spirit cards).
  - Companion cards (COMP-0) appear as gentle allies.
  - No real-world flags and no AI crests; card art is hand-painted.
- **Hands:** It's a self-contained ruleset in Rust as pure logic, which is easy to test and needs no network. Cards are RON data, and the AI opponent is a simple heuristic or search. The UI is a Bevy table scene.
- **Core:** Runtime cost is small. A Core vote is needed for the save (collection) and the card-art asset budget.
- **TOLC hard rules:**
  - **No real-money packs.**
  - **No wagering of in-game currency.** Conquer Online's poker and dice are deliberately not copied, because of the risk to minors and of gambling habits. Instead, stakes are **cosmetic trophies only** (table titles, card backs) that can't be traded for value.
  - **A small shared core set** that every player can build from.
  - **No rarity pay ladder.** Rarity only changes the art (painted foil), never the power.
  - No FOMO rotation and no timed sets.

## 3. CARD-0 design

1. **How you get cards:** as loot from zones and set-pieces (deterministic first-clear drops plus visible drop tables), as gifts from companions, from **crafting** cards at a scribe's table using harvested inks and pigments (this ties into the ledger economy), and from quests. No random paid packs.
2. **Play:** short duels (5–10 minutes) against AI opponents at hub tables first, offline. PvP comes later, once networking exists. You can use pre-built starter decks or build your own.
3. **Trading:** uses the same safe-trade path as companions: offline gift letters now, two-sided escrow with a preview later.
4. **Fairness:** the shared core set within the 64-card cap means any deck can compete. There's an optional "table draft" mode where everyone builds from the same pool.
5. **Fun:** table banter from NPC players, painted animated cards, and a hub tournament ladder that awards trophies only.
6. **Optional real-world skills** (design intent only, no proven outcomes claimed): probability, planning ahead, and resource management.

## 4. Card plan (each card still goes through the council gates)

| Card | Scope | Effort | Core vote? |
|---|---|---|---|
| CARD-1 | Rules engine (pure Rust, tests) + RON card schema | M | No |
| CARD-2 | Core set: hard cap of 64, 8 families × 8 (Sherif's ruling); families 2 Draek + 2 Quellorian + 4 Places (council proposal) | M | Yes (art assets, cap) |
| CARD-2b | Fixed or player-chosen draws, no random effects, visible pre-match seed | S | No |
| CARD-2c | Mirror mode + automated AI balance tests and win-rate band in CI | M | No |
| CARD-2d | Offline ranked AI tiers + local skill rating | M | Yes (save) |
| CARD-3 | AI opponent + hub table scene | M | No |
| CARD-4 | Collection in the save + loot/craft sources | M | Yes (save) |
| CARD-5 | Scribe crafting (ledger tie-in) | S | No |
| CARD-6 | Gift-letter trading (shared with COMP-6) | S | Yes (save/signing) |
| CARD-7 | Trophy tournaments (cosmetic) | S | No |
| CARD-8 | PvP (post-networking) | L | Yes |

## 5. Open questions for Sherif

1. Paid version: yes or no? The default is no, with payments parked.
2. Is cosmetic-only staking at tables acceptable?

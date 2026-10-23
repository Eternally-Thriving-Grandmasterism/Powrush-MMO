# CARD-0: In-World Collectible Duel Card Game

Status: **DRAFT. Council recommendation, pending Sherif's OK.** Nothing here has been voted on.
Inspiration only: no names, card text, art, rules text, code or trademarks are copied from any game.

## The ask (Sherif Botros, 2026-10-23 3:58 AM ET, verbatim)

> "Is it practical to add trading cards as loot in the game with a payable hearthstone like card game in the Powrush-MMO universe similar to the poker and dice games in conquer online?"

How we read it: we took **"payable" as "playable"**. Whether there should be a *paid* version is an **open question for Sherif**. Payments are parked, and the council's default is that nothing is paid.

## Council answer: yes, it's practical

We recommend an in-world collectible duel card game, played at tables in the hub towns (the Draek Brood Spire and the Quellorian TAUN) and in Sanctuary.

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
4. **Fairness:** the shared core set means any deck can compete. There's an optional "table draft" mode where everyone builds from the same pool.
5. **Fun:** table banter from NPC players, painted animated cards, and a hub tournament ladder that awards trophies only.
6. **Optional real-world skills** (design intent only, no proven outcomes claimed): probability, planning ahead, and resource management.

## 4. Card plan (each card still goes through the council gates)

| Card | Scope | Effort | Core vote? |
|---|---|---|---|
| CARD-1 | Rules engine (pure Rust, tests) + RON card schema | M | No |
| CARD-2 | Core set: ~40 cards across Draek, Quellorian and Places | M | Yes (art assets) |
| CARD-3 | AI opponent + hub table scene | M | No |
| CARD-4 | Collection in the save + loot/craft sources | M | Yes (save) |
| CARD-5 | Scribe crafting (ledger tie-in) | S | No |
| CARD-6 | Gift-letter trading (shared with COMP-6) | S | Yes (save/signing) |
| CARD-7 | Trophy tournaments (cosmetic) | S | No |
| CARD-8 | PvP (post-networking) | L | Yes |

## 5. Open questions for Sherif

1. Paid version: yes or no? The default is no, with payments parked.
2. Is cosmetic-only staking at tables acceptable?

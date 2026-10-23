# COMP-0: Companions Design

Status: **DRAFT. Council recommendation, pending Sherif's OK.** Nothing here has been voted on. This expands **F5 Little companions** in `docs/RND_CLASSIC_GAMEPLAY.md`.
Inspiration only: no names, creatures, assets, code or trademarks are copied from any game.

## The ask (Sherif Botros, 2026-10-23 3:57 AM ET, verbatim)

> "Companions should have trading, training, breeding, etc., but a way better system than ARK survival and Pokémon please, Mates!"

The design rules from earlier rulings still apply: aim for maximal fun; real-world skill lines are optional and never required; pure fun is welcome; TOLC (mercy, no dark patterns, no loot boxes, FOMO or pay-to-win); offline-by-default; Bevy 0.20.

## 1. Research (cited; **UNKNOWN** where we couldn't confirm)

How we sourced this: we read the ARK imprinting page in full while writing this. The other claims come from the pages cited and from well-documented community knowledge, and we did not re-read each of those pages this pass. Anything marked UNKNOWN is uncertain and shouldn't be cited as fact.

### ARK: Survival Evolved / Ascended

- **Imprinting** (https://ark.wiki.gg/wiki/Imprinting, read in full):
  - A baby asks for care (a random food item, a cuddle, or a walk) on a timer that is **about 8 hours on official settings**.
  - Full imprint gives **+20% to most stats**, plus **+30% damage and resistance only when the person who raised it is riding it**.
  - A large creature can need many sessions. A Rex is about 9% per session, so about 11 sessions spread across its maturation.
  - The food requested can be random and unrelated to what the creature eats (a carnivore asking for berries).
  - Not caring has no penalty beyond losing the bonus.
- **Taming** (https://ark.wiki.gg/wiki/Taming): the classic method is **knocking the creature unconscious** (raising its torpor) and then feeding it while it's down. Some creatures use passive feeding instead.
- **Breeding and mutations** (https://ark.wiki.gg/wiki/Breeding): offspring inherit stats from the parents, plus a small random chance of mutations. Players commonly run mutation-stacking programmes. The exact rates are **UNKNOWN** here (we didn't check them this pass).
- **Common player complaints** (community knowledge, exact sources not gathered): staying online for imprint timers ("babysitting"), long maturation waits, mutation-stacking grind, and the cruelty feel of knock-out taming. **UNKNOWN**: we have no survey data on how common each complaint is.

### Pokémon (main series)

- **IVs** (individual values) are hidden per-stat values fixed when the creature is generated. **EVs** (effort values) are hidden points earned from defeating specific species, and they raise stats. Sources: https://bulbapedia.bulbagarden.net/wiki/Individual_values and https://bulbapedia.bulbagarden.net/wiki/Effort_values (the fetch timed out this pass, so these facts are from well-documented community knowledge).
- **Breeding** (https://bulbapedia.bulbagarden.net/wiki/Pok%C3%A9mon_breeding): eggs are hatched by walking, and players chain-breed and hatch many eggs to get good IVs and natures.
- **Trading:** some species only evolve when traded, which pushes players to find a trade partner. Trade scams are a long-running community concern. **UNKNOWN**: we didn't check per-generation safeguards.
- **Complaints** (community knowledge): hidden stat grinding, egg-hatching grind, version exclusives and trade-gated content, and **power creep** between generations. **UNKNOWN**: we have no quantified data.

### Other strong companion systems

| Game | Standout mechanic | Source |
|---|---|---|
| Monster Rancher (Tecmo, 1997) | Monsters are **generated from music CDs**. You raise them through a training-and-rest schedule, and **stress and fatigue** visibly affect them. | https://en.wikipedia.org/wiki/Monster_Rancher |
| Creatures (1996) | The "Norns" have **simulated neural networks and biochemistry**. You teach them with words and gentle encouragement or discouragement, and they breed with **genetic inheritance**. | https://en.wikipedia.org/wiki/Creatures_(video_game_series) |
| Digimon World (1999) | It is about **raising a creature**: it has care needs, and how you train and care for it decides how it evolves. Details are **UNKNOWN** beyond that. | https://en.wikipedia.org/wiki/Digimon_World_(video_game) |
| Palworld (Pocketpair, 2024) | Creatures **work at your base** and fight alongside you. Its capture-and-work design drew criticism over the creatures' welfare. | https://en.wikipedia.org/wiki/Palworld |

## 2. Council deliberation (PATSAGi, TOLC lens)

- **Clerk (canon):** Companions are **friends, never property you capture**. The Depths glow-moth and Heartwood burrower are the starting kin. Each culture has its own way with companions: Draek brood-kin traditions in the Spire, and Quellorian TAUN steward-keepers. These are invented here and need a lore pass.
- **Hands (Bevy, offline):** Companions are ECS entities with data-driven species tables (RON). Bond, traits and lineage live in the save. Trading works as an offline-signed gift file now, with escrow added once networking exists.
- **Core (runtime):** Following AI is cheap, with one active companion and a few at home. The save schema grows (lineage graph), so a **Core vote is needed**.
- **TOLC hard rules:**
  - No knock-outs, cages, starving or forced labour.
  - No permanent death.
  - No real-time timers that require you to be present.
  - No hidden-stat RNG to grind.
  - No paid eggs or gacha.
  - No species that can only be had by trading.
  - No power creep. Companions are side-grades.

## 3. COMP-0 design

1. **Befriending, not taming.** Each species has a readable *trust ritual*. A glow-moth comes to lantern light patterns you make. A burrower comes to food you leave and to you sitting quietly nearby. The creature chooses, and you can retry freely with no failure penalty.
2. **Bond through doing together.** Bond grows from shared harvesting, exploring, play and rest. Skills are trained by practising them together (fetch, dig, glow-signal), never by grinding fights. No hidden stats; everything is shown on a bond page.
3. **Gentle breeding with readable inheritance.**
   - Two bonded companions who are friends can raise a young one at a nest in a Peace zone.
   - The **inheritance preview** shows each visible trait (colour, pattern, temperament, talent) with its source parent and plain odds, Punnett-square style.
   - Occasional surprise sparks are cosmetic only.
   - Growth advances in in-game time; there are no real-time care timers and no penalty for being away.
4. **Safe trading and gifting.**
   - Offline: a signed *gift letter* passes a companion on only with its consent (its bond must be high enough, and it remembers you).
   - Online, later: two-sided escrow with a full preview and confirmation from both parties.
   - Nothing is trade-locked, and there are no real-money sales.
5. **Peace vs Combat roles.**
   - Peace: helper (finds nodes, carries a small pack, lights paths) and play.
   - Combat: support (distract, shield, scout). If badly hurt, the companion **retreats to rest at home**. **It never dies.** You can always leave it home.
6. **Welfare.**
   - Mood is visible. Overwork makes companions tired, so they rest.
   - Release is a farewell ceremony, and the companion may visit later.
   - Neglect never causes harm, only a calm "missed you".
7. **Cosmetics.** Painted collars, lanterns and ribbons are crafted or found, never sold in loot boxes. No AI crests and no flags.
8. **Pure fun.** Tricks, emotes, tiny races (this ties to F1), hide-and-seek in Heartwood, and photo poses.

Optional real-world skill lines (design intent only, no proven learning outcomes are claimed):
- Care and patience, from trust rituals.
- Genetics basics, from reading inheritance.
- Negotiation and fair dealing, from trading.
- Empathy, from reading moods.

## 4. Better than ARK and Pokémon

| Pain point | ARK | Pokémon | COMP-0 |
|---|---|---|---|
| Acquisition | Knock out, then feed | Weaken in battle, then capture | Trust ritual; the creature chooses |
| Care timers | ~8h real-time imprint requests | Egg hatching by walking steps | In-game time only; no presence needed |
| Stats | Mutation stacking; random care requests | Hidden IVs/EVs to grind | Visible traits; previewed odds |
| Death | Creatures can die | Faint | Retreat and rest; never permanent death |
| Trading | Trust risk | Trade-evolutions; scams | Consent-based gift letters; escrow later; nothing trade-locked |
| Power | Rider bonuses make creatures decisive | Power creep between generations | Capped side-grades |
| Money | — | — | Never sold, never gacha |

## 5. Card plan (each card still goes through the council gates)

| Card | Scope | Effort | Core vote? |
|---|---|---|---|
| COMP-1 | Species data (glow-moth, burrower) + follow AI | M | Yes (save) |
| COMP-2 | Trust rituals | M | No |
| COMP-3 | Bond page + train-by-doing | M | Yes (save) |
| COMP-4 | Peace helper roles | S | No |
| COMP-5 | Nest + inheritance preview + lineage | L | Yes (save schema) |
| COMP-6 | Offline gift letters | M | Yes (save/signing) |
| COMP-7 | Combat support + retreat-to-rest | M | Yes (sim) |
| COMP-8 | Welfare/mood | S | No |
| COMP-9 | Cosmetics + tricks | S | Yes (assets) |
| COMP-10 | Online escrow trading (post-networking) | L | Yes |

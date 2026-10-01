# NAME RITE — offline, after the hour

**CARD CLERK-GODSPEED-PREP.** Tip `2a7e90c0`. Design tick, not a Cargo bump.

**Contact:** [info@Rathor.ai](mailto:info@Rathor.ai).

The house may offer a name after Hour 1 has been walked. It must not block Title → Play → walk → well.

Online grey. Peak memory: walked · tended · week was the bill · yard remembered.

## Law

- Optional. Refuse keeps the sovereign line: this hour is yours alone.
- Local word list in repo. No fetch. No model. `ai_assist_used` stays false.
- Same machine, same seed, same offer, until they refuse or accept.
- Accept, refuse, or type. No race, class, guild, or public roster.
- One sentence from Heartwood or Sanctuary. Not a new Act. Not mid-WASD.
- Missing list: hour still runs unnamed.

## Offline name algorithm

LIVE: #590 NAME-RITE-SEED @ `1da57daf` and #591 NAME-RITE-OFFER @ `a6f838da`.

1. The seed is persisted. When the seed is 0, reuse order is the main file, then `.bak`, then mint a new one.
2. Two fixed lists in `shared/` (given, house). ASCII, short, no open corpus.
3. Index = `seed % len`. Offer = `{given} {house}`. Seed 0 never offers a name.
4. `name_rite::refuse()` exists in shared and adds one (`wrapping_add(1)`). `client/**` never calls it. On main, Escape or an empty draft skips with no rotation.
5. After Settled, the offer is pre-filled on the existing NameHouse hint. Enter accepts.
6. No ban list in this card. Filter-over-ban stays `docs/NAME_DOOR_LAW.md`.

## Procedural lore method

Recipe, not a model essay. Same family as GenShare.

1. Inputs already lived: Place chip, wells tended, Flow/Reserve held, name if accepted.
2. One template sentence. Slots fill from those inputs. No new facts.
3. Same inputs → same sentence. Camera and particles are not inputs.
4. `docs/AMBROSIAN_*` is stock. It is not the generator and not Title.
5. Do not stream paragraphs. Do not call out for lore.

## PATH (LIVE)

- `shared/house_name/name_rite.rs` (#588 NAME-RITE-LIST @ `61ae66f`, the word lists).
- `shared/house_name.rs` (#590 @ `1da57daf`).
- `client/src/title_screen.rs` (#591 @ `a6f838da`).
- No `lib.rs` mod. No Title chrome. No Online.

## Refuse

- Creator gauntlet before the first well.
- Internet name API. AI assist on.
- Parent Act, new PlaceId, guild roster.
- Shipping the algorithm from this file alone.

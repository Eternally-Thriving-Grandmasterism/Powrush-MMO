# CHARACTER_IDENTITY.md: creator, backstory, loyalties and cosmetics

**Contact:** info@Rathor.ai

GREEN-DOCS. Design tick, not a Cargo bump. **Title Online stays grey.** Card DOC-IDENT-0, read against `main` at `3fd96f37`. This is a proposal, not a work order: nothing lands in the client until Core confirms a slice. No dates. No new People, no Place, no stats, no shop.

Parent law (cross-linked, not restated):
- [FACTION_HERALDRY](FACTION_HERALDRY.md): finishes on the item, bench work, wear, no transmog, no mall, house heraldry editor, refusals.
- [GENSHARE](GENSHARE.md): `dress` is cosmetic string law; recipes shared offline with explicit consent.
- [LAUNCH_UX](LAUNCH_UX.md) D3: seals and heritage after Settled; no race select at Title.
- [PLAYABLE_RACES](PLAYABLE_RACES.md), [design/FACTION_LEADERS](design/FACTION_LEADERS.md), [FACTIONS_OVERVIEW](FACTIONS_OVERVIEW.md), [HUMAN_RACE_MECHANICS](HUMAN_RACE_MECHANICS.md), [ART_BIBLE](ART_BIBLE.md), [visual-pass/TARGET_BOARD](visual-pass/TARGET_BOARD.md), [NET_OFFLINE_CONTRACT](NET_OFFLINE_CONTRACT.md).

## 1. Law
1. **Identity never changes stats.** Looks, origins and loyalties carry story and standing, never power (Temper, Lumen and Ward stay the only ladder).
2. **Recipe, not triangles.** Every choice is stored as small ids and palette indices, extending `dress` (GENSHARE). No images, no uploads.
3. **Offline first.** Everything works with Online grey. Sharing is GenShare Method B (file or paste, explicit confirm, never mid-WASD).
4. **Canon only.** Presets and origins cite an existing doc. Anything without a source is marked **UNKNOWN** and needs the steward before it ships.

> **LEVELS-CANON-1 (2026-10-27):** "the only ladder" means the only *power* ladder. Levels unlock recipes and access, not raw stats ([LEVELS_CANON](design/LEVELS_CANON.md) R3).

## 2. Creator scope
- **When:** after Settled, extending D3 (LAUNCH_UX). Skippable; a default preset applies. Never at Title.
- **Peoples:** the five in PLAYABLE_RACES only: Human, Cydruid, Quellorian, Ambrosian, Draek. No sixth People.
- **Presets:** 3 to 4 per People, each a full readable look.
- **Optional options (curated, not free sliders):** tone (skin, chitin, crystal) from the palette; hair, crest or crown shape (few choices); face preset; marks or scars; seam or light colour within that People's canon; build in 3 steps. Canon guards: Draek and Vrendak's kind are plated chitin, never scaly or reptile (CODEX_ART_PROMPTS); Humans alone may show alignment-shift traits (veins, grafts, crystal inclusions; HUMAN_RACE_MECHANICS §3).
- **Art fit:** options are shape and material swaps on low-poly silhouettes under the painted-look lighting (ART_BIBLE direction override). Whether geometry density rises stays ART_BIBLE open question 2: **UNKNOWN**.

## 3. Backstory
- **Preset origins** (about 4 per People), one line each, citing a doc. Human examples with sources: Earth defender, Coalition scavenger (Independent Earth Coalition), Quellorian-allied, uneasy Draek-truce survivor (HUMAN_RACE_MECHANICS §2). Origins for the other four Peoples: **UNKNOWN** until each is drafted from its race doc and reviewed.
- **Optional custom bio:** up to about 280 characters, word filter, stored locally, shared only by opt-in GenShare.
- No stats, no unlocks from origins.

## 4. Loyalties
- Choices: Quellorian / Aetherion Luminari Alliance; Independent Earth Coalition (Humans); Cydruid groves; Ambrosian observers; unaligned. Draek-aligned opens only through existing redemption and truce arcs.
- Affects: starting standing, dialogue lines, the trim colours offered. Never stats.
- Whether loyalty seeds `faction_reputation`: **Core vote** (ID-5).

## 5. Cosmetic slots (FACTION_HERALDRY bench and wear law)
| Target | Slots | Services |
|---|---|---|
| Armour | shoulder patch, chest decal, tabard | decal, stitch, paint |
| Gear | one grip or wrap, one engraving, one small decal | wrap, engraving, decal |
| Vehicle | two decal slots, hull paint | decal, paint, metal finish |

Every change is bench work with materials and time, fails softly, wears, and is restored by maintenance. "Skins" means paint and finish jobs, never purchasable unlocks. Fixed slot positions keep readability at 32 px and about 30 m.

## 6. Guild crests
Guild tabards and banners use the house heraldry editor (FACTION_HERALDRY §3 and §4): curated original library, rule of tincture, 3:1 contrast, 32 px preview, motto filter. `src/guild.rs` gains a crest recipe reference (HER-2). Shared guild banners and the report queue wait for "online yes".

## 7. Out of scope
- **Real-world flags: out.** Sherif's own ruling, 2026-10-23 2:51 AM ET, verbatim: *"I figured, but worth considering, because people love their loyalty symbols. However, they will likely cause problems. So let's skip anything harmful like that, and also, the AI generated stuff can be skipped and left for much later due to unnecessary feature creep complexity over a minor gain."* FACTION_HERALDRY's refusal of real-world marks and national arms stands. Rationale: the councils earlier rejected IP-based flag assignment for accuracy, privacy and harassment reasons.
- **AI-generated crests (Grok Imagine via Rathor.ai): skipped until much later per Sherif's 2:51 AM ruling** (feature-creep complexity over a minor gain). Nothing is built.
- Transmog, wardrobe, cosmetic shop, AH, RMT, uploaded images, sockets while Online is grey.

## 8. Card plan (ordered, one concern each)
| # | Card | Likely files | Accept | Core vote |
|---|---|---|---|---|
| 1 | DOC-IDENT-0 | this file | canon cited, UNKNOWNs marked | no |
| 2 | ID-1 recipe struct | `shared/character_recipe.rs`, `shared/genshare.rs` | round-trip tests; no stat fields | **yes** (persist schema) |
| 3 | ID-2 creator presets | client creator panel after Settled | no Title race select; readable at Low; frame time within the panel budget set on the card | no |
| 4 | ID-3 appearance options | character material and palette | no extra draw calls per character; readable at about 30 m | **yes** (runtime) |
| 5 | ID-4 backstory | origin content files, bio field, filter | every origin cites a doc | no |
| 6 | ID-5 loyalties | `client/src/faction.rs`, reputation hook | no stats; survives save and load | **yes** |
| 7 | COS-1 slot system | fabricator services, satchel line, Lineage | explicit confirm; never mid-WASD; wear works | **yes** |
| 8 | HER-1 heraldry editor | `shared/` beside `house_name.rs`, House or Ledger UI | tincture and contrast warnings; 32 px preview | no |
| 9 | HER-2 guild tabard | `src/guild.rs`, tabard slot | crest recipe on tabard; readable at 32 px | **yes** |

Frame-time budgets per card are not yet set (TARGET_BOARD): **UNKNOWN**, to be set on each card.

## 9. Steward-only
Revisiting AI crests (skipped until much later); any new canon beyond the cited docs; Online light; public claims and publishing; ART_BIBLE open questions.

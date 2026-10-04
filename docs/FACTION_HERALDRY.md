# FACTION_HERALDRY.md: workshop finishes and house heraldry, without transmog

**Contact:** info@Rathor.ai
GREEN-DOCS. Design tick, not a Cargo bump. **Title Online stays grey.** Steam Offline SKU. This is a proposal, not a work order: nothing lands from these sentences until Core confirms a slice. No dates.

Parent law (cross-linked, not restated):
- `docs/MERCY_TEMPER_PROGRESSION.md` (Temper, Lumen, Ward, Lineage; items never deleted)
- `docs/GENSHARE.md` (`dress` = cosmetic string law; recipe, not triangles)
- `docs/PLACES_BIBLE.md` (no combat stats on recipes; seals and heritage are cosmetic)
- `docs/LAUNCH_UX.md` D3 (House seals and heritage)
- `docs/JOY_WITHOUT_MALL.md` (no auction house, RMT or guild-as-mall)
- `docs/NET_OFFLINE_CONTRACT.md`
- `docs/VISUAL_TARGET.md`

## 1. Law
1. **Cosmetics never change stats.** Temper, Lumen and Ward stay the only power ladder. A finish carries who you look like, not what you hit for.
2. **Physical, not projected.** A look lives on that one item, as paint, wrap, engraving or stitching. No transmog: no wardrobe, no ghost skins, and no copying one item's appearance onto an unrelated item. To change a look, work on the item.
3. **Real work.** Every change costs local materials and time at a bench or a crafter, and can fail softly: the item rests and is never destroyed.
4. **Wear tells the story.** Finishes age with use (chipped paint, a polished grip, a faded banner). Maintenance restores them. Lineage records who did the work.
5. **No mall.** No cosmetic shop, no auction house, no RMT. Trade in finishes is a person-to-person service.

## 2. Workshop services (hang off the existing fabricator; no second bench in Hour 1)

| Service | Changes | Inputs | Wear |
|---|---|---|---|
| Paint job | Base and accent colour; finish (gloss, satin, matte or enamel) | Pigment, binder, time | Edge chips; repaint |
| Grip or wrap redo | Handle material and pattern | Wrap material, time | Polishes or frays; re-wrap |
| Engraving | A short line or motif on metal | Bench time, crafter skill | Permanent; joins Lineage |
| Decal or transfer | House emblem on predefined slots | Heraldry recipe (§3) | Fades; reapply |
| Tabard or banner | Stitched house arms on cloth, or a banner at the House post | Cloth, thread, dye | Fades; restitch |
| Metal finish | Polish, patina or blueing | Consumables | Dulls; re-polish |

The work runs on the bench, in the hand loop, with explicit confirm and never in the middle of WASD. The satchel gains one line, the finish name, following the Temper satchel-line pattern.

## 3. House heraldry editor
- **Layers:**
  1. **Field:** one tincture.
  2. **Division:** none, per pale, per fess, per bend, quarterly, chevron or saltire.
  3. **Charge:** one or two from a curated, original library.
  4. **Border:** plain, engrailed or indented.
  5. **Motto:** a scroll of 24 characters or fewer.
- **Symmetry:** vertical mirror by default, centre snap, a counter-change toggle, fixed charge sizes. No freehand pixels and no image upload.
- **Seed:** the shipped D3 seals `well` / `grove` / `ember` (`shared/house_name.rs`) become the first three charges. Nothing shipped is discarded.
- **Data:** a small recipe (ids and palette indices), never an image. It is proposed as a new optional key beside seal and heritage in `dress` (`GENSHARE.md`), and is shared as a recipe only, so peers rebuild it locally.

## 4. Palette and readability
- About 12 curated tinctures: metals (gold, silver) and colours (royal purple, ruby, deep blue, deep green, black, slate, iron), each with a dark-world and a light-world value. No neon fields.
- **Rule of tincture:** no colour on colour and no metal on metal.
- **Contrast:** at least 3:1 between a charge and its field, enforced with a live warning and an automatic metal-edge fallback.
- **Preview:** at 32 px and at about 30 m in-world. If it is unreadable at 32 px, the editor says so.

## 5. Moderation, abuse and IP
- The curated library sharply reduces symbol abuse. A combination check also blocks known hate-symbol arrangements of crosses, runes, numbers and rotations. Mottos pass a word filter.
- **Offline phase:** adopting another house's recipe needs explicit consent (GenShare Method B). Nothing is broadcast.
- **Online phase (only after steward "online yes"):** report a banner, hide it locally at once, send it to a review queue, and ban that arrangement at recipe level.
- **IP:** the library is original. No real trademarks, sports crests, national arms or franchise logos. There is a similarity checklist for presets and a "looks like a real brand" report reason.

## 6. Presets and templates bar
- About 24 launch presets: roughly 4 per practice plus neutral house arms. Each passes the 32 px silhouette test and the rule of tincture, gets a council review, and carries a one-line meaning so players choose them for story.
- The craft bar learns technique from StarCraft 2 decal and portrait work (one strong silhouette, a confident metal edge, read at a glance) and from WoW tabards (layered field and charge). It copies none of their art, shapes or names.

## 7. Phase ladder (honest, no dates)

| Phase | Fits today's repo | Contents |
|---|---|---|
| 0: docs | Yes | This file. |
| 1: local recipe | Yes (Bevy 0.14, Offline, Online grey) | A heraldry recipe struct in `shared/` beside `house_name.rs`, persisted in the existing House JSON under `data/`. The seals become charges. A simple editor at the House or Ledger and a banner at the House post. No stats. |
| 2: workshop finishes | Yes, local | Paint, wrap and engraving on the fabricator; wear and maintenance; Lineage entries; a satchel line; a crafter NPC on a local schedule (`NPC_SCHEDULE_SPEC.md`). |
| 3: share as a recipe | Partly (GenShare A, B and D are offline) | Export and import the heraldry recipe through a file or a paste with consent. No sockets. |
| Later: online | No, waits for "online yes" | Player crafters trading services, a shared faction banner, the report and review queue, recipe bans. |

## Refuses
Transmog or wardrobe · cosmetic shop · AH / RMT · stats on finishes · freehand or uploaded images · real-world marks · sockets while Online is grey · second HUD · Hour 1–3 changes.

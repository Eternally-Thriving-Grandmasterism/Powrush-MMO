# EA_BAR.md — the Early Access loop as one verb chain

**Contact:** info@Rathor.ai  
**Independent of xAI.** No certification or warranty claims.

**Docs-only.** Card LOOP-SPINE-1: queue row 9 in [#738 6867796772](https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO/issues/738#issuecomment-6867796772), CUT [#738 6873719031](https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO/issues/738#issuecomment-6873719031). No `client/**` · `shared/**` · assets · player-facing copy. No gameplay change. Every `file:line` below is anchored at main `b51c7f76`; re-anchor before a later card quotes one.

> **What this page is.** A council reconciliation page. It is not Sherif's words and not a work order. It records two council picks (§3), adopted by the PATSAGi Councils under Sherif's typed delegation (t1767u/t1950u, as recorded in 6867796772). Sherif may veto or amend either. Nothing here is called fixed; per 6867796772 §1, nothing is called fixed until Sherif plays it.

D-numbers (D1, D2, D3, D16) are rows of the PLAYTEST-T2008 doc-review map that 6867796772 rules on.

---

## 1. Where the bar comes from

- Lead relayed Sherif's EA-bar message (t2012u). Core has not verified it directly (6867796772, header). This page quotes none of it. Sherif confirms or amends the bar's wording before anything here is read as his; `EA_BAR.md` waits for this card and his confirmation ([#738 6867920511](https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO/issues/738#issuecomment-6867920511), out of scope list).
- The councils ruled from that relay: §5b names "the CO × WoW bar" for gear, and §5c names a minimum TTD leg for EA. The GDD's reference maps stay the teachers: Conquer Online `docs/GDD_IMMERSION_REVISION.md:48`, WoW `:73`, Transport Tycoon Deluxe `:90`.
- The GDD core stays as written: sacred verbs Tend / Take / Flow / Reserve / Mend, score = tons + restored, credit never gold (`docs/GDD_IMMERSION_REVISION.md:42`). The TTD chain is harvest → refine → ship → manufacture (`:94`, Rung D `:149`).
- The visual bar, the canonical Drive folder (D6) and powrush.com copy are reserved to Sherif. This page decides none of them.

## 2. The verb chain at `b51c7f76`

The EA loop, as this card defines it: **gather → refine → haul → make → equip or unlock → harder content**, closed each week by the House bill. Each row names what is on main at a live `file:line`, and the named next card for the gap. Row numbers are queue rows in 6867796772 §4.

| Verb | On main (`file:line` at `b51c7f76`) | Gap and named next card |
|---|---|---|
| **Gather** | Tend reach `HARVEST_REACH` (`client/src/mercy_harvest_nodes.rs:46`). Guidance chain `GuidanceObjective` (`client/src/first_session_guidance.rs:361-399`). Satchel slots are Vitality / Harmony / Joy only (`client/src/human_inventory.rs:32-36`). One owned tree writes one raw unit to `SAVE_NAME` (`client/src/wood_expedition.rs:18`), field `unit` (`:44`). | Wood is not a satchel row and has no consumer: **WOOD-USE-1** (row 11). Node variety: no queue row names a card. |
| **Refine** | No recipe takes wood. `Recipe` is MendSpool, LaneCrate, TendHook (`shared/fabricator.rs:21-26`); Tend Hook costs 1 MendSpool (`:197-199`). | Wood → output: **WOOD-USE-1** (row 11), the §5c refine leg. |
| **Haul** | Carry feel only: `heavy` (`client/src/living_body.rs:22`), `carry_mul` (`:82-84`), set at `:143`. Place travel `HexTravelState` (`client/src/hex_travel.rs:81-83`); Threshold rides Heartwood, no fifth `PlaceId` (`:5`, `:116`). | No cargo leg adds tons by route: **HAUL-LEG-1** (row 12), the §5c haul leg. Carts and vehicles come after EA (§5c). |
| **Make** | The fabricator is a civic beat: plant, then MendSpool and LaneCrate refresh the week slab (`client/src/fabricator.rs:3-4`); `craft` (`shared/fabricator.rs:155`). The client header says no new recipe (`client/src/fabricator.rs:13`). | Making from hauled goods: no queue row names a card. It is not part of the §5c minimum. |
| **Fight** | Sanctuary practice fight `STAVEHOLD_FIGHT_HOME` (`client/src/combat_core.rs:50`), `fight_home` (`:252`). XP per landed hit (`:316`), daily cap (`:318`). | Guidance never points at it: **HUD-CLEAN-1** (Lane U3), which folds **GUIDE-FIGHT-1** (row 14). Combat never scores the week (§4). |
| **Loot** | Named finds roll once per defeat (`client/src/loot_drops.rs:1`), no odds shown (`:3`). `FINDS_SAVE` (`:37`). `FoundFinds` is names only, no stats (`:43-45`); an owned name is skipped (`:52`). Wired at `client/src/combat_core.rs:578`. A find carries a name, a sentence, a role and Temper / seated / Ward fields (`shared/temper.rs:220-227`); six in `NAMED_FINDS` (`:236-243`). | Not a satchel row: **LOOT-TO-SATCHEL-1** (row 10). |
| **Equip** | No slot and no effect: finds carry no stats (`client/src/loot_drops.rs:43`). COMBAT_AND_PVP §9 already assumes equipped gear stays on the body (`docs/design/COMBAT_AND_PVP.md:248`). | **GEAR-SLOT-1** (row 13), only if Sherif does not veto §5b. |
| **Unlock / harder content** | `total_level` (`client/src/skill_xp.rs:217-221`), Total line (`:224-226`). Tiers gate on total level 15 / 30 (`client/src/difficulty.rs:11-12`) in `gate` (`:58-68`). LEVELS_CANON R3: levels unlock recipes and access, not raw stats (`docs/design/LEVELS_CANON.md:17`). | No recipe or access unlock yet; tiers only. The levels-to-ladder mapping is banked as **LEVELS-LADDER-MAP-1** (`docs/design/LEVELS_CANON.md:17-18`). No queue row names an unlock card; this page invents none. |
| **Week bill** | `WeekAudit` `tons_moved` / `restored_count` (`shared/week_audit.rs:10-16`), fed by `sync_from_climate` (`:30-34`). `tons_and_restored` never counts kills, gold or Market (`:44-47`). Spoken by `house_week_line` (`shared/pause_ledger_face.rs:76-79`). | No refine or haul leg writes it yet: rows 11 and 12. |

## 3. Lifted for EA — council picks, Sherif may veto

Only these two. Each cites 6867796772 §5b or §5c and nothing else. Both are council picks adopted by the PATSAGi Councils under Sherif's typed delegation; Sherif may veto or amend either. This page records them. It does not spend them.

**D1 · Gear (§5b).** Council words: "the CO × WoW bar lifts the GDD's blanket refusal **only** for gear with readable, bounded effects on the one power ladder (Temper, Lumen, Ward). There is still no published power scalar, no cash-shop power and no socket gambling."

- Already bannered at `docs/GDD_IMMERSION_REVISION.md:46` (§2.1) and `:138` (Rung B "gear slots"). The Refuse rows at `:58-59` and the Rung B line at `:136` are not rewritten.
- It sits on LEVELS_CANON R3, one power ladder (`docs/design/LEVELS_CANON.md:17`), and COMBAT_AND_PVP §9 (`docs/design/COMBAT_AND_PVP.md:248`).
- GEAR-SLOT-1 (row 13) may be briefed against it. It ships only if Sherif lets §5b stand.

**D16 · Minimum TTD for EA (§5c).** Council words: "one refine leg (wood to output) plus one haul leg that counts tons. Carts and vehicles come after EA."

- The gap: Rung D's Order line says "after B and C" (`docs/GDD_IMMERSION_REVISION.md:153`). The queue sequences WOOD-USE-1 and HAUL-LEG-1 "After 9" (6867796772 §4, rows 11–12), not after rungs B and C.
- Bannered after `docs/GDD_IMMERSION_REVISION.md:153`. The banner records the EA minimum only. It confirms no rung, marks no §5 row, and authors no carts or vehicles.
- Rung D's hard shape (`docs/GDD_IMMERSION_REVISION.md:152`: no price, no sell, Market out of the House sum) and its Gate line (one leg per PR, `shared` sim first) still bind both legs.

## 4. Already bannered — cited, not reopened

DOCS-SYNC-1 (#1003) bannered these. This page lifts nothing for them.

- **D2 · Levels and the XP bar.** Skill levels and a skills panel are canon; an always-on XP bar stays refused. Banners: `docs/GDD_IMMERSION_REVISION.md:71`, `docs/FUN_WITHOUT_WOW.md:95`. Canon: LEVELS_CANON R1–R5 (`docs/design/LEVELS_CANON.md:15-24`).
- **D3 · Combat and the week.** Combat exists but never scores the week. Banner: `docs/GDD_IMMERSION_REVISION.md:88`. Code: the score legs are tons + restored only (`shared/week_audit.rs:44`).

## 5. Still refused for EA

| Refused | Where |
|---|---|
| A published power scalar (Battle Power) | `docs/GDD_IMMERSION_REVISION.md:58`; §5b |
| Cash-shop power, mall, socket gambling | `docs/GDD_IMMERSION_REVISION.md:59`; §5b |
| XP from kills | `docs/GDD_IMMERSION_REVISION.md:60`; `docs/design/LEVELS_CANON.md:15` |
| An always-on XP bar | `docs/GDD_IMMERSION_REVISION.md:71`; `docs/FUN_WITHOUT_WOW.md:93`, `:95` |
| Combat scoring the week | `docs/GDD_IMMERSION_REVISION.md:88`; `shared/week_audit.rs:44` |
| Gold, auction, price, or Market in the House sum | `docs/GDD_IMMERSION_REVISION.md:82`, `:100`, `:152` |
| Carts, vehicles, fleets or timetables in EA | §5c; `docs/GDD_IMMERSION_REVISION.md:99` |
| A fifth Place | `docs/GDD_IMMERSION_REVISION.md:98`; `client/src/hex_travel.rs:5` |

## 6. What this page unblocks

In queue order (6867796772 §4). Each card gets its own PATHS, caps and fixtures from Core; this page names none.

| Row | Card | Leg | Condition |
|---|---|---|---|
| 10 | LOOT-TO-SATCHEL-1 | loot → satchel | After 9 |
| 11 | WOOD-USE-1 | refine (§5c) | After 9 |
| 12 | HAUL-LEG-1 | haul that counts tons (§5c) | After 9 |
| 13 | GEAR-SLOT-1 | equip, bounded (§5b) | After 9; only if Sherif does not veto §5b |
| U3 | HUD-CLEAN-1, folding GUIDE-FIGHT-1 (row 14) | guidance and HUD | After UI-KIT-1 and this page ([#738 6867950960](https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO/issues/738#issuecomment-6867950960)) |

## 7. Reserved to Sherif

- Veto or amend §5b and §5c.
- Confirm or amend the EA-bar wording relayed as t2012u before this page is read as his.
- D6 (the canonical Drive folder), the visual bar, and powrush.com copy.
- Money, licences, asset packs, Steam, deploys, tags, releases and public claims.
- Whether a later loop card plays right. Nothing is called fixed until he plays it (6867796772 §1).

## 8. Out of scope here

No code. No in-game strings: the guidance lines at `client/src/first_session_guidance.rs:384-399` are unchanged. No assets, pricing, store copy or D6 Drive IDs. GDD proposal rows and §5 verdict rows are not rewritten. The stale comment at `client/src/combat_core.rs:2` (D11) is left for a code card.

## Related

[`GDD_IMMERSION_REVISION.md`](GDD_IMMERSION_REVISION.md) · [`FUN_WITHOUT_WOW.md`](FUN_WITHOUT_WOW.md) · [`design/LEVELS_CANON.md`](design/LEVELS_CANON.md) · [`design/COMBAT_AND_PVP.md`](design/COMBAT_AND_PVP.md) · [`PATSAGI_MERGE_COURT.md`](PATSAGI_MERGE_COURT.md)

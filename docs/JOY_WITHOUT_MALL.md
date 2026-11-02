# JOY_WITHOUT_MALL.md — rarity + social joy without the mall

**Stamp:** 2026-09-12 · CARD **H-2026-09-12-JOY-LAW** · tip `0deee8d` (main)  
**Contact:** info@Rathor.ai  
**Independent of xAI.** No certification / AGSi warranty / legal-product claims.

GREEN-DOCS. One page. Design tick, not a Cargo bump. **Title Online stays grey.** Steam Offline SKU. PATHS this file only — no invent PATHS. ART_BIBLE refs only; no binary dump. Does not ship Online, AH, chat, or guild chrome.

---

## Rarity (keep)

- **Two numbers** = Temper `+N` × Lumen seats × Ward kind.
- **Trophy ≠ strongest.** Shell stays mitigation.
- **Lineage** = this hook’s `+N` + first-well-restored.
- **Identify** = one sound + one satchel flash.
- **`restored`** — not an ethereal shop SKU.
- **Show the find** on House / yearbook. Never trade.

## Refuse rarity poison

AH · billion-run EV · perfect-or-trash trading · target-farm tables · RMT bounty · BIS-that-is-also-rarest · affix soup spreadsheet.

## Shown standing (LOOT-LEGIBLE-1)

- **Two numbers, in code:** Temper `+N` and Lumen `a/b`. Ward is a label. `shared/temper.rs` `shown_standing` reads only those two, with no roll.
- **Four standings:** Worn · Kept · Proven · Storied, each with its own word, shape and luminance step. Words and shapes are proposals; Sherif may veto.
- **Named finds:** at most 6, one card each. No drop table or weights (that is LOOT-DROPS-1).
- **Vendor law:** a vendor never offers above Worn, and nothing converts into a find.
- **Dead roll:** the root `src/inventory.rs` `Rarity` roll is dead code. Do not revive it. Removal is banked as ROOT-SRC-PRUNE-1.
- **Unbroken pool:** `client/src/difficulty.rs:135` declares `UNBROKEN_COSMETIC_ONLY_POOL = true`, and only its test at :332 reads it. No cosmetic pool exists in code yet.
- **Named-find roll, inactive:** `client/src/loot_drops.rs` `roll_named_find` (:23) is the LOOT-DROPS-1 pure half, and nothing outside tests calls it (:1). It draws only from the 6 `NAMED_FINDS` (`shared/temper.rs:236`).
- **Weights are council defaults:** Kept 8 · Proven 4 · Storied 2 · Worn 0 (`loot_drops.rs:15-17`), total 28 (test :50), with `ROLE_SKEW` ×2 (:9). This is council wording, not Sherif's ruling, and Sherif may veto or retune it.
- **Above the vendor:** every rolled find sits above `VENDOR_STANDING_CEILING` (`shared/temper.rs:247`, test `loot_drops.rs:66-78`). The roll has no convert, exchange, trade, buy or sell fn (:74-77).
- **Unbroken reads as Sworn:** `power_tier` (`loot_drops.rs:11-13`) means Unbroken's extra over Sworn yields no named find (test :57-64).
- **Skill premium off:** `client/src/run_ledger.rs:135` sets `SKILL_PREMIUM_LIVE = false`. The 2.5× ceiling (`SKILL_PREMIUM_MAX`, :124) is a council default that Sherif may veto.
- **Open for Sherif:** `ROLE_SKEW` ×2 leans finds toward a role, which sits beside the "target-farm tables" refusal at :22. Whether that counts is Sherif's call, and the skew stays inactive until he rules (LOOT-DROPS-WIRE-1 is held).

---

## Social (keep)

- **Place + async traces + human-scale opt-in.**
- **Journey chirp ≈ tend.**
- **Sky dual-consent ≈ P5 picker**, default-off.
- **Solitude is complete.**
- **Traces, not nameplates.**
- **Speech last** — only after Online yes + pair consent.

## Refuse social poison

Fake 12 online · always-on global chat first-run · guild-as-mall · Discord-as-default · LFG role slots · LLM fake neighbours · raid before solo hour works.

---

## Law lines

| Line | Law |
|---|---|
| SKU | **Steam Offline** |
| Title | **Online grey** until steward writes `online yes` |
| PATHS | This file only — **no invent PATHS** |
| Art | **ART_BIBLE** refs only — no binary dump |
| Scope | Docs bank. No `client/**` · `shared/**` · Cargo · deletes |

## Cross-links

[`OFFLINE_SKU.md`](OFFLINE_SKU.md) · [`OFFLINE_ECONOMY_COURT.md`](OFFLINE_ECONOMY_COURT.md) · [`FUN_WITHOUT_WOW.md`](FUN_WITHOUT_WOW.md) · [`ART_BIBLE.md`](ART_BIBLE.md) · [`STRANGER_LOOP.md`](STRANGER_LOOP.md) · [`NET_OFFLINE_CONTRACT.md`](NET_OFFLINE_CONTRACT.md)

## Refuses

`client/**` · `shared/**` · Cargo · file deletes · Title Online light · sockets · AH / RMT / farm tables · always-on chat · Discord-as-default · LLM neighbours · invent PATHS · binary dump · certification / AGSi warranty / legal-product claims · xAI endorsement.

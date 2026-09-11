# MERCY_TEMPER_PROGRESSION.md

**Design tick:** 23.2.T — 2026-09-11  
**Not a Cargo version.** Workspace stays `21.88.0` until Hands merge a playable change.  
**Status:** SPEC ONLY. Do not ship on the living-ecology PR.  
**Canon order:** README.md > COMPLETION_BRIEF.md > this spec for *post–Hour-3* tools only.  
**Repo:** https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO  
**License:** AG-SML (same as repo). No cash shop. No P2W. Online grey.

---

## 0. Agent briefing (read this first)

### What this is
A post–Hour-3 crafting / gear ladder for players:

- ARK: next tool unlocks next honest gather verb
- Halls of Torment: frequent small, visible upgrades
- Classic Conquer Online: +1…+9, sockets, “turtle” mitigation
- **Without** Conquer’s mall, destroy-on-fail, or potency war

Powrush names:

| Player-facing | Meaning |
|---|---|
| **Temper** | +1 to +9 on one item |
| **Lumen** | socket |
| **Ward** | gem / blessing (Shell = turtle-class mitigation) |
| **Lineage** | item remembers the tend that made it. Not a tradable soul. |

### What this is NOT
- Not Hour 1–3 product work
- Not living ecology / NPC schedule / human_presence / F dress
- Not Market, gold, NFT, payments, Title Online, second HUD, crime, ownership of persons, combat

### Live court (2026-09-11) — do not collide

From POST-MERGE #369 @ `16ede9e` / Hands cards:

- E1 **spent**: `living_day.rs` Dawn–Night posts, no ownership/crime, hour can finish without persons
- E2 **YELLOW, next one-leg**: `client/src/living_ecology.rs` — stub/feel work loops at Place posts, same Tend/Mend verbs, Online grey
- E3 **later**: `human_presence` scheduled presence rhyme — only after E2 merges + new POST-MERGE card
- F **GREEN-DOCS after E2–E3**: dress is a separate card. Premature `PLACE_DRESS_SPEC` is cancelled
- Discipline: **one PR then HOLD**
- Gate (never `--workspace` as product-green):
  - `cargo test -p shared -p rsil-identity`
  - `cargo test -p powrush-client --lib`
- Hands beat lore. No OFFER NEXT beside an active card.
- Online grey. Local ok.

**This spec waits until E2 is merged and a new POST-MERGE card exists.**  
Until then: docs-only PR is allowed. Code PR is not.

### Agent execution order (when a card is cut)

1. Docs-only PR: this file + 8-line README pointer under “after Hour three / Embassy book”. HOLD.
2. After E2 merge + new card: **Slice T1** shared types + tests only.
3. HOLD. New card.
4. **Slice T2** fabricator recipes (Tend Hook, Temper +1). No UI chrome beyond satchel line.
5. HOLD.
6. **Slice T3** satchel display `+N · Lumen · Ward`.
7. HOLD.
8. **Slice T4** care-cycle offer (one choice after well returns to Idle).
9. HOLD.
10. **Slice T5** Shell Ward from resting a stressed node.
11. Never combine T-slices with E2/E3/F.

---

## 1. Player fantasy (acceptance sentence)

A stranger who has finished Hour 3 can, without a shop:

1. Craft a **Tend Hook** at the existing fabricator (MendSpool path).
2. See it in satchel as `Tend Hook +0`.
3. After a well they tended returns to **Idle**, get **one** offer: Temper +1 / open Lumen / distill Ward.
4. Fail Temper → tool rests. Item is never deleted.
5. At +3 the hook shows `1/1 Lumen`. Shell Ward seats there and reduces stress they inflict on wells — not player PvP damage (combat is parked).

If any step needs gold, a mall stone, or Title Online, the slice is rejected.

---

## 2. Mapping onto the shipped loop

Current verbs (README):  
WASD · E tend · I satchel · R allocate (1 flow / 2 reserve) · Q House / Proof Pack / fabricator · Embassy E seat · Tab ledger (tons + restored).

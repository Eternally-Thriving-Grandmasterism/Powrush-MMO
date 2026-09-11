# MERCY_TEMPER_PROGRESSION.md

**Design tick:** 23.2.T — 2026-09-11  
**Not a Cargo version.** Workspace stays `21.88.0` until Hands merge a playable change.  
**Status:** SPEC ONLY. Do not ship on the living-ecology PR.  
**Canon order:** README.md > COMPLETION_BRIEF.md > this spec for *post–Hour-3* tools only.  
**Repo:** https://github.com/Eternally-Thriving-Grandmasterism/Powrush-MMO  
**License:** AG-SML (same as repo). No cash shop. No P2W. Online grey.

**Contact:** info@Rathor.ai  
**Independent of xAI.** No certification / AGSi warranty / legal-product claims.

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

```
Hour 1  tend / satchel / allocate          DO NOT TOUCH
Hour 2  House / yard remembers             DO NOT TOUCH
Hour 3  fabricator + Proof Pack + book     READ ONLY
Hour 4+ Temper loop                        THIS SPEC
```

Unlock currency is already on Tab: **restored + tons**. Reuse it. Do not invent gold.

Fabricator already consumes **MendSpool + LaneCrate → Proof Pack**. New recipes hang off that machine only. Do not add a second bench in Hour 1.

---

## 3. Where to code (paths)

Prefer **shared** for types. Client only displays and calls events.  
Parked trees (`server/`, `game/`, `simulation/`, payments) stay parked.

| Concern | Put it here | Do not put it here |
|---|---|---|
| Item / temper / lumen / ward types | `shared/src/` new module `temper.rs` (or `shared/src/items/temper.rs` if items dir exists) | client-only structs that cannot be tested in `-p shared` |
| Recipe table | `shared/src/temper_recipes.rs` | hardcoded in UI |
| Persistence fields | same JSON family as satchel / hour packs under `data/` — extend satchel payload, do not invent a second save root | `data/powrush_shard_climate.json` ecology ledger |
| Fabricator hook | follow existing Hour-3 fabricator / Proof Pack bind (search `Proof Pack`, `MendSpool`, `LaneCrate`, `hour_three`) | new Place post in `living_ecology.rs` |
| Satchel line | existing satchel UI (`I`) — one extra label | second HUD |
| Care-cycle offer | event after node state **Stressed/Resting → Idle** in harvest/well code | NPC schedule, presence, dress |
| Tests | `shared` unit tests + `powrush-client --lib` display tests | `cargo test --workspace` |

Search first (agents must `rg` before inventing files):

```
rg -n "MendSpool|LaneCrate|Proof Pack|satchel|fabricator|NodeState|Stressed" --glob '!docs/archive/**'
rg -n "living_ecology|living_day|NPC_SCHEDULE" client/src
```

If Hour-3 fabricator lives under `client/src/` hour modules, **call it**; do not fork a second fabricator.

`living_ecology.rs` law (`NPC_SCHEDULE_SPEC` @ `75612d4`): same Tend/Mend at Place posts. Temper offers fire from **well state change**, not from NPC presence. Ecology slice stays person-optional. Temper slice must also work with **zero persons on the map**.

---

## 4. Data model (Slice T1)

Keep fields tiny. No floating combat stats.

```rust
// shared/src/temper.rs
use serde::{Deserialize, Serialize};

pub const TEMPER_MAX: u8 = 9;
pub const MERCY_VALENCE_FLOOR: f64 = 0.999;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolTier {
    Hands = 0,
    TendHook = 1,
    LaneCrateMk1 = 2,
    ClimatePick = 3,
    MendSpindle = 4,
    HarmonyLoom = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WardKind {
    Shell,   // turtle: reduce stress inflicted on wells
    Current, // flow allocation feels cleaner (shorter recover)
    Reserve, // reserve mends a neighbour faster
    Dawn,    // first-take contest stays fair (Mira path)
    Book,    // Embassy blueprint page chance (no mall)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lumen {
    pub index: u8,
    pub ward: Option<WardKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperedItem {
    pub id: u64,
    pub tier: ToolTier,
    pub temper: u8,          // 0..=9
    pub lumens: Vec<Lumen>,  // len = lumen_slots(temper)
    pub resting: bool,       // true after failed temper; blocks next temper
    pub lineage: Lineage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lineage {
    /// Who tended / crafted. Not a market identity. Not a soul token.
    pub steward_label: String,
    pub restored_hex_at_craft: u32,
    pub proof_pack: bool,
}

pub fn lumen_slots(temper: u8) -> u8 {
    match temper {
        0..=2 => 0,
        3..=5 => 1,
        6..=8 => 2,
        9 => 3,
        _ => 0,
    }
}

pub fn can_temper(item: &TemperedItem) -> Result<(), TemperError> {
    if item.resting {
        return Err(TemperError::ToolResting);
    }
    if item.temper >= TEMPER_MAX {
        return Err(TemperError::AlreadyNine);
    }
    Ok(())
}

/// Failure never deletes the item.
pub fn apply_temper(item: &mut TemperedItem, success: bool) {
    if success {
        item.temper = (item.temper + 1).min(TEMPER_MAX);
        let want = lumen_slots(item.temper) as usize;
        while item.lumens.len() < want {
            item.lumens.push(Lumen {
                index: item.lumens.len() as u8,
                ward: None,
            });
        }
        item.resting = false;
    } else {
        item.resting = true;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemperError {
    ToolResting,
    AlreadyNine,
    NoEmptyLumen,
    WellStressed, // cannot extract harder from a stressed well
}

pub fn seat_ward(item: &mut TemperedItem, ward: WardKind) -> Result<(), TemperError> {
    let slot = item
        .lumens
        .iter_mut()
        .find(|l| l.ward.is_none())
        .ok_or(TemperError::NoEmptyLumen)?;
    slot.ward = Some(ward);
    Ok(())
}

pub fn unseat_ward(item: &mut TemperedItem, index: u8) -> Option<WardKind> {
    item.lumens
        .iter_mut()
        .find(|l| l.index == index)
        .and_then(|l| l.ward.take())
}
```

**Required tests in `shared`:**

- `lumen_slots(2)==0`, `lumen_slots(3)==1`, `lumen_slots(9)==3`
- fail temper sets `resting`, does not change `temper`, does not drop item
- success +1 opens slots and never exceeds 9
- `seat_ward` fails when full
- Hands-tier item can exist with temper 0 and no lumens

Serde must round-trip so satchel JSON stays loadable.

---

## 5. Economy (RBE, no mall)

Costs are **care artifacts already in the hour**, not currency.

| Action | Cost | Unlock gate |
|---|---|---|
| Craft Tend Hook | 1 MendSpool | Hour 3 fabricator planted |
| Temper +1…+6 | surplus **flow** already allocated + tool not resting | item in satchel |
| Temper +7…+8 | House standing + Tab **restored** threshold | Hour 2 settled |
| Temper +9 | Embassy book page + Proof Pack lineage | Hour 3 seat |
| Open Lumen | automatic at +3/+6/+9 | — |
| Distill Shell Ward | rest one **Stressed → Idle** well without taking | harvest state machine |
| Distill Current Ward | allocate flow that visibly grows the field | R allocate 1 |
| Distill Reserve Ward | reserve that later mends a neighbour | R allocate 2 |
| Seat / unseat Ward | Mend Spindle (tier 4) or Embassy | later slice |

**Anti-P2W law**

- No payments crate
- No “temper stone” item that can be listed
- No destroy-on-fail
- No stat that only a cash pack can cap
- Better tools **must not** take more from a Stressed well (`TemperError::WellStressed` if a hook tries)

Shell Ward effect (Slice T5):  
`stress_inflicted *= 0.7` (turtle −30% well-stress, not PvP). Clamp so a Hook +9 + Shell still cannot force a Stressed node.

---

## 6. Tool ladder (ARK shape)

| Tier | Item | Honest verb | What it must not do |
|---|---|---|---|
| 0 | Hands | E tend on Glowing | — |
| 1 | Tend Hook | shorter tend window on Idle/Glowing | no take from Stressed |
| 2 | Lane Crate Mk I | carry more flow before node tires | no hidden bag of holding for hoard |
| 3 | Climate Pick | tend Resting nodes without flipping them to Stressed | no force-open of dead wells |
| 4 | Mend Spindle | clear `resting` flag; unseat Ward | no instant +9 |
| 5 | Harmony Loom | weave Ward at fabricator | no mall gem printer |

Tiers 2–5 are **later cards**. Slice T2 ships **Tend Hook only**.

Recipe sketch:

```rust
pub struct Recipe {
    pub id: &'static str,
    pub output: ToolTier,
    pub need_mend_spool: u8,
    need_lane_crate: u8,
    pub need_proof_pack: bool,
    pub need_embassy_seat: bool,
}

pub const TEND_HOOK: Recipe = Recipe {
    id: "tend_hook_v1",
    output: ToolTier::TendHook,
    need_mend_spool: 1,
    need_lane_crate: 0,
    need_proof_pack: false,
    need_embassy_seat: false,
};
```

If the live satchel already stores named stacks (`MendSpool`), match those string IDs exactly. Do not rename Hour-3 items.

---

## 7. Care-cycle offer (Halls of Torment cadence)

Trigger: harvest node transitions to **Idle** after this steward tended it this session.

Show **one** card, not a loot table:

1. Temper this tool
2. Distill a Ward (if the transition was Stressed→Idle, offer Shell)
3. Dismiss

No second HUD. Reuse the existing one-card teaching surface (Hour-1 card style). H still hides guidance.

Dismiss is always valid. Offers never block walking or E.

---

## 8. Satchel display

One line under the item name:

```
Tend Hook +3 · Lumen 1/1 · Ward: Shell
Tend Hook +1 · Lumen 0/0
Tend Hook +4 · resting
```

No DPS numbers. No comparison arrows vs other players.

---

## 9. How to code (Hands rules)

1. **Read live files** before writing. `rg` first.
2. **One PR, then HOLD.** Slice T1 is types+tests only.
3. **Product-green tests only:**
   ```
   cargo test -p shared -p rsil-identity
   cargo test -p powrush-client --lib
   ```
   Never treat `cargo test --workspace` as ship-green (parked crates).
4. **Online grey.** No net, no presence fake, no “12 players.”
5. **Do not edit** `living_ecology.rs` / `living_day.rs` in a Temper PR.
6. **Do not invent** PLACE_DRESS, crime, ownership, combat, Market.
7. Prefer events over tight coupling (`ARCHITECTURE.md`).
8. Persistence: extend existing satchel / hour JSON; migrate old saves by defaulting `temper=0`, empty lumens.
9. Feature flag if needed: `temper_loop` default **off** until T4 playtest. Hour 1–3 must run unchanged with flag off.
10. Comments: short, say *why*. No lore novels in code. Hands beat lore.
11. Commit message style matching court: one slice id, Online grey, what file, what not.

Example commit:

```
T1 temper types in shared (no client verb). Online grey.
Does not touch living_ecology. Tests: -p shared.
```

---

## 10. Slice cards for autonomous agents

Cut these only after E2 POST-MERGE.

### T0 — docs (allowed now)
- Add this file under `docs/MERCY_TEMPER_PROGRESSION.md`
- Optional README sentence after Hour-3 list: “Post-Embassy tool temper is spec-only; see docs/MERCY_TEMPER_PROGRESSION.md.”
- PR then HOLD.

### T1 — types
- Owner: shared
- Files: `shared/src/temper.rs` + `mod` wire + tests
- Done when: `cargo test -p shared` green; no client change
- HOLD

### T2 — Tend Hook recipe
- Owner: Hands on fabricator bind
- Craft Tend Hook from 1 MendSpool at existing Hour-3 fabricator
- Flag off → recipe hidden
- Tests: recipe rejects missing spool; does not spend on Stressed well
- HOLD

### T3 — satchel line
- Owner: client satchel
- Render `+N · Lumen · Ward · resting`
- `--lib` test for format helper
- HOLD

### T4 — care-cycle offer
- Owner: harvest/well state machine (not ecology NPC file)
- One card on Idle-after-tend
- HOLD

### T5 — Shell Ward
- Owner: harvest stress math
- Distill on Stressed→Idle; seat in empty Lumen; 0.7 stress factor
- HOLD

Later (not this wave): Climate Pick, Mend Spindle, Harmony Loom, +7–+9 gates, Book Ward.

---

## 11. Fun-check (must stay true)

- Hour still finishes **without persons** (E1 law).
- Same Tend/Mend verbs. No new crime verb.
- Stranger can feel “my hook got better” in one care cycle.
- A +9 Shell hook is proud, not paywalled.
- Ecology posts at Dawn–Night keep working if Temper flag is off.

---

## 12. Reject list (close the PR)

- payments, mall, gold, NFT, Title Online, second HUD
- destroy-on-fail
- editing `living_ecology.rs` on this card
- F dress / human_presence on this card
- `--workspace` as the only test
- two slices in one PR
- OFFER NEXT stacked on an open YELLOW card

---

## 13. Steward note

Classic Conquer died when +9 and turtle gems became cash-shop potency.  
Powrush keeps the *feeling* (this tool is mine, it grew with my care) and throws away the till.

ARK’s next pickaxe is here as the next **honest tend tool**.  
Halls of Torment’s every-minute bump is here as the Idle-after-tend card.

Build T0 now if you want. Build T1 only on a fresh POST-MERGE card after E2.

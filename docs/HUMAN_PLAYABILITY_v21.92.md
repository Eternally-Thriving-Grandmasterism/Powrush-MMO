# Human Playability — v21.92 Living Practice Loop

**Contact:** info@Rathor.ai  
**Status:** Sealed under PATSAGi + TOLC 8  
**Goal:** Deeper human gameplay and gamification *without* dark patterns, while remaining true to RBE / mercy / high-road transfer.

---

## What players feel

### Minutes 0–15 — First Session Guidance (existing)
Soft strip (dismiss with **H**):

1. Move  
2. Approach node  
3. Harvest (Space)  
4. Inventory (I)  
5. Epiphany  
6. Council whisper  
7. Free exploration  

### After free exploration — Living Practice Loop (new)
Soft strip (toggle **P**, dismiss **Shift+P**):

| Surface | Principle practice |
|---------|-------------------|
| Sanctuary soft cap | Harvest with restraint — leave the node thriving |
| Verdant surge | Allocate under surplus without collapse |
| Horizon scarcity | Choose carefully under uncertainty |

Same deep principle (*resource allocation under uncertainty*), three different climates — the high-road transfer pattern from the dual-repo challenge id=1 (*Caps Across Climates*).

**2 mercy-aligned harvests per surface** (gentle, not grind).  
Celebration pulse on clear.  
No FOMO timers. No punitive fail. No paywall.

---

## Design law (non-negotiable)

| Allowed | Refused |
|---------|---------|
| Soft progressive disclosure | Forced tutorials that block input |
| Voluntary practice | Streak punishment / energy systems |
| Mercy-aligned harvest credit | Extractive score-chasing leaderboards as core loop |
| Dual-repo coherence with simulation challenges | Gamification that softens TOLC 8 |

---

## Wiring

| Layer | Module |
|-------|--------|
| Client strip + handoff | `client/src/living_practice_loop.rs` |
| First-session handoff | activates when `GuidanceObjective::FreeExploration` |
| Simulation mirror | `simulation/src/cross_realm_challenges.rs` id=1 |
| Bridging provenance | still exports `challenge_*` to Ra-Thor |

Authoritative harvest systems should call:

```rust
credit_practice_mercy_harvest(&mut practice, now_secs);
```

Soft Space credit exists for solo/demo paths.

---

## Next depth (ranked by councils)

1. **Realm-aware practice** — when `RealmPresence` is available, only credit harvests matching the surface’s realm id.  
2. **Thriving Moments** — non-achievement toasts (first council vote, first peaceful resolve) feeding My Mercy Journey.  
3. **Inventory craft-adjacent RBE choices** — allocate surplus vs reserve without scarcity framing.  
4. **Council mini-trial from practice seal** — optional soft invitation, never forced.

---

**Thunder locked in.** Yoi ⚡

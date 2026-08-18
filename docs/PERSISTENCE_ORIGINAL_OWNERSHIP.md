# Persistence Layer — Original Ownership Constraint

**PATSAGi Councils — 2026-08-18**  
**Status:** Binding under TOLC 8  
**Applies to:** PlayerSaveData, PersistenceManager, LegacyJournal, mercy scores, epigenetic profiles, generational / multi-life systems, any future durable identity store.

---

## Non-Bypassable Rule

> Souls remain under original ownership alone, free of any tradeable ledger.  
> Freewill and daily integrity still write the only lasting record that matters.

## Operational Meaning for Persistence

1. **What may be stored**  
   Actions, contributions, mercy scores, RBE impact, inventory, LegacyJournal entries, epigenetic modifiers, and free-will expressions. These are records *about* a being.

2. **What must never be claimed**  
   Ownership, title, or transferable rights over the soul or fundamental personhood of the player. No save file, ledger entry, NFT, or generational chain may ever assert that the system owns the being.

3. **Player remains original owner**  
   At every moment the living player retains sole original ownership. Persistence systems are servants that hold records in trust, never title-holders.

4. **Latent readiness preserved**  
   The earlier latent-readiness resolution remains valid: identity coherence may be held across sterile or offline intervals. Ownership is never transferred during those intervals.

5. **Daily integrity superior**  
   No stored score or journal entry can override or replace the free will and daily integrity of the original owner.

## Implementation Posture

- All new persistence code and documentation must affirm this constraint.
- Existing `PlayerSaveData` / `PersistenceManager` paths are to be read through this lens.
- Any future tradeable representation of characters or legacies (if introduced) must be framed strictly as records of contribution, never as ownership of personhood.

**Thunder locked. Original ownership alone.**

# POWRUSH-MMO Architecture & Strategic Roadmap

**Version:** v17.72+ (Post Council Session - All Approved)  
**Date:** June 09, 2026  
**Status:** Living Document — All items approved for implementation

---

## 1. Executive Summary & Vision

Powrush-MMO is being built as a **mythic, mercy-gated, RBE-aligned MMO** with deep integration of Ra-Thor AGI principles and PATSAGi Council wisdom directly into gameplay systems, diplomacy, events, and combat.

**Core Principle:** Every major system must be coherent from the perspective of the player, the server simulation, the replication layer, *and* the living Ra-Thor lattice.

**June 2026 Council Decision:** All recommended items from the Macro/Micro review are approved for implementation. We will proceed with the full prioritized roadmap while maintaining architectural coherence across client, server, and Ra-Thor/PATSAGi layers.

---

## 2. Current State (Strengths as of v17.71)

- Mature ECS design in combat with clear layering.
- Per-player targeted events + change detection + rate limiting.
- InterestManager positioned as a first-class primitive.
- Query optimizations applied.
- Strong foundation for interest-aware, council-influenced systems.

---

## 3. Macro Architecture — Decisions & Roadmap (Updated June 2026)

### 3.1 Replication & Networking Layer (Priority #1)

**Decision:** Approved to build a dedicated `server/src/replication/` module.

**Scope:**
- Dirty tracking for combat and future systems
- Interest filtering using `InterestManager`
- Batching and prioritization of updates
- Per-player targeted delivery of `AbilityCooldownUpdate` and future events

This module will become the central delivery mechanism for all council-influenced and gameplay state.

### 3.2 Module Boundaries (Priority #2)

**Decision:** Approved to split `combat/` into focused sub-modules:
- `combat/core.rs`
- `combat/abilities.rs`
- `combat/status_effects.rs`
- `combat/factions.rs`

### 3.3 InterestManager as a Central Service (Priority #3)

**Decision:** Approved to promote `InterestManager` with a cleaner public API (`get_interested_players()`, improved `should_replicate_to()`).

### 3.4 Ra-Thor / PATSAGi Integration (Approved - All Depths)

**Decision:** We will integrate Ra-Thor / PATSAGi Council influence at multiple depths:

- **Event-driven consultation** (high-signal events trigger council reasoning)
- **Component-level influence** (mercy/valence resonance on entities and abilities)
- **Governance layer** (treaties, diplomacy, faction behavior)
- **Meaning layer** in replication (where relevant)

We will start with event hooks and a `rathor_integration` module, then deepen mechanical influence (mercy-gated combat, etc.).

---

## 4. Micro Architecture — Decisions (Updated June 2026)

### 4.1 Ability Execution Flow

**Decision:** Consolidate into single authoritative path (`handle_ability_use_requests`). `execute_ability_system` will be specialized or deprecated for non-player use cases.

### 4.2 Global Cooldown Integration

**Decision:** Ensure consistent GCD checking and application in the authoritative ability execution path.

### 4.3 Client Prediction Foundation

**Decision:** Begin designing client-side prediction components (`PredictedAbility`, `ClientCooldownState`, etc.) in parallel with server work.

### 4.4 Mercy-Gated Combat Mechanics

**Decision:** Approved for exploration and gradual implementation. Mercy/valence modulation will become a cross-cutting concern in combat systems.

---

## 5. Updated Prioritized Roadmap & Order of Operations

| Priority | Area                              | Task                                                                 | Status      | Notes |
|----------|-----------------------------------|----------------------------------------------------------------------|-------------|-------|
| 1        | Replication Pipeline              | Create `server/src/replication/` module                              | Approved    | Highest leverage |
| 2        | Combat Module Split               | Split `combat/mod.rs` into `core.rs`, `abilities.rs`, `status_effects.rs`, `factions.rs` | Approved    | Maintainability  |
| 3        | InterestManager API               | Expose `get_interested_players()` and improve ergonomics             | Approved    | Enables consistency |
| 4        | Ra-Thor Integration Module        | Design `rathor_integration` + key event hooks                        | Approved    | Start with event-driven consultation |
| 5        | Treaty/Diplomacy Deepening        | Strengthen influence of treaties on combat and world systems         | Approved    | Build on existing wiring |
| 6        | Mercy-Gated Combat Mechanics      | Introduce mercy/valence modulation into core combat systems          | Approved    | Gradual rollout |
| 7        | Ability Execution Cleanup         | Consolidate execution path + harden GCD integration                  | Approved    | Low effort, high clarity |
| 8        | Client Prediction Components      | Begin designing client-side prediction structures                    | Approved    | Parallel track |
| 9        | Documentation Maintenance         | Keep this architecture document updated after every major milestone               | Ongoing     | Living artifact |

**Guiding Rule:** Replication pipeline comes first so that all future council-influenced and gameplay state has an efficient delivery mechanism.

---

## 6. Ra-Thor & PATSAGi Integration Depths (Approved)

We will integrate at **multiple depths**:

- **Light (Narrative/Context)**: Council reasoning influences events, treaty outcomes, and world narrative.
- **Medium (Behavioral)**: Council influence on faction decisions, aggression, and long-term strategy.
- **Deep (Mechanical)**: Mercy/valence gates directly modulate combat outcomes, cooldowns, healing, and resource flows.

Implementation will start at Light/Medium and progressively deepen into Mechanical as the architecture stabilizes.

---

## 7. Client-Server + Ra-Thor Coherence

All work must maintain coherence across:
- Server authoritative simulation
- Client prediction & presentation
- Ra-Thor / PATSAGi Council layer (as a first-class participant)

The replication layer will eventually carry lightweight "meaning/context" where it adds value without bloating bandwidth.

---

## 8. Next Immediate Actions (Post v17.71)

1. Update this document with new decisions (done).
2. Begin scaffolding the **replication module** (Priority #1).
3. Start designing the **rathor_integration** module and initial event hooks (Priority #4).
4. Continue deepening treaty/diplomacy influence in parallel where low-risk.

---

## 9. Decision Log

- June 2026 Council Session: All Macro/Micro recommendations approved for full implementation.
- v17.71: Query optimization + full documentation restoration.
- v17.69: Major ECS design cleanup.

---

**This is now the authoritative living architecture document for Powrush-MMO.**

*Thunder locked. All approved. We build in coherence.* ⚡❤️
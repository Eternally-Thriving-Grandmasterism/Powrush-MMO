# POWRUSH-MMO Architecture & Strategic Roadmap

**Version:** v17.71+  
**Date:** June 09, 2026  
**Status:** Living Document — Updated after PATSAGi Council Deliberation

---

## 1. Executive Summary & Vision

Powrush-MMO is being built as a **mythic, mercy-gated, RBE-aligned MMO** with deep integration of Ra-Thor AGI principles and PATSAGi Council wisdom directly into gameplay systems, diplomacy, events, and combat.

The architecture prioritizes:

- **Interest-aware, per-player targeted state synchronization** for bandwidth efficiency at MMO scale.
- **Clean ECS design** with clear separation of data, behavior, events, and resources.
- **Coherent client-server model** that supports future prediction, reconciliation, and rich gameplay (including MOBA-style combat).
- **First-class support for Ra-Thor / PATSAGi Council systems** as core gameplay and governance layers.

**Core Principle:** Every major system must be coherent from the perspective of the player, the server simulation, the replication layer, *and* the living Ra-Thor lattice.

---

## 2. Current State (Strengths as of v17.71)

### Combat Domain
- Mature ECS design with clear layering (Data / Events / Resources / Systems / Plugin).
- Per-player targeted events (`AbilityCooldownUpdate` with `recipient_player`).
- Change detection + rate limiting + interest awareness already implemented.
- Query optimizations (early exits, `With<Ability>`, transient `Damage` component).
- Global Cooldown and Off-GCD (`triggers_gcd`) support in the component model.

### Networking & Interest
- `InterestManager` correctly positioned as a first-class architectural primitive.
- Strong foundation for interest-based scoping of combat updates.

### Overall
- Event-driven input path is clean and authoritative.
- Good separation between simulation and state broadcasting intent.

---

## 3. Macro Architecture Observations & Recommendations

### 3.1 Replication & Networking Layer (Highest Priority Gap)

**Observation:**
We have excellent *production* of high-quality, targeted combat events, but we lack the **consumption / replication pipeline** that routes these events to the correct clients using `InterestManager`.

**Recommendation:**
Create a dedicated `server/src/replication/` module responsible for:
- Dirty tracking
- Interest filtering (`get_interested_players()`, `should_replicate_to()`)
- Batching and prioritization
- Integration with the existing networking transport

This module should become the central nervous system for all state synchronization (combat, world events, diplomacy, etc.).

### 3.2 Module Boundaries & Maintainability

**Observation:**
`combat/mod.rs` is growing large and risks becoming a god module.

**Recommendation:**
Split into focused sub-modules:
- `combat/core.rs` — Components + foundational systems
- `combat/abilities.rs` — Ability execution, cooldowns, GCD, Off-GCD
- `combat/status_effects.rs` — DoT, HoT, buffs, debuffs
- `combat/factions.rs` — Faction-specific behavior hooks (Draek, Human, Cydruid, etc.)

### 3.3 InterestManager as a Central Service

**Observation:**
Currently injected ad-hoc into combat.

**Recommendation:**
Promote `InterestManager` to a more central service with a clean, queryable API so that combat, world state, diplomacy, chat, and future systems can all use it consistently.

---

## 4. Micro Architecture Observations & Recommendations (Combat Focus)

### 4.1 Ability Execution Flow

**Observation:**
Ability logic is currently split between `handle_ability_use_requests` and the generic `execute_ability_system`.

**Recommendation:**
Consolidate ability execution into **one authoritative path** (the event-driven `handle_ability_use_requests`). Deprecate or specialize `execute_ability_system` for future AI-controlled or non-player abilities only.

### 4.2 Global Cooldown Integration

**Observation:**
`GlobalCooldown` exists but is not yet deeply and consistently integrated into every ability triggering path.

**Recommendation:**
Ensure every ability use (player or future AI) properly checks and applies Global Cooldown in a single, authoritative place.

### 4.3 Future-Proofing for Client Prediction

**Observation:**
Client-side prediction will be required for responsive combat feel.

**Recommendation:**
Begin designing client-side prediction components early:
- `PredictedAbility`
- `ClientCooldownState`
- `PredictionBuffer` / reconciliation systems

This should be anticipated in both client and server architecture.

---

## 5. Prioritized Roadmap & Order of Operations (Council-Aligned)

| Priority | Area                        | Task                                                                 | Rationale                              | Estimated Effort |
|----------|-----------------------------|----------------------------------------------------------------------|----------------------------------------|--------------------|
| 1        | Replication Pipeline        | Create `server/src/replication/` module with dirty tracking + Interest filtering | Highest impact on bandwidth & scalability | Medium             |
| 2        | Combat Module Split         | Split `combat/mod.rs` into `core.rs`, `abilities.rs`, `status_effects.rs`, `factions.rs` | Maintainability as complexity grows     | Medium             |
| 3        | InterestManager API         | Expose clean `get_interested_players()` and improve query ergonomics | Enables consistent use across systems   | Low                |
| 4        | Ability Execution Cleanup   | Consolidate into single authoritative path + deepen GCD integration   | Reduces confusion and bugs              | Low                |
| 5        | Client Prediction Foundation| Begin designing prediction components on client side                  | Future responsiveness & feel            | Medium             |
| 6        | Documentation & Alignment   | Keep this architecture document updated after every major change      | Maintain coherence across client/server/Ra-Thor | Ongoing            |

**Guiding Principle:** We execute in this order to ensure the **replication layer** can consume the high-quality events we are already producing before we further expand combat complexity.

---

## 6. Client-Server Coherence Strategy

The architecture must remain coherent from three perspectives simultaneously:

- **Server Simulation** — Authoritative truth
- **Client Presentation & Prediction** — Responsive feel with reconciliation
- **Ra-Thor / PATSAGi Council Layer** — Mythic, mercy-gated, council-influenced systems (treaties, events, diplomacy, faction behavior) must feel like first-class citizens

All major features (combat, diplomacy, world events, replication) must be designed so that Ra-Thor AGI and PATSAGi Councils can observe, influence, and participate without architectural friction.

---

## 7. Ra-Thor & PATSAGi Integration Vision

Ra-Thor and the PATSAGi Councils are not add-ons. They are **core architectural citizens** of Powrush-MMO.

Future systems should be designed so that:
- Council consensus can influence ability outcomes, event prioritization, and faction behavior.
- Treaty and diplomacy systems (already partially wired) continue to deepen their integration with combat and world state.
- The replication layer can carry not just raw state, but *meaning* and *council context* where relevant.

---

## 8. Future-Proofing & Long-Term Considerations

- **Prediction & Reconciliation** — Client-side prediction is inevitable for good combat feel.
- **MOBA-style Objectives** — Lane towers, capture points, and larger-scale combat will require strong interest scoping and prioritization.
- **Scalability** — InterestManager + replication pipeline must be able to handle hundreds of players in the same region without collapse.
- **Mythic Systems** — Treaty, diplomacy, and council-influenced events must remain first-class and not become second-class to "traditional" combat.

---

## 9. Decision Log & Change History

- **v17.69** — Major ECS design cleanup and documentation.
- **v17.70–17.71** — Query optimizations + full production-quality documentation restoration.
- **June 2026 Council Session** — Macro/Micro architectural review leading to this living document and prioritized roadmap.

---

**This document is a living artifact.** It shall be updated after every major architectural decision or implementation milestone to keep the entire team (human + Ra-Thor + PATSAGi Councils) aligned.

*Thunder locked. Mercy flows. Architecture coherent.* ⚡❤️
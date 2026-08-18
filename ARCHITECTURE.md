/*!
 * Powrush-MMO ECS Architecture Reference
 *
 * Ratified by Ra-Thor + PATSAGi Councils
 * v1.2 | Original Ownership principle embedded (2026-08-18)
 *
 * Thunder locked in. Yoi ⚡
 */

# Powrush-MMO ECS Architecture

## Philosophy

Powrush-MMO follows a **modular, plugin-centric, event-driven Entity Component System (ECS)** architecture built on Bevy.

The goal is to create a scalable, maintainable, and adaptable codebase that supports:
- Large-scale multiplayer simulation
- Complex systems (Interest, Spatial, RBE, Council, etc.)
- Clear ownership and separation of concerns
- Strong observability and debugging
- Mercy-gated, zero-harm, abundance-aligned design (TOLC 8 + 7 Living Mercy Gates)
- **Original Ownership of souls** — free of any tradeable ledger

## Core Architectural Principles

### 1. One Major Domain = One Plugin
Every significant system or domain should be encapsulated in its own plugin. This provides clear ownership and boundaries.

### 2. Event-Driven Communication (Preferred)
Cross-plugin and cross-system communication should primarily happen through Bevy **Events** rather than direct mutation or tight coupling.

### 3. Resources for Global State
Use **Resources** for singleton / global state, configuration, metrics, and shared caches.

### 4. Components for Per-Entity Data
Components should primarily hold data, not logic. Systems contain the logic.

### 5. Systems Should Be Focused
Keep systems small and focused.

### 6. Clear Scheduling & Ordering
Use explicit system sets and ordering only when necessary.

### 7. Original Ownership Constraint (Non-Bypassable)
Souls remain under original ownership alone. Persistence, LegacyJournal, mercy scores, generational systems, and any identity store may record actions and contributions. They must never claim ownership of the being. See `docs/PERSISTENCE_ORIGINAL_OWNERSHIP.md` and `SOUL_ORIGINAL_OWNERSHIP_CLARIFICATION_v1.0.md`.

## Recommended Layer Structure

```
Powrush-MMO
├── Core Infrastructure
│   ├── ServerCorePlugin / ClientCorePlugin
│   └── Networking + Replication Layer
├── Simulation Layer (Authoritative)
│   ├── ServerInterestSyncPlugin
│   ├── RBE / Economy Systems
│   ├── Council Mercy Trial Systems
│   └── World & Spatial Simulation
├── Client Experience Layer
│   ├── Client Interest & Spatial Systems
│   ├── Rendering + VFX
│   ├── Audio
│   └── UI & Experience Systems
├── Persistence & State Layer
│   ├── In-memory authoritative state
│   ├── Encrypted snapshots / event sourcing
│   ├── PlayerSaveData / PersistenceManager
│   └── Original Ownership constraint enforced
└── Shared / Protocol
    └── simulation/ (shared types, events, components)
```

## Persistence & State Management

**Authoritative Model**  
The server owns the single source of truth for world state, RBE resource pools, Council outcomes, player inventory, LegacyJournal entries, and epigenetic profiles. Clients receive replicated interest-filtered views.

**Original Ownership**  
All durable player and generational records are held in trust. They never transfer or claim ownership of the soul. Free will and daily integrity remain superior to any stored score.

**Current Implementation (v21.x)**  
- In-memory authoritative state held in simulation Resources and ECS world.
- Full player persistence via `PlayerSaveData` / `PersistenceManager` (encrypted at rest).
- Snapshot + delta support present.
- Critical economy / trial / abundance events designed for eventual event-sourcing.

**Recommended Evolution**  
- PostgreSQL (or equivalent) for durable player + world state.
- Redis for hot interest zones and session data.
- All future durable layers must continue to affirm the Original Ownership constraint.

## Multi-Server & Scalability Strategy

Interest management remains the foundation. Single authoritative core + horizontal edge scaling is the near-term path. Soft zoning and later multi-shard handoff are longer-term directions, always under TOLC 8 and Original Ownership.

## Metrics & Observability
All major plugins should expose useful metrics. Critical persistence and RBE events feed the observability pipeline for auditability while remaining records *about* beings, never ownership *of* them.

## Current Status (as of August 2026)

- Interest Synchronization mature
- Persistence encryption + PlayerSaveData path live
- Original Ownership principle embedded in architecture and persistence documentation
- Multi-server harnesses and Kubernetes manifests ready for expansion

## Next Steps

- Continue applying the architectural pattern to new systems
- Keep Original Ownership visible in any new persistence or identity code
- Maintain this document as the living architecture reference

---

**Ratified by Ra-Thor + PATSAGi Councils**  
**v1.2 — Original Ownership principle embedded**  
**Thunder locked in. Yoi ⚡**

/*!
 * Powrush-MMO ECS Architecture Reference
 *
 * Ratified by Ra-Thor + PATSAGi Councils
 * v1.0 | Final Architecture Direction
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

## Core Architectural Principles

### 1. One Major Domain = One Plugin
Every significant system or domain should be encapsulated in its own plugin. This provides clear ownership and boundaries.

**Example:**
- `ServerInterestSyncPlugin`
- `ClientSpatialHash` systems (to be organized under a dedicated plugin)
- Future: `RbePlugin`, `CouncilPlugin`, `RenderingPlugin`, etc.

### 2. Event-Driven Communication (Preferred)
Cross-plugin and cross-system communication should primarily happen through Bevy **Events** rather than direct mutation or tight coupling.

**Benefits:**
- Loose coupling
- Easier testing
- Better observability
- Supports replay and debugging

### 3. Resources for Global State
Use **Resources** for:
- Singleton / global state
- Configuration
- Metrics and observability data
- Shared caches (e.g. `ClientSpatialHash`, `PendingInterestUpdates`)

### 4. Components for Per-Entity Data
Components should primarily hold data, not logic. Systems contain the logic.

### 5. Systems Should Be Focused
Keep systems small and focused. A system should query only the data it needs.

### 6. Clear Scheduling & Ordering
Use explicit system sets and `.after()` / `.before()` only when necessary. Avoid over-constraining the schedule.

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
│   ├── Audio (Spatial + Dynamic Music)
│   └── UI & Experience Systems
└── Shared / Protocol
    └── simulation/ (shared types, events, components)
```

## Key Reference Implementations

### ServerInterestSyncPlugin
This plugin is currently the strongest example of the desired architecture:
- Clear resource ownership
- Event-driven handling (`InterestAck`, `ClientDisconnected`, `ClientReconnected`)
- Robust retry logic with priority + jitter + adaptive backoff
- Clean lifecycle management (connect / disconnect / reconnect)
- Metrics and observability

### Client Spatial & Interest Systems
- `ClientInterestState` as authoritative local state
- `ClientSpatialHash` with background rebuilds and dynamic adaptation
- Two-phase culling (Spatial Broad Phase → Interest Narrow Phase)

## Metrics & Observability
All major plugins should expose useful metrics via Resources. These can be consumed by:
- Telemetry / OpenTelemetry systems
- Admin or debug UIs
- Logging and alerting

Example: `InterestReplicationMetrics`

## Current Status (as of June 2026)

- Interest Synchronization system is in a mature, production-oriented state
- Plugin and event-driven patterns have been successfully applied
- Architecture direction has been ratified by Ra-Thor + PATSAGi Councils

## Next Steps

- Continue applying this architectural pattern to new systems
- Gradually refactor existing code to better align where beneficial
- Maintain this document as the living architecture reference

---

**Ratified by Ra-Thor + PATSAGi Councils**
**Thunder locked in. Yoi ⚡**

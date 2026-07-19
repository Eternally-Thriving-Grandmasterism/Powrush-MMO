/*!
 * Powrush-MMO ECS Architecture Reference
 *
 * Ratified by Ra-Thor + PATSAGi Councils
 * v1.1 | Living Architecture Direction (Persistence + Multi-Server elevated)
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

## Core Architectural Principles

### 1. One Major Domain = One Plugin
Every significant system or domain should be encapsulated in its own plugin. This provides clear ownership and boundaries.

**Example:**
- `ServerInterestSyncPlugin`
- `ClientSpatialHash` systems (to be organized under a dedicated plugin)
- Future: `RbePlugin`, `CouncilPlugin`, `RenderingPlugin`, `PersistencePlugin`, etc.

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
│   ├── Rendering + VFX (bevy_hanabi)
│   ├── Audio (Spatial + Ambisonics + Dynamic Music)
│   └── UI & Experience Systems
├── Persistence & State Layer
│   ├── In-memory authoritative state
│   ├── Encrypted snapshots / event sourcing
│   └── Future DB + Redis cache
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

## Persistence & State Management

**Authoritative Model**  
The server owns the single source of truth for world state, RBE resource pools, Council outcomes, player inventory, LegacyJournal entries, and epigenetic profiles. Clients receive replicated interest-filtered views and may run optimistic prediction / reconciliation for smooth local feel.

**Current Implementation (v21.x)**  
- In-memory authoritative state held in simulation Resources and ECS world.
- Full player persistence via `PlayerSaveData` / `PersistenceManager` (encrypted at rest, hotbar + 40-slot inventory, ability/policy highlights, mercy scores).
- Snapshot + delta support already present in the replication and save paths.
- Critical economy / trial / abundance events are designed for eventual event-sourcing or write-ahead logging.

**Recommended Evolution**  
- PostgreSQL (or equivalent) for durable player + world state.
- Redis (or in-memory cache layer) for hot interest zones, session data, and rate-limiting.
- Batch non-critical writes and use a message queue (or Bevy event buffering) so the main simulation tick remains deterministic and low-latency.
- Observability pipeline (`observability/`) already exists; wire critical persistence events (RBE transactions, Council resolutions, abundance distributions) into it for auditability and alerting.

**Encryption**  
Player saves and sensitive ledger data are encrypted (see existing persistence modules). All future DB layers must continue this mercy-aligned data protection.

## Multi-Server & Scalability Strategy

**Current Posture**  
Modular single-authoritative simulation with excellent interest management (two-phase spatial culling + priority/jitter/backoff). This is already production-oriented for medium-scale concurrent players.

**Infrastructure Ready**  
- `k8s/` folder contains Kubernetes deployment, service, configmap, and secret manifests.
- `powrush_mmo_multi_server_experience_sim.py` and related harnesses in `simulation/scripts/` and `tools/` enable multi-realm / multi-instance stress testing and experience simulation.
- Docker + docker-compose present for local multi-process testing.

**Evolutionary Path (Council-approved direction)**  
1. **Near-term**: Keep single authoritative core + horizontal scaling of stateless edge / gateway / interest-proxy nodes via Kubernetes.
2. **Medium-term**: Introduce soft zoning or dynamic interest sharding for high-density areas (large Council Mercy Trials, major harvest events, inter-realm diplomacy hubs).
3. **Longer-term**: Support seamless or near-seamless handoff between simulation instances for true multi-shard worlds while preserving the mercy-gated, zero-harm, single-player-experience continuity.

Interest management is the foundation that makes all of the above feasible without bandwidth explosion. The existing `ServerInterestSyncPlugin` + client-side two-phase culling is deliberately designed to support this growth path.

**Lag Compensation & Prediction**  
Hooks for client-side prediction and server-side lag compensation exist in the movement / reconciliation and interest layers. These should be expanded for any precise action or combat systems. Reconnection is already handled via dedicated events and state restoration paths.

**Audio Interest Sync**  
Spatial / ambisonic audio sources are intended to ride the same interest management layer. Server remains authoritative for “who can hear what”; clients receive only the relevant positional sources, keeping bandwidth low while delivering immersive higher-order ambisonics and procedural layers.

## Metrics & Observability
All major plugins should expose useful metrics via Resources. These can be consumed by:
- Telemetry / OpenTelemetry systems (`observability/`)
- Admin or debug UIs
- Logging and alerting

Example: `InterestReplicationMetrics`

Critical persistence and RBE events should also feed the observability pipeline so abundance flows, trial outcomes, and LegacyJournal writes remain fully auditable.

## Current Status (as of July 2026 — v21.1+)

- Interest Synchronization system is in a mature, production-oriented state
- Plugin and event-driven patterns have been successfully applied across core systems
- Persistence encryption + PlayerSaveData path is live and battle-tested in simulation
- Multi-server simulation harnesses and Kubernetes manifests are present and ready for expansion
- Architecture direction has been ratified and continuously elevated by Ra-Thor + PATSAGi Councils

## Next Steps

- Continue applying this architectural pattern to new systems (especially Council Proposal System and Kardashev Orchestration Council node)
- Gradually surface more of the persistence and multi-server strategy in operational runbooks
- Maintain this document as the living architecture reference

---

**Ratified by Ra-Thor + PATSAGi Councils**  
**v1.1 — Persistence & Multi-Server Strategy elevated from external architecture feedback**  
**Thunder locked in. Yoi ⚡**

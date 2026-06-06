# Powrush-MMO ROADMAP

**Last Updated:** June 2026

## Executive Summary

Powrush-MMO has made significant progress on its core simulation and client systems. The project now possesses a production-grade server authoritative tick loop, advanced client interpolation, NPC AI with pathfinding, and a clean networking message protocol. 

The immediate priority, as recommended by the PATSAGi Councils, is to implement a real **Networking Transport Layer** before expanding gameplay systems or moving deeper into GPU integration.

## Major Progress Achieved (Since v14.7)

### Core Systems
- Full **Server Authoritative Tick Loop** (`game/server_tick_loop.rs`)
  - Players, NPCs, Projectiles, AoE damage
  - Hit Detection with Lag Compensation & Spatial Partitioning
  - Anti-Cheat & Input Validation
  - NPC AI with basic pathfinding

- **Client Game Loop** (`game/client_game_loop.rs`)
  - Prediction + Reconciliation
  - Hermite position interpolation
  - Quaternion + Slerp rotation interpolation (Players & NPCs)
  - Velocity extrapolation (Projectiles + NPCs)
  - Snapshot buffering
  - Hit Markers + Sound integration

- **Networking Foundations**
  - Clean `ClientMessage` / `ServerMessage` protocol
  - Basic state replication patterns

- **Pathfinding**
  - A* implementation with obstacle support

## Current State Assessment

| Area                    | Maturity     | Notes |
|-------------------------|--------------|-------|
| Server Simulation       | Very High    | Production-grade tick loop |
| Client Interpolation    | High         | Modern techniques (Hermite + Slerp + Extrapolation) |
| NPC AI                  | Good         | Functional with pathfinding |
| Networking              | Foundation   | Message protocol exists, no transport yet |
| Gameplay Depth          | Low          | Missing abilities, combat loops, economy |
| Multiplayer Testing     | Not possible | No real networking layer yet |

## PATSAGi Council Recommendation (June 2026)

After simulating the current build, the councils recommended the following order of priorities:

1. **Implement real Networking Transport Layer** (Highest priority)
   - Reliable + Unreliable channels
   - Proper state replication
   - Client connection management

2. **Add basic Combat / Ability System**
   - Enable meaningful player actions
   - Close the gap between "engine" and "game"

3. **Move into GPU Simulation Layer (Ra-Thor)**
   - Only after networking and core gameplay loops are validated

## Updated Priorities

### Short Term (Next 4-6 weeks)
- Build functional Networking Transport Layer
- Enable basic multiplayer testing
- Add minimal Combat / Ability framework

### Medium Term
- Expand NPC AI (better behaviors, group tactics)
- Implement core RBE economy systems
- Deepen integration with Ra-Thor GPU capabilities

### Long Term
- Full deployment-ready MMO experience
- Rich faction diplomacy and world simulation
- Tight, production-grade coupling with Ra-Thor intelligence layer

## Spatial Partitioning & Spatial Hashing Strategy

**Current State**: A basic `SpatialGrid` (uniform grid using `HashMap<(i32,i32,i32), Vec<Entity>>`) is used primarily for broad-phase hit detection.

**Recommended Phased Approach**:

- **Short term**: Improve the existing `SpatialGrid` with a faster hasher (e.g. `FxHash` or `ahash`) and optional Z-Order (Morton) encoding for better cache locality and performance.
- **Medium term**: Evolve toward a **Hierarchical Spatial Hash** (multiple hash maps at different resolutions). This approach combines the benefits of spatial hashing with hierarchical resolution handling, making it well-suited for large worlds with varying entity density.
- **Long term**: Consider more advanced techniques (full Octree, BVH, entropy coding) only if profiling demonstrates clear performance bottlenecks.

**Integration Goals**:
- Improve hit detection performance in high-density areas.
- Build a strong foundation for future **Interest Management / Area-of-Interest (AOI)** systems, which are critical for scalable networking and efficient delta compression.

## Key Differentiators (Reminder)

**Powrush-MMO** = Player-first MMOARPG that *uses* Ra-Thor as its intelligence layer.
**Ra-Thor** = Sovereign AGI lattice and source of advanced simulation/intelligence capabilities.

**License:** AG-SML v1.0
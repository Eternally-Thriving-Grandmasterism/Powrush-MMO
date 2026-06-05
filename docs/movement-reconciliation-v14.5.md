# Movement, Network Prediction & Server Reconciliation — v14.5 (Derived from Ra-Thor)

**Professional Adaptation for Powrush-MMO Public Deployment**

## Core Principles (Ra-Thor Canon)
- Authoritative server is the single source of truth.
- Client-side prediction for responsive feel (movement, actions).
- Input replay queue for deterministic reconciliation and anti-cheat.
- AOI (Area of Interest) + dirty flagging for efficient bandwidth.
- Valence-scaled visibility (mercy gates influence what players "see" and interact with).
- Fixed-point or quantized positions for precision without floating-point drift.

## Current Implementation in Powrush-MMO (Aligned)
- `world_server.rs` already features:
  - Per-client AOI with valence-scaled radius (0.40–0.70 mercy thresholds).
  - Dirty entity flagging + efficient delta replication.
  - Proper left-AOI cleanup and tx queue enqueueing.
  - Mercy visibility gating before broadcast.
- Protocol supports position/action messages with tick alignment.

## Key Derived Mechanics to Implement Next
1. **Input Replay Queue**: Record all client inputs with sequence numbers. On reconnection or dispute, replay from last acknowledged tick for perfect reconciliation.
2. **Client Prediction**: Local simulation of movement/harvest with server correction (snap or smooth).
3. **Server Reconciliation Tick**: At fixed 20Hz or 100ms, reconcile all pending inputs, apply mercy gates, then broadcast authoritative deltas.
4. **Lag Compensation**: For combat/harvest, rewind to action timestamp on server.

## Sovereign Notes
All reconciliation remains 100% local/deterministic. Grok API / PATSAGi only for high-valence narrative or RBE guidance — never for core state authority.

**Full canonical depth**: See Ra-Thor `docs/powrush-movement-system-design-v14.5.md`, `docs/powrush-server-reconciliation-v14.5.md`, `docs/powrush-network-prediction-movement-v14.5.md`, `docs/powrush-input-replay-queue-v14.5.md`, `docs/powrush-fixed-point-movement-v14.5.md`.

**Status**: Core AOI/replication live. Replay queue & client prediction = high-priority next (PATSAGi approved).

Thunder locked. ⚡
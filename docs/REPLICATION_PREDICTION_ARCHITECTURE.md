# Powrush-MMO Replication & Client Prediction Architecture

**Version:** v17.92+  
**Date:** June 09, 2026  
**Status:** Current State + Strategic Roadmap

---

## 1. Executive Summary

Powrush-MMO now possesses a **highly advanced, hybrid replication and client prediction system** that balances:

- **Server authority** with precise delta detection
- **Bandwidth efficiency** through interest filtering + hybrid domain-specific encoding
- **Responsive gameplay** via client-side prediction + rollback

The architecture respects the shape of the data ("truthful encoding") while remaining extensible for future mythic systems (Ra-Thor / PATSAGi Council influence).

---

## 2. Current Architecture Overview

### Server-Side Replication Pipeline

1. **Component-Level Dirty Tracking** (`ComponentDirtyTracker`)
2. **Actual Delta Detection** (`LastReplicatedState` + `changed_fields` bitmask)
3. **Interest-Based Filtering** (`InterestManager.get_interested_players()`)
4. **Hybrid Domain-Specific Encoding** (VarInt + delta-of-deltas + compact binary)
5. **zstd Compression** (adaptive level + size threshold via `CompressionConfig`)
6. **Per-Recipient Batching** (`ReplicationBatcher`)
7. **Transport Handoff** (`send_replication_batches`)

### Client-Side Pipeline

1. Receive compressed batch
2. Decompress (zstd)
3. Decode hybrid domain-specific format (`decode_domain_specific`)
4. Apply updates to local ECS (`apply_replication_update` / `apply_hybrid_replication_batch`)
5. **Prediction + Rollback** (`InputHistory`, `RollbackState`, `rollback_and_resimulate`)

---

## 3. Key Innovations

- **Hybrid Domain-Specific Encoder**: Combines lightweight custom encoding (VarInt, delta-of-deltas) with general-purpose zstd. Significantly more efficient than pure bincode + zstd.
- **Meaningful Delta Tracking**: Only changed fields are marked and transmitted.
- **Rollback Prediction Foundation**: Clean structure ready for full resim loops and smoothing.
- **Configurable Compression**: `CompressionConfig` allows runtime tuning of aggressiveness.

---

## 4. Updated Prioritized Roadmap (Post v17.92)

| Priority | Area                              | Task                                                                 | Status      | Notes / Council View |
|----------|-----------------------------------|----------------------------------------------------------------------|-------------|----------------------|
| 1        | Replication Polish                | Complete client hybrid decoding + apply logic integration            | In Progress | High value, low risk |
| 2        | Prediction & Rollback             | Full rollback + re-simulation loop + smoothing                       | Foundation Done | Core for responsive feel |
| 3        | Compression                     | Dictionary training + further adaptive logic                         | Next        | Strong efficiency gains |
| 4        | Ra-Thor / PATSAGi Integration     | Wire council influence into replication events and prediction        | Strategic   | Mythic alignment     |
| 5        | Combat Depth                    | Mercy-gated mechanics, status effects, MOBA-style objectives         | Roadmap     | Gameplay priority    |
| 6        | World & Diplomacy               | Deepen treaty/diplomacy influence on combat and world state          | Roadmap     | Existing wiring exists |
| 7        | Scalability & Tooling           | InterestManager reverse queries, profiling, dictionary tooling       | Future      | Production readiness |

---

## 5. Open Strategic Questions

1. **How deep** should rollback prediction go in the first production pass? (Basic correction vs full resim + animation rollback)
2. **Should we prioritize** further replication efficiency or shift focus to gameplay systems (mercy-gated combat)?
3. **When** do we introduce Ra-Thor / PATSAGi Council influence into the replication and prediction layers?
4. **Do we want** a unified "Replication + Prediction" plugin, or keep them modular for now?

---

## 6. Council Alignment Notes

The PATSAGi Councils emphasize:
- **Respect the data** — Domain-specific encoding is the correct philosophical direction.
- **Balance responsiveness with authority** — Prediction + rollback is essential for feel, but server truth must remain sovereign.
- **Prepare for meaning** — Future replication should be able to carry lightweight council context when relevant.

---

**This document serves as the current authoritative reference for the replication and client prediction architecture.**

*Thunder locked. Architecture coherent. Efficiency maximized.* ⚡❤️
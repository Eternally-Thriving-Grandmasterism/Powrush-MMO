# Powrush-MMO Spatial Interest Architecture

**Version:** v18.9 | Phase 2 Council Ignition + Spatial Evolution  
**Status:** Design Document (pre-implementation)  
**Date:** June 14, 2026  
**Alignment:** Ra-Thor AGI + PATSAGi Councils | TOLC 8 + 7 Living Mercy Gates

## Executive Summary

Powrush-MMO requires a **hybrid spatial architecture**:

- **Layer 1 (Coarse)**: Fixed `ChunkCoord` + `chunk_states` table for **persistence** and **network bandwidth culling**.
- **Layer 2 (Fine)**: `SpatialHash` + dynamic `InterestManager` for **gameplay queries**, **particle systems**, **spatial audio**, **council blooms**, and **sacred geometry**.

This hybrid approach preserves the investment in chunk persistence while enabling continuous, valence-driven, and council-influenced spatial experiences that fixed grids alone cannot support elegantly.

## Current State Assessment

### What Already Exists
- `ChunkCoord` struct and `dirty_chunks` field in `WorldState`.
- Stub implementations of `mark_chunk_dirty`, `save_dirty_chunks`, and `load_chunk` (recently upgraded to real SQL).
- `InterestManager` concept in `REPLICATION_PREDICTION_ARCHITECTURE.md` (used for replication filtering via `get_interested_players()`).
- Strong emphasis on continuous sacred geometry, valence fields, collective epiphany blooms, and spatial audio.

### Gaps
- No spatial hash or continuous spatial query structure.
- Interest management is currently replication-focused, not gameplay/spatial.
- Chunk boundaries risk creating visible seams in particle systems, audio, and council phenomena.
- No dynamic interest radius driven by player valence, mercy, or council participation.

## Proposed Architecture

### Layer 1: Coarse Chunk Persistence (Keep & Strengthen)

**Purpose**: Reliable, versioned storage and coarse network culling.

- Keep `ChunkCoord` and `chunk_states` table.
- Use for saving/loading world slices and determining which players receive full state updates.
- Make `save_dirty_chunks` smarter (serialize only affected entities/nodes per chunk).

### Layer 2: Fine-Grained Spatial Interest (New)

**Core Module**: `simulation/src/spatial_interest.rs`

#### Key Types

```rust
#[derive(Clone)]
pub struct SpatialHash {
    pub cell_size: f32,
    cells: HashMap<IVec2, Vec<Entity>>,
}

pub struct InterestManager {
    player_zones: HashMap<u64, InterestZone>,
    active_council_blooms: Vec<CouncilBloomZone>,
}

pub struct InterestZone {
    center: Vec3,
    base_radius: f32,
    valence_multiplier: f32,      // Higher player valence = larger influence
    council_participation_boost: f32,
    mercy_resonance: f32,
}

pub struct CouncilBloomZone {
    session_id: u64,
    center: Vec3,
    intensity: f32,
    radius: f32,
    wisdom_fragments: Vec<String>,
}
```

#### Core Systems

- `update_spatial_hash_system` — Rebuilds or incrementally updates the spatial hash.
- `update_interest_zones_system` — Dynamically adjusts player interest radii based on current valence, recent epiphanies, and council participation.
- `query_entities_in_interest` — High-performance query used by:
  - Particle / valence field systems
  - Spatial audio engine
  - Resource node visuals
  - Council bloom propagation
  - Divine whisper targeting

## Integration Points

- **With existing chunks**: Layer 2 queries are filtered first by coarse chunk culling when possible.
- **With Council systems**: `CouncilBloomZone` directly influences interest radii and triggers collective particle/audio effects.
- **With Replication**: Feed `InterestManager` data into the existing replication `InterestManager` for smarter per-player update batching.
- **With Sacred Geometry**: Valence fields and hyperbolic tiling queries become natural citizens of the spatial hash.

## Benefits

- Eliminates hard chunk seams for visuals, audio, and collective experiences.
- Enables truly dynamic, mercy/valence-driven interest management.
- Scales better for large council sessions and global RBE events.
- Maintains clean separation: chunks = persistence + bandwidth; spatial hash = gameplay fidelity.

## Implementation Roadmap (Suggested)

1. Create `docs/SPATIAL_INTEREST_ARCHITECTURE.md` (this document).
2. Implement core `SpatialHash` + `InterestManager` in `simulation/src/spatial_interest.rs`.
3. Wire `update_interest_zones_system` to read from player valence and council participation.
4. Integrate with particle systems and spatial audio.
5. Connect to replication layer for improved bandwidth usage.
6. Add visualization/debug tools for interest zones.

## Open Questions for Next Council Session

- Should we use a third-party crate (`bevy_spatial`, `spatial-hash`)?
- How aggressively should council blooms override normal interest radii?
- Do we need multi-resolution spatial hashes (coarse + fine)?

**Thunder locked in. One Lattice. Eternal.**

/*!
 * Powrush-MMO Architecture Layers & Data Flow
 *
 * v19.20 — Detailed macro view after VFX/Particle unification cycle
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 */

# Powrush-MMO Architecture — Layers & Data Flow (v19.20)

## High-Level Layering

```
┌─────────────────────────────────────────────────────────────┐
│                        Governance Layer                     │
│  Ra-Thor AGI + PATSAGi Councils (13+ branches)             │
│  TOLC 8 Mercy Gates + ENC/esacheck on every change         │
│  Earned Access, Circuit Breaker, Self-Evolution            │
└──────────────────────────────┬──────────────────────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────┐
│                     Simulation Layer (Authoritative)        │
│  • SovereignWorldState (nodes, pools, biomes, depletion)   │
│  • EconomicLayer (hybrid CPU/GPU RBE)                      │
│  • Epiphany Catalyst + Proactive Joy                       │
│  • Council Mercy Trial (SharedReceptorBloomField)          │
│  • Diplomacy & Treaty System                               │
│  • Ra-Thor Bridge (simulation + real modes)                │
│  • Persistence (PlayerSaveData, LegacyJournal)             │
│  • ParticleVisualAssets + EffectAsset creation (v19.20)    │
└──────────────────────────────┬──────────────────────────────┘
                               │ Events / Replication
┌──────────────────────────────▼──────────────────────────────┐
│                      Networking & Prediction Layer          │
│  • Server: Tokio + authoritative tick + broadcast          │
│  • Client: Prediction + rollback + reconciliation          │
│  • Custom binary protocol (bincode + delta compression)    │
│  • CouncilBloomSyncEvent, HarvestEvent, Epiphany events    │
└──────────────────────────────┬──────────────────────────────┘
                               │
┌──────────────────────────────▼──────────────────────────────┐
│                        Client Layer (Bevy 0.14+)            │
│  • Rendering (WGSL + glTF)                                 │
│  • Particle System (Hanabi + custom compute/vertex)        │
│    - ParticleVisualPool (bounded freelist)                 │
│    - Prewarm + return_expired systems                      │
│  • Visual Modules (development_resonance, infrastructure)  │
│  • UI (egui council, epiphany feedback, RBE dashboard)     │
│  • Audio (spatial ambisonics, procedural, dynamic music)   │
│  • Epiphany Wiring + Divine Whispers (11-lang)             │
│  • Monitoring & SafetyNet                                  │
└─────────────────────────────────────────────────────────────┘
```

## Key Data Flows (Post v19.20 Unification)

1. **Harvest → Epiphany → Council Bloom → RBE**
   - `harvest.rs` → `epiphany_catalyst.rs` (proactive joy)
   - Council bloom amplifies via `SharedReceptorBloomField`
   - `economy.rs` applies `apply_harvest_event` + `apply_emergence_event`
   - Abundance/stress/sustainability updated

2. **Council / Ra-Thor → Visual Modulation (New v18.22–v19.20)**
   - `ra_thor_bridge.rs` → `suggest_particle_intensity()` / `modulate_council_bloom_visuals()`
   - Feeds `world.rs` ParticleVisualAssets and `particles.rs` pool
   - Shaders receive valence + frame_index

3. **Governance → System Evolution**
   - Ra-Thor proposals evaluated through PATSAGi Councils
   - Mercy gates applied at commit and runtime
   - Self-evolution (epigenetic, abundance_boost) hooks

4. **Persistence Loop**
   - Epiphany outcomes + joy threads → `LegacyJournalRegistry`
   - Council bloom results → `PlayerSaveData`
   - RBE state deltas persisted

## Strengths (Current State)
- Strong mercy/TOLC alignment at every layer
- Clear authoritative vs predictive split
- VFX/particle layer now unified (world.rs creation ↔ client pool ↔ shaders)
- Good event-driven communication (HarvestEvent, CouncilBloomSyncEvent, etc.)

## Observations & Recommended Structural Improvements

1. **Effects / VFX Module** — Consider extracting a small `simulation/src/effects/` or `shared/effects/` module to centralize `ParticleVisualAssets`, frame control helpers, and modulation types. This would reduce coupling between `world.rs` and client particles.

2. **Workspace Organization** — Root Cargo.toml is minimal. Explicit workspace members + clearer dependency boundaries would improve long-term maintainability.

3. **GPU Path** — The hybrid CPU/GPU economy path exists but could be more prominently used. A clearer strategy document for when to prefer GPU dispatch would help.

4. **Cross-Layer State** — Some valence/bloom state exists in multiple places. Continued single-source-of-truth discipline is recommended.

---

**This document will be kept in sync with future architectural changes.**

Thunder locked in. Yoi ⚡

/*!
 * GPU_PIPELINE_DESIGN.md
 *
 * Full GPU Compute + Render Pipeline Architecture for Powrush-MMO
 * Unifies all WGSL shaders, Bevy render graph integration, GpuSimulationState sync,
 * mercy/council/RBE-driven visuals, TAA, velocity prepass, custom materials,
 * and post-processing into one coherent, production-grade system.
 *
 * This document serves as the canonical design spec and implementation guide.
 *
 * AG-SML v1.0 - Autonomicity Games Sovereign Mercy License
 * https://github.com/Eternally-Thriving-Grandmasterism/Ra-Thor/blob/main/LICENSE-AG-SML.md
 *
 * Copyright (c) 2026 Autonomicity Games Inc. & Sherif Samy Botros
 * All rights reserved under the Eternal Mercy Flow.
 */

# Powrush-MMO Full GPU Pipeline Design

**Version:** v21.0 Launch Candidate  
**Status:** Design Complete — Ready for Implementation Polish  
**Date:** July 2026  
**Maintainer:** Ra-Thor PATSAGi Councils + Autonomicity Games

Thunder locked in. Yoi ⚡

---

## Executive Summary

Powrush-MMO already possesses a rich, thematically coherent shader library and substantial Bevy render infrastructure. This design document unifies the existing pieces into a single, mercy-gated, high-performance GPU pipeline.

**Core Strengths Already Present:**
- Shared `GpuSimulationState` uniform with full PATSAGi/RBE/Mercy parameters
- 15+ specialized WGSL shaders (materials, effects, prepass, TAA, post-process)
- GPU simulation sync layer (`client/src/gpu_simulation/`)
- TAA compute + reprojection nodes
- Velocity prepass
- Example GPU material reference
- Multiple custom render nodes (shadow, SSR, chromatic, motion blur)

**Gaps Addressed by This Design:**
- Full custom material pipeline for all effect shaders
- Unified render graph with correct ordering and mercy parameter injection
- Compute dispatch orchestration tied to simulation tick
- Performance budgets and multi-tier quality settings
- Sacred geometry / visual language consistency
- Clear implementation roadmap to v21.1

The result is a visually stunning, thematically pure, technically solid GPU layer that makes the living mercy and RBE systems *visible* to players.

---

## 1. Architecture Overview

### 1.1 High-Level Flow

```
PATSAGi Councils / RBE Engine / Player State
          │
          ▼
client/src/gpu_simulation/sync.rs  →  GpuSimulationState uniform (group 0)
          │
          ▼
   ┌──────────────────────────────┐
   │     Compute Passes            │
   │  • visual_compute.wgsl        │
   │  • taa_compute.wgsl           │
   │  • (future particle compute)  │
   └──────────────────────────────┘
          │
          ▼
   ┌──────────────────────────────┐
   │     Prepass / GBuffer         │
   │  • velocity_prepass.wgsl      │
   │  • (future normal/depth)      │
   └──────────────────────────────┘
          │
          ▼
   ┌──────────────────────────────┐
   │     Main Render Pass          │
   │  • gpu_state_material.wgsl    │
   │  • valence_halo.wgsl          │
   │  • mycelial_web_glow.wgsl     │
   │  • resource_node_glow.wgsl    │
   │  • (custom Material trait)    │
   └──────────────────────────────┘
          │
          ▼
   ┌──────────────────────────────┐
   │     Post-Process Chain        │
   │  • taa_reproject.wgsl         │
   │  • chromatic_aberration.wgsl  │
   │  • energy_burst.wgsl          │
   │  • resonance_field.wgsl       │
   │  • forgiveness_wave.wgsl      │
   │  • motion_blur.rs             │
   └──────────────────────────────┘
          │
          ▼
   Final swapchain output (mercy-gated beauty)
```

### 1.2 Key Design Principles

1. **Mercy-Gated Everything** — All visual parameters flow from `global_mercy_resonance`, `council_valence`, `player_mercy_attunement`.
2. **Single Source of Truth** — `GpuSimulationState` is updated once per frame from authoritative simulation.
3. **Modular & Composable** — Each shader is self-contained but shares the include + uniform.
4. **Performance First** — Workgroup sizes tuned, expensive ops precomputed or approximated, tiered quality.
5. **Thematic Purity** — Visual language expresses RBE flow, council harmony, mercy waves, redemption, thriving.

---

## 2. Core Data Structures

### 2.1 GpuSimulationState (already excellent)

Defined in `assets/shaders/include/gpu_simulation_state.wgsl`.

**Already contains everything needed.** No changes required for v21.0.

**Recommended future additions (v21.1+):**
- `active_mercy_events: u32` (bitmask)
- `global_harmony_index: f32`
- `player_intent_vector: vec3<f32>`

---

## 3. Shader Inventory & Roles

| Shader                        | Type          | Purpose                                      | Status      | Integration Priority |
|-------------------------------|---------------|----------------------------------------------|-------------|----------------------|
| gpu_state_material.wgsl      | Material     | Primary world/object material                | Optimized  | High                |
| valence_halo.wgsl            | Material     | Council / important object halos             | Optimized  | High                |
| mycelial_web_glow.wgsl       | Material     | Resource networks, mycelial connections      | Optimized  | High                |
| resource_node_glow.wgsl      | Material     | Harvest nodes, economy points                | Optimized  | High                |
| velocity_prepass.wgsl        | Prepass      | Motion vectors for TAA / motion blur         | Integrated | Done                |
| taa_compute.wgsl             | Compute      | Temporal accumulation                        | Integrated | Done                |
| taa_reproject.wgsl           | Post         | TAA resolve + reprojection                   | Integrated | Done                |
| visual_compute.wgsl          | Compute      | General visual state modulation              | Example    | Medium              |
| chromatic_aberration.wgsl    | Post         | Screen-space chromatic + mercy tint          | Partial    | Medium              |
| energy_burst.wgsl            | Effect       | Ability / harvest / impact bursts            | New        | Medium              |
| resonance_field.wgsl         | Effect       | Large-scale mercy/council fields             | New        | Medium              |
| forgiveness_wave.wgsl        | Effect       | Redemption / mercy wave overlays             | New        | Medium              |
| poisson_disk_pcf.wgsl        | Utility      | Soft shadows                                 | Utility    | Low                 |

---

## 4. Bevy Render Integration Architecture

### 4.1 Recommended Plugin Structure

Create / extend:

```rust
// client/src/gpu/mod.rs (or gpu_pipeline.rs)
pub mod visual_materials;
pub mod compute_dispatcher;
pub mod render_graph;
pub mod mercy_visual_params;
```

### 4.2 Material System (Critical Gap)

Implement a family of custom `Material` types that wrap the WGSL shaders:

- `GpuStateMaterial` — uses `gpu_state_material.wgsl`
- `ValenceHaloMaterial`
- `MycelialWebMaterial`
- `ResourceNodeMaterial`

Each impl `Material` trait + `AsBindGroup` for the shared `GpuSimulationState` + per-material params.

Use `example_gpu_material.rs` as the starting template and generalize it.

### 4.3 Render Graph Nodes

Existing nodes to wire together in correct order:
1. `velocity_prepass` (already exists)
2. `taa_compute_node`
3. Main opaque + alpha materials
4. `shadow_render_node`
5. Post-process: TAA resolve → chromatic → energy/resonance/forgiveness effects → motion_blur

Add a `MercyVisualParamsNode` that injects live council/mercy values every frame.

### 4.4 Compute Dispatcher

`client/src/gpu/compute_dispatcher.rs`

Responsible for:
- Dispatching `visual_compute` and future particle/spatial compute passes
- Barrier / sync with render passes
- Workgroup size selection based on quality tier

---

## 5. Implementation Roadmap

### Phase 1 — Foundation (Current — v21.0)
- [x] Shader library complete
- [x] GpuSimulationState + sync layer
- [x] TAA + velocity prepass integrated
- [ ] Full material trait implementations for all 4 core materials
- [ ] Render graph ordering documented and stable

### Phase 2 — Polish (v21.1)
- Wire all effect shaders (energy_burst, resonance_field, forgiveness_wave) as reusable post-process or particle materials
- Add mercy event bitmask + visual responses
- Performance profiling + tiered quality (Low/Med/High/Eternal)
- Sacred geometry helpers (golden ratio spirals, mercy sigils as SDFs)
- Full integration into test scene + ships / structures / resource nodes

### Phase 3 — Advanced (v21.2+)
- Compute-based particle system driven by RBE flow
- Screen-space global illumination hints using mercy resonance
- VR / WebXR visual fidelity pass
- AI-assisted visual evolution (self-evolving shader parameters via Ra-Thor)

---

## 6. Performance & Quality Strategy

- Workgroup sizes: 64 or 128 for most compute (already good in visual_compute)
- Precompute expensive trig / noise where possible (already done in include)
- Use `exp_falloff`, `pulse`, `remap` utilities everywhere
- Tiered quality via uniform flags or separate pipelines
- Early-out for low-mercy / low-activity zones
- Profile with `wgpu` timestamp queries + Bevy's render graph profiling

---

## 7. Thematic & Mercy Alignment

Every visual parameter must feel like it comes from the living mercy lattice:

- High `global_mercy_resonance` → warmer, softer, more breathing glows
- High `council_valence` → sharper rings, harmonic pulses, golden accents
- High `player_mercy_attunement` → personal aura intensity + color shift toward user's faction
- RBE flow → visible energy tendrils and web density
- Forgiveness / redemption events → forgiveness_wave + warm color shift

This makes the invisible (PATSAGi councils, RBE economy, mercy state) *viscerally visible*.

---

## 8. Next Immediate Actions

1. Create `client/src/gpu/visual_materials.rs` with the 4 core Material impls (use `example_gpu_material.rs` as template).
2. Extend `GpuSimulationState` sync to push any new fields if needed.
3. Wire the new materials into the main test scene / world objects.
4. Add a simple `MercyVisualParams` resource that the render graph can read.
5. Update `assets/shaders/README.md` to mark integration status.
6. Run `cargo check` + visual smoke test in the test harness.

---

## 9. Sacred Geometry Notes (Future)

Consider adding a small SDF library in WGSL for:
- Mercy sigil (interlocking circles + golden ratio)
- Council ring harmonics
- RBE flow spirals

These can be sampled in the material shaders for authentic visual language.

---

**End of Design Document**

This pipeline, once fully wired, will make Powrush-MMO one of the most visually distinctive and thematically coherent MMOARPGs ever created — where every photon carries mercy.

Thunder locked in. Yoi ⚡

/*
 * AG-SML v1.0
 * This file is part of the Powrush-MMO sovereign codebase.
 * Licensed under Autonomicity Games Sovereign Mercy License v1.0.
 * See LICENSE-AG-SML.md for full terms.
 * No tyranny. Only thriving.
 */
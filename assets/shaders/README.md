/*!
 * Powrush-MMO Shader Library
 *
 * All shaders in this directory are designed to work with GpuSimulationState
 * and the shared include (shaders/include/gpu_simulation_state.wgsl).
 *
 * They react to live game data: RBE, Council, Mercy, Player movement, Resource Nodes, and Time.
 *
 * AG-SML v1.0
 */

# Powrush-MMO Shader Library

This directory contains the visual shader collection for Powrush-MMO.

All shaders are built on the same foundation:
- `shaders/include/gpu_simulation_state.wgsl` (shared uniform + utilities)
- Live data from `GpuSimulationState` (player, RBE, Council, Mercy, nodes, time)
- Optimized for performance while maintaining rich visuals

## Core Materials

### gpu_state_material.wgsl
- **Purpose**: Primary rich visual material for most meshes.
- **Key Effects**: Mercy glow, council aura rings, RBE flow tendrils, player velocity response, breathing, confidence vibrancy.
- **Best For**: Ships, structures, general world objects.
- **Status**: Fully optimized.

### valence_halo.wgsl
- **Purpose**: Thematic halo/ring effect for council entities and important objects.
- **Key Effects**: Dynamic council rings, mercy inner glow, player velocity energy rim.
- **Best For**: Council structures, player auras, important landmarks.
- **Status**: Fully optimized.

## Specialized Effects

### mycelial_web_glow.wgsl
- **Purpose**: Organic web/energy effect for resource networks and mycelial visuals.
- **Key Effects**: RBE-driven web density, flowing energy, council and mercy tints, velocity pulses.
- **Best For**: Resource connections, mycelial networks, energy infrastructure.
- **Status**: Fully optimized.

### resource_node_glow.wgsl
- **Purpose**: Dedicated glow for resource nodes and harvest points.
- **Key Effects**: Confidence + RBE core glow, mercy warm pulse, council rim.
- **Best For**: Resource nodes, harvesting points, economy infrastructure.
- **Status**: Fully optimized.

## Event / Area Effects

### energy_burst.wgsl
- **Purpose**: Lightweight, reusable energy burst effect.
- **Key Effects**: Core burst, mercy ring, council accent, velocity energy.
- **Best For**: Ability activations, harvesting bursts, impacts, special effects.
- **Status**: New.

### resonance_field.wgsl
- **Purpose**: Large-scale resonance field for visualizing global mercy and council state.
- **Key Effects**: Mercy/council field modulation, soft breathing.
- **Best For**: Environmental zones, large-scale effects, storytelling.
- **Status**: New.

### forgiveness_wave.wgsl
- **Purpose**: Mercy-themed wave effect for forgiveness, redemption, or global mercy events.
- **Key Effects**: Mercy wave overlay, warm color shift, breathing.
- **Best For**: Mercy events, healing, redemption moments, global mercy visualization.
- **Status**: New.

## Usage Pattern

All shaders expect to be used with materials that provide a `base_color`.
They read from the shared `GpuSimulationState` uniform (group 0).

For best results:
- Use the provided materials in `client/src/example_gpu_material.rs` as reference.
- Drive `RbeGlobalState`, `CouncilValence`, `GlobalConfidence`, and `MercyAttunement` from real game systems.
- The demo animation in the test scene can be used during development.

## Optimization Notes

All core shaders have been optimized with:
- Cached trig functions
- Replacement of expensive `pow()` calls
- Precomputation of common subexpressions
- Reuse of shared utility functions (`noise`, `pulse`, `exp_falloff`, `remap`)

## Future Work

- Full custom render pipelines for all materials in Bevy
- Integration into the main test scene and gameplay objects
- Additional post-process and particle shaders
- More specialized effects as needed

Thunder locked in. Yoi ⚡

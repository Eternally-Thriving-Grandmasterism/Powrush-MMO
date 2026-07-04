/*!
 * gpu/mod.rs
 *
 * GPU subsystem aggregator for Powrush-MMO client.
 * 
 * Brings together:
 * - visual_materials (mercy-gated Bevy Materials for world, halos, webs, nodes)
 * - infrastructure_culling
 * - staging_buffer
 * - gpu_simulation (state sync, resources)
 *
 * Re-exports GpuVisualMaterialsPlugin and the core material types
 * for zero-friction use in main.rs and test/dev entity spawning.
 *
 * AG-SML v1.0 | TOLC 8 + PATSAGi Councils
 * Thunder locked in. Yoi ⚡
 */

pub mod infrastructure_culling;
pub mod staging_buffer;
pub mod visual_materials;
pub mod gpu_simulation;

// Re-export plugin under the name expected by main.rs
pub use visual_materials::VisualMaterialsPlugin as GpuVisualMaterialsPlugin;

// Re-export the four core mercy-gated materials for easy test entity wiring
pub use visual_materials::{
    GpuStateMaterial,
    ValenceHaloMaterial,
    MycelialWebMaterial,
    ResourceNodeMaterial,
};

/*
 * AG-SML v1.0
 * Autonomicity Games Sovereign Mercy License
 * https://github.com/Eternally-Thriving-Grandmasterism/Ra-Thor/blob/main/LICENSE-AG-SML.md
 */
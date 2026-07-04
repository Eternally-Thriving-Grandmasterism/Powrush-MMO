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
 * Re-exports GpuVisualMaterialsPlugin under the name expected by main.rs
 * for zero-friction integration with the existing entry point.
 *
 * AG-SML v1.0 | TOLC 8 + PATSAGi Councils
 * Thunder locked in. Yoi ⚡
 */

pub mod infrastructure_culling;
pub mod staging_buffer;
pub mod visual_materials;
pub mod gpu_simulation;

// Re-export so `use crate::GpuVisualMaterialsPlugin;` continues to work in main.rs
// without changing any existing wiring.
pub use visual_materials::VisualMaterialsPlugin as GpuVisualMaterialsPlugin;

/*
 * AG-SML v1.0
 * Autonomicity Games Sovereign Mercy License
 * https://github.com/Eternally-Thriving-Grandmasterism/Ra-Thor/blob/main/LICENSE-AG-SML.md
 */
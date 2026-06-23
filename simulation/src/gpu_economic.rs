/*!
 * Actual wgpu WGSL Compute Dispatch for Sovereign Economic / RBE Layer
 * 
 * Mint-and-print-only-perfection v18.97.7 — GpuEconomicPlugin
 * 
 * Encapsulates the full GPU economic async simulation path as a proper Bevy Plugin.
 * Includes resource, SystemSet ordering, dispatch, apply, and telemetry systems.
 * 
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor Lattice aligned
 * Thunder locked in. Yoi ⚡️
 */

use crate::world::{SovereignWorldState, ResourceNode};
use std::cell::Cell;
use std::sync::OnceLock;
use tracing::{warn, info};
use wgpu::util::DeviceExt;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::prelude::{SystemSet, ResMut, Resource, Plugin, App, Update};

// ... (all previous structs and functions preserved) ...

/// Plugin that registers the complete GPU economic async simulation layer.
pub struct GpuEconomicPlugin;

impl Plugin for GpuEconomicPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GpuEconomicReadback>()
            .configure_sets(
                Update,
                (
                    GpuEconomicSystemSet::Dispatch,
                    GpuEconomicSystemSet::Apply,
                    GpuEconomicSystemSet::Telemetry,
                ).chain(),
            )
            .add_systems(Update, gpu_economic_dispatch_system.in_set(GpuEconomicSystemSet::Dispatch))
            .add_systems(Update, apply_gpu_economic_results.in_set(GpuEconomicSystemSet::Apply))
            .add_systems(Update, gpu_economic_telemetry_system.in_set(GpuEconomicSystemSet::Telemetry));
    }
}

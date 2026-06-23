/*!
 * TickResult now tracks zones with active visual highlighting from policies.
 */

#[derive(Debug, Default, Clone)]
pub struct TickResult {
    // ... existing fields ...
    pub active_policy_count: usize,
    pub active_policy_types: Vec<PolicyType>,
    pub synergies_active: bool,
    pub conflicts_active: bool,

    pub zones_with_visual_highlight: usize,  // NEW: zones currently highlighted by policies
}

// In run_tick, after world updates:
let zones_with_visual_highlight = self.world.interest_zones
    .values()
    .filter(|z| z.visual_highlight > 0.15)
    .count();

let mut tick_result = TickResult {
    // ...
    zones_with_visual_highlight,
    ..Default::default()
};

// ============================================================================
// GPU Economic Async Readback Setup (v18.97.5)
// ============================================================================

use bevy::prelude::*;

/// Registers the `GpuEconomicReadback` resource and both dedicated GPU economic systems:
/// - `gpu_economic_dispatch_system` (submits work)
/// - `apply_gpu_economic_results` (applies completed readbacks)
///
/// Call once during plugin initialization.
pub fn setup_gpu_economic_async_readback(app: &mut App) {
    app
        .init_resource::<crate::gpu_economic::GpuEconomicReadback>()
        .add_systems(Update, (
            crate::gpu_economic::gpu_economic_dispatch_system,
            crate::gpu_economic::apply_gpu_economic_results,
        ).chain());  // dispatch first, then apply in same frame when possible
}

// Note for full integration:
// The dedicated dispatch system now handles submission every frame.
// `EconomicLayer::batch_update` can remain CPU-only or be gradually deprecated
// in favor of the Bevy systems for the GPU path.

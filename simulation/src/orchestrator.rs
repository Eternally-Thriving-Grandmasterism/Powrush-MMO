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

/// Registers the `GpuEconomicReadback` resource and the non-blocking
/// `apply_gpu_economic_results` system into the Bevy application.
///
/// This enables the production async GPU economic simulation path:
/// - Call `dispatch_gpu_economic_compute_async(...)` to submit work
/// - The registered system automatically polls and applies results every frame
///
/// # Usage
/// Call once during plugin initialization, e.g.:
///
/// ```ignore
/// impl Plugin for OrchestratorPlugin {
///     fn build(&self, app: &mut App) {
///         // ... other setup ...
///         setup_gpu_economic_async_readback(app);
///     }
/// }
/// ```
///
/// After this call, the async readback path is fully active in the `Update` schedule.
pub fn setup_gpu_economic_async_readback(app: &mut App) {
    app
        .init_resource::<crate::gpu_economic::GpuEconomicReadback>()
        .add_systems(Update, crate::gpu_economic::apply_gpu_economic_results);
}

// Note for full integration:
// In economy.rs (gpu branch of batch_update), transition from the legacy
// dispatch_gpu_economic_update(...) to dispatch_gpu_economic_compute_async(...)
// when the gpu feature is enabled and cpu_precision_mode is false.

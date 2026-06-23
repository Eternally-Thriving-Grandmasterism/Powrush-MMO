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
// GPU Economic Async Readback Wiring (v18.97.4)
// ============================================================================
// Minimal helper to wire the production async GPU economic path into the Bevy app.
// Call this from OrchestratorPlugin::build() or BevySimulationPlugin.
// 
// Usage example in plugin:
//   app
//       .init_resource::<crate::gpu_economic::GpuEconomicReadback>()
//       .add_systems(Update, crate::gpu_economic::apply_gpu_economic_results);

use bevy::prelude::*;

/// Wires GpuEconomicReadback resource and the non-blocking apply system.
/// Call once during plugin/app initialization.
pub fn wire_gpu_economic_readback(app: &mut App) {
    app
        .init_resource::<crate::gpu_economic::GpuEconomicReadback>()
        .add_systems(Update, crate::gpu_economic::apply_gpu_economic_results);

    // Optional: you can also add a startup system or ensure dispatch is called
    // from the economic batch or orchestrator tick when gpu feature is enabled.
}

// Note: For full integration, also ensure that in economy.rs batch_update (gpu branch)
// we eventually call dispatch_gpu_economic_compute_async(...) with the resource
// and current frame/tick when not in cpu_precision_mode.

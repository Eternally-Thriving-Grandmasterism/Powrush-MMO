/*!
 * TickResult now tracks zones with active visual highlighting from policies.
 */

#[derive(Debug, Default, Clone)]
pub struct TickResult { ... }

// In run_tick... (unchanged)

// ============================================================================
// GPU Economic Async Readback Setup (v18.97.6) - SystemSet Chaining
// ============================================================================

use bevy::prelude::*;
use crate::gpu_economic::{GpuEconomicSystemSet, gpu_economic_dispatch_system, apply_gpu_economic_results, GpuEconomicReadback};

/// Registers GPU economic systems using explicit SystemSet ordering.
/// This is the recommended Bevy pattern for clear, maintainable system ordering.
pub fn setup_gpu_economic_async_readback(app: &mut App) {
    app
        .init_resource::<GpuEconomicReadback>()
        .configure_sets(
            Update,
            (
                GpuEconomicSystemSet::Dispatch,
                GpuEconomicSystemSet::Apply,
            ).chain(),
        )
        .add_systems(Update, gpu_economic_dispatch_system.in_set(GpuEconomicSystemSet::Dispatch))
        .add_systems(Update, apply_gpu_economic_results.in_set(GpuEconomicSystemSet::Apply));
}

// Note: SystemSet makes future additions (e.g. telemetry, validation) easy to order correctly.

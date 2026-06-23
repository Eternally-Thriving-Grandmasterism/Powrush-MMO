/*!
 * TickResult now tracks zones with active visual highlighting from policies.
 */

#[derive(Debug, Default, Clone)]
pub struct TickResult { ... }

// In run_tick... (unchanged)

// GPU Economic setup is now fully handled by GpuEconomicPlugin.
// The previous setup_gpu_economic_async_readback helper has been superseded.

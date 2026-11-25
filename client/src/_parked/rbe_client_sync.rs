/*!
 * RBE Client Sync (refactored)
 * 
 * This module now primarily re-exports the gpu_simulation module.
 * All GPU state, buffers, plugin, sync logic, and placeholder resources
 * have been moved to client/src/gpu_simulation/ for better modularity.
 * 
 * Usage remains the same:
 *   use crate::rbe_client_sync::GpuSimulationStatePlugin;
 *   use crate::rbe_client_sync::sync_gpu_simulation_state;
 */

pub mod gpu_simulation;

pub use gpu_simulation::*;

// Any remaining non-GPU RBE client sync logic can live here in the future.
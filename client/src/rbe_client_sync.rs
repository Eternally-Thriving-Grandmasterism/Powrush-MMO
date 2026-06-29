/*!
 * RBE Client Sync
 * 
 * Re-exports the gpu_simulation module for better organization.
 * Heavy logic has been moved to client/src/gpu_simulation/
 */

pub mod gpu_simulation;

// Re-export main types for convenience
pub use gpu_simulation::*;
/*!
 * RBE Client Sync (refactored for modularity)
 * 
 * Most GPU simulation logic has been moved to client/src/gpu_simulation/
 * This file now serves as a thin entry point and re-exports.
 */

pub mod gpu_simulation;

// Re-export main public API
pub use gpu_simulation::*;

// Keep any remaining RBE-specific sync logic here if needed in the future.
/*!
 * Performance, Memory, and Game Integration Notes
 */

// Performance & Memory:
// - Current layout (fixed arrays + #[repr(C)]) is already very good for GPU upload.
// - If GpuSimulationState grows significantly larger, consider splitting it
//   into multiple smaller resources (e.g. GpuHotbarState, GpuCouncilState).
// - This allows more granular change detection and smaller GPU uploads.

// Actual Game Integration:
// The recommended pattern is:
// 1. Have dedicated game systems that own the authoritative data
//    (e.g. RbeEconomySystem, CouncilSystem, PlayerMovementSystem)
// 2. Have sync_gpu_simulation_state read from those systems at the end of the frame
// 3. GpuSimulationStatePlugin handles the dirty-checked GPU upload automatically
//
// Example:
// .add_systems(Update, (
//     rbe_economy_system,
//     council_system,
//     player_movement_system,
//     sync_gpu_simulation_state, // runs after the above
// ).chain())
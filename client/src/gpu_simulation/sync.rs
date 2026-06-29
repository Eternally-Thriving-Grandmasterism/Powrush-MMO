/*!
 * gpu_simulation::sync
 *
 * System that copies live game state (RBE, Council, Mercy, Player, Time)
 * into GpuSimulationState so shaders and custom materials can react.
 *
 * Now includes real LocalPlayer + Transform wiring for player_position.
 *
 * AG-SML v1.0
 */

use bevy::prelude::*;
use crate::gpu_simulation::state::GpuSimulationState;
use crate::gpu_simulation::resources::{GlobalConfidence, RbeGlobalState, CouncilValence, MercyAttunement};
use simulation::council_systems::RecentMercyResonance;

// Import the local player marker (adjust path if IsLocalPlayer is re-exported elsewhere)
use crate::local_player::IsLocalPlayer;

/// Main sync system. Runs every frame in Update.
pub fn sync_gpu_simulation_state(
    mut gpu_state: ResMut<GpuSimulationState>,
    time: Res<Time>,
    // Real mercy resonance from simulation crate
    mercy_resonance: Option<Res<RecentMercyResonance>>,
    // Bridge resources (updated by real RBE / Council / Player systems)
    global_confidence: Option<Res<GlobalConfidence>>,
    rbe_state: Option<Res<RbeGlobalState>>,
    council_valence: Option<Res<CouncilValence>>,
    // Player mercy attunement
    player_mercy_query: Query<&MercyAttunement>,
    // === NEW: Real LocalPlayer Transform wiring ===
    local_player_query: Query<&Transform, With<IsLocalPlayer>>,
) {
    // Always update time (critical for shader animations)
    gpu_state.time = time.elapsed_seconds();
    gpu_state.delta_time = time.delta_seconds();

    // === Mercy / Resonance ===
    if let Some(mercy) = mercy_resonance {
        gpu_state.global_mercy_resonance = mercy.value;
    }

    // === Global Confidence ===
    if let Some(conf) = global_confidence {
        gpu_state.global_confidence = conf.value;
    }

    // === RBE State ===
    if let Some(rbe) = rbe_state {
        gpu_state.rbe_flow_rate = rbe.flow_rate;
        gpu_state.total_rbe_circulating = rbe.total_circulating;
        gpu_state.player_rbe_balance = rbe.player_balance;
    }

    // === Council Valence ===
    if let Some(valence) = council_valence {
        gpu_state.council_valence = valence.value;
        gpu_state.active_council_action = valence.active_action;
        gpu_state.council_participants = valence.participants;
    }

    // === Player Mercy Attunement ===
    for attunement in &player_mercy_query {
        gpu_state.player_mercy_attunement = attunement.value;
        gpu_state.player_thrivability = attunement.thrivability;
        break;
    }

    // === REAL LocalPlayer Transform ===
    // This now pulls actual player position from the entity marked with IsLocalPlayer
    for transform in &local_player_query {
        let pos = transform.translation;
        gpu_state.player_position = [pos.x, pos.y, pos.z];

        // Velocity: Not yet wired.
        // Options:
        //   1. Add a Velocity component to the local player (recommended)
        //   2. Use prediction history from client prediction system
        //   3. Compute delta from previous frame (simple but less accurate)
        // For now we leave velocity at zero or last known value.
        // gpu_state.player_velocity = [...]; 
        break;
    }
}

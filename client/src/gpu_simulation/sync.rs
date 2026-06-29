/*!
 * gpu_simulation::sync
 *
 * System that copies live game state (RBE, Council, Mercy, Player, Time)
 * into GpuSimulationState so shaders and custom materials can react.
 *
 * This is the central wiring point between game systems and GPU visuals.
 * Real authoritative sources:
 *   - RBE:          simulation::rbe* / client::rbe_client_sync / server rbe_harvest
 *   - Council:      simulation::council_systems / client::council_*
 *   - Mercy:        simulation::council_systems::RecentMercyResonance + MercyAttunement
 *   - Player:       LocalPlayer / Transform + player marker components
 *   - Time:         Bevy Time
 *
 * AG-SML v1.0
 */

use bevy::prelude::*;
use crate::gpu_simulation::state::GpuSimulationState;
use crate::gpu_simulation::resources::{GlobalConfidence, RbeGlobalState, CouncilValence, MercyAttunement};
use simulation::council_systems::RecentMercyResonance;

/// Main sync system. Runs every frame in Update.
/// Pulls from real game systems (via the bridge resources) and writes to GPU state.
pub fn sync_gpu_simulation_state(
    mut gpu_state: ResMut<GpuSimulationState>,
    time: Res<Time>,
    // Real mercy resonance from simulation crate
    mercy_resonance: Option<Res<RecentMercyResonance>>,
    // Bridge resources (updated by real RBE / Council / Player systems)
    global_confidence: Option<Res<GlobalConfidence>>,
    rbe_state: Option<Res<RbeGlobalState>>,
    council_valence: Option<Res<CouncilValence>>,
    // Example: pull player mercy directly if entity has the component
    player_mercy_query: Query<&MercyAttunement>,
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

    // === RBE State (real data should flow here from rbe systems) ===
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

    // === Player data example (position/velocity would come from Transform + LocalPlayer) ===
    // For now we take the first MercyAttunement we find as player proxy.
    // In production: query LocalPlayer + Transform + MercyAttunement
    for attunement in &player_mercy_query {
        gpu_state.player_mercy_attunement = attunement.value;
        gpu_state.player_thrivability = attunement.thrivability;
        // TODO: also set player_position and player_velocity from Transform
        break;
    }
}

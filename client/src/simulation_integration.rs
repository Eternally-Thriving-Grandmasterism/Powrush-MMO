// client/src/simulation_integration.rs
// Powrush Client - Sovereign Simulation Integration v17.99.22
// Wires simulation crate for in-game 'What-If / Replay' mode
// Allows players and PATSAGi Councils to test interventions in the live client

use bevy::prelude::*;
use simulation::{run_sovereign_scenario, inject_patsagi_intervention, Telemetry};

#[derive(Resource, Default)]
pub struct SimulationReplayState {
    pub current_telemetry: Option<Telemetry>,
    pub last_intervention_result: Option<String>,
}

pub fn setup_simulation_integration(app: &mut App) {
    app.init_resource::<SimulationReplayState>()
        .add_systems(Update, simulation_replay_ui);
}

fn simulation_replay_ui(
    mut state: ResMut<SimulationReplayState>,
    // In real: query for UI buttons in game HUD
) {
    // Example: On 'What-If' button press in client
    // let report = run_sovereign_scenario("balanced_growth", 100, true);
    // state.current_telemetry = report.telemetry;
    // For PATSAGi: inject_patsagi_intervention(json) with mercy gate
}

// Future: Sync telemetry to Bevy ECS for visual replay of archetype evolution, RBE flows, etc.
// All calls pass TOLC 8 non-bypassable gates via the simulation crate.
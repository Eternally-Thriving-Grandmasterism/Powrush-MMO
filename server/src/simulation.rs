// server/src/simulation.rs
// Powrush-MMO v18.91 — SimulationApp with SovereignSimulationOrchestrator integration
// Wires TickResult + SimulationTick into the Bevy simulation layer

use bevy::prelude::*;
use simulation::orchestrator::{SovereignSimulationOrchestrator, SimulationTick, SimulationTickEvent, TickResult};
use crate::combat::CombatPlugin;
use crate::replication::ReplicationPlugin;
use crate::rathor_integration::RathorIntegrationPlugin;

/// Dedicated Bevy App for all simulation systems.
pub struct SimulationApp {
    pub app: App,
}

impl SimulationApp {
    pub fn new() -> Self {
        let mut app = App::new();

        // Register core simulation plugins
        app.add_plugins((
            CombatPlugin,
            ReplicationPlugin,
            RathorIntegrationPlugin,
        ));

        // Initialize orchestrator and tick tracking
        app.init_resource::<SimulationTick>();
        app.insert_resource(SovereignSimulationOrchestrator::new(
            simulation::world::SovereignWorldState::default()
        ));

        // Add the main simulation tick system
        app.add_systems(Update, run_simulation_tick);

        Self { app }
    }

    pub fn tick(&mut self) {
        self.app.update();
    }

    pub fn world(&self) -> &World {
        self.app.world()
    }

    pub fn world_mut(&mut self) -> &mut World {
        self.app.world_mut()
    }
}

/// Main system that runs the SovereignSimulationOrchestrator every frame
/// and emits SimulationTickEvent with the rich TickResult.
fn run_simulation_tick(
    mut orchestrator: ResMut<SovereignSimulationOrchestrator>,
    mut tick_resource: ResMut<SimulationTick>,
    mut tick_events: EventWriter<SimulationTickEvent>,
) {
    match orchestrator.run_tick(Some(&mut tick_resource)) {
        Ok(result) => {
            // Emit event so replication, persistence, and other systems can react
            tick_events.send(SimulationTickEvent {
                tick: tick_resource.tick,
                sim_time_ms: tick_resource.sim_time_ms,
                result,
            });
        }
        Err(e) => {
            // Mercy violation or other error — log and skip
            warn!("Simulation tick failed due to mercy violation: {:?}", e);
        }
    }
}

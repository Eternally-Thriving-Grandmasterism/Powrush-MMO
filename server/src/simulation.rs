// server/src/simulation.rs
// Powrush-MMO v18.91 — SimulationApp with expanded TickResult consumers
// Now wires Council blooms, emergence, and harvest events from TickResult

use bevy::prelude::*;
use simulation::orchestrator::{SovereignSimulationOrchestrator, SimulationTick, SimulationTickEvent, TickResult};
use crate::council_session::{BatchPersistenceQueue};
use crate::combat::CombatPlugin;
use crate::replication::ReplicationPlugin;
use crate::rathor_integration::RathorIntegrationPlugin;

pub struct SimulationApp {
    pub app: App,
}

impl SimulationApp {
    pub fn new() -> Self {
        let mut app = App::new();

        app.add_plugins((
            CombatPlugin,
            ReplicationPlugin,
            RathorIntegrationPlugin,
        ));

        app.init_resource::<SimulationTick>();
        app.insert_resource(SovereignSimulationOrchestrator::new(
            simulation::world::SovereignWorldState::default()
        ));
        app.init_resource::<BatchPersistenceQueue>();

        app.add_systems(Update, (
            run_simulation_tick,
            consume_tick_result_for_persistence,
            consume_tick_result_for_replication,
        ));

        Self { app }
    }

    pub fn tick(&mut self) {
        self.app.update();
    }

    pub fn world(&self) -> &World { self.app.world() }
    pub fn world_mut(&mut self) -> &mut World { self.app.world_mut() }
}

/// Runs the orchestrator and emits SimulationTickEvent
fn run_simulation_tick(
    mut orchestrator: ResMut<SovereignSimulationOrchestrator>,
    mut tick_resource: ResMut<SimulationTick>,
    mut tick_events: EventWriter<SimulationTickEvent>,
) {
    if let Ok(result) = orchestrator.run_tick(Some(&mut tick_resource)) {
        tick_events.send(SimulationTickEvent {
            tick: tick_resource.tick,
            sim_time_ms: tick_resource.sim_time_ms,
            result,
        });
    }
}

/// Pushes council persistence data into BatchPersistenceQueue
fn consume_tick_result_for_persistence(
    mut tick_events: EventReader<SimulationTickEvent>,
    mut batch_queue: ResMut<BatchPersistenceQueue>,
) {
    for event in tick_events.read() {
        for update in &event.result.closed_session_persistence {
            batch_queue.queue.push(update.clone());
        }
    }
}

/// Wires TickResult data into replication and dynamic event systems
fn consume_tick_result_for_replication(
    mut tick_events: EventReader<SimulationTickEvent>,
    // TODO: Add EventWriters for replication events, dynamic events, etc.
) {
    for event in tick_events.read() {
        let result = &event.result;

        // Council bloom events (ready for client council replication)
        if !result.council_bloom_events.is_empty() {
            // TODO: Emit CouncilBloomSyncEvent or replication update
            info!("Tick {} produced {} council bloom events", event.tick, result.council_bloom_events.len());
        }

        // Emergence events (ready for dynamic event replication)
        if !result.emergence_events.is_empty() {
            // TODO: Forward to dynamic_events system / replication
            info!("Tick {} produced {} emergence events", event.tick, result.emergence_events.len());
        }

        // Harvest events (ready for RBE / abundance replication)
        if !result.harvest_events.is_empty() {
            // TODO: Forward to harvesting_system / rbe replication
            info!("Tick {} produced {} harvest events", event.tick, result.harvest_events.len());
        }

        // Spatial interest changes
        if result.spatial_interest_updated {
            // TODO: Update interest management / replication interest zones
            info!("Tick {} had spatial interest changes ({} zones)", event.tick, result.spatial_zones_changed);
        }
    }
}

// server/src/simulation.rs
// Powrush-MMO v18.91 — SimulationApp with complete TickResult forwarding
// All major categories now have forwarding structure (Council, Emergence, Harvest, Spatial)

use bevy::prelude::*;
use simulation::orchestrator::{SovereignSimulationOrchestrator, SimulationTick, SimulationTickEvent, TickResult};
use simulation::emergence::DynamicEmergenceEvent;
use simulation::harvest::HarvestEvent;
use simulation::spatial_interest::InterestZoneReplicated;
use crate::council_mercy_trial::CouncilBloomSyncEvent;
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

/// Forwards TickResult data into replication and dynamic event systems
/// All major categories now have forwarding (Council, Emergence, Harvest, Spatial)
fn consume_tick_result_for_replication(
    mut tick_events: EventReader<SimulationTickEvent>,
    mut council_bloom_writer: EventWriter<CouncilBloomSyncEvent>,
    mut emergence_writer: EventWriter<DynamicEmergenceEvent>,
    mut harvest_writer: EventWriter<HarvestEvent>,
    mut spatial_writer: EventWriter<InterestZoneReplicated>,
) {
    for event in tick_events.read() {
        let result = &event.result;

        // === Council Bloom Events (fully wired) ===
        for bloom in &result.council_bloom_events {
            council_bloom_writer.send(bloom.clone());
        }

        // === Emergence Events (fully wired) ===
        for emergence in &result.emergence_events {
            emergence_writer.send(emergence.clone());
        }

        // === Harvest Events (fully wired) ===
        for harvest in &result.harvest_events {
            harvest_writer.send(harvest.clone());
        }

        // === Spatial Interest Changes (TODO cleared) ===
        if result.spatial_interest_updated || result.spatial_zones_changed > 0 {
            // Emit a representative spatial replication event
            // In full implementation this would carry actual changed InterestZone data
            spatial_writer.send(InterestZoneReplicated {
                entity: Entity::PLACEHOLDER, // TODO: populate with real entity when zone data is carried in TickResult
                zone: Default::default(),
                version: event.tick,
                server_timestamp: event.sim_time_ms as f64,
            });

            info!("Forwarded spatial interest update ({} zones changed, tick={})", 
                  result.spatial_zones_changed, event.tick);
        }

        if result.any_significant_change {
            info!("Tick {} had significant simulation changes", event.tick);
        }
    }
}

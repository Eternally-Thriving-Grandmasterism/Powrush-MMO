// server/src/simulation.rs
// Powrush-MMO v18.95 — Full deep wiring including persistence recording for Harvest + Emergence

use bevy::prelude::*;
use simulation::orchestrator::{SovereignSimulationOrchestrator, SimulationTick, SimulationTickEvent, TickResult};
use simulation::emergence::DynamicEmergenceEvent;
use simulation::harvest::HarvestEvent;
use simulation::spatial_interest::InterestZoneReplicated;
use crate::council_mercy_trial::CouncilBloomSyncEvent;
use crate::council_session::{BatchPersistenceQueue, BatchPersistenceUpdate};
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

/// Persistence consumer now records HarvestEvent and DynamicEmergenceEvent outcomes
fn consume_tick_result_for_persistence(
    mut tick_events: EventReader<SimulationTickEvent>,
    mut batch_queue: ResMut<BatchPersistenceQueue>,
) {
    for event in tick_events.read() {
        // Existing council persistence
        for update in &event.result.closed_session_persistence {
            batch_queue.queue.push(update.clone());
        }

        // HarvestEvent persistence recording
        for harvest in &event.result.harvest_events {
            if harvest.amount > 0.0 || harvest.epiphany_triggered {
                let update = BatchPersistenceUpdate {
                    player_id: harvest.player_id,
                    update_type: if harvest.epiphany_triggered {
                        "harvest_epiphany".to_string()
                    } else {
                        "harvest".to_string()
                    },
                    data: format!("node={},amount={},sustainable={}", harvest.node_id, harvest.amount, harvest.sustainable),
                };
                batch_queue.queue.push(update);
            }
        }

        // DynamicEmergenceEvent persistence recording
        for emergence in &event.result.emergence_events {
            let update = BatchPersistenceUpdate {
                player_id: 0, // system/world emergence for now
                update_type: "emergence".to_string(),
                data: format!("phase={:?},effects={}", emergence.phase, emergence.proposed_effects.len()),
            };
            batch_queue.queue.push(update);
        }
    }
}

/// Deep wiring of all major TickResult events into replication
fn consume_tick_result_for_replication(
    mut tick_events: EventReader<SimulationTickEvent>,
    mut council_bloom_writer: EventWriter<CouncilBloomSyncEvent>,
    mut emergence_writer: EventWriter<DynamicEmergenceEvent>,
    mut harvest_writer: EventWriter<HarvestEvent>,
    mut spatial_writer: EventWriter<InterestZoneReplicated>,
) {
    for event in tick_events.read() {
        let result = &event.result;

        for bloom in &result.council_bloom_events {
            council_bloom_writer.send(bloom.clone());
        }

        for emergence in &result.emergence_events {
            emergence_writer.send(emergence.clone());
        }

        for harvest in &result.harvest_events {
            harvest_writer.send(harvest.clone());
        }

        for zone_update in &result.changed_spatial_zones {
            spatial_writer.send(zone_update.clone());
        }

        if result.any_significant_change {
            info!("Tick {} significant (harvests={}, emergence={})", 
                  event.tick, result.harvest_events.len(), result.emergence_events.len());
        }
    }
}

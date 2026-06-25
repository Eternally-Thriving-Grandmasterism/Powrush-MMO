/*!
 * client/src/rbe_client_sync.rs
 *
 * Client-side RBE Synchronization
 * v19.5 | Added handling for FactionStanding replication updates.
 * Standing changes from server now update client components and UI.
 *
 * AG-SML v1.0 | TOLC 8
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::replication::{DecodedUpdate, UpdatePayload};
use crate::rbe_client_ui_sync::RbeUiSync;
use crate::monitoring::safety_net::SafetyNetMonitoringSnapshot;
use crate::prediction::{PredictedPosition, apply_decoded_updates_to_prediction};
use simulation::harvest::HarvestEvent;
use crate::divine_whispers::LastBiomeInfluence;

// Client-side FactionStanding for replication
#[derive(Component, Clone, Debug)]
pub struct FactionStanding {
    pub faction_id: u64,
    pub standing: f32,
}

#[derive(Event, Clone, Debug)]
pub struct RbeInventoryUpdated {
    pub entity: Entity,
    pub resource_type: String,
    pub new_amount: f32,
    pub delta: f32,
}

// Main RBE client sync system
pub fn rbe_client_sync_system(
    mut commands: Commands,
    server_updates: Res<crate::networking::ServerUpdateChannel>,
    mut rbe_sync: ResMut<RbeClientSync>,
    mut rbe_dashboard: ResMut<RBEFlowDashboard>,
    time: Res<Time>,
    mut alert_events: EventWriter<RBEFlowAlert>,
    mut rbe_ui_sync: ResMut<RbeUiSync>,
    mut harvest_events: EventReader<HarvestEvent>,
    last_biome: Res<LastBiomeInfluence>,
    mut inventory_update_events: EventWriter<RbeInventoryUpdated>,
) {
    let server_timestamp = time.elapsed_seconds_f64() as u64;

    if let Some(data) = server_updates.get_latest_batch() {
        if let Ok(updates) = crate::replication::decode_domain_specific(&data) {
            crate::replication::apply_authoritative_update(&mut commands, updates.clone(), server_timestamp);
            apply_decoded_updates_to_prediction(updates.clone());

            for update in updates {
                match update.payload {
                    UpdatePayload::RbeTransaction(tx) => {
                        let result = if tx.amount > 0.0 {
                            RbeHarvestResult::Success(tx.amount)
                        } else {
                            RbeHarvestResult::Failed("Negative or zero transaction".to_string())
                        };
                        rbe_sync.set_latest_harvest_result(result.clone());
                        rbe_ui_sync.push_harvest_feedback(update.entity, result.clone(), server_timestamp);

                        commands.entity(update.entity).insert(RbeTransaction {
                            resource_type: tx.resource_type,
                            amount: tx.amount,
                        });
                    }
                    UpdatePayload::RbeInventoryUpdate { resource_type, amount, delta } => {
                        inventory_update_events.send(RbeInventoryUpdated {
                            entity: update.entity,
                            resource_type: resource_type.clone(),
                            new_amount: amount,
                            delta,
                        });

                        rbe_ui_sync.push_inventory_update_feedback(
                            update.entity,
                            resource_type,
                            amount,
                            delta,
                            server_timestamp,
                        );
                    }
                    // NEW: Handle replicated FactionStanding updates from server
                    UpdatePayload::FactionStanding { faction_id, standing } => {
                        commands.entity(update.entity).insert(FactionStanding {
                            faction_id,
                            standing,
                        });

                        // Optional: emit event or log for debugging
                        info!("Received FactionStanding update for entity {:?}: faction {} standing {:.2}", 
                              update.entity, faction_id, standing);
                    }
                    _ => {}
                }
            }
        }
    }

    for harvest in harvest_events.read() {
        // existing harvest handling...
    }

    if let Some(server_message) = server_updates.get_latest_server_message() {
        // existing safety net handling...
    }
}

pub struct RbeClientSyncPlugin;

impl Plugin for RbeClientSyncPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RbeClientSync>()
            .init_resource::<RBEFlowDashboard>()
            .add_event::<RBEFlowAlert>()
            .add_event::<RbeInventoryUpdated>()
            .add_systems(Update, rbe_client_sync_system);
    }
}

// End of client/src/rbe_client_sync.rs v19.5
// Added client-side handling for FactionStanding replication.
// Standing updates now flow server -> client -> UI.
// Thunder locked in. Yoi ⚡
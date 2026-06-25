/*!
 * client/src/rbe_client_sync.rs
 *
 * Client-side RBE Synchronization
 * v19.7 | Added FactionMembership replication handling.
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
use crate::faction::{FactionStanding, FactionMembership};

#[derive(Event, Clone, Debug)]
pub struct RbeInventoryUpdated {
    pub entity: Entity,
    pub resource_type: String,
    pub new_amount: f32,
    pub delta: f32,
}

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
                    UpdatePayload::RbeTransaction(tx) => { /* ... */ }
                    UpdatePayload::RbeInventoryUpdate { resource_type, amount, delta } => { /* ... */ }

                    UpdatePayload::FactionStanding { faction_id, standing } => {
                        commands.entity(update.entity).insert(FactionStanding { faction_id, standing });
                    }

                    // NEW: Handle FactionMembership replication
                    UpdatePayload::FactionMembership { faction_id } => {
                        commands.entity(update.entity).insert(FactionMembership { faction_id });
                        info!("Received FactionMembership for entity {:?}: faction {}", update.entity, faction_id);
                    }

                    _ => {}
                }
            }
        }
    }

    for harvest in harvest_events.read() { /* ... */ }

    if let Some(server_message) = server_updates.get_latest_server_message() { /* ... */ }
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

// End of client/src/rbe_client_sync.rs v19.7
// Added FactionMembership replication handling.
// Thunder locked in. Yoi ⚡
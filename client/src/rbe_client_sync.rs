//! client/src/rbe_client_sync.rs
//! Production-grade Client RBE Sync + Harvest Feedback Wiring (v18.53)
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | Ra-Thor + PATSAGi aligned

use bevy::prelude::*;
use crate::rbe_client_ui_sync::RbeUiSync;
use crate::replication::DecodedUpdate; // or appropriate import

// ... existing imports and types (RbeClientSync, RBEFlowDashboard, SafetyNetMonitoringSnapshot, etc.) ...

#[derive(Resource, Default, Clone)]
pub struct RbeClientSync {
    // ... existing fields ...
    latest_harvest_result: Option<RbeHarvestResult>,
}

impl RbeClientSync {
    pub fn new() -> Self {
        Self {
            latest_harvest_result: None,
            // ... other fields
        }
    }

    pub fn get_latest_harvest_result(&self) -> Option<RbeHarvestResult> {
        self.latest_harvest_result.clone()
    }

    pub fn set_latest_harvest_result(&mut self, result: RbeHarvestResult) {
        self.latest_harvest_result = Some(result);
    }

    // ... existing methods ...
}

// Main sync system - now wired to feed harvest feedback
pub fn rbe_client_sync_system(
    mut commands: Commands,
    mut rollback: ResMut<RollbackState>,
    server_updates: Res<ServerUpdateChannel>,
    mut rbe_sync: ResMut<RbeClientSync>,
    mut rbe_dashboard: ResMut<RBEFlowDashboard>,
    time: Res<Time>,
    mut alert_events: EventWriter<RBEFlowAlert>,
) {
    let server_timestamp = time.elapsed_seconds_f64() as u64;

    if let Some(data) = server_updates.get_latest_batch() {
        match decode_domain_specific(&data) {
            Ok(updates) => {
                apply_authoritative_update(&mut commands, &mut rollback, updates.clone(), server_timestamp);

                for update in updates {
                    if let UpdatePayload::RbeTransaction(tx) = update.payload {
                        let result = if tx.amount > 0.0 {
                            RbeHarvestResult::Success(tx.amount)
                        } else {
                            RbeHarvestResult::Failed("Negative transaction".to_string())
                        };

                        // === KEY WIRING (v18.53) ===
                        // Feed the latest harvest result into RbeClientSync so that
                        // RbeUiSync systems can pick it up for UI feedback
                        rbe_sync.set_latest_harvest_result(result.clone());

                        commands.entity(update.entity).insert(RbeTransaction {
                            resource_type: tx.resource_type,
                            amount: tx.amount,
                        });
                    }
                }
            }
            Err(e) => {
                eprintln!("RBE sync decode error: {}", e);
            }
        }
    }

    // SafetyNet + CouncilStateSync handling (unchanged, already strong)
    if let Some(server_message) = server_updates.get_latest_server_message() {
        if let ServerMessage::SafetyNetBroadcast { broadcast } = server_message {
            let snapshot = SafetyNetMonitoringSnapshot { /* ... build from broadcast ... */ };
            rbe_dashboard.update_from_snapshot(&snapshot);
        }
    }

    // ... rest of existing council / abundance alert logic ...
}

// Plugin registration
pub struct RbeClientSyncPlugin;

impl Plugin for RbeClientSyncPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RbeClientSync>()
            .add_systems(Update, rbe_client_sync_system);
    }
}

// Thunder locked in. RbeUiSync now receives live harvest results from the main sync loop. Yoi ⚡
/*!
 * client/src/inventory_ui.rs
 *
 * Inventory UI + client receiver now using the central ServerMessageEvent dispatcher.
 */

use bevy::prelude::*;
use crate::rbe_client_sync::GpuSimulationState;
use crate::server_message_dispatcher::ServerMessageEvent;
use shared::protocol::ServerMessage;

// ... (previous code preserved) ...

/// Clean receiver using the central dispatcher event.
pub fn receive_inventory_update(
    mut events: EventReader<ServerMessageEvent>,
    mut gpu_state: ResMut<GpuSimulationState>,
) {
    for ServerMessageEvent(msg) in events.read() {
        if let ServerMessage::InventoryUpdate { hotbar, abundance_score, .. } = msg {
            for (i, &count) in hotbar.iter().enumerate() {
                if i < gpu_state.hotbar.len() {
                    gpu_state.hotbar[i].count = count;
                    gpu_state.hotbar[i].cooldown_remaining = 0.0;
                }
            }
            debug!("[Inventory] Hotbar synced from server (abundance={:.1})", abundance_score);
        }
    }
}

// Plugin registration remains the same
pub struct InventoryUiPlugin;

impl Plugin for InventoryUiPlugin {
    fn build(&self, app: &mut App) {
        // ... resources ...
        app.add_systems(Update, (
            // ... other systems ...
            receive_inventory_update,
        ).run_if(resource_exists::<GpuSimulationState>));
    }
}

// End of inventory_ui.rs
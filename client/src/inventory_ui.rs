/*!
 * client/src/inventory_ui.rs
 *
 * Now includes client-side receiver for ServerMessage::InventoryUpdate
 * with full hotbar array sync to GpuSimulationState.
 */

use bevy::prelude::*;
use crate::rbe_client_sync::GpuSimulationState;
use crate::networking::ServerUpdateChannel;
use shared::protocol::ServerMessage;
use bincode;

// ... (all previous InventorySlot, InventoryDragState, hotbar functions, etc. preserved) ...

/// System: Receives InventoryUpdate from server and syncs the full hotbar
/// to the local GpuSimulationState (authoritative replication).
pub fn receive_inventory_update(
    mut update_channel: ResMut<ServerUpdateChannel>,
    mut gpu_state: ResMut<GpuSimulationState>,
) {
    // Drain all pending messages this frame
    while let Ok(bytes) = update_channel.rx.try_recv() {
        if let Ok(msg) = bincode::deserialize::<ServerMessage>(&bytes) {
            if let ServerMessage::InventoryUpdate { hotbar, abundance_score, .. } = msg {
                // Apply full authoritative hotbar
                for (i, &count) in hotbar.iter().enumerate() {
                    if i < gpu_state.hotbar.len() {
                        gpu_state.hotbar[i].count = count;
                        // Reset cooldown on sync (or preserve if you track it server-side)
                        gpu_state.hotbar[i].cooldown_remaining = 0.0;
                    }
                }

                // Optional: also sync abundance
                // gpu_state.player_rbe_balance = abundance_score; // if desired

                debug!("[Inventory] Synced full hotbar from server (abundance={:.1})", abundance_score);
            }
        }
    }
}

// Update the plugin to include the new receiver
pub struct InventoryUiPlugin;

impl Plugin for InventoryUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InventoryDragState>()
           .init_resource::<InventoryFilter>()
           .init_resource::<DemoInventory>()
           .add_systems(Update, (
                toggle_inventory_panel,
                update_inventory_grid,
                update_item_tooltips,
                apply_inventory_filters,
                start_drag,
                update_drag_ghost,
                handle_drop,
                cancel_drag_input,
                receive_inventory_update,      // NEW: full hotbar sync from server
            ).run_if(resource_exists::<GpuSimulationState>));
    }
}

// End of inventory_ui.rs - client now fully receives authoritative hotbar from InventoryUpdate
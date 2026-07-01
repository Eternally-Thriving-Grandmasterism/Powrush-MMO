/*!
 * client/src/inventory_ui.rs
 *
 * Inventory UI systems (panel, drag & drop, hotbar rendering, tooltips, filters).
 * Replication logic has been moved to inventory_replication.rs for symmetry with server.
 */

use bevy::prelude::*;
use crate::inventory_replication::receive_inventory_update;

// ... (all hotbar functions, InventorySlot, InventoryDragState, toggle_inventory_panel, etc. remain here) ...

// The receive_inventory_update system is now imported from inventory_replication

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
                receive_inventory_update,   // from inventory_replication
            ).run_if(resource_exists::<GpuSimulationState>));
    }
}

// End of inventory_ui.rs
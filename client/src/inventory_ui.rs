/*!
 * client/src/inventory_ui.rs
 * General 40-slot inventory grid drag & drop implementation (including hotbar cross-dragging).
 */

use bevy::prelude::*;
use crate::inventory_replication::receive_inventory_update;
use crate::rbe_client_sync::GpuSimulationState;
use shared::protocol::HotbarSlot;

// ... existing get_rarity_color, InventorySlot, InventoryDragState, etc. ...

/// Enhanced handle_drop supporting full grid + hotbar cross dragging
pub fn handle_drop(
    mut commands: Commands,
    mut drag: ResMut<InventoryDragState>,
    mut demo_inv: ResMut<DemoInventory>,
    mut gpu_state: ResMut<GpuSimulationState>,
    target_query: Query<(&InventorySlot, &Interaction, Entity)>,
) {
    if !drag.is_dragging || drag.source.is_none() { return; }
    let src = drag.source.unwrap();

    for (tgt, interaction, _entity) in target_query.iter() {
        if *interaction == Interaction::Pressed || *interaction == Interaction::Hovered {
            if tgt.index == src.index && tgt.is_hotbar == src.is_hotbar {
                cancel_drag(&mut commands, &mut drag);
                return;
            }

            // Basic validation hook (rarity / item type can be expanded here)
            let can_move = validate_move(&src, tgt);

            if can_move {
                if src.is_hotbar && tgt.is_hotbar {
                    // Hotbar <-> Hotbar swap (already wired to GpuSimulationState)
                    let s = src.index as usize;
                    let t = tgt.index as usize;
                    if s < 8 && t < 8 {
                        let tmp = gpu_state.hotbar[s].count;
                        gpu_state.hotbar[s].count = gpu_state.hotbar[t].count;
                        gpu_state.hotbar[t].count = tmp;
                    }
                } else {
                    // Grid <-> Grid or Grid <-> Hotbar (demo inventory for now)
                    demo_inv.swap(src.index as usize, tgt.index as usize);

                    // TODO: When full HotbarSlot data is in GpuSimulationState,
                    // perform real move/swap here and mark dirty for replication
                }

                info!("[Inventory] Drag drop: {:?} -> {:?}", src, tgt);
            } else {
                warn!("[Inventory] Move blocked by validation");
            }

            cancel_drag(&mut commands, &mut drag);
            return;
        }
    }

    cancel_drag(&mut commands, &mut drag);
}

/// Basic move validation (expand with real item metadata + rarity rules)
fn validate_move(source: &InventorySlot, target: &InventorySlot) -> bool {
    // Example: prevent moving mythic items into certain slots, or check item category
    // For now permissive
    true
}

// ... rest of drag systems (start_drag, update_drag_ghost, etc.) remain ...

pub struct InventoryUiPlugin;

impl Plugin for InventoryUiPlugin {
    fn build(&self, app: &mut App) {
        // ...
        app.add_systems(Update, (
            // ...
            handle_drop,
            // ...
        ));
    }
}

// End of inventory_ui.rs
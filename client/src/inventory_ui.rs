/*!
 * client/src/inventory_ui.rs
 * Client now sends InventoryMove for general grid drags.
 */

use bevy::prelude::*;
use crate::networking::OutgoingClientMessages;
use shared::protocol::ClientMessage;

// ... existing InventorySlot, InventoryDragState, etc. ...

pub fn handle_drop(
    mut commands: Commands,
    mut drag: ResMut<InventoryDragState>,
    mut demo_inv: ResMut<DemoInventory>,
    mut gpu_state: ResMut<GpuSimulationState>,
    target_query: Query<(&InventorySlot, &Interaction, Entity)>,
    outgoing: Res<OutgoingClientMessages>,
) {
    if !drag.is_dragging || drag.source.is_none() { return; }
    let src = drag.source.unwrap();

    for (tgt, interaction, _e) in target_query.iter() {
        if *interaction == Interaction::Pressed || *interaction == Interaction::Hovered {
            if tgt.index == src.index && tgt.is_hotbar == src.is_hotbar {
                cancel_drag(&mut commands, &mut drag);
                return;
            }

            let validity = validate_move(&src, tgt);

            if validity.allowed {
                // Optimistic local update
                if src.is_hotbar && tgt.is_hotbar {
                    // Hotbar swap (local + will send message)
                    let s = src.index as usize;
                    let t = tgt.index as usize;
                    if s < 8 && t < 8 {
                        let tmp = gpu_state.hotbar[s].count;
                        gpu_state.hotbar[s].count = gpu_state.hotbar[t].count;
                        gpu_state.hotbar[t].count = tmp;
                    }

                    // Send hotbar-specific message
                    let _ = outgoing.tx.send(ClientMessage::InventoryHotbarMove {
                        from_slot: src.index as u8,
                        to_slot: tgt.index as u8,
                    });
                } else {
                    // General grid move (or grid <-> hotbar)
                    demo_inv.swap(src.index as usize, tgt.index as usize);

                    // Send general inventory move to server
                    let _ = outgoing.tx.send(ClientMessage::InventoryMove {
                        from: src.index,
                        to: tgt.index,
                    });
                }

                info!("[Inventory] Sent move to server: {:?} -> {:?}", src, tgt);
            }

            cancel_drag(&mut commands, &mut drag);
            return;
        }
    }

    cancel_drag(&mut commands, &mut drag);
}

// ... rest of file ...

pub struct InventoryUiPlugin;

impl Plugin for InventoryUiPlugin {
    fn build(&self, app: &mut App) {
        // ...
        // OutgoingClientMessages should already be inserted by NetworkingPlugin
    }
}

// End of inventory_ui.rs
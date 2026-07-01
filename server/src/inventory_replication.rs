/*!
 * server/src/inventory_replication.rs
 * Now handles full HotbarSlot (item_id + metadata) instead of just counts.
 */

use shared::protocol::{ClientMessage, ServerMessage, HotbarSlot};
use crate::persistence_polish::{PersistenceManager, PlayerSaveData};
use tracing::{info, warn};

pub fn handle_inventory_hotbar_move(
    player_id: u64,
    message: &ClientMessage,
    persistence: &mut PersistenceManager,
) -> Option<ServerMessage> {
    let ClientMessage::InventoryHotbarMove { from_slot, to_slot } = message else { return None; };

    if from_slot == to_slot || from_slot >= 8 || to_slot >= 8 { return None; }

    let mut player_data = persistence.load_player(player_id)
        .unwrap_or_else(|| PlayerSaveData::new(player_id));

    // Swap full HotbarSlot
    if from_slot < 8 && to_slot < 8 {
        let tmp = player_data.hotbar[from_slot as usize].clone();
        player_data.hotbar[from_slot as usize] = player_data.hotbar[to_slot as usize].clone();
        player_data.hotbar[to_slot as usize] = tmp;
    }

    if let Err(e) = persistence.save_player(&player_data) {
        warn!("Failed to save hotbar: {}", e);
    }

    info!("[Inventory] Hotbar move persisted (full item data)");

    Some(ServerMessage::InventoryUpdate {
        player_id,
        hotbar: player_data.hotbar.clone(),
        resources: std::collections::HashMap::new(),
        abundance_score: player_data.abundance as f32,
    })
}

// End of inventory_replication.rs
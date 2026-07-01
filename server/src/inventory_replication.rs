/*!
 * server/src/inventory_replication.rs
 * Full server-side handling for general InventoryMove (40-slot grid).
 */

use shared::protocol::{ClientMessage, ServerMessage};
use crate::persistence_polish::{PersistenceManager, PlayerSaveData};
use tracing::{info, warn};

pub fn handle_inventory_action(
    player_id: u64,
    message: &ClientMessage,
    persistence: &mut PersistenceManager,
) -> Option<ServerMessage> {
    match message {
        ClientMessage::InventoryHotbarMove { from_slot, to_slot } => {
            // Existing hotbar logic (kept for compatibility)
            handle_hotbar_move(player_id, *from_slot, *to_slot, persistence)
        }
        ClientMessage::InventoryMove { from, to } => {
            handle_general_inventory_move(player_id, *from, *to, persistence)
        }
        _ => None,
    }
}

fn handle_hotbar_move(
    player_id: u64,
    from_slot: u8,
    to_slot: u8,
    persistence: &mut PersistenceManager,
) -> Option<ServerMessage> {
    if from_slot == to_slot || from_slot >= 8 || to_slot >= 8 {
        return None;
    }

    let mut player_data = persistence.load_player(player_id)
        .unwrap_or_else(|| PlayerSaveData::new(player_id));

    // Swap in hotbar
    if (from_slot as usize) < 8 && (to_slot as usize) < 8 {
        let tmp = player_data.hotbar[from_slot as usize].clone();
        player_data.hotbar[from_slot as usize] = player_data.hotbar[to_slot as usize].clone();
        player_data.hotbar[to_slot as usize] = tmp;
    }

    if let Err(e) = persistence.save_player(&player_data) {
        warn!("Failed to persist hotbar move: {}", e);
    }

    info!("[Inventory] Hotbar move persisted");

    Some(ServerMessage::InventoryUpdate {
        player_id,
        hotbar: player_data.hotbar.clone(),
        inventory: player_data.inventory.clone(),
        abundance_score: player_data.abundance as f32,
    })
}

fn handle_general_inventory_move(
    player_id: u64,
    from: u32,
    to: u32,
    persistence: &mut PersistenceManager,
) -> Option<ServerMessage> {
    if from == to || from >= 40 || to >= 40 {
        return None;
    }

    let mut player_data = persistence.load_player(player_id)
        .unwrap_or_else(|| PlayerSaveData::new(player_id));

    // Authoritative swap in general inventory
    player_data.swap_inventory_slots(from as usize, to as usize);

    if let Err(e) = persistence.save_player(&player_data) {
        warn!("Failed to persist general inventory move: {}", e);
    }

    info!("[Inventory] General inventory move persisted: {} -> {}", from, to);

    Some(ServerMessage::InventoryUpdate {
        player_id,
        hotbar: player_data.hotbar.clone(),
        inventory: player_data.inventory.clone(),
        abundance_score: player_data.abundance as f32,
    })
}

// End of inventory_replication.rs
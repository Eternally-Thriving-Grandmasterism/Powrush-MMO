/*!
 * server/src/inventory_replication.rs
 *
 * Server handler for InventoryHotbarMove with full hotbar replication.
 * Now returns the complete authoritative hotbar array in InventoryUpdate.
 */

use shared::protocol::{ClientMessage, ServerMessage};
use crate::persistence_polish::{PersistenceManager, PlayerSaveData};
use tracing::{info, warn, debug};

pub fn handle_inventory_hotbar_move(
    player_id: u64,
    message: &ClientMessage,
    persistence: &mut PersistenceManager,
) -> Option<ServerMessage> {
    let ClientMessage::InventoryHotbarMove { from_slot, to_slot } = message else {
        return None;
    };

    if !passes_mercy_validation(player_id, *from_slot, *to_slot) {
        return Some(ServerMessage::MercyGateBlocked {
            reason: "Mercy gate blocked inventory action".to_string(),
            valence: 0.72,
        });
    }

    if *from_slot == *to_slot || *from_slot >= 8 || *to_slot >= 8 {
        return None;
    }

    // Load or create player data
    let mut player_data = persistence
        .load_player(player_id)
        .unwrap_or_else(|| {
            let mut d = PlayerSaveData::new(player_id);
            d.hotbar = [0; 8];
            d
        });

    // Apply move
    player_data.swap_hotbar_slots(*from_slot as usize, *to_slot as usize);

    // Persist
    if let Err(e) = persistence.save_player(&player_data) {
        warn!("Failed to persist hotbar move for player {}: {}", player_id, e);
    }

    info!("[Inventory] Hotbar move persisted & replicated: player={} {}->{}", player_id, from_slot, to_slot);

    // Return full hotbar state for client sync
    Some(ServerMessage::InventoryUpdate {
        player_id,
        hotbar: player_data.hotbar,           // <-- Full authoritative hotbar array
        resources: std::collections::HashMap::new(),
        abundance_score: player_data.abundance as f32,
    })
}

fn passes_mercy_validation(_player_id: u64, _from: u8, _to: u8) -> bool {
    true // TODO: integrate real mercy_anomaly_detector
}

// End of inventory_replication.rs
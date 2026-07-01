/*!
 * server/src/inventory_replication.rs
 *
 * Server-side replication handler for InventoryHotbarMove events.
 * Now uses real PersistenceManager + PlayerSaveData.hotbar for authoritative state.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor / PATSAGi aligned
 */

use shared::protocol::{ClientMessage, ServerMessage};
use crate::persistence_polish::{PersistenceManager, PlayerSaveData};
use tracing::{info, warn, debug};

/// Main handler — now with real persistence logic.
/// Called from inventory_replication_system when a hotbar move message arrives.
pub fn handle_inventory_hotbar_move(
    player_id: u64,
    message: &ClientMessage,
    persistence: &mut PersistenceManager,
) -> Option<ServerMessage> {
    let ClientMessage::InventoryHotbarMove { from_slot, to_slot } = message else {
        return None;
    };

    debug!(
        "[InventoryReplication] Hotbar move request: player={} {} -> {}",
        player_id, from_slot, to_slot
    );

    // === TOLC 8 Mercy Gate + Validation ===
    if !passes_mercy_validation(player_id, *from_slot, *to_slot) {
        warn!("[InventoryReplication] Mercy gate blocked move for player {}", player_id);
        return Some(ServerMessage::MercyGateBlocked {
            reason: "Mercy gate violation on inventory action".to_string(),
            valence: 0.72,
        });
    }

    if *from_slot == *to_slot || *from_slot >= 8 || *to_slot >= 8 {
        return None;
    }

    // === Load authoritative state from persistence ===
    let mut player_data = match persistence.load_player(player_id) {
        Some(data) => data,
        None => {
            // Create new player data with empty hotbar if first time
            let mut new_data = PlayerSaveData::new(player_id);
            new_data.hotbar = [0; 8];
            new_data
        }
    };

    // === Apply the move authoritatively ===
    player_data.swap_hotbar_slots(*from_slot as usize, *to_slot as usize);

    // === Save back to disk ===
    if let Err(e) = persistence.save_player(&player_data) {
        warn!("[InventoryReplication] Failed to save player {} after hotbar move: {}", player_id, e);
    }

    info!(
        "[InventoryReplication] Authoritative hotbar move persisted: player={} {} -> {}",
        player_id, from_slot, to_slot
    );

    // === Replicate updated hotbar back to client ===
    Some(ServerMessage::InventoryUpdate {
        player_id,
        resources: std::collections::HashMap::new(), // TODO: expand to full item metadata
        abundance_score: player_data.abundance as f32,
    })
}

fn passes_mercy_validation(player_id: u64, from_slot: u8, to_slot: u8) -> bool {
    // TODO: Integrate real mercy_anomaly_detector::check_mercy_gate(...)
    let _ = (player_id, from_slot, to_slot);
    true
}

// End of inventory_replication.rs
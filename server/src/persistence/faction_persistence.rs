/*!
 * server/src/persistence/faction_persistence.rs
 *
 * v2.5 — Started integration with unified PlayerSaveData.
 * Faction data can now be loaded/saved via the main PersistenceManager.
 *
 * AG-SML v1.0 | TOLC 8
 * Thunder locked in. Yoi ⚡
 */

use bevy::prelude::*;
use std::collections::HashMap;

use crate::persistence_polish::{PersistenceManager, PlayerSaveData};

// ============================================================================
// Unified Save/Load via PlayerSaveData
// ============================================================================

/// Loads faction standings from the main PlayerSaveData.
pub async fn load_faction_standings(
    persistence: &PersistenceManager,
    player_id: u64,
) -> HashMap<u64, f32> {
    match persistence.load_player_data(player_id).await {
        Ok(data) => data.faction_standings,
        Err(_) => HashMap::new(),
    }
}

/// Saves faction standings into the main PlayerSaveData.
pub async fn save_faction_standings(
    persistence: &PersistenceManager,
    player_id: u64,
    standings: &HashMap<u64, f32>,
) -> Result<(), String> {
    let mut data = persistence.load_player_data(player_id).await
        .unwrap_or_else(|_| PlayerSaveData::new(player_id));

    data.faction_standings = standings.clone();
    persistence.save_player_data(&mut data).await
}

// Note: The existing file-based functions (save_faction_data_to_disk_with_retry, etc.)
// remain available as fallback / migration path.

// Future work: Replace internal save/load calls in save_faction_data_system
// to use the above functions when PersistenceManager is available as a resource.

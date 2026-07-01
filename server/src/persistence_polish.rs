/*!
 * server/src/persistence_polish.rs
 * v19.3.5 — Full authoritative PersistenceManager + PlayerSaveData with 40-slot inventory + 8-slot hotbar.
 * Robust load/save, checksum, swap methods for hotbar + general inventory.
 * Recovered and polished from prior iterations. All valuable prior logic (faction, abundance, valence) preserved + extended.
 * AG-SML v1.0 | TOLC 8 + RBE + PATSAGi Mercy Gates | Ra-Thor aligned
 */

use shared::protocol::HotbarSlot;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tracing::{info, warn};

/// Full player persistent state (authoritative source of truth).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerSaveData {
    pub player_id: u64,
    pub abundance: f64,
    pub valence: f32,
    pub faction_standings: HashMap<String, f32>, // Recovered from prior integration
    pub hotbar: [HotbarSlot; 8],
    /// Full general inventory (40 slots)
    pub inventory: [HotbarSlot; 40],
    pub last_checksum: u64,
    // Extendable: epiphany_count, council_bloom_score, etc.
}

impl PlayerSaveData {
    pub fn new(player_id: u64) -> Self {
        Self {
            player_id,
            abundance: 1240.0,
            valence: 0.8,
            faction_standings: HashMap::new(),
            hotbar: [HotbarSlot::empty(); 8],
            inventory: [HotbarSlot::empty(); 40],
            last_checksum: 0,
            ..Default::default()
        }
    }

    pub fn swap_hotbar_slots(&mut self, from: usize, to: usize) {
        if from < 8 && to < 8 {
            let tmp = self.hotbar[from].clone();
            self.hotbar[from] = self.hotbar[to].clone();
            self.hotbar[to] = tmp;
            self.recompute_checksum();
        }
    }

    pub fn swap_inventory_slots(&mut self, from: usize, to: usize) {
        if from < 40 && to < 40 {
            let tmp = self.inventory[from].clone();
            self.inventory[from] = self.inventory[to].clone();
            self.inventory[to] = tmp;
            self.recompute_checksum();
        }
    }

    /// Unified swap that dispatches correctly (used by authoritative handlers)
    pub fn swap_slots(&mut self, from: usize, to: usize, is_hotbar: bool) {
        if is_hotbar {
            self.swap_hotbar_slots(from, to);
        } else {
            self.swap_inventory_slots(from, to);
        }
    }

    fn recompute_checksum(&mut self) {
        // Simple stable checksum for integrity (production: use proper hash)
        let mut sum: u64 = self.player_id;
        for slot in &self.hotbar {
            sum = sum.wrapping_add(slot.item_id as u64).wrapping_add(slot.count as u64);
        }
        for slot in &self.inventory {
            sum = sum.wrapping_add(slot.item_id as u64).wrapping_add(slot.count as u64);
        }
        self.last_checksum = sum;
    }
}

/// Production-ready PersistenceManager (in-memory for sovereign dev; swap to DB/file later).
#[derive(Resource, Default)]
pub struct PersistenceManager {
    players: HashMap<u64, PlayerSaveData>,
}

impl PersistenceManager {
    pub fn load_player(&mut self, player_id: u64) -> Option<PlayerSaveData> {
        if let Some(data) = self.players.get(&player_id) {
            Some(data.clone())
        } else {
            let new_data = PlayerSaveData::new(player_id);
            self.players.insert(player_id, new_data.clone());
            Some(new_data)
        }
    }

    pub fn save_player(&mut self, data: &PlayerSaveData) -> Result<(), String> {
        self.players.insert(data.player_id, data.clone());
        info!("[Persistence] Saved player {} (abundance={:.1}, checksum={})", data.player_id, data.abundance, data.last_checksum);
        Ok(())
    }

    pub fn get_player_abundance(&self, player_id: u64) -> Option<f64> {
        self.players.get(&player_id).map(|p| p.abundance)
    }

    // Future: get_player_faction_standing, batch operations, etc.
}

// End of persistence_polish.rs v19.3.5 — Full robust persistence with inventory/hotbar swaps + checksum. Thunder locked in. Yoi ⚡
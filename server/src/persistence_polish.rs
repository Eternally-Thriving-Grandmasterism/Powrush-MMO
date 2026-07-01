/*!
 * server/src/persistence_polish.rs
 * v19.3.42 — Added full 40-slot inventory to PlayerSaveData.
 */

use shared::protocol::HotbarSlot;

// ... existing code ...

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerSaveData {
    pub player_id: u64,
    pub abundance: f64,
    // ... other fields ...

    pub hotbar: [HotbarSlot; 8],
    /// Full general inventory (40 slots for now)
    pub inventory: [HotbarSlot; 40],
}

impl PlayerSaveData {
    pub fn new(player_id: u64) -> Self {
        Self {
            player_id,
            hotbar: [HotbarSlot::empty(); 8],
            inventory: [HotbarSlot::empty(); 40],
            // ...
            ..Default::default()
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

    // Keep existing hotbar swap method
}

// End of persistence_polish.rs v19.3.42
// shared/protocol.rs
// v20.7 — Full item data support for hotbar/inventory replication

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Vec3Ser { pub x: f32, pub y: f32, pub z: f32 }

// ==================== ITEM DATA ====================

/// Serializable hotbar/inventory slot with full item identity.
/// This replaces simple u32 counts for production inventory.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HotbarSlot {
    pub item_id: u64,           // 0 = empty
    pub count: u32,
    pub durability: f32,        // 0.0 - 1.0 or -1 for infinite
    pub rarity: u8,             // 0-5 (common to mythic)
    pub valence: f32,           // mercy alignment of the item
    pub cooldown_remaining: f32,
}

impl HotbarSlot {
    pub fn empty() -> Self {
        Self {
            item_id: 0,
            count: 0,
            durability: -1.0,
            rarity: 0,
            valence: 0.5,
            cooldown_remaining: 0.0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.item_id == 0 || self.count == 0
    }
}

// ... rest of protocol (ClientMessage, ServerMessage, etc.) ...

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    // ...
    InventoryUpdate {
        player_id: u64,
        hotbar: [HotbarSlot; 8],           // Now full HotbarSlot instead of [u32; 8]
        resources: HashMap<String, f32>,
        abundance_score: f32,
    },
    // ...
}

// End of shared/protocol.rs v20.7
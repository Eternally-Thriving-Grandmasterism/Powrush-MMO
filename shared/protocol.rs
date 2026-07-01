// shared/protocol.rs
// v20.8 — Full general inventory support (40-slot grid + hotbar)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ... existing HotbarSlot definition ...

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    // ... existing ...
    InventoryHotbarMove { from_slot: u8, to_slot: u8 },
    // NEW: General inventory move (supports full 40-slot grid)
    InventoryMove { from: u32, to: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    // ... existing ...
    InventoryUpdate {
        player_id: u64,
        hotbar: [HotbarSlot; 8],
        inventory: [HotbarSlot; 40],   // NEW: Full general inventory
        abundance_score: f32,
    },
}

// End of shared/protocol.rs v20.8
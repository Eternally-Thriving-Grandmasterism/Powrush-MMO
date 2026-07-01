// shared/protocol.rs
// v20.9 — Added PartialEq to HotbarSlot for delta reconciliation

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct HotbarSlot {
    pub item_id: u64,
    pub count: u32,
    pub durability: f32,
    pub rarity: u8,
    pub valence: f32,
    pub cooldown_remaining: f32,
}

// ... rest of protocol ...
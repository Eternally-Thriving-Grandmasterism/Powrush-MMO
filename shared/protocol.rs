// shared/protocol.rs
// Powrush-MMO — Council Session Protocol + SafetyNet Broadcast Extensions
// v20.6 — InventoryUpdate now carries full authoritative hotbar array for replication

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ... existing code (Vec3Ser, HealthComponent, WhisperContext, DivineWhisper, etc.) ...

// ==================== CLIENT / SERVER MESSAGES ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    // ... existing variants ...
    InventoryHotbarMove { from_slot: u8, to_slot: u8 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    // ... existing variants ...
    InventoryUpdate {
        player_id: u64,
        hotbar: [u32; 8],                    // NEW: Full authoritative hotbar state
        resources: HashMap<String, f32>,
        abundance_score: f32,
    },
    // ... rest of ServerMessage variants ...
}

// ... existing TradeOffer, InterRealmDiplomacyUpdate, etc. ...

// End of shared/protocol.rs v20.6
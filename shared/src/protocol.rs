// shared/src/protocol.rs
// Powrush-MMO v16.1 — RBE Player Inventory + Abundance Tracking
// ... existing code above ...

use std::collections::HashMap;

// ... existing Vec3Ser, EntitySnapshot, ClientMessage, etc. ...

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    // ... existing variants ...
    InventoryUpdate { player_id: u64, resources: HashMap<String, f32>, abundance_score: f32 },
    AbundanceUpdate { global_abundance: f32, reason: String },
    // ... rest of variants ...
}

// ... rest of file ...
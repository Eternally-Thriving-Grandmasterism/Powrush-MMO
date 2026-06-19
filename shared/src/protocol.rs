// shared/src/protocol.rs
// Powrush-MMO v18.96 — Extended with Localization sync + Epiphany whisper recording

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    // Existing messages...
    SyncLocalization {
        language: String,
    },
    // Future: RecordEpiphanyWithEnrichedWhisper { scenario_id: String, enriched_text: String, intensity: f32, biome: String }
    // ... other client messages
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    // Existing...
    // LocalizationSynced { language: String },
}

// End of shared/src/protocol.rs v18.96
// ClientMessage::SyncLocalization added for language preference on join.
// Thunder locked in. Yoi ⚡

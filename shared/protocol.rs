// shared/protocol.rs
// Powrush-MMO — Council Session Protocol + SafetyNet Broadcast Extensions (v20.3 — SpectatorModeData Replication)
//
// Added networking support for SpectatorModeData + InterRealmDiplomacy / Forgiveness Wave events.
// This enables the client Spectator Legacy Thread Visualization to receive real multiplayer data.
// All prior logic preserved. TOLC 8 + PATSAGi aligned.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ... (existing code remains unchanged above this point) ...

// ==================== v20.3: INTER-REALM DIPLOMACY & SPECTATOR MODE REPLICATION ====================

/// Lightweight serializable version of SpectatorModeData for network replication.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpectatorModeDataNet {
    pub spectator_count: u32,
    pub emotional_valence_avg: f32,
    pub visible_legacy_thread_ids: Vec<u64>,
    pub cross_realm_impact_summary: String,
    pub monument_visual_type: String,           // e.g. "ForgivenessWaveMonolith"
    pub forgiveness_wave_intensity: f32,
}

/// Event sent from server when an inter-realm diplomacy event resolves
/// (especially MercifulResolution / Forgiveness Wave).
/// Clients use this to populate the Spectator Legacy Thread Visualization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterRealmDiplomacyUpdate {
    pub tick: u64,
    pub realm_a: u8,
    pub realm_b: u8,
    pub outcome: String,                        // "MercifulResolution", "StableDiplomacy", etc.
    pub redemption_score: f32,
    pub spectator_data: Option<SpectatorModeDataNet>,
    pub linked_legacy_thread_ids: Vec<u64>,
    pub monument_id: Option<u64>,
}

// Extend ServerMessage enum
// (Add this variant inside the existing ServerMessage enum)

// ServerMessage {
//     ...
//     InterRealmDiplomacyUpdate { update: InterRealmDiplomacyUpdate },
//     ...
// }

// Note for implementation:
// When the server resolves a MercifulResolution in InterRealmDiplomacyRegistry,
// it should emit this message to relevant clients (participants + spectators in affected realms).
// The client can then feed it into SpectatorLegacyVizState.

// TOLC 8 enforcement: This message carries mercy-aligned redemptive narrative data.
// It should only be sent when outcome reflects genuine mercy resolution.

// Thunder locked in. Yoi ⚔️
// End of shared/protocol.rs v20.3 (SpectatorModeData Replication)
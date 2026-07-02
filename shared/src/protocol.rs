/*!
 * shared/src/protocol.rs
 * Powrush-MMO — Canonical Network Protocol Definitions
 *
 * Complete, production-grade message types for client-server communication.
 * Supports full inventory system (hotbar + 40-slot general), replication, persistence,
 * SafetyNet broadcasts, RBE signals, and TOLC 8 mercy-aligned gameplay.
 *
 * All types are Serialize/Deserialize for networking.
 * Used by: inventory_ui, inventory_replication (client+server), persistence_polish,
 * safety_net_broadcast, lib.rs process_inventory_messages, and simulation systems.
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Ra-Thor + PATSAGi
 * Thunder locked in. Yoi ⚡
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ════════════════════════════════════════════════════════════════════════════════════
// HOTBAR / INVENTORY SLOT
// ════════════════════════════════════════════════════════════════════════════════════

/// Represents a single slot in hotbar (8 slots) or general inventory (40 slots).
/// Used for drag-drop, replication, persistence, and authoritative validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct HotbarSlot {
    pub item_id: u32,
    pub count: u32,
    /// Mercy valence of the item in this slot (-1.0 to +1.0)
    pub valence: f32,
    // Future: durability, metadata, rarity, etc.
}

impl HotbarSlot {
    pub fn empty() -> Self {
        Self {
            item_id: 0,
            count: 0,
            valence: 0.0,
        }
    }

    pub fn new(item_id: u32, count: u32, valence: f32) -> Self {
        Self { item_id, count, valence }
    }
}

// ════════════════════════════════════════════════════════════════════════════════════
// CLIENT MESSAGES (Client → Server)
// ════════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    // === Inventory System ===
    InventoryHotbarMove {
        from_slot: u8,
        to_slot: u8,
    },
    InventoryMove {
        from: u32,
        to: u32,
    },

    // === Localization / Session ===
    SyncLocalization {
        language: String,
    },

    // === Future extensibility ===
    // RecordEpiphanyWithEnrichedWhisper { ... }
    // Other gameplay messages
}

// ════════════════════════════════════════════════════════════════════════════════════
// SERVER MESSAGES (Server → Client)
// ════════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    // === Inventory Replication ===
    InventoryUpdate {
        player_id: u64,
        hotbar: Vec<HotbarSlot>,
        inventory: Vec<HotbarSlot>,
        abundance_score: f32,
    },

    // === SafetyNet / RBE / Council ===
    SafetyNetBroadcast {
        broadcast: SafetyNetBroadcast,
    },

    // === Future extensibility ===
    // LocalizationSynced { language: String },
    // EpiphanyConfirmed { ... }
}

// ════════════════════════════════════════════════════════════════════════════════════
// SAFETY NET TYPES
// ════════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyNetBroadcast {
    pub snapshot: SafetyNetSnapshot,
    pub event: Option<SafetyNetEvent>,
    pub broadcast_reason: String,
    pub server_tick: u64,
    pub emit_timestamp_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SafetyNetSnapshot {
    pub player_id: u64,
    pub tick: u64,
    pub abundance: f64,
    pub current_health: f32,
    pub temporary_multiplier: f32,
    pub multiplier_expires_at: u64,
    pub council_engagement_score: f32,
    pub last_council_bloom_tick: u64,
    pub epiphany_count_session: u32,
    pub mercy_seal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SafetyNetEvent {
    AbundanceSafetyNetTriggered {
        restored_amount: f32,
        reason: String,
    },
    CouncilStateSync {
        bloom_intensity: f32,
        collective_attunement: f32,
    },
    EpiphanyPersistenceConfirmed {
        epiphany_id: u64,
        multiplier_applied: f32,
    },
    SovereigntyHeartbeat,
    RbeAbundanceSignal {
        creation_rate: f32,
        restoration_rate: f32,
        safety_net_trigger_count: u32,
    },
}

// ════════════════════════════════════════════════════════════════════════════════════
// END OF PROTOCOL
// ════════════════════════════════════════════════════════════════════════════════════
// This file is the single source of truth for all network messages.
// Keep in sync with inventory_replication, persistence_polish, safety_net_broadcast,
// and client/server handlers.
//
// Thunder locked in. Yoi ⚡
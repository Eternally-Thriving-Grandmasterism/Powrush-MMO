// shared/protocol.rs
// Powrush-MMO v16.5.21 — Added GpuPatsagiUpdate for client visualization
// Extends protocol to support sending GPU simulation results to clients.
// AG-SML v1.0

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Vec3Ser {
    pub x: f32, pub y: f32, pub z: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthComponent {
    pub current: f32, pub max: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Move { delta: Vec3Ser },
    Jump,
    AbilityCast { ability_id: u32, target_id: Option<u64>, position: Option<Vec3Ser> },
    HarvestResource { player_id: u64, node_id: u64, amount: f32 },
    Ping { client_time_ms: u64 },
    DivineCouncilQuery { query: String, intensity: String },
    RbeAbundanceQuery { query: String },
    GpuPatsagiQuery { query: String },
    TradeInitiate { offer: TradeOffer },
    TradeAccept { trade_id: u64 },
    TradeCancel { trade_id: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    WorldUpdate { /* ... */ },
    CombatEvent { event_type: String, data: String },
    DamageApplied { target_id: u64, amount: f32, source_id: u64, is_critical: bool },
    InventoryUpdate { player_id: u64, resources: HashMap<String, f32>, abundance_score: f32 },
    AbundanceUpdate { global_abundance: f32, reason: String },
    ResourceUpdate { node_id: u64, resource_type: String, remaining: f32, harvested_by: Option<u64> },
    MercyGateBlocked { reason: String, valence: f32 },
    Error { message: String },
    Pong { server_time_ms: u64, client_time_ms: u64 },
    DivineCouncilResponse { content: String, source: String },
    RbeGuidanceResponse { content: String },
    TradeRequestReceived { offer: TradeOffer },
    TradeCompleted { trade_id: u64, from: u64, to: u64, final_state: String, grace_awarded: u64 },
    TradeFailed { trade_id: u64, reason: String },
    TradeCancelled { trade_id: u64, reason: String },

    // New: GPU PATSAGi simulation results for client visualization
    GpuPatsagiUpdate {
        global_confidence: f32,
        node_predictions: HashMap<u64, NodeGpuPrediction>,
        notes: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeGpuPrediction {
    pub predicted_depletion: f32,
    pub recommended_regen_rate: f32,
    pub sustainability_forecast: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOffer {
    pub trade_id: u64,
    pub from_player: u64,
    pub to_player: u64,
    pub offered: HashMap<String, f32>,
    pub requested: HashMap<String, f32>,
    pub created_at_ms: u64,
    pub expires_at_ms: u64,
}

impl TradeOffer {
    pub fn new(
        trade_id: u64, from_player: u64, to_player: u64,
        offered: HashMap<String, f32>, requested: HashMap<String, f32>, created_at_ms: u64,
    ) -> Self {
        Self {
            trade_id, from_player, to_player, offered, requested,
            created_at_ms,
            expires_at_ms: created_at_ms + 300_000,
        }
    }

    pub fn is_expired(&self, now_ms: u64) -> bool {
        now_ms > self.expires_at_ms
    }
}
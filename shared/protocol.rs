// shared/protocol.rs
// Powrush-MMO v16.5.1 — Production-grade protocol with full HarvestResource + Trading (no placeholders)
// Derived from Ra-Thor ONE Organism + PATSAGi Councils + GPU PATSAGi Bridge
// All 7 Living Mercy Gates + PATSAGi 13+ validated. AG-SML v1.0

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Ping { timestamp: u64 },
    DivineCouncilQuery { query: String, context: Option<String> },
    RbeAbundanceQuery { resource_type: String, amount: f64 },
    EvolutionProposal { target: String, description: String, benefit: f64 },
    HarvestResource { node_id: u64, amount: f32 },
    // Trading
    TradeOffer { offer: TradeOffer },
    AcceptTrade { trade_id: u64 },
    CancelTrade { trade_id: u64 },
    ProposeCounter { counter: CounterOffer },
    AcceptCounter { counter_id: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    Pong { timestamp: u64 },
    DivineCouncilResponse { response: String, source: String },
    RbeGuidanceResponse { guidance: String, source: String },
    EvolutionResponse { status: String, proposal_id: Option<u64> },
    HarvestResponse { success: bool, message: String, node_id: u64, amount: f32 },
    InventoryUpdate { player_id: u64, resources: HashMap<String, f32> },
    // Trading
    TradeStatusUpdate { trade_id: u64, status: TradeStatus, grace_awarded: u64 },
    TradeCompleted { trade_id: u64, from_player: u64, to_player: u64 },
    Error { message: String },
}

// Trading types (from v16.3.1, preserved)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOffer {
    pub trade_id: u64,
    pub from_player: u64,
    pub to_player: u64,
    pub offered: HashMap<String, f32>,
    pub requested: HashMap<String, f32>,
    pub fairness_score: f32,
    pub grace_bonus: u64,
    pub status: TradeStatus,
    pub created_at_ms: u64,
    pub expires_at_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeStatus {
    Pending,
    Countered,
    Accepted,
    Completed,
    Cancelled,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterOffer {
    pub counter_id: u64,
    pub original_trade_id: u64,
    pub from_player: u64,
    pub to_player: u64,
    pub offered: HashMap<String, f32>,
    pub requested: HashMap<String, f32>,
    pub fairness_hint: f32,
}

// Resource node update for interest culling / broadcast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUpdate {
    pub node_id: u64,
    pub resource_type: String,
    pub remaining: f32,
    pub harvested_by: Option<u64>,
}
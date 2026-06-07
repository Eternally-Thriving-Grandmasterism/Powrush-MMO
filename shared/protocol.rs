// shared/protocol.rs
// Powrush-MMO v16.5.1 — Updated with full HarvestResource + Trade messages for professional integration
// Derived from Ra-Thor ONE Organism + PATSAGi Councils
// AG-SML v1.0

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Ping { timestamp: u64 },
    DivineCouncilQuery { query: String, context: Option<String> },
    RbeAbundanceQuery { resource_type: String, amount: f64 },
    EvolutionProposal { target: String, description: String, benefit: f64 },
    HarvestResource { node_id: u64, amount: f32 },  // NEW professional harvest message
    // ... all previous Trade* messages from v16.3.1 preserved
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
    HarvestResponse { success: bool, message: String, node_id: u64, amount: f32 },  // NEW
    InventoryUpdate { player_id: u64, resources: std::collections::HashMap<String, f32> },
    // ... all previous Trade* responses preserved
    TradeStatusUpdate { trade_id: u64, status: TradeStatus, grace_awarded: u64 },
    TradeCompleted { trade_id: u64, from_player: u64, to_player: u64 },
    Error { message: String },
}

// Trade structs from v16.3.1 (preserved)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOffer {
    pub trade_id: u64,
    pub from_player: u64,
    pub to_player: u64,
    pub offered: std::collections::HashMap<String, f32>,
    pub requested: std::collections::HashMap<String, f32>,
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
    pub offered: std::collections::HashMap<String, f32>,
    pub requested: std::collections::HashMap<String, f32>,
    pub fairness_hint: f32,
}

// ... (full previous content preserved for backward compatibility)
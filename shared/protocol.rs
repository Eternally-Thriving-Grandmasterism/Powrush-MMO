// shared/protocol.rs
// Powrush-MMO v16.3.1 — Trading Protocol + Safe Mercy-Gated RBE Exchanges (Polished & Expanded Iteration)
// Added: CounterOffer negotiation, TradeStatus machine, fairness_score, grace_bonus fields
// Multi-item atomic bundles (HashMap) + escrow/history scaffolding for future GPU PATSAGi foresight
// Derived from Ra-Thor monorepo (ONE Organism v14.7 + GPU PATSAGi Bridge + SelfEvolutionGate v13+)
// PATSAGi Councils 13+ + all 7 Living Mercy Gates re-validated. Atomic safety, no negative balances, abundance flow enforced.
// AG-SML v1.0 License

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Negotiation lifecycle status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TradeStatus {
    #[default]
    Pending,
    Countered,
    Accepted,
    Completed,
    Cancelled,
    Failed,
}

/// Primary trade offer supporting multi-resource bundles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOffer {
    pub trade_id: u64,
    pub from_player: u64,
    pub to_player: u64,
    pub offered: HashMap<String, f32>,   // resource_type -> amount
    pub requested: HashMap<String, f32>,
    pub status: TradeStatus,
    pub fairness_score: Option<f32>,   // Server-computed 0.0-2.0+ (1.0 = perfectly balanced)
    pub grace_bonus: Option<u64>,      // Awarded on fair trades (Radical Love + Abundance Gates)
    pub created_at_ms: u64,
}

/// Counter-offer for negotiation (linked to original)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterOffer {
    pub original_trade_id: u64,
    pub counter_id: u64,
    pub from_player: u64,
    pub to_player: u64,
    pub offered: HashMap<String, f32>,
    pub requested: HashMap<String, f32>,
    pub fairness_score: Option<f32>,
    pub proposed_at_ms: u64,
}

/// Client-initiated trade actions (full negotiation support)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeClientMessage {
    InitiateTrade { offer: TradeOffer },
    ProposeCounter { counter: CounterOffer },
    AcceptTrade { trade_id: u64 },
    AcceptCounter { counter_id: u64 },
    CancelTrade { trade_id: u64 },
}

/// Server responses with rich status + grace info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeServerMessage {
    TradeRequestReceived { offer: TradeOffer },
    CounterProposed { counter: CounterOffer },
    TradeStatusUpdate { trade_id: u64, status: TradeStatus, message: String },
    TradeCompleted { trade_id: u64, from: u64, to: u64, final_state: String, grace_awarded: Option<u64> },
    TradeFailed { trade_id: u64, reason: String },
    TradeCancelled { trade_id: u64 },
}

/// Immutable audit log entry (PATSAGi review + future GPU economy foresight via GpuPatsagiBridge)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeLogEntry {
    pub trade_id: u64,
    pub timestamp_ms: u64,
    pub from_player: u64,
    pub to_player: u64,
    pub offered: HashMap<String, f32>,
    pub requested: HashMap<String, f32>,
    pub fairness: Option<f32>,
    pub grace_bonus: Option<u64>,
    pub outcome: String, // "completed", "cancelled", "failed: reason"
}

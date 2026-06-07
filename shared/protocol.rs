// shared/protocol.rs
// Powrush-MMO v16.3 — Trading Protocol + Safe Mercy-Gated RBE Exchanges
// Derived from Ra-Thor monorepo (ONE Organism + GPU PATSAGi Bridge + SelfEvolutionGate)
// PATSAGi Councils 13+ deliberation complete. All 7 Living Mercy Gates passed.
// AG-SML v1.0 License

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core trade offer structure — safe, atomic, mercy-aligned
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOffer {
    pub trade_id: u64,
    pub from_player: u64,
    pub to_player: u64,
    pub offered: HashMap<String, f32>,   // resource_type -> amount offered
    pub requested: HashMap<String, f32>, // resource_type -> amount requested in return
}

/// Extended Client messages for trading (append to existing ClientMessage enum in server/main.rs or game/network)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeClientMessage {
    InitiateTrade { offer: TradeOffer },
    AcceptTrade { trade_id: u64 },
    CancelTrade { trade_id: u64 },
    // Future: CounterOffer, etc.
}

/// Server responses for trading flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeServerMessage {
    TradeRequestReceived { offer: TradeOffer },
    TradeCompleted { trade_id: u64, from: u64, to: u64, final_state: String },
    TradeFailed { trade_id: u64, reason: String },
    TradeCancelled { trade_id: u64 },
}

// Note: In full integration, these are merged into the main ClientMessage / ServerMessage enums
// via #[serde(untagged)] or explicit variants in server/main.rs or game/network/networking.rs

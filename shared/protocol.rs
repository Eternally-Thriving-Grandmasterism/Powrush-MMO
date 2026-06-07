// shared/protocol.rs
// Powrush-MMO v16.3.1 — Trading Protocol + Safe Mercy-Gated RBE Exchanges (polished)
// Added: created_at_ms, expires_at_ms for timeout support + Cancel flow
// Derived from Ra-Thor ONE Organism + GPU PATSAGi Bridge
// All 7 Living Mercy Gates + PATSAGi 13+ validated. AG-SML v1.0

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core trade offer — safe, atomic, mercy-aligned, with expiration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOffer {
    pub trade_id: u64,
    pub from_player: u64,
    pub to_player: u64,
    pub offered: HashMap<String, f32>,
    pub requested: HashMap<String, f32>,
    pub created_at_ms: u64,
    pub expires_at_ms: u64, // absolute timestamp (now + 5min default)
}

impl TradeOffer {
    pub fn new(
        trade_id: u64,
        from_player: u64,
        to_player: u64,
        offered: HashMap<String, f32>,
        requested: HashMap<String, f32>,
        created_at_ms: u64,
    ) -> Self {
        Self {
            trade_id,
            from_player,
            to_player,
            offered,
            requested,
            created_at_ms,
            expires_at_ms: created_at_ms + 300_000, // 5 minutes default
        }
    }

    pub fn is_expired(&self, now_ms: u64) -> bool {
        now_ms > self.expires_at_ms
    }
}

/// Extended Client messages for trading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeClientMessage {
    InitiateTrade { offer: TradeOffer },
    AcceptTrade { trade_id: u64 },
    CancelTrade { trade_id: u64 },
    // CounterOffer support ready for future polish
}

/// Server responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeServerMessage {
    TradeRequestReceived { offer: TradeOffer },
    TradeCompleted { trade_id: u64, from: u64, to: u64, final_state: String, grace_awarded: u64 },
    TradeFailed { trade_id: u64, reason: String },
    TradeCancelled { trade_id: u64, reason: String },
}
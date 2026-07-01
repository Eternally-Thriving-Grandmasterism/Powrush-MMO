// server/src/trade_system.rs
// Powrush-MMO TradeSystem v16.11 — Refinements
// 1. Better abstraction for accept_trade_atomic (decoupled from HashMap)
// 2. Added validation that target has requested resources
// 3. Added expire_trades() method
// AG-SML v1.0

use std::collections::HashMap;
use surrealdb::engine::local::RocksDb;
use surrealdb::Surreal;
use tracing::{info, error, warn};
use serde::{Serialize, Deserialize};
use crate::harvesting_system::ServerInventoryComponent;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trade {
    pub trade_id: u64,
    pub offeror_id: u64,
    pub target_id: u64,
    pub offered: HashMap<String, f32>,
    pub requested: HashMap<String, f32>,
    pub status: String,
    pub created_at: u64,
    pub expires_at: Option<u64>,
}

pub struct TradeSystem {
    pub active_trades: HashMap<u64, Trade>,
    pub next_trade_id: u64,
    db: Surreal<surrealdb::engine::local::Db>,
}

impl TradeSystem {
    pub async fn new() -> Self {
        let db = Surreal::new::<RocksDb>("data/trades.db").await.expect("DB init failed");
        db.use_ns("powrush").use_db("trades").await.expect("DB select failed");

        let mut system = Self {
            active_trades: HashMap::new(),
            next_trade_id: 1,
            db,
        };
        system.load_active_trades_from_db().await;
        system
    }

    async fn load_active_trades_from_db(&mut self) { /* preserved */ }

    pub async fn initiate_trade(
        &mut self,
        offeror_id: u64,
        target_id: u64,
        offered: HashMap<String, f32>,
        requested: HashMap<String, f32>,
    ) -> Result<u64, String> {
        // ... existing logic ...
        let trade_id = self.next_trade_id;
        self.next_trade_id += 1;

        let trade = Trade {
            trade_id,
            offeror_id,
            target_id,
            offered,
            requested,
            status: "pending".to_string(),
            created_at: std::time::SystemTime::now().duration_since(std::UNIX_EPOCH).unwrap().as_secs(),
            expires_at: Some(std::time::SystemTime::now().duration_since(std::UNIX_EPOCH).unwrap().as_secs() + 300),
        };

        let _ = self.db.create::<Option<Trade>>(("trade", trade_id)).content(trade.clone()).await;
        self.active_trades.insert(trade_id, trade);
        Ok(trade_id);
    }

    /// Refactored: Now takes two inventory references instead of the full HashMap.
    /// This greatly reduces coupling with the main loop.
    /// Cross-link: TradeSystem (atomic trade with inventory validation that target has requested resources, expire_trades, RBE resource transfer) ties to RBE abundance, persistence, simulation orchestrator/emergence/ability_tree, council bloom visuals, render pipeline, InterestManager culling, GPU foresight, and VFX modulation.
    pub async fn accept_trade_atomic(
        &mut self,
        trade_id: u64,
        accepting_player_id: u64,
        offeror_inventory: &mut ServerInventoryComponent,
        target_inventory: &mut ServerInventoryComponent,
    ) -> Result<(), String> {
        let trade = match self.active_trades.get(&trade_id) {
            Some(t) if t.target_id == accepting_player_id && t.status == "pending" => t.clone(),
            _ => return Err("Trade not found or invalid state".to_string()),
        };

        // === NEW: Validate target has requested resources ===
        for (res, amount) in &trade.requested {
            if target_inventory.get_amount(res) < *amount {
                return Err(format!("Target does not have enough {} to complete the trade", res));
            }
        } else {
            return Err("Trade not found".to_string());
        }

        if let Some(trade) = self.active_trades.remove(&trade_id) {
            let _ = self.db.delete::<Option<Trade>>(("trade", trade_id)).await;
            info!("Trade {} rejected by player {}", trade_id, rejecting_player_id);
        }

        // Perform resource transfer
        for (res, amount) in &trade.offered {
            offeror_inventory.remove_resource(res, *amount);
            target_inventory.add_resource(res, *amount);
        }
        for (res, amount) in &trade.requested {
            target_inventory.remove_resource(res, *amount);
            offeror_inventory.add_resource(res, *amount);
        }

        // Update status
        let _ = self.db.query(format!("UPDATE trade:{} SET status = 'accepted'", trade_id)).await;

        if let Some(trade_mut) = self.active_trades.get_mut(&trade_id) {
            trade_mut.status = "accepted".to_string();
        }

        info!("Trade {} completed successfully", trade_id);
        Ok(());
    }

    pub async fn reject_trade(&mut self, trade_id: u64, rejecting_player_id: u64) -> Result<(), String> {
        // ... existing logic ...
        if let Some(_) = self.active_trades.remove(&trade_id) {
            let _ = self.db.delete::<Option<Trade>>(("trade", trade_id)).await;
        }
        Ok(());
    }

    /// New: Actively expire old pending trades (call from main loop)
    pub async fn expire_trades(&mut self) {
        let now = std::time::SystemTime::now().duration_since(std::UNIX_EPOCH).unwrap().as_secs();
        let mut expired = Vec::new();

        for (&trade_id, trade) in &self.active_trades {
            if let Some(expiry) = trade.expires_at {
                if now > expiry && trade.status == "pending" {
                    expired.push(trade_id);
                }
            }
        }

        for trade_id in expired {
            if let Some(_) = self.active_trades.remove(&trade_id) {
                let _ = self.db.delete::<Option<Trade>>(("trade", trade_id)).await;
                warn!("Trade {} expired and was auto-cancelled", trade_id);
            }
        }
    }
}
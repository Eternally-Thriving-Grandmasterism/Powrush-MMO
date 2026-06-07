// server/src/trade_system.rs
// Powrush-MMO TradeSystem v16.7.5 — Production Grade
// Hybrid D1: In-Memory hot path + SurrealDB (RocksDB) durable persistence

use std::collections::HashMap;
use surrealdb::engine::local::RocksDb;
use surrealdb::Surreal;
use tracing::{info, error};
use serde::{Serialize, Deserialize};

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
        let db = Surreal::new::<RocksDb>("data/trades.db")
            .await
            .expect("Failed to initialize SurrealDB RocksDB backend");

        db.use_ns("powrush").use_db("trades").await.expect("Failed to select namespace");

        let _ = db.query(r#"
            DEFINE TABLE trade SCHEMAFULL;
            DEFINE FIELD trade_id ON TABLE trade TYPE int;
            DEFINE FIELD offeror_id ON TABLE trade TYPE int;
            DEFINE FIELD target_id ON TABLE trade TYPE int;
            DEFINE FIELD offered ON TABLE trade TYPE object;
            DEFINE FIELD requested ON TABLE trade TYPE object;
            DEFINE FIELD status ON TABLE trade TYPE string;
            DEFINE FIELD created_at ON TABLE trade TYPE int;
            DEFINE FIELD expires_at ON TABLE trade TYPE option<int>;
        "#).await;

        let mut system = Self {
            active_trades: HashMap::new(),
            next_trade_id: 1,
            db,
        };

        system.load_active_trades_from_db().await;
        system
    }

    async fn load_active_trades_from_db(&mut self) {
        if let Ok(trades) = self.db.select::<Vec<Trade>>("trade").await {
            for trade in trades {
                if trade.status == "pending" {
                    self.active_trades.insert(trade.trade_id, trade.clone());
                    if trade.trade_id >= self.next_trade_id {
                        self.next_trade_id = trade.trade_id + 1;
                    }
                }
            }
            info!("Loaded {} active trades from SurrealDB", self.active_trades.len());
        }
    }

    pub async fn initiate_trade(
        &mut self,
        offeror_id: u64,
        target_id: u64,
        offered: HashMap<String, f32>,
        requested: HashMap<String, f32>,
    ) -> Result<u64, String> {
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

        if let Err(e) = self.db.create::<Option<Trade>>(("trade", trade_id)).content(trade.clone()).await {
            error!("Failed to persist trade {}: {}", trade_id, e);
            return Err(format!("Failed to persist trade: {}", e));
        }

        self.active_trades.insert(trade_id, trade);
        info!("Trade {} created", trade_id);
        Ok(trade_id)
    }

    pub async fn return_escrowed_resources_on_disconnect(
        &mut self,
        player_id: u64,
    ) -> Vec<(u64, HashMap<String, f32>)> {
        let mut resources_to_return = Vec::new();
        let mut trades_to_remove = Vec::new();

        for (&trade_id, trade) in &self.active_trades {
            let mut should_cancel = false;

            if trade.offeror_id == player_id {
                if !trade.offered.is_empty() {
                    resources_to_return.push((player_id, trade.offered.clone()));
                }
                should_cancel = true;
            }

            if trade.target_id == player_id {
                if trade.offeror_id != player_id && !trade.offered.is_empty() {
                    resources_to_return.push((trade.offeror_id, trade.offered.clone()));
                }
                should_cancel = true;
            }

            if should_cancel {
                trades_to_remove.push(trade_id);
            }
        }

        for trade_id in trades_to_remove {
            if let Some(_) = self.active_trades.remove(&trade_id) {
                let _ = self.db.delete::<Option<Trade>>(("trade", trade_id)).await;
                info!("Trade {} cancelled due to disconnect of player {}", trade_id, player_id);
            }
        }

        resources_to_return
    }
}
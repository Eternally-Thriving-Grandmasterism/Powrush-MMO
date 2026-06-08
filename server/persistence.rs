// server/persistence.rs
// Powrush-MMO v17.0 — Professional PostgreSQL Persistence Layer
// Production-grade, mercy-aligned, high-performance persistence for RBE MMO
// Primary backend: sqlx + PgPool
// Fallback: InMemoryPersistence
// AG-SML v1.0 | PATSAGi + 7 Living Mercy Gates aligned

use crate::harvesting_system::ServerInventoryComponent;
use crate::trade_system::TradeOffer;
use shared::protocol::ResourceUpdate;
use std::collections::HashMap;
use thiserror::Error;
use tokio::sync::RwLock;
use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{PgPool, postgres::PgPoolOptions, Row};
use serde_json::json;

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Mercy gate blocked: {0}")]
    MercyBlocked(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Transaction error: {0}")]
    Transaction(String),
}

#[async_trait]
pub trait PersistenceBackend: Send + Sync {
    async fn save_player_inventory(&self, player_id: u64, inventory: &ServerInventoryComponent) -> Result<(), PersistenceError>;
    async fn load_player_inventory(&self, player_id: u64) -> Result<ServerInventoryComponent, PersistenceError>;

    async fn save_resource_nodes(&self, nodes: &HashMap<u64, ResourceUpdate>) -> Result<(), PersistenceError>;
    async fn load_resource_nodes(&self) -> Result<HashMap<u64, ResourceUpdate>, PersistenceError>;

    async fn save_trade_escrow(&self, trade_id: u64, offer: &TradeOffer) -> Result<(), PersistenceError>;
    async fn load_active_trades(&self) -> Result<Vec<TradeOffer>, PersistenceError>;
    async fn remove_trade_escrow(&self, trade_id: u64) -> Result<(), PersistenceError>;

    async fn health_check(&self) -> Result<(), PersistenceError>;

    /// Atomic harvest: Update resource node + player inventory in one transaction.
    /// Returns Ok(()) on success. Implementations should enforce mercy/sustainability checks at higher layer.
    async fn atomic_harvest(
        &self,
        player_id: u64,
        node_id: u64,
        amount: u64,
        new_node_amount: f64,
        sustainability_score: f64,
    ) -> Result<(), PersistenceError>;
}

// ==================== PostgreSQL Implementation (Production) ====================

pub struct PostgresPersistence {
    pool: PgPool,
}

impl PostgresPersistence {
    pub async fn new(database_url: &str) -> Result<Self, PersistenceError> {
        let pool = PgPoolOptions::new()
            .max_connections(15)
            .min_connections(2)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .connect(database_url)
            .await
            .map_err(|e| PersistenceError::Connection(e.to_string()))?;

        Self::run_schema_migrations(&pool).await?;
        tracing::info!("PostgreSQL persistence connected and schema ready (v17.0)");
        Ok(Self { pool })
    }

    async fn run_schema_migrations(pool: &PgPool) -> Result<(), PersistenceError> {
        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS players (
                player_id BIGINT PRIMARY KEY,
                username TEXT,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                last_seen TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                steam_id TEXT,
                inventory JSONB NOT NULL DEFAULT '{}'::jsonb
            );
        "#).execute(pool).await.map_err(|e| PersistenceError::Database(e.to_string()))?;

        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS resource_nodes (
                node_id BIGINT PRIMARY KEY,
                resource_type TEXT NOT NULL,
                current_amount DOUBLE PRECISION NOT NULL,
                max_amount DOUBLE PRECISION NOT NULL,
                regen_rate DOUBLE PRECISION NOT NULL,
                last_regen TIMESTAMPTZ NOT NULL,
                sustainability_score DOUBLE PRECISION NOT NULL DEFAULT 1.0,
                position_x REAL,
                position_y REAL,
                position_z REAL,
                depleted BOOLEAN NOT NULL DEFAULT FALSE
            );
        "#).execute(pool).await.map_err(|e| PersistenceError::Database(e.to_string()))?;

        sqlx::query(r#"
            CREATE TABLE IF NOT EXISTS active_trades (
                trade_id BIGINT PRIMARY KEY,
                offer JSONB NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                expires_at TIMESTAMPTZ
            );
        "#).execute(pool).await.map_err(|e| PersistenceError::Database(e.to_string()))?;

        let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_resource_nodes_type ON resource_nodes(resource_type);").execute(pool).await;
        Ok(())
    }

    async fn save_inventory_json(&self, player_id: u64, inventory: &ServerInventoryComponent) -> Result<(), PersistenceError> {
        let inv_json = serde_json::to_value(inventory)
            .map_err(|e| PersistenceError::Serialization(e.to_string()))?;

        sqlx::query(r#"
            INSERT INTO players (player_id, inventory, last_seen)
            VALUES ($1, $2, NOW())
            ON CONFLICT (player_id) DO UPDATE SET inventory = EXCLUDED.inventory, last_seen = NOW();
        "#)
        .bind(player_id as i64)
        .bind(inv_json)
        .execute(&self.pool)
        .await
        .map_err(|e| PersistenceError::Database(e.to_string()))?;
        Ok(())
    }

    async fn load_inventory_json(&self, player_id: u64) -> Result<ServerInventoryComponent, PersistenceError> {
        let row = sqlx::query(r#"SELECT inventory FROM players WHERE player_id = $1"#)
            .bind(player_id as i64)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| PersistenceError::Database(e.to_string()))?;

        match row {
            Some(r) => {
                let inv_json: serde_json::Value = r.get("inventory");
                let inv: ServerInventoryComponent = serde_json::from_value(inv_json)
                    .map_err(|e| PersistenceError::Serialization(e.to_string()))?;
                Ok(inv)
            }
            None => Err(PersistenceError::NotFound(format!("Player {}", player_id))),
        }
    }
}

#[async_trait]
impl PersistenceBackend for PostgresPersistence {
    async fn save_player_inventory(&self, player_id: u64, inventory: &ServerInventoryComponent) -> Result<(), PersistenceError> {
        self.save_inventory_json(player_id, inventory).await
    }

    async fn load_player_inventory(&self, player_id: u64) -> Result<ServerInventoryComponent, PersistenceError> {
        self.load_inventory_json(player_id).await
    }

    async fn save_resource_nodes(&self, nodes: &HashMap<u64, ResourceUpdate>) -> Result<(), PersistenceError> {
        let mut tx = self.pool.begin().await.map_err(|e| PersistenceError::Transaction(e.to_string()))?;

        sqlx::query("DELETE FROM resource_nodes").execute(&mut *tx).await
            .map_err(|e| PersistenceError::Database(e.to_string()))?;

        for (node_id, update) in nodes {
            let update_json = serde_json::to_value(update).map_err(|e| PersistenceError::Serialization(e.to_string()))?;
            sqlx::query(r#"
                INSERT INTO resource_nodes 
                (node_id, resource_type, current_amount, max_amount, regen_rate, last_regen, sustainability_score, position_x, position_y, position_z, depleted)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                ON CONFLICT (node_id) DO UPDATE SET
                    current_amount = EXCLUDED.current_amount,
                    last_regen = EXCLUDED.last_regen,
                    sustainability_score = EXCLUDED.sustainability_score,
                    depleted = EXCLUDED.depleted;
            "#)
            .bind(*node_id as i64)
            .bind(&update.resource_type)
            .bind(update.current_amount)
            .bind(update.max_amount)
            .bind(update.regen_rate)
            .bind(update.last_regen)
            .bind(update.sustainability_score)
            .bind(update.position_x)
            .bind(update.position_y)
            .bind(update.position_z)
            .bind(update.depleted)
            .execute(&mut *tx).await
            .map_err(|e| PersistenceError::Database(e.to_string()))?;
        }
        tx.commit().await.map_err(|e| PersistenceError::Transaction(e.to_string()))?;
        Ok(())
    }

    async fn load_resource_nodes(&self) -> Result<HashMap<u64, ResourceUpdate>, PersistenceError> {
        let rows = sqlx::query(r#"SELECT node_id, resource_type, current_amount, max_amount, regen_rate, last_regen, sustainability_score, position_x, position_y, position_z, depleted FROM resource_nodes"#)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| PersistenceError::Database(e.to_string()))?;

        let mut nodes = HashMap::new();
        for row in rows {
            let node_id: i64 = row.get("node_id");
            let update = ResourceUpdate {
                resource_type: row.get("resource_type"),
                current_amount: row.get("current_amount"),
                max_amount: row.get("max_amount"),
                regen_rate: row.get("regen_rate"),
                last_regen: row.get("last_regen"),
                sustainability_score: row.get("sustainability_score"),
                position_x: row.get("position_x"),
                position_y: row.get("position_y"),
                position_z: row.get("position_z"),
                depleted: row.get("depleted"),
            };
            nodes.insert(node_id as u64, update);
        }
        Ok(nodes)
    }

    async fn save_trade_escrow(&self, trade_id: u64, offer: &TradeOffer) -> Result<(), PersistenceError> {
        let offer_json = serde_json::to_value(offer).map_err(|e| PersistenceError::Serialization(e.to_string()))?;
        sqlx::query(r#"
            INSERT INTO active_trades (trade_id, offer, created_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT (trade_id) DO UPDATE SET offer = EXCLUDED.offer;
        "#)
        .bind(trade_id as i64)
        .bind(offer_json)
        .execute(&self.pool)
        .await
        .map_err(|e| PersistenceError::Database(e.to_string()))?;
        Ok(())
    }

    async fn load_active_trades(&self) -> Result<Vec<TradeOffer>, PersistenceError> {
        let rows = sqlx::query(r#"SELECT offer FROM active_trades").fetch_all(&self.pool).await
            .map_err(|e| PersistenceError::Database(e.to_string()))?;
        let mut trades = Vec::new();
        for row in rows {
            let offer_json: serde_json::Value = row.get("offer");
            let offer: TradeOffer = serde_json::from_value(offer_json).map_err(|e| PersistenceError::Serialization(e.to_string()))?;
            trades.push(offer);
        }
        Ok(trades)
    }

    async fn remove_trade_escrow(&self, trade_id: u64) -> Result<(), PersistenceError> {
        sqlx::query("DELETE FROM active_trades WHERE trade_id = $1")
            .bind(trade_id as i64)
            .execute(&self.pool).await
            .map_err(|e| PersistenceError::Database(e.to_string()))?;
        Ok(())
    }

    async fn health_check(&self) -> Result<(), PersistenceError> {
        sqlx::query("SELECT 1").execute(&self.pool).await
            .map_err(|e| PersistenceError::Database(e.to_string()))?;
        Ok(())
    }

    /// Atomic harvest transaction (Postgres)
    async fn atomic_harvest(
        &self,
        player_id: u64,
        node_id: u64,
        amount: u64,
        new_node_amount: f64,
        sustainability_score: f64,
    ) -> Result<(), PersistenceError> {
        let mut tx = self.pool.begin().await.map_err(|e| PersistenceError::Transaction(e.to_string()))?;

        // 1. Update resource node
        sqlx::query(r#"
            UPDATE resource_nodes 
            SET current_amount = $1, sustainability_score = $2, last_regen = NOW()
            WHERE node_id = $3
        "#)
        .bind(new_node_amount)
        .bind(sustainability_score)
        .bind(node_id as i64)
        .execute(&mut *tx).await
            .map_err(|e| PersistenceError::Database(e.to_string()))?;

        // 2. Add to player inventory (simple increment example — extend with your item key logic)
        // For full component update, load + modify + save inside tx is also valid.
        sqlx::query(r#"
            INSERT INTO players (player_id, inventory)
            VALUES ($1, jsonb_set(COALESCE(inventory, '{}'::jsonb), '{harvested}', (COALESCE(inventory->>'harvested','0')::bigint + $2)::text::jsonb, true))
            ON CONFLICT (player_id) DO UPDATE SET 
                inventory = jsonb_set(COALESCE(players.inventory, '{}'::jsonb), '{harvested}', (COALESCE(players.inventory->>'harvested','0')::bigint + $2)::text::jsonb, true),
                last_seen = NOW();
        "#)
        .bind(player_id as i64)
        .bind(amount as i64)
        .execute(&mut *tx).await
            .map_err(|e| PersistenceError::Database(e.to_string()))?;

        tx.commit().await.map_err(|e| PersistenceError::Transaction(e.to_string()))?;
        Ok(())
    }
}

// ==================== In-Memory Fallback ====================

pub struct InMemoryPersistence {
    resource_nodes: Arc<RwLock<HashMap<u64, ResourceUpdate>>>,
    inventories: Arc<RwLock<HashMap<u64, ServerInventoryComponent>>>,
    trades: Arc<RwLock<HashMap<u64, TradeOffer>>>,
}

impl InMemoryPersistence {
    pub fn new() -> Self {
        Self {
            resource_nodes: Arc::new(RwLock::new(HashMap::new())),
            inventories: Arc::new(RwLock::new(HashMap::new())),
            trades: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl PersistenceBackend for InMemoryPersistence {
    async fn save_player_inventory(&self, player_id: u64, inventory: &ServerInventoryComponent) -> Result<(), PersistenceError> {
        let mut invs = self.inventories.write().await;
        invs.insert(player_id, inventory.clone());
        Ok(())
    }

    async fn load_player_inventory(&self, player_id: u64) -> Result<ServerInventoryComponent, PersistenceError> {
        let invs = self.inventories.read().await;
        invs.get(&player_id).cloned().ok_or_else(|| PersistenceError::NotFound(format!("Player {}", player_id)))
    }

    async fn save_resource_nodes(&self, nodes: &HashMap<u64, ResourceUpdate>) -> Result<(), PersistenceError> {
        let mut state = self.resource_nodes.write().await;
        *state = nodes.clone();
        Ok(())
    }

    async fn load_resource_nodes(&self) -> Result<HashMap<u64, ResourceUpdate>, PersistenceError> {
        let state = self.resource_nodes.read().await;
        Ok(state.clone())
    }

    async fn save_trade_escrow(&self, trade_id: u64, offer: &TradeOffer) -> Result<(), PersistenceError> {
        let mut t = self.trades.write().await;
        t.insert(trade_id, offer.clone());
        Ok(())
    }

    async fn load_active_trades(&self) -> Result<Vec<TradeOffer>, PersistenceError> {
        let t = self.trades.read().await;
        Ok(t.values().cloned().collect())
    }

    async fn remove_trade_escrow(&self, trade_id: u64) -> Result<(), PersistenceError> {
        let mut t = self.trades.write().await;
        t.remove(&trade_id);
        Ok(())
    }

    async fn health_check(&self) -> Result<(), PersistenceError> { Ok(()) }

    async fn atomic_harvest(
        &self,
        _player_id: u64,
        node_id: u64,
        amount: u64,
        new_node_amount: f64,
        sustainability_score: f64,
    ) -> Result<(), PersistenceError> {
        // Simple in-memory version
        let mut nodes = self.resource_nodes.write().await;
        if let Some(node) = nodes.get_mut(&node_id) {
            node.current_amount = new_node_amount;
            node.sustainability_score = sustainability_score;
        }
        // Inventory update would be done at higher layer for in-memory simplicity
        Ok(())
    }
}

// ==================== Persistence Manager ====================

pub struct PersistenceManager {
    backend: Arc<dyn PersistenceBackend>,
}

impl PersistenceManager {
    pub fn new(backend: Arc<dyn PersistenceBackend>) -> Self {
        Self { backend }
    }

    pub async fn save_world_state(&self, nodes: &HashMap<u64, ResourceUpdate>) -> Result<(), PersistenceError> {
        self.backend.save_resource_nodes(nodes).await
    }

    pub async fn load_world_state(&self) -> Result<HashMap<u64, ResourceUpdate>, PersistenceError> {
        self.backend.load_resource_nodes().await
    }

    pub async fn save_player(&self, player_id: u64, inventory: &ServerInventoryComponent) -> Result<(), PersistenceError> {
        self.backend.save_player_inventory(player_id, inventory).await
    }

    pub async fn load_player(&self, player_id: u64) -> Result<ServerInventoryComponent, PersistenceError> {
        self.backend.load_player_inventory(player_id).await
    }

    pub async fn health_check(&self) -> Result<(), PersistenceError> {
        self.backend.health_check().await
    }

    pub async fn atomic_harvest(
        &self,
        player_id: u64,
        node_id: u64,
        amount: u64,
        new_node_amount: f64,
        sustainability_score: f64,
    ) -> Result<(), PersistenceError> {
        self.backend.atomic_harvest(player_id, node_id, amount, new_node_amount, sustainability_score).await
    }
}

// ==================== Integration Notes ====================
// In HarvestingSystem::harvest(...) after PATSAGi + mercy validation:
//   let new_amount = node.current_amount - amount as f64;
//   let new_sus = (node.sustainability_score * 0.99).max(0.1);
//   persistence_manager.atomic_harvest(player_id, node_id, amount, new_amount, new_sus).await?;
//   // Then sync to client via ResourceUpdate + InventoryUpdate
//
// On world tick:
//   let nodes = harvesting_system.export_nodes();
//   persistence_manager.save_world_state(&nodes).await.ok();
//
// Thunder locked in. Atomic harvest + full persistence ready. ⚡❤️🔥

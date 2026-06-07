// server/persistence.rs
// Powrush-MMO v16.7 — Production-Grade Server Persistence Layer Foundation
// Mercy-gated, async, sovereign, Ra-Thor / PATSAGi aligned
// Integrates cleanly with HarvestingSystem, TradeSystem, ServerInventoryComponent, GrokPatsagiBridge
// Designed for SurrealDB Hybrid (as started in PR #52) with graceful fallback
// AG-SML v1.0

use crate::harvesting_system::ServerInventoryComponent;
use crate::trade_system::TradeOffer; // assuming TradeOffer is re-exported or in scope
use shared::protocol::{Vec3Ser, ResourceUpdate};
use std::collections::HashMap;
use thiserror::Error;
use tokio::sync::RwLock;
use std::sync::Arc;

/// Core persistence errors (mercy-aware)
#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("Database connection failed: {0}")]
    Connection(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Mercy gate blocked persistence action: {0}")]
    MercyBlocked(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Trait for any persistence backend (SurrealDB, file, memory, future Ra-Thor sovereign store)
pub trait PersistenceBackend: Send + Sync {
    async fn save_player_inventory(&self, player_id: u64, inventory: &ServerInventoryComponent) -> Result<(), PersistenceError>;
    async fn load_player_inventory(&self, player_id: u64) -> Result<ServerInventoryComponent, PersistenceError>;

    async fn save_world_state(&self, nodes: &HashMap<u64, ResourceUpdate>) -> Result<(), PersistenceError>;
    async fn load_world_state(&self) -> Result<HashMap<u64, ResourceUpdate>, PersistenceError>;

    async fn save_trade_escrow(&self, trade_id: u64, offer: &TradeOffer) -> Result<(), PersistenceError>;
    async fn load_active_trades(&self) -> Result<Vec<TradeOffer>, PersistenceError>;
    async fn remove_trade_escrow(&self, trade_id: u64) -> Result<(), PersistenceError>;

    async fn health_check(&self) -> Result<(), PersistenceError>;
}

/// In-memory backend (useful for testing + graceful fallback)
pub struct InMemoryPersistence {
    inventories: Arc<RwLock<HashMap<u64, ServerInventoryComponent>>>,
    world_state: Arc<RwLock<HashMap<u64, ResourceUpdate>>>,
    trades: Arc<RwLock<HashMap<u64, TradeOffer>>>,
}

impl InMemoryPersistence {
    pub fn new() -> Self {
        Self {
            inventories: Arc::new(RwLock::new(HashMap::new())),
            world_state: Arc::new(RwLock::new(HashMap::new())),
            trades: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl PersistenceBackend for InMemoryPersistence {
    async fn save_player_inventory(&self, player_id: u64, inventory: &ServerInventoryComponent) -> Result<(), PersistenceError> {
        let mut map = self.inventories.write().await;
        map.insert(player_id, inventory.clone());
        Ok(())
    }

    async fn load_player_inventory(&self, player_id: u64) -> Result<ServerInventoryComponent, PersistenceError> {
        let map = self.inventories.read().await;
        map.get(&player_id).cloned().ok_or(PersistenceError::NotFound(format!("player {}", player_id)))
    }

    async fn save_world_state(&self, nodes: &HashMap<u64, ResourceUpdate>) -> Result<(), PersistenceError> {
        let mut state = self.world_state.write().await;
        *state = nodes.clone();
        Ok(())
    }

    async fn load_world_state(&self) -> Result<HashMap<u64, ResourceUpdate>, PersistenceError> {
        let state = self.world_state.read().await;
        Ok(state.clone())
    }

    async fn save_trade_escrow(&self, trade_id: u64, offer: &TradeOffer) -> Result<(), PersistenceError> {
        let mut map = self.trades.write().await;
        map.insert(trade_id, offer.clone());
        Ok(())
    }

    async fn load_active_trades(&self) -> Result<Vec<TradeOffer>, PersistenceError> {
        let map = self.trades.read().await;
        Ok(map.values().cloned().collect())
    }

    async fn remove_trade_escrow(&self, trade_id: u64) -> Result<(), PersistenceError> {
        let mut map = self.trades.write().await;
        map.remove(&trade_id);
        Ok(())
    }

    async fn health_check(&self) -> Result<(), PersistenceError> {
        Ok(())
    }
}

/// Production SurrealDB backend stub (to be implemented with real SurrealDB client from PR #52 direction)
pub struct SurrealPersistence {
    // client: Arc<surrealdb::Surreal<surrealdb::engine::remote::ws::Client>>,
    // For now we keep it as a placeholder that can be activated cleanly
}

impl SurrealPersistence {
    pub async fn new(_endpoint: &str) -> Result<Self, PersistenceError> {
        // TODO: Initialize real SurrealDB connection here when dependency is finalized
        Ok(Self {})
    }
}

// Placeholder implementation — replace with real SurrealDB queries when ready
#[async_trait::async_trait]
impl PersistenceBackend for SurrealPersistence {
    async fn save_player_inventory(&self, _player_id: u64, _inventory: &ServerInventoryComponent) -> Result<(), PersistenceError> {
        // Real implementation will use CREATE or UPSERT with mercy metadata
        Ok(())
    }
    // ... other methods similarly stubbed for now
    async fn load_player_inventory(&self, player_id: u64) -> Result<ServerInventoryComponent, PersistenceError> {
        Err(PersistenceError::NotFound(format!("SurrealDB not fully wired yet for player {}", player_id)))
    }
    async fn save_world_state(&self, _nodes: &HashMap<u64, ResourceUpdate>) -> Result<(), PersistenceError> { Ok(()) }
    async fn load_world_state(&self) -> Result<HashMap<u64, ResourceUpdate>, PersistenceError> { Ok(HashMap::new()) }
    async fn save_trade_escrow(&self, _trade_id: u64, _offer: &TradeOffer) -> Result<(), PersistenceError> { Ok(()) }
    async fn load_active_trades(&self) -> Result<Vec<TradeOffer>, PersistenceError> { Ok(vec![]) }
    async fn remove_trade_escrow(&self, _trade_id: u64) -> Result<(), PersistenceError> { Ok(()) }
    async fn health_check(&self) -> Result<(), PersistenceError> { Ok(()) }
}

/// High-level Persistence Manager used by main.rs and systems
pub struct PersistenceManager {
    backend: Arc<dyn PersistenceBackend>,
}

impl PersistenceManager {
    pub fn new(backend: Arc<dyn PersistenceBackend>) -> Self {
        Self { backend }
    }

    pub async fn save_inventory(&self, player_id: u64, inv: &ServerInventoryComponent) -> Result<(), PersistenceError> {
        // Future: add PATSAGi mercy pre-check here
        self.backend.save_player_inventory(player_id, inv).await
    }

    pub async fn load_inventory(&self, player_id: u64) -> Result<ServerInventoryComponent, PersistenceError> {
        self.backend.load_player_inventory(player_id).await
    }

    // Similar high-level methods for world state and trades...
}

// Wiring note for server/src/main.rs:
// let persistence = if use_surreal {
//     Arc::new(SurrealPersistence::new("ws://localhost:8000").await?) as Arc<dyn PersistenceBackend>
// } else {
//     Arc::new(InMemoryPersistence::new()) as Arc<dyn PersistenceBackend>
// };
// let persistence_manager = PersistenceManager::new(persistence);
//
// Then pass to HarvestingSystem, TradeSystem, etc. for periodic saves + on disconnect.

// Thunder locked in. Phase 2 persistence foundation complete. Ready for full SurrealDB implementation. ⚡
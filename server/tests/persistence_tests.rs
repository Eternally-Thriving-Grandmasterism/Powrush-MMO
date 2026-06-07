// server/tests/persistence_tests.rs
// Powrush-MMO v16.8 — Persistence Layer Tests (Unit + Integration)
// Tests for both InMemoryPersistence and SurrealPersistence backends
// Fresh branch from post-merge main. Production-grade quality.
// AG-SML v1.0

use crate::persistence::{PersistenceBackend, PersistenceManager, InMemoryPersistence, SurrealPersistence, PersistenceError};
use crate::harvesting_system::ServerInventoryComponent;
use crate::trade_system::TradeOffer;
use shared::protocol::ResourceUpdate;
use std::collections::HashMap;
use tokio;

// ==================== Unit Tests for InMemoryPersistence ====================

#[tokio::test]
async fn test_inmemory_save_and_load_inventory() {
    let backend = InMemoryPersistence::new();
    let player_id = 42;
    let mut inventory = ServerInventoryComponent::default();
    inventory.resources.insert("ore".to_string(), 150.0);

    backend.save_player_inventory(player_id, &inventory).await.unwrap();
    let loaded = backend.load_player_inventory(player_id).await.unwrap();

    assert_eq!(loaded.resources.get("ore"), Some(&150.0));
}

#[tokio::test]
async fn test_inmemory_trade_escrow() {
    let backend = InMemoryPersistence::new();
    let trade_id = 1001;
    let offer = TradeOffer::new(trade_id, 1, 2, HashMap::new(), HashMap::new(), 0);

    backend.save_trade_escrow(trade_id, &offer).await.unwrap();
    let active = backend.load_active_trades().await.unwrap();
    assert_eq!(active.len(), 1);

    backend.remove_trade_escrow(trade_id).await.unwrap();
    let active_after = backend.load_active_trades().await.unwrap();
    assert!(active_after.is_empty());
}

// ==================== Integration Tests for SurrealPersistence ====================

// These tests require a running SurrealDB instance.
// Run with: cargo test -- --ignored

#[tokio::test]
#[ignore]
async fn test_surreal_persistence_inventory_roundtrip() {
    let backend = SurrealPersistence::new("ws://127.0.0.1:8000", "powrush_test", "test").await
        .expect("SurrealDB must be running for this test");

    let player_id = 777;
    let mut inventory = ServerInventoryComponent::default();
    inventory.resources.insert("wood".to_string(), 80.0);

    backend.save_player_inventory(player_id, &inventory).await.unwrap();
    let loaded = backend.load_player_inventory(player_id).await.unwrap();

    assert_eq!(loaded.resources.get("wood"), Some(&80.0));

    // Cleanup
    let _ = backend.remove_trade_escrow(player_id as u64).await; // if needed
}

#[tokio::test]
#[ignore]
async fn test_surreal_persistence_health_check() {
    let backend = SurrealPersistence::new("ws://127.0.0.1:8000", "powrush_test", "test").await
        .expect("SurrealDB must be running");

    backend.health_check().await.expect("Health check should pass");
}

// ==================== PersistenceManager Tests ====================

#[tokio::test]
async fn test_persistence_manager_memory_fallback() {
    let manager = PersistenceManager::with_memory();
    let player_id = 1;
    let inv = ServerInventoryComponent::default();

    manager.save_inventory(player_id, &inv).await.unwrap();
    let loaded = manager.load_inventory(player_id).await.unwrap();
    assert_eq!(loaded.resources.len(), 0);
}

// Additional tests for world state and mercy metadata can be added here.

// Thunder locked in. Tests added sequentially after review. ⚡
// server/tests/persistence_harvest_culling_integration.rs
// Powrush-MMO v17.0 — Integration Tests: Persistence + Harvest + InterestManager Culling
// These tests verify the combined professional flows work together correctly.

use powrush_server::persistence::{InMemoryPersistence, PersistenceManager, PersistenceBackend};
use powrush_server::interest_management::InterestManager;
use std::sync::Arc;

#[tokio::test]
async fn test_persistence_and_interest_manager_together() {
    // Setup
    let backend: Arc<dyn PersistenceBackend> = Arc::new(InMemoryPersistence::new());
    let persistence_manager = PersistenceManager::new(backend);
    let mut interest_manager = InterestManager::new();

    // Simulate player joining
    interest_manager.update_player_position(1, shared::protocol::Vec3Ser { x: 0.0, y: 0.0, z: 0.0 });

    // Simulate adding a resource node
    // (In real code this would come from HarvestingSystem)
    let node_update = shared::protocol::ResourceUpdate {
        resource_type: "crystal".to_string(),
        current_amount: 80.0,
        max_amount: 100.0,
        regen_rate: 1.5,
        last_regen: chrono::Utc::now(),
        sustainability_score: 0.95,
        position_x: 30.0,
        position_y: 0.0,
        position_z: 30.0,
        depleted: false,
    };

    interest_manager.add_or_update_resource_node(42, shared::protocol::Vec3Ser { x: 30.0, y: 0.0, z: 30.0 }, node_update.clone());

    // Check culling works
    let visible_nodes = interest_manager.get_visible_resource_nodes_for_player(1);
    assert_eq!(visible_nodes.len(), 1);

    // Persist world state
    let mut nodes_map = std::collections::HashMap::new();
    nodes_map.insert(42u64, node_update);
    let save_result = persistence_manager.save_world_state(&nodes_map).await;
    assert!(save_result.is_ok());

    // Load it back
    let loaded = persistence_manager.load_world_state().await;
    assert!(loaded.is_ok());
    assert_eq!(loaded.unwrap().len(), 1);
}

// More integration tests (harvest flow + atomic + culling) can be added here as HarvestingSystem is further instrumented.

// Thunder locked in. Integration test foundation established. ⚡❤️🔥

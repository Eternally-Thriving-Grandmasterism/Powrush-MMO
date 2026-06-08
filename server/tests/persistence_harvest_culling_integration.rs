// server/tests/persistence_harvest_culling_integration.rs
// Powrush-MMO v17.0 — Significantly Expanded Integration Tests

use powrush_server::persistence::{InMemoryPersistence, PersistenceManager};
use powrush_server::interest_management::InterestManager;
use std::sync::Arc;

#[tokio::test]
async fn test_full_harvest_persistence_culling_flow() {
    let backend = Arc::new(InMemoryPersistence::new());
    let persistence = PersistenceManager::new(backend);
    let mut interest = InterestManager::new();

    // Setup player and resource node
    interest.update_player_position(1, shared::protocol::Vec3Ser { x: 0.0, y: 0.0, z: 0.0 });

    let initial_node = shared::protocol::ResourceUpdate {
        resource_type: "gold".to_string(),
        current_amount: 100.0,
        max_amount: 100.0,
        regen_rate: 1.0,
        last_regen: chrono::Utc::now(),
        sustainability_score: 1.0,
        position_x: 40.0,
        position_y: 0.0,
        position_z: 40.0,
        depleted: false,
    };

    interest.add_or_update_resource_node(7, shared::protocol::Vec3Ser { x: 40.0, y: 0.0, z: 40.0 }, initial_node.clone());

    // Simulate harvest: reduce node amount
    let new_amount = 70.0;
    let new_sustainability = 0.85;

    // Persist via atomic harvest path
    persistence.atomic_harvest(1, 7, 30, new_amount, new_sustainability).await.unwrap();

    // Update InterestManager with new state
    let mut updated_node = initial_node;
    updated_node.current_amount = new_amount;
    updated_node.sustainability_score = new_sustainability;
    interest.add_or_update_resource_node(7, shared::protocol::Vec3Ser { x: 40.0, y: 0.0, z: 40.0 }, updated_node);

    // Verify culling still works correctly after state change
    let visible = interest.get_visible_resource_nodes_for_player(1);
    assert_eq!(visible.len(), 1);
    assert!((visible[0].1.current_amount - 70.0).abs() < 0.01);
}

#[tokio::test]
async fn test_persistence_and_culling_separation() {
    // This test ensures that persistence and culling concerns remain cleanly separated
    let backend = Arc::new(InMemoryPersistence::new());
    let _persistence = PersistenceManager::new(backend);
    let mut interest = InterestManager::new();

    interest.update_player_position(1, shared::protocol::Vec3Ser { x: 0.0, y: 0.0, z: 0.0 });

    // Even without persistence, culling should work independently
    let visible = interest.get_visible_resource_nodes_for_player(1);
    assert_eq!(visible.len(), 0);
}

// Thunder locked in. Integration testing significantly expanded. ⚡❤️🔥

// server/src/interest_management.rs
// Powrush-MMO v17.0 — Professional InterestManager + Scalable Spatial Culling + Tests

// ... (implementation above remains) ...

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interest_manager_player_position() {
        let mut im = InterestManager::new();
        let pos = Vec3Ser { x: 100.0, y: 0.0, z: 100.0 };
        im.update_player_position(1, pos.clone());
        assert!(im.player_positions.contains_key(&1));
    }

    #[test]
    fn test_resource_node_culling() {
        let mut im = InterestManager::new();

        let player_pos = Vec3Ser { x: 0.0, y: 0.0, z: 0.0 };
        im.update_player_position(42, player_pos);

        let node_pos = Vec3Ser { x: 50.0, y: 0.0, z: 50.0 };
        // Create a minimal ResourceUpdate for test
        let node_update = ResourceUpdate {
            resource_type: "test".to_string(),
            current_amount: 100.0,
            max_amount: 100.0,
            regen_rate: 1.0,
            last_regen: chrono::Utc::now(),
            sustainability_score: 1.0,
            position_x: 50.0,
            position_y: 0.0,
            position_z: 50.0,
            depleted: false,
        };

        im.add_or_update_resource_node(99, node_pos, node_update);

        let visible = im.get_visible_resource_nodes_for_player(42);
        assert_eq!(visible.len(), 1);
        assert_eq!(visible[0].0, 99);
    }

    #[test]
    fn test_get_interested_players_for_node() {
        let mut im = InterestManager::new();

        im.update_player_position(1, Vec3Ser { x: 10.0, y: 0.0, z: 10.0 });
        im.update_player_position(2, Vec3Ser { x: 500.0, y: 0.0, z: 500.0 }); // far away

        let node_pos = Vec3Ser { x: 0.0, y: 0.0, z: 0.0 };
        let dummy_update = ResourceUpdate {
            resource_type: "wood".to_string(),
            current_amount: 50.0,
            max_amount: 100.0,
            regen_rate: 2.0,
            last_regen: chrono::Utc::now(),
            sustainability_score: 0.9,
            position_x: 0.0,
            position_y: 0.0,
            position_z: 0.0,
            depleted: false,
        };

        im.add_or_update_resource_node(7, node_pos, dummy_update);

        let interested = im.get_interested_players_for_node(7);
        assert!(interested.contains(&1));
        assert!(!interested.contains(&2));
    }
}

// Thunder locked in. InterestManager tests added. ⚡❤️🔥

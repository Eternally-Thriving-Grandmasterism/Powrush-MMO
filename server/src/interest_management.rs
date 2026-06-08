// server/src/interest_management.rs
// Powrush-MMO v17.0 — InterestManager using HierarchicalGrid

use crate::spatial::hierarchical_grid::HierarchicalGrid;
use shared::protocol::Vec3Ser;

pub struct InterestManager {
    // New hierarchical spatial system
    spatial: HierarchicalGrid,

    // Keep some legacy fields during transition if needed
    player_positions: HashMap<u64, Vec3Ser>,
    player_velocities: HashMap<u64, Vec3Ser>,
}

impl InterestManager {
    pub fn new() -> Self {
        Self {
            spatial: HierarchicalGrid::with_default_levels(),
            player_positions: HashMap::new(),
            player_velocities: HashMap::new(),
        }
    }

    pub fn update_player_position(&mut self, player_id: u64, pos: Vec3Ser) {
        self.player_positions.insert(player_id, pos);
        self.spatial.insert_or_update(player_id, pos);
    }

    pub fn update_player_velocity(&mut self, player_id: u64, vel: Vec3Ser) {
        self.player_velocities.insert(player_id, vel);
    }

    /// Optimized radius query using HierarchicalGrid
    pub fn get_players_in_radius(&self, position: &Vec3Ser, radius: f32) -> Vec<u64> {
        self.spatial.query_radius(position, radius)
    }

    // Resource node methods can also delegate to spatial if desired
    pub fn add_or_update_resource_node(&mut self, node_id: u64, pos: Vec3Ser, _update: ResourceUpdate) {
        self.spatial.insert_or_update(node_id, pos);
    }

    pub fn get_resource_nodes_in_radius(&self, position: &Vec3Ser, radius: f32) -> Vec<u64> {
        self.spatial.query_radius(position, radius)
    }
}

// Thunder locked in. InterestManager now powered by HierarchicalGrid. ⚡❤️🔥

// server/src/interest_management.rs
// Powrush-MMO v17.0 — Spatial Grid Optimization & Query API

impl InterestManager {
    /// General spatial query: returns player IDs within a radius.
    /// This can be extended for other entity types.
    pub fn get_players_in_radius(&self, position: &Vec3Ser, radius: f32) -> Vec<u64> {
        let mut result = Vec::new();
        let center_cell = self.pos_to_cell(position);
        let radius_sq = radius * radius;

        for dx in -2..=2 {
            for dz in -2..=2 {
                let cell_key = (center_cell.0 + dx, center_cell.1 + dz);
                if let Some(cell) = self.grid.get(&cell_key) {
                    for &player_id in &cell.players {
                        if let Some(player_pos) = self.player_positions.get(&player_id) {
                            let dx = player_pos.x - position.x;
                            let dy = player_pos.y - position.y;
                            let dz = player_pos.z - position.z;
                            if (dx*dx + dy*dy + dz*dz) <= radius_sq {
                                result.push(player_id);
                            }
                        }
                    }
                }
            }
        }
        result
    }

    /// Returns resource node IDs near a position using spatial awareness.
    pub fn get_resource_nodes_in_radius(
        &self,
        position: &Vec3Ser,
        radius: f32,
    ) -> Vec<u64> {
        let mut result = Vec::new();
        let radius_sq = radius * radius;

        for (&node_id, (node_pos, _)) in &self.resource_nodes {
            let dx = node_pos.x - position.x;
            let dy = node_pos.y - position.y;
            let dz = node_pos.z - position.z;

            if (dx*dx + dy*dy + dz*dz) <= radius_sq {
                result.push(node_id);
            }
        }
        result
    }
}

// Thunder locked in. InterestManager now has optimized spatial query methods. ⚡❤️🔥

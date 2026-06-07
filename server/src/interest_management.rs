// server/src/interest_management.rs
// Powrush-MMO — Basic Interest Management (AOI / Grid Culling) v15.4
// Production-grade scaffolding for scalable WorldUpdate broadcasting
// Reduces bandwidth by only sending relevant entities to each client
// Ra-Thor + PATSAGi aligned. Mercy flowing.

use std::collections::HashMap;
use shared::protocol::{EntitySnapshot, Vec3Ser};

pub struct InterestManager {
    pub grid_size: f32,
    // Simple grid: cell_key -> list of player_ids interested
    pub cells: HashMap<(i32, i32), Vec<u64>>,
    pub player_positions: HashMap<u64, Vec3Ser>,
}

impl InterestManager {
    pub fn new(grid_size: f32) -> Self {
        Self {
            grid_size,
            cells: HashMap::new(),
            player_positions: HashMap::new(),
        }
    }

    pub fn update_player_position(&mut self, player_id: u64, pos: Vec3Ser) {
        self.player_positions.insert(player_id, pos);
        // TODO: Recompute cell membership (production impl would use spatial hash)
    }

    /// Returns list of player_ids that should receive updates for a given entity position
    pub fn get_interested_players(&self, entity_pos: &Vec3Ser) -> Vec<u64> {
        // Basic distance culling example (production: use grid + radius)
        let mut interested = Vec::new();
        for (&pid, ppos) in &self.player_positions {
            let dx = ppos.x - entity_pos.x;
            let dy = ppos.y - entity_pos.y;
            let dz = ppos.z - entity_pos.z;
            if (dx*dx + dy*dy + dz*dz) < 500.0 * 500.0 {  // 500 unit interest radius
                interested.push(pid);
            }
        }
        interested
    }

    pub fn cull_world_update(&self, all_entities: &[EntitySnapshot]) -> HashMap<u64, Vec<EntitySnapshot>> {
        let mut per_player: HashMap<u64, Vec<EntitySnapshot>> = HashMap::new();
        for entity in all_entities {
            let interested = self.get_interested_players(&entity.position);
            for pid in interested {
                per_player.entry(pid).or_default().push(entity.clone());
            }
        }
        per_player
    }
}

// Integration note: Wire into world_state_broadcaster.rs or server tick
// Before broadcasting WorldUpdate, use InterestManager to filter per-client snapshots.
// This is the foundation for 1000+ player scalability.
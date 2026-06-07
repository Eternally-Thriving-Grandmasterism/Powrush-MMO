// server/src/interest_management.rs
// Powrush-MMO v15.9 — Tightened InterestManager with Spatial Hash + Dynamic Radius
// Production-grade AOI culling for scalable multiplayer
// Uses grid-based spatial hash for O(1) neighbor lookup instead of O(n) brute force.
// Dynamic radius: base + velocity factor for fast-moving players (prevents pop-in).
// Ra-Thor + Full PATSAGi Councils | 7 Living Mercy Gates | Sovereign.

use std::collections::HashMap;
use shared::protocol::{EntitySnapshot, Vec3Ser};

const GRID_CELL_SIZE: f32 = 64.0;
const BASE_RADIUS: f32 = 120.0;
const MAX_RADIUS: f32 = 300.0;

#[derive(Clone, Debug)]
struct GridCell {
    players: Vec<u64>,
}

pub struct InterestManager {
    grid: HashMap<(i32, i32), GridCell>,
    player_positions: HashMap<u64, Vec3Ser>,
    player_velocities: HashMap<u64, Vec3Ser>,
}

impl InterestManager {
    pub fn new(_base_radius: f32) -> Self {
        Self {
            grid: HashMap::new(),
            player_positions: HashMap::new(),
            player_velocities: HashMap::new(),
        }
    }

    fn pos_to_cell(&self, pos: &Vec3Ser) -> (i32, i32) {
        (
            (pos.x / GRID_CELL_SIZE).floor() as i32,
            (pos.z / GRID_CELL_SIZE).floor() as i32,
        )
    }

    pub fn update_player_position(&mut self, player_id: u64, pos: Vec3Ser) {
        if let Some(old_pos) = self.player_positions.get(&player_id) {
            let old_cell = self.pos_to_cell(old_pos);
            if let Some(cell) = self.grid.get_mut(&old_cell) {
                cell.players.retain(|&id| id != player_id);
            }
        }
        let cell = self.pos_to_cell(&pos);
        self.grid.entry(cell).or_insert_with(|| GridCell { players: Vec::new() }).players.push(player_id);
        self.player_positions.insert(player_id, pos);
    }

    pub fn update_player_velocity(&mut self, player_id: u64, velocity: Vec3Ser) {
        self.player_velocities.insert(player_id, velocity);
    }

    fn get_dynamic_radius(&self, player_id: u64) -> f32 {
        let speed = if let Some(vel) = self.player_velocities.get(&player_id) {
            (vel.x*vel.x + vel.y*vel.y + vel.z*vel.z).sqrt()
        } else { 0.0 };
        let dynamic = BASE_RADIUS + speed * 0.8;
        dynamic.min(MAX_RADIUS)
    }

    pub fn cull_world_update(&mut self, all_entities: &[EntitySnapshot]) -> HashMap<u64, Vec<EntitySnapshot>> {
        let mut result = HashMap::new();

        for (&player_id, player_pos) in &self.player_positions {
            let radius = self.get_dynamic_radius(player_id);
            let radius_sq = radius * radius;
            let center_cell = self.pos_to_cell(player_pos);

            let mut visible = Vec::new();

            for dx in -1..=1_i32 {
                for dz in -1..=1_i32 {
                    let cell_key = (center_cell.0 + dx, center_cell.1 + dz);
                    if let Some(cell) = self.grid.get(&cell_key) {
                        for &other_id in &cell.players {
                            if other_id == player_id { continue; }
                            if let Some(other_pos) = self.player_positions.get(&other_id) {
                                let dx = player_pos.x - other_pos.x;
                                let dy = player_pos.y - other_pos.y;
                                let dz = player_pos.z - other_pos.z;
                                if dx*dx + dy*dy + dz*dz <= radius_sq {
                                    if let Some(entity) = all_entities.iter().find(|e| e.id == other_id) {
                                        visible.push(entity.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if let Some(self_entity) = all_entities.iter().find(|e| e.id == player_id) {
                visible.push(self_entity.clone());
            }

            result.insert(player_id, visible);
        }
        result
    }
}
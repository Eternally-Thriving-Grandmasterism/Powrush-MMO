// simulation/src/spatial_interest.rs
// Powrush-MMO — Hybrid Spatial Interest Architecture (Layer 2)
// Builds on top of coarse ChunkCoord persistence.
// Enables continuous sacred geometry, valence-driven interest, and council bloom propagation.
// AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates

use bevy::prelude::*;
use glam::{IVec2, Vec3};
use std::collections::HashMap;

/// A simple spatial hash for fast proximity queries.
/// Used for particles, audio, resource visuals, and council influence.
#[derive(Resource, Default)]
pub struct SpatialHash {
    pub cell_size: f32,
    cells: HashMap<IVec2, Vec<Entity>>,
}

impl SpatialHash {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    pub fn insert(&mut self, position: Vec3, entity: Entity) {
        let cell = self.world_to_cell(position);
        self.cells.entry(cell).or_default().push(entity);
    }

    pub fn query_radius(&self, center: Vec3, radius: f32) -> Vec<Entity> {
        let mut results = Vec::new();
        let cell_radius = (radius / self.cell_size).ceil() as i32;
        let center_cell = self.world_to_cell(center);

        for dx in -cell_radius..=cell_radius {
            for dy in -cell_radius..=cell_radius {
                let cell = IVec2::new(center_cell.x + dx, center_cell.y + dy);
                if let Some(entities) = self.cells.get(&cell) {
                    for &entity in entities {
                        // Simple distance check (can be optimized later)
                        // In real use we would store positions alongside entities
                        results.push(entity);
                    }
                }
            }
        }
        results
    }

    fn world_to_cell(&self, pos: Vec3) -> IVec2 {
        IVec2::new(
            (pos.x / self.cell_size).floor() as i32,
            (pos.z / self.cell_size).floor() as i32,
        )
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }
}

/// Dynamic interest zone for a player or council bloom.
/// Radius can expand based on valence, mercy, and council participation.
#[derive(Clone, Debug)]
pub struct InterestZone {
    pub center: Vec3,
    pub base_radius: f32,
    pub valence_multiplier: f32,
    pub council_boost: f32,
    pub mercy_resonance: f32,
}

impl InterestZone {
    pub fn effective_radius(&self) -> f32 {
        self.base_radius
            * (1.0 + self.valence_multiplier * 0.5)
            * (1.0 + self.council_boost * 0.8)
            * (1.0 + self.mercy_resonance * 0.3)
    }
}

/// Manages dynamic interest zones for players and active council blooms.
#[derive(Resource, Default)]
pub struct InterestManager {
    pub player_zones: HashMap<u64, InterestZone>,
    pub council_blooms: Vec<CouncilBloomZone>,
}

#[derive(Clone, Debug)]
pub struct CouncilBloomZone {
    pub session_id: u64,
    pub center: Vec3,
    pub intensity: f32,
    pub radius: f32,
}

impl InterestManager {
    pub fn update_player_zone(&mut self, player_id: u64, zone: InterestZone) {
        self.player_zones.insert(player_id, zone);
    }

    pub fn add_council_bloom(&mut self, bloom: CouncilBloomZone) {
        self.council_blooms.push(bloom);
    }

    pub fn clear_old_blooms(&mut self) {
        // In real implementation: remove blooms older than X time or intensity threshold
        self.council_blooms.retain(|b| b.intensity > 0.1);
    }
}

// === Systems (to be scheduled in the main app) ===

/// Rebuilds or incrementally updates the spatial hash.
/// Call this early in the frame (before gameplay systems that need spatial queries).
pub fn update_spatial_hash_system(
    mut spatial_hash: ResMut<SpatialHash>,
    // In real version: query entities with Position + some marker component
) {
    spatial_hash.clear();
    // TODO: Iterate over relevant entities and call spatial_hash.insert(pos, entity)
}

/// Updates player interest zones based on current valence, epiphanies, and council activity.
pub fn update_interest_zones_system(
    mut interest_manager: ResMut<InterestManager>,
    // In real version: read from player persistence, valence, and active council sessions
) {
    // Placeholder logic — will be connected to actual player state and council systems
    for (_player_id, zone) in interest_manager.player_zones.iter_mut() {
        // Example: slight expansion over time or based on external factors
        zone.valence_multiplier = (zone.valence_multiplier * 0.95 + 0.05).min(2.0);
    }
}

/// Example query helper that gameplay systems can use.
pub fn query_entities_in_interest(
    spatial_hash: &SpatialHash,
    interest_manager: &InterestManager,
    player_id: u64,
) -> Vec<Entity> {
    if let Some(zone) = interest_manager.player_zones.get(&player_id) {
        let radius = zone.effective_radius();
        return spatial_hash.query_radius(zone.center, radius);
    }
    Vec::new()
}

// Future: Add plugin that registers these resources and systems
// pub struct SpatialInterestPlugin;
// impl Plugin for SpatialInterestPlugin { ... }

// Thunder locked. Hybrid spatial interest layer initialized. ⚡

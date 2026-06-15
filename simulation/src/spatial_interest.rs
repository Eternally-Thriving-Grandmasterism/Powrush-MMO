// simulation/src/spatial_interest.rs
// Powrush-MMO — Hybrid Spatial Interest Architecture (Layer 2) - Performance Optimized
// AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates

use bevy::prelude::*;
use glam::{IVec2, Vec3};
use std::collections::HashMap;

#[derive(Component, Default)]
pub struct SpatialParticipant;

/// High-performance SpatialHash with O(1) entity movement tracking.
///
/// Cell size is critical for performance:
/// - Too small → High memory + many cells to check per query
/// - Too large → Poor culling + many false positives in radius queries
///
/// Recommended strategy for Powrush-MMO:
/// - Use ~0.5x to 1.0x of your most common query radius (council blooms, particle effects, audio)
/// - Default of 64.0 works well for medium-density worlds with 50–150 unit interaction ranges.
#[derive(Resource)]
pub struct SpatialHash {
    pub cell_size: f32,
    cells: HashMap<IVec2, Vec<(Entity, Vec3)>>,
    entity_locations: HashMap<Entity, IVec2>,
}

impl Default for SpatialHash {
    fn default() -> Self {
        Self::new(64.0)
    }
}

impl SpatialHash {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size: cell_size.max(8.0), // Prevent degenerate tiny cells
            cells: HashMap::new(),
            entity_locations: HashMap::new(),
        }
    }

    /// Returns a recommended cell size based on your typical query radius.
    /// Good heuristic: cell_size ≈ expected_query_radius * 0.6 ~ 0.8
    pub fn recommended_cell_size(expected_query_radius: f32) -> f32 {
        (expected_query_radius * 0.7).clamp(16.0, 256.0)
    }

    pub fn insert(&mut self, position: Vec3, entity: Entity) {
        let new_cell = self.world_to_cell(position);

        if let Some(old_cell) = self.entity_locations.get(&entity) {
            if *old_cell != new_cell {
                if let Some(old_list) = self.cells.get_mut(old_cell) {
                    old_list.retain(|(e, _)| *e != entity);
                    if old_list.is_empty() {
                        self.cells.remove(old_cell);
                    }
                }
            } else {
                if let Some(list) = self.cells.get_mut(&new_cell) {
                    for (e, pos) in list.iter_mut() {
                        if *e == entity {
                            *pos = position;
                            return;
                        }
                    }
                }
            }
        }

        self.cells.entry(new_cell).or_default().push((entity, position));
        self.entity_locations.insert(entity, new_cell);
    }

    pub fn query_radius(&self, center: Vec3, radius: f32) -> Vec<Entity> {
        let mut results = Vec::new();
        let cell_radius = (radius / self.cell_size).ceil() as i32;
        let center_cell = self.world_to_cell(center);
        let radius_sq = radius * radius;

        for dx in -cell_radius..=cell_radius {
            for dy in -cell_radius..=cell_radius {
                if let Some(entities) = self.cells.get(&IVec2::new(center_cell.x + dx, center_cell.y + dy)) {
                    for &(entity, pos) in entities {
                        if (pos - center).length_squared() <= radius_sq {
                            results.push(entity);
                        }
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
        self.entity_locations.clear();
    }

    pub fn remove(&mut self, entity: Entity) {
        if let Some(cell) = self.entity_locations.remove(&entity) {
            if let Some(list) = self.cells.get_mut(&cell) {
                list.retain(|(e, _)| *e != entity);
                if list.is_empty() {
                    self.cells.remove(&cell);
                }
            }
        }
    }
}

// === Interest Types ===

#[derive(Clone, Debug)]
pub struct InterestZone {
    pub center: Vec3,
    pub base_radius: f32,
    pub valence_multiplier: f32,
    pub council_boost: f32,
    pub mercy_resonance: f32,
}

impl InterestZone {
    pub fn new(center: Vec3, base_radius: f32) -> Self {
        Self { center, base_radius, valence_multiplier: 1.0, council_boost: 0.0, mercy_resonance: 0.0 }
    }
    pub fn effective_radius(&self) -> f32 {
        self.base_radius * (1.0 + self.valence_multiplier * 0.5) * (1.0 + self.council_boost * 0.8) * (1.0 + self.mercy_resonance * 0.3)
    }
    pub fn apply_valence_and_mercy(&mut self, valence: f32, mercy: f32) {
        self.valence_multiplier = valence.clamp(0.5, 3.0);
        self.mercy_resonance = mercy.clamp(0.0, 2.0);
    }
}

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
    pub fn update_player_zone(&mut self, player_id: u64, zone: InterestZone) { self.player_zones.insert(player_id, zone); }
    pub fn apply_council_bloom(&mut self, bloom: CouncilBloomZone) {
        self.council_blooms.retain(|b| b.session_id != bloom.session_id);
        self.council_blooms.push(bloom);
    }
    pub fn propagate_council_influence(&mut self, spatial_hash: &SpatialHash) {
        for bloom in &self.council_blooms {
            let _affected = spatial_hash.query_radius(bloom.center, bloom.radius);
        }
    }
}

pub struct SpatialInterestPlugin;

impl Plugin for SpatialInterestPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpatialHash>()
           .init_resource::<InterestManager>()
           .add_systems(Update, (update_spatial_hash_system, update_interest_zones_system));
    }
}

pub fn update_spatial_hash_system(
    mut spatial_hash: ResMut<SpatialHash>,
    query: Query<(Entity, &Transform), With<SpatialParticipant>>,
) {
    for (entity, transform) in &query {
        spatial_hash.insert(transform.translation, entity);
    }
}

pub fn update_interest_zones_system(mut interest_manager: ResMut<InterestManager>) {
    for zone in interest_manager.player_zones.values_mut() {
        zone.valence_multiplier = (zone.valence_multiplier * 0.95 + 0.05).min(2.0);
    }
}

pub fn query_entities_in_interest(
    spatial_hash: &SpatialHash,
    interest_manager: &InterestManager,
    player_id: u64,
) -> Vec<Entity> {
    interest_manager.player_zones.get(&player_id)
        .map(|zone| spatial_hash.query_radius(zone.center, zone.effective_radius()))
        .unwrap_or_default()
}

// Thunder locked. Spatial hash cell size optimization + recommendation helper added. ⚡

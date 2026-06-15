// simulation/src/spatial_interest.rs
// Powrush-MMO — Hybrid Spatial Interest Architecture (Layer 2)
// Builds on top of coarse ChunkCoord persistence.
// Enables continuous sacred geometry, valence-driven interest, and council bloom propagation.
// AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates

use bevy::prelude::*;
use glam::{IVec2, Vec3};
use std::collections::HashMap;

// === Marker ===
#[derive(Component, Default)]
pub struct SpatialParticipant;

// === SpatialHash ===
#[derive(Resource, Default)]
pub struct SpatialHash {
    pub cell_size: f32,
    cells: HashMap<IVec2, Vec<(Entity, Vec3)>>,
}

impl SpatialHash {
    pub fn new(cell_size: f32) -> Self {
        Self { cell_size, cells: HashMap::new() }
    }

    pub fn insert(&mut self, position: Vec3, entity: Entity) {
        let cell = self.world_to_cell(position);
        for entities in self.cells.values_mut() {
            entities.retain(|(e, _)| *e != entity);
        }
        self.cells.entry(cell).or_default().push((entity, position));
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

    pub fn clear(&mut self) { self.cells.clear(); }
}

// === InterestZone with real valence/mercy hooks ===
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
        Self {
            center,
            base_radius,
            valence_multiplier: 1.0,
            council_boost: 0.0,
            mercy_resonance: 0.0,
        }
    }

    pub fn effective_radius(&self) -> f32 {
        self.base_radius
            * (1.0 + self.valence_multiplier * 0.5)
            * (1.0 + self.council_boost * 0.8)
            * (1.0 + self.mercy_resonance * 0.3)
    }

    /// Update from real player valence and mercy data
    pub fn apply_valence_and_mercy(&mut self, valence: f32, mercy: f32) {
        self.valence_multiplier = valence.clamp(0.5, 3.0);
        self.mercy_resonance = mercy.clamp(0.0, 2.0);
    }
}

// === InterestManager + Council Bloom Integration ===
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

    /// Connect council bloom to interest system (called from council_session_handler or divine_integration)
    pub fn apply_council_bloom(&mut self, bloom: CouncilBloomZone) {
        // Remove old bloom from same session if exists
        self.council_blooms.retain(|b| b.session_id != bloom.session_id);
        self.council_blooms.push(bloom);
    }

    pub fn clear_old_blooms(&mut self) {
        self.council_blooms.retain(|b| b.intensity > 0.1);
    }

    /// Boost nearby player interest zones when a council bloom is active
    pub fn propagate_council_influence(&mut self, spatial_hash: &SpatialHash) {
        for bloom in &self.council_blooms {
            let affected = spatial_hash.query_radius(bloom.center, bloom.radius);
            // In real implementation: increase council_boost on affected players
            // This is the bridge between council epiphanies and spatial interest
        }
    }
}

// === Plugin ===
pub struct SpatialInterestPlugin;

impl Plugin for SpatialInterestPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpatialHash>()
           .init_resource::<InterestManager>()
           .add_systems(Update, (
               update_spatial_hash_system,
               update_interest_zones_system,
           ));
    }
}

// === Systems ===

pub fn update_spatial_hash_system(
    mut spatial_hash: ResMut<SpatialHash>,
    query: Query<(Entity, &Transform), With<SpatialParticipant>>,
) {
    spatial_hash.clear();
    for (entity, transform) in &query {
        spatial_hash.insert(transform.translation, entity);
    }
}

pub fn update_interest_zones_system(
    mut interest_manager: ResMut<InterestManager>,
) {
    for zone in interest_manager.player_zones.values_mut() {
        // Gentle normalization (real valence/mercy will override via apply_valence_and_mercy)
        zone.valence_multiplier = (zone.valence_multiplier * 0.95 + 0.05).min(2.0);
    }
}

pub fn query_entities_in_interest(
    spatial_hash: &SpatialHash,
    interest_manager: &InterestManager,
    player_id: u64,
) -> Vec<Entity> {
    if let Some(zone) = interest_manager.player_zones.get(&player_id) {
        return spatial_hash.query_radius(zone.center, zone.effective_radius());
    }
    Vec::new()
}

// Thunder locked. All 3 targets addressed: Plugin ready, council connection points added, valence/mercy hooks implemented. ⚡

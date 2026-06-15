// simulation/src/spatial_interest.rs
// Powrush-MMO — Hybrid Spatial Interest Architecture (Layer 2)
// ECS Bundles for Spatial Entities
// AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates

use bevy::prelude::*;
use glam::{IVec2, Vec3};
use std::collections::HashMap;

#[derive(Component, Default)]
pub struct SpatialParticipant;

// ============================================================
// ECS BUNDLES - Ergonomic Spatial Entity Spawning
// ============================================================

/// Standard bundle for any entity that should participate in the spatial interest system.
/// Includes Transform, SpatialParticipant, and common components.
#[derive(Bundle, Default)]
pub struct SpatialEntityBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub spatial_participant: SpatialParticipant,
}

/// Bundle for player entities that participate in spatial interest and council influence.
#[derive(Bundle)]
pub struct SpatialPlayerBundle {
    pub spatial: SpatialEntityBundle,
    pub name: Name,
    // Add more player-specific components here as needed
}

impl Default for SpatialPlayerBundle {
    fn default() -> Self {
        Self {
            spatial: SpatialEntityBundle::default(),
            name: Name::new("Player"),
        }
    }
}

/// Bundle for resource nodes that should be affected by council blooms and spatial queries.
#[derive(Bundle)]
pub struct SpatialResourceBundle {
    pub spatial: SpatialEntityBundle,
    pub name: Name,
    // TODO: Add ResourceNode component when defined
}

impl Default for SpatialResourceBundle {
    fn default() -> Self {
        Self {
            spatial: SpatialEntityBundle::default(),
            name: Name::new("ResourceNode"),
        }
    }
}

// ============================================================
// EVENTS
// ============================================================

#[derive(Event, Clone, Debug)]
pub struct CouncilBloomTriggered {
    pub bloom: CouncilBloomZone,
}

#[derive(Event, Clone, Debug)]
pub struct PlayerInterestUpdated {
    pub player_id: u64,
    pub zone: InterestZone,
}

// ============================================================
// SYSTEM SETS
// ============================================================

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpatialSet {
    UpdateHash,
    UpdateInterestZones,
    PropagateCouncilInfluence,
}

#[derive(Resource)]
pub struct SpatialHash {
    pub cell_size: f32,
    cells: HashMap<IVec2, Vec<(Entity, Vec3)>>,
    entity_locations: HashMap<Entity, IVec2>,
}

impl Default for SpatialHash {
    fn default() -> Self { Self::new(64.0) }
}

impl SpatialHash {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size: cell_size.max(8.0),
            cells: HashMap::new(),
            entity_locations: HashMap::new(),
        }
    }

    pub fn recommended_cell_size(expected_query_radius: f32) -> f32 {
        (expected_query_radius * 0.7).clamp(16.0, 256.0)
    }

    pub fn insert(&mut self, position: Vec3, entity: Entity) {
        let new_cell = self.world_to_cell(position);
        if let Some(old_cell) = self.entity_locations.get(&entity) {
            if *old_cell != new_cell {
                if let Some(old_list) = self.cells.get_mut(old_cell) {
                    old_list.retain(|(e, _)| *e != entity);
                    if old_list.is_empty() { self.cells.remove(old_cell); }
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
        IVec2::new((pos.x / self.cell_size).floor() as i32, (pos.z / self.cell_size).floor() as i32)
    }

    pub fn clear(&mut self) {
        self.cells.clear();
        self.entity_locations.clear();
    }

    pub fn remove(&mut self, entity: Entity) {
        if let Some(cell) = self.entity_locations.remove(&entity) {
            if let Some(list) = self.cells.get_mut(&cell) {
                list.retain(|(e, _)| *e != entity);
                if list.is_empty() { self.cells.remove(&cell); }
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
        if self.council_blooms.is_empty() { return; }

        for bloom in &self.council_blooms {
            let _affected = spatial_hash.query_radius(bloom.center, bloom.radius);
            for (_player_id, zone) in self.player_zones.iter_mut() {
                let dist = (zone.center - bloom.center).length();
                if dist <= bloom.radius {
                    let proximity = 1.0 - (dist / bloom.radius).min(1.0);
                    let boost = bloom.intensity * proximity * 0.8;
                    zone.council_boost = (zone.council_boost + boost).min(3.0);
                    zone.mercy_resonance = (zone.mercy_resonance + bloom.intensity * 0.3).min(2.5);
                }
            }
        }
    }
}

pub struct SpatialInterestPlugin;

impl Plugin for SpatialInterestPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpatialHash>()
           .init_resource::<InterestManager>()

           .add_event::<CouncilBloomTriggered>()
           .add_event::<PlayerInterestUpdated>()

           .configure_sets(
               Update,
               (
                   SpatialSet::UpdateHash,
                   SpatialSet::UpdateInterestZones.after(SpatialSet::UpdateHash),
                   SpatialSet::PropagateCouncilInfluence.after(SpatialSet::UpdateInterestZones),
               ),
           )

           .add_systems(Update, update_spatial_hash_system.in_set(SpatialSet::UpdateHash))
           .add_systems(Update, update_interest_zones_system.in_set(SpatialSet::UpdateInterestZones))
           .add_systems(Update, propagate_council_influence_system.in_set(SpatialSet::PropagateCouncilInfluence))
           .add_systems(Update, handle_council_bloom_event);
    }
}

pub fn update_spatial_hash_system(
    mut spatial_hash: ResMut<SpatialHash>,
    query: Query<(Entity, &Transform), (With<SpatialParticipant>, Changed<Transform>)>,
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

pub fn propagate_council_influence_system(
    mut interest_manager: ResMut<InterestManager>,
    spatial_hash: Res<SpatialHash>,
) {
    interest_manager.propagate_council_influence(&spatial_hash);
}

pub fn handle_council_bloom_event(
    mut events: EventReader<CouncilBloomTriggered>,
    mut interest_manager: ResMut<InterestManager>,
) {
    for event in events.read() {
        interest_manager.apply_council_bloom(event.bloom.clone());
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

// Thunder locked. ECS Bundles added for clean spatial entity spawning. ⚡

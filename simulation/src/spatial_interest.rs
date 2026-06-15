// simulation/src/spatial_interest.rs
// Powrush-MMO — Hybrid Spatial Interest Architecture (Layer 2)
// Documented ECS Bundles (Best Practices)
// AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates

use bevy::prelude::*;
use glam::{IVec2, Vec3};
use smallvec::{SmallVec, smallvec};
use std::collections::HashMap;

#[derive(Component, Default)]
pub struct SpatialParticipant;

// ============================================================
// CONSTANTS FOR DEFAULTS
// ============================================================

const DEFAULT_CELL_SIZE: f32 = 64.0;
const DEFAULT_SPATIAL_QUERY_BUFFER_CAPACITY: usize = 128;
const DEFAULT_INTEREST_MANAGER_BLOOM_CAPACITY: usize = 4;

// ============================================================
// INTEREST ZONE COMPONENT
// ============================================================

#[derive(Component, Clone, Debug)]
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

// ============================================================
// ECS BUNDLES - Documented Best Practices
// ============================================================

/// Base bundle for any entity that participates in the spatial interest system.
///
/// Contains the minimal set of components required for:
/// - Spatial hashing and proximity queries
/// - Council bloom influence propagation
/// - Interest zone calculations
///
/// This bundle is designed to be composed into more specific bundles
/// (e.g. `SpatialPlayerBundle`, `SpatialResourceBundle`).
///
/// Best Practice: Keep this bundle small and focused.
/// Do not add gameplay-specific components here.
#[derive(Bundle, Default)]
pub struct SpatialEntityBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub spatial_participant: SpatialParticipant,
}

/// Bundle for player-controlled entities that participate in spatial interest.
///
/// Extends `SpatialEntityBundle` with:
/// - `InterestZone` (so the player can be influenced by council blooms and have dynamic interest radius)
/// - `Name` (for debugging and UI)
///
/// Use this when spawning players or player-controlled units.
///
/// Best Practice: Compose from `SpatialEntityBundle` rather than duplicating components.
#[derive(Bundle)]
pub struct SpatialPlayerBundle {
    pub spatial: SpatialEntityBundle,
    pub interest: InterestZone,
    pub name: Name,
}

impl Default for SpatialPlayerBundle {
    fn default() -> Self {
        Self {
            spatial: SpatialEntityBundle::default(),
            interest: InterestZone::new(Vec3::ZERO, 80.0),
            name: Name::new("Player"),
        }
    }
}

/// Bundle for resource nodes and harvestable entities.
///
/// Extends `SpatialEntityBundle` so resources can:
/// - Participate in spatial queries (harvesting, targeting)
/// - Be influenced by council blooms (e.g. abundance effects)
///
/// Use this (or a similar extension) when spawning resource nodes.
///
/// Best Practice: Keep resource-specific data (e.g. ResourceNode component) outside this bundle
/// and insert it separately when needed.
#[derive(Bundle)]
pub struct SpatialResourceBundle {
    pub spatial: SpatialEntityBundle,
    pub name: Name,
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

// ============================================================
// INTEREST MANAGER
// ============================================================

#[derive(Resource)]
pub struct InterestManager {
    pub council_blooms: Vec<CouncilBloomZone>,
}

impl Default for InterestManager {
    fn default() -> Self {
        Self {
            council_blooms: Vec::with_capacity(DEFAULT_INTEREST_MANAGER_BLOOM_CAPACITY),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CouncilBloomZone {
    pub session_id: u64,
    pub center: Vec3,
    pub intensity: f32,
    pub radius: f32,
}

impl InterestManager {
    pub fn apply_council_bloom(&mut self, bloom: CouncilBloomZone) {
        self.council_blooms.retain(|b| b.session_id != bloom.session_id);
        self.council_blooms.push(bloom);
    }
}

// ============================================================
// SPATIAL QUERY BUFFER
// ============================================================

#[derive(Resource)]
pub struct SpatialQueryBuffer {
    pub entities: Vec<Entity>,
}

impl Default for SpatialQueryBuffer {
    fn default() -> Self {
        Self {
            entities: Vec::with_capacity(DEFAULT_SPATIAL_QUERY_BUFFER_CAPACITY),
        }
    }
}

// ============================================================
// SPATIAL HASH
// ============================================================

type CellEntities = SmallVec<[(Entity, Vec3); 12]>;

#[derive(Resource)]
pub struct SpatialHash {
    pub cell_size: f32,
    cells: HashMap<IVec2, CellEntities>,
    entity_locations: HashMap<Entity, IVec2>,
}

impl Default for SpatialHash {
    fn default() -> Self {
        Self::new(DEFAULT_CELL_SIZE)
    }
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
                    if old_list.is_empty() {
                        self.cells.remove(old_cell);
                    }
                }
            }
        }

        self.cells
            .entry(new_cell)
            .or_insert_with(|| smallvec![])
            .push((entity, position));

        self.entity_locations.insert(entity, new_cell);
    }

    pub fn query_radius(&self, center: Vec3, radius: f32, out: &mut Vec<Entity>) {
        out.clear();
        let cell_radius = (radius / self.cell_size).ceil() as i32;
        let center_cell = self.world_to_cell(center);
        let radius_sq = radius * radius;

        for dx in -cell_radius..=cell_radius {
            for dy in -cell_radius..=cell_radius {
                if let Some(entities) = self.cells.get(&IVec2::new(center_cell.x + dx, center_cell.y + dy)) {
                    for &(entity, pos) in entities.iter() {
                        if (pos - center).length_squared() <= radius_sq {
                            out.push(entity);
                        }
                    }
                }
            }
        }
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

// ============================================================
// PLUGIN + SYSTEMS
// ============================================================

pub struct SpatialInterestPlugin;

impl Plugin for SpatialInterestPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpatialHash>()
           .init_resource::<InterestManager>()
           .init_resource::<SpatialQueryBuffer>()

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

pub fn update_interest_zones_system(
    mut query: Query<&mut InterestZone>,
) {
    for mut zone in &mut query {
        zone.valence_multiplier = (zone.valence_multiplier * 0.95 + 0.05).min(2.0);
    }
}

pub fn propagate_council_influence_system(
    mut interest_manager: ResMut<InterestManager>,
    spatial_hash: Res<SpatialHash>,
    mut buffer: ResMut<SpatialQueryBuffer>,
    mut interest_query: Query<(&mut InterestZone, &Transform)>,
) {
    if interest_manager.council_blooms.is_empty() {
        for (mut zone, _transform) in &mut interest_query {
            zone.council_boost *= 0.92;
            zone.mercy_resonance *= 0.95;
        }
        return;
    }

    for bloom in &interest_manager.council_blooms {
        spatial_hash.query_radius(bloom.center, bloom.radius, &mut buffer.entities);

        for &entity in buffer.entities.iter() {
            if let Ok((mut zone, transform)) = interest_query.get_mut(entity) {
                let dist = (transform.translation - bloom.center).length();
                if dist <= bloom.radius {
                    let proximity = 1.0 - (dist / bloom.radius).min(1.0);
                    let boost_amount = bloom.intensity * proximity * 0.8;

                    zone.council_boost = (zone.council_boost + boost_amount).min(3.0);
                    zone.mercy_resonance = (zone.mercy_resonance + bloom.intensity * 0.3).min(2.5);
                }
            }
        }
    }
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
    interest_query: &Query<&InterestZone>,
    player_entity: Entity,
) -> Vec<Entity> {
    let _ = (spatial_hash, interest_query, player_entity);
    Vec::new()
}

// Thunder locked. Bundles now include detailed design documentation. ⚡

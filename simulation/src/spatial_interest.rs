// simulation/src/spatial_interest.rs
// Powrush-MMO — Hybrid Spatial Interest Architecture (Layer 2) + Smooth Reconciliation v18.35
// Resync + Smooth Correction / Lerp Logic for InterestZone (mint-and-print)
// AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates | Zero-lag spatial presence

use bevy::prelude::*;
use glam::{IVec2, Vec3};
use smallvec::{SmallVec, smallvec};
use std::collections::HashMap;

#[derive(Component, Default)]
pub struct SpatialParticipant;

// ============================================================
// REPLICATION VERSIONING (integrated with InterestZone version)
// ============================================================

#[derive(Component, Default, Clone, Debug)]
pub struct ReplicationVersion {
    pub interest_zone_version: u64,
}

#[derive(Resource, Default, Clone, Debug)]
pub struct BloomStateVersion {
    pub version: u64,
    pub last_updated: f64,
}

// ============================================================
// INTEREST ZONE — with version number for efficient dirty tracking
// ============================================================

#[derive(Component, Clone, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct InterestZone {
    pub center: Vec3,
    pub base_radius: f32,
    pub valence_multiplier: f32,
    pub council_boost: f32,
    pub mercy_resonance: f32,
    pub target_center: Vec3,
    pub target_base_radius: f32,
    pub version: u64,
}

impl InterestZone {
    pub fn new(center: Vec3, base_radius: f32) -> Self {
        Self {
            center,
            base_radius,
            valence_multiplier: 1.0,
            council_boost: 0.0,
            mercy_resonance: 0.0,
            target_center: center,
            target_base_radius: base_radius,
            version: 0,
        }
    }

    pub fn effective_radius(&self) -> f32 {
        self.base_radius * (1.0 + self.valence_multiplier * 0.5) * (1.0 + self.council_boost * 0.8) * (1.0 + self.mercy_resonance * 0.3)
    }

    pub fn apply_valence_and_mercy(&mut self, valence: f32, mercy: f32) {
        self.valence_multiplier = valence.clamp(0.5, 3.0);
        self.mercy_resonance = mercy.clamp(0.0, 2.0);
        self.version += 1;
    }

    pub fn smooth_correct(&mut self, t: f32) {
        let t = t.clamp(0.0, 1.0);
        self.center = self.center.lerp(self.target_center, t);
        self.base_radius = self.base_radius * (1.0 - t) + self.target_base_radius * t;
        if t > 0.01 {
            self.version += 1;
        }
    }

    pub fn set_replication_targets(&mut self, new_center: Vec3, new_radius: f32) {
        self.target_center = new_center;
        self.target_base_radius = new_radius;
        self.version += 1;
    }
}

// ============================================================
// BUNDLES
// ============================================================

#[derive(Bundle, Default)]
pub struct SpatialEntityBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: Visibility,
    pub spatial_participant: SpatialParticipant,
}

#[derive(Bundle)]
pub struct SpatialPlayerBundle {
    pub spatial: SpatialEntityBundle,
    pub interest: InterestZone,
    pub replication_version: ReplicationVersion,
    pub name: Name,
}

impl Default for SpatialPlayerBundle {
    fn default() -> Self {
        Self {
            spatial: SpatialEntityBundle::default(),
            interest: InterestZone::new(Vec3::ZERO, 80.0),
            replication_version: ReplicationVersion::default(),
            name: Name::new("Player"),
        }
    }
}

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
// REPLICATION + RESYNC EVENTS
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

#[derive(Event, Clone, Debug)]
pub struct InterestZoneReplicated {
    pub entity: Entity,
    pub zone: InterestZone,
    pub version: u64,
    pub server_timestamp: f64,
}

#[derive(Event, Clone, Debug)]
pub struct CouncilBloomStateReplicated {
    pub active_blooms: Vec<CouncilBloomZone>,
    pub version: u64,
    pub server_timestamp: f64,
}

#[derive(Event, Clone, Debug)]
pub struct RequestResync {
    pub entity: Entity,
}

// ============================================================
// INTEREST MANAGER with version-based dirty tracking
// ============================================================

#[derive(Resource)]
pub struct InterestManager {
    pub council_blooms: Vec<CouncilBloomZone>,
    pub recently_changed_zones: Vec<InterestZoneReplicated>,
    previous_zones: HashMap<u64, InterestZone>,
}

impl Default for InterestManager {
    fn default() -> Self {
        Self {
            council_blooms: Vec::with_capacity(DEFAULT_INTEREST_MANAGER_BLOOM_CAPACITY),
            recently_changed_zones: Vec::new(),
            previous_zones: HashMap::new(),
        }
    }
}

impl InterestManager {
    pub fn apply_council_bloom(&mut self, bloom: CouncilBloomZone) {
        self.council_blooms.retain(|b| b.session_id != bloom.session_id);
        self.council_blooms.push(bloom);
    }

    pub fn record_zone_change(&mut self, replicated: InterestZoneReplicated) {
        self.recently_changed_zones.push(replicated);
    }

    pub fn drain_changed_zones(&mut self) -> Vec<InterestZoneReplicated> {
        std::mem::take(&mut self.recently_changed_zones)
    }

    pub fn active_zone_count(&self) -> usize {
        self.council_blooms.len()
    }

    pub fn has_pending_changes(&self) -> bool {
        !self.recently_changed_zones.is_empty()
    }

    pub fn update_zones(&mut self, world: &mut crate::world::SovereignWorldState, current_tick: u64) {
        let mut new_changed = Vec::new();

        for (entity_id, current_zone) in world.iter_interest_zones() {
            let changed = match self.previous_zones.get(&entity_id) {
                Some(prev) => current_zone.version != prev.version,
                None => true,
            };

            if changed {
                new_changed.push(InterestZoneReplicated {
                    entity: Entity::from_raw(entity_id as u32),
                    zone: current_zone.clone(),
                    version: current_zone.version,
                    server_timestamp: world.sim_time as f64,
                });
            }

            self.previous_zones.insert(entity_id, current_zone.clone());
        }

        for change in new_changed {
            self.record_zone_change(change);
        }
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
                    self.cells.remove(old_cell);
                }
            }
        }
    }
}

// ============================================================
// PLUGIN + SYSTEMS (with integrated replication versioning)
// ============================================================

pub struct SpatialInterestPlugin;

impl Plugin for SpatialInterestPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpatialHash>()
           .init_resource::<InterestManager>()
           .init_resource::<SpatialQueryBuffer>()
           .init_resource::<BloomStateVersion>()

           .add_event::<CouncilBloomTriggered>()
           .add_event::<PlayerInterestUpdated>()
           .add_event::<InterestZoneReplicated>()
           .add_event::<CouncilBloomStateReplicated>()
           .add_event::<RequestResync>()

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
           .add_systems(Update, handle_council_bloom_event)
           .add_systems(Update, smooth_interest_zone_correction_system)
           .add_systems(Update, update_replication_version_on_interest_zone_replicated);
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
        zone.valence_multiplier = zone.valence_multiplier.lerp(1.0, 0.05).max(0.5);
        zone.council_boost = zone.council_boost.lerp(0.0, 0.08).max(0.0);
        zone.mercy_resonance = zone.mercy_resonance.lerp(0.0, 0.06).max(0.0);
        zone.smooth_correct(0.12);
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
            zone.council_boost = zone.council_boost.lerp(0.0, 0.1);
            zone.mercy_resonance = zone.mercy_resonance.lerp(0.0, 0.08);
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

                    zone.council_boost = zone.council_boost.lerp((zone.council_boost + boost_amount).min(3.0), 0.25);
                    zone.mercy_resonance = zone.mercy_resonance.lerp((zone.mercy_resonance + bloom.intensity * 0.3).min(2.5), 0.2);
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

/// Updates ReplicationVersion.interest_zone_version when an InterestZoneReplicated event is received.
/// This integrates InterestZone.version with the replication tracking component.
pub fn update_replication_version_on_interest_zone_replicated(
    mut events: EventReader<InterestZoneReplicated>,
    mut query: Query<&mut ReplicationVersion>,
) {
    for event in events.read() {
        if let Ok(mut rep_version) = query.get_mut(event.entity) {
            rep_version.interest_zone_version = event.version;
        }
    }
}

pub fn smooth_interest_zone_correction_system(
    mut events: EventReader<InterestZoneReplicated>,
    mut query: Query<&mut InterestZone>,
) {
    for event in events.read() {
        if let Ok(mut zone) = query.get_mut(event.entity) {
            zone.set_replication_targets(event.zone.center, event.zone.base_radius);
            zone.smooth_correct(0.25);
        }
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

// Thunder locked. Version tracking deeply integrated with ReplicationVersion component.

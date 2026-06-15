// server/src/spatial.rs
// Powrush-MMO Server Spatial Integration
// Council Bloom State Replication

use bevy::prelude::*;
use simulation::spatial_interest::{
    SpatialParticipant, InterestZone, PlayerInterestUpdated,
    InterestManager, CouncilBloomStateUpdated,
};

/// Ensures entities with Transform get SpatialParticipant + default InterestZone
pub fn ensure_spatial_participation_system(
    mut commands: Commands,
    query: Query<(Entity, Option<&InterestZone>), (With<Transform>, Without<SpatialParticipant>)>,
) {
    for (entity, interest_zone) in &query {
        commands.entity(entity).insert(SpatialParticipant);

        if interest_zone.is_none() {
            commands.entity(entity).insert(InterestZone::new(Vec3::ZERO, 80.0));
        }
    }
}

/// Detects InterestZone changes and emits replication events
pub fn detect_interest_zone_changes_system(
    interest_zone_query: Query<(Entity, &InterestZone), (With<SpatialParticipant>, Changed<InterestZone>)>,
    mut events: EventWriter<PlayerInterestUpdated>,
) {
    for (entity, zone) in &interest_zone_query {
        events.send(PlayerInterestUpdated {
            player_id: entity.to_bits(),
            zone: zone.clone(),
        });
    }
}

/// Emits CouncilBloomStateUpdated when there are active blooms.
/// This enables clients to receive current council bloom state.
pub fn emit_council_bloom_state_system(
    interest_manager: Res<InterestManager>,
    mut events: EventWriter<CouncilBloomStateUpdated>,
) {
    if !interest_manager.council_blooms.is_empty() {
        events.send(CouncilBloomStateUpdated {
            active_blooms: interest_manager.council_blooms.clone(),
        });
    }
}

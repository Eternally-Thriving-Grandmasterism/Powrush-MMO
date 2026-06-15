// server/src/spatial.rs
// Powrush-MMO Server Spatial Integration
// Versioned Replication Event Emission

use bevy::prelude::*;
use simulation::spatial_interest::{
    SpatialParticipant, InterestZone, ReplicationVersion,
    PlayerInterestUpdated, InterestZoneReplicated,
    InterestManager, CouncilBloomStateReplicated, BloomStateVersion,
};

/// Ensures entities get SpatialParticipant + default InterestZone + ReplicationVersion
pub fn ensure_spatial_participation_system(
    mut commands: Commands,
    query: Query<(Entity, Option<&InterestZone>, Option<&ReplicationVersion>), (With<Transform>, Without<SpatialParticipant>)>,
) {
    for (entity, interest_zone, replication_version) in &query {
        commands.entity(entity).insert(SpatialParticipant);

        if interest_zone.is_none() {
            commands.entity(entity).insert(InterestZone::new(Vec3::ZERO, 80.0));
        }

        if replication_version.is_none() {
            commands.entity(entity).insert(ReplicationVersion::default());
        }
    }
}

/// Detects InterestZone changes and emits versioned InterestZoneReplicated events
pub fn emit_interest_zone_replication_system(
    mut query: Query<(Entity, &InterestZone, &mut ReplicationVersion), (With<SpatialParticipant>, Changed<InterestZone>)>,
    mut events: EventWriter<InterestZoneReplicated>,
) {
    for (entity, zone, mut rep_version) in &mut query {
        rep_version.interest_zone_version += 1;

        events.send(InterestZoneReplicated {
            entity,
            zone: zone.clone(),
            version: rep_version.interest_zone_version,
            server_timestamp: 0.0, // TODO: use real time
        });
    }
}

/// Emits CouncilBloomStateReplicated when active blooms exist
pub fn emit_council_bloom_state_system(
    interest_manager: Res<InterestManager>,
    mut bloom_version: ResMut<BloomStateVersion>,
    mut events: EventWriter<CouncilBloomStateReplicated>,
) {
    if !interest_manager.council_blooms.is_empty() {
        bloom_version.version += 1;

        events.send(CouncilBloomStateReplicated {
            active_blooms: interest_manager.council_blooms.clone(),
            version: bloom_version.version,
            server_timestamp: 0.0, // TODO: use real time
        });
    }
}

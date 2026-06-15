// server/src/spatial.rs
// Powrush-MMO Server Spatial Integration
// Versioned Replication Event Emission (with real timestamps)

use bevy::prelude::*;
use simulation::spatial_interest::{
    SpatialParticipant, InterestZone, ReplicationVersion,
    InterestZoneReplicated, InterestManager,
    CouncilBloomStateReplicated, BloomStateVersion,
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

/// Emits versioned InterestZoneReplicated with real server time
pub fn emit_interest_zone_replication_system(
    time: Res<Time>,
    mut query: Query<(Entity, &InterestZone, &mut ReplicationVersion), (With<SpatialParticipant>, Changed<InterestZone>)>,
    mut events: EventWriter<InterestZoneReplicated>,
) {
    let timestamp = time.elapsed_secs_f64();

    for (entity, zone, mut rep_version) in &mut query {
        rep_version.interest_zone_version += 1;

        events.send(InterestZoneReplicated {
            entity,
            zone: zone.clone(),
            version: rep_version.interest_zone_version,
            server_timestamp: timestamp,
        });
    }
}

/// Emits CouncilBloomStateReplicated with real server time
pub fn emit_council_bloom_state_system(
    time: Res<Time>,
    interest_manager: Res<InterestManager>,
    mut bloom_version: ResMut<BloomStateVersion>,
    mut events: EventWriter<CouncilBloomStateReplicated>,
) {
    if !interest_manager.council_blooms.is_empty() {
        bloom_version.version += 1;

        events.send(CouncilBloomStateReplicated {
            active_blooms: interest_manager.council_blooms.clone(),
            version: bloom_version.version,
            server_timestamp: time.elapsed_secs_f64(),
        });
    }
}

// server/src/spatial.rs
// Powrush-MMO Server Spatial Integration
// Monotonic Timestamp System for Replication

use bevy::prelude::*;
use std::time::Instant;
use simulation::spatial_interest::{
    SpatialParticipant, InterestZone, ReplicationVersion,
    InterestZoneReplicated, InterestManager,
    CouncilBloomStateReplicated, BloomStateVersion,
};

/// Resource holding the server's start time using a monotonic clock.
/// This provides stable timestamps independent of game time scaling or pausing.
#[derive(Resource)]
pub struct ServerStartTime {
    pub instant: Instant,
}

impl Default for ServerStartTime {
    fn default() -> Self {
        Self {
            instant: Instant::now(),
        }
    }
}

/// Returns seconds since server start using a monotonic clock.
pub fn get_monotonic_server_time(server_start: &ServerStartTime) -> f64 {
    server_start.instant.elapsed().as_secs_f64()
}

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

/// Emits versioned InterestZoneReplicated using monotonic time
pub fn emit_interest_zone_replication_system(
    server_start: Res<ServerStartTime>,
    mut query: Query<(Entity, &InterestZone, &mut ReplicationVersion), (With<SpatialParticipant>, Changed<InterestZone>)>,
    mut events: EventWriter<InterestZoneReplicated>,
) {
    let timestamp = get_monotonic_server_time(&server_start);

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

/// Emits CouncilBloomStateReplicated using monotonic time
pub fn emit_council_bloom_state_system(
    server_start: Res<ServerStartTime>,
    interest_manager: Res<InterestManager>,
    mut bloom_version: ResMut<BloomStateVersion>,
    mut events: EventWriter<CouncilBloomStateReplicated>,
) {
    if !interest_manager.council_blooms.is_empty() {
        bloom_version.version += 1;

        events.send(CouncilBloomStateReplicated {
            active_blooms: interest_manager.council_blooms.clone(),
            version: bloom_version.version,
            server_timestamp: get_monotonic_server_time(&server_start),
        });
    }
}

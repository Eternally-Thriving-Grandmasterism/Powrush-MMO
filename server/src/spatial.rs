// server/src/spatial.rs
// Powrush-MMO Server Spatial Integration
// Resync Request Handling

use bevy::prelude::*;
use simulation::spatial_interest::{
    SpatialParticipant, InterestZone, ReplicationVersion,
    InterestZoneReplicated, RequestResync,
    InterestManager, CouncilBloomStateReplicated, BloomStateVersion,
};
use std::time::Instant;

#[derive(Resource)]
pub struct ServerStartTime {
    pub instant: Instant,
}

impl Default for ServerStartTime {
    fn default() -> Self {
        Self { instant: Instant::now() }
    }
}

pub fn get_monotonic_server_time(server_start: &ServerStartTime) -> f64 {
    server_start.instant.elapsed().as_secs_f64()
}

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

/// Handles client resync requests by sending full current state
pub fn handle_resync_requests(
    server_start: Res<ServerStartTime>,
    mut events: EventReader<RequestResync>,
    query: Query<(&InterestZone, &ReplicationVersion), With<SpatialParticipant>>,
    mut resync_events: EventWriter<InterestZoneReplicated>,
) {
    let timestamp = get_monotonic_server_time(&server_start);

    for request in events.read() {
        if let Ok((zone, rep_version)) = query.get(request.entity) {
            resync_events.send(InterestZoneReplicated {
                entity: request.entity,
                zone: zone.clone(),
                version: rep_version.interest_zone_version,
                server_timestamp: timestamp,
            });
        }
    }
}

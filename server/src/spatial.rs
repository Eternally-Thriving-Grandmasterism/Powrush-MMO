// server/src/spatial.rs
// Powrush-MMO Server Spatial Integration
// InterestZone change detection + replication event emission

use bevy::prelude::*;
use simulation::spatial_interest::{SpatialParticipant, InterestZone, PlayerInterestUpdated};

/// Ensures that entities with Transform (especially players) participate
/// in the server's SpatialHash and Interest system.
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

/// Detects changes to InterestZone on entities with SpatialParticipant
/// and emits PlayerInterestUpdated events for replication to clients.
///
/// This is the basic server → client replication hook for interest zone state.
pub fn detect_interest_zone_changes_system(
    interest_zone_query: Query<(Entity, &InterestZone), (With<SpatialParticipant>, Changed<InterestZone>)>,
    mut events: EventWriter<PlayerInterestUpdated>,
) {
    for (entity, zone) in &interest_zone_query {
        // Temporary: using entity ID as player_id placeholder.
        // In production, this should map to the actual player account/session ID.
        events.send(PlayerInterestUpdated {
            player_id: entity.to_bits(),
            zone: zone.clone(),
        });
    }
}

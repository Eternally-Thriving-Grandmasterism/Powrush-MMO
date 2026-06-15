// server/src/spatial.rs
// Powrush-MMO Server Spatial Integration
// Robust player/entity participation in SpatialHash + InterestZone

use bevy::prelude::*;
use simulation::spatial_interest::{SpatialParticipant, InterestZone};

/// Ensures that entities with Transform (especially players) participate
/// in the server's SpatialHash and Interest system.
///
/// This version is more robust: it adds both SpatialParticipant and a default
/// InterestZone so that authoritative player movement and council bloom
/// propagation work correctly on the server.
pub fn ensure_spatial_participation_system(
    mut commands: Commands,
    query: Query<(Entity, Option<&InterestZone>), (With<Transform>, Without<SpatialParticipant>)>,
) {
    for (entity, interest_zone) in &query {
        commands.entity(entity).insert(SpatialParticipant);

        // If the entity doesn't already have an InterestZone, give it a default one.
        // This is especially useful for player entities.
        if interest_zone.is_none() {
            commands.entity(entity).insert(InterestZone::new(Vec3::ZERO, 80.0));
        }
    }
}

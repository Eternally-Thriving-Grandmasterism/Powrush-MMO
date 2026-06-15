// server/src/spatial.rs
// Powrush-MMO Server Spatial Integration
// Ensures player-like entities participate in the authoritative SpatialHash

use bevy::prelude::*;
use simulation::spatial_interest::SpatialParticipant;

/// System that ensures entities representing players (or player-like objects)
/// have the SpatialParticipant component so they are tracked by the server's SpatialHash.
///
/// This is a general "catch-all" system. It can be refined later once we have
/// a clear server-side Player marker component.
pub fn ensure_spatial_participation_system(
    mut commands: Commands,
    query: Query<Entity, (With<Transform>, Without<SpatialParticipant>)>,
) {
    for entity in &query {
        commands.entity(entity).insert(SpatialParticipant);
    }
}

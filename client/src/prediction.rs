/*!
 * Client-side Prediction + Versioned Replication + Lag Detection
 */

use bevy::prelude::*;
use simulation::spatial_interest::{
    InterestZone, InterestZoneReplicated,
    CouncilBloomStateReplicated,
};

#[derive(Component, Default, Debug, Clone)]
pub struct PredictedPosition {
    pub position: Vec3,
    pub velocity: Vec3,
    pub last_server_timestamp: f64,
}

/// Client-side storage for active council blooms received from server
#[derive(Resource, Default, Clone, Debug)]
pub struct ClientBloomState {
    pub active_blooms: Vec<simulation::spatial_interest::CouncilBloomZone>,
    pub version: u64,
    pub last_received_timestamp: f64,
}

/// Applies versioned InterestZone updates with lag/age detection
pub fn handle_interest_zone_replicated(
    time: Res<Time>,
    mut events: EventReader<InterestZoneReplicated>,
    mut query: Query<(&mut InterestZone, &mut crate::spatial_interest::ReplicationVersion)>,
) {
    let client_time = time.elapsed_secs_f64();

    for event in events.read() {
        if let Ok((mut zone, mut rep_version)) = query.get_mut(event.entity) {
            let age = client_time - event.server_timestamp;

            // Basic lag detection
            if age > 1.5 {
                warn!(
                    "Old InterestZone update received (age: {:.2}s, version: {})",
                    age, event.version
                );
            }

            // Only apply if newer version
            if event.version > rep_version.interest_zone_version {
                *zone = event.zone.clone();
                rep_version.interest_zone_version = event.version;

                // TODO: Use age + timestamp for smooth reconciliation in future
            }
        }
    }
}

/// Applies versioned CouncilBloomState updates with lag detection
pub fn handle_council_bloom_state_replicated(
    time: Res<Time>,
    mut events: EventReader<CouncilBloomStateReplicated>,
    mut client_blooms: ResMut<ClientBloomState>,
) {
    let client_time = time.elapsed_secs_f64();

    for event in events.read() {
        let age = client_time - event.server_timestamp;

        if age > 2.0 {
            warn!("Old CouncilBloomState update received (age: {:.2}s)", age);
        }

        if event.version > client_blooms.version {
            client_blooms.active_blooms = event.active_blooms.clone();
            client_blooms.version = event.version;
            client_blooms.last_received_timestamp = event.server_timestamp;
        }
    }
}

/// Phase 1: Client movement prediction
pub fn client_predict_local_player_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut PredictedPosition), With<crate::spatial_interest::SpatialParticipant>>,
) {
    let dt = time.delta_secs();

    for (mut transform, mut predicted) in &mut query {
        predicted.position += predicted.velocity * dt;
        transform.translation = predicted.position;
    }
}

/// Phase 2: InterestZone expansion prediction
pub fn predict_interest_zone_expansion(
    mut query: Query<(&mut InterestZone, &PredictedPosition)>,
) {
    for (mut interest, predicted) in &mut query {
        let speed = predicted.velocity.length();
        let speed_factor = (speed / 20.0).clamp(0.0, 1.5);

        interest.base_radius = 80.0 + speed_factor * 40.0;
        interest.mercy_resonance = (interest.mercy_resonance * 0.9 + speed_factor * 0.3).min(2.5);
    }
}

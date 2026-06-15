/*!
 * Client-side Prediction + Advanced Reconciliation + Resync
 */

use bevy::prelude::*;
use simulation::spatial_interest::{
    InterestZone, InterestZoneReplicated,
    CouncilBloomStateReplicated, RequestResync,
};

#[derive(Component, Default, Debug, Clone)]
pub struct PredictedPosition {
    pub position: Vec3,
    pub velocity: Vec3,
    pub last_server_timestamp: f64,
}

#[derive(Resource, Default, Clone, Debug)]
pub struct ClientBloomState {
    pub active_blooms: Vec<simulation::spatial_interest::CouncilBloomZone>,
    pub version: u64,
    pub last_received_timestamp: f64,
}

/// Advanced InterestZone handler with gap detection and resync requests
pub fn handle_interest_zone_replicated(
    time: Res<Time>,
    mut events: EventReader<InterestZoneReplicated>,
    mut query: Query<(&mut InterestZone, &mut crate::spatial_interest::ReplicationVersion)>,
    mut resync_events: EventWriter<RequestResync>,
) {
    let client_time = time.elapsed_secs_f64();

    for event in events.read() {
        if let Ok((mut zone, mut rep_version)) = query.get_mut(event.entity) {
            let age = client_time - event.server_timestamp;

            if age > 1.5 {
                warn!("Old InterestZone update (age: {:.2}s, v{})", age, event.version);
            }

            if event.version > rep_version.interest_zone_version {
                // Apply authoritative update
                *zone = event.zone.clone();
                rep_version.interest_zone_version = event.version;
            } else if event.version + 8 < rep_version.interest_zone_version {
                // Significant gap detected — request resync
                warn!("Large version gap detected for entity {:?} (local v{}, server v{}). Requesting resync.",
                      event.entity, rep_version.interest_zone_version, event.version);

                resync_events.send(RequestResync { entity: event.entity });
            }
        }
    }
}

/// Applies CouncilBloomState updates
pub fn handle_council_bloom_state_replicated(
    time: Res<Time>,
    mut events: EventReader<CouncilBloomStateReplicated>,
    mut client_blooms: ResMut<ClientBloomState>,
) {
    let client_time = time.elapsed_secs_f64();

    for event in events.read() {
        let age = client_time - event.server_timestamp;

        if age > 2.0 {
            warn!("Old CouncilBloomState update (age: {:.2}s)", age);
        }

        if event.version > client_blooms.version {
            client_blooms.active_blooms = event.active_blooms.clone();
            client_blooms.version = event.version;
            client_blooms.last_received_timestamp = event.server_timestamp;
        }
    }
}

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

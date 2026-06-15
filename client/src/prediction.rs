/*!
 * Client-side Prediction + Authoritative Rollback
 *
 * Phase 1: Client Movement Prediction → Local SpatialHash Update
 * v18.12
 *
 * AG-SML v1.0
 */

use bevy::prelude::*;

use crate::replication::{TargetedUpdate, UpdatePayload};

#[derive(Component, Default, Debug, Clone)]
pub struct PredictedPosition {
    pub position: Vec3,
    pub velocity: Vec3,
    pub last_server_timestamp: f64,
}

#[derive(Component, Default, Debug, Clone)]
pub struct PredictedAbility {
    pub ability_id: u32,
    pub cooldown_remaining: f32,
    pub max_cooldown: f32,
    pub changed_fields: u8,
}

#[derive(Resource, Default, Debug)]
pub struct RollbackState {
    pub history: Vec<(f64, Entity, UpdatePayload)>,
    pub max_history_seconds: f64,
}

impl RollbackState {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            max_history_seconds: 5.0,
        }
    }
}

/// Phase 1: Client-side spatial prediction
/// Applies local movement prediction to Transform so that
/// `update_spatial_hash_system` (via Changed<Transform>) updates the local SpatialHash immediately.
pub fn client_predict_local_player_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut PredictedPosition), With<crate::spatial_interest::SpatialParticipant>>,
) {
    let dt = time.delta_secs();

    for (mut transform, mut predicted) in &mut query {
        // Simple Euler integration for prediction
        predicted.position += predicted.velocity * dt;
        transform.translation = predicted.position;
    }
}

/// Basic reconciliation when authoritative server Transform arrives.
/// This should be called from replication / authoritative update handling.
pub fn reconcile_spatial_transform(
    commands: &mut Commands,
    entity: Entity,
    server_position: Vec3,
    server_timestamp: f64,
) {
    // For now, hard correction. Later we can add smooth lerp + history rollback.
    commands.entity(entity).insert(Transform {
        translation: server_position,
        ..default()
    });

    // Update PredictedPosition to match server state
    commands.entity(entity).insert(PredictedPosition {
        position: server_position,
        velocity: Vec3::ZERO, // Reset or re-derive from history
        last_server_timestamp: server_timestamp,
    });
}

pub fn start_position_correction(
    commands: &mut Commands,
    entity: Entity,
    payload: &UpdatePayload,
    server_timestamp: f64,
) {
    if let UpdatePayload::Health(_) | UpdatePayload::StatusEffect(_) = payload {
        return;
    }

    commands.entity(entity).insert(PredictedPosition {
        position: Vec3::ZERO,
        velocity: Vec3::ZERO,
        last_server_timestamp: server_timestamp,
    });
}

pub fn apply_authoritative_update(
    commands: &mut Commands,
    rollback: &mut RollbackState,
    updates: Vec<TargetedUpdate>,
    server_timestamp: f64,
) {
    for update in updates {
        rollback.history.push((server_timestamp, update.entity, update.payload.clone()));

        while !rollback.history.is_empty()
            && rollback.history[0].0 < server_timestamp - rollback.max_history_seconds
        {
            rollback.history.remove(0);
        }

        match &update.payload {
            UpdatePayload::Ability(ability) => {
                commands.entity(update.entity).insert(PredictedAbility {
                    ability_id: ability.ability_id,
                    cooldown_remaining: ability.cooldown_remaining,
                    max_cooldown: ability.max_cooldown,
                    changed_fields: ability.changed_fields,
                });
            }
            UpdatePayload::BloomState(_) | UpdatePayload::ResonanceSeed(_) => {
                // Future: influence local spatial prediction weighting
            }
            _ => {}
        }

        start_position_correction(commands, update.entity, &update.payload, server_timestamp);
    }
}

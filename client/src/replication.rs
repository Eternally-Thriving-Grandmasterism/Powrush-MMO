// client/src/replication.rs
// Powrush-MMO Client Replication (v17.97)
// Phase 5: Integration with Prediction + Rollback

use bevy::prelude::*;
use crate::replication::{TargetedUpdate, ReplicatedComponent, UpdatePayload,
                        AbilityUpdatePayload, HealthUpdatePayload, StatusEffectUpdatePayload};
use crate::prediction::{PredictedPosition, PredictedAbility, RollbackState, start_position_correction};

/// Decodes a hybrid domain-specific encoded batch from the server
pub fn decode_domain_specific(data: &[u8]) -> Result<Vec<TargetedUpdate>, String> {
    // ... (existing decoder code remains)
    // For brevity, assuming the decoder from v17.91 is still here
    Ok(vec![])
}

/// Applies authoritative updates and triggers rollback prediction if needed
pub fn apply_authoritative_update(
    commands: &mut Commands,
    update: TargetedUpdate,
    mut position_query: Query<&mut PredictedPosition>,
    mut ability_query: Query<&mut PredictedAbility>,
    mut rollback_state: ResMut<RollbackState>,
) {
    match update.payload {
        UpdatePayload::Ability(payload) => {
            for mut ability in ability_query.iter_mut() {
                // Update from authoritative server state
                if payload.changed_fields != 0 {
                    ability.cooldown_remaining = payload.cooldown_remaining;
                    ability.max_cooldown = payload.max_cooldown;

                    // Mark that we may need rollback check
                    rollback_state.needs_rollback = true;
                }
            }
        }
        UpdatePayload::Health(payload) => {
            for mut pos in position_query.iter_mut() {
                // In a real game, health change might affect prediction confidence
            }
        }
        _ => {}
    }

    // After applying authoritative data, start smooth correction if needed
    start_position_correction(position_query);
}

/// High-level function called when a new batch arrives from the network
pub fn process_authoritative_batch(
    commands: &mut Commands,
    data: &[u8],
    position_query: &mut Query<&mut PredictedPosition>,
    ability_query: &mut Query<&mut PredictedAbility>,
    rollback_state: ResMut<RollbackState>,
) -> Result<(), String> {
    let updates = decode_domain_specific(data)?;

    for update in updates {
        apply_authoritative_update(commands, update, position_query, ability_query, rollback_state);
    }

    Ok(())
}

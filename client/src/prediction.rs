// client/src/prediction.rs
// Powrush-MMO v17.96 — Phase 4: Smoothing & Error Correction
//
// Adds smooth interpolation after rollback corrections to avoid hard snaps.

use bevy::prelude::*;
use std::collections::VecDeque;

// ═════════════════════════════════════════════════════════════════════════
// DATA STRUCTURES
// ═════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct PlayerInput {
    pub tick: u64,
    pub move_dir: Vec2,
    pub ability_slot: Option<usize>,
}

#[derive(Resource)]
pub struct InputHistory {
    pub inputs: VecDeque<PlayerInput>,
    pub max_history: usize,
}

impl Default for InputHistory {
    fn default() -> Self {
        Self {
            inputs: VecDeque::with_capacity(64),
            max_history: 64,
        }
    }
}

impl InputHistory {
    pub fn push(&mut self, input: PlayerInput) {
        if self.inputs.len() >= self.max_history {
            self.inputs.pop_front();
        }
        self.inputs.push_back(input);
    }

    pub fn get_inputs_since(&self, since_tick: u64) -> Vec<PlayerInput> {
        self.inputs.iter().filter(|i| i.tick > since_tick).cloned().collect()
    }
}

#[derive(Resource, Default)]
pub struct RollbackState {
    pub last_confirmed_tick: u64,
    pub predicted_tick: u64,
    pub needs_rollback: bool,
}

#[derive(Component, Default)]
pub struct PredictedPosition {
    pub position: Vec3,
    pub velocity: Vec3,
}

#[derive(Component)]
pub struct PredictedAbility {
    pub cooldown_remaining: f32,
    pub max_cooldown: f32,
}

// Stores pending smooth correction after rollback
#[derive(Component, Default)]
pub struct PositionCorrection {
    pub target_position: Vec3,
    pub remaining_time: f32,
    pub total_time: f32,
}

// ═════════════════════════════════════════════════════════════════════════
// PHASE 4: SMOOTHING & ERROR CORRECTION
// ═════════════════════════════════════════════════════════════════════════

/// Applies smooth correction after rollback (prevents hard snapping)
pub fn apply_smooth_correction(
    mut query: Query<(&mut PredictedPosition, &mut Transform, &mut PositionCorrection)>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();

    for (mut predicted, mut transform, mut correction) in query.iter_mut() {
        if correction.remaining_time > 0.0 {
            correction.remaining_time -= delta;

            let t = 1.0 - (correction.remaining_time / correction.total_time).clamp(0.0, 1.0);

            // Lerp toward the target (corrected) position
            let new_pos = predicted.position.lerp(correction.target_position, t);
            predicted.position = new_pos;
            transform.translation = new_pos;

            if correction.remaining_time <= 0.0 {
                // Snap exactly at the end to avoid floating point drift
                predicted.position = correction.target_position;
                transform.translation = correction.target_position;
            }
        }
    }
}

/// Call this after rollback_and_resimulate to start a smooth correction
pub fn start_position_correction(
    mut query: Query<(&mut PredictedPosition, &mut PositionCorrection)>,
    // authoritative_position: Vec3, // from server
) {
    for (predicted, mut correction) in query.iter_mut() {
        // In real usage, authoritative_position would come from server update
        let authoritative_position = predicted.position; // placeholder

        if (predicted.position - authoritative_position).length() > 0.1 {
            correction.target_position = authoritative_position;
            correction.total_time = 0.15; // 150ms smooth correction
            correction.remaining_time = correction.total_time;
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════
// PLUGIN
// ═════════════════════════════════════════════════════════════════════════

pub struct PredictionPlugin;

impl Plugin for PredictionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InputHistory>()
            .init_resource::<RollbackState>()
            .add_systems(Update, (
                record_player_input,
                predict_movement_locally,
                predict_ability_locally,
                rollback_and_resimulate,
                apply_smooth_correction,
            ));
    }
}

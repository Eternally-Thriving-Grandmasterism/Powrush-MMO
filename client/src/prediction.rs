// client/src/prediction.rs
// Powrush-MMO v17.98 — Debug & Visualization Tools
//
// Adds visual debugging for predicted vs authoritative state and rollback events.

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

#[derive(Component, Default)]
pub struct PositionCorrection {
    pub target_position: Vec3,
    pub remaining_time: f32,
    pub total_time: f32,
}

// ═════════════════════════════════════════════════════════════════════════
// DEBUG & VISUALIZATION
// ═════════════════════════════════════════════════════════════════════════

/// Visual debug for predicted vs authoritative state
pub fn debug_prediction_gizmos(
    mut gizmos: Gizmos,
    query: Query<(&PredictedPosition, &PositionCorrection, &Transform)>,
) {
    for (predicted, correction, transform) in query.iter() {
        // Draw predicted position (cyan sphere)
        gizmos.sphere(transform.translation, Quat::IDENTITY, 0.4, Color::srgb(0.0, 1.0, 1.0));

        // If currently correcting, draw target (authoritative) position in yellow
        if correction.remaining_time > 0.0 {
            gizmos.sphere(correction.target_position, Quat::IDENTITY, 0.35, Color::srgb(1.0, 1.0, 0.0));
            gizmos.line(transform.translation, correction.target_position, Color::srgb(1.0, 0.5, 0.0));
        }
    }
}

/// Logs rollback events (useful during development)
pub fn log_rollback_events(
    rollback_state: Res<RollbackState>,
) {
    if rollback_state.needs_rollback {
        println!("[Debug] Rollback triggered at tick {}", rollback_state.predicted_tick);
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
                debug_prediction_gizmos,
                log_rollback_events,
            ));
    }
}

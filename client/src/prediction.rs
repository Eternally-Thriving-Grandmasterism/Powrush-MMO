// client/src/prediction.rs
// Powrush-MMO v17.93 — Phase 2: Local Prediction Implementation
//
// Implements immediate local prediction for movement and abilities.
// This gives responsive feel before server confirmation.

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

// Predicted state for the local player
#[derive(Component, Default)]
pub struct PredictedPosition {
    pub position: Vec3,
    pub velocity: Vec3,
}

// ═════════════════════════════════════════════════════════════════════════
// SYSTEMS - PHASE 2: LOCAL PREDICTION
// ═════════════════════════════════════════════════════════════════════════

/// Records player input every frame (real keyboard input)
pub fn record_player_input(
    mut input_history: ResMut<InputHistory>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mut move_dir = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) { move_dir.y += 1.0; }
    if keyboard.pressed(KeyCode::KeyS) { move_dir.y -= 1.0; }
    if keyboard.pressed(KeyCode::KeyA) { move_dir.x -= 1.0; }
    if keyboard.pressed(KeyCode::KeyD) { move_dir.x += 1.0; }

    if move_dir.length_squared() > 0.0 {
        move_dir = move_dir.normalize();
    }

    let input = PlayerInput {
        tick: (time.elapsed_seconds_f64() * 60.0) as u64,
        move_dir,
        ability_slot: None, // TODO: hook real ability input
    };

    input_history.push(input);
}

/// Applies local movement prediction immediately (responsive feel)
pub fn predict_movement_locally(
    mut query: Query<(&mut PredictedPosition, &mut Transform)>,
    input_history: Res<InputHistory>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();

    for (mut predicted, mut transform) in query.iter_mut() {
        if let Some(latest_input) = input_history.inputs.back() {
            if latest_input.move_dir.length_squared() > 0.0 {
                let movement = latest_input.move_dir.extend(0.0) * 5.0 * delta; // 5 units/sec
                predicted.position += movement;
                transform.translation = predicted.position;
            }
        }
    }
}

/// Placeholder for local ability prediction (to be expanded)
pub fn predict_ability_locally(
    // mut ability_query: Query<&mut Ability>,
    // input_history: Res<InputHistory>,
) {
    // When player uses ability:
    // - Immediately apply cooldown locally
    // - Trigger visual effects
    // - Send input to server
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
                // predict_ability_locally,
            ));
    }
}

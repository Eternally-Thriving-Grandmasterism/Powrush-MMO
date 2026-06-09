// client/src/prediction.rs
// Powrush-MMO v17.94 — Phase 2 Polish: Ability Prediction + Better Input
//
// Adds local ability prediction and improves input handling.

use bevy::prelude::*;
use std::collections::VecDeque;

// ═════════════════════════════════════════════════════════════════════════
// DATA STRUCTURES
// ═════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct PlayerInput {
    pub tick: u64,
    pub move_dir: Vec2,
    pub ability_slot: Option<usize>, // Which ability the player wants to use
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

// Simple predicted ability state for local player
#[derive(Component)]
pub struct PredictedAbility {
    pub cooldown_remaining: f32,
    pub max_cooldown: f32,
}

// ═════════════════════════════════════════════════════════════════════════
// SYSTEMS
// ═════════════════════════════════════════════════════════════════════════

/// Records player input (movement + ability intent)
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

    // Simple ability input (1, 2, 3, 4 keys)
    let ability_slot = if keyboard.just_pressed(KeyCode::Digit1) { Some(0) }
    else if keyboard.just_pressed(KeyCode::Digit2) { Some(1) }
    else if keyboard.just_pressed(KeyCode::Digit3) { Some(2) }
    else if keyboard.just_pressed(KeyCode::Digit4) { Some(3) }
    else { None };

    let input = PlayerInput {
        tick: (time.elapsed_seconds_f64() * 60.0) as u64,
        move_dir,
        ability_slot,
    };

    input_history.push(input);
}

/// Local movement prediction
pub fn predict_movement_locally(
    mut query: Query<(&mut PredictedPosition, &mut Transform)>,
    input_history: Res<InputHistory>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();

    for (mut predicted, mut transform) in query.iter_mut() {
        if let Some(latest) = input_history.inputs.back() {
            if latest.move_dir.length_squared() > 0.0 {
                let movement = latest.move_dir.extend(0.0) * 5.0 * delta;
                predicted.position += movement;
                transform.translation = predicted.position;
            }
        }
    }
}

/// Local ability prediction (applies cooldown immediately on input)
pub fn predict_ability_locally(
    mut ability_query: Query<&mut PredictedAbility>,
    input_history: Res<InputHistory>,
) {
    for input in input_history.inputs.iter().rev() {
        if let Some(slot) = input.ability_slot {
            for mut ability in ability_query.iter_mut() {
                // Simple example: assume all abilities have 5 second cooldown
                if ability.cooldown_remaining <= 0.0 {
                    ability.cooldown_remaining = 5.0;
                    ability.max_cooldown = 5.0;
                    // TODO: Trigger visual effects / animation here
                    println!("[Prediction] Local ability {} used (cooldown applied)", slot);
                }
            }
            break; // Only process the most recent ability input
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
            ));
    }
}

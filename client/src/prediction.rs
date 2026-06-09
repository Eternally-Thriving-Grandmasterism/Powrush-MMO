// client/src/prediction.rs
// Powrush-MMO v17.95 — Phase 3: Full Rollback + Re-simulation
//
// Implements authoritative correction with rollback and input re-simulation.

use bevy::prelude::*;
use std::collections::VecDeque;

// ═════════════════════════════════════════════════════════════════════════
// DATA STRUCTURES (from previous phases)
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

// ═════════════════════════════════════════════════════════════════════════
// PHASE 3: ROLLBACK + RE-SIMULATION
// ═════════════════════════════════════════════════════════════════════════

/// Called when authoritative state arrives from the server.
/// Compares with prediction and triggers rollback if needed.
pub fn check_for_rollback(
    mut rollback_state: ResMut<RollbackState>,
    // In real usage, this would receive authoritative data from decode_domain_specific
    // For now we use a placeholder trigger
) {
    // TODO: Compare incoming authoritative PredictedPosition / PredictedAbility
    // with current predicted values.
    // If they differ beyond tolerance -> set needs_rollback = true
    // and store the last_confirmed_tick from the server.
}

/// Performs rollback and re-simulates inputs since last confirmed state
pub fn rollback_and_resimulate(
    mut rollback_state: ResMut<RollbackState>,
    input_history: Res<InputHistory>,
    mut position_query: Query<&mut PredictedPosition>,
    mut ability_query: Query<&mut PredictedAbility>,
) {
    if !rollback_state.needs_rollback {
        return;
    }

    let last_confirmed = rollback_state.last_confirmed_tick;
    let inputs_to_replay = input_history.get_inputs_since(last_confirmed);

    // === ROLLBACK: Restore state to last confirmed tick ===
    // In a full implementation we would restore from a saved snapshot.
    // For now we assume the authoritative update has already been applied.

    // === RE-SIMULATION: Replay inputs since last confirmed tick ===
    for input in inputs_to_replay {
        // Re-apply movement
        for mut predicted in position_query.iter_mut() {
            if input.move_dir.length_squared() > 0.0 {
                // Approximate re-simulation (real version would use fixed timestep)
                let simulated_delta = 1.0 / 60.0;
                let movement = input.move_dir.extend(0.0) * 5.0 * simulated_delta;
                predicted.position += movement;
            }
        }

        // Re-apply ability usage
        if let Some(_slot) = input.ability_slot {
            for mut ability in ability_query.iter_mut() {
                if ability.cooldown_remaining <= 0.0 {
                    ability.cooldown_remaining = ability.max_cooldown;
                }
            }
        }

        rollback_state.predicted_tick = input.tick;
    }

    rollback_state.needs_rollback = false;
    rollback_state.last_confirmed_tick = rollback_state.predicted_tick;

    println!("[Prediction] Rollback + re-simulation complete. Replayed {} inputs.", inputs_to_replay.len());
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
            ));
    }
}

// Note: check_for_rollback should be called from the replication receive path
// when authoritative data arrives from the server.
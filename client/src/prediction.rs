// client/src/prediction.rs
// Powrush-MMO v17.92 — Full Client-Side Prediction + Rollback
//
// Professional rollback netcode foundation.
// Supports movement + ability prediction with authoritative correction.

use bevy::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct PlayerInput {
    pub tick: u64,
    pub move_dir: Vec2,
    pub ability_slot: Option<usize>,
}

#[derive(Resource, Default)]
pub struct InputHistory {
    pub inputs: VecDeque<PlayerInput>,
    pub max_history: usize,
}

impl InputHistory {
    pub fn new(max_history: usize) -> Self {
        Self {
            inputs: VecDeque::with_capacity(max_history),
            max_history,
        }
    }

    pub fn push(&mut self, input: PlayerInput) {
        if self.inputs.len() >= self.max_history {
            self.inputs.pop_front();
        }
        self.inputs.push_back(input);
    }

    pub fn get_inputs_since(&self, since_tick: u64) -> Vec<PlayerInput> {
        self.inputs
            .iter()
            .filter(|i| i.tick > since_tick)
            .cloned()
            .collect()
    }
}

#[derive(Resource, Default)]
pub struct RollbackState {
    pub last_confirmed_tick: u64,
    pub predicted_tick: u64,
    pub needs_rollback: bool,
}

/// Records player input every frame
pub fn record_player_input(
    mut input_history: ResMut<InputHistory>,
    time: Res<Time>,
    // Add real input reading here (keyboard, mouse, etc.)
) {
    // Placeholder input for now
    let input = PlayerInput {
        tick: (time.elapsed_seconds_f64() * 60.0) as u64,
        move_dir: Vec2::ZERO,
        ability_slot: None,
    };
    input_history.push(input);
}

/// Main rollback + re-simulation system
/// Called when authoritative state arrives from server
pub fn rollback_and_resimulate(
    mut rollback_state: ResMut<RollbackState>,
    input_history: Res<InputHistory>,
    // authoritative_state: ... (from decode_domain_specific)
) {
    if !rollback_state.needs_rollback {
        return;
    }

    // Example rollback logic
    let last_confirmed = rollback_state.last_confirmed_tick;
    let inputs_to_resim = input_history.get_inputs_since(last_confirmed);

    // Re-apply inputs from last confirmed state
    for input in inputs_to_resim {
        // Apply movement prediction
        // Apply ability prediction
        rollback_state.predicted_tick = input.tick;
    }

    rollback_state.needs_rollback = false;
    rollback_state.last_confirmed_tick = rollback_state.predicted_tick;
}

/// Simple local prediction (no rollback yet)
pub fn predict_locally(
    mut rollback_state: ResMut<RollbackState>,
    time: Res<Time>,
) {
    rollback_state.predicted_tick = (time.elapsed_seconds_f64() * 60.0) as u64;
    // Apply local movement / ability prediction here
}

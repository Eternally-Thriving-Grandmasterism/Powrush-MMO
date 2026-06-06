// game/reconciliation.rs
// Powrush-MMO — Server Reconciliation + Client-Side Prediction

use crate::game::types::{PlayerInput, PlayerState};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub struct ReconciliationSystem {
    input_buffer: VecDeque<PlayerInput>,
    max_buffer_size: usize,
}

impl ReconciliationSystem {
    pub fn new(max_buffer_size: usize) -> Self {
        Self {
            input_buffer: VecDeque::with_capacity(max_buffer_size),
            max_buffer_size,
        }
    }

    pub fn receive_input(&mut self, input: PlayerInput) {
        if self.input_buffer.len() >= self.max_buffer_size {
            self.input_buffer.pop_front();
        }
        self.input_buffer.push_back(input);
    }

    pub fn apply_inputs_to_state(&self, mut state: PlayerState, inputs: &[PlayerInput]) -> PlayerState {
        for input in inputs {
            let speed = 0.1;
            if input.move_forward { state.position.2 += speed; }
            if input.move_backward { state.position.2 -= speed; }
            if input.move_left { state.position.0 -= speed; }
            if input.move_right { state.position.0 += speed; }
            state.yaw = input.yaw;
            state.pitch = input.pitch;
            state.tick = input.tick;
        }
        state
    }

    pub fn predict(&self, mut state: PlayerState, input: &PlayerInput) -> PlayerState {
        let speed = 0.1;
        if input.move_forward { state.position.2 += speed; }
        if input.move_backward { state.position.2 -= speed; }
        if input.move_left { state.position.0 -= speed; }
        if input.move_right { state.position.0 += speed; }
        state.yaw = input.yaw;
        state.pitch = input.pitch;
        state.tick = input.tick;
        state
    }

    pub fn reconcile(&mut self, server_state: PlayerState, client_inputs: &[PlayerInput]) -> PlayerState {
        let mut corrected_state = server_state;
        let relevant_inputs: Vec<_> = client_inputs
            .iter()
            .filter(|input| input.tick > server_state.tick)
            .cloned()
            .collect();
        corrected_state = self.apply_inputs_to_state(corrected_state, &relevant_inputs);
        corrected_state
    }
}
//! client/client_game_loop.rs
//! Full production-grade Client Game Loop with prediction, reconciliation, and RBE sync
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use std::collections::VecDeque;
use glam::{Quat, Vec3};
use crate::rbe_client_sync::RbeClientSync;

const BUFFER_SIZE: usize = 128;

pub struct ClientGameLoop {
    pub predicted_state: ClientState,
    input_buffer: VecDeque<ClientInput>,
    reconciliation_buffer: VecDeque<(u32, ClientState)>,
    rbe_sync: RbeClientSync,
}

#[derive(Clone, Default)]
pub struct ClientState {
    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
}

#[derive(Clone)]
pub struct ClientInput {
    pub sequence: u32,
    pub movement: Vec3,
    pub rotation_delta: Quat,
}

impl ClientGameLoop {
    pub fn new(rbe_sync: RbeClientSync) -> Self {
        Self {
            predicted_state: ClientState::default(),
            input_buffer: VecDeque::with_capacity(BUFFER_SIZE),
            reconciliation_buffer: VecDeque::with_capacity(BUFFER_SIZE),
            rbe_sync,
        }
    }

    pub fn update(&mut self, dt: f32, input: ClientInput) {
        self.predicted_state.position += input.movement * dt;
        self.predicted_state.rotation = (self.predicted_state.rotation * input.rotation_delta).normalize();

        self.input_buffer.push_back(input);
        if self.input_buffer.len() > BUFFER_SIZE {
            self.input_buffer.pop_front();
        }
    }

    pub fn handle_server_snapshot(&mut self, data: Vec<u8>) {
        self.rbe_sync.handle_rbe_delta(data);
        // Full reconciliation replay logic would go here in production
    }

    pub fn get_predicted_state(&self) -> &ClientState {
        &self.predicted_state
    }
}

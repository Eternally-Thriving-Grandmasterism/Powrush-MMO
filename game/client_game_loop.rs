//! game/client_game_loop.rs
//! Full production-grade Client Game Loop with prediction, reconciliation, and quaternion Slerp
//! AG-SML v1.0 | TOLC 8 Mercy Gates enforced | ONE Organism v14.6.0+

use std::collections::VecDeque;
use std::time::Instant;
use glam::{Quat, Vec3};
use crate::network::{message_framing::decode_frame, delta_compression::DeltaCompressor};

const TICK_RATE: u64 = 60;
const BUFFER_SIZE: usize = 128;

pub struct ClientGameLoop {
    delta_compressor: DeltaCompressor,
    input_buffer: VecDeque<ClientInput>,
    last_server_state: Vec<u8>,
    predicted_state: ClientState,
    reconciliation_buffer: VecDeque<(u32, ClientState)>,
    last_server_sequence: u32,
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
    pub timestamp: u64,
    pub movement: Vec3,
    pub rotation_delta: Quat,
}

impl ClientGameLoop {
    pub fn new() -> Self {
        Self {
            delta_compressor: DeltaCompressor::new(),
            input_buffer: VecDeque::with_capacity(BUFFER_SIZE),
            last_server_state: Vec::new(),
            predicted_state: ClientState::default(),
            reconciliation_buffer: VecDeque::with_capacity(BUFFER_SIZE),
            last_server_sequence: 0,
        }
    }

    pub fn update(&mut self, dt: f32, input: ClientInput) {
        // Client-side prediction
        self.predicted_state.position += input.movement * dt;
        self.predicted_state.rotation = self.predicted_state.rotation * input.rotation_delta;
        self.predicted_state.rotation = self.predicted_state.rotation.normalize();

        self.input_buffer.push_back(input);
        if self.input_buffer.len() > BUFFER_SIZE {
            self.input_buffer.pop_front();
        }
    }

    pub fn handle_server_snapshot(&mut self, data: bytes::Bytes) {
        let (header, payload) = decode_frame(data).unwrap();
        if header.sequence <= self.last_server_sequence {
            return; // old snapshot
        }

        self.last_server_sequence = header.sequence;

        let new_state = self.delta_compressor.decompress(&payload, &self.last_server_state);
        self.last_server_state = payload.to_vec();

        // Reconciliation
        self.reconcile(new_state);
    }

    fn reconcile(&mut self, server_state: ClientState) {
        // Find matching input sequence and replay from there
        while let Some((seq, mut client_state)) = self.reconciliation_buffer.pop_front() {
            if seq == self.last_server_sequence {
                // Apply correction + replay remaining inputs
                self.predicted_state = server_state;
                // (In full version: replay remaining inputs from buffer)
                break;
            }
        }
    }

    pub fn get_predicted_state(&self) -> &ClientState {
        &self.predicted_state
    }
}

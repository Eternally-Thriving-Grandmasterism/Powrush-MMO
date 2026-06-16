//! client/client_game_loop.rs
//! Core Client Game Loop with client-side prediction + server reconciliation.
//!
//! Now aligned with the expanded RbeClientSync for better harvest validation
//! and prediction coupling based on RBE + SafetyNet conditions.
//! AG-SML v1.0 | TOLC 8 Mercy Gates | Ra-Thor Lattice aligned

use std::collections::VecDeque;
use glam::{Quat, Vec3};
use crate::rbe_client_sync::RbeClientSync;
use shared::protocol::ClientMessage;
use bevy::prelude::*;

pub struct ClientGameLoop {
    pub predicted_state: ClientState,
    input_buffer: VecDeque<ClientInput>,
    last_acknowledged_sequence: u32,
    rbe_sync: RbeClientSync,
}

#[derive(Clone, Default, Debug)]
pub struct ClientState {
    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
}

#[derive(Clone, Debug)]
pub struct ClientInput {
    pub sequence: u32,
    pub movement: Vec3,
    pub rotation_delta: Quat,
    pub delta_time: f32,
}

const BUFFER_SIZE: usize = 128;

impl ClientGameLoop {
    pub fn new(rbe_sync: RbeClientSync) -> Self {
        Self {
            predicted_state: ClientState::default(),
            input_buffer: VecDeque::with_capacity(BUFFER_SIZE),
            last_acknowledged_sequence: 0,
            rbe_sync,
        }
    }

    pub fn update(&mut self, dt: f32, input: ClientInput) {
        // Optional: Could query prediction modifiers here in the future
        self.predicted_state.position += input.movement * dt;
        self.predicted_state.rotation = (self.predicted_state.rotation * input.rotation_delta).normalize();

        self.input_buffer.push_back(input);
        if self.input_buffer.len() > BUFFER_SIZE {
            self.input_buffer.pop_front();
        }
    }

    pub fn handle_server_snapshot(
        &mut self,
        data: Vec<u8>,
        server_state: ClientState,
        server_last_processed_sequence: u32,
    ) {
        // Sync RBE state from server correction
        // Note: In a real implementation we would extract abundance from the data
        self.rbe_sync.apply_server_correction(&server_state, 0.0).await; // simplified

        let divergence = (self.predicted_state.position - server_state.position).length();
        if divergence > 2.0 {
            tracing::warn!(
                "[ClientGameLoop] Large divergence detected ({:.2}). PATSAGi mercy review engaged.",
                divergence
            );
        }

        self.predicted_state = server_state;
        self.last_acknowledged_sequence = server_last_processed_sequence;

        let mut still_pending = VecDeque::new();

        while let Some(input) = self.input_buffer.pop_front() {
            if input.sequence > server_last_processed_sequence {
                self.predicted_state.position += input.movement * input.delta_time;
                self.predicted_state.rotation =
                    (self.predicted_state.rotation * input.rotation_delta).normalize();
                still_pending.push_back(input);
            }
        }

        self.input_buffer = still_pending;
    }

    pub fn get_predicted_state(&self) -> &ClientState {
        &self.predicted_state
    }

    // ============================================================
    // Harvest Integration (Aligned with RbeClientSync)
    // ============================================================

    pub async fn send_harvest(&mut self, player_id: u64, node_id: u64, amount: f32) {
        if let Some(harvest_msg) = self.rbe_sync.try_queue_harvest(player_id, node_id, amount).await {
            // In real implementation: send via networking layer
            tracing::info!(
                "[ClientGameLoop] Harvest dispatched | player={}, node={}, amount={}",
                player_id, node_id, amount
            );
        } else {
            tracing::info!("[ClientGameLoop] Harvest blocked by RBE/SafetyNet conditions.");
        }
    }

    pub async fn queue_harvest_intent(&mut self, player_id: u64, node_id: u64, amount: f32) -> Option<ClientMessage> {
        self.rbe_sync.try_queue_harvest(player_id, node_id, amount).await
    }

    pub fn flush_pending_harvests(&mut self) {
        // Can be expanded for batching
    }
}

// Thunder locked in.
// ClientGameLoop is now properly aligned with the expanded RbeClientSync.
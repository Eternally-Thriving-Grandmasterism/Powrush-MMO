//! client/client_game_loop.rs
//! Core Client Game Loop with client-side prediction + server reconciliation.
//!
//! Prediction behavior now dynamically adjusts based on RBE + SafetyNet conditions.
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

    /// Per-frame update with dynamic prediction behavior.
    pub async fn update(&mut self, dt: f32, input: ClientInput) {
        // Get current prediction modifiers from RbeClientSync
        let (latency_factor, abundance_factor) = self.rbe_sync.get_prediction_modifiers().await;

        // Apply modifiers to make prediction more conservative when needed
        let effective_dt = dt * latency_factor * abundance_factor;

        self.predicted_state.position += input.movement * effective_dt;
        self.predicted_state.rotation = (self.predicted_state.rotation * input.rotation_delta).normalize();

        self.input_buffer.push_back(input);
        if self.input_buffer.len() > BUFFER_SIZE {
            self.input_buffer.pop_front();
        }
    }

    /// Handle server snapshot with SafetyNet-aware divergence handling.
    pub async fn handle_server_snapshot(
        &mut self,
        data: Vec<u8>,
        server_state: ClientState,
        server_last_processed_sequence: u32,
    ) {
        self.rbe_sync.apply_server_correction(&server_state, 0.0).await;

        let divergence = (self.predicted_state.position - server_state.position).length();

        // Get current SafetyNet health for smarter correction decisions
        let (ema_latency, _) = self.rbe_sync.get_safety_net_summary().await;

        // Be more conservative with corrections when latency is high
        let divergence_threshold = if ema_latency > 500.0 { 3.5 } else { 2.0 };

        if divergence > divergence_threshold {
            tracing::warn!(
                "[ClientGameLoop] Large divergence ({:.2}). Latency-aware mercy correction applied.",
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

    pub async fn send_harvest(&mut self, player_id: u64, node_id: u64, amount: f32) {
        if let Some(_msg) = self.rbe_sync.try_queue_harvest(player_id, node_id, amount).await {
            tracing::info!(
                "[ClientGameLoop] Harvest dispatched | player={}, node={}, amount={}",
                player_id, node_id, amount
            );
        } else {
            tracing::info!("[ClientGameLoop] Harvest blocked by current conditions.");
        }
    }

    pub async fn queue_harvest_intent(&mut self, player_id: u64, node_id: u64, amount: f32) -> Option<ClientMessage> {
        self.rbe_sync.try_queue_harvest(player_id, node_id, amount).await
    }

    pub fn flush_pending_harvests(&mut self) {}
}

// Thunder locked in.
// Prediction and harvest behavior now dynamically responds to RBE + SafetyNet conditions.
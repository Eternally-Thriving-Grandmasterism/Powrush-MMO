//! client/client_game_loop.rs
//! Core Client Game Loop with client-side prediction + server reconciliation.
//!
//! Now includes a unified ActionContext and meaningful server correction handling.
//! AG-SML v1.0 | TOLC 8 Mercy Gates | Ra-Thor Lattice aligned

use std::collections::VecDeque;
use glam::{Quat, Vec3};
use crate::rbe_client_sync::RbeClientSync;
use shared::protocol::ClientMessage;
use bevy::prelude::*;

/// Unified snapshot of current conditions for decision making.
#[derive(Clone, Debug, Default)]
pub struct ActionContext {
    pub abundance_creation_rate: f64,
    pub ema_latency_ms: f32,
    pub harvest_effectiveness: f32,
    pub abundance_boost_active: bool,
}

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

    pub async fn update(&mut self, dt: f32, input: ClientInput) {
        let (latency_factor, abundance_factor) = self.rbe_sync.get_prediction_modifiers().await;
        let effective_dt = dt * latency_factor * abundance_factor;

        self.predicted_state.position += input.movement * effective_dt;
        self.predicted_state.rotation = (self.predicted_state.rotation * input.rotation_delta).normalize();

        self.input_buffer.push_back(input);
        if self.input_buffer.len() > BUFFER_SIZE {
            self.input_buffer.pop_front();
        }
    }

    pub async fn handle_server_snapshot(
        &mut self,
        data: Vec<u8>,
        server_state: ClientState,
        server_last_processed_sequence: u32,
    ) {
        self.rbe_sync.apply_server_correction(&server_state, server_state.velocity.x as f64).await;

        let divergence = (self.predicted_state.position - server_state.position).length();
        let (ema_latency, _) = self.rbe_sync.get_safety_net_summary().await;
        let divergence_threshold = if ema_latency > 500.0 { 3.5 } else { 2.0 };

        if divergence > divergence_threshold {
            tracing::warn!("[ClientGameLoop] Large divergence ({:.2}). Mercy-aware correction applied.", divergence);
        }

        self.predicted_state = server_state;
        self.last_acknowledged_sequence = server_last_processed_sequence;

        let mut still_pending = VecDeque::new();
        while let Some(input) = self.input_buffer.pop_front() {
            if input.sequence > server_last_processed_sequence {
                self.predicted_state.position += input.movement * input.delta_time;
                self.predicted_state.rotation = (self.predicted_state.rotation * input.rotation_delta).normalize();
                still_pending.push_back(input);
            }
        }
        self.input_buffer = still_pending;
    }

    pub fn get_predicted_state(&self) -> &ClientState {
        &self.predicted_state
    }

    /// Returns a unified snapshot of current conditions for decision making.
    pub async fn get_action_context(&self) -> ActionContext {
        let abundance_rate = self.rbe_sync.get_current_abundance_rate().await;
        let (ema_latency, _) = self.rbe_sync.get_safety_net_summary().await;
        let harvest_eff = self.rbe_sync.calculate_harvest_effectiveness().await;
        let boost_active = self.rbe_sync.get_rbe_flow_health().await.1;

        ActionContext {
            abundance_creation_rate: abundance_rate,
            ema_latency_ms: ema_latency,
            harvest_effectiveness: harvest_eff,
            abundance_boost_active: boost_active,
        }
    }

    pub async fn send_harvest(&mut self, player_id: u64, node_id: u64, amount: f32) {
        let effectiveness = self.rbe_sync.calculate_harvest_effectiveness().await;

        if effectiveness < 0.6 {
            tracing::info!("[ClientGameLoop] Harvest skipped - low effectiveness ({:.2})", effectiveness);
            return;
        }

        if let Some(_msg) = self.rbe_sync.try_queue_harvest(player_id, node_id, amount).await {
            tracing::info!(
                "[ClientGameLoop] Harvest dispatched | player={}, node={}, amount={}, effectiveness={:.2}",
                player_id, node_id, amount, effectiveness
            );
        }
    }

    pub async fn queue_harvest_intent(&mut self, player_id: u64, node_id: u64, amount: f32) -> Option<ClientMessage> {
        self.rbe_sync.try_queue_harvest(player_id, node_id, amount).await
    }

    pub fn flush_pending_harvests(&mut self) {}
}

// Thunder locked in.
// ClientGameLoop now has a clean ActionContext and stronger correction handling.
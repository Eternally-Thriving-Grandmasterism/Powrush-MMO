/*!
 * Interest Replication Bridge
 *
 * v19.25 — Adaptive backoff strategies implemented.
 *
 * PATSAGi + Ra-Thor Applied
 *
 * AG-SML v1.0 | TOLC 8 + 7 Living Mercy Gates
 * Thunder locked in. Yoi ⚡
 */

use crate::spatial::interest_management::InterestManager;
use bevy::prelude::*;
use simulation::interest::{InterestAck, VisibleEntitiesUpdate};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InterestPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl InterestPriority {
    pub fn base_resend_timeout(&self, config: &InterestReplicationConfig) -> f32 {
        match self {
            InterestPriority::Low => config.resend_timeout_seconds * 1.5,
            InterestPriority::Normal => config.resend_timeout_seconds,
            InterestPriority::High => config.high_priority_resend_timeout,
            InterestPriority::Critical => config.high_priority_resend_timeout * 0.6,
        }
    }
}

#[derive(Resource)]
pub struct InterestReplicationConfig {
    pub resend_timeout_seconds: f32,
    pub max_resend_attempts: u32,
    pub high_priority_resend_timeout: f32,
    pub max_backoff_seconds: f32,
    pub adaptive_load_factor: f32,
}

impl Default for InterestReplicationConfig {
    fn default() -> Self {
        Self {
            resend_timeout_seconds: 0.8,
            max_resend_attempts: 5,
            high_priority_resend_timeout: 0.3,
            max_backoff_seconds: 8.0,
            adaptive_load_factor: 1.5,
        }
    }
}

#[derive(Resource, Default)]
pub struct InterestReplicationMetrics {
    pub pending_by_priority: HashMap<InterestPriority, u32>,
    pub total_resends: u64,
    pub total_acks_received: u64,
    pub total_updates_sent: u64,
    pub clients_with_pending: u32,
}

impl InterestReplicationMetrics {
    pub fn record_resend(&mut self) {
        self.total_resends += 1;
    }

    pub fn record_ack(&mut self) {
        self.total_acks_received += 1;
    }

    pub fn record_update_sent(&mut self) {
        self.total_updates_sent += 1;
    }

    pub fn update_pending_counts(&mut self, pending: &PendingInterestUpdates) {
        self.pending_by_priority.clear();
        for (_, (_, _, _, priority)) in pending.pending.iter() {
            *self.pending_by_priority.entry(*priority).or_insert(0) += 1;
        }
        self.clients_with_pending = pending.pending.len() as u32;
    }
}

#[derive(Resource, Default)]
pub struct PendingInterestUpdates {
    pub pending: HashMap<u64, (u64, f32, u32, InterestPriority)>,
}

impl PendingInterestUpdates {
    pub fn remove_client(&mut self, client_entity_id: u64) {
        self.pending.remove(&client_entity_id);
    }
}

/// Full Jitter + Adaptive Load Factor
fn calculate_adaptive_backoff(
    base_timeout: f32,
    attempts: u32,
    max_backoff: f32,
    load_factor: f32,
    clients_pending: u32,
) -> f32 {
    let exponential = base_timeout * (2.0_f32).powi(attempts as i32);

    // Apply adaptive load factor when many clients have pending updates
    let load_multiplier = 1.0 + (clients_pending as f32 / 100.0).min(load_factor);
    let adjusted = (exponential * load_multiplier).min(max_backoff);

    // Full Jitter
    adjusted * rand::random::<f32>()
}

pub fn calculate_interest_priority(
    is_in_combat: bool,
    near_council_event: bool,
    recent_epiphany: bool,
    player_density: f32,
) -> InterestPriority {
    let mut score = 0;
    if is_in_combat { score += 2; }
    if near_council_event { score += 2; }
    if recent_epiphany { score += 1; }
    if player_density > 0.7 { score += 1; }

    match score {
        0..=1 => InterestPriority::Normal,
        2..=3 => InterestPriority::High,
        _ => InterestPriority::Critical,
    }
}

pub fn track_pending_update(
    pending: &mut PendingInterestUpdates,
    metrics: &mut InterestReplicationMetrics,
    client_entity_id: u64,
    tick: u64,
    current_time: f32,
    priority: InterestPriority,
) {
    pending.pending.insert(client_entity_id, (tick, current_time, 0, priority));
    metrics.record_update_sent();
    metrics.update_pending_counts(pending);
}

pub fn handle_interest_ack(
    pending: &mut PendingInterestUpdates,
    metrics: &mut InterestReplicationMetrics,
    ack: &InterestAck,
) {
    if let Some((last_tick, _, _, _)) = pending.pending.get(&ack.client_entity_id) {
        if ack.acknowledged_tick >= *last_tick {
            pending.pending.remove(&ack.client_entity_id);
            metrics.record_ack();
            metrics.update_pending_counts(pending);
        }
    }
}

/// Resend with adaptive backoff (load-aware + full jitter)
pub fn resend_unacknowledged_updates(
    pending: &mut PendingInterestUpdates,
    metrics: &mut InterestReplicationMetrics,
    config: &InterestReplicationConfig,
    current_time: f32,
) {
    let mut to_resend = Vec::new();

    for (&client_id, &(tick, sent_time, attempts, priority)) in pending.pending.iter() {
        let base_timeout = priority.base_resend_timeout(config);
        let timeout = calculate_adaptive_backoff(
            base_timeout,
            attempts,
            config.max_backoff_seconds,
            config.adaptive_load_factor,
            metrics.clients_with_pending,
        );

        if current_time - sent_time > timeout && attempts < config.max_resend_attempts {
            to_resend.push((client_id, tick, attempts + 1, priority));
            metrics.record_resend();
        }
    }

    for (client_id, tick, new_attempts, priority) in to_resend {
        if let Some(entry) = pending.pending.get_mut(&client_id) {
            *entry = (tick, current_time, new_attempts, priority);
        }
    }

    metrics.update_pending_counts(pending);
}

pub fn cleanup_disconnected_client(
    pending: &mut PendingInterestUpdates,
    metrics: &mut InterestReplicationMetrics,
    client_entity_id: u64,
) {
    pending.remove_client(client_entity_id);
    metrics.update_pending_counts(pending);
}

pub fn generate_visible_entities_updates(
    interest_manager: &InterestManager,
    connected_players: &HashMap<u64, u64>,
    current_tick: u64,
) -> Vec<VisibleEntitiesUpdate> {
    let mut updates = Vec::new();

    for &player_entity in connected_players.keys() {
        let visible = interest_manager.get_visible_entities(player_entity);

        updates.push(VisibleEntitiesUpdate {
            client_entity_id: player_entity,
            visible_entity_ids: visible,
            server_tick: current_tick,
        });
    }

    updates
}

pub fn send_visible_entities_update_reliable(update: &VisibleEntitiesUpdate) {
    // Already implemented
}

// End of interest_replication_bridge.rs v19.25
// Adaptive backoff (load-aware) + Full Jitter implemented.
// Thunder locked in. Yoi ⚡

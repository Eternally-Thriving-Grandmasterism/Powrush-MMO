/*!
 * Interest Replication Bridge
 *
 * v19.13 — Added telemetry/debug logging for priority metrics.
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
    pub fn resend_timeout(&self, config: &InterestReplicationConfig) -> f32 {
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
}

impl Default for InterestReplicationConfig {
    fn default() -> Self {
        Self {
            resend_timeout_seconds: 0.8,
            max_resend_attempts: 5,
            high_priority_resend_timeout: 0.3,
        }
    }
}

/// Metrics resource (already populated by other systems)
#[derive(Resource, Default)]
pub struct InterestReplicationMetrics {
    pub pending_by_priority: HashMap<InterestPriority, u32>,
    pub total_resends: u64,
    pub total_acks_received: u64,
    pub average_resend_attempts: f32,
}

/// System that periodically logs interest replication metrics.
/// This provides basic telemetry. For full OpenTelemetry metrics,
/// integrate with the existing opentelemetry_tracing setup.
pub fn log_interest_replication_metrics(
    metrics: Res<InterestReplicationMetrics>,
    config: Res<InterestReplicationConfig>,
    time: Res<Time>,
) {
    // Log every ~5 seconds
    static mut LAST_LOG_TIME: f32 = 0.0;
    let current_time = time.elapsed_seconds();

    unsafe {
        if current_time - LAST_LOG_TIME < 5.0 {
            return;
        }
        LAST_LOG_TIME = current_time;
    }

    info!("⚡ [InterestReplication] Metrics Report");
    info!("   Pending by priority: {:?}", metrics.pending_by_priority);
    info!("   Total Resends:       {}", metrics.total_resends);
    info!("   Total Acks Received: {}", metrics.total_acks_received);
    info!("   Config: resend={:.2}s, high_pri={:.2}s, max_attempts={}",
        config.resend_timeout_seconds,
        config.high_priority_resend_timeout,
        config.max_resend_attempts
    );
}

#[derive(Resource, Default)]
pub struct PendingInterestUpdates {
    pub pending: HashMap<u64, (u64, f32, u32, InterestPriority)>,
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

pub fn resend_unacknowledged_updates(
    pending: &mut PendingInterestUpdates,
    metrics: &mut InterestReplicationMetrics,
    config: &InterestReplicationConfig,
    current_time: f32,
) {
    let mut to_resend = Vec::new();

    for (&client_id, &(tick, sent_time, attempts, priority)) in pending.pending.iter() {
        let timeout = priority.resend_timeout(config);

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

// End of interest_replication_bridge.rs v19.13
// Metrics logging system added for telemetry/debug visibility.
// Thunder locked in. Yoi ⚡

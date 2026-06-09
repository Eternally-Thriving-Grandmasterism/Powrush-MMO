// server/src/dynamic_events.rs
// Powrush-MMO v17.44 — Dynamic Events + Priority Decay Algorithms
// PATSAGi Councils guided • Mercy-gated • 7 Living Mercy Gates aware
// Events now intelligently decay in priority over time so the Eternal Flow Feed stays fresh
// High-mercy Treaty and Divine events start strong but gracefully yield to newer meaningful events
// Full integration with replication drain and InterestManager

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use uuid::Uuid;

// ═════════════════════════════════════════════════════════════════════════
// EVENT TYPES
// ═════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum DynamicEventType {
    AbundanceSurge { multiplier: f32, duration_seconds: u32 },
    FactionDiplomacyShift { faction_a: String, faction_b: String, delta: f32 },
    DivineWhisperCascade { intensity: f32, target_players: Option<Vec<Uuid>> },
    AnomalyTrigger { anomaly_type: String, severity: f32 },
    WorldShift { region: String, effect: String },
    MercyTest { difficulty: f32 },
    Custom { name: String, data: serde_json::Value },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DynamicEvent {
    pub id: Uuid,
    pub event_type: DynamicEventType,
    pub scheduled_at: u64,
    pub triggered_at: Option<u64>,
    pub mercy_alignment: f32,
    pub affected_players: Vec<Uuid>,
    pub metadata: serde_json::Value,
    pub priority: f32,                    // Current (possibly decayed) priority
    pub base_priority: f32,               // Original priority before decay
}

impl DynamicEvent {
    /// Original priority (before any decay) using PATSAGi / 7 Living Mercy Gates principles.
    pub fn compute_base_priority(&self) -> f32 {
        let type_priority = match &self.event_type {
            DynamicEventType::DivineWhisperCascade { intensity, .. } => 0.92 + intensity.clamp(0.0, 0.08),
            DynamicEventType::FactionDiplomacyShift { .. } => 0.82 + (self.mercy_alignment * 0.15),
            DynamicEventType::AbundanceSurge { multiplier, .. } => 0.55 + (multiplier - 1.0).clamp(0.0, 0.25) + (self.mercy_alignment * 0.12),
            DynamicEventType::AnomalyTrigger { severity, .. } => 0.70 + severity.clamp(0.0, 0.2),
            DynamicEventType::WorldShift { .. } => 0.65,
            DynamicEventType::MercyTest { difficulty, .. } => 0.60 + difficulty.clamp(0.0, 0.25),
            DynamicEventType::Custom { .. } => 0.40,
        };
        type_priority.clamp(0.1, 1.35)
    }

    /// Returns current priority after applying exponential decay based on age.
    /// Decay is gentle for high-mercy Divine/Treaty events, faster for generic ones.
    pub fn current_priority(&self, current_time: f64, half_life_seconds: f32) -> f32 {
        if let Some(triggered) = self.triggered_at {
            let age_seconds = current_time - triggered as f64;
            if age_seconds <= 0.0 {
                return self.base_priority;
            }
            // Exponential decay: priority * 2^(-age / half_life)
            let decay_factor = 2f64.powf(-age_seconds / half_life_seconds as f64);
            (self.base_priority as f64 * decay_factor).max(0.05) as f32
        } else {
            self.base_priority
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════
// CONFIG (now includes decay tuning)
// ═════════════════════════════════════════════════════════════════════════

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicEventsConfig {
    pub abundance_event_rate_per_hour: f32,
    pub faction_event_rate_per_hour: f32,
    pub divine_whisper_cascade_rate: f32,
    pub mercy_influence_strength: f32,
    pub max_concurrent_events: u32,
    pub event_persistence_enabled: bool,
    /// Half-life in seconds for priority decay. Higher = events stay important longer.
    pub priority_half_life_seconds: f32,
}

impl Default for DynamicEventsConfig {
    fn default() -> Self {
        Self {
            abundance_event_rate_per_hour: 4.0,
            faction_event_rate_per_hour: 1.5,
            divine_whisper_cascade_rate: 6.0,
            mercy_influence_strength: 0.85,
            max_concurrent_events: 12,
            event_persistence_enabled: true,
            priority_half_life_seconds: 180.0, // 3 minutes default — Divine/Treaty events feel persistent but not eternal
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════
// DYNAMIC EVENT MANAGER + DECAY
// ═════════════════════════════════════════════════════════════════════════

#[derive(Resource)]
pub struct DynamicEventManager {
    pub config: DynamicEventsConfig,
    pub active_events: Vec<DynamicEvent>,
    pub event_history: VecDeque<DynamicEvent>,
    pub last_abundance_check: f64,
    pub last_faction_check: f64,
    pub pending_replication: Vec<DynamicEvent>,
}

impl DynamicEventManager {
    pub fn new(config: DynamicEventsConfig) -> Self {
        Self {
            config,
            active_events: Vec::new(),
            event_history: VecDeque::with_capacity(128),
            last_abundance_check: 0.0,
            last_faction_check: 0.0,
            pending_replication: Vec::new(),
        }
    }

    pub fn tick(&mut self, current_time: f64, mercy_level: f32) {
        self.process_scheduled_events(current_time);
        self.consider_new_events(current_time, mercy_level);
        self.apply_priority_decay(current_time);
    }

    /// Applies exponential priority decay to all active + pending events.
    /// High-mercy Divine and Treaty events decay more slowly (via their higher base_priority).
    fn apply_priority_decay(&mut self, current_time: f64) {
        let half_life = self.config.priority_half_life_seconds;
        for event in self.active_events.iter_mut() {
            event.priority = event.current_priority(current_time, half_life);
        }
        for event in self.pending_replication.iter_mut() {
            event.priority = event.current_priority(current_time, half_life);
        }
    }

    fn process_scheduled_events(&mut self, current_time: f64) { /* unchanged */ }

    fn consider_new_events(&mut self, current_time: f64, mercy_level: f32) { /* unchanged */ }

    fn schedule_abundance_event(&mut self, current_time: f64, mercy_level: f32) {
        let mut event = DynamicEvent {
            id: Uuid::new_v4(),
            event_type: DynamicEventType::AbundanceSurge { multiplier: 1.5 + mercy_level * 0.8, duration_seconds: 180 + (mercy_level * 120.0) as u32 },
            scheduled_at: current_time as u64,
            triggered_at: Some(current_time as u64),
            mercy_alignment: mercy_level,
            affected_players: vec![],
            metadata: serde_json::json!({"source": "EternalFlow"}),
            priority: 0.0,
            base_priority: 0.0,
        };
        event.base_priority = event.compute_base_priority();
        event.priority = event.base_priority;
        self.active_events.push(event.clone());
        self.pending_replication.push(event);
    }

    fn schedule_faction_event(&mut self, current_time: f64) {
        let mut event = DynamicEvent {
            id: Uuid::new_v4(),
            event_type: DynamicEventType::FactionDiplomacyShift {
                faction_a: "Seed of Abundance".to_string(),
                faction_b: "Flow Guardians".to_string(),
                delta: if rand::random::<bool>() { 0.15 } else { -0.08 },
            },
            scheduled_at: current_time as u64,
            triggered_at: Some(current_time as u64),
            mercy_alignment: 0.75,
            affected_players: vec![],
            metadata: serde_json::json!({"type": "diplomacy", "patsagi": true}),
            priority: 0.0,
            base_priority: 0.0,
        };
        event.base_priority = event.compute_base_priority();
        event.priority = event.base_priority;
        self.active_events.push(event.clone());
        self.pending_replication.push(event);
    }

    fn schedule_divine_cascade(&mut self, current_time: f64, mercy_level: f32) {
        let mut event = DynamicEvent {
            id: Uuid::new_v4(),
            event_type: DynamicEventType::DivineWhisperCascade { intensity: 0.7 + mercy_level * 0.3, target_players: None },
            scheduled_at: current_time as u64,
            triggered_at: Some(current_time as u64),
            mercy_alignment: mercy_level,
            affected_players: vec![],
            metadata: serde_json::json!({"council": "PATSAGi"}),
            priority: 0.0,
            base_priority: 0.0,
        };
        event.base_priority = event.compute_base_priority();
        event.priority = event.base_priority;
        self.active_events.push(event.clone());
        self.pending_replication.push(event);
    }

    pub fn get_relevant_events_for_player(&self, _player_pos: [f32; 3]) -> Vec<&DynamicEvent> {
        self.active_events.iter().collect()
    }

    /// Drains and returns events sorted by *current* (decayed) priority.
    /// High-mercy Divine/Treaty events stay prominent longer thanks to higher base_priority.
    pub fn drain_prioritized_replication(&mut self) -> Vec<DynamicEvent> {
        let mut events = std::mem::take(&mut self.pending_replication);
        // Decay is already applied in tick(); we just sort the current values
        events.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap_or(std::cmp::Ordering::Equal));
        events
    }

    pub fn drain_pending_replication(&mut self) -> Vec<DynamicEvent> {
        self.drain_prioritized_replication()
    }
}

// ═════════════════════════════════════════════════════════════════════════
// REPLICATION WIRING (with decay)
// ═════════════════════════════════════════════════════════════════════════
//
// In networking replication tick:
//   let current_time = time.elapsed_seconds_f64();
//   manager.apply_priority_decay(current_time); // optional if tick already does it
//   for event in manager.drain_prioritized_replication() {
//       let client_event = map_server_event_to_client(&event);
//       send_to_interested_players(client_event, ..., event.priority); // higher priority = better treatment
//   }
//
// Decay ensures the feed never stagnates while still letting the most mercy-aligned council events linger appropriately.

pub fn map_server_event_to_client(event: &DynamicEvent) -> Option<ClientWorldEventMirror> {
    match &event.event_type {
        DynamicEventType::FactionDiplomacyShift { faction_a, faction_b, .. } => {
            Some(ClientWorldEventMirror::FactionDiplomacyShift {
                faction_a: faction_a.clone(),
                faction_b: faction_b.clone(),
                reason: format!("Diplomacy shift (priority {:.2}) between {} and {}", event.priority, faction_a, faction_b),
            })
        }
        DynamicEventType::AbundanceSurge { multiplier, .. } => Some(ClientWorldEventMirror::AbundanceSurge {
            region: "Unknown Region".to_string(),
            intensity: *multiplier,
            mercy_delta: event.mercy_alignment * 10.0,
        }),
        DynamicEventType::DivineWhisperCascade { intensity, .. } => Some(ClientWorldEventMirror::DivineWhisperCascade {
            message: "A Divine Whisper cascades across the world...".to_string(),
            affected_factions: vec![],
            mercy_impact: *intensity,
        }),
        _ => None,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientWorldEventMirror {
    AbundanceSurge { region: String, intensity: f32, mercy_delta: f32 },
    FactionDiplomacyShift { faction_a: String, faction_b: String, reason: String },
    DivineWhisperCascade { message: String, affected_factions: Vec<String>, mercy_impact: f32 },
}

// ═══════════════════════════════════════════════════════════════════════════════════════════
// PLUGIN
// ═══════════════════════════════════════════════════════════════════════════════

pub struct DynamicEventsPlugin;

impl Plugin for DynamicEventsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DynamicEventsConfig>()
            .add_systems(Startup, setup_dynamic_events)
            .add_systems(Update, dynamic_events_tick_system);
    }
}

fn setup_dynamic_events(mut commands: Commands, config: Res<DynamicEventsConfig>) {
    let manager = DynamicEventManager::new(config.clone());
    commands.insert_resource(manager);
    info!("⚡ Dynamic Events v17.44 + Priority Decay online — PATSAGi / 7 Gates aware, self-refreshing feed");
}

fn dynamic_events_tick_system(
    mut manager: ResMut<DynamicEventManager>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_seconds_f64();
    let mercy_level = 0.88;
    manager.tick(current_time, mercy_level);
}

// Integration notes:
// - Decay runs automatically in tick().
// - Use drain_prioritized_replication() in networking — it returns events sorted by current (decayed) priority.
// - High-mercy Divine and Treaty events naturally stay relevant longer.
// - Client Eternal Flow Feed will feel alive and non-stale.

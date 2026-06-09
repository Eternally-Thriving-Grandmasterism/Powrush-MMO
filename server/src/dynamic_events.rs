// server/src/dynamic_events.rs
// Powrush-MMO v17.43 — Dynamic Events + Event Prioritization Logic
// PATSAGi Councils guided • Mercy-gated • 7 Living Mercy Gates aware
// Prioritization favors DivineWhisper, high-mercy Treaty DiplomacyShifts, and player-relevant Abundance
// Integrates with replication drain, InterestManager, HierarchicalGrid
// Full backward compatibility

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
    pub mercy_alignment: f32,      // 0.0 - 1.0 from 7 Living Mercy Gates
    pub affected_players: Vec<Uuid>,
    pub metadata: serde_json::Value,
    pub priority: f32,             // 0.0 (low) - 1.0+ (critical) — computed via 7 Gates + type + context
}

impl DynamicEvent {
    /// Computes base priority using PATSAGi / 7 Living Mercy Gates principles.
    /// Higher = more likely to be replicated first and highlighted in Eternal Flow Feed.
    /// DivineWhisper and high-mercy Treaty DiplomacyShifts are favored.
    pub fn base_priority(&self) -> f32 {
        let type_priority = match &self.event_type {
            DynamicEventType::DivineWhisperCascade { intensity, .. } => 0.92 + intensity.clamp(0.0, 0.08),
            DynamicEventType::FactionDiplomacyShift { .. } => {
                // Treaty events with rich PATSAGi council reason get strong boost
                0.82 + (self.mercy_alignment * 0.15)
            }
            DynamicEventType::AbundanceSurge { multiplier, .. } => {
                0.55 + (multiplier - 1.0).clamp(0.0, 0.25) + (self.mercy_alignment * 0.12)
            }
            DynamicEventType::AnomalyTrigger { severity, .. } => 0.70 + severity.clamp(0.0, 0.2),
            DynamicEventType::WorldShift { .. } => 0.65,
            DynamicEventType::MercyTest { difficulty, .. } => 0.60 + difficulty.clamp(0.0, 0.25),
            DynamicEventType::Custom { .. } => 0.40,
        };

        // Slight recency / urgency boost (newer events slightly higher)
        let recency_boost = if let Some(triggered) = self.triggered_at {
            // Simple linear decay; in prod use current_time
            (triggered as f32 % 1000.0) / 5000.0
        } else { 0.0 };

        (type_priority + recency_boost).clamp(0.1, 1.35)
    }
}

// ═════════════════════════════════════════════════════════════════════════
// CONFIG
// ═════════════════════════════════════════════════════════════════════════

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct DynamicEventsConfig {
    pub abundance_event_rate_per_hour: f32,
    pub faction_event_rate_per_hour: f32,
    pub divine_whisper_cascade_rate: f32,
    pub mercy_influence_strength: f32,
    pub max_concurrent_events: u32,
    pub event_persistence_enabled: bool,
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
        }
    }
}

// ═════════════════════════════════════════════════════════════════════════
// DYNAMIC EVENT MANAGER + PRIORITIZATION
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
    }

    fn process_scheduled_events(&mut self, current_time: f64) {
        let mut to_remove = Vec::new();
        for (i, event) in self.active_events.iter_mut().enumerate() {
            if let Some(triggered) = event.triggered_at {
                if current_time - triggered as f64 > 300.0 {
                    to_remove.push(i);
                }
            }
        }
        for i in to_remove.into_iter().rev() {
            if let Some(event) = self.active_events.remove(i) {
                self.event_history.push_front(event);
                if self.event_history.len() > 64 {
                    self.event_history.pop_back();
                }
            }
        }
    }

    fn consider_new_events(&mut self, current_time: f64, mercy_level: f32) {
        if current_time - self.last_abundance_check > (3600.0 / self.config.abundance_event_rate_per_hour as f64) {
            if rand::random::<f32>() < (0.6 + mercy_level * 0.4) {
                self.schedule_abundance_event(current_time, mercy_level);
            }
            self.last_abundance_check = current_time;
        }

        if current_time - self.last_faction_check > (3600.0 / self.config.faction_event_rate_per_hour as f64) {
            if rand::random::<f32>() < 0.35 {
                self.schedule_faction_event(current_time);
            }
            self.last_faction_check = current_time;
        }

        if rand::random::<f32>() < self.config.divine_whisper_cascade_rate / 100.0 {
            self.schedule_divine_cascade(current_time, mercy_level);
        }
    }

    fn schedule_abundance_event(&mut self, current_time: f64, mercy_level: f32) {
        let mut event = DynamicEvent {
            id: Uuid::new_v4(),
            event_type: DynamicEventType::AbundanceSurge {
                multiplier: 1.5 + mercy_level * 0.8,
                duration_seconds: 180 + (mercy_level * 120.0) as u32,
            },
            scheduled_at: current_time as u64,
            triggered_at: Some(current_time as u64),
            mercy_alignment: mercy_level,
            affected_players: vec![],
            metadata: serde_json::json!({"source": "EternalFlow"}),
            priority: 0.0, // set below
        };
        event.priority = event.base_priority();
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
        };
        event.priority = event.base_priority();
        self.active_events.push(event.clone());
        self.pending_replication.push(event);
    }

    fn schedule_divine_cascade(&mut self, current_time: f64, mercy_level: f32) {
        let mut event = DynamicEvent {
            id: Uuid::new_v4(),
            event_type: DynamicEventType::DivineWhisperCascade {
                intensity: 0.7 + mercy_level * 0.3,
                target_players: None,
            },
            scheduled_at: current_time as u64,
            triggered_at: Some(current_time as u64),
            mercy_alignment: mercy_level,
            affected_players: vec![],
            metadata: serde_json::json!({"council": "PATSAGi"}),
            priority: 0.0,
        };
        event.priority = event.base_priority();
        self.active_events.push(event.clone());
        self.pending_replication.push(event);
    }

    pub fn get_relevant_events_for_player(&self, _player_pos: [f32; 3]) -> Vec<&DynamicEvent> {
        self.active_events.iter().collect()
    }

    /// Returns pending events sorted by priority (highest first) for replication.
    /// Networking layer should prefer high-priority (Divine + high-mercy Treaty) events.
    pub fn drain_prioritized_replication(&mut self) -> Vec<DynamicEvent> {
        let mut events = std::mem::take(&mut self.pending_replication);
        events.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap_or(std::cmp::Ordering::Equal));
        events
    }

    // Legacy drain kept for compatibility
    pub fn drain_pending_replication(&mut self) -> Vec<DynamicEvent> {
        self.drain_prioritized_replication()
    }
}

// ═════════════════════════════════════════════════════════════════════════
// REPLICATION WIRING (updated with prioritization)
// ═════════════════════════════════════════════════════════════════════════
//
// Networking replication tick:
//   let mut manager = ...get_resource_mut::<DynamicEventManager>()...;
//   for event in manager.drain_prioritized_replication() {
//       if let Some(client_event) = map_server_event_to_client(&event) {
//           // High priority events are sent first / with higher reliability / to more players
//           send_to_interested_players(client_event, &interest_manager, &grid, &connections, event.priority);
//       }
//   }
//
// Prioritization already baked into every scheduled event via base_priority() + 7 Gates mercy_alignment.
// Treaty DiplomacyShifts and DivineWhisperCascade naturally rise to the top.

pub fn map_server_event_to_client(event: &DynamicEvent) -> Option<ClientWorldEventMirror> {
    match &event.event_type {
        DynamicEventType::FactionDiplomacyShift { faction_a, faction_b, .. } => {
            Some(ClientWorldEventMirror::FactionDiplomacyShift {
                faction_a: faction_a.clone(),
                faction_b: faction_b.clone(),
                reason: format!("Diplomacy shift (priority {:.2}) between {} and {}", event.priority, faction_a, faction_b),
            })
        }
        DynamicEventType::AbundanceSurge { multiplier, .. } => {
            Some(ClientWorldEventMirror::AbundanceSurge {
                region: "Unknown Region".to_string(),
                intensity: *multiplier,
                mercy_delta: event.mercy_alignment * 10.0,
            })
        }
        DynamicEventType::DivineWhisperCascade { intensity, .. } => {
            Some(ClientWorldEventMirror::DivineWhisperCascade {
                message: "A Divine Whisper cascades across the world...".to_string(),
                affected_factions: vec![],
                mercy_impact: *intensity,
            })
        }
        _ => None,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ClientWorldEventMirror {
    AbundanceSurge { region: String, intensity: f32, mercy_delta: f32 },
    FactionDiplomacyShift { faction_a: String, faction_b: String, reason: String },
    DivineWhisperCascade { message: String, affected_factions: Vec<String>, mercy_impact: f32 },
}

// ═════════════════════════════════════════════════════════════════════════
// PLUGIN
// ═════════════════════════════════════════════════════════════════════════

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
    info!("⚡ Dynamic Events + Prioritization (v17.43) online — PATSAGi / 7 Gates aware replication ready");
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
// - Use drain_prioritized_replication() (or the alias drain_pending_replication) in your networking layer.
// - High priority events (DivineWhisper > high-mercy Treaty Diplomacy > Abundance) are sent first.
// - Combine with InterestManager + HierarchicalGrid for player-specific relevance + priority.
// - Client feed will naturally surface high-priority events first when sorted client-side (future).
